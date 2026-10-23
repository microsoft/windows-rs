#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut super::HCATADMIN, pgsubsystem: Option<*const windows_core::GUID>, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut super::HCATADMIN, pgsubsystem : *const windows_core::GUID, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<P2>(phcatadmin: *mut super::HCATADMIN, pgsubsystem: Option<*const windows_core::GUID>, pwszhashalgorithm: P2, pstronghashpolicy: Option<super::PCCERT_STRONG_SIGN_PARA>, dwflags: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut super::HCATADMIN, pgsubsystem : *const windows_core::GUID, pwszhashalgorithm : windows_core::PCWSTR, pstronghashpolicy : super::PCCERT_STRONG_SIGN_PARA, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext2(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, pwszhashalgorithm.param().abi(), pstronghashpolicy.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<P1, P2>(hcatadmin: super::HCATADMIN, pwszcatalogfile: P1, pwszselectbasename: P2, dwflags: super::DWORD) -> HCATINFO
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, pwszselectbasename : windows_core::PCWSTR, dwflags : super::DWORD) -> HCATINFO);
    unsafe { CryptCATAdminAddCatalog(hcatadmin, pwszcatalogfile.param().abi(), pwszselectbasename.param().abi(), dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle(hfile: super::HANDLE, pcbhash: *mut super::DWORD, pbhash: Option<*mut super::BYTE>, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::HANDLE, pcbhash : *mut super::DWORD, pbhash : *mut super::BYTE, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle(hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin: super::HCATADMIN, hfile: super::HANDLE, pcbhash: *mut super::DWORD, pbhash: Option<*mut super::BYTE>, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : super::HCATADMIN, hfile : super::HANDLE, pcbhash : *mut super::DWORD, pbhash : *mut super::BYTE, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle2(hcatadmin, hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: super::HCATADMIN, pbhash: &[u8], dwflags: super::DWORD, phprevcatinfo: Option<*mut HCATINFO>) -> HCATINFO {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : super::HCATADMIN, pbhash : *const super::BYTE, cbhash : super::DWORD, dwflags : super::DWORD, phprevcatinfo : *mut HCATINFO) -> HCATINFO);
    unsafe { CryptCATAdminEnumCatalogFromHash(hcatadmin, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), dwflags, phprevcatinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup(dwflags: super::DWORD, fresume: windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : super::DWORD, fresume : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CryptCATAdminPauseServiceForBackup(dwflags, fresume) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: super::HCATADMIN, hcatinfo: HCATINFO, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : super::HCATADMIN, hcatinfo : HCATINFO, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseCatalogContext(hcatadmin as _, hcatinfo as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: super::HCATADMIN, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : super::HCATADMIN, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseContext(hcatadmin as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog(hcatadmin: super::HCATADMIN, pwszcatalogfile: &windows_core::PCWSTR, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : super::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminRemoveCatalog(hcatadmin as _, core::mem::transmute_copy(pwszcatalogfile), dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath(hcatadmin: super::HCATADMIN, pwszcatalogfile: *const super::WCHAR, pscatinfo: *mut CATALOG_INFO, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : super::HCATADMIN, pwszcatalogfile : *const super::WCHAR, pscatinfo : *mut CATALOG_INFO, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATAdminResolveCatalogPath(hcatadmin, pwszcatalogfile, pscatinfo as _, dwflags) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATAllocSortedMemberInfo(hcatalog as _, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *mut CRYPTCATCDF) -> windows_core::BOOL);
    unsafe { CryptCATCDFClose(pcdf as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *mut CRYPTCATCDF, pmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumAttributes(pcdf as _, pmember as _, pprevattr as _, pfnparseerror) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumCatAttributes(pcdf as _, pprevattr as _, pfnparseerror) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *mut CRYPTCATCDF, pprevmember : *mut CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATCDFEnumMembers(pcdf as _, pprevmember as _, pfnparseerror) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFOpen<P0>(pwszfilepath: P0, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFOpen(pwszfilepath : windows_core::PCWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF);
    unsafe { CryptCATCDFOpen(pwszfilepath.param().abi(), pfnparseerror) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCatalogInfoFromContext(hcatinfo: HCATINFO, pscatinfo: *mut CATALOG_INFO, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCatalogInfoFromContext(hcatinfo : HCATINFO, pscatinfo : *mut CATALOG_INFO, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptCATCatalogInfoFromContext(hcatinfo, pscatinfo as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATClose(hcatalog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATClose(hcatalog as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateAttr(hcatalog: super::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateAttr(hcatalog as _, pcatmember as _, pprevattr as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr(hcatalog: super::HANDLE, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::HANDLE, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateCatAttr(hcatalog as _, pprevattr as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateMember(hcatalog: super::HANDLE, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::HANDLE, pprevmember : *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATEnumerateMember(hcatalog as _, pprevmember as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo(hcatalog: super::HANDLE, pcatmember: *mut CRYPTCATMEMBER) {
    windows_core::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER));
    unsafe { CryptCATFreeSortedMemberInfo(hcatalog as _, pcatmember as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<P2>(hcatalog: super::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P2) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetAttrInfo(hcatalog as _, pcatmember as _, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetCatAttrInfo(hcatalog as _, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATGetMemberInfo(hcatalog as _, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::HANDLE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *mut CRYPTCATSTORE) -> super::HANDLE);
    unsafe { CryptCATHandleFromStore(pcatstore as _) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATOpen<P0>(pwszfilename: P0, fdwopenflags: super::DWORD, hprov: super::HCRYPTPROV, dwpublicversion: super::DWORD, dwencodingtype: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_core::PCWSTR, fdwopenflags : super::DWORD, hprov : super::HCRYPTPROV, dwpublicversion : super::DWORD, dwencodingtype : super::DWORD) -> super::HANDLE);
    unsafe { CryptCATOpen(pwszfilename.param().abi(), fdwopenflags, hprov, dwpublicversion, dwencodingtype) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATPersistStore(hcatalog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATPersistStore(hcatalog as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<P2>(hcatalog: super::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P2, dwattrtypeandaction: super::DWORD, cbdata: super::DWORD, pbdata: *mut super::BYTE) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : super::DWORD, cbdata : super::DWORD, pbdata : *mut super::BYTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutAttrInfo(hcatalog as _, pcatmember as _, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<P1>(hcatalog: super::HANDLE, pwszreferencetag: P1, dwattrtypeandaction: super::DWORD, cbdata: super::DWORD, pbdata: *mut super::BYTE) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::HANDLE, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : super::DWORD, cbdata : super::DWORD, pbdata : *mut super::BYTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutCatAttrInfo(hcatalog as _, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata as _) }
}
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<P1, P2>(hcatalog: super::HANDLE, pwszfilename: P1, pwszreferencetag: P2, pgsubjecttype: *mut windows_core::GUID, dwcertversion: super::DWORD, cbsipindirectdata: super::DWORD, pbsipindirectdata: *mut super::BYTE) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::HANDLE, pwszfilename : windows_core::PCWSTR, pwszreferencetag : windows_core::PCWSTR, pgsubjecttype : *mut windows_core::GUID, dwcertversion : super::DWORD, cbsipindirectdata : super::DWORD, pbsipindirectdata : *mut super::BYTE) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATPutMemberInfo(hcatalog as _, pwszfilename.param().abi(), pwszreferencetag.param().abi(), pgsubjecttype as _, dwcertversion, cbsipindirectdata, pbsipindirectdata as _) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATStoreFromHandle(hcatalog: super::HANDLE) -> *mut CRYPTCATSTORE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::HANDLE) -> *mut CRYPTCATSTORE);
    unsafe { CryptCATStoreFromHandle(hcatalog as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsCatalogFile(hfile: super::HANDLE, pwszfilename: Option<*const super::WCHAR>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::HANDLE, pwszfilename : *const super::WCHAR) -> windows_core::BOOL);
    unsafe { IsCatalogFile(hfile as _, pwszfilename.unwrap_or(core::mem::zeroed()) as _) }
}
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
    pub pwszReferenceTag: windows_core::PWSTR,
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
    pub fEOF: windows_core::BOOL,
    pub pwszResultDir: windows_core::PWSTR,
    pub hCATStore: super::HANDLE,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "mssip", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPTCATMEMBER {
    pub cbStruct: super::DWORD,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub pwszFileName: windows_core::PWSTR,
    pub gSubjectType: windows_core::GUID,
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
    pub pwszP7File: windows_core::PWSTR,
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
pub type HCATINFO = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: super::DWORD, dwlocalerror: super::DWORD, pwszline: *mut super::WCHAR)>;
pub const szOID_CATALOG_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.3");
