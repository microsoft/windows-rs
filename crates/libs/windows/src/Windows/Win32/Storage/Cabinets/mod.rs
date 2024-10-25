pub const CB_MAX_CABINET_NAME: u32 = 256u32;
pub const CB_MAX_CAB_PATH: u32 = 256u32;
pub const CB_MAX_DISK: i32 = 2147483647i32;
pub const CB_MAX_DISK_NAME: u32 = 256u32;
pub const CB_MAX_FILENAME: u32 = 256u32;
pub const FCIERR_ALLOC_FAIL: FCIERROR = 3i32;
pub const FCIERR_BAD_COMPR_TYPE: FCIERROR = 5i32;
pub const FCIERR_CAB_FILE: FCIERROR = 6i32;
pub const FCIERR_CAB_FORMAT_LIMIT: FCIERROR = 9i32;
pub const FCIERR_MCI_FAIL: FCIERROR = 8i32;
pub const FCIERR_NONE: FCIERROR = 0i32;
pub const FCIERR_OPEN_SRC: FCIERROR = 1i32;
pub const FCIERR_READ_SRC: FCIERROR = 2i32;
pub const FCIERR_TEMP_FILE: FCIERROR = 4i32;
pub const FCIERR_USER_ABORT: FCIERROR = 7i32;
pub const FDIERROR_ALLOC_FAIL: FDIERROR = 5i32;
pub const FDIERROR_BAD_COMPR_TYPE: FDIERROR = 6i32;
pub const FDIERROR_CABINET_NOT_FOUND: FDIERROR = 1i32;
pub const FDIERROR_CORRUPT_CABINET: FDIERROR = 4i32;
pub const FDIERROR_EOF: FDIERROR = 12i32;
pub const FDIERROR_MDI_FAIL: FDIERROR = 7i32;
pub const FDIERROR_NONE: FDIERROR = 0i32;
pub const FDIERROR_NOT_A_CABINET: FDIERROR = 2i32;
pub const FDIERROR_RESERVE_MISMATCH: FDIERROR = 9i32;
pub const FDIERROR_TARGET_FILE: FDIERROR = 8i32;
pub const FDIERROR_UNKNOWN_CABINET_VERSION: FDIERROR = 3i32;
pub const FDIERROR_USER_ABORT: FDIERROR = 11i32;
pub const FDIERROR_WRONG_CABINET: FDIERROR = 10i32;
pub const INCLUDED_FCI: u32 = 1u32;
pub const INCLUDED_FDI: u32 = 1u32;
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
pub const _A_EXEC: u32 = 64u32;
pub const _A_NAME_IS_UTF: u32 = 128u32;
pub const cpu80286: FDICREATE_CPU_TYPE = 0i32;
pub const cpu80386: FDICREATE_CPU_TYPE = 1i32;
pub const cpuUNKNOWN: FDICREATE_CPU_TYPE = -1i32;
pub const fdidtDECRYPT: FDIDECRYPTTYPE = 2i32;
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = 0i32;
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = 1i32;
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = 0i32;
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = 3i32;
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = 2i32;
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = 5i32;
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = 4i32;
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = 1i32;
pub const statusCabinet: u32 = 2u32;
pub const statusFile: u32 = 0u32;
pub const statusFolder: u32 = 1u32;
pub const tcompBAD: u32 = 15u32;
pub const tcompLZX_WINDOW_HI: u32 = 5376u32;
pub const tcompLZX_WINDOW_LO: u32 = 3840u32;
pub const tcompMASK_LZX_WINDOW: u32 = 7936u32;
pub const tcompMASK_QUANTUM_LEVEL: u32 = 240u32;
pub const tcompMASK_QUANTUM_MEM: u32 = 7936u32;
pub const tcompMASK_RESERVED: u32 = 57344u32;
pub const tcompMASK_TYPE: u32 = 15u32;
pub const tcompQUANTUM_LEVEL_HI: u32 = 112u32;
pub const tcompQUANTUM_LEVEL_LO: u32 = 16u32;
pub const tcompQUANTUM_MEM_HI: u32 = 5376u32;
pub const tcompQUANTUM_MEM_LO: u32 = 2560u32;
pub const tcompSHIFT_LZX_WINDOW: u32 = 8u32;
pub const tcompSHIFT_QUANTUM_LEVEL: u32 = 4u32;
pub const tcompSHIFT_QUANTUM_MEM: u32 = 8u32;
pub const tcompTYPE_LZX: u32 = 3u32;
pub const tcompTYPE_MSZIP: u32 = 1u32;
pub const tcompTYPE_NONE: u32 = 0u32;
pub const tcompTYPE_QUANTUM: u32 = 2u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FCIERROR(pub i32);
impl windows_core::TypeKind for FCIERROR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDICREATE_CPU_TYPE(pub i32);
impl windows_core::TypeKind for FDICREATE_CPU_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDIDECRYPTTYPE(pub i32);
impl windows_core::TypeKind for FDIDECRYPTTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDIERROR(pub i32);
impl windows_core::TypeKind for FDIERROR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FDINOTIFICATIONTYPE(pub i32);
impl windows_core::TypeKind for FDINOTIFICATIONTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub szDisk: [i8; 256],
    pub szCab: [i8; 256],
    pub szCabPath: [i8; 256],
}
impl Default for CCAB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CCAB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ERF {
    pub erfOper: i32,
    pub erfType: i32,
    pub fError: super::super::Foundation::BOOL,
}
impl Default for ERF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ERF {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for FDICABINETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDICABINETINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT {
    pub fdidt: FDIDECRYPTTYPE,
    pub pvUser: *mut core::ffi::c_void,
    pub Anonymous: FDIDECRYPT_0,
}
impl Default for FDIDECRYPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDIDECRYPT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union FDIDECRYPT_0 {
    pub cabinet: FDIDECRYPT_0_0,
    pub folder: FDIDECRYPT_0_1,
    pub decrypt: FDIDECRYPT_0_2,
}
impl Default for FDIDECRYPT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDIDECRYPT_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT_0_0 {
    pub pHeaderReserve: *mut core::ffi::c_void,
    pub cbHeaderReserve: u16,
    pub setID: u16,
    pub iCabinet: i32,
}
impl Default for FDIDECRYPT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDIDECRYPT_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT_0_2 {
    pub pDataReserve: *mut core::ffi::c_void,
    pub cbDataReserve: u16,
    pub pbData: *mut core::ffi::c_void,
    pub cbData: u16,
    pub fSplit: super::super::Foundation::BOOL,
    pub cbPartial: u16,
}
impl Default for FDIDECRYPT_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDIDECRYPT_0_2 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT_0_1 {
    pub pFolderReserve: *mut core::ffi::c_void,
    pub cbFolderReserve: u16,
    pub iFolder: u16,
}
impl Default for FDIDECRYPT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDIDECRYPT_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDINOTIFICATION {
    pub cb: i32,
    pub psz1: windows_core::PSTR,
    pub psz2: windows_core::PSTR,
    pub psz3: windows_core::PSTR,
    pub pv: *mut core::ffi::c_void,
    pub hf: isize,
    pub date: u16,
    pub time: u16,
    pub attribs: u16,
    pub setID: u16,
    pub iCabinet: u16,
    pub iFolder: u16,
    pub fdie: FDIERROR,
}
impl Default for FDINOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FDINOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDISPILLFILE {
    pub ach: [i8; 2],
    pub cbFile: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for FDISPILLFILE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDISPILLFILE {
    pub ach: [i8; 2],
    pub cbFile: i32,
}
#[cfg(target_arch = "x86")]
impl Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for FDISPILLFILE {
    type TypeKind = windows_core::CopyType;
}
pub type PFNALLOC = Option<unsafe extern "system" fn(cb: u32) -> *mut core::ffi::c_void>;
pub type PFNCLOSE = Option<unsafe extern "system" fn(hf: isize) -> i32>;
pub type PFNFCIALLOC = Option<unsafe extern "system" fn(cb: u32) -> *mut core::ffi::c_void>;
pub type PFNFCICLOSE = Option<unsafe extern "system" fn(hf: isize, err: *mut i32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIDELETE = Option<unsafe extern "system" fn(pszfile: windows_core::PCSTR, err: *mut i32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIFILEPLACED = Option<unsafe extern "system" fn(pccab: *mut CCAB, pszfile: windows_core::PCSTR, cbfile: i32, fcontinuation: super::super::Foundation::BOOL, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIFREE = Option<unsafe extern "system" fn(memory: *mut core::ffi::c_void)>;
pub type PFNFCIGETNEXTCABINET = Option<unsafe extern "system" fn(pccab: *mut CCAB, cbprevcab: u32, pv: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PFNFCIGETOPENINFO = Option<unsafe extern "system" fn(pszname: windows_core::PCSTR, pdate: *mut u16, ptime: *mut u16, pattribs: *mut u16, err: *mut i32, pv: *mut core::ffi::c_void) -> isize>;
pub type PFNFCIGETTEMPFILE = Option<unsafe extern "system" fn(psztempname: windows_core::PSTR, cbtempname: i32, pv: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PFNFCIOPEN = Option<unsafe extern "system" fn(pszfile: windows_core::PCSTR, oflag: i32, pmode: i32, err: *mut i32, pv: *mut core::ffi::c_void) -> isize>;
pub type PFNFCIREAD = Option<unsafe extern "system" fn(hf: isize, memory: *mut core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut core::ffi::c_void) -> u32>;
pub type PFNFCISEEK = Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32, err: *mut i32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCISTATUS = Option<unsafe extern "system" fn(typestatus: u32, cb1: u32, cb2: u32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIWRITE = Option<unsafe extern "system" fn(hf: isize, memory: *mut core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut core::ffi::c_void) -> u32>;
pub type PFNFDIDECRYPT = Option<unsafe extern "system" fn(pfdid: *mut FDIDECRYPT) -> i32>;
pub type PFNFDINOTIFY = Option<unsafe extern "system" fn(fdint: FDINOTIFICATIONTYPE, pfdin: *mut FDINOTIFICATION) -> isize>;
pub type PFNFREE = Option<unsafe extern "system" fn(pv: *const core::ffi::c_void)>;
pub type PFNOPEN = Option<unsafe extern "system" fn(pszfile: windows_core::PCSTR, oflag: i32, pmode: i32) -> isize>;
pub type PFNREAD = Option<unsafe extern "system" fn(hf: isize, pv: *mut core::ffi::c_void, cb: u32) -> u32>;
pub type PFNSEEK = Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32) -> i32>;
pub type PFNWRITE = Option<unsafe extern "system" fn(hf: isize, pv: *const core::ffi::c_void, cb: u32) -> u32>;
