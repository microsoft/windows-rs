#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut isize, pgsubsystem: ::core::option::Option<*const ::windows::core::GUID>, dwflags: u32) -> super::super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminAcquireContext ( phcatadmin : *mut isize , pgsubsystem : *const :: windows::core::GUID , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminAcquireContext(phcatadmin, ::core::mem::transmute(pgsubsystem.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<P0>(phcatadmin: *mut isize, pgsubsystem: ::core::option::Option<*const ::windows::core::GUID>, pwszhashalgorithm: P0, pstronghashpolicy: ::core::option::Option<*const super::CERT_STRONG_SIGN_PARA>, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminAcquireContext2 ( phcatadmin : *mut isize , pgsubsystem : *const :: windows::core::GUID , pwszhashalgorithm : :: windows::core::PCWSTR , pstronghashpolicy : *const super:: CERT_STRONG_SIGN_PARA , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminAcquireContext2(phcatadmin, ::core::mem::transmute(pgsubsystem.unwrap_or(::std::ptr::null())), pwszhashalgorithm.into_param().abi(), ::core::mem::transmute(pstronghashpolicy.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<P0, P1>(hcatadmin: isize, pwszcatalogfile: P0, pwszselectbasename: P1, dwflags: u32) -> isize
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminAddCatalog ( hcatadmin : isize , pwszcatalogfile : :: windows::core::PCWSTR , pwszselectbasename : :: windows::core::PCWSTR , dwflags : u32 ) -> isize );
    CryptCATAdminAddCatalog(hcatadmin, pwszcatalogfile.into_param().abi(), pwszselectbasename.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle<P0>(hfile: P0, pcbhash: *mut u32, pbhash: ::core::option::Option<*mut u8>, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminCalcHashFromFileHandle ( hfile : super::super::super::Foundation:: HANDLE , pcbhash : *mut u32 , pbhash : *mut u8 , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminCalcHashFromFileHandle(hfile.into_param().abi(), pcbhash, ::core::mem::transmute(pbhash.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2<P0>(hcatadmin: isize, hfile: P0, pcbhash: *mut u32, pbhash: ::core::option::Option<*mut u8>, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminCalcHashFromFileHandle2 ( hcatadmin : isize , hfile : super::super::super::Foundation:: HANDLE , pcbhash : *mut u32 , pbhash : *mut u8 , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminCalcHashFromFileHandle2(hcatadmin, hfile.into_param().abi(), pcbhash, ::core::mem::transmute(pbhash.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: isize, pbhash: &[u8], dwflags: u32, phprevcatinfo: ::core::option::Option<*mut isize>) -> isize {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminEnumCatalogFromHash ( hcatadmin : isize , pbhash : *const u8 , cbhash : u32 , dwflags : u32 , phprevcatinfo : *mut isize ) -> isize );
    CryptCATAdminEnumCatalogFromHash(hcatadmin, ::core::mem::transmute(pbhash.as_ptr()), pbhash.len() as _, dwflags, ::core::mem::transmute(phprevcatinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup<P0>(dwflags: u32, fresume: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminPauseServiceForBackup ( dwflags : u32 , fresume : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminPauseServiceForBackup(dwflags, fresume.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: isize, hcatinfo: isize, dwflags: u32) -> super::super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminReleaseCatalogContext ( hcatadmin : isize , hcatinfo : isize , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminReleaseCatalogContext(hcatadmin, hcatinfo, dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: isize, dwflags: u32) -> super::super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminReleaseContext ( hcatadmin : isize , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminReleaseContext(hcatadmin, dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog<P0>(hcatadmin: isize, pwszcatalogfile: P0, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminRemoveCatalog ( hcatadmin : isize , pwszcatalogfile : :: windows::core::PCWSTR , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminRemoveCatalog(hcatadmin, pwszcatalogfile.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath<P0>(hcatadmin: isize, pwszcatalogfile: P0, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAdminResolveCatalogPath ( hcatadmin : isize , pwszcatalogfile : :: windows::core::PCWSTR , pscatinfo : *mut CATALOG_INFO , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATAdminResolveCatalogPath(hcatadmin, pwszcatalogfile.into_param().abi(), pscatinfo, dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATAllocSortedMemberInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pwszreferencetag : :: windows::core::PCWSTR ) -> *mut CRYPTCATMEMBER );
    CryptCATAllocSortedMemberInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATCDFClose ( pcdf : *mut CRYPTCATCDF ) -> super::super::super::Foundation:: BOOL );
    CryptCATCDFClose(pcdf)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATCDFEnumAttributes ( pcdf : *mut CRYPTCATCDF , pmember : *mut CRYPTCATMEMBER , pprevattr : *mut CRYPTCATATTRIBUTE , pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATCDFEnumAttributes(pcdf, pmember, pprevattr, pfnparseerror)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATCDFEnumCatAttributes ( pcdf : *mut CRYPTCATCDF , pprevattr : *mut CRYPTCATATTRIBUTE , pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATCDFEnumCatAttributes(pcdf, pprevattr, pfnparseerror)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATCDFEnumMembers ( pcdf : *mut CRYPTCATCDF , pprevmember : *mut CRYPTCATMEMBER , pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK ) -> *mut CRYPTCATMEMBER );
    CryptCATCDFEnumMembers(pcdf, pprevmember, pfnparseerror)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCDFOpen<P0>(pwszfilepath: P0, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATCDFOpen ( pwszfilepath : :: windows::core::PCWSTR , pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK ) -> *mut CRYPTCATCDF );
    CryptCATCDFOpen(pwszfilepath.into_param().abi(), pfnparseerror)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATCatalogInfoFromContext(hcatinfo: isize, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> super::super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATCatalogInfoFromContext ( hcatinfo : isize , pscatinfo : *mut CATALOG_INFO , dwflags : u32 ) -> super::super::super::Foundation:: BOOL );
    CryptCATCatalogInfoFromContext(hcatinfo, pscatinfo, dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATClose<P0>(hcatalog: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATClose ( hcatalog : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: BOOL );
    CryptCATClose(hcatalog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATEnumerateAttr<P0>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATEnumerateAttr ( hcatalog : super::super::super::Foundation:: HANDLE , pcatmember : *mut CRYPTCATMEMBER , pprevattr : *mut CRYPTCATATTRIBUTE ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATEnumerateAttr(hcatalog.into_param().abi(), pcatmember, pprevattr)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr<P0>(hcatalog: P0, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATEnumerateCatAttr ( hcatalog : super::super::super::Foundation:: HANDLE , pprevattr : *mut CRYPTCATATTRIBUTE ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATEnumerateCatAttr(hcatalog.into_param().abi(), pprevattr)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATEnumerateMember<P0>(hcatalog: P0, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATEnumerateMember ( hcatalog : super::super::super::Foundation:: HANDLE , pprevmember : *mut CRYPTCATMEMBER ) -> *mut CRYPTCATMEMBER );
    CryptCATEnumerateMember(hcatalog.into_param().abi(), pprevmember)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo<P0>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER)
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATFreeSortedMemberInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pcatmember : *mut CRYPTCATMEMBER ) -> ( ) );
    CryptCATFreeSortedMemberInfo(hcatalog.into_param().abi(), pcatmember)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<P0, P1>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATGetAttrInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pcatmember : *mut CRYPTCATMEMBER , pwszreferencetag : :: windows::core::PCWSTR ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATGetAttrInfo(hcatalog.into_param().abi(), pcatmember, pwszreferencetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATGetCatAttrInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pwszreferencetag : :: windows::core::PCWSTR ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATGetCatAttrInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATGetMemberInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pwszreferencetag : :: windows::core::PCWSTR ) -> *mut CRYPTCATMEMBER );
    CryptCATGetMemberInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::super::super::Foundation::HANDLE {
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATHandleFromStore ( pcatstore : *mut CRYPTCATSTORE ) -> super::super::super::Foundation:: HANDLE );
    CryptCATHandleFromStore(pcatstore)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATOpen<P0>(pwszfilename: P0, fdwopenflags: CRYPTCAT_OPEN_FLAGS, hprov: usize, dwpublicversion: CRYPTCAT_VERSION, dwencodingtype: u32) -> super::super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATOpen ( pwszfilename : :: windows::core::PCWSTR , fdwopenflags : CRYPTCAT_OPEN_FLAGS , hprov : usize , dwpublicversion : CRYPTCAT_VERSION , dwencodingtype : u32 ) -> super::super::super::Foundation:: HANDLE );
    CryptCATOpen(pwszfilename.into_param().abi(), fdwopenflags, hprov, dwpublicversion, dwencodingtype)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATPersistStore<P0>(hcatalog: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATPersistStore ( hcatalog : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: BOOL );
    CryptCATPersistStore(hcatalog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<P0, P1>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATPutAttrInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pcatmember : *mut CRYPTCATMEMBER , pwszreferencetag : :: windows::core::PCWSTR , dwattrtypeandaction : u32 , cbdata : u32 , pbdata : *mut u8 ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATPutAttrInfo(hcatalog.into_param().abi(), pcatmember, pwszreferencetag.into_param().abi(), dwattrtypeandaction, cbdata, pbdata)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATPutCatAttrInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pwszreferencetag : :: windows::core::PCWSTR , dwattrtypeandaction : u32 , cbdata : u32 , pbdata : *mut u8 ) -> *mut CRYPTCATATTRIBUTE );
    CryptCATPutCatAttrInfo(hcatalog.into_param().abi(), pwszreferencetag.into_param().abi(), dwattrtypeandaction, cbdata, pbdata)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<P0, P1, P2>(hcatalog: P0, pwszfilename: P1, pwszreferencetag: P2, pgsubjecttype: *mut ::windows::core::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATPutMemberInfo ( hcatalog : super::super::super::Foundation:: HANDLE , pwszfilename : :: windows::core::PCWSTR , pwszreferencetag : :: windows::core::PCWSTR , pgsubjecttype : *mut :: windows::core::GUID , dwcertversion : u32 , cbsipindirectdata : u32 , pbsipindirectdata : *mut u8 ) -> *mut CRYPTCATMEMBER );
    CryptCATPutMemberInfo(hcatalog.into_param().abi(), pwszfilename.into_param().abi(), pwszreferencetag.into_param().abi(), pgsubjecttype, dwcertversion, cbsipindirectdata, pbsipindirectdata)
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptCATStoreFromHandle<P0>(hcatalog: P0) -> *mut CRYPTCATSTORE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn CryptCATStoreFromHandle ( hcatalog : super::super::super::Foundation:: HANDLE ) -> *mut CRYPTCATSTORE );
    CryptCATStoreFromHandle(hcatalog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCatalogFile<P0, P1>(hfile: P0, pwszfilename: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "wintrust.dll""system" fn IsCatalogFile ( hfile : super::super::super::Foundation:: HANDLE , pwszfilename : :: windows::core::PCWSTR ) -> super::super::super::Foundation:: BOOL );
    IsCatalogFile(hfile.into_param().abi(), pwszfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ADDCATALOG_HARDLINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ADDCATALOG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_AUTHENTICATED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_DATAASCII: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_DATABASE64: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_DATAREPLACE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_NAMEASCII: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_NAMEOBJID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_AREA_ATTRIBUTE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_AREA_HEADER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_AREA_MEMBER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_ATTR_TOOFEWVALUES: u32 = 131074u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_ATTR_TYPECOMBO: u32 = 131076u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_BAD_GUID_CONV: u32 = 131073u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_DUPLICATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_MEMBER_FILENOTFOUND: u32 = 65540u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_MEMBER_FILE_PATH: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_MEMBER_INDIRECTDATA: u32 = 65538u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_TAGNOTFOUND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_E_CDF_UNSUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_FILEEXT: ::windows::core::PCWSTR = ::windows::w!("CAT");
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_MAX_MEMBERTAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_MEMBER_SORTED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const szOID_CATALOG_LIST: ::windows::core::PCSTR = ::windows::s!("1.3.6.1.4.1.311.12.1.1");
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const szOID_CATALOG_LIST_MEMBER: ::windows::core::PCSTR = ::windows::s!("1.3.6.1.4.1.311.12.1.2");
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const szOID_CATALOG_LIST_MEMBER2: ::windows::core::PCSTR = ::windows::s!("1.3.6.1.4.1.311.12.1.3");
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRYPTCAT_OPEN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_ALWAYS: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_CREATENEW: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_EXISTING: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_EXCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_SORTED: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_OPEN_FLAGS_MASK: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4294901760u32);
impl ::core::marker::Copy for CRYPTCAT_OPEN_FLAGS {}
impl ::core::clone::Clone for CRYPTCAT_OPEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTCAT_OPEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRYPTCAT_OPEN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRYPTCAT_OPEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTCAT_OPEN_FLAGS").field(&self.0).finish()
    }
}
impl CRYPTCAT_OPEN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRYPTCAT_VERSION(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_VERSION_1: CRYPTCAT_VERSION = CRYPTCAT_VERSION(256u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub const CRYPTCAT_VERSION_2: CRYPTCAT_VERSION = CRYPTCAT_VERSION(512u32);
impl ::core::marker::Copy for CRYPTCAT_VERSION {}
impl ::core::clone::Clone for CRYPTCAT_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTCAT_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRYPTCAT_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRYPTCAT_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTCAT_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub struct CATALOG_INFO {
    pub cbStruct: u32,
    pub wszCatalogFile: [u16; 260],
}
impl ::core::marker::Copy for CATALOG_INFO {}
impl ::core::clone::Clone for CATALOG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CATALOG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATALOG_INFO").field("cbStruct", &self.cbStruct).field("wszCatalogFile", &self.wszCatalogFile).finish()
    }
}
impl ::windows::core::TypeKind for CATALOG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.wszCatalogFile == other.wszCatalogFile
    }
}
impl ::core::cmp::Eq for CATALOG_INFO {}
impl ::core::default::Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: ::windows::core::PWSTR,
    pub dwAttrTypeAndAction: u32,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for CRYPTCATATTRIBUTE {}
impl ::core::clone::Clone for CRYPTCATATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPTCATATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATATTRIBUTE").field("cbStruct", &self.cbStruct).field("pwszReferenceTag", &self.pwszReferenceTag).field("dwAttrTypeAndAction", &self.dwAttrTypeAndAction).field("cbValue", &self.cbValue).field("pbValue", &self.pbValue).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows::core::TypeKind for CRYPTCATATTRIBUTE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CRYPTCATATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszReferenceTag == other.pwszReferenceTag && self.dwAttrTypeAndAction == other.dwAttrTypeAndAction && self.cbValue == other.cbValue && self.pbValue == other.pbValue && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for CRYPTCATATTRIBUTE {}
impl ::core::default::Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: super::super::super::Foundation::BOOL,
    pub pwszResultDir: ::windows::core::PWSTR,
    pub hCATStore: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTCATCDF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTCATCDF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATCDF").field("cbStruct", &self.cbStruct).field("hFile", &self.hFile).field("dwCurFilePos", &self.dwCurFilePos).field("dwLastMemberOffset", &self.dwLastMemberOffset).field("fEOF", &self.fEOF).field("pwszResultDir", &self.pwszResultDir).field("hCATStore", &self.hCATStore).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CRYPTCATCDF {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTCATCDF {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.hFile == other.hFile && self.dwCurFilePos == other.dwCurFilePos && self.dwLastMemberOffset == other.dwLastMemberOffset && self.fEOF == other.fEOF && self.pwszResultDir == other.pwszResultDir && self.hCATStore == other.hCATStore
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTCATCDF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: ::windows::core::PWSTR,
    pub pwszFileName: ::windows::core::PWSTR,
    pub gSubjectType: ::windows::core::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut super::Sip::SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::super::super::Foundation::HANDLE,
    pub sEncodedIndirectData: super::CRYPT_INTEGER_BLOB,
    pub sEncodedMemberInfo: super::CRYPT_INTEGER_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPTCATMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPTCATMEMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for CRYPTCATMEMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATMEMBER")
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
impl ::windows::core::TypeKind for CRYPTCATMEMBER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for CRYPTCATMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszReferenceTag == other.pwszReferenceTag && self.pwszFileName == other.pwszFileName && self.gSubjectType == other.gSubjectType && self.fdwMemberFlags == other.fdwMemberFlags && self.pIndirectData == other.pIndirectData && self.dwCertVersion == other.dwCertVersion && self.dwReserved == other.dwReserved && self.hReserved == other.hReserved && self.sEncodedIndirectData == other.sEncodedIndirectData && self.sEncodedMemberInfo == other.sEncodedMemberInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for CRYPTCATMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: ::windows::core::PWSTR,
    pub hProv: usize,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: CRYPTCAT_OPEN_FLAGS,
    pub hReserved: super::super::super::Foundation::HANDLE,
    pub hAttrs: super::super::super::Foundation::HANDLE,
    pub hCryptMsg: *mut ::core::ffi::c_void,
    pub hSorted: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTCATSTORE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTCATSTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATSTORE").field("cbStruct", &self.cbStruct).field("dwPublicVersion", &self.dwPublicVersion).field("pwszP7File", &self.pwszP7File).field("hProv", &self.hProv).field("dwEncodingType", &self.dwEncodingType).field("fdwStoreFlags", &self.fdwStoreFlags).field("hReserved", &self.hReserved).field("hAttrs", &self.hAttrs).field("hCryptMsg", &self.hCryptMsg).field("hSorted", &self.hSorted).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CRYPTCATSTORE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTCATSTORE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwPublicVersion == other.dwPublicVersion && self.pwszP7File == other.pwszP7File && self.hProv == other.hProv && self.dwEncodingType == other.dwEncodingType && self.fdwStoreFlags == other.fdwStoreFlags && self.hReserved == other.hReserved && self.hAttrs == other.hAttrs && self.hCryptMsg == other.hCryptMsg && self.hSorted == other.hSorted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut CRYPTCATSTORE,
    pub pMember: *mut CRYPTCATMEMBER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for MS_ADDINFO_CATALOGMEMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for MS_ADDINFO_CATALOGMEMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MS_ADDINFO_CATALOGMEMBER").field("cbStruct", &self.cbStruct).field("pStore", &self.pStore).field("pMember", &self.pMember).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::windows::core::TypeKind for MS_ADDINFO_CATALOGMEMBER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for MS_ADDINFO_CATALOGMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pStore == other.pStore && self.pMember == other.pMember
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Catalog\"`*"]
pub type PFN_CDF_PARSE_ERROR_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: ::windows::core::PCWSTR) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
