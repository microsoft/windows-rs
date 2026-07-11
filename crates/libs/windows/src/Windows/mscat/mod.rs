#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut super::wintrust::HCATADMIN, pgsubsystem: Option<*const windows_core::GUID>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut super::wintrust::HCATADMIN, pgsubsystem : *const windows_core::GUID, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<P2>(phcatadmin: *mut super::wintrust::HCATADMIN, pgsubsystem: Option<*const windows_core::GUID>, pwszhashalgorithm: P2, pstronghashpolicy: Option<*const super::wincrypt::CERT_STRONG_SIGN_PARA>, dwflags: Option<u32>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut super::wintrust::HCATADMIN, pgsubsystem : *const windows_core::GUID, pwszhashalgorithm : windows_core::PCWSTR, pstronghashpolicy : *const super::wincrypt::CERT_STRONG_SIGN_PARA, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext2(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, pwszhashalgorithm.param().abi(), pstronghashpolicy.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<P1, P2>(hcatadmin: super::wintrust::HCATADMIN, pwszcatalogfile: P1, pwszselectbasename: P2, dwflags: u32) -> HCATINFO
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : super::wintrust::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, pwszselectbasename : windows_core::PCWSTR, dwflags : u32) -> HCATINFO);
    unsafe { CryptCATAdminAddCatalog(hcatadmin, pwszcatalogfile.param().abi(), pwszselectbasename.param().abi(), dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle(hfile: super::winnt::HANDLE, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::winnt::HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle(hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin: super::wintrust::HCATADMIN, hfile: super::winnt::HANDLE, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : super::wintrust::HCATADMIN, hfile : super::winnt::HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle2(hcatadmin, hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: super::wintrust::HCATADMIN, pbhash: &[u8], dwflags: Option<u32>, phprevcatinfo: Option<*mut HCATINFO>) -> HCATINFO {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : super::wintrust::HCATADMIN, pbhash : *const u8, cbhash : u32, dwflags : u32, phprevcatinfo : *mut HCATINFO) -> HCATINFO);
    unsafe { CryptCATAdminEnumCatalogFromHash(hcatadmin, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), dwflags.unwrap_or(core::mem::zeroed()) as _, phprevcatinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup(dwflags: u32, fresume: bool) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : u32, fresume : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CryptCATAdminPauseServiceForBackup(dwflags, fresume.into()) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: super::wintrust::HCATADMIN, hcatinfo: HCATINFO, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : super::wintrust::HCATADMIN, hcatinfo : HCATINFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseCatalogContext(hcatadmin, hcatinfo, dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: super::wintrust::HCATADMIN, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : super::wintrust::HCATADMIN, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseContext(hcatadmin, dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog<P1>(hcatadmin: super::wintrust::HCATADMIN, pwszcatalogfile: P1, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : super::wintrust::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminRemoveCatalog(hcatadmin, pwszcatalogfile.param().abi(), dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wintrust"))]
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath<P1>(hcatadmin: super::wintrust::HCATADMIN, pwszcatalogfile: P1, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : super::wintrust::HCATADMIN, pwszcatalogfile : windows_core::PCWSTR, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminResolveCatalogPath(hcatadmin, pwszcatalogfile.param().abi(), pscatinfo as _, dwflags) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<P1>(hcatalog: super::winnt::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATAllocSortedMemberInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *mut CRYPTCATCDF) -> windows_core::BOOL);
    unsafe { CryptCATCDFClose(pcdf as _) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *mut CRYPTCATCDF, pmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumAttributes(pcdf as _, pmember as _, pprevattr as _, pfnparseerror) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumCatAttributes(pcdf as _, pprevattr as _, pfnparseerror) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *mut CRYPTCATCDF, pprevmember : *mut CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATCDFEnumMembers(pcdf as _, pprevmember as _, pfnparseerror) }
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
pub unsafe fn CryptCATClose(hcatalog: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATClose(hcatalog) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateAttr(hcatalog: super::winnt::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateAttr(hcatalog, pcatmember as _, pprevattr as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr(hcatalog: super::winnt::HANDLE, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::winnt::HANDLE, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateCatAttr(hcatalog, pprevattr as _) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATEnumerateMember(hcatalog: super::winnt::HANDLE, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::winnt::HANDLE, pprevmember : *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATEnumerateMember(hcatalog, pprevmember as _) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo(hcatalog: super::winnt::HANDLE, pcatmember: *mut CRYPTCATMEMBER) {
    windows_core::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER));
    unsafe { CryptCATFreeSortedMemberInfo(hcatalog, pcatmember as _) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<P2>(hcatalog: super::winnt::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P2) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetAttrInfo(hcatalog, pcatmember as _, pwszreferencetag.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<P1>(hcatalog: super::winnt::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetCatAttrInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<P1>(hcatalog: super::winnt::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATGetMemberInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::winnt::HANDLE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *mut CRYPTCATSTORE) -> super::winnt::HANDLE);
    unsafe { CryptCATHandleFromStore(pcatstore as _) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATOpen<P0>(pwszfilename: P0, fdwopenflags: u32, hprov: super::wincrypt::HCRYPTPROV, dwpublicversion: u32, dwencodingtype: u32) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_core::PCWSTR, fdwopenflags : u32, hprov : super::wincrypt::HCRYPTPROV, dwpublicversion : u32, dwencodingtype : u32) -> super::winnt::HANDLE);
    unsafe { CryptCATOpen(pwszfilename.param().abi(), fdwopenflags, hprov, dwpublicversion, dwencodingtype) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATPersistStore(hcatalog: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATPersistStore(hcatalog) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<P2>(hcatalog: super::winnt::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P2, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::winnt::HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutAttrInfo(hcatalog, pcatmember as _, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<P1>(hcatalog: super::winnt::HANDLE, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::winnt::HANDLE, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutCatAttrInfo(hcatalog, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata as _) }
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<P1, P2>(hcatalog: super::winnt::HANDLE, pwszfilename: P1, pwszreferencetag: P2, pgsubjecttype: *mut windows_core::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::winnt::HANDLE, pwszfilename : windows_core::PCWSTR, pwszreferencetag : windows_core::PCWSTR, pgsubjecttype : *mut windows_core::GUID, dwcertversion : u32, cbsipindirectdata : u32, pbsipindirectdata : *mut u8) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATPutMemberInfo(hcatalog, pwszfilename.param().abi(), pwszreferencetag.param().abi(), pgsubjecttype as _, dwcertversion, cbsipindirectdata, pbsipindirectdata as _) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptCATStoreFromHandle(hcatalog: super::winnt::HANDLE) -> *mut CRYPTCATSTORE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::winnt::HANDLE) -> *mut CRYPTCATSTORE);
    unsafe { CryptCATStoreFromHandle(hcatalog) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsCatalogFile<P1>(hfile: super::winnt::HANDLE, pwszfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::winnt::HANDLE, pwszfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IsCatalogFile(hfile, pwszfilename.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::winnt::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: windows_core::BOOL,
    pub pwszResultDir: windows_core::PWSTR,
    pub hCATStore: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub pwszFileName: windows_core::PWSTR,
    pub gSubjectType: windows_core::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut super::mssip::SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::winnt::HANDLE,
    pub sEncodedIndirectData: super::wincrypt::CRYPT_ATTR_BLOB,
    pub sEncodedMemberInfo: super::wincrypt::CRYPT_ATTR_BLOB,
}
#[cfg(all(feature = "mssip", feature = "wincrypt", feature = "winnt"))]
impl Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: windows_core::PWSTR,
    pub hProv: super::wincrypt::HCRYPTPROV,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: u32,
    pub hReserved: super::winnt::HANDLE,
    pub hAttrs: super::winnt::HANDLE,
    pub hCryptMsg: super::wincrypt::HCRYPTMSG,
    pub hSorted: super::winnt::HANDLE,
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
pub const CRYPTCAT_FILEEXT: windows_core::PCWSTR = windows_core::w!("CAT");
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
#[cfg(feature = "winnt")]
pub type HCATINFO = super::winnt::HANDLE;
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: *mut u16)>;
pub const szOID_CATALOG_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.3");
