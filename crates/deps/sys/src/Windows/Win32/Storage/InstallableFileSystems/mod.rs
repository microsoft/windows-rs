#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterAttach();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterAttachAtAltitude();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterClose();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FilterConnectCommunicationPort();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterCreate();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterDetach();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterFindClose();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterFindFirst();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterFindNext();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterGetDosName();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterGetInformation();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FilterGetMessage();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterInstanceClose();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceCreate();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindClose();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindFirst();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindNext();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterInstanceGetInformation();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterLoad();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterReplyMessage();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterSendMessage();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterUnload();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeFindClose();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterVolumeFindFirst();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeFindNext();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindClose();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindFirst();
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindNext();
}
