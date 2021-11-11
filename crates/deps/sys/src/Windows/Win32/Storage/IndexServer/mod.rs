#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn BindIFilterFromStorage(pstg: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn BindIFilterFromStream(pstm: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIFilter(pwcspath: super::super::Foundation::PWSTR, punkouter: ::windows::runtime::RawPtr, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIFilterEx(pwcspath: super::super::Foundation::PWSTR, dwflags: u32, riid: *const ::windows::runtime::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
