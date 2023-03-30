#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIAddFile<P0, P1, P2>(hfci: *const ::core::ffi::c_void, pszsourcefile: P0, pszfilename: P1, fexecute: P2, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS, pfnfcigoi: PFNFCIGETOPENINFO, typecompress: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FCIAddFile ( hfci : *const ::core::ffi::c_void , pszsourcefile : ::windows::core::PCSTR , pszfilename : ::windows::core::PCSTR , fexecute : super::super::Foundation:: BOOL , pfnfcignc : PFNFCIGETNEXTCABINET , pfnfcis : PFNFCISTATUS , pfnfcigoi : PFNFCIGETOPENINFO , typecompress : u16 ) -> super::super::Foundation:: BOOL );
    FCIAddFile(hfci, pszsourcefile.into_param().abi(), pszfilename.into_param().abi(), fexecute.into_param().abi(), pfnfcignc, pfnfcis, pfnfcigoi, typecompress)
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCICreate(perf: *const ERF, pfnfcifp: PFNFCIFILEPLACED, pfna: PFNFCIALLOC, pfnf: PFNFCIFREE, pfnopen: PFNFCIOPEN, pfnread: PFNFCIREAD, pfnwrite: PFNFCIWRITE, pfnclose: PFNFCICLOSE, pfnseek: PFNFCISEEK, pfndelete: PFNFCIDELETE, pfnfcigtf: PFNFCIGETTEMPFILE, pccab: *const CCAB, pv: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FCICreate ( perf : *const ERF , pfnfcifp : PFNFCIFILEPLACED , pfna : PFNFCIALLOC , pfnf : PFNFCIFREE , pfnopen : PFNFCIOPEN , pfnread : PFNFCIREAD , pfnwrite : PFNFCIWRITE , pfnclose : PFNFCICLOSE , pfnseek : PFNFCISEEK , pfndelete : PFNFCIDELETE , pfnfcigtf : PFNFCIGETTEMPFILE , pccab : *const CCAB , pv : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    FCICreate(perf, pfnfcifp, pfna, pfnf, pfnopen, pfnread, pfnwrite, pfnclose, pfnseek, pfndelete, pfnfcigtf, pccab, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIDestroy(hfci: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FCIDestroy ( hfci : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FCIDestroy(hfci)
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIFlushCabinet<P0>(hfci: *const ::core::ffi::c_void, fgetnextcab: P0, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FCIFlushCabinet ( hfci : *const ::core::ffi::c_void , fgetnextcab : super::super::Foundation:: BOOL , pfnfcignc : PFNFCIGETNEXTCABINET , pfnfcis : PFNFCISTATUS ) -> super::super::Foundation:: BOOL );
    FCIFlushCabinet(hfci, fgetnextcab.into_param().abi(), pfnfcignc, pfnfcis)
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FCIFlushFolder(hfci: *const ::core::ffi::c_void, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FCIFlushFolder ( hfci : *const ::core::ffi::c_void , pfnfcignc : PFNFCIGETNEXTCABINET , pfnfcis : PFNFCISTATUS ) -> super::super::Foundation:: BOOL );
    FCIFlushFolder(hfci, pfnfcignc, pfnfcis)
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDICopy<P0, P1>(hfdi: *const ::core::ffi::c_void, pszcabinet: P0, pszcabpath: P1, flags: i32, pfnfdin: PFNFDINOTIFY, pfnfdid: PFNFDIDECRYPT, pvuser: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FDICopy ( hfdi : *const ::core::ffi::c_void , pszcabinet : ::windows::core::PCSTR , pszcabpath : ::windows::core::PCSTR , flags : i32 , pfnfdin : PFNFDINOTIFY , pfnfdid : PFNFDIDECRYPT , pvuser : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FDICopy(hfdi, pszcabinet.into_param().abi(), pszcabpath.into_param().abi(), flags, pfnfdin, pfnfdid, ::core::mem::transmute(pvuser.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDICreate(pfnalloc: PFNALLOC, pfnfree: PFNFREE, pfnopen: PFNOPEN, pfnread: PFNREAD, pfnwrite: PFNWRITE, pfnclose: PFNCLOSE, pfnseek: PFNSEEK, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FDICreate ( pfnalloc : PFNALLOC , pfnfree : PFNFREE , pfnopen : PFNOPEN , pfnread : PFNREAD , pfnwrite : PFNWRITE , pfnclose : PFNCLOSE , pfnseek : PFNSEEK , cputype : FDICREATE_CPU_TYPE , perf : *mut ERF ) -> *mut ::core::ffi::c_void );
    FDICreate(pfnalloc, pfnfree, pfnopen, pfnread, pfnwrite, pfnclose, pfnseek, cputype, perf)
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDIDestroy(hfdi: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FDIDestroy ( hfdi : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FDIDestroy(hfdi)
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDIIsCabinet(hfdi: *const ::core::ffi::c_void, hf: isize, pfdici: ::core::option::Option<*mut FDICABINETINFO>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FDIIsCabinet ( hfdi : *const ::core::ffi::c_void , hf : isize , pfdici : *mut FDICABINETINFO ) -> super::super::Foundation:: BOOL );
    FDIIsCabinet(hfdi, hf, ::core::mem::transmute(pfdici.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FDITruncateCabinet<P0>(hfdi: *const ::core::ffi::c_void, pszcabinetname: P0, ifoldertodelete: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "cabinet.dll""cdecl" fn FDITruncateCabinet ( hfdi : *const ::core::ffi::c_void , pszcabinetname : ::windows::core::PCSTR , ifoldertodelete : u16 ) -> super::super::Foundation:: BOOL );
    FDITruncateCabinet(hfdi, pszcabinetname.into_param().abi(), ifoldertodelete)
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const INCLUDED_FCI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const INCLUDED_FDI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
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
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FCIERROR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FCIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FCIERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FDICREATE_CPU_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FDICREATE_CPU_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDICREATE_CPU_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FDIDECRYPTTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FDIDECRYPTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDIDECRYPTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FDIERROR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FDIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDIERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FDINOTIFICATIONTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FDINOTIFICATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDINOTIFICATIONTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
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
    pub szDisk: [u8; 256],
    pub szCab: [u8; 256],
    pub szCabPath: [u8; 256],
}
impl ::core::marker::Copy for CCAB {}
impl ::core::clone::Clone for CCAB {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::windows::core::TypeKind for CCAB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CCAB {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.cbFolderThresh == other.cbFolderThresh && self.cbReserveCFHeader == other.cbReserveCFHeader && self.cbReserveCFFolder == other.cbReserveCFFolder && self.cbReserveCFData == other.cbReserveCFData && self.iCab == other.iCab && self.iDisk == other.iDisk && self.fFailOnIncompressible == other.fFailOnIncompressible && self.setID == other.setID && self.szDisk == other.szDisk && self.szCab == other.szCab && self.szCabPath == other.szCabPath
    }
}
impl ::core::cmp::Eq for CCAB {}
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
impl ::windows::core::TypeKind for ERF {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ERF {
    fn eq(&self, other: &Self) -> bool {
        self.erfOper == other.erfOper && self.erfType == other.erfType && self.fError == other.fError
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
impl ::windows::core::TypeKind for FDICABINETINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDICABINETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbCabinet == other.cbCabinet && self.cFolders == other.cFolders && self.cFiles == other.cFiles && self.setID == other.setID && self.iCabinet == other.iCabinet && self.fReserve == other.fReserve && self.hasprev == other.hasprev && self.hasnext == other.hasnext
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
impl ::windows::core::TypeKind for FDIDECRYPT {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for FDIDECRYPT_0 {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for FDIDECRYPT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pHeaderReserve == other.pHeaderReserve && self.cbHeaderReserve == other.cbHeaderReserve && self.setID == other.setID && self.iCabinet == other.iCabinet
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
impl ::windows::core::TypeKind for FDIDECRYPT_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pDataReserve == other.pDataReserve && self.cbDataReserve == other.cbDataReserve && self.pbData == other.pbData && self.cbData == other.cbData && self.fSplit == other.fSplit && self.cbPartial == other.cbPartial
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
impl ::windows::core::TypeKind for FDIDECRYPT_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDIDECRYPT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pFolderReserve == other.pFolderReserve && self.cbFolderReserve == other.cbFolderReserve && self.iFolder == other.iFolder
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
impl ::windows::core::TypeKind for FDINOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FDINOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.psz1 == other.psz1 && self.psz2 == other.psz2 && self.psz3 == other.psz3 && self.pv == other.pv && self.hf == other.hf && self.date == other.date && self.time == other.time && self.attribs == other.attribs && self.setID == other.setID && self.iCabinet == other.iCabinet && self.iFolder == other.iFolder && self.fdie == other.fdie
    }
}
impl ::core::cmp::Eq for FDINOTIFICATION {}
impl ::core::default::Default for FDINOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FDISPILLFILE {
    pub ach: [u8; 2],
    pub cbFile: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FDISPILLFILE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FDISPILLFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for FDISPILLFILE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
#[cfg(target_arch = "x86")]
pub struct FDISPILLFILE {
    pub ach: [u8; 2],
    pub cbFile: i32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FDISPILLFILE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FDISPILLFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for FDISPILLFILE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
pub type PFNFCIFREE = ::core::option::Option<unsafe extern "system" fn(memory: *mut ::core::ffi::c_void) -> ()>;
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
pub type PFNFREE = ::core::option::Option<unsafe extern "system" fn(pv: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNOPEN = ::core::option::Option<unsafe extern "system" fn(pszfile: ::windows::core::PCSTR, oflag: i32, pmode: i32) -> isize>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNREAD = ::core::option::Option<unsafe extern "system" fn(hf: isize, pv: *mut ::core::ffi::c_void, cb: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNSEEK = ::core::option::Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_Cabinets\"`*"]
pub type PFNWRITE = ::core::option::Option<unsafe extern "system" fn(hf: isize, pv: *const ::core::ffi::c_void, cb: u32) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
