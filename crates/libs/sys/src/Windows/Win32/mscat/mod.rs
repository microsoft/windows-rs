#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut super::HCATADMIN, pgsubsystem : *const windows_sys::core::GUID, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wincrypt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut super::HCATADMIN, pgsubsystem : *const windows_sys::core::GUID, pwszhashalgorithm : windows_sys::core::PCWSTR, pstronghashpolicy : super::PCCERT_STRONG_SIGN_PARA, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_sys::core::PCWSTR, pwszselectbasename : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> HCATINFO);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::HANDLE, pcbhash : *mut super::DWORD, pbhash : *mut super::BYTE, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : super::HCATADMIN, hfile : super::HANDLE, pcbhash : *mut super::DWORD, pbhash : *mut super::BYTE, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : super::HCATADMIN, pbhash : *const super::BYTE, cbhash : super::DWORD, dwflags : super::DWORD, phprevcatinfo : *mut HCATINFO) -> HCATINFO);
#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : super::DWORD, fresume : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : super::HCATADMIN, hcatinfo : HCATINFO, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : super::HCATADMIN, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : super::HCATADMIN, pwszcatalogfile : *const super::WCHAR, pscatinfo : *mut CATALOG_INFO, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *mut CRYPTCATCDF) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *mut CRYPTCATCDF, pmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *mut CRYPTCATCDF, pprevmember : *mut CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCDFOpen(pwszfilepath : windows_sys::core::PCWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATCatalogInfoFromContext(hcatinfo : HCATINFO, pscatinfo : *mut CATALOG_INFO, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::HANDLE, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::HANDLE, pprevmember : *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER));
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *mut CRYPTCATSTORE) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_sys::core::PCWSTR, fdwopenflags : super::DWORD, hprov : super::HCRYPTPROV, dwpublicversion : super::DWORD, dwencodingtype : super::DWORD) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_sys::core::PCWSTR, dwattrtypeandaction : super::DWORD, cbdata : super::DWORD, pbdata : *mut super::BYTE) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_sys::core::PCWSTR, dwattrtypeandaction : super::DWORD, cbdata : super::DWORD, pbdata : *mut super::BYTE) -> *mut CRYPTCATATTRIBUTE);
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::HANDLE, pwszfilename : windows_sys::core::PCWSTR, pwszreferencetag : windows_sys::core::PCWSTR, pgsubjecttype : *mut windows_sys::core::GUID, dwcertversion : super::DWORD, cbsipindirectdata : super::DWORD, pbsipindirectdata : *mut super::BYTE) -> *mut CRYPTCATMEMBER);
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::HANDLE) -> *mut CRYPTCATSTORE);
#[cfg(feature = "winnt")]
windows_link::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::HANDLE, pwszfilename : *const super::WCHAR) -> windows_sys::core::BOOL);
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CATALOG_INFO {
    pub cbStruct: super::DWORD,
    pub wszCatalogFile: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: super::DWORD,
    pub pwszReferenceTag: windows_sys::core::PWSTR,
    pub dwAttrTypeAndAction: super::DWORD,
    pub cbValue: super::DWORD,
    pub pbValue: *mut super::BYTE,
    pub dwReserved: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPTCATCDF {
    pub cbStruct: super::DWORD,
    pub hFile: super::HANDLE,
    pub dwCurFilePos: super::DWORD,
    pub dwLastMemberOffset: super::DWORD,
    pub fEOF: windows_sys::core::BOOL,
    pub pwszResultDir: windows_sys::core::PWSTR,
    pub hCATStore: super::HANDLE,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPTCATMEMBER {
    pub cbStruct: super::DWORD,
    pub pwszReferenceTag: windows_sys::core::PWSTR,
    pub pwszFileName: windows_sys::core::PWSTR,
    pub gSubjectType: windows_sys::core::GUID,
    pub fdwMemberFlags: super::DWORD,
    pub pIndirectData: *mut super::SIP_INDIRECT_DATA,
    pub dwCertVersion: super::DWORD,
    pub dwReserved: super::DWORD,
    pub hReserved: super::HANDLE,
    pub sEncodedIndirectData: super::CRYPT_ATTR_BLOB,
    pub sEncodedMemberInfo: super::CRYPT_ATTR_BLOB,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPTCATSTORE {
    pub cbStruct: super::DWORD,
    pub dwPublicVersion: super::DWORD,
    pub pwszP7File: windows_sys::core::PWSTR,
    pub hProv: super::HCRYPTPROV,
    pub dwEncodingType: super::DWORD,
    pub fdwStoreFlags: super::DWORD,
    pub hReserved: super::HANDLE,
    pub hAttrs: super::HANDLE,
    pub hCryptMsg: super::HCRYPTMSG,
    pub hSorted: super::HANDLE,
}
pub const CRYPTCAT_ADDCATALOG_HARDLINK: i32 = 1;
pub const CRYPTCAT_ADDCATALOG_NONE: i32 = 0;
pub const CRYPTCAT_ATTR_AUTHENTICATED: i32 = 268435456;
pub const CRYPTCAT_ATTR_DATAASCII: i32 = 65536;
pub const CRYPTCAT_ATTR_DATABASE64: i32 = 131072;
pub const CRYPTCAT_ATTR_DATAREPLACE: i32 = 262144;
pub const CRYPTCAT_ATTR_NAMEASCII: i32 = 1;
pub const CRYPTCAT_ATTR_NAMEOBJID: i32 = 2;
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: i32 = 16777216;
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: i32 = 536870912;
pub const CRYPTCAT_E_AREA_ATTRIBUTE: i32 = 131072;
pub const CRYPTCAT_E_AREA_HEADER: i32 = 0;
pub const CRYPTCAT_E_AREA_MEMBER: i32 = 65536;
pub const CRYPTCAT_E_CDF_ATTR_TOOFEWVALUES: i32 = 131074;
pub const CRYPTCAT_E_CDF_ATTR_TYPECOMBO: i32 = 131076;
pub const CRYPTCAT_E_CDF_BAD_GUID_CONV: i32 = 131073;
pub const CRYPTCAT_E_CDF_DUPLICATE: i32 = 2;
pub const CRYPTCAT_E_CDF_MEMBER_FILENOTFOUND: i32 = 65540;
pub const CRYPTCAT_E_CDF_MEMBER_FILE_PATH: i32 = 65537;
pub const CRYPTCAT_E_CDF_MEMBER_INDIRECTDATA: i32 = 65538;
pub const CRYPTCAT_E_CDF_TAGNOTFOUND: i32 = 4;
pub const CRYPTCAT_E_CDF_UNSUPPORTED: i32 = 1;
pub const CRYPTCAT_FILEEXT: windows_sys::core::PCWSTR = windows_sys::core::w!("CAT");
pub const CRYPTCAT_MAX_MEMBERTAG: i32 = 128;
pub const CRYPTCAT_MEMBER_SORTED: i32 = 1073741824;
pub const CRYPTCAT_OPEN_ALWAYS: i32 = 2;
pub const CRYPTCAT_OPEN_CREATENEW: i32 = 1;
pub const CRYPTCAT_OPEN_EXCLUDE_PAGE_HASHES: i32 = 65536;
pub const CRYPTCAT_OPEN_EXISTING: i32 = 4;
pub const CRYPTCAT_OPEN_FLAGS_MASK: u32 = 4294901760;
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: i32 = 131072;
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: i32 = 536870912;
pub const CRYPTCAT_OPEN_SORTED: i32 = 1073741824;
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: i32 = 268435456;
pub const CRYPTCAT_VERSION_1: i32 = 256;
pub const CRYPTCAT_VERSION_2: i32 = 512;
pub type HCATINFO = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: super::DWORD, dwlocalerror: super::DWORD, pwszline: *mut super::WCHAR)>;
pub const szOID_CATALOG_LIST: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.12.1.3");
