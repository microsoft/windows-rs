#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const CB_MAX_CABINET_NAME: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const CB_MAX_CAB_PATH: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const CB_MAX_DISK: i32 = 2147483647i32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const CB_MAX_DISK_NAME: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const CB_MAX_FILENAME: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const INCLUDED_FCI: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const INCLUDED_FDI: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const _A_EXEC: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_Cabinets`*"]
pub const _A_NAME_IS_UTF: u32 = 128u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIAddFile(hfci: *const ::core::ffi::c_void, pszsourcefile: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, fexecute: super::super::Foundation::BOOL, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS, pfnfcigoi: PFNFCIGETOPENINFO, typecompress: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCICreate(perf: *const ERF, pfnfcifp: PFNFCIFILEPLACED, pfna: PFNFCIALLOC, pfnf: PFNFCIFREE, pfnopen: PFNFCIOPEN, pfnread: PFNFCIREAD, pfnwrite: PFNFCIWRITE, pfnclose: PFNFCICLOSE, pfnseek: PFNFCISEEK, pfndelete: PFNFCIDELETE, pfnfcigtf: PFNFCIGETTEMPFILE, pccab: *const CCAB, pv: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushCabinet(hfci: *const ::core::ffi::c_void, fgetnextcab: super::super::Foundation::BOOL, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICopy(hfdi: *const ::core::ffi::c_void, pszcabinet: super::super::Foundation::PSTR, pszcabpath: super::super::Foundation::PSTR, flags: i32, pfnfdin: PFNFDINOTIFY, pfnfdid: PFNFDIDECRYPT, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICreate(pfnalloc: PFNALLOC, pfnfree: PFNFREE, pfnopen: PFNOPEN, pfnread: PFNREAD, pfnwrite: PFNWRITE, pfnclose: PFNCLOSE, pfnseek: PFNSEEK, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void;
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
