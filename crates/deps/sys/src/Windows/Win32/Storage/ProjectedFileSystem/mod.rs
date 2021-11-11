#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjAllocateAlignedBuffer();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjClearNegativePathCache();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjCompleteCommand();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjDeleteFile();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjDoesNameContainWildCards();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFileNameCompare();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFileNameMatch();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFillDirEntryBuffer();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFillDirEntryBuffer2();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjFreeAlignedBuffer();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjGetOnDiskFileState();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjGetVirtualizationInstanceInfo();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjMarkDirectoryAsPlaceholder();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjStartVirtualizing();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjStopVirtualizing();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjUpdateFileIfNeeded();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`*"]
    pub fn PrjWriteFileData();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjWritePlaceholderInfo();
    #[doc = "*Required features: `Win32_Storage_ProjectedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjWritePlaceholderInfo2();
}
