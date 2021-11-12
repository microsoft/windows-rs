#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIAddFile(hfci: *const ::core::ffi::c_void, pszsourcefile: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, fexecute: super::super::Foundation::BOOL, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS, pfnfcigoi: PFNFCIGETOPENINFO, typecompress: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCICreate(perf: *const ERF, pfnfcifp: PFNFCIFILEPLACED, pfna: PFNFCIALLOC, pfnf: PFNFCIFREE, pfnopen: PFNFCIOPEN, pfnread: PFNFCIREAD, pfnwrite: PFNFCIWRITE, pfnclose: PFNFCICLOSE, pfnseek: PFNFCISEEK, pfndelete: PFNFCIDELETE, pfnfcigtf: PFNFCIGETTEMPFILE, pccab: *const CCAB, pv: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushCabinet(hfci: *const ::core::ffi::c_void, fgetnextcab: super::super::Foundation::BOOL, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICopy(hfdi: *const ::core::ffi::c_void, pszcabinet: super::super::Foundation::PSTR, pszcabpath: super::super::Foundation::PSTR, flags: i32, pfnfdin: PFNFDINOTIFY, pfnfdid: PFNFDIDECRYPT, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICreate(pfnalloc: PFNALLOC, pfnfree: PFNFREE, pfnopen: PFNOPEN, pfnread: PFNREAD, pfnwrite: PFNWRITE, pfnclose: PFNCLOSE, pfnseek: PFNSEEK, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIDestroy(hfdi: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIIsCabinet(hfdi: *const ::core::ffi::c_void, hf: isize, pfdici: *mut FDICABINETINFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDITruncateCabinet(hfdi: *const ::core::ffi::c_void, pszcabinetname: super::super::Foundation::PSTR, ifoldertodelete: u16) -> super::super::Foundation::BOOL;
}
pub const CB_MAX_CABINET_NAME: u32 = 256u32;
pub const CB_MAX_CAB_PATH: u32 = 256u32;
pub const CB_MAX_DISK: i32 = 2147483647i32;
pub const CB_MAX_DISK_NAME: u32 = 256u32;
pub const CB_MAX_FILENAME: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CCAB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ERF(i32);
#[repr(transparent)]
pub struct FCIERROR(pub i32);
pub const FCIERR_NONE: FCIERROR = FCIERROR(0i32);
pub const FCIERR_OPEN_SRC: FCIERROR = FCIERROR(1i32);
pub const FCIERR_READ_SRC: FCIERROR = FCIERROR(2i32);
pub const FCIERR_ALLOC_FAIL: FCIERROR = FCIERROR(3i32);
pub const FCIERR_TEMP_FILE: FCIERROR = FCIERROR(4i32);
pub const FCIERR_BAD_COMPR_TYPE: FCIERROR = FCIERROR(5i32);
pub const FCIERR_CAB_FILE: FCIERROR = FCIERROR(6i32);
pub const FCIERR_USER_ABORT: FCIERROR = FCIERROR(7i32);
pub const FCIERR_MCI_FAIL: FCIERROR = FCIERROR(8i32);
pub const FCIERR_CAB_FORMAT_LIMIT: FCIERROR = FCIERROR(9i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FDICABINETINFO(i32);
#[repr(transparent)]
pub struct FDICREATE_CPU_TYPE(pub u32);
pub const cpu80286: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(0u32);
pub const cpu80386: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FDIDECRYPT(i32);
#[repr(transparent)]
pub struct FDIDECRYPTTYPE(pub i32);
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = FDIDECRYPTTYPE(0i32);
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = FDIDECRYPTTYPE(1i32);
pub const fdidtDECRYPT: FDIDECRYPTTYPE = FDIDECRYPTTYPE(2i32);
#[repr(transparent)]
pub struct FDIERROR(pub i32);
pub const FDIERROR_NONE: FDIERROR = FDIERROR(0i32);
pub const FDIERROR_CABINET_NOT_FOUND: FDIERROR = FDIERROR(1i32);
pub const FDIERROR_NOT_A_CABINET: FDIERROR = FDIERROR(2i32);
pub const FDIERROR_UNKNOWN_CABINET_VERSION: FDIERROR = FDIERROR(3i32);
pub const FDIERROR_CORRUPT_CABINET: FDIERROR = FDIERROR(4i32);
pub const FDIERROR_ALLOC_FAIL: FDIERROR = FDIERROR(5i32);
pub const FDIERROR_BAD_COMPR_TYPE: FDIERROR = FDIERROR(6i32);
pub const FDIERROR_MDI_FAIL: FDIERROR = FDIERROR(7i32);
pub const FDIERROR_TARGET_FILE: FDIERROR = FDIERROR(8i32);
pub const FDIERROR_RESERVE_MISMATCH: FDIERROR = FDIERROR(9i32);
pub const FDIERROR_WRONG_CABINET: FDIERROR = FDIERROR(10i32);
pub const FDIERROR_USER_ABORT: FDIERROR = FDIERROR(11i32);
pub const FDIERROR_EOF: FDIERROR = FDIERROR(12i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FDINOTIFICATION(i32);
#[repr(transparent)]
pub struct FDINOTIFICATIONTYPE(pub i32);
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(0i32);
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(1i32);
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(2i32);
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(3i32);
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(4i32);
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(5i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FDISPILLFILE(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FDISPILLFILE(i32);
pub const INCLUDED_FCI: u32 = 1u32;
pub const INCLUDED_FDI: u32 = 1u32;
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
pub type PFNALLOC = unsafe extern "system" fn(cb: u32) -> *mut ::core::ffi::c_void;
pub type PFNCLOSE = unsafe extern "system" fn(hf: isize) -> i32;
pub type PFNFCIALLOC = unsafe extern "system" fn(cb: u32) -> *mut ::core::ffi::c_void;
pub type PFNFCICLOSE = unsafe extern "system" fn(hf: isize, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIDELETE = unsafe extern "system" fn(pszfile: super::super::Foundation::PSTR, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIFILEPLACED = unsafe extern "system" fn(pccab: *mut CCAB, pszfile: super::super::Foundation::PSTR, cbfile: i32, fcontinuation: super::super::Foundation::BOOL, pv: *mut ::core::ffi::c_void) -> i32;
pub type PFNFCIFREE = unsafe extern "system" fn(memory: *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETNEXTCABINET = unsafe extern "system" fn(pccab: *mut CCAB, cbprevcab: u32, pv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETOPENINFO = unsafe extern "system" fn(pszname: super::super::Foundation::PSTR, pdate: *mut u16, ptime: *mut u16, pattribs: *mut u16, err: *mut i32, pv: *mut ::core::ffi::c_void) -> isize;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETTEMPFILE = unsafe extern "system" fn(psztempname: super::super::Foundation::PSTR, cbtempname: i32, pv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIOPEN = unsafe extern "system" fn(pszfile: super::super::Foundation::PSTR, oflag: i32, pmode: i32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> isize;
pub type PFNFCIREAD = unsafe extern "system" fn(hf: isize, memory: *mut ::core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> u32;
pub type PFNFCISEEK = unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32;
pub type PFNFCISTATUS = unsafe extern "system" fn(typestatus: u32, cb1: u32, cb2: u32, pv: *mut ::core::ffi::c_void) -> i32;
pub type PFNFCIWRITE = unsafe extern "system" fn(hf: isize, memory: *mut ::core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFDIDECRYPT = unsafe extern "system" fn(pfdid: *mut FDIDECRYPT) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFDINOTIFY = unsafe extern "system" fn(fdint: FDINOTIFICATIONTYPE, pfdin: *mut FDINOTIFICATION) -> isize;
pub type PFNFREE = unsafe extern "system" fn(pv: *const ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFNOPEN = unsafe extern "system" fn(pszfile: super::super::Foundation::PSTR, oflag: i32, pmode: i32) -> isize;
pub type PFNREAD = unsafe extern "system" fn(hf: isize, pv: *mut ::core::ffi::c_void, cb: u32) -> u32;
pub type PFNSEEK = unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32) -> i32;
pub type PFNWRITE = unsafe extern "system" fn(hf: isize, pv: *const ::core::ffi::c_void, cb: u32) -> u32;
pub const _A_EXEC: u32 = 64u32;
pub const _A_NAME_IS_UTF: u32 = 128u32;
