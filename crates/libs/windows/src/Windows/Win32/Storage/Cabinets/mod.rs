#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CCAB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCAB")
            .field("cb", &self.cb)
            .field("cbFolderThresh", &self.cbFolderThresh)
            .field("cbReserveCFHeader", &self.cbReserveCFHeader)
            .field("cbReserveCFFolder", &self.cbReserveCFFolder)
            .field("cbReserveCFData", &self.cbReserveCFData)
            .field("iCab", &self.iCab)
            .field("iDisk", &self.iDisk)
            .field("fFailOnIncompressible", &self.fFailOnIncompressible)
            .field("setID", &self.setID)
            .field("szDisk", &self.szDisk)
            .field("szCab", &self.szCab)
            .field("szCabPath", &self.szCabPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CCAB {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CCAB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCAB>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CCAB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CCAB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ERF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERF").field("erfOper", &self.erfOper).field("erfType", &self.erfType).field("fError", &self.fError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ERF {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ERF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ERF>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ERF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ERF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIAddFile<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hfci: *const ::core::ffi::c_void, pszsourcefile: Param1, pszfilename: Param2, fexecute: Param3, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS, pfnfcigoi: PFNFCIGETOPENINFO, typecompress: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIAddFile(hfci: *const ::core::ffi::c_void, pszsourcefile: ::windows::core::PCSTR, pszfilename: ::windows::core::PCSTR, fexecute: super::super::Foundation::BOOL, pfnfcignc: ::windows::core::RawPtr, pfnfcis: ::windows::core::RawPtr, pfnfcigoi: ::windows::core::RawPtr, typecompress: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FCIAddFile(::core::mem::transmute(hfci), pszsourcefile.into_param().abi(), pszfilename.into_param().abi(), fexecute.into_param().abi(), ::core::mem::transmute(pfnfcignc), ::core::mem::transmute(pfnfcis), ::core::mem::transmute(pfnfcigoi), ::core::mem::transmute(typecompress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCICreate(perf: *const ERF, pfnfcifp: PFNFCIFILEPLACED, pfna: PFNFCIALLOC, pfnf: PFNFCIFREE, pfnopen: PFNFCIOPEN, pfnread: PFNFCIREAD, pfnwrite: PFNFCIWRITE, pfnclose: PFNFCICLOSE, pfnseek: PFNFCISEEK, pfndelete: PFNFCIDELETE, pfnfcigtf: PFNFCIGETTEMPFILE, pccab: *const CCAB, pv: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCICreate(perf: *const ERF, pfnfcifp: ::windows::core::RawPtr, pfna: ::windows::core::RawPtr, pfnf: ::windows::core::RawPtr, pfnopen: ::windows::core::RawPtr, pfnread: ::windows::core::RawPtr, pfnwrite: ::windows::core::RawPtr, pfnclose: ::windows::core::RawPtr, pfnseek: ::windows::core::RawPtr, pfndelete: ::windows::core::RawPtr, pfnfcigtf: ::windows::core::RawPtr, pccab: *const CCAB, pv: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FCICreate(::core::mem::transmute(perf), ::core::mem::transmute(pfnfcifp), ::core::mem::transmute(pfna), ::core::mem::transmute(pfnf), ::core::mem::transmute(pfnopen), ::core::mem::transmute(pfnread), ::core::mem::transmute(pfnwrite), ::core::mem::transmute(pfnclose), ::core::mem::transmute(pfnseek), ::core::mem::transmute(pfndelete), ::core::mem::transmute(pfnfcigtf), ::core::mem::transmute(pccab), ::core::mem::transmute(pv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FCIDestroy(::core::mem::transmute(hfci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FCIERROR(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_NONE: FCIERROR = FCIERROR(0i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_OPEN_SRC: FCIERROR = FCIERROR(1i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_READ_SRC: FCIERROR = FCIERROR(2i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_ALLOC_FAIL: FCIERROR = FCIERROR(3i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_TEMP_FILE: FCIERROR = FCIERROR(4i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_BAD_COMPR_TYPE: FCIERROR = FCIERROR(5i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_CAB_FILE: FCIERROR = FCIERROR(6i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_USER_ABORT: FCIERROR = FCIERROR(7i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_MCI_FAIL: FCIERROR = FCIERROR(8i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FCIERR_CAB_FORMAT_LIMIT: FCIERROR = FCIERROR(9i32);
impl ::core::marker::Copy for FCIERROR {}
impl ::core::clone::Clone for FCIERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FCIERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FCIERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for FCIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FCIERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIFlushCabinet<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hfci: *const ::core::ffi::c_void, fgetnextcab: Param1, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIFlushCabinet(hfci: *const ::core::ffi::c_void, fgetnextcab: super::super::Foundation::BOOL, pfnfcignc: ::windows::core::RawPtr, pfnfcis: ::windows::core::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FCIFlushCabinet(::core::mem::transmute(hfci), fgetnextcab.into_param().abi(), ::core::mem::transmute(pfnfcignc), ::core::mem::transmute(pfnfcis)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: ::windows::core::RawPtr, pfnfcis: ::windows::core::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FCIFlushFolder(::core::mem::transmute(hfci), ::core::mem::transmute(pfnfcignc), ::core::mem::transmute(pfnfcis)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FDICABINETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDICABINETINFO").field("cbCabinet", &self.cbCabinet).field("cFolders", &self.cFolders).field("cFiles", &self.cFiles).field("setID", &self.setID).field("iCabinet", &self.iCabinet).field("fReserve", &self.fReserve).field("hasprev", &self.hasprev).field("hasnext", &self.hasnext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDICABINETINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDICABINETINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDICABINETINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDICABINETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDICABINETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FDICREATE_CPU_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const cpuUNKNOWN: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(-1i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const cpu80286: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const cpu80386: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(1i32);
impl ::core::marker::Copy for FDICREATE_CPU_TYPE {}
impl ::core::clone::Clone for FDICREATE_CPU_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FDICREATE_CPU_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FDICREATE_CPU_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FDICREATE_CPU_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDICREATE_CPU_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDICopy<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hfdi: *const ::core::ffi::c_void, pszcabinet: Param1, pszcabpath: Param2, flags: i32, pfnfdin: PFNFDINOTIFY, pfnfdid: PFNFDIDECRYPT, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDICopy(hfdi: *const ::core::ffi::c_void, pszcabinet: ::windows::core::PCSTR, pszcabpath: ::windows::core::PCSTR, flags: i32, pfnfdin: ::windows::core::RawPtr, pfnfdid: ::windows::core::RawPtr, pvuser: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FDICopy(::core::mem::transmute(hfdi), pszcabinet.into_param().abi(), pszcabpath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(pfnfdin), ::core::mem::transmute(pfnfdid), ::core::mem::transmute(pvuser)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDICreate(pfnalloc: PFNALLOC, pfnfree: PFNFREE, pfnopen: PFNOPEN, pfnread: PFNREAD, pfnwrite: PFNWRITE, pfnclose: PFNCLOSE, pfnseek: PFNSEEK, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDICreate(pfnalloc: ::windows::core::RawPtr, pfnfree: ::windows::core::RawPtr, pfnopen: ::windows::core::RawPtr, pfnread: ::windows::core::RawPtr, pfnwrite: ::windows::core::RawPtr, pfnclose: ::windows::core::RawPtr, pfnseek: ::windows::core::RawPtr, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FDICreate(::core::mem::transmute(pfnalloc), ::core::mem::transmute(pfnfree), ::core::mem::transmute(pfnopen), ::core::mem::transmute(pfnread), ::core::mem::transmute(pfnwrite), ::core::mem::transmute(pfnclose), ::core::mem::transmute(pfnseek), ::core::mem::transmute(cputype), ::core::mem::transmute(perf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDIDECRYPT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDIDECRYPT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDIDECRYPT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDIDECRYPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDIDECRYPT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDIDECRYPT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDIDECRYPT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDIDECRYPT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FDIDECRYPT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDIDECRYPT_0_0").field("pHeaderReserve", &self.pHeaderReserve).field("cbHeaderReserve", &self.cbHeaderReserve).field("setID", &self.setID).field("iCabinet", &self.iCabinet).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDIDECRYPT_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDIDECRYPT_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDIDECRYPT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDIDECRYPT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FDIDECRYPT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDIDECRYPT_0_1").field("pDataReserve", &self.pDataReserve).field("cbDataReserve", &self.cbDataReserve).field("pbData", &self.pbData).field("cbData", &self.cbData).field("fSplit", &self.fSplit).field("cbPartial", &self.cbPartial).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDIDECRYPT_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDIDECRYPT_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDIDECRYPT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDIDECRYPT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FDIDECRYPT_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDIDECRYPT_0_2").field("pFolderReserve", &self.pFolderReserve).field("cbFolderReserve", &self.cbFolderReserve).field("iFolder", &self.iFolder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDIDECRYPT_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDIDECRYPT_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDIDECRYPT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDIDECRYPT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FDIDECRYPTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = FDIDECRYPTTYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = FDIDECRYPTTYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdidtDECRYPT: FDIDECRYPTTYPE = FDIDECRYPTTYPE(2i32);
impl ::core::marker::Copy for FDIDECRYPTTYPE {}
impl ::core::clone::Clone for FDIDECRYPTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FDIDECRYPTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FDIDECRYPTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FDIDECRYPTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDIDECRYPTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDIDestroy(hfdi: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDIDestroy(hfdi: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FDIDestroy(::core::mem::transmute(hfdi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FDIERROR(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_NONE: FDIERROR = FDIERROR(0i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_CABINET_NOT_FOUND: FDIERROR = FDIERROR(1i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_NOT_A_CABINET: FDIERROR = FDIERROR(2i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_UNKNOWN_CABINET_VERSION: FDIERROR = FDIERROR(3i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_CORRUPT_CABINET: FDIERROR = FDIERROR(4i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_ALLOC_FAIL: FDIERROR = FDIERROR(5i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_BAD_COMPR_TYPE: FDIERROR = FDIERROR(6i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_MDI_FAIL: FDIERROR = FDIERROR(7i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_TARGET_FILE: FDIERROR = FDIERROR(8i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_RESERVE_MISMATCH: FDIERROR = FDIERROR(9i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_WRONG_CABINET: FDIERROR = FDIERROR(10i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_USER_ABORT: FDIERROR = FDIERROR(11i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const FDIERROR_EOF: FDIERROR = FDIERROR(12i32);
impl ::core::marker::Copy for FDIERROR {}
impl ::core::clone::Clone for FDIERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FDIERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FDIERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for FDIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDIERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDIIsCabinet(hfdi: *const ::core::ffi::c_void, hf: isize, pfdici: *mut FDICABINETINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDIIsCabinet(hfdi: *const ::core::ffi::c_void, hf: isize, pfdici: *mut FDICABINETINFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FDIIsCabinet(::core::mem::transmute(hfdi), ::core::mem::transmute(hf), ::core::mem::transmute(pfdici)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub struct FDINOTIFICATION {
    pub cb: i32,
    pub psz1: ::windows::core::PSTR,
    pub psz2: ::windows::core::PSTR,
    pub psz3: ::windows::core::PSTR,
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
impl ::core::fmt::Debug for FDINOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDINOTIFICATION").field("cb", &self.cb).field("psz1", &self.psz1).field("psz2", &self.psz2).field("psz3", &self.psz3).field("pv", &self.pv).field("hf", &self.hf).field("date", &self.date).field("time", &self.time).field("attribs", &self.attribs).field("setID", &self.setID).field("iCabinet", &self.iCabinet).field("iFolder", &self.iFolder).field("fdie", &self.fdie).finish()
    }
}
unsafe impl ::windows::core::Abi for FDINOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FDINOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDINOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FDINOTIFICATION {}
impl ::core::default::Default for FDINOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FDINOTIFICATIONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(5i32);
impl ::core::marker::Copy for FDINOTIFICATIONTYPE {}
impl ::core::clone::Clone for FDINOTIFICATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FDINOTIFICATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FDINOTIFICATIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FDINOTIFICATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDINOTIFICATIONTYPE").field(&self.0).finish()
    }
}
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDISPILLFILE {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDISPILLFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDISPILLFILE>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDISPILLFILE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FDISPILLFILE {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDISPILLFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FDISPILLFILE>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDISPILLFILE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDITruncateCabinet<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hfdi: *const ::core::ffi::c_void, pszcabinetname: Param1, ifoldertodelete: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDITruncateCabinet(hfdi: *const ::core::ffi::c_void, pszcabinetname: ::windows::core::PCSTR, ifoldertodelete: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FDITruncateCabinet(::core::mem::transmute(hfdi), pszcabinetname.into_param().abi(), ::core::mem::transmute(ifoldertodelete)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub type PFNFCIDELETE = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows::core::PCSTR, err: *mut i32, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIFILEPLACED = ::core::option::Option<unsafe extern "system" fn(pccab: *mut CCAB, pszfile: ::windows::core::PCSTR, cbfile: i32, fcontinuation: super::super::Foundation::BOOL, pv: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIFREE = ::core::option::Option<unsafe extern "system" fn(memory: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETNEXTCABINET = ::core::option::Option<unsafe extern "system" fn(pccab: *mut CCAB, cbprevcab: u32, pv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIGETOPENINFO = ::core::option::Option<unsafe extern "system" fn(pszname: ::windows::core::PCSTR, pdate: *mut u16, ptime: *mut u16, pattribs: *mut u16, err: *mut i32, pv: *mut ::core::ffi::c_void) -> isize>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETTEMPFILE = ::core::option::Option<unsafe extern "system" fn(psztempname: ::windows::core::PSTR, cbtempname: i32, pv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNFCIOPEN = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows::core::PCSTR, oflag: i32, pmode: i32, err: *mut i32, pv: *mut ::core::ffi::c_void) -> isize>;
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
pub type PFNOPEN = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows::core::PCSTR, oflag: i32, pmode: i32) -> isize>;
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
