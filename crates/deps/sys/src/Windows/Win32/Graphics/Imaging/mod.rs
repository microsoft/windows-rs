#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Imaging`*"]
    pub fn WICConvertBitmapSource();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICCreateBitmapFromSection();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICCreateBitmapFromSectionEx();
    #[doc = "*Required features: `Win32_Graphics_Imaging`*"]
    pub fn WICGetMetadataContentSize();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICMapGuidToShortName();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICMapSchemaToName();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICMapShortNameToGuid();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WICMatchMetadataContent();
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WICSerializeMetadataContent();
}
