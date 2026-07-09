#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut super::wintrust::HCATADMIN, pgsubsystem : *const windows_sys::core::GUID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_wincrypt", feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut super::wintrust::HCATADMIN, pgsubsystem : *const windows_sys::core::GUID, pwszhashalgorithm : windows_sys::core::PCWSTR, pstronghashpolicy : *const super::wincrypt::CERT_STRONG_SIGN_PARA, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : super::wintrust::HCATADMIN, pwszcatalogfile : windows_sys::core::PCWSTR, pwszselectbasename : windows_sys::core::PCWSTR, dwflags : u32) -> HCATINFO);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::winnt::HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : super::wintrust::HCATADMIN, hfile : super::winnt::HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : super::wintrust::HCATADMIN, pbhash : *const u8, cbhash : u32, dwflags : u32, phprevcatinfo : *mut HCATINFO) -> HCATINFO);
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : u32, fresume : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : super::wintrust::HCATADMIN, hcatinfo : HCATINFO, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : super::wintrust::HCATADMIN, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : super::wintrust::HCATADMIN, pwszcatalogfile : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wintrust"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : super::wintrust::HCATADMIN, pwszcatalogfile : windows_sys::core::PCWSTR, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATMEMBER);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *mut CRYPTCATCDF) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *mut CRYPTCATCDF, pmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *mut CRYPTCATCDF, pprevmember : *mut CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFOpen(pwszfilepath : windows_sys::core::PCWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATCatalogInfoFromContext(hcatinfo : HCATINFO, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::winnt::HANDLE, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::winnt::HANDLE, pprevmember : *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER));
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *mut CRYPTCATSTORE) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_sys::core::PCWSTR, fdwopenflags : u32, hprov : super::wincrypt::HCRYPTPROV, dwpublicversion : u32, dwencodingtype : u32) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_sys::core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::winnt::HANDLE, pwszfilename : windows_sys::core::PCWSTR, pwszreferencetag : windows_sys::core::PCWSTR, pgsubjecttype : *mut windows_sys::core::GUID, dwcertversion : u32, cbsipindirectdata : u32, pbsipindirectdata : *mut u8) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "Win32_wincrypt", feature = "Win32_winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::winnt::HANDLE) -> *mut CRYPTCATSTORE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::winnt::HANDLE, pwszfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CATALOG_INFO {
    pub cbStruct: u32,
    pub wszCatalogFile: [u16; 260],
}
impl Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_sys::core::PWSTR,
    pub dwAttrTypeAndAction: u32,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
impl Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::winnt::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: windows_sys::core::BOOL,
    pub pwszResultDir: windows_sys::core::PWSTR,
    pub hCATStore: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for CRYPTCATCDF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_sys::core::PWSTR,
    pub pwszFileName: windows_sys::core::PWSTR,
    pub gSubjectType: windows_sys::core::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut super::mssip::SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::winnt::HANDLE,
    pub sEncodedIndirectData: super::wincrypt::CRYPT_ATTR_BLOB,
    pub sEncodedMemberInfo: super::wincrypt::CRYPT_ATTR_BLOB,
}
#[cfg(all(feature = "Win32_mssip", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
impl Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wincrypt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: windows_sys::core::PWSTR,
    pub hProv: super::wincrypt::HCRYPTPROV,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: u32,
    pub hReserved: super::winnt::HANDLE,
    pub hAttrs: super::winnt::HANDLE,
    pub hCryptMsg: super::wincrypt::HCRYPTMSG,
    pub hSorted: super::winnt::HANDLE,
}
#[cfg(all(feature = "Win32_wincrypt", feature = "Win32_winnt"))]
impl Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTCAT_ADDCATALOG_HARDLINK: u32 = 1;
pub const CRYPTCAT_ADDCATALOG_NONE: u32 = 0;
pub const CRYPTCAT_ATTR_AUTHENTICATED: u32 = 268435456;
pub const CRYPTCAT_ATTR_DATAASCII: u32 = 65536;
pub const CRYPTCAT_ATTR_DATABASE64: u32 = 131072;
pub const CRYPTCAT_ATTR_DATAREPLACE: u32 = 262144;
pub const CRYPTCAT_ATTR_NAMEASCII: u32 = 1;
pub const CRYPTCAT_ATTR_NAMEOBJID: u32 = 2;
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: u32 = 16777216;
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: u32 = 536870912;
pub const CRYPTCAT_E_AREA_ATTRIBUTE: u32 = 131072;
pub const CRYPTCAT_E_AREA_HEADER: u32 = 0;
pub const CRYPTCAT_E_AREA_MEMBER: u32 = 65536;
pub const CRYPTCAT_E_CDF_ATTR_TOOFEWVALUES: u32 = 131074;
pub const CRYPTCAT_E_CDF_ATTR_TYPECOMBO: u32 = 131076;
pub const CRYPTCAT_E_CDF_BAD_GUID_CONV: u32 = 131073;
pub const CRYPTCAT_E_CDF_DUPLICATE: u32 = 2;
pub const CRYPTCAT_E_CDF_MEMBER_FILENOTFOUND: u32 = 65540;
pub const CRYPTCAT_E_CDF_MEMBER_FILE_PATH: u32 = 65537;
pub const CRYPTCAT_E_CDF_MEMBER_INDIRECTDATA: u32 = 65538;
pub const CRYPTCAT_E_CDF_TAGNOTFOUND: u32 = 4;
pub const CRYPTCAT_E_CDF_UNSUPPORTED: u32 = 1;
pub const CRYPTCAT_FILEEXT: windows_sys::core::PCWSTR = windows_sys::core::w!("CAT");
pub const CRYPTCAT_MAX_MEMBERTAG: u32 = 128;
pub const CRYPTCAT_MEMBER_SORTED: u32 = 1073741824;
pub const CRYPTCAT_OPEN_ALWAYS: u32 = 2;
pub const CRYPTCAT_OPEN_CREATENEW: u32 = 1;
pub const CRYPTCAT_OPEN_EXCLUDE_PAGE_HASHES: u32 = 65536;
pub const CRYPTCAT_OPEN_EXISTING: u32 = 4;
pub const CRYPTCAT_OPEN_FLAGS_MASK: u32 = 4294901760;
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: u32 = 131072;
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: u32 = 536870912;
pub const CRYPTCAT_OPEN_SORTED: u32 = 1073741824;
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: u32 = 268435456;
pub const CRYPTCAT_VERSION_1: u32 = 256;
pub const CRYPTCAT_VERSION_2: u32 = 512;
#[cfg(feature = "Win32_winnt")]
pub type HCATINFO = super::winnt::HANDLE;
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: *mut u16)>;
pub const szOID_CATALOG_LIST: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.12.1.3");
