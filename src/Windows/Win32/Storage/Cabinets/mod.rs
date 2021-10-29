#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const CB_MAX_CABINET_NAME: u32 = 256u32;
pub const CB_MAX_CAB_PATH: u32 = 256u32;
pub const CB_MAX_DISK: i32 = 2147483647i32;
pub const CB_MAX_DISK_NAME: u32 = 256u32;
pub const CB_MAX_FILENAME: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CCAB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CCAB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CCAB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCAB")
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
impl ::std::cmp::PartialEq for CCAB {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.cbFolderThresh == other.cbFolderThresh
            && self.cbReserveCFHeader == other.cbReserveCFHeader
            && self.cbReserveCFFolder == other.cbReserveCFFolder
            && self.cbReserveCFData == other.cbReserveCFData
            && self.iCab == other.iCab
            && self.iDisk == other.iDisk
            && self.fFailOnIncompressible == other.fFailOnIncompressible
            && self.setID == other.setID
            && self.szDisk == other.szDisk
            && self.szCab == other.szCab
            && self.szCabPath == other.szCabPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CCAB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CCAB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ERF {
    pub erfOper: i32,
    pub erfType: i32,
    pub fError: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ERF {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ERF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ERF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ERF")
            .field("erfOper", &self.erfOper)
            .field("erfType", &self.erfType)
            .field("fError", &self.fError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ERF {
    fn eq(&self, other: &Self) -> bool {
        self.erfOper == other.erfOper
            && self.erfType == other.erfType
            && self.fError == other.fError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ERF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ERF {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIAddFile<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfci: *const ::std::ffi::c_void,
    pszsourcefile: Param1,
    pszfilename: Param2,
    fexecute: Param3,
    pfnfcignc: ::std::option::Option<PFNFCIGETNEXTCABINET>,
    pfnfcis: ::std::option::Option<PFNFCISTATUS>,
    pfnfcigoi: ::std::option::Option<PFNFCIGETOPENINFO>,
    typecompress: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIAddFile(
                hfci: *const ::std::ffi::c_void,
                pszsourcefile: super::super::Foundation::PSTR,
                pszfilename: super::super::Foundation::PSTR,
                fexecute: super::super::Foundation::BOOL,
                pfnfcignc: ::windows::runtime::RawPtr,
                pfnfcis: ::windows::runtime::RawPtr,
                pfnfcigoi: ::windows::runtime::RawPtr,
                typecompress: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FCIAddFile(
            ::std::mem::transmute(hfci),
            pszsourcefile.into_param().abi(),
            pszfilename.into_param().abi(),
            fexecute.into_param().abi(),
            ::std::mem::transmute(pfnfcignc),
            ::std::mem::transmute(pfnfcis),
            ::std::mem::transmute(pfnfcigoi),
            ::std::mem::transmute(typecompress),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCICreate(
    perf: *const ERF,
    pfnfcifp: ::std::option::Option<PFNFCIFILEPLACED>,
    pfna: ::std::option::Option<PFNFCIALLOC>,
    pfnf: ::std::option::Option<PFNFCIFREE>,
    pfnopen: ::std::option::Option<PFNFCIOPEN>,
    pfnread: ::std::option::Option<PFNFCIREAD>,
    pfnwrite: ::std::option::Option<PFNFCIWRITE>,
    pfnclose: ::std::option::Option<PFNFCICLOSE>,
    pfnseek: ::std::option::Option<PFNFCISEEK>,
    pfndelete: ::std::option::Option<PFNFCIDELETE>,
    pfnfcigtf: ::std::option::Option<PFNFCIGETTEMPFILE>,
    pccab: *const CCAB,
    pv: *const ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCICreate(
                perf: *const ERF,
                pfnfcifp: ::windows::runtime::RawPtr,
                pfna: ::windows::runtime::RawPtr,
                pfnf: ::windows::runtime::RawPtr,
                pfnopen: ::windows::runtime::RawPtr,
                pfnread: ::windows::runtime::RawPtr,
                pfnwrite: ::windows::runtime::RawPtr,
                pfnclose: ::windows::runtime::RawPtr,
                pfnseek: ::windows::runtime::RawPtr,
                pfndelete: ::windows::runtime::RawPtr,
                pfnfcigtf: ::windows::runtime::RawPtr,
                pccab: *const CCAB,
                pv: *const ::std::ffi::c_void,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(FCICreate(
            ::std::mem::transmute(perf),
            ::std::mem::transmute(pfnfcifp),
            ::std::mem::transmute(pfna),
            ::std::mem::transmute(pfnf),
            ::std::mem::transmute(pfnopen),
            ::std::mem::transmute(pfnread),
            ::std::mem::transmute(pfnwrite),
            ::std::mem::transmute(pfnclose),
            ::std::mem::transmute(pfnseek),
            ::std::mem::transmute(pfndelete),
            ::std::mem::transmute(pfnfcigtf),
            ::std::mem::transmute(pccab),
            ::std::mem::transmute(pv),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIDestroy(hfci: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIDestroy(hfci: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FCIDestroy(::std::mem::transmute(hfci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for FCIERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FCIERROR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIFlushCabinet<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfci: *const ::std::ffi::c_void,
    fgetnextcab: Param1,
    pfnfcignc: ::std::option::Option<PFNFCIGETNEXTCABINET>,
    pfnfcis: ::std::option::Option<PFNFCISTATUS>,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIFlushCabinet(
                hfci: *const ::std::ffi::c_void,
                fgetnextcab: super::super::Foundation::BOOL,
                pfnfcignc: ::windows::runtime::RawPtr,
                pfnfcis: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FCIFlushCabinet(
            ::std::mem::transmute(hfci),
            fgetnextcab.into_param().abi(),
            ::std::mem::transmute(pfnfcignc),
            ::std::mem::transmute(pfnfcis),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIFlushFolder(
    hfci: *const ::std::ffi::c_void,
    pfnfcignc: ::std::option::Option<PFNFCIGETNEXTCABINET>,
    pfnfcis: ::std::option::Option<PFNFCISTATUS>,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FCIFlushFolder(
                hfci: *const ::std::ffi::c_void,
                pfnfcignc: ::windows::runtime::RawPtr,
                pfnfcis: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FCIFlushFolder(
            ::std::mem::transmute(hfci),
            ::std::mem::transmute(pfnfcignc),
            ::std::mem::transmute(pfnfcis),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl FDICABINETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FDICABINETINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FDICABINETINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FDICABINETINFO")
            .field("cbCabinet", &self.cbCabinet)
            .field("cFolders", &self.cFolders)
            .field("cFiles", &self.cFiles)
            .field("setID", &self.setID)
            .field("iCabinet", &self.iCabinet)
            .field("fReserve", &self.fReserve)
            .field("hasprev", &self.hasprev)
            .field("hasnext", &self.hasnext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FDICABINETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbCabinet == other.cbCabinet
            && self.cFolders == other.cFolders
            && self.cFiles == other.cFiles
            && self.setID == other.setID
            && self.iCabinet == other.iCabinet
            && self.fReserve == other.fReserve
            && self.hasprev == other.hasprev
            && self.hasnext == other.hasnext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FDICABINETINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FDICABINETINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FDICREATE_CPU_TYPE(pub u32);
pub const cpu80286: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(0u32);
pub const cpu80386: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(1u32);
impl ::std::convert::From<u32> for FDICREATE_CPU_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FDICREATE_CPU_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FDICREATE_CPU_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FDICREATE_CPU_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FDICREATE_CPU_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FDICREATE_CPU_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FDICREATE_CPU_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDICopy<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hfdi: *const ::std::ffi::c_void,
    pszcabinet: Param1,
    pszcabpath: Param2,
    flags: i32,
    pfnfdin: ::std::option::Option<PFNFDINOTIFY>,
    pfnfdid: ::std::option::Option<PFNFDIDECRYPT>,
    pvuser: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDICopy(
                hfdi: *const ::std::ffi::c_void,
                pszcabinet: super::super::Foundation::PSTR,
                pszcabpath: super::super::Foundation::PSTR,
                flags: i32,
                pfnfdin: ::windows::runtime::RawPtr,
                pfnfdid: ::windows::runtime::RawPtr,
                pvuser: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FDICopy(
            ::std::mem::transmute(hfdi),
            pszcabinet.into_param().abi(),
            pszcabpath.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pfnfdin),
            ::std::mem::transmute(pfnfdid),
            ::std::mem::transmute(pvuser),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDICreate(
    pfnalloc: ::std::option::Option<PFNALLOC>,
    pfnfree: ::std::option::Option<PFNFREE>,
    pfnopen: ::std::option::Option<PFNOPEN>,
    pfnread: ::std::option::Option<PFNREAD>,
    pfnwrite: ::std::option::Option<PFNWRITE>,
    pfnclose: ::std::option::Option<PFNCLOSE>,
    pfnseek: ::std::option::Option<PFNSEEK>,
    cputype: FDICREATE_CPU_TYPE,
    perf: *mut ERF,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDICreate(
                pfnalloc: ::windows::runtime::RawPtr,
                pfnfree: ::windows::runtime::RawPtr,
                pfnopen: ::windows::runtime::RawPtr,
                pfnread: ::windows::runtime::RawPtr,
                pfnwrite: ::windows::runtime::RawPtr,
                pfnclose: ::windows::runtime::RawPtr,
                pfnseek: ::windows::runtime::RawPtr,
                cputype: FDICREATE_CPU_TYPE,
                perf: *mut ERF,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(FDICreate(
            ::std::mem::transmute(pfnalloc),
            ::std::mem::transmute(pfnfree),
            ::std::mem::transmute(pfnopen),
            ::std::mem::transmute(pfnread),
            ::std::mem::transmute(pfnwrite),
            ::std::mem::transmute(pfnclose),
            ::std::mem::transmute(pfnseek),
            ::std::mem::transmute(cputype),
            ::std::mem::transmute(perf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDIDECRYPT {
    pub fdidt: FDIDECRYPTTYPE,
    pub pvUser: *mut ::std::ffi::c_void,
    pub Anonymous: FDIDECRYPT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl FDIDECRYPT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FDIDECRYPT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FDIDECRYPT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FDIDECRYPT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FDIDECRYPT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union FDIDECRYPT_0 {
    pub cabinet: FDIDECRYPT_0_0,
    pub folder: FDIDECRYPT_0_2,
    pub decrypt: FDIDECRYPT_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl FDIDECRYPT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FDIDECRYPT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FDIDECRYPT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FDIDECRYPT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FDIDECRYPT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FDIDECRYPT_0_0 {
    pub pHeaderReserve: *mut ::std::ffi::c_void,
    pub cbHeaderReserve: u16,
    pub setID: u16,
    pub iCabinet: i32,
}
impl FDIDECRYPT_0_0 {}
impl ::std::default::Default for FDIDECRYPT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FDIDECRYPT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_cabinet_e__Struct")
            .field("pHeaderReserve", &self.pHeaderReserve)
            .field("cbHeaderReserve", &self.cbHeaderReserve)
            .field("setID", &self.setID)
            .field("iCabinet", &self.iCabinet)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FDIDECRYPT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pHeaderReserve == other.pHeaderReserve
            && self.cbHeaderReserve == other.cbHeaderReserve
            && self.setID == other.setID
            && self.iCabinet == other.iCabinet
    }
}
impl ::std::cmp::Eq for FDIDECRYPT_0_0 {}
unsafe impl ::windows::runtime::Abi for FDIDECRYPT_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDIDECRYPT_0_1 {
    pub pDataReserve: *mut ::std::ffi::c_void,
    pub cbDataReserve: u16,
    pub pbData: *mut ::std::ffi::c_void,
    pub cbData: u16,
    pub fSplit: super::super::Foundation::BOOL,
    pub cbPartial: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl FDIDECRYPT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FDIDECRYPT_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FDIDECRYPT_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_decrypt_e__Struct")
            .field("pDataReserve", &self.pDataReserve)
            .field("cbDataReserve", &self.cbDataReserve)
            .field("pbData", &self.pbData)
            .field("cbData", &self.cbData)
            .field("fSplit", &self.fSplit)
            .field("cbPartial", &self.cbPartial)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FDIDECRYPT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pDataReserve == other.pDataReserve
            && self.cbDataReserve == other.cbDataReserve
            && self.pbData == other.pbData
            && self.cbData == other.cbData
            && self.fSplit == other.fSplit
            && self.cbPartial == other.cbPartial
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FDIDECRYPT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FDIDECRYPT_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FDIDECRYPT_0_2 {
    pub pFolderReserve: *mut ::std::ffi::c_void,
    pub cbFolderReserve: u16,
    pub iFolder: u16,
}
impl FDIDECRYPT_0_2 {}
impl ::std::default::Default for FDIDECRYPT_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FDIDECRYPT_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_folder_e__Struct")
            .field("pFolderReserve", &self.pFolderReserve)
            .field("cbFolderReserve", &self.cbFolderReserve)
            .field("iFolder", &self.iFolder)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FDIDECRYPT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pFolderReserve == other.pFolderReserve
            && self.cbFolderReserve == other.cbFolderReserve
            && self.iFolder == other.iFolder
    }
}
impl ::std::cmp::Eq for FDIDECRYPT_0_2 {}
unsafe impl ::windows::runtime::Abi for FDIDECRYPT_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FDIDECRYPTTYPE(pub i32);
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = FDIDECRYPTTYPE(0i32);
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = FDIDECRYPTTYPE(1i32);
pub const fdidtDECRYPT: FDIDECRYPTTYPE = FDIDECRYPTTYPE(2i32);
impl ::std::convert::From<i32> for FDIDECRYPTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FDIDECRYPTTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDIDestroy(hfdi: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDIDestroy(hfdi: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FDIDestroy(::std::mem::transmute(hfdi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for FDIERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FDIERROR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDIIsCabinet(
    hfdi: *const ::std::ffi::c_void,
    hf: isize,
    pfdici: *mut FDICABINETINFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDIIsCabinet(
                hfdi: *const ::std::ffi::c_void,
                hf: isize,
                pfdici: *mut FDICABINETINFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FDIIsCabinet(
            ::std::mem::transmute(hfdi),
            ::std::mem::transmute(hf),
            ::std::mem::transmute(pfdici),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDINOTIFICATION {
    pub cb: i32,
    pub psz1: super::super::Foundation::PSTR,
    pub psz2: super::super::Foundation::PSTR,
    pub psz3: super::super::Foundation::PSTR,
    pub pv: *mut ::std::ffi::c_void,
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
impl FDINOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FDINOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FDINOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FDINOTIFICATION")
            .field("cb", &self.cb)
            .field("psz1", &self.psz1)
            .field("psz2", &self.psz2)
            .field("psz3", &self.psz3)
            .field("pv", &self.pv)
            .field("hf", &self.hf)
            .field("date", &self.date)
            .field("time", &self.time)
            .field("attribs", &self.attribs)
            .field("setID", &self.setID)
            .field("iCabinet", &self.iCabinet)
            .field("iFolder", &self.iFolder)
            .field("fdie", &self.fdie)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FDINOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.psz1 == other.psz1
            && self.psz2 == other.psz2
            && self.psz3 == other.psz3
            && self.pv == other.pv
            && self.hf == other.hf
            && self.date == other.date
            && self.time == other.time
            && self.attribs == other.attribs
            && self.setID == other.setID
            && self.iCabinet == other.iCabinet
            && self.iFolder == other.iFolder
            && self.fdie == other.fdie
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FDINOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FDINOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FDINOTIFICATIONTYPE(pub i32);
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(0i32);
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(1i32);
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(2i32);
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(3i32);
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(4i32);
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(5i32);
impl ::std::convert::From<i32> for FDINOTIFICATIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FDINOTIFICATIONTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FDISPILLFILE {
    pub ach: [super::super::Foundation::CHAR; 2],
    pub cbFile: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl FDISPILLFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FDISPILLFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FDISPILLFILE")
            .field("ach", &self.ach)
            .field("cbFile", &self.cbFile)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FDISPILLFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ach == other.ach && self.cbFile == other.cbFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FDISPILLFILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FDISPILLFILE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDITruncateCabinet<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hfdi: *const ::std::ffi::c_void,
    pszcabinetname: Param1,
    ifoldertodelete: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FDITruncateCabinet(
                hfdi: *const ::std::ffi::c_void,
                pszcabinetname: super::super::Foundation::PSTR,
                ifoldertodelete: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FDITruncateCabinet(
            ::std::mem::transmute(hfdi),
            pszcabinetname.into_param().abi(),
            ::std::mem::transmute(ifoldertodelete),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const INCLUDED_FCI: u32 = 1u32;
pub const INCLUDED_FDI: u32 = 1u32;
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
pub type PFNALLOC = unsafe extern "system" fn(cb: u32) -> *mut ::std::ffi::c_void;
pub type PFNCLOSE = unsafe extern "system" fn(hf: isize) -> i32;
pub type PFNFCIALLOC = unsafe extern "system" fn(cb: u32) -> *mut ::std::ffi::c_void;
pub type PFNFCICLOSE =
    unsafe extern "system" fn(hf: isize, err: *mut i32, pv: *mut ::std::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIDELETE = unsafe extern "system" fn(
    pszfile: super::super::Foundation::PSTR,
    err: *mut i32,
    pv: *mut ::std::ffi::c_void,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIFILEPLACED = unsafe extern "system" fn(
    pccab: *mut CCAB,
    pszfile: super::super::Foundation::PSTR,
    cbfile: i32,
    fcontinuation: super::super::Foundation::BOOL,
    pv: *mut ::std::ffi::c_void,
) -> i32;
pub type PFNFCIFREE = unsafe extern "system" fn(memory: *mut ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETNEXTCABINET = unsafe extern "system" fn(
    pccab: *mut CCAB,
    cbprevcab: u32,
    pv: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETOPENINFO = unsafe extern "system" fn(
    pszname: super::super::Foundation::PSTR,
    pdate: *mut u16,
    ptime: *mut u16,
    pattribs: *mut u16,
    err: *mut i32,
    pv: *mut ::std::ffi::c_void,
) -> isize;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIGETTEMPFILE = unsafe extern "system" fn(
    psztempname: super::super::Foundation::PSTR,
    cbtempname: i32,
    pv: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFCIOPEN = unsafe extern "system" fn(
    pszfile: super::super::Foundation::PSTR,
    oflag: i32,
    pmode: i32,
    err: *mut i32,
    pv: *mut ::std::ffi::c_void,
) -> isize;
pub type PFNFCIREAD = unsafe extern "system" fn(
    hf: isize,
    memory: *mut ::std::ffi::c_void,
    cb: u32,
    err: *mut i32,
    pv: *mut ::std::ffi::c_void,
) -> u32;
pub type PFNFCISEEK = unsafe extern "system" fn(
    hf: isize,
    dist: i32,
    seektype: i32,
    err: *mut i32,
    pv: *mut ::std::ffi::c_void,
) -> i32;
pub type PFNFCISTATUS = unsafe extern "system" fn(
    typestatus: u32,
    cb1: u32,
    cb2: u32,
    pv: *mut ::std::ffi::c_void,
) -> i32;
pub type PFNFCIWRITE = unsafe extern "system" fn(
    hf: isize,
    memory: *mut ::std::ffi::c_void,
    cb: u32,
    err: *mut i32,
    pv: *mut ::std::ffi::c_void,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFDIDECRYPT = unsafe extern "system" fn(pfdid: *mut FDIDECRYPT) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFDINOTIFY =
    unsafe extern "system" fn(fdint: FDINOTIFICATIONTYPE, pfdin: *mut FDINOTIFICATION) -> isize;
pub type PFNFREE = unsafe extern "system" fn(pv: *const ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFNOPEN = unsafe extern "system" fn(
    pszfile: super::super::Foundation::PSTR,
    oflag: i32,
    pmode: i32,
) -> isize;
pub type PFNREAD =
    unsafe extern "system" fn(hf: isize, pv: *mut ::std::ffi::c_void, cb: u32) -> u32;
pub type PFNSEEK = unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32) -> i32;
pub type PFNWRITE =
    unsafe extern "system" fn(hf: isize, pv: *const ::std::ffi::c_void, cb: u32) -> u32;
pub const _A_EXEC: u32 = 64u32;
pub const _A_NAME_IS_UTF: u32 = 128u32;
