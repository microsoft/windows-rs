#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Imaging`*"]
    pub fn WICConvertBitmapSource(dstformat: *const ::windows::runtime::GUID, pisrc: ::windows::runtime::RawPtr, ppidst: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICCreateBitmapFromSection(width: u32, height: u32, pixelformat: *const ::windows::runtime::GUID, hsection: super::super::Foundation::HANDLE, stride: u32, offset: u32, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICCreateBitmapFromSectionEx(width: u32, height: u32, pixelformat: *const ::windows::runtime::GUID, hsection: super::super::Foundation::HANDLE, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`*"]
    pub fn WICGetMetadataContentSize(guidcontainerformat: *const ::windows::runtime::GUID, piwriter: ::windows::runtime::RawPtr, pcbsize: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICMapGuidToShortName(guid: *const ::windows::runtime::GUID, cchname: u32, wzname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICMapSchemaToName(guidmetadataformat: *const ::windows::runtime::GUID, pwzschema: super::super::Foundation::PWSTR, cchname: u32, wzname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WICMapShortNameToGuid(wzname: super::super::Foundation::PWSTR, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WICMatchMetadataContent(guidcontainerformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID, pistream: ::windows::runtime::RawPtr, pguidmetadataformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Imaging`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WICSerializeMetadataContent(guidcontainerformat: *const ::windows::runtime::GUID, piwriter: ::windows::runtime::RawPtr, dwpersistoptions: u32, pistream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
