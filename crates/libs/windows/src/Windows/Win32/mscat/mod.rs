#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut super::HCATADMIN, pgsubsystem: Option<*const windows_core::GUID>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut super::HCATADMIN, pgsubsystem : *const windows_core::GUID, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<P2>(phcatadmin: *mut super::HCATADMIN, pgsubsystem: Option<*const windows_core::GUID>, pwszhashalgorithm: P2, pstronghashpolicy: Option<*const super::CERT_STRONG_SIGN_PARA>, dwflags: Option<u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut super::HCATADMIN, pgsubsystem : *const windows_core::GUID, pwszhashalgorithm : windows_core::PCWSTR, pstronghashpolicy : *const super::CERT_STRONG_SIGN_PARA, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext2(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, pwszhashalgorithm.param().abi(), pstronghashpolicy.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<P1, P2>(hcatadmin: super::HCATADMIN, pwszcatalogfile: P1, pwszselectbasename: P2, dwflags: u32) -> HCATINFO
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, pwszselectbasename : windows_core::PCWSTR, dwflags : u32) -> HCATINFO);
    unsafe { CryptCATAdminAddCatalog(hcatadmin, pwszcatalogfile.param().abi(), pwszselectbasename.param().abi(), dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle(hfile: super::HANDLE, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle(hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin: super::HCATADMIN, hfile: super::HANDLE, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : super::HCATADMIN, hfile : super::HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle2(hcatadmin, hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: super::HCATADMIN, pbhash: &[u8], dwflags: Option<u32>, phprevcatinfo: Option<*mut HCATINFO>) -> HCATINFO {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : super::HCATADMIN, pbhash : *const u8, cbhash : u32, dwflags : u32, phprevcatinfo : *mut HCATINFO) -> HCATINFO);
    unsafe { CryptCATAdminEnumCatalogFromHash(hcatadmin, pbhash.as_ptr(), pbhash.len().try_into().unwrap(), dwflags.unwrap_or(core::mem::zeroed()) as _, phprevcatinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup(dwflags: u32, fresume: bool) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : u32, fresume : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CryptCATAdminPauseServiceForBackup(dwflags, fresume.into()) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: super::HCATADMIN, hcatinfo: HCATINFO, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : super::HCATADMIN, hcatinfo : HCATINFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseCatalogContext(hcatadmin, hcatinfo, dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: super::HCATADMIN, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : super::HCATADMIN, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseContext(hcatadmin, dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog<P1>(hcatadmin: super::HCATADMIN, pwszcatalogfile: P1, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminRemoveCatalog(hcatadmin, pwszcatalogfile.param().abi(), dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath<P1>(hcatadmin: super::HCATADMIN, pwszcatalogfile: P1, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminResolveCatalogPath(hcatadmin, pwszcatalogfile.param().abi(), pscatinfo as _, dwflags) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATAllocSortedMemberInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *const CRYPTCATCDF) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *const CRYPTCATCDF) -> windows_core::BOOL);
    unsafe { CryptCATCDFClose(pcdf) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *const CRYPTCATCDF, pmember: *const CRYPTCATMEMBER, pprevattr: *const CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *const CRYPTCATCDF, pmember : *const CRYPTCATMEMBER, pprevattr : *const CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumAttributes(pcdf, pmember, pprevattr, pfnparseerror) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumCatAttributes(pcdf as _, pprevattr as _, pfnparseerror) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *const CRYPTCATCDF, pprevmember: *const CRYPTCATMEMBER, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *const CRYPTCATCDF, pprevmember : *const CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATCDFEnumMembers(pcdf, pprevmember, pfnparseerror) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATCDFOpen<P0>(pwszfilepath: P0, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFOpen(pwszfilepath : windows_core::PCWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF);
    unsafe { CryptCATCDFOpen(pwszfilepath.param().abi(), pfnparseerror) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATCatalogInfoFromContext(hcatinfo: HCATINFO, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCatalogInfoFromContext(hcatinfo : HCATINFO, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATCatalogInfoFromContext(hcatinfo, pscatinfo as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATClose(hcatalog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATClose(hcatalog) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateAttr(hcatalog: super::HANDLE, pcatmember: *const CRYPTCATMEMBER, pprevattr: *const CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::HANDLE, pcatmember : *const CRYPTCATMEMBER, pprevattr : *const CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateAttr(hcatalog, pcatmember, pprevattr) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr(hcatalog: super::HANDLE, pprevattr: *const CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::HANDLE, pprevattr : *const CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateCatAttr(hcatalog, pprevattr) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateMember(hcatalog: super::HANDLE, pprevmember: *const CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::HANDLE, pprevmember : *const CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATEnumerateMember(hcatalog, pprevmember) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo(hcatalog: super::HANDLE, pcatmember: *const CRYPTCATMEMBER) {
    windows_core::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::HANDLE, pcatmember : *const CRYPTCATMEMBER));
    unsafe { CryptCATFreeSortedMemberInfo(hcatalog, pcatmember) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<P2>(hcatalog: super::HANDLE, pcatmember: *const CRYPTCATMEMBER, pwszreferencetag: P2) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::HANDLE, pcatmember : *const CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetAttrInfo(hcatalog, pcatmember, pwszreferencetag.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetCatAttrInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATGetMemberInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *const CRYPTCATSTORE) -> super::HANDLE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *const CRYPTCATSTORE) -> super::HANDLE);
    unsafe { CryptCATHandleFromStore(pcatstore) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATOpen<P0>(pwszfilename: P0, fdwopenflags: u32, hprov: Option<super::HCRYPTPROV>, dwpublicversion: Option<u32>, dwencodingtype: Option<u32>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_core::PCWSTR, fdwopenflags : u32, hprov : super::HCRYPTPROV, dwpublicversion : u32, dwencodingtype : u32) -> super::HANDLE);
    unsafe { CryptCATOpen(pwszfilename.param().abi(), fdwopenflags, hprov.unwrap_or(core::mem::zeroed()) as _, dwpublicversion.unwrap_or(core::mem::zeroed()) as _, dwencodingtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATPersistStore(hcatalog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATPersistStore(hcatalog) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<P2>(hcatalog: super::HANDLE, pcatmember: *const CRYPTCATMEMBER, pwszreferencetag: P2, dwattrtypeandaction: u32, cbdata: u32, pbdata: *const u8) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::HANDLE, pcatmember : *const CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *const u8) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutAttrInfo(hcatalog, pcatmember, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *const u8) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *const u8) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutCatAttrInfo(hcatalog, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<P1, P2>(hcatalog: super::HANDLE, pwszfilename: P1, pwszreferencetag: P2, pgsubjecttype: *const windows_core::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *const u8) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::HANDLE, pwszfilename : windows_core::PCWSTR, pwszreferencetag : windows_core::PCWSTR, pgsubjecttype : *const windows_core::GUID, dwcertversion : u32, cbsipindirectdata : u32, pbsipindirectdata : *const u8) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATPutMemberInfo(hcatalog, pwszfilename.param().abi(), pwszreferencetag.param().abi(), pgsubjecttype, dwcertversion, cbsipindirectdata, pbsipindirectdata) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATStoreFromHandle(hcatalog: super::HANDLE) -> *mut CRYPTCATSTORE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::HANDLE) -> *mut CRYPTCATSTORE);
    unsafe { CryptCATStoreFromHandle(hcatalog) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsCatalogFile<P1>(hfile: Option<super::HANDLE>, pwszfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::HANDLE, pwszfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IsCatalogFile(hfile.unwrap_or(core::mem::zeroed()) as _, pwszfilename.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub dwAttrTypeAndAction: u32,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: windows_core::BOOL,
    pub pwszResultDir: windows_core::PWSTR,
    pub hCATStore: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub pwszFileName: windows_core::PWSTR,
    pub gSubjectType: windows_core::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut super::SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::HANDLE,
    pub sEncodedIndirectData: super::CRYPT_ATTR_BLOB,
    pub sEncodedMemberInfo: super::CRYPT_ATTR_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: windows_core::PWSTR,
    pub hProv: super::HCRYPTPROV,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: u32,
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
pub const CRYPTCAT_FILEEXT: windows_core::PCWSTR = windows_core::w!("CAT");
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
#[cfg(feature = "winnt")]
pub type HCATINFO = super::HANDLE;
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: *mut u16)>;
pub const szOID_CATALOG_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.3");
