#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIAddFile(hfci: *const ::core::ffi::c_void, pszsourcefile: ::windows_sys::core::PCSTR, pszfilename: ::windows_sys::core::PCSTR, fexecute: super::super::Foundation::BOOL, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS, pfnfcigoi: PFNFCIGETOPENINFO, typecompress: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCICreate(perf: *const ERF, pfnfcifp: PFNFCIFILEPLACED, pfna: PFNFCIALLOC, pfnf: PFNFCIFREE, pfnopen: PFNFCIOPEN, pfnread: PFNFCIREAD, pfnwrite: PFNFCIWRITE, pfnclose: PFNFCICLOSE, pfnseek: PFNFCISEEK, pfndelete: PFNFCIDELETE, pfnfcigtf: PFNFCIGETTEMPFILE, pccab: *const CCAB, pv: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushCabinet(hfci: *const ::core::ffi::c_void, fgetnextcab: super::super::Foundation::BOOL, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICopy(hfdi: *const ::core::ffi::c_void, pszcabinet: ::windows_sys::core::PCSTR, pszcabpath: ::windows_sys::core::PCSTR, flags: i32, pfnfdin: PFNFDINOTIFY, pfnfdid: PFNFDIDECRYPT, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICreate(pfnalloc: PFNALLOC, pfnfree: PFNFREE, pfnopen: PFNOPEN, pfnread: PFNREAD, pfnwrite: PFNWRITE, pfnclose: PFNCLOSE, pfnseek: PFNSEEK, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIDestroy(hfdi: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIIsCabinet(hfdi: *const ::core::ffi::c_void, hf: isize, pfdici: *mut FDICABINETINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDITruncateCabinet(hfdi: *const ::core::ffi::c_void, pszcabinetname: ::windows_sys::core::PCSTR, ifoldertodelete: u16) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const CB_MAX_CABINET_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const CB_MAX_CAB_PATH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const CB_MAX_DISK: i32 = 2147483647i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const CB_MAX_DISK_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const CB_MAX_FILENAME: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type FCIERROR = i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_NONE: FCIERROR = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_OPEN_SRC: FCIERROR = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_READ_SRC: FCIERROR = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_ALLOC_FAIL: FCIERROR = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_TEMP_FILE: FCIERROR = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_BAD_COMPR_TYPE: FCIERROR = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_CAB_FILE: FCIERROR = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_USER_ABORT: FCIERROR = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_MCI_FAIL: FCIERROR = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_CAB_FORMAT_LIMIT: FCIERROR = 9i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type FDICREATE_CPU_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const cpuUNKNOWN: FDICREATE_CPU_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const cpu80286: FDICREATE_CPU_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const cpu80386: FDICREATE_CPU_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type FDIDECRYPTTYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdidtDECRYPT: FDIDECRYPTTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type FDIERROR = i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_NONE: FDIERROR = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_CABINET_NOT_FOUND: FDIERROR = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_NOT_A_CABINET: FDIERROR = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_UNKNOWN_CABINET_VERSION: FDIERROR = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_CORRUPT_CABINET: FDIERROR = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_ALLOC_FAIL: FDIERROR = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_BAD_COMPR_TYPE: FDIERROR = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_MDI_FAIL: FDIERROR = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_TARGET_FILE: FDIERROR = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_RESERVE_MISMATCH: FDIERROR = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_WRONG_CABINET: FDIERROR = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_USER_ABORT: FDIERROR = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_EOF: FDIERROR = 12i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub struct FDINOTIFICATION {
    pub cb: i32,
    pub psz1: ::windows_sys::core::PSTR,
    pub psz2: ::windows_sys::core::PSTR,
    pub psz3: ::windows_sys::core::PSTR,
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
impl ::core::marker::Copy for FDINOTIFICATION {}
impl ::core::clone::Clone for FDINOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type FDINOTIFICATIONTYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FDISPILLFILE {
    pub ach: [super::super::Foundation::CHAR; 2],
    pub cbFile: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDISPILLFILE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDISPILLFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FDISPILLFILE {
    pub ach: [super::super::Foundation::CHAR; 2],
    pub cbFile: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FDISPILLFILE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FDISPILLFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const INCLUDED_FCI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const INCLUDED_FDI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNALLOC = ::core::option::Option<unsafe extern "system" fn(cb: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNCLOSE = ::core::option::Option<unsafe extern "system" fn(hf: isize) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIALLOC = ::core::option::Option<unsafe extern "system" fn(cb: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCICLOSE = ::core::option::Option<unsafe extern "system" fn(hf: isize, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIDELETE = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows_sys::core::PCSTR, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIFILEPLACED = ::core::option::Option<unsafe extern "system" fn(pccab: *mut CCAB, pszfile: ::windows_sys::core::PCSTR, cbfile: i32, fcontinuation: super::super::Foundation::BOOL, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIFREE = ::core::option::Option<unsafe extern "system" fn(memory: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETNEXTCABINET = ::core::option::Option<unsafe extern "system" fn(pccab: *mut CCAB, cbprevcab: u32, pv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIGETOPENINFO = ::core::option::Option<unsafe extern "system" fn(pszname: ::windows_sys::core::PCSTR, pdate: *mut u16, ptime: *mut u16, pattribs: *mut u16, err: *mut i32, pv: *mut ::core::ffi::c_void) -> isize>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETTEMPFILE = ::core::option::Option<unsafe extern "system" fn(psztempname: ::windows_sys::core::PSTR, cbtempname: i32, pv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIOPEN = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows_sys::core::PCSTR, oflag: i32, pmode: i32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> isize>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIREAD = ::core::option::Option<unsafe extern "system" fn(hf: isize, memory: *mut ::core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCISEEK = ::core::option::Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCISTATUS = ::core::option::Option<unsafe extern "system" fn(typestatus: u32, cb1: u32, cb2: u32, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIWRITE = ::core::option::Option<unsafe extern "system" fn(hf: isize, memory: *mut ::core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFDIDECRYPT = ::core::option::Option<unsafe extern "system" fn(pfdid: *mut FDIDECRYPT) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFDINOTIFY = ::core::option::Option<unsafe extern "system" fn(fdint: FDINOTIFICATIONTYPE, pfdin: *mut FDINOTIFICATION) -> isize>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFREE = ::core::option::Option<unsafe extern "system" fn(pv: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNOPEN = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows_sys::core::PCSTR, oflag: i32, pmode: i32) -> isize>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNREAD = ::core::option::Option<unsafe extern "system" fn(hf: isize, pv: *mut ::core::ffi::c_void, cb: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNSEEK = ::core::option::Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNWRITE = ::core::option::Option<unsafe extern "system" fn(hf: isize, pv: *const ::core::ffi::c_void, cb: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const _A_EXEC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const _A_NAME_IS_UTF: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const statusCabinet: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const statusFile: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const statusFolder: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompBAD: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompLZX_WINDOW_HI: u32 = 5376u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompLZX_WINDOW_LO: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompMASK_LZX_WINDOW: u32 = 7936u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompMASK_QUANTUM_LEVEL: u32 = 240u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompMASK_QUANTUM_MEM: u32 = 7936u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompMASK_RESERVED: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompMASK_TYPE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompQUANTUM_LEVEL_HI: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompQUANTUM_LEVEL_LO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompQUANTUM_MEM_HI: u32 = 5376u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompQUANTUM_MEM_LO: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompSHIFT_LZX_WINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompSHIFT_QUANTUM_LEVEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompSHIFT_QUANTUM_MEM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompTYPE_LZX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompTYPE_MSZIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const tcompTYPE_QUANTUM: u32 = 2u32;
