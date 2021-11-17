#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIAddFile(hfci: *const ::core::ffi::c_void, pszsourcefile: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, fexecute: super::super::Foundation::BOOL, pfnfcignc: ::core::option::Option<PFNFCIGETNEXTCABINET>, pfnfcis: ::core::option::Option<PFNFCISTATUS>, pfnfcigoi: ::core::option::Option<PFNFCIGETOPENINFO>, typecompress: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCICreate(
        perf: *const ERF,
        pfnfcifp: ::core::option::Option<PFNFCIFILEPLACED>,
        pfna: ::core::option::Option<PFNFCIALLOC>,
        pfnf: ::core::option::Option<PFNFCIFREE>,
        pfnopen: ::core::option::Option<PFNFCIOPEN>,
        pfnread: ::core::option::Option<PFNFCIREAD>,
        pfnwrite: ::core::option::Option<PFNFCIWRITE>,
        pfnclose: ::core::option::Option<PFNFCICLOSE>,
        pfnseek: ::core::option::Option<PFNFCISEEK>,
        pfndelete: ::core::option::Option<PFNFCIDELETE>,
        pfnfcigtf: ::core::option::Option<PFNFCIGETTEMPFILE>,
        pccab: *const CCAB,
        pv: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushCabinet(hfci: *const ::core::ffi::c_void, fgetnextcab: super::super::Foundation::BOOL, pfnfcignc: ::core::option::Option<PFNFCIGETNEXTCABINET>, pfnfcis: ::core::option::Option<PFNFCISTATUS>) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: ::core::option::Option<PFNFCIGETNEXTCABINET>, pfnfcis: ::core::option::Option<PFNFCISTATUS>) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICopy(hfdi: *const ::core::ffi::c_void, pszcabinet: super::super::Foundation::PSTR, pszcabpath: super::super::Foundation::PSTR, flags: i32, pfnfdin: ::core::option::Option<PFNFDINOTIFY>, pfnfdid: ::core::option::Option<PFNFDIDECRYPT>, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICreate(pfnalloc: ::core::option::Option<PFNALLOC>, pfnfree: ::core::option::Option<PFNFREE>, pfnopen: ::core::option::Option<PFNOPEN>, pfnread: ::core::option::Option<PFNREAD>, pfnwrite: ::core::option::Option<PFNWRITE>, pfnclose: ::core::option::Option<PFNCLOSE>, pfnseek: ::core::option::Option<PFNSEEK>, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void;
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CCAB {
    pub cb: u32,
    pub cbFolderThresh: u32,
    pub cbReserveCFHeader: u32,
    pub cbReserveCFFolder: u32,
    pub cbReserveCFData: u32,
    pub iCab: i32,
    pub iDisk: i32,
    pub fFailOnIncompressible: i32,
    pub setID: u16,
    pub szDisk: [super::super::Foundation::CHAR; 256],
    pub szCab: [super::super::Foundation::CHAR; 256],
    pub szCabPath: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CCAB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CCAB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ERF {
    pub erfOper: i32,
    pub erfType: i32,
    pub fError: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ERF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ERF {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FCIERROR = i32;
pub const FCIERR_NONE: FCIERROR = 0i32;
pub const FCIERR_OPEN_SRC: FCIERROR = 1i32;
pub const FCIERR_READ_SRC: FCIERROR = 2i32;
pub const FCIERR_ALLOC_FAIL: FCIERROR = 3i32;
pub const FCIERR_TEMP_FILE: FCIERROR = 4i32;
pub const FCIERR_BAD_COMPR_TYPE: FCIERROR = 5i32;
pub const FCIERR_CAB_FILE: FCIERROR = 6i32;
pub const FCIERR_USER_ABORT: FCIERROR = 7i32;
pub const FCIERR_MCI_FAIL: FCIERROR = 8i32;
pub const FCIERR_CAB_FORMAT_LIMIT: FCIERROR = 9i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDICABINETINFO {
    pub cbCabinet: i32,
    pub cFolders: u16,
    pub cFiles: u16,
    pub setID: u16,
    pub iCabinet: u16,
    pub fReserve: super::super::Foundation::BOOL,
    pub hasprev: super::super::Foundation::BOOL,
    pub hasnext: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDICABINETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDICABINETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FDICREATE_CPU_TYPE = u32;
pub const cpu80286: FDICREATE_CPU_TYPE = 0u32;
pub const cpu80386: FDICREATE_CPU_TYPE = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDIDECRYPT {
    pub fdidt: FDIDECRYPTTYPE,
    pub pvUser: *mut ::core::ffi::c_void,
    pub Anonymous: FDIDECRYPT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDIDECRYPT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDIDECRYPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union FDIDECRYPT_0 {
    pub cabinet: FDIDECRYPT_0_0,
    pub folder: FDIDECRYPT_0_2,
    pub decrypt: FDIDECRYPT_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDIDECRYPT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDIDECRYPT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDIDECRYPT_0_0 {
    pub pHeaderReserve: *mut ::core::ffi::c_void,
    pub cbHeaderReserve: u16,
    pub setID: u16,
    pub iCabinet: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDIDECRYPT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDIDECRYPT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDIDECRYPT_0_1 {
    pub pDataReserve: *mut ::core::ffi::c_void,
    pub cbDataReserve: u16,
    pub pbData: *mut ::core::ffi::c_void,
    pub cbData: u16,
    pub fSplit: super::super::Foundation::BOOL,
    pub cbPartial: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDIDECRYPT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDIDECRYPT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDIDECRYPT_0_2 {
    pub pFolderReserve: *mut ::core::ffi::c_void,
    pub cbFolderReserve: u16,
    pub iFolder: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDIDECRYPT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDIDECRYPT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FDIDECRYPTTYPE = i32;
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = 0i32;
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = 1i32;
pub const fdidtDECRYPT: FDIDECRYPTTYPE = 2i32;
pub type FDIERROR = i32;
pub const FDIERROR_NONE: FDIERROR = 0i32;
pub const FDIERROR_CABINET_NOT_FOUND: FDIERROR = 1i32;
pub const FDIERROR_NOT_A_CABINET: FDIERROR = 2i32;
pub const FDIERROR_UNKNOWN_CABINET_VERSION: FDIERROR = 3i32;
pub const FDIERROR_CORRUPT_CABINET: FDIERROR = 4i32;
pub const FDIERROR_ALLOC_FAIL: FDIERROR = 5i32;
pub const FDIERROR_BAD_COMPR_TYPE: FDIERROR = 6i32;
pub const FDIERROR_MDI_FAIL: FDIERROR = 7i32;
pub const FDIERROR_TARGET_FILE: FDIERROR = 8i32;
pub const FDIERROR_RESERVE_MISMATCH: FDIERROR = 9i32;
pub const FDIERROR_WRONG_CABINET: FDIERROR = 10i32;
pub const FDIERROR_USER_ABORT: FDIERROR = 11i32;
pub const FDIERROR_EOF: FDIERROR = 12i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDINOTIFICATION {
    pub cb: i32,
    pub psz1: super::super::Foundation::PSTR,
    pub psz2: super::super::Foundation::PSTR,
    pub psz3: super::super::Foundation::PSTR,
    pub pv: *mut ::core::ffi::c_void,
    pub hf: isize,
    pub date: u16,
    pub time: u16,
    pub attribs: u16,
    pub setID: u16,
    pub iCabinet: u16,
    pub iFolder: u16,
    pub fdie: FDIERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDINOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDINOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FDINOTIFICATIONTYPE = i32;
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = 0i32;
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = 1i32;
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = 2i32;
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = 3i32;
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = 4i32;
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = 5i32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FDISPILLFILE {
    pub ach: [super::super::Foundation::CHAR; 2],
    pub cbFile: i32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDISPILLFILE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDISPILLFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FDISPILLFILE {
    pub ach: [super::super::Foundation::CHAR; 2],
    pub cbFile: i32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDISPILLFILE {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDISPILLFILE {
    fn clone(&self) -> Self {
        *self
    }
}
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
