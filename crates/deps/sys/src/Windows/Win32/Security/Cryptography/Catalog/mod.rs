#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminAcquireContext(phcatadmin: *mut isize, pgsubsystem: *const ::windows::runtime::GUID, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminAcquireContext2(phcatadmin: *mut isize, pgsubsystem: *const ::windows::runtime::GUID, pwszhashalgorithm: super::super::super::Foundation::PWSTR, pstronghashpolicy: *const super::CERT_STRONG_SIGN_PARA, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminAddCatalog(hcatadmin: isize, pwszcatalogfile: super::super::super::Foundation::PWSTR, pwszselectbasename: super::super::super::Foundation::PWSTR, dwflags: u32) -> isize;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminCalcHashFromFileHandle(hfile: super::super::super::Foundation::HANDLE, pcbhash: *mut u32, pbhash: *mut u8, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin: isize, hfile: super::super::super::Foundation::HANDLE, pcbhash: *mut u32, pbhash: *mut u8, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`*"]
    pub fn CryptCATAdminEnumCatalogFromHash(hcatadmin: isize, pbhash: *const u8, cbhash: u32, dwflags: u32, phprevcatinfo: *mut isize) -> isize;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminPauseServiceForBackup(dwflags: u32, fresume: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminReleaseCatalogContext(hcatadmin: isize, hcatinfo: isize, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminReleaseContext(hcatadmin: isize, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminRemoveCatalog(hcatadmin: isize, pwszcatalogfile: super::super::super::Foundation::PWSTR, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminResolveCatalogPath(hcatadmin: isize, pwszcatalogfile: super::super::super::Foundation::PWSTR, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATAllocSortedMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATMEMBER;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATMEMBER;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCDFOpen(pwszfilepath: super::super::super::Foundation::PWSTR, pfnparseerror: ::windows::runtime::RawPtr) -> *mut CRYPTCATCDF;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCatalogInfoFromContext(hcatinfo: isize, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATClose(hcatalog: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATEnumerateAttr(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATEnumerateCatAttr(hcatalog: super::super::super::Foundation::HANDLE, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATEnumerateMember(hcatalog: super::super::super::Foundation::HANDLE, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATFreeSortedMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER);
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATGetAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATGetCatAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATGetMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR) -> *mut CRYPTCATMEMBER;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATOpen(pwszfilename: super::super::super::Foundation::PWSTR, fdwopenflags: CRYPTCAT_OPEN_FLAGS, hprov: usize, dwpublicversion: CRYPTCAT_VERSION, dwencodingtype: u32) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATPersistStore(hcatalog: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATPutAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: super::super::super::Foundation::PWSTR, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATPutCatAttrInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: super::super::super::Foundation::PWSTR, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATPutMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pwszfilename: super::super::super::Foundation::PWSTR, pwszreferencetag: super::super::super::Foundation::PWSTR, pgsubjecttype: *mut ::windows::runtime::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATStoreFromHandle(hcatalog: super::super::super::Foundation::HANDLE) -> *mut CRYPTCATSTORE;
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCatalogFile(hfile: super::super::super::Foundation::HANDLE, pwszfilename: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
}
