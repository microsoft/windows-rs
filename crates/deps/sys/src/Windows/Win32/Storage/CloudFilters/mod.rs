#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfCloseHandle();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfConnectSyncRoot();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfConvertToPlaceholder();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn CfCreatePlaceholders();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfDehydratePlaceholder();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfDisconnectSyncRoot();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_CorrelationVector"))]
    pub fn CfExecute();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfGetCorrelationVector();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetPlaceholderInfo();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetPlaceholderRangeInfo();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfGetPlaceholderStateFromAttributeTag();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Storage_FileSystem`*"]
    #[cfg(feature = "Win32_Storage_FileSystem")]
    pub fn CfGetPlaceholderStateFromFileInfo();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn CfGetPlaceholderStateFromFindData();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfGetPlatformInfo();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetSyncRootInfoByHandle();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetSyncRootInfoByPath();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetTransferKey();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetWin32HandleFromProtectedHandle();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfHydratePlaceholder();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfOpenFileWithOplock();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfQuerySyncProviderStatus();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReferenceProtectedHandle();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfRegisterSyncRoot();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReleaseProtectedHandle();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReleaseTransferKey();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfReportProviderProgress();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfReportProviderProgress2();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReportSyncStatus();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfRevertPlaceholder();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfSetCorrelationVector();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfSetInSyncState();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfSetPinState();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfUnregisterSyncRoot();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
    pub fn CfUpdatePlaceholder();
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfUpdateSyncProviderStatus();
}
