#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIAddFile(hfci: *const ::core::ffi::c_void, pszsourcefile: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, fexecute: super::super::Foundation::BOOL, pfnfcignc: ::windows::runtime::RawPtr, pfnfcis: ::windows::runtime::RawPtr, pfnfcigoi: ::windows::runtime::RawPtr, typecompress: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCICreate(perf: *const ERF, pfnfcifp: ::windows::runtime::RawPtr, pfna: ::windows::runtime::RawPtr, pfnf: ::windows::runtime::RawPtr, pfnopen: ::windows::runtime::RawPtr, pfnread: ::windows::runtime::RawPtr, pfnwrite: ::windows::runtime::RawPtr, pfnclose: ::windows::runtime::RawPtr, pfnseek: ::windows::runtime::RawPtr, pfndelete: ::windows::runtime::RawPtr, pfnfcigtf: ::windows::runtime::RawPtr, pccab: *const CCAB, pv: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushCabinet(hfci: *const ::core::ffi::c_void, fgetnextcab: super::super::Foundation::BOOL, pfnfcignc: ::windows::runtime::RawPtr, pfnfcis: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: ::windows::runtime::RawPtr, pfnfcis: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICopy(hfdi: *const ::core::ffi::c_void, pszcabinet: super::super::Foundation::PSTR, pszcabpath: super::super::Foundation::PSTR, flags: i32, pfnfdin: ::windows::runtime::RawPtr, pfnfdid: ::windows::runtime::RawPtr, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICreate(pfnalloc: ::windows::runtime::RawPtr, pfnfree: ::windows::runtime::RawPtr, pfnopen: ::windows::runtime::RawPtr, pfnread: ::windows::runtime::RawPtr, pfnwrite: ::windows::runtime::RawPtr, pfnclose: ::windows::runtime::RawPtr, pfnseek: ::windows::runtime::RawPtr, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIDestroy(hfdi: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIIsCabinet(hfdi: *const ::core::ffi::c_void, hf: isize, pfdici: *mut FDICABINETINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDITruncateCabinet(hfdi: *const ::core::ffi::c_void, pszcabinetname: super::super::Foundation::PSTR, ifoldertodelete: u16) -> super::super::Foundation::BOOL;
}
