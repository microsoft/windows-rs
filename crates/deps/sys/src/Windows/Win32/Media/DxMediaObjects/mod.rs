#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub fn DMOEnum(guidcategory: *const ::windows::runtime::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMOGetName(clsiddmo: *const ::windows::runtime::GUID, szname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub fn DMOGetTypes(clsiddmo: *const ::windows::runtime::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMORegister(szname: super::super::Foundation::PWSTR, clsiddmo: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub fn DMOUnregister(clsiddmo: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCopyMediaType(pmtdest: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, pmtsrc: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDeleteMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoFreeMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoInitMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, cbformat: u32) -> ::windows::runtime::HRESULT;
}
