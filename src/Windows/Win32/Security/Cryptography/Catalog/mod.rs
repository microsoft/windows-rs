#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CATALOG_INFO {
    pub cbStruct: u32,
    pub wszCatalogFile: [u16; 260],
}
impl CATALOG_INFO {}
impl ::std::default::Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CATALOG_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CATALOG_INFO").field("cbStruct", &self.cbStruct).field("wszCatalogFile", &self.wszCatalogFile).finish()
    }
}
impl ::std::cmp::PartialEq for CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.wszCatalogFile == other.wszCatalogFile
    }
}
impl ::std::cmp::Eq for CATALOG_INFO {}
unsafe impl ::windows::runtime::Abi for CATALOG_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: super::super::super::Foundation::PWSTR,
    pub dwAttrTypeAndAction: u32,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTCATATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTCATATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATATTRIBUTE").field("cbStruct", &self.cbStruct).field("pwszReferenceTag", &self.pwszReferenceTag).field("dwAttrTypeAndAction", &self.dwAttrTypeAndAction).field("cbValue", &self.cbValue).field("pbValue", &self.pbValue).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTCATATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszReferenceTag == other.pwszReferenceTag && self.dwAttrTypeAndAction == other.dwAttrTypeAndAction && self.cbValue == other.cbValue && self.pbValue == other.pbValue && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTCATATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTCATATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: super::super::super::Foundation::BOOL,
    pub pwszResultDir: super::super::super::Foundation::PWSTR,
    pub hCATStore: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTCATCDF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTCATCDF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATCDF")
            .field("cbStruct", &self.cbStruct)
            .field("hFile", &self.hFile)
            .field("dwCurFilePos", &self.dwCurFilePos)
            .field("dwLastMemberOffset", &self.dwLastMemberOffset)
            .field("fEOF", &self.fEOF)
            .field("pwszResultDir", &self.pwszResultDir)
            .field("hCATStore", &self.hCATStore)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTCATCDF {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.hFile == other.hFile && self.dwCurFilePos == other.dwCurFilePos && self.dwLastMemberOffset == other.dwLastMemberOffset && self.fEOF == other.fEOF && self.pwszResultDir == other.pwszResultDir && self.hCATStore == other.hCATStore
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTCATCDF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: super::super::super::Foundation::PWSTR,
    pub pwszFileName: super::super::super::Foundation::PWSTR,
    pub gSubjectType: ::windows::runtime::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut super::Sip::SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::super::super::Foundation::HANDLE,
    pub sEncodedIndirectData: super::CRYPTOAPI_BLOB,
    pub sEncodedMemberInfo: super::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl CRYPTCATMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::fmt::Debug for CRYPTCATMEMBER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATMEMBER")
            .field("cbStruct", &self.cbStruct)
            .field("pwszReferenceTag", &self.pwszReferenceTag)
            .field("pwszFileName", &self.pwszFileName)
            .field("gSubjectType", &self.gSubjectType)
            .field("fdwMemberFlags", &self.fdwMemberFlags)
            .field("pIndirectData", &self.pIndirectData)
            .field("dwCertVersion", &self.dwCertVersion)
            .field("dwReserved", &self.dwReserved)
            .field("hReserved", &self.hReserved)
            .field("sEncodedIndirectData", &self.sEncodedIndirectData)
            .field("sEncodedMemberInfo", &self.sEncodedMemberInfo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for CRYPTCATMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pwszReferenceTag == other.pwszReferenceTag
            && self.pwszFileName == other.pwszFileName
            && self.gSubjectType == other.gSubjectType
            && self.fdwMemberFlags == other.fdwMemberFlags
            && self.pIndirectData == other.pIndirectData
            && self.dwCertVersion == other.dwCertVersion
            && self.dwReserved == other.dwReserved
            && self.hReserved == other.hReserved
            && self.sEncodedIndirectData == other.sEncodedIndirectData
            && self.sEncodedMemberInfo == other.sEncodedMemberInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for CRYPTCATMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for CRYPTCATMEMBER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: super::super::super::Foundation::PWSTR,
    pub hProv: usize,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: CRYPTCAT_OPEN_FLAGS,
    pub hReserved: super::super::super::Foundation::HANDLE,
    pub hAttrs: super::super::super::Foundation::HANDLE,
    pub hCryptMsg: *mut ::std::ffi::c_void,
    pub hSorted: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTCATSTORE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATSTORE")
            .field("cbStruct", &self.cbStruct)
            .field("dwPublicVersion", &self.dwPublicVersion)
            .field("pwszP7File", &self.pwszP7File)
            .field("hProv", &self.hProv)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("fdwStoreFlags", &self.fdwStoreFlags)
            .field("hReserved", &self.hReserved)
            .field("hAttrs", &self.hAttrs)
            .field("hCryptMsg", &self.hCryptMsg)
            .field("hSorted", &self.hSorted)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTCATSTORE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwPublicVersion == other.dwPublicVersion && self.pwszP7File == other.pwszP7File && self.hProv == other.hProv && self.dwEncodingType == other.dwEncodingType && self.fdwStoreFlags == other.fdwStoreFlags && self.hReserved == other.hReserved && self.hAttrs == other.hAttrs && self.hCryptMsg == other.hCryptMsg && self.hSorted == other.hSorted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTCATSTORE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRYPTCAT_ADDCATALOG_HARDLINK: u32 = 1u32;
pub const CRYPTCAT_ADDCATALOG_NONE: u32 = 0u32;
pub const CRYPTCAT_ATTR_AUTHENTICATED: u32 = 268435456u32;
pub const CRYPTCAT_ATTR_DATAASCII: u32 = 65536u32;
pub const CRYPTCAT_ATTR_DATABASE64: u32 = 131072u32;
pub const CRYPTCAT_ATTR_DATAREPLACE: u32 = 262144u32;
pub const CRYPTCAT_ATTR_NAMEASCII: u32 = 1u32;
pub const CRYPTCAT_ATTR_NAMEOBJID: u32 = 2u32;
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: u32 = 16777216u32;
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: u32 = 536870912u32;
pub const CRYPTCAT_E_AREA_ATTRIBUTE: u32 = 131072u32;
pub const CRYPTCAT_E_AREA_HEADER: u32 = 0u32;
pub const CRYPTCAT_E_AREA_MEMBER: u32 = 65536u32;
pub const CRYPTCAT_E_CDF_ATTR_TOOFEWVALUES: u32 = 131074u32;
pub const CRYPTCAT_E_CDF_ATTR_TYPECOMBO: u32 = 131076u32;
pub const CRYPTCAT_E_CDF_BAD_GUID_CONV: u32 = 131073u32;
pub const CRYPTCAT_E_CDF_DUPLICATE: u32 = 2u32;
pub const CRYPTCAT_E_CDF_MEMBER_FILENOTFOUND: u32 = 65540u32;
pub const CRYPTCAT_E_CDF_MEMBER_FILE_PATH: u32 = 65537u32;
pub const CRYPTCAT_E_CDF_MEMBER_INDIRECTDATA: u32 = 65538u32;
pub const CRYPTCAT_E_CDF_TAGNOTFOUND: u32 = 4u32;
pub const CRYPTCAT_E_CDF_UNSUPPORTED: u32 = 1u32;
pub const CRYPTCAT_MAX_MEMBERTAG: u32 = 64u32;
pub const CRYPTCAT_MEMBER_SORTED: u32 = 1073741824u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CRYPTCAT_OPEN_FLAGS(pub u32);
pub const CRYPTCAT_OPEN_ALWAYS: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(2u32);
pub const CRYPTCAT_OPEN_CREATENEW: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1u32);
pub const CRYPTCAT_OPEN_EXISTING: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4u32);
pub const CRYPTCAT_OPEN_EXCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(65536u32);
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(131072u32);
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(268435456u32);
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(536870912u32);
pub const CRYPTCAT_OPEN_SORTED: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1073741824u32);
pub const CRYPTCAT_OPEN_FLAGS_MASK: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4294901760u32);
impl ::std::convert::From<u32> for CRYPTCAT_OPEN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRYPTCAT_OPEN_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CRYPTCAT_VERSION(pub u32);
pub const CRYPTCAT_VERSION_1: CRYPTCAT_VERSION = CRYPTCAT_VERSION(256u32);
pub const CRYPTCAT_VERSION_2: CRYPTCAT_VERSION = CRYPTCAT_VERSION(512u32);
impl ::std::convert::From<u32> for CRYPTCAT_VERSION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRYPTCAT_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRYPTCAT_VERSION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRYPTCAT_VERSION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRYPTCAT_VERSION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRYPTCAT_VERSION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRYPTCAT_VERSION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut isize, pgsubsystem: *const ::windows::runtime::GUID, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminAcquireContext(phcatadmin: *mut isize, pgsubsystem: *const ::windows::runtime::GUID, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminAcquireContext(::std::mem::transmute(phcatadmin), ::std::mem::transmute(pgsubsystem), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(phcatadmin: *mut isize, pgsubsystem: *const ::windows::runtime::GUID, pwszhashalgorithm: Param2, pstronghashpolicy: *const super::CERT_STRONG_SIGN_PARA, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminAcquireContext2(phcatadmin: *mut isize, pgsubsystem: *const ::windows::runtime::GUID, pwszhashalgorithm: super::super::super::Foundation::PWSTR, pstronghashpolicy: *const super::CERT_STRONG_SIGN_PARA, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminAcquireContext2(::std::mem::transmute(phcatadmin), ::std::mem::transmute(pgsubsystem), pwszhashalgorithm.into_param().abi(), ::std::mem::transmute(pstronghashpolicy), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatadmin: isize, pwszcatalogfile: Param1, pwszselectbasename: Param2, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminAddCatalog(hcatadmin: isize, pwszcatalogfile: super::super::super::Foundation::PWSTR, pwszselectbasename: super::super::super::Foundation::PWSTR, dwflags: u32) -> isize;
        }
        ::std::mem::transmute(CryptCATAdminAddCatalog(::std::mem::transmute(hcatadmin), pwszcatalogfile.into_param().abi(), pwszselectbasename.into_param().abi(), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hfile: Param0, pcbhash: *mut u32, pbhash: *mut u8, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminCalcHashFromFileHandle(hfile: super::super::super::Foundation::HANDLE, pcbhash: *mut u32, pbhash: *mut u8, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminCalcHashFromFileHandle(hfile.into_param().abi(), ::std::mem::transmute(pcbhash), ::std::mem::transmute(pbhash), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatadmin: isize, hfile: Param1, pcbhash: *mut u32, pbhash: *mut u8, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin: isize, hfile: super::super::super::Foundation::HANDLE, pcbhash: *mut u32, pbhash: *mut u8, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminCalcHashFromFileHandle2(::std::mem::transmute(hcatadmin), hfile.into_param().abi(), ::std::mem::transmute(pcbhash), ::std::mem::transmute(pbhash), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: isize, pbhash: *const u8, cbhash: u32, dwflags: u32, phprevcatinfo: *mut isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminEnumCatalogFromHash(hcatadmin: isize, pbhash: *const u8, cbhash: u32, dwflags: u32, phprevcatinfo: *mut isize) -> isize;
        }
        ::std::mem::transmute(CryptCATAdminEnumCatalogFromHash(::std::mem::transmute(hcatadmin), ::std::mem::transmute(pbhash), ::std::mem::transmute(cbhash), ::std::mem::transmute(dwflags), ::std::mem::transmute(phprevcatinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(dwflags: u32, fresume: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminPauseServiceForBackup(dwflags: u32, fresume: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminPauseServiceForBackup(::std::mem::transmute(dwflags), fresume.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: isize, hcatinfo: isize, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminReleaseCatalogContext(hcatadmin: isize, hcatinfo: isize, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminReleaseCatalogContext(::std::mem::transmute(hcatadmin), ::std::mem::transmute(hcatinfo), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: isize, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminReleaseContext(hcatadmin: isize, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminReleaseContext(::std::mem::transmute(hcatadmin), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatadmin: isize, pwszcatalogfile: Param1, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminRemoveCatalog(hcatadmin: isize, pwszcatalogfile: super::super::super::Foundation::PWSTR, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminRemoveCatalog(::std::mem::transmute(hcatadmin), pwszcatalogfile.into_param().abi(), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatadmin: isize, pwszcatalogfile: Param1, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminResolveCatalogPath(hcatadmin: isize, pwszcatalogfile: super::super::super::Foundation::PWSTR, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminResolveCatalogPath(::std::mem::transmute(hcatadmin), pwszcatalogfile.into_param().abi(), ::std::mem::transmute(pscatinfo), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pwszreferencetag: Param1) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAllocSortedMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATAllocSortedMemberInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATCDFClose(::std::mem::transmute(pcdf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATCDFEnumAttributes(::std::mem::transmute(pcdf), ::std::mem::transmute(pmember), ::std::mem::transmute(pprevattr), ::std::mem::transmute(pfnparseerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATCDFEnumCatAttributes(::std::mem::transmute(pcdf), ::std::mem::transmute(pprevattr), ::std::mem::transmute(pfnparseerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATCDFEnumMembers(::std::mem::transmute(pcdf), ::std::mem::transmute(pprevmember), ::std::mem::transmute(pfnparseerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCDFOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwszfilepath: Param0, pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>) -> *mut CRYPTCATCDF {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFOpen(pwszfilepath: super::super::super::Foundation::PWSTR, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATCDF;
        }
        ::std::mem::transmute(CryptCATCDFOpen(pwszfilepath.into_param().abi(), ::std::mem::transmute(pfnparseerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCatalogInfoFromContext(hcatinfo: isize, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCatalogInfoFromContext(hcatinfo: isize, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATCatalogInfoFromContext(::std::mem::transmute(hcatinfo), ::std::mem::transmute(pscatinfo), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATClose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATClose(hcatalog: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATClose(hcatalog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATEnumerateAttr<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATEnumerateAttr(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATEnumerateAttr(hcatalog.into_param().abi(), ::std::mem::transmute(pcatmember), ::std::mem::transmute(pprevattr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATEnumerateCatAttr(hcatalog: super::super::super::Foundation::HANDLE, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATEnumerateCatAttr(hcatalog.into_param().abi(), ::std::mem::transmute(pprevattr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATEnumerateMember<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATEnumerateMember(hcatalog: super::super::super::Foundation::HANDLE, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATEnumerateMember(hcatalog.into_param().abi(), ::std::mem::transmute(pprevmember)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0, pcatmember: *mut CRYPTCATMEMBER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATFreeSortedMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER);
        }
        ::std::mem::transmute(CryptCATFreeSortedMemberInfo(hcatalog.into_param().abi(), ::std::mem::transmute(pcatmember)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: Param2) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATGetAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATGetAttrInfo(hcatalog.into_param().abi(), ::std::mem::transmute(pcatmember), pwszreferencetag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pwszreferencetag: Param1) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATGetCatAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATGetCatAttrInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pwszreferencetag: Param1) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATGetMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATGetMemberInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CryptCATHandleFromStore(::std::mem::transmute(pcatstore)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwszfilename: Param0, fdwopenflags: CRYPTCAT_OPEN_FLAGS, hprov: usize, dwpublicversion: CRYPTCAT_VERSION, dwencodingtype: u32) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATOpen(pwszfilename: super::super::super::Foundation::PWSTR, fdwopenflags: CRYPTCAT_OPEN_FLAGS, hprov: usize, dwpublicversion: CRYPTCAT_VERSION, dwencodingtype: u32) -> super::super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CryptCATOpen(pwszfilename.into_param().abi(), ::std::mem::transmute(fdwopenflags), ::std::mem::transmute(hprov), ::std::mem::transmute(dwpublicversion), ::std::mem::transmute(dwencodingtype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATPersistStore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPersistStore(hcatalog: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATPersistStore(hcatalog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: Param2, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPutAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: super::super::super::Foundation::PWSTR, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATPutAttrInfo(hcatalog.into_param().abi(), ::std::mem::transmute(pcatmember), pwszreferencetag.into_param().abi(), ::std::mem::transmute(dwattrtypeandaction), ::std::mem::transmute(cbdata), ::std::mem::transmute(pbdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pwszreferencetag: Param1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPutCatAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATPutCatAttrInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi(), ::std::mem::transmute(dwattrtypeandaction), ::std::mem::transmute(cbdata), ::std::mem::transmute(pbdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hcatalog: Param0, pwszfilename: Param1, pwszreferencetag: Param2, pgsubjecttype: *mut ::windows::runtime::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPutMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszfilename: super::super::super::Foundation::PWSTR, pwszreferencetag: super::super::super::Foundation::PWSTR, pgsubjecttype: *mut ::windows::runtime::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATPutMemberInfo(hcatalog.into_param().abi(), pwszfilename.into_param().abi(), pwszreferencetag.into_param().abi(), ::std::mem::transmute(pgsubjecttype), ::std::mem::transmute(dwcertversion), ::std::mem::transmute(cbsipindirectdata), ::std::mem::transmute(pbsipindirectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATStoreFromHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hcatalog: Param0) -> *mut CRYPTCATSTORE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATStoreFromHandle(hcatalog: super::super::super::Foundation::HANDLE) -> *mut CRYPTCATSTORE;
        }
        ::std::mem::transmute(CryptCATStoreFromHandle(hcatalog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCatalogFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hfile: Param0, pwszfilename: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCatalogFile(hfile: super::super::super::Foundation::HANDLE, pwszfilename: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsCatalogFile(hfile.into_param().abi(), pwszfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CDF_PARSE_ERROR_CALLBACK = unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: super::super::super::Foundation::PWSTR);
