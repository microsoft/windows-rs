#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheCheckManifest<P0, P1>(pwszmasterurl: P0, pwszmanifesturl: P1, pbmanifestdata: &[u8], pbmanifestresponseheaders: &[u8], pestate: *mut APP_CACHE_STATE, phnewappcache: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheCheckManifest ( pwszmasterurl : ::windows::core::PCWSTR , pwszmanifesturl : ::windows::core::PCWSTR , pbmanifestdata : *const u8 , dwmanifestdatasize : u32 , pbmanifestresponseheaders : *const u8 , dwmanifestresponseheaderssize : u32 , pestate : *mut APP_CACHE_STATE , phnewappcache : *mut *mut ::core::ffi::c_void ) -> u32 );
    AppCacheCheckManifest(pwszmasterurl.into_param().abi(), pwszmanifesturl.into_param().abi(), ::core::mem::transmute(pbmanifestdata.as_ptr()), pbmanifestdata.len() as _, ::core::mem::transmute(pbmanifestresponseheaders.as_ptr()), pbmanifestresponseheaders.len() as _, pestate, phnewappcache)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheCloseHandle(happcache: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheCloseHandle ( happcache : *const ::core::ffi::c_void ) -> ( ) );
    AppCacheCloseHandle(happcache)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheCreateAndCommitFile<P0, P1>(happcache: *const ::core::ffi::c_void, pwszsourcefilepath: P0, pwszurl: P1, pbresponseheaders: &[u8]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheCreateAndCommitFile ( happcache : *const ::core::ffi::c_void , pwszsourcefilepath : ::windows::core::PCWSTR , pwszurl : ::windows::core::PCWSTR , pbresponseheaders : *const u8 , dwresponseheaderssize : u32 ) -> u32 );
    AppCacheCreateAndCommitFile(happcache, pwszsourcefilepath.into_param().abi(), pwszurl.into_param().abi(), ::core::mem::transmute(pbresponseheaders.as_ptr()), pbresponseheaders.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheDeleteGroup<P0>(pwszmanifesturl: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheDeleteGroup ( pwszmanifesturl : ::windows::core::PCWSTR ) -> u32 );
    AppCacheDeleteGroup(pwszmanifesturl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheDeleteIEGroup<P0>(pwszmanifesturl: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheDeleteIEGroup ( pwszmanifesturl : ::windows::core::PCWSTR ) -> u32 );
    AppCacheDeleteIEGroup(pwszmanifesturl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheDuplicateHandle(happcache: *const ::core::ffi::c_void, phduplicatedappcache: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheDuplicateHandle ( happcache : *const ::core::ffi::c_void , phduplicatedappcache : *mut *mut ::core::ffi::c_void ) -> u32 );
    AppCacheDuplicateHandle(happcache, phduplicatedappcache)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheFinalize(happcache: *const ::core::ffi::c_void, pbmanifestdata: &[u8], pestate: *mut APP_CACHE_FINALIZE_STATE) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheFinalize ( happcache : *const ::core::ffi::c_void , pbmanifestdata : *const u8 , dwmanifestdatasize : u32 , pestate : *mut APP_CACHE_FINALIZE_STATE ) -> u32 );
    AppCacheFinalize(happcache, ::core::mem::transmute(pbmanifestdata.as_ptr()), pbmanifestdata.len() as _, pestate)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheFreeDownloadList(pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheFreeDownloadList ( pdownloadlist : *mut APP_CACHE_DOWNLOAD_LIST ) -> ( ) );
    AppCacheFreeDownloadList(pdownloadlist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheFreeGroupList ( pappcachegrouplist : *mut APP_CACHE_GROUP_LIST ) -> ( ) );
    AppCacheFreeGroupList(pappcachegrouplist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeIESpace(ftcutoff: super::super::Foundation::FILETIME) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheFreeIESpace ( ftcutoff : super::super::Foundation:: FILETIME ) -> u32 );
    AppCacheFreeIESpace(::core::mem::transmute(ftcutoff))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeSpace(ftcutoff: super::super::Foundation::FILETIME) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheFreeSpace ( ftcutoff : super::super::Foundation:: FILETIME ) -> u32 );
    AppCacheFreeSpace(::core::mem::transmute(ftcutoff))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheGetDownloadList(happcache: *const ::core::ffi::c_void, pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheGetDownloadList ( happcache : *const ::core::ffi::c_void , pdownloadlist : *mut APP_CACHE_DOWNLOAD_LIST ) -> u32 );
    AppCacheGetDownloadList(happcache, pdownloadlist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheGetFallbackUrl<P0>(happcache: *const ::core::ffi::c_void, pwszurl: P0, ppwszfallbackurl: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheGetFallbackUrl ( happcache : *const ::core::ffi::c_void , pwszurl : ::windows::core::PCWSTR , ppwszfallbackurl : *mut ::windows::core::PWSTR ) -> u32 );
    AppCacheGetFallbackUrl(happcache, pwszurl.into_param().abi(), ppwszfallbackurl)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheGetGroupList ( pappcachegrouplist : *mut APP_CACHE_GROUP_LIST ) -> u32 );
    AppCacheGetGroupList(pappcachegrouplist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetIEGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheGetIEGroupList ( pappcachegrouplist : *mut APP_CACHE_GROUP_LIST ) -> u32 );
    AppCacheGetIEGroupList(pappcachegrouplist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetInfo(happcache: *const ::core::ffi::c_void, pappcacheinfo: *mut APP_CACHE_GROUP_INFO) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheGetInfo ( happcache : *const ::core::ffi::c_void , pappcacheinfo : *mut APP_CACHE_GROUP_INFO ) -> u32 );
    AppCacheGetInfo(happcache, pappcacheinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheGetManifestUrl(happcache: *const ::core::ffi::c_void, ppwszmanifesturl: *mut ::windows::core::PWSTR) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheGetManifestUrl ( happcache : *const ::core::ffi::c_void , ppwszmanifesturl : *mut ::windows::core::PWSTR ) -> u32 );
    AppCacheGetManifestUrl(happcache, ppwszmanifesturl)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn AppCacheLookup<P0>(pwszurl: P0, dwflags: u32, phappcache: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn AppCacheLookup ( pwszurl : ::windows::core::PCWSTR , dwflags : u32 , phappcache : *mut *mut ::core::ffi::c_void ) -> u32 );
    AppCacheLookup(pwszurl.into_param().abi(), dwflags, phappcache)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitUrlCacheEntryA<P0, P1, P2, P3>(lpszurlname: P0, lpszlocalfilename: P1, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpheaderinfo: ::core::option::Option<&[u8]>, lpszfileextension: P2, lpszoriginalurl: P3) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CommitUrlCacheEntryA ( lpszurlname : ::windows::core::PCSTR , lpszlocalfilename : ::windows::core::PCSTR , expiretime : super::super::Foundation:: FILETIME , lastmodifiedtime : super::super::Foundation:: FILETIME , cacheentrytype : u32 , lpheaderinfo : *const u8 , cchheaderinfo : u32 , lpszfileextension : ::windows::core::PCSTR , lpszoriginalurl : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    CommitUrlCacheEntryA(lpszurlname.into_param().abi(), lpszlocalfilename.into_param().abi(), ::core::mem::transmute(expiretime), ::core::mem::transmute(lastmodifiedtime), cacheentrytype, ::core::mem::transmute(lpheaderinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpheaderinfo.as_deref().map_or(0, |slice| slice.len() as _), lpszfileextension.into_param().abi(), lpszoriginalurl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitUrlCacheEntryBinaryBlob<P0>(pwszurlname: P0, dwtype: u32, ftexpiretime: super::super::Foundation::FILETIME, ftmodifiedtime: super::super::Foundation::FILETIME, pbblob: ::core::option::Option<&[u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CommitUrlCacheEntryBinaryBlob ( pwszurlname : ::windows::core::PCWSTR , dwtype : u32 , ftexpiretime : super::super::Foundation:: FILETIME , ftmodifiedtime : super::super::Foundation:: FILETIME , pbblob : *const u8 , cbblob : u32 ) -> u32 );
    CommitUrlCacheEntryBinaryBlob(pwszurlname.into_param().abi(), dwtype, ::core::mem::transmute(ftexpiretime), ::core::mem::transmute(ftmodifiedtime), ::core::mem::transmute(pbblob.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbblob.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitUrlCacheEntryW<P0, P1, P2, P3>(lpszurlname: P0, lpszlocalfilename: P1, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpszheaderinfo: ::core::option::Option<&[u16]>, lpszfileextension: P2, lpszoriginalurl: P3) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CommitUrlCacheEntryW ( lpszurlname : ::windows::core::PCWSTR , lpszlocalfilename : ::windows::core::PCWSTR , expiretime : super::super::Foundation:: FILETIME , lastmodifiedtime : super::super::Foundation:: FILETIME , cacheentrytype : u32 , lpszheaderinfo : ::windows::core::PCWSTR , cchheaderinfo : u32 , lpszfileextension : ::windows::core::PCWSTR , lpszoriginalurl : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    CommitUrlCacheEntryW(lpszurlname.into_param().abi(), lpszlocalfilename.into_param().abi(), ::core::mem::transmute(expiretime), ::core::mem::transmute(lastmodifiedtime), cacheentrytype, ::core::mem::transmute(lpszheaderinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszheaderinfo.as_deref().map_or(0, |slice| slice.len() as _), lpszfileextension.into_param().abi(), lpszoriginalurl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMD5SSOHash<P0, P1, P2>(pszchallengeinfo: P0, pwszrealm: P1, pwsztarget: P2, pbhexhash: *mut u8) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateMD5SSOHash ( pszchallengeinfo : ::windows::core::PCWSTR , pwszrealm : ::windows::core::PCWSTR , pwsztarget : ::windows::core::PCWSTR , pbhexhash : *mut u8 ) -> super::super::Foundation:: BOOL );
    CreateMD5SSOHash(pszchallengeinfo.into_param().abi(), pwszrealm.into_param().abi(), pwsztarget.into_param().abi(), pbhexhash)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheContainerA<P0, P1, P2>(name: P0, lpcacheprefix: P1, lpszcachepath: P2, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbbuffer: ::core::option::Option<*const u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateUrlCacheContainerA ( name : ::windows::core::PCSTR , lpcacheprefix : ::windows::core::PCSTR , lpszcachepath : ::windows::core::PCSTR , kbcachelimit : u32 , dwcontainertype : u32 , dwoptions : u32 , pvbuffer : *const ::core::ffi::c_void , cbbuffer : *const u32 ) -> super::super::Foundation:: BOOL );
    CreateUrlCacheContainerA(name.into_param().abi(), lpcacheprefix.into_param().abi(), lpszcachepath.into_param().abi(), kbcachelimit, dwcontainertype, dwoptions, ::core::mem::transmute(pvbuffer.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cbbuffer.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheContainerW<P0, P1, P2>(name: P0, lpcacheprefix: P1, lpszcachepath: P2, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbbuffer: ::core::option::Option<*const u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateUrlCacheContainerW ( name : ::windows::core::PCWSTR , lpcacheprefix : ::windows::core::PCWSTR , lpszcachepath : ::windows::core::PCWSTR , kbcachelimit : u32 , dwcontainertype : u32 , dwoptions : u32 , pvbuffer : *const ::core::ffi::c_void , cbbuffer : *const u32 ) -> super::super::Foundation:: BOOL );
    CreateUrlCacheContainerW(name.into_param().abi(), lpcacheprefix.into_param().abi(), lpszcachepath.into_param().abi(), kbcachelimit, dwcontainertype, dwoptions, ::core::mem::transmute(pvbuffer.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cbbuffer.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheEntryA<P0, P1>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P1, lpszfilename: &mut [u8; 260], dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateUrlCacheEntryA ( lpszurlname : ::windows::core::PCSTR , dwexpectedfilesize : u32 , lpszfileextension : ::windows::core::PCSTR , lpszfilename : ::windows::core::PSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    CreateUrlCacheEntryA(lpszurlname.into_param().abi(), dwexpectedfilesize, lpszfileextension.into_param().abi(), ::core::mem::transmute(lpszfilename.as_ptr()), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheEntryExW<P0, P1, P2>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P1, lpszfilename: &mut [u16; 260], dwreserved: u32, fpreserveincomingfilename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateUrlCacheEntryExW ( lpszurlname : ::windows::core::PCWSTR , dwexpectedfilesize : u32 , lpszfileextension : ::windows::core::PCWSTR , lpszfilename : ::windows::core::PWSTR , dwreserved : u32 , fpreserveincomingfilename : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CreateUrlCacheEntryExW(lpszurlname.into_param().abi(), dwexpectedfilesize, lpszfileextension.into_param().abi(), ::core::mem::transmute(lpszfilename.as_ptr()), dwreserved, fpreserveincomingfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheEntryW<P0, P1>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P1, lpszfilename: &mut [u16; 260], dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateUrlCacheEntryW ( lpszurlname : ::windows::core::PCWSTR , dwexpectedfilesize : u32 , lpszfileextension : ::windows::core::PCWSTR , lpszfilename : ::windows::core::PWSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    CreateUrlCacheEntryW(lpszurlname.into_param().abi(), dwexpectedfilesize, lpszfileextension.into_param().abi(), ::core::mem::transmute(lpszfilename.as_ptr()), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn CreateUrlCacheGroup(dwflags: u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> i64 {
    ::windows_targets::link ! ( "wininet.dll""system" fn CreateUrlCacheGroup ( dwflags : u32 , lpreserved : *const ::core::ffi::c_void ) -> i64 );
    CreateUrlCacheGroup(dwflags, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteIE3Cache<P0, P1, P2>(hwnd: P0, hinst: P1, lpszcmd: P2, ncmdshow: i32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteIE3Cache ( hwnd : super::super::Foundation:: HWND , hinst : super::super::Foundation:: HMODULE , lpszcmd : ::windows::core::PCSTR , ncmdshow : i32 ) -> u32 );
    DeleteIE3Cache(hwnd.into_param().abi(), hinst.into_param().abi(), lpszcmd.into_param().abi(), ncmdshow)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheContainerA<P0>(name: P0, dwoptions: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteUrlCacheContainerA ( name : ::windows::core::PCSTR , dwoptions : u32 ) -> super::super::Foundation:: BOOL );
    DeleteUrlCacheContainerA(name.into_param().abi(), dwoptions)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheContainerW<P0>(name: P0, dwoptions: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteUrlCacheContainerW ( name : ::windows::core::PCWSTR , dwoptions : u32 ) -> super::super::Foundation:: BOOL );
    DeleteUrlCacheContainerW(name.into_param().abi(), dwoptions)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheEntry<P0>(lpszurlname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteUrlCacheEntry ( lpszurlname : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeleteUrlCacheEntry(lpszurlname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheEntryA<P0>(lpszurlname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteUrlCacheEntryA ( lpszurlname : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeleteUrlCacheEntryA(lpszurlname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheEntryW<P0>(lpszurlname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteUrlCacheEntryW ( lpszurlname : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeleteUrlCacheEntryW(lpszurlname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheGroup(groupid: i64, dwflags: u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteUrlCacheGroup ( groupid : i64 , dwflags : u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DeleteUrlCacheGroup(groupid, dwflags, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteWpadCacheForNetworks(param0: WPAD_CACHE_DELETE) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn DeleteWpadCacheForNetworks ( param0 : WPAD_CACHE_DELETE ) -> super::super::Foundation:: BOOL );
    DeleteWpadCacheForNetworks(param0)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DetectAutoProxyUrl(pszautoproxyurl: &mut [u8], dwdetectflags: PROXY_AUTO_DETECT_TYPE) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn DetectAutoProxyUrl ( pszautoproxyurl : ::windows::core::PSTR , cchautoproxyurl : u32 , dwdetectflags : PROXY_AUTO_DETECT_TYPE ) -> super::super::Foundation:: BOOL );
    DetectAutoProxyUrl(::core::mem::transmute(pszautoproxyurl.as_ptr()), pszautoproxyurl.len() as _, dwdetectflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DoConnectoidsExist() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn DoConnectoidsExist ( ) -> super::super::Foundation:: BOOL );
    DoConnectoidsExist()
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExportCookieFileA<P0, P1>(szfilename: P0, fappend: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ExportCookieFileA ( szfilename : ::windows::core::PCSTR , fappend : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    ExportCookieFileA(szfilename.into_param().abi(), fappend.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExportCookieFileW<P0, P1>(szfilename: P0, fappend: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ExportCookieFileW ( szfilename : ::windows::core::PCWSTR , fappend : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    ExportCookieFileW(szfilename.into_param().abi(), fappend.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindCloseUrlCache<P0>(henumhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindCloseUrlCache ( henumhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    FindCloseUrlCache(henumhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheContainerA(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE {
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheContainerA ( pdwmodified : *mut u32 , lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOA , lpcbcontainerinfo : *mut u32 , dwoptions : u32 ) -> super::super::Foundation:: HANDLE );
    FindFirstUrlCacheContainerA(pdwmodified, lpcontainerinfo, lpcbcontainerinfo, dwoptions)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheContainerW(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE {
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheContainerW ( pdwmodified : *mut u32 , lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOW , lpcbcontainerinfo : *mut u32 , dwoptions : u32 ) -> super::super::Foundation:: HANDLE );
    FindFirstUrlCacheContainerW(pdwmodified, lpcontainerinfo, lpcbcontainerinfo, dwoptions)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryA<P0>(lpszurlsearchpattern: P0, lpfirstcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheEntryA ( lpszurlsearchpattern : ::windows::core::PCSTR , lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = FindFirstUrlCacheEntryA(lpszurlsearchpattern.into_param().abi(), ::core::mem::transmute(lpfirstcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExA<P0>(lpszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: ::core::option::Option<*const ::core::ffi::c_void>, lpcbgroupattributes: ::core::option::Option<*const u32>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheEntryExA ( lpszurlsearchpattern : ::windows::core::PCSTR , dwflags : u32 , dwfilter : u32 , groupid : i64 , lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 , lpgroupattributes : *const ::core::ffi::c_void , lpcbgroupattributes : *const u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    let result__ = FindFirstUrlCacheEntryExA(lpszurlsearchpattern.into_param().abi(), dwflags, dwfilter, groupid, ::core::mem::transmute(lpfirstcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, ::core::mem::transmute(lpgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcbgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExW<P0>(lpszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: ::core::option::Option<*const ::core::ffi::c_void>, lpcbgroupattributes: ::core::option::Option<*const u32>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheEntryExW ( lpszurlsearchpattern : ::windows::core::PCWSTR , dwflags : u32 , dwfilter : u32 , groupid : i64 , lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 , lpgroupattributes : *const ::core::ffi::c_void , lpcbgroupattributes : *const u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    let result__ = FindFirstUrlCacheEntryExW(lpszurlsearchpattern.into_param().abi(), dwflags, dwfilter, groupid, ::core::mem::transmute(lpfirstcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, ::core::mem::transmute(lpgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcbgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryW<P0>(lpszurlsearchpattern: P0, lpfirstcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheEntryW ( lpszurlsearchpattern : ::windows::core::PCWSTR , lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = FindFirstUrlCacheEntryW(lpszurlsearchpattern.into_param().abi(), ::core::mem::transmute(lpfirstcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheGroup(dwflags: u32, dwfilter: u32, lpsearchcondition: ::core::option::Option<*const ::core::ffi::c_void>, dwsearchcondition: u32, lpgroupid: *mut i64, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link ! ( "wininet.dll""system" fn FindFirstUrlCacheGroup ( dwflags : u32 , dwfilter : u32 , lpsearchcondition : *const ::core::ffi::c_void , dwsearchcondition : u32 , lpgroupid : *mut i64 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    let result__ = FindFirstUrlCacheGroup(dwflags, dwfilter, ::core::mem::transmute(lpsearchcondition.unwrap_or(::std::ptr::null())), dwsearchcondition, lpgroupid, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheContainerA<P0>(henumhandle: P0, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheContainerA ( henumhandle : super::super::Foundation:: HANDLE , lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOA , lpcbcontainerinfo : *mut u32 ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheContainerA(henumhandle.into_param().abi(), lpcontainerinfo, lpcbcontainerinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheContainerW<P0>(henumhandle: P0, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheContainerW ( henumhandle : super::super::Foundation:: HANDLE , lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOW , lpcbcontainerinfo : *mut u32 ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheContainerW(henumhandle.into_param().abi(), lpcontainerinfo, lpcbcontainerinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryA<P0>(henumhandle: P0, lpnextcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheEntryA ( henumhandle : super::super::Foundation:: HANDLE , lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheEntryA(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryExA<P0>(henumhandle: P0, lpnextcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: ::core::option::Option<*const ::core::ffi::c_void>, lpcbgroupattributes: ::core::option::Option<*const u32>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheEntryExA ( henumhandle : super::super::Foundation:: HANDLE , lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 , lpgroupattributes : *const ::core::ffi::c_void , lpcbgroupattributes : *const u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheEntryExA(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, ::core::mem::transmute(lpgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcbgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryExW<P0>(henumhandle: P0, lpnextcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: ::core::option::Option<*const ::core::ffi::c_void>, lpcbgroupattributes: ::core::option::Option<*const u32>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheEntryExW ( henumhandle : super::super::Foundation:: HANDLE , lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 , lpgroupattributes : *const ::core::ffi::c_void , lpcbgroupattributes : *const u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheEntryExW(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, ::core::mem::transmute(lpgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcbgroupattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryW<P0>(henumhandle: P0, lpnextcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheEntryW ( henumhandle : super::super::Foundation:: HANDLE , lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheEntryW(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheGroup<P0>(hfind: P0, lpgroupid: *mut i64, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindNextUrlCacheGroup ( hfind : super::super::Foundation:: HANDLE , lpgroupid : *mut i64 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FindNextUrlCacheGroup(hfind.into_param().abi(), lpgroupid, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn FindP3PPolicySymbol<P0>(pszsymbol: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FindP3PPolicySymbol ( pszsymbol : ::windows::core::PCSTR ) -> i32 );
    FindP3PPolicySymbol(pszsymbol.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUrlCacheSpaceA<P0>(lpszcachepath: P0, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FreeUrlCacheSpaceA ( lpszcachepath : ::windows::core::PCSTR , dwsize : u32 , dwfilter : u32 ) -> super::super::Foundation:: BOOL );
    FreeUrlCacheSpaceA(lpszcachepath.into_param().abi(), dwsize, dwfilter)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUrlCacheSpaceW<P0>(lpszcachepath: P0, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FreeUrlCacheSpaceW ( lpszcachepath : ::windows::core::PCWSTR , dwsize : u32 , dwfilter : u32 ) -> super::super::Foundation:: BOOL );
    FreeUrlCacheSpaceW(lpszcachepath.into_param().abi(), dwsize, dwfilter)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCommandA<P0, P1>(hconnect: *const ::core::ffi::c_void, fexpectresponse: P0, dwflags: FTP_FLAGS, lpszcommand: P1, dwcontext: usize, phftpcommand: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpCommandA ( hconnect : *const ::core::ffi::c_void , fexpectresponse : super::super::Foundation:: BOOL , dwflags : FTP_FLAGS , lpszcommand : ::windows::core::PCSTR , dwcontext : usize , phftpcommand : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FtpCommandA(hconnect, fexpectresponse.into_param().abi(), dwflags, lpszcommand.into_param().abi(), dwcontext, ::core::mem::transmute(phftpcommand.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCommandW<P0, P1>(hconnect: *const ::core::ffi::c_void, fexpectresponse: P0, dwflags: FTP_FLAGS, lpszcommand: P1, dwcontext: usize, phftpcommand: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpCommandW ( hconnect : *const ::core::ffi::c_void , fexpectresponse : super::super::Foundation:: BOOL , dwflags : FTP_FLAGS , lpszcommand : ::windows::core::PCWSTR , dwcontext : usize , phftpcommand : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FtpCommandW(hconnect, fexpectresponse.into_param().abi(), dwflags, lpszcommand.into_param().abi(), dwcontext, ::core::mem::transmute(phftpcommand.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCreateDirectoryA<P0>(hconnect: *const ::core::ffi::c_void, lpszdirectory: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpCreateDirectoryA ( hconnect : *const ::core::ffi::c_void , lpszdirectory : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    FtpCreateDirectoryA(hconnect, lpszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCreateDirectoryW<P0>(hconnect: *const ::core::ffi::c_void, lpszdirectory: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpCreateDirectoryW ( hconnect : *const ::core::ffi::c_void , lpszdirectory : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FtpCreateDirectoryW(hconnect, lpszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpDeleteFileA<P0>(hconnect: *const ::core::ffi::c_void, lpszfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpDeleteFileA ( hconnect : *const ::core::ffi::c_void , lpszfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    FtpDeleteFileA(hconnect, lpszfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpDeleteFileW<P0>(hconnect: *const ::core::ffi::c_void, lpszfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpDeleteFileW ( hconnect : *const ::core::ffi::c_void , lpszfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FtpDeleteFileW(hconnect, lpszfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn FtpFindFirstFileA<P0>(hconnect: *const ::core::ffi::c_void, lpszsearchfile: P0, lpfindfiledata: ::core::option::Option<*mut super::super::Storage::FileSystem::WIN32_FIND_DATAA>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpFindFirstFileA ( hconnect : *const ::core::ffi::c_void , lpszsearchfile : ::windows::core::PCSTR , lpfindfiledata : *mut super::super::Storage::FileSystem:: WIN32_FIND_DATAA , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    FtpFindFirstFileA(hconnect, lpszsearchfile.into_param().abi(), ::core::mem::transmute(lpfindfiledata.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn FtpFindFirstFileW<P0>(hconnect: *const ::core::ffi::c_void, lpszsearchfile: P0, lpfindfiledata: ::core::option::Option<*mut super::super::Storage::FileSystem::WIN32_FIND_DATAW>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpFindFirstFileW ( hconnect : *const ::core::ffi::c_void , lpszsearchfile : ::windows::core::PCWSTR , lpfindfiledata : *mut super::super::Storage::FileSystem:: WIN32_FIND_DATAW , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    FtpFindFirstFileW(hconnect, lpszsearchfile.into_param().abi(), ::core::mem::transmute(lpfindfiledata.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetCurrentDirectoryA(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: ::windows::core::PSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpGetCurrentDirectoryA ( hconnect : *const ::core::ffi::c_void , lpszcurrentdirectory : ::windows::core::PSTR , lpdwcurrentdirectory : *mut u32 ) -> super::super::Foundation:: BOOL );
    FtpGetCurrentDirectoryA(hconnect, ::core::mem::transmute(lpszcurrentdirectory), lpdwcurrentdirectory)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetCurrentDirectoryW(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: ::windows::core::PWSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpGetCurrentDirectoryW ( hconnect : *const ::core::ffi::c_void , lpszcurrentdirectory : ::windows::core::PWSTR , lpdwcurrentdirectory : *mut u32 ) -> super::super::Foundation:: BOOL );
    FtpGetCurrentDirectoryW(hconnect, ::core::mem::transmute(lpszcurrentdirectory), lpdwcurrentdirectory)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetFileA<P0, P1, P2>(hconnect: *const ::core::ffi::c_void, lpszremotefile: P0, lpsznewfile: P1, ffailifexists: P2, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpGetFileA ( hconnect : *const ::core::ffi::c_void , lpszremotefile : ::windows::core::PCSTR , lpsznewfile : ::windows::core::PCSTR , ffailifexists : super::super::Foundation:: BOOL , dwflagsandattributes : u32 , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    FtpGetFileA(hconnect, lpszremotefile.into_param().abi(), lpsznewfile.into_param().abi(), ffailifexists.into_param().abi(), dwflagsandattributes, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetFileEx<P0, P1, P2>(hftpsession: *const ::core::ffi::c_void, lpszremotefile: P0, lpsznewfile: P1, ffailifexists: P2, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpGetFileEx ( hftpsession : *const ::core::ffi::c_void , lpszremotefile : ::windows::core::PCSTR , lpsznewfile : ::windows::core::PCWSTR , ffailifexists : super::super::Foundation:: BOOL , dwflagsandattributes : u32 , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    FtpGetFileEx(hftpsession, lpszremotefile.into_param().abi(), lpsznewfile.into_param().abi(), ffailifexists.into_param().abi(), dwflagsandattributes, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn FtpGetFileSize(hfile: *const ::core::ffi::c_void, lpdwfilesizehigh: ::core::option::Option<*mut u32>) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpGetFileSize ( hfile : *const ::core::ffi::c_void , lpdwfilesizehigh : *mut u32 ) -> u32 );
    FtpGetFileSize(hfile, ::core::mem::transmute(lpdwfilesizehigh.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetFileW<P0, P1, P2>(hconnect: *const ::core::ffi::c_void, lpszremotefile: P0, lpsznewfile: P1, ffailifexists: P2, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpGetFileW ( hconnect : *const ::core::ffi::c_void , lpszremotefile : ::windows::core::PCWSTR , lpsznewfile : ::windows::core::PCWSTR , ffailifexists : super::super::Foundation:: BOOL , dwflagsandattributes : u32 , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    FtpGetFileW(hconnect, lpszremotefile.into_param().abi(), lpsznewfile.into_param().abi(), ffailifexists.into_param().abi(), dwflagsandattributes, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn FtpOpenFileA<P0>(hconnect: *const ::core::ffi::c_void, lpszfilename: P0, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpOpenFileA ( hconnect : *const ::core::ffi::c_void , lpszfilename : ::windows::core::PCSTR , dwaccess : u32 , dwflags : FTP_FLAGS , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    FtpOpenFileA(hconnect, lpszfilename.into_param().abi(), dwaccess, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn FtpOpenFileW<P0>(hconnect: *const ::core::ffi::c_void, lpszfilename: P0, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpOpenFileW ( hconnect : *const ::core::ffi::c_void , lpszfilename : ::windows::core::PCWSTR , dwaccess : u32 , dwflags : FTP_FLAGS , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    FtpOpenFileW(hconnect, lpszfilename.into_param().abi(), dwaccess, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpPutFileA<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocalfile: P0, lpsznewremotefile: P1, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpPutFileA ( hconnect : *const ::core::ffi::c_void , lpszlocalfile : ::windows::core::PCSTR , lpsznewremotefile : ::windows::core::PCSTR , dwflags : FTP_FLAGS , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    FtpPutFileA(hconnect, lpszlocalfile.into_param().abi(), lpsznewremotefile.into_param().abi(), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpPutFileEx<P0, P1>(hftpsession: *const ::core::ffi::c_void, lpszlocalfile: P0, lpsznewremotefile: P1, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpPutFileEx ( hftpsession : *const ::core::ffi::c_void , lpszlocalfile : ::windows::core::PCWSTR , lpsznewremotefile : ::windows::core::PCSTR , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    FtpPutFileEx(hftpsession, lpszlocalfile.into_param().abi(), lpsznewremotefile.into_param().abi(), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpPutFileW<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocalfile: P0, lpsznewremotefile: P1, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpPutFileW ( hconnect : *const ::core::ffi::c_void , lpszlocalfile : ::windows::core::PCWSTR , lpsznewremotefile : ::windows::core::PCWSTR , dwflags : FTP_FLAGS , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    FtpPutFileW(hconnect, lpszlocalfile.into_param().abi(), lpsznewremotefile.into_param().abi(), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRemoveDirectoryA<P0>(hconnect: *const ::core::ffi::c_void, lpszdirectory: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpRemoveDirectoryA ( hconnect : *const ::core::ffi::c_void , lpszdirectory : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    FtpRemoveDirectoryA(hconnect, lpszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRemoveDirectoryW<P0>(hconnect: *const ::core::ffi::c_void, lpszdirectory: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpRemoveDirectoryW ( hconnect : *const ::core::ffi::c_void , lpszdirectory : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FtpRemoveDirectoryW(hconnect, lpszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRenameFileA<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszexisting: P0, lpsznew: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpRenameFileA ( hconnect : *const ::core::ffi::c_void , lpszexisting : ::windows::core::PCSTR , lpsznew : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    FtpRenameFileA(hconnect, lpszexisting.into_param().abi(), lpsznew.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRenameFileW<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszexisting: P0, lpsznew: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpRenameFileW ( hconnect : *const ::core::ffi::c_void , lpszexisting : ::windows::core::PCWSTR , lpsznew : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FtpRenameFileW(hconnect, lpszexisting.into_param().abi(), lpsznew.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpSetCurrentDirectoryA<P0>(hconnect: *const ::core::ffi::c_void, lpszdirectory: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpSetCurrentDirectoryA ( hconnect : *const ::core::ffi::c_void , lpszdirectory : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    FtpSetCurrentDirectoryA(hconnect, lpszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpSetCurrentDirectoryW<P0>(hconnect: *const ::core::ffi::c_void, lpszdirectory: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn FtpSetCurrentDirectoryW ( hconnect : *const ::core::ffi::c_void , lpszdirectory : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FtpSetCurrentDirectoryW(hconnect, lpszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskInfoA<P0>(pszpath: P0, pdwclustersize: ::core::option::Option<*mut u32>, pdlavail: ::core::option::Option<*mut u64>, pdltotal: ::core::option::Option<*mut u64>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GetDiskInfoA ( pszpath : ::windows::core::PCSTR , pdwclustersize : *mut u32 , pdlavail : *mut u64 , pdltotal : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetDiskInfoA(pszpath.into_param().abi(), ::core::mem::transmute(pdwclustersize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdlavail.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdltotal.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheConfigInfoA(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOA, lpcbcacheconfiginfo: ::core::option::Option<*const u32>, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheConfigInfoA ( lpcacheconfiginfo : *mut INTERNET_CACHE_CONFIG_INFOA , lpcbcacheconfiginfo : *const u32 , dwfieldcontrol : CACHE_CONFIG ) -> super::super::Foundation:: BOOL );
    GetUrlCacheConfigInfoA(lpcacheconfiginfo, ::core::mem::transmute(lpcbcacheconfiginfo.unwrap_or(::std::ptr::null())), dwfieldcontrol)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheConfigInfoW(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOW, lpcbcacheconfiginfo: ::core::option::Option<*const u32>, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheConfigInfoW ( lpcacheconfiginfo : *mut INTERNET_CACHE_CONFIG_INFOW , lpcbcacheconfiginfo : *const u32 , dwfieldcontrol : CACHE_CONFIG ) -> super::super::Foundation:: BOOL );
    GetUrlCacheConfigInfoW(lpcacheconfiginfo, ::core::mem::transmute(lpcbcacheconfiginfo.unwrap_or(::std::ptr::null())), dwfieldcontrol)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryBinaryBlob<P0>(pwszurlname: P0, dwtype: *mut u32, pftexpiretime: *mut super::super::Foundation::FILETIME, pftaccesstime: *mut super::super::Foundation::FILETIME, pftmodifiedtime: *mut super::super::Foundation::FILETIME, ppbblob: *mut *mut u8, pcbblob: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheEntryBinaryBlob ( pwszurlname : ::windows::core::PCWSTR , dwtype : *mut u32 , pftexpiretime : *mut super::super::Foundation:: FILETIME , pftaccesstime : *mut super::super::Foundation:: FILETIME , pftmodifiedtime : *mut super::super::Foundation:: FILETIME , ppbblob : *mut *mut u8 , pcbblob : *mut u32 ) -> u32 );
    GetUrlCacheEntryBinaryBlob(pwszurlname.into_param().abi(), dwtype, pftexpiretime, pftaccesstime, pftmodifiedtime, ppbblob, pcbblob)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoA<P0>(lpszurlname: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheEntryInfoA ( lpszurlname : ::windows::core::PCSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetUrlCacheEntryInfoA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbcacheentryinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExA<P0, P1>(lpszurl: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: ::core::option::Option<*mut u32>, lpszredirecturl: P1, lpcbredirecturl: ::core::option::Option<*const u32>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheEntryInfoExA ( lpszurl : ::windows::core::PCSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 , lpszredirecturl : ::windows::core::PCSTR , lpcbredirecturl : *const u32 , lpreserved : *const ::core::ffi::c_void , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    GetUrlCacheEntryInfoExA(lpszurl.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpszredirecturl.into_param().abi(), ::core::mem::transmute(lpcbredirecturl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExW<P0, P1>(lpszurl: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: ::core::option::Option<*mut u32>, lpszredirecturl: P1, lpcbredirecturl: ::core::option::Option<*const u32>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheEntryInfoExW ( lpszurl : ::windows::core::PCWSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 , lpszredirecturl : ::windows::core::PCWSTR , lpcbredirecturl : *const u32 , lpreserved : *const ::core::ffi::c_void , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    GetUrlCacheEntryInfoExW(lpszurl.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpszredirecturl.into_param().abi(), ::core::mem::transmute(lpcbredirecturl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoW<P0>(lpszurlname: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheEntryInfoW ( lpszurlname : ::windows::core::PCWSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetUrlCacheEntryInfoW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbcacheentryinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo: *mut u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheGroupAttributeA ( gid : i64 , dwflags : u32 , dwattributes : u32 , lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOA , lpcbgroupinfo : *mut u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetUrlCacheGroupAttributeA(gid, dwflags, dwattributes, lpgroupinfo, lpcbgroupinfo, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo: *mut u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheGroupAttributeW ( gid : i64 , dwflags : u32 , dwattributes : u32 , lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOW , lpcbgroupinfo : *mut u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetUrlCacheGroupAttributeW(gid, dwflags, dwattributes, lpgroupinfo, lpcbgroupinfo, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn GetUrlCacheHeaderData ( nidx : u32 , lpdwdata : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetUrlCacheHeaderData(nidx, lpdwdata)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherCreateLocatorA<P0, P1, P2>(lpszhost: P0, nserverport: u16, lpszdisplaystring: P1, lpszselectorstring: P2, dwgophertype: u32, lpszlocator: ::windows::core::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherCreateLocatorA ( lpszhost : ::windows::core::PCSTR , nserverport : u16 , lpszdisplaystring : ::windows::core::PCSTR , lpszselectorstring : ::windows::core::PCSTR , dwgophertype : u32 , lpszlocator : ::windows::core::PSTR , lpdwbufferlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GopherCreateLocatorA(lpszhost.into_param().abi(), nserverport, lpszdisplaystring.into_param().abi(), lpszselectorstring.into_param().abi(), dwgophertype, ::core::mem::transmute(lpszlocator), lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherCreateLocatorW<P0, P1, P2>(lpszhost: P0, nserverport: u16, lpszdisplaystring: P1, lpszselectorstring: P2, dwgophertype: u32, lpszlocator: ::windows::core::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherCreateLocatorW ( lpszhost : ::windows::core::PCWSTR , nserverport : u16 , lpszdisplaystring : ::windows::core::PCWSTR , lpszselectorstring : ::windows::core::PCWSTR , dwgophertype : u32 , lpszlocator : ::windows::core::PWSTR , lpdwbufferlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GopherCreateLocatorW(lpszhost.into_param().abi(), nserverport, lpszdisplaystring.into_param().abi(), lpszselectorstring.into_param().abi(), dwgophertype, ::core::mem::transmute(lpszlocator), lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherFindFirstFileA<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocator: P0, lpszsearchstring: P1, lpfinddata: ::core::option::Option<*mut GOPHER_FIND_DATAA>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherFindFirstFileA ( hconnect : *const ::core::ffi::c_void , lpszlocator : ::windows::core::PCSTR , lpszsearchstring : ::windows::core::PCSTR , lpfinddata : *mut GOPHER_FIND_DATAA , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    GopherFindFirstFileA(hconnect, lpszlocator.into_param().abi(), lpszsearchstring.into_param().abi(), ::core::mem::transmute(lpfinddata.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherFindFirstFileW<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocator: P0, lpszsearchstring: P1, lpfinddata: ::core::option::Option<*mut GOPHER_FIND_DATAW>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherFindFirstFileW ( hconnect : *const ::core::ffi::c_void , lpszlocator : ::windows::core::PCWSTR , lpszsearchstring : ::windows::core::PCWSTR , lpfinddata : *mut GOPHER_FIND_DATAW , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    GopherFindFirstFileW(hconnect, lpszlocator.into_param().abi(), lpszsearchstring.into_param().abi(), ::core::mem::transmute(lpfinddata.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetAttributeA<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocator: P0, lpszattributename: P1, lpbuffer: &mut [u8], lpdwcharactersreturned: *mut u32, lpfnenumerator: GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherGetAttributeA ( hconnect : *const ::core::ffi::c_void , lpszlocator : ::windows::core::PCSTR , lpszattributename : ::windows::core::PCSTR , lpbuffer : *mut u8 , dwbufferlength : u32 , lpdwcharactersreturned : *mut u32 , lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    GopherGetAttributeA(hconnect, lpszlocator.into_param().abi(), lpszattributename.into_param().abi(), ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _, lpdwcharactersreturned, lpfnenumerator, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetAttributeW<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocator: P0, lpszattributename: P1, lpbuffer: &mut [u8], lpdwcharactersreturned: *mut u32, lpfnenumerator: GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherGetAttributeW ( hconnect : *const ::core::ffi::c_void , lpszlocator : ::windows::core::PCWSTR , lpszattributename : ::windows::core::PCWSTR , lpbuffer : *mut u8 , dwbufferlength : u32 , lpdwcharactersreturned : *mut u32 , lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    GopherGetAttributeW(hconnect, lpszlocator.into_param().abi(), lpszattributename.into_param().abi(), ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _, lpdwcharactersreturned, lpfnenumerator, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetLocatorTypeA<P0>(lpszlocator: P0, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherGetLocatorTypeA ( lpszlocator : ::windows::core::PCSTR , lpdwgophertype : *mut u32 ) -> super::super::Foundation:: BOOL );
    GopherGetLocatorTypeA(lpszlocator.into_param().abi(), lpdwgophertype)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetLocatorTypeW<P0>(lpszlocator: P0, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherGetLocatorTypeW ( lpszlocator : ::windows::core::PCWSTR , lpdwgophertype : *mut u32 ) -> super::super::Foundation:: BOOL );
    GopherGetLocatorTypeW(lpszlocator.into_param().abi(), lpdwgophertype)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn GopherOpenFileA<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocator: P0, lpszview: P1, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherOpenFileA ( hconnect : *const ::core::ffi::c_void , lpszlocator : ::windows::core::PCSTR , lpszview : ::windows::core::PCSTR , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    GopherOpenFileA(hconnect, lpszlocator.into_param().abi(), lpszview.into_param().abi(), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn GopherOpenFileW<P0, P1>(hconnect: *const ::core::ffi::c_void, lpszlocator: P0, lpszview: P1, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn GopherOpenFileW ( hconnect : *const ::core::ffi::c_void , lpszlocator : ::windows::core::PCWSTR , lpszview : ::windows::core::PCWSTR , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    GopherOpenFileW(hconnect, lpszlocator.into_param().abi(), lpszview.into_param().abi(), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpAddRequestHeadersA(hrequest: *const ::core::ffi::c_void, lpszheaders: &[u8], dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpAddRequestHeadersA ( hrequest : *const ::core::ffi::c_void , lpszheaders : ::windows::core::PCSTR , dwheaderslength : u32 , dwmodifiers : HTTP_ADDREQ_FLAG ) -> super::super::Foundation:: BOOL );
    HttpAddRequestHeadersA(hrequest, ::core::mem::transmute(lpszheaders.as_ptr()), lpszheaders.len() as _, dwmodifiers)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpAddRequestHeadersW(hrequest: *const ::core::ffi::c_void, lpszheaders: &[u16], dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpAddRequestHeadersW ( hrequest : *const ::core::ffi::c_void , lpszheaders : ::windows::core::PCWSTR , dwheaderslength : u32 , dwmodifiers : HTTP_ADDREQ_FLAG ) -> super::super::Foundation:: BOOL );
    HttpAddRequestHeadersW(hrequest, ::core::mem::transmute(lpszheaders.as_ptr()), lpszheaders.len() as _, dwmodifiers)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpCheckDavComplianceA<P0, P1, P2>(lpszurl: P0, lpszcompliancetoken: P1, lpffound: *mut i32, hwnd: P2, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpCheckDavComplianceA ( lpszurl : ::windows::core::PCSTR , lpszcompliancetoken : ::windows::core::PCSTR , lpffound : *mut i32 , hwnd : super::super::Foundation:: HWND , lpvreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    HttpCheckDavComplianceA(lpszurl.into_param().abi(), lpszcompliancetoken.into_param().abi(), lpffound, hwnd.into_param().abi(), lpvreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpCheckDavComplianceW<P0, P1, P2>(lpszurl: P0, lpszcompliancetoken: P1, lpffound: *mut i32, hwnd: P2, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpCheckDavComplianceW ( lpszurl : ::windows::core::PCWSTR , lpszcompliancetoken : ::windows::core::PCWSTR , lpffound : *mut i32 , hwnd : super::super::Foundation:: HWND , lpvreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    HttpCheckDavComplianceW(lpszurl.into_param().abi(), lpszcompliancetoken.into_param().abi(), lpffound, hwnd.into_param().abi(), lpvreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpCloseDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpCloseDependencyHandle ( hdependencyhandle : *const ::core::ffi::c_void ) -> ( ) );
    HttpCloseDependencyHandle(hdependencyhandle)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpDuplicateDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void, phduplicateddependencyhandle: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpDuplicateDependencyHandle ( hdependencyhandle : *const ::core::ffi::c_void , phduplicateddependencyhandle : *mut *mut ::core::ffi::c_void ) -> u32 );
    HttpDuplicateDependencyHandle(hdependencyhandle, phduplicateddependencyhandle)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpEndRequestA(hrequest: *const ::core::ffi::c_void, lpbuffersout: ::core::option::Option<*mut INTERNET_BUFFERSA>, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpEndRequestA ( hrequest : *const ::core::ffi::c_void , lpbuffersout : *mut INTERNET_BUFFERSA , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    HttpEndRequestA(hrequest, ::core::mem::transmute(lpbuffersout.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpEndRequestW(hrequest: *const ::core::ffi::c_void, lpbuffersout: ::core::option::Option<*mut INTERNET_BUFFERSW>, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpEndRequestW ( hrequest : *const ::core::ffi::c_void , lpbuffersout : *mut INTERNET_BUFFERSW , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    HttpEndRequestW(hrequest, ::core::mem::transmute(lpbuffersout.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpGetServerCredentials<P0>(pwszurl: P0, ppwszusername: *mut ::windows::core::PWSTR, ppwszpassword: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpGetServerCredentials ( pwszurl : ::windows::core::PCWSTR , ppwszusername : *mut ::windows::core::PWSTR , ppwszpassword : *mut ::windows::core::PWSTR ) -> u32 );
    HttpGetServerCredentials(pwszurl.into_param().abi(), ppwszusername, ppwszpassword)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpIndicatePageLoadComplete(hdependencyhandle: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpIndicatePageLoadComplete ( hdependencyhandle : *const ::core::ffi::c_void ) -> u32 );
    HttpIndicatePageLoadComplete(hdependencyhandle)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpIsHostHstsEnabled<P0>(pcwszurl: P0, pfishsts: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpIsHostHstsEnabled ( pcwszurl : ::windows::core::PCWSTR , pfishsts : *mut super::super::Foundation:: BOOL ) -> u32 );
    HttpIsHostHstsEnabled(pcwszurl.into_param().abi(), pfishsts)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpOpenDependencyHandle<P0>(hrequesthandle: *const ::core::ffi::c_void, fbackground: P0, phdependencyhandle: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpOpenDependencyHandle ( hrequesthandle : *const ::core::ffi::c_void , fbackground : super::super::Foundation:: BOOL , phdependencyhandle : *mut *mut ::core::ffi::c_void ) -> u32 );
    HttpOpenDependencyHandle(hrequesthandle, fbackground.into_param().abi(), phdependencyhandle)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpOpenRequestA<P0, P1, P2, P3>(hconnect: *const ::core::ffi::c_void, lpszverb: P0, lpszobjectname: P1, lpszversion: P2, lpszreferrer: P3, lplpszaccepttypes: ::core::option::Option<*const ::windows::core::PCSTR>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpOpenRequestA ( hconnect : *const ::core::ffi::c_void , lpszverb : ::windows::core::PCSTR , lpszobjectname : ::windows::core::PCSTR , lpszversion : ::windows::core::PCSTR , lpszreferrer : ::windows::core::PCSTR , lplpszaccepttypes : *const ::windows::core::PCSTR , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    HttpOpenRequestA(hconnect, lpszverb.into_param().abi(), lpszobjectname.into_param().abi(), lpszversion.into_param().abi(), lpszreferrer.into_param().abi(), ::core::mem::transmute(lplpszaccepttypes.unwrap_or(::std::ptr::null())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpOpenRequestW<P0, P1, P2, P3>(hconnect: *const ::core::ffi::c_void, lpszverb: P0, lpszobjectname: P1, lpszversion: P2, lpszreferrer: P3, lplpszaccepttypes: ::core::option::Option<*const ::windows::core::PCWSTR>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpOpenRequestW ( hconnect : *const ::core::ffi::c_void , lpszverb : ::windows::core::PCWSTR , lpszobjectname : ::windows::core::PCWSTR , lpszversion : ::windows::core::PCWSTR , lpszreferrer : ::windows::core::PCWSTR , lplpszaccepttypes : *const ::windows::core::PCWSTR , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    HttpOpenRequestW(hconnect, lpszverb.into_param().abi(), lpszobjectname.into_param().abi(), lpszversion.into_param().abi(), lpszreferrer.into_param().abi(), ::core::mem::transmute(lplpszaccepttypes.unwrap_or(::std::ptr::null())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpPushClose<P0>(hwait: P0)
where
    P0: ::windows::core::IntoParam<HTTP_PUSH_WAIT_HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpPushClose ( hwait : HTTP_PUSH_WAIT_HANDLE ) -> ( ) );
    HttpPushClose(hwait.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpPushEnable(hrequest: *const ::core::ffi::c_void, ptransportsetting: *const HTTP_PUSH_TRANSPORT_SETTING, phwait: *mut HTTP_PUSH_WAIT_HANDLE) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpPushEnable ( hrequest : *const ::core::ffi::c_void , ptransportsetting : *const HTTP_PUSH_TRANSPORT_SETTING , phwait : *mut HTTP_PUSH_WAIT_HANDLE ) -> u32 );
    HttpPushEnable(hrequest, ptransportsetting, phwait)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpPushWait<P0>(hwait: P0, etype: HTTP_PUSH_WAIT_TYPE, pnotificationstatus: ::core::option::Option<*mut HTTP_PUSH_NOTIFICATION_STATUS>) -> u32
where
    P0: ::windows::core::IntoParam<HTTP_PUSH_WAIT_HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpPushWait ( hwait : HTTP_PUSH_WAIT_HANDLE , etype : HTTP_PUSH_WAIT_TYPE , pnotificationstatus : *mut HTTP_PUSH_NOTIFICATION_STATUS ) -> u32 );
    HttpPushWait(hwait.into_param().abi(), etype, ::core::mem::transmute(pnotificationstatus.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpQueryInfoA(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpQueryInfoA ( hrequest : *const ::core::ffi::c_void , dwinfolevel : u32 , lpbuffer : *mut ::core::ffi::c_void , lpdwbufferlength : *mut u32 , lpdwindex : *mut u32 ) -> super::super::Foundation:: BOOL );
    HttpQueryInfoA(hrequest, dwinfolevel, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), lpdwbufferlength, ::core::mem::transmute(lpdwindex.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpQueryInfoW(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpQueryInfoW ( hrequest : *const ::core::ffi::c_void , dwinfolevel : u32 , lpbuffer : *mut ::core::ffi::c_void , lpdwbufferlength : *mut u32 , lpdwindex : *mut u32 ) -> super::super::Foundation:: BOOL );
    HttpQueryInfoW(hrequest, dwinfolevel, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), lpdwbufferlength, ::core::mem::transmute(lpdwindex.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestA(hrequest: *const ::core::ffi::c_void, lpszheaders: ::core::option::Option<&[u8]>, lpoptional: ::core::option::Option<*const ::core::ffi::c_void>, dwoptionallength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpSendRequestA ( hrequest : *const ::core::ffi::c_void , lpszheaders : ::windows::core::PCSTR , dwheaderslength : u32 , lpoptional : *const ::core::ffi::c_void , dwoptionallength : u32 ) -> super::super::Foundation:: BOOL );
    HttpSendRequestA(hrequest, ::core::mem::transmute(lpszheaders.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpoptional.unwrap_or(::std::ptr::null())), dwoptionallength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestExA(hrequest: *const ::core::ffi::c_void, lpbuffersin: ::core::option::Option<*const INTERNET_BUFFERSA>, lpbuffersout: ::core::option::Option<*mut INTERNET_BUFFERSA>, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpSendRequestExA ( hrequest : *const ::core::ffi::c_void , lpbuffersin : *const INTERNET_BUFFERSA , lpbuffersout : *mut INTERNET_BUFFERSA , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    HttpSendRequestExA(hrequest, ::core::mem::transmute(lpbuffersin.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpbuffersout.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestExW(hrequest: *const ::core::ffi::c_void, lpbuffersin: ::core::option::Option<*const INTERNET_BUFFERSW>, lpbuffersout: ::core::option::Option<*mut INTERNET_BUFFERSW>, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpSendRequestExW ( hrequest : *const ::core::ffi::c_void , lpbuffersin : *const INTERNET_BUFFERSW , lpbuffersout : *mut INTERNET_BUFFERSW , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    HttpSendRequestExW(hrequest, ::core::mem::transmute(lpbuffersin.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpbuffersout.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestW(hrequest: *const ::core::ffi::c_void, lpszheaders: ::core::option::Option<&[u16]>, lpoptional: ::core::option::Option<*const ::core::ffi::c_void>, dwoptionallength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpSendRequestW ( hrequest : *const ::core::ffi::c_void , lpszheaders : ::windows::core::PCWSTR , dwheaderslength : u32 , lpoptional : *const ::core::ffi::c_void , dwoptionallength : u32 ) -> super::super::Foundation:: BOOL );
    HttpSendRequestW(hrequest, ::core::mem::transmute(lpszheaders.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpoptional.unwrap_or(::std::ptr::null())), dwoptionallength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketClose(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: ::core::option::Option<*const ::core::ffi::c_void>, dwreasonlength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpWebSocketClose ( hwebsocket : *const ::core::ffi::c_void , usstatus : u16 , pvreason : *const ::core::ffi::c_void , dwreasonlength : u32 ) -> super::super::Foundation:: BOOL );
    HttpWebSocketClose(hwebsocket, usstatus, ::core::mem::transmute(pvreason.unwrap_or(::std::ptr::null())), dwreasonlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn HttpWebSocketCompleteUpgrade(hrequest: *const ::core::ffi::c_void, dwcontext: usize) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpWebSocketCompleteUpgrade ( hrequest : *const ::core::ffi::c_void , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    HttpWebSocketCompleteUpgrade(hrequest, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketQueryCloseStatus(hwebsocket: *const ::core::ffi::c_void, pusstatus: *mut u16, pvreason: ::core::option::Option<*mut ::core::ffi::c_void>, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpWebSocketQueryCloseStatus ( hwebsocket : *const ::core::ffi::c_void , pusstatus : *mut u16 , pvreason : *mut ::core::ffi::c_void , dwreasonlength : u32 , pdwreasonlengthconsumed : *mut u32 ) -> super::super::Foundation:: BOOL );
    HttpWebSocketQueryCloseStatus(hwebsocket, pusstatus, ::core::mem::transmute(pvreason.unwrap_or(::std::ptr::null_mut())), dwreasonlength, pdwreasonlengthconsumed)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketReceive(hwebsocket: *const ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pbuffertype: *mut HTTP_WEB_SOCKET_BUFFER_TYPE) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpWebSocketReceive ( hwebsocket : *const ::core::ffi::c_void , pvbuffer : *mut ::core::ffi::c_void , dwbufferlength : u32 , pdwbytesread : *mut u32 , pbuffertype : *mut HTTP_WEB_SOCKET_BUFFER_TYPE ) -> super::super::Foundation:: BOOL );
    HttpWebSocketReceive(hwebsocket, pvbuffer, dwbufferlength, pdwbytesread, pbuffertype)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketSend(hwebsocket: *const ::core::ffi::c_void, buffertype: HTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwbufferlength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpWebSocketSend ( hwebsocket : *const ::core::ffi::c_void , buffertype : HTTP_WEB_SOCKET_BUFFER_TYPE , pvbuffer : *const ::core::ffi::c_void , dwbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    HttpWebSocketSend(hwebsocket, buffertype, ::core::mem::transmute(pvbuffer.unwrap_or(::std::ptr::null())), dwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketShutdown(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: ::core::option::Option<*const ::core::ffi::c_void>, dwreasonlength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn HttpWebSocketShutdown ( hwebsocket : *const ::core::ffi::c_void , usstatus : u16 , pvreason : *const ::core::ffi::c_void , dwreasonlength : u32 ) -> super::super::Foundation:: BOOL );
    HttpWebSocketShutdown(hwebsocket, usstatus, ::core::mem::transmute(pvreason.unwrap_or(::std::ptr::null())), dwreasonlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImportCookieFileA<P0>(szfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ImportCookieFileA ( szfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    ImportCookieFileA(szfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImportCookieFileW<P0>(szfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ImportCookieFileW ( szfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    ImportCookieFileW(szfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IncrementUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn IncrementUrlCacheHeaderData ( nidx : u32 , lpdwdata : *mut u32 ) -> super::super::Foundation:: BOOL );
    IncrementUrlCacheHeaderData(nidx, lpdwdata)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternalInternetGetCookie<P0>(lpszurl: P0, lpszcookiedata: ::windows::core::PSTR, lpdwdatasize: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternalInternetGetCookie ( lpszurl : ::windows::core::PCSTR , lpszcookiedata : ::windows::core::PSTR , lpdwdatasize : *mut u32 ) -> u32 );
    InternalInternetGetCookie(lpszurl.into_param().abi(), ::core::mem::transmute(lpszcookiedata), lpdwdatasize)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAlgIdToStringA(ai: u32, lpstr: ::windows::core::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetAlgIdToStringA ( ai : u32 , lpstr : ::windows::core::PSTR , lpdwstrlength : *mut u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetAlgIdToStringA(ai, ::core::mem::transmute(lpstr), lpdwstrlength, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAlgIdToStringW(ai: u32, lpstr: ::windows::core::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetAlgIdToStringW ( ai : u32 , lpstr : ::windows::core::PWSTR , lpdwstrlength : *mut u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetAlgIdToStringW(ai, ::core::mem::transmute(lpstr), lpdwstrlength, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetAttemptConnect(dwreserved: u32) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetAttemptConnect ( dwreserved : u32 ) -> u32 );
    InternetAttemptConnect(dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAutodial<P0>(dwflags: INTERNET_AUTODIAL, hwndparent: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetAutodial ( dwflags : INTERNET_AUTODIAL , hwndparent : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    InternetAutodial(dwflags, hwndparent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAutodialHangup(dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetAutodialHangup ( dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetAutodialHangup(dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCanonicalizeUrlA<P0>(lpszurl: P0, lpszbuffer: ::windows::core::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCanonicalizeUrlA ( lpszurl : ::windows::core::PCSTR , lpszbuffer : ::windows::core::PSTR , lpdwbufferlength : *mut u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetCanonicalizeUrlA(lpszurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), lpdwbufferlength, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCanonicalizeUrlW<P0>(lpszurl: P0, lpszbuffer: ::windows::core::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCanonicalizeUrlW ( lpszurl : ::windows::core::PCWSTR , lpszbuffer : ::windows::core::PWSTR , lpdwbufferlength : *mut u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetCanonicalizeUrlW(lpszurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), lpdwbufferlength, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCheckConnectionA<P0>(lpszurl: P0, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCheckConnectionA ( lpszurl : ::windows::core::PCSTR , dwflags : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetCheckConnectionA(lpszurl.into_param().abi(), dwflags, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCheckConnectionW<P0>(lpszurl: P0, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCheckConnectionW ( lpszurl : ::windows::core::PCWSTR , dwflags : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetCheckConnectionW(lpszurl.into_param().abi(), dwflags, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetClearAllPerSiteCookieDecisions() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetClearAllPerSiteCookieDecisions ( ) -> super::super::Foundation:: BOOL );
    InternetClearAllPerSiteCookieDecisions()
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCloseHandle(hinternet: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCloseHandle ( hinternet : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InternetCloseHandle(hinternet)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCombineUrlA<P0, P1>(lpszbaseurl: P0, lpszrelativeurl: P1, lpszbuffer: ::windows::core::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCombineUrlA ( lpszbaseurl : ::windows::core::PCSTR , lpszrelativeurl : ::windows::core::PCSTR , lpszbuffer : ::windows::core::PSTR , lpdwbufferlength : *mut u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetCombineUrlA(lpszbaseurl.into_param().abi(), lpszrelativeurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), lpdwbufferlength, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCombineUrlW<P0, P1>(lpszbaseurl: P0, lpszrelativeurl: P1, lpszbuffer: ::windows::core::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCombineUrlW ( lpszbaseurl : ::windows::core::PCWSTR , lpszrelativeurl : ::windows::core::PCWSTR , lpszbuffer : ::windows::core::PWSTR , lpdwbufferlength : *mut u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetCombineUrlW(lpszbaseurl.into_param().abi(), lpszrelativeurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), lpdwbufferlength, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossing<P0, P1, P2, P3>(hwnd: P0, szurlprev: P1, szurlnew: P2, bpost: P3) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetConfirmZoneCrossing ( hwnd : super::super::Foundation:: HWND , szurlprev : ::windows::core::PCSTR , szurlnew : ::windows::core::PCSTR , bpost : super::super::Foundation:: BOOL ) -> u32 );
    InternetConfirmZoneCrossing(hwnd.into_param().abi(), szurlprev.into_param().abi(), szurlnew.into_param().abi(), bpost.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossingA<P0, P1, P2, P3>(hwnd: P0, szurlprev: P1, szurlnew: P2, bpost: P3) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetConfirmZoneCrossingA ( hwnd : super::super::Foundation:: HWND , szurlprev : ::windows::core::PCSTR , szurlnew : ::windows::core::PCSTR , bpost : super::super::Foundation:: BOOL ) -> u32 );
    InternetConfirmZoneCrossingA(hwnd.into_param().abi(), szurlprev.into_param().abi(), szurlnew.into_param().abi(), bpost.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossingW<P0, P1, P2, P3>(hwnd: P0, szurlprev: P1, szurlnew: P2, bpost: P3) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetConfirmZoneCrossingW ( hwnd : super::super::Foundation:: HWND , szurlprev : ::windows::core::PCWSTR , szurlnew : ::windows::core::PCWSTR , bpost : super::super::Foundation:: BOOL ) -> u32 );
    InternetConfirmZoneCrossingW(hwnd.into_param().abi(), szurlprev.into_param().abi(), szurlnew.into_param().abi(), bpost.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetConnectA<P0, P1, P2>(hinternet: *const ::core::ffi::c_void, lpszservername: P0, nserverport: u16, lpszusername: P1, lpszpassword: P2, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetConnectA ( hinternet : *const ::core::ffi::c_void , lpszservername : ::windows::core::PCSTR , nserverport : u16 , lpszusername : ::windows::core::PCSTR , lpszpassword : ::windows::core::PCSTR , dwservice : u32 , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    InternetConnectA(hinternet, lpszservername.into_param().abi(), nserverport, lpszusername.into_param().abi(), lpszpassword.into_param().abi(), dwservice, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetConnectW<P0, P1, P2>(hinternet: *const ::core::ffi::c_void, lpszservername: P0, nserverport: u16, lpszusername: P1, lpszpassword: P2, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetConnectW ( hinternet : *const ::core::ffi::c_void , lpszservername : ::windows::core::PCWSTR , nserverport : u16 , lpszusername : ::windows::core::PCWSTR , lpszpassword : ::windows::core::PCWSTR , dwservice : u32 , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    InternetConnectW(hinternet, lpszservername.into_param().abi(), nserverport, lpszusername.into_param().abi(), lpszpassword.into_param().abi(), dwservice, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConvertUrlFromWireToWideChar<P0, P1>(pcszurl: &[u8], pcwszbaseurl: P0, dwcodepagehost: u32, dwcodepagepath: u32, fencodepathextra: P1, dwcodepageextra: u32, ppwszconvertedurl: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetConvertUrlFromWireToWideChar ( pcszurl : ::windows::core::PCSTR , cchurl : u32 , pcwszbaseurl : ::windows::core::PCWSTR , dwcodepagehost : u32 , dwcodepagepath : u32 , fencodepathextra : super::super::Foundation:: BOOL , dwcodepageextra : u32 , ppwszconvertedurl : *mut ::windows::core::PWSTR ) -> u32 );
    InternetConvertUrlFromWireToWideChar(::core::mem::transmute(pcszurl.as_ptr()), pcszurl.len() as _, pcwszbaseurl.into_param().abi(), dwcodepagehost, dwcodepagepath, fencodepathextra.into_param().abi(), dwcodepageextra, ppwszconvertedurl)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinHttp\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
#[inline]
pub unsafe fn InternetCrackUrlA(lpszurl: &[u8], dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSA) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCrackUrlA ( lpszurl : ::windows::core::PCSTR , dwurllength : u32 , dwflags : super::WinHttp:: WIN_HTTP_CREATE_URL_FLAGS , lpurlcomponents : *mut URL_COMPONENTSA ) -> super::super::Foundation:: BOOL );
    InternetCrackUrlA(::core::mem::transmute(lpszurl.as_ptr()), lpszurl.len() as _, dwflags, lpurlcomponents)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinHttp\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
#[inline]
pub unsafe fn InternetCrackUrlW(lpszurl: &[u16], dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSW) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCrackUrlW ( lpszurl : ::windows::core::PCWSTR , dwurllength : u32 , dwflags : super::WinHttp:: WIN_HTTP_CREATE_URL_FLAGS , lpurlcomponents : *mut URL_COMPONENTSW ) -> super::super::Foundation:: BOOL );
    InternetCrackUrlW(::core::mem::transmute(lpszurl.as_ptr()), lpszurl.len() as _, dwflags, lpurlcomponents)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCreateUrlA(lpurlcomponents: *const URL_COMPONENTSA, dwflags: u32, lpszurl: ::windows::core::PSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCreateUrlA ( lpurlcomponents : *const URL_COMPONENTSA , dwflags : u32 , lpszurl : ::windows::core::PSTR , lpdwurllength : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetCreateUrlA(lpurlcomponents, dwflags, ::core::mem::transmute(lpszurl), lpdwurllength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCreateUrlW(lpurlcomponents: *const URL_COMPONENTSW, dwflags: u32, lpszurl: ::windows::core::PWSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetCreateUrlW ( lpurlcomponents : *const URL_COMPONENTSW , dwflags : u32 , lpszurl : ::windows::core::PWSTR , lpdwurllength : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetCreateUrlW(lpurlcomponents, dwflags, ::core::mem::transmute(lpszurl), lpdwurllength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetDial<P0, P1>(hwndparent: P0, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut u32, dwreserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetDial ( hwndparent : super::super::Foundation:: HWND , lpszconnectoid : ::windows::core::PCSTR , dwflags : u32 , lpdwconnection : *mut u32 , dwreserved : u32 ) -> u32 );
    InternetDial(hwndparent.into_param().abi(), lpszconnectoid.into_param().abi(), dwflags, lpdwconnection, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetDialA<P0, P1>(hwndparent: P0, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetDialA ( hwndparent : super::super::Foundation:: HWND , lpszconnectoid : ::windows::core::PCSTR , dwflags : u32 , lpdwconnection : *mut usize , dwreserved : u32 ) -> u32 );
    InternetDialA(hwndparent.into_param().abi(), lpszconnectoid.into_param().abi(), dwflags, lpdwconnection, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetDialW<P0, P1>(hwndparent: P0, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetDialW ( hwndparent : super::super::Foundation:: HWND , lpszconnectoid : ::windows::core::PCWSTR , dwflags : u32 , lpdwconnection : *mut usize , dwreserved : u32 ) -> u32 );
    InternetDialW(hwndparent.into_param().abi(), lpszconnectoid.into_param().abi(), dwflags, lpdwconnection, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionA(pszsitename: ::windows::core::PSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetEnumPerSiteCookieDecisionA ( pszsitename : ::windows::core::PSTR , pcsitenamesize : *mut u32 , pdwdecision : *mut u32 , dwindex : u32 ) -> super::super::Foundation:: BOOL );
    InternetEnumPerSiteCookieDecisionA(::core::mem::transmute(pszsitename), pcsitenamesize, pdwdecision, dwindex)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionW(pszsitename: ::windows::core::PWSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetEnumPerSiteCookieDecisionW ( pszsitename : ::windows::core::PWSTR , pcsitenamesize : *mut u32 , pdwdecision : *mut u32 , dwindex : u32 ) -> super::super::Foundation:: BOOL );
    InternetEnumPerSiteCookieDecisionW(::core::mem::transmute(pszsitename), pcsitenamesize, pdwdecision, dwindex)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetErrorDlg<P0>(hwnd: P0, hrequest: ::core::option::Option<*mut ::core::ffi::c_void>, dwerror: u32, dwflags: u32, lppvdata: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetErrorDlg ( hwnd : super::super::Foundation:: HWND , hrequest : *mut ::core::ffi::c_void , dwerror : u32 , dwflags : u32 , lppvdata : *mut *mut ::core::ffi::c_void ) -> u32 );
    InternetErrorDlg(hwnd.into_param().abi(), ::core::mem::transmute(hrequest.unwrap_or(::std::ptr::null_mut())), dwerror, dwflags, ::core::mem::transmute(lppvdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFindNextFileA(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetFindNextFileA ( hfind : *const ::core::ffi::c_void , lpvfinddata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InternetFindNextFileA(hfind, lpvfinddata)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFindNextFileW(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetFindNextFileW ( hfind : *const ::core::ffi::c_void , lpvfinddata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InternetFindNextFileW(hfind, lpvfinddata)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFortezzaCommand<P0>(dwcommand: u32, hwnd: P0, dwreserved: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetFortezzaCommand ( dwcommand : u32 , hwnd : super::super::Foundation:: HWND , dwreserved : usize ) -> super::super::Foundation:: BOOL );
    InternetFortezzaCommand(dwcommand, hwnd.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFreeCookies(pcookies: ::core::option::Option<*mut INTERNET_COOKIE2>, dwcookiecount: u32) {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetFreeCookies ( pcookies : *mut INTERNET_COOKIE2 , dwcookiecount : u32 ) -> ( ) );
    InternetFreeCookies(::core::mem::transmute(pcookies.unwrap_or(::std::ptr::null_mut())), dwcookiecount)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFreeProxyInfoList(pproxyinfolist: *mut WININET_PROXY_INFO_LIST) {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetFreeProxyInfoList ( pproxyinfolist : *mut WININET_PROXY_INFO_LIST ) -> ( ) );
    InternetFreeProxyInfoList(pproxyinfolist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedState(lpdwflags: *mut INTERNET_CONNECTION, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetConnectedState ( lpdwflags : *mut INTERNET_CONNECTION , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetGetConnectedState(lpdwflags, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedStateEx(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: ::core::option::Option<&mut [u8]>, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetConnectedStateEx ( lpdwflags : *mut INTERNET_CONNECTION , lpszconnectionname : ::windows::core::PSTR , dwnamelen : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetGetConnectedStateEx(lpdwflags, ::core::mem::transmute(lpszconnectionname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len() as _), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedStateExA(lpdwflags: ::core::option::Option<*mut INTERNET_CONNECTION>, lpszconnectionname: ::core::option::Option<&mut [u8]>, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetConnectedStateExA ( lpdwflags : *mut INTERNET_CONNECTION , lpszconnectionname : ::windows::core::PSTR , cchnamelen : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetGetConnectedStateExA(::core::mem::transmute(lpdwflags.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpszconnectionname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len() as _), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedStateExW(lpdwflags: ::core::option::Option<*mut INTERNET_CONNECTION>, lpszconnectionname: ::core::option::Option<&mut [u16]>, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetConnectedStateExW ( lpdwflags : *mut INTERNET_CONNECTION , lpszconnectionname : ::windows::core::PWSTR , cchnamelen : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetGetConnectedStateExW(::core::mem::transmute(lpdwflags.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpszconnectionname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len() as _), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieA<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: ::windows::core::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetCookieA ( lpszurl : ::windows::core::PCSTR , lpszcookiename : ::windows::core::PCSTR , lpszcookiedata : ::windows::core::PSTR , lpdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetCookieA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), ::core::mem::transmute(lpszcookiedata), lpdwsize)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieEx2<P0, P1>(pcwszurl: P0, pcwszcookiename: P1, dwflags: u32, ppcookies: *mut *mut INTERNET_COOKIE2, pdwcookiecount: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetCookieEx2 ( pcwszurl : ::windows::core::PCWSTR , pcwszcookiename : ::windows::core::PCWSTR , dwflags : u32 , ppcookies : *mut *mut INTERNET_COOKIE2 , pdwcookiecount : *mut u32 ) -> u32 );
    InternetGetCookieEx2(pcwszurl.into_param().abi(), pcwszcookiename.into_param().abi(), dwflags, ppcookies, pdwcookiecount)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieExA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetCookieExA ( lpszurl : ::windows::core::PCSTR , lpszcookiename : ::windows::core::PCSTR , lpszcookiedata : ::windows::core::PCSTR , lpdwsize : *mut u32 , dwflags : INTERNET_COOKIE_FLAGS , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InternetGetCookieExA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), lpdwsize, dwflags, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieExW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetCookieExW ( lpszurl : ::windows::core::PCWSTR , lpszcookiename : ::windows::core::PCWSTR , lpszcookiedata : ::windows::core::PCWSTR , lpdwsize : *mut u32 , dwflags : INTERNET_COOKIE_FLAGS , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InternetGetCookieExW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), lpdwsize, dwflags, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieW<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: ::windows::core::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetCookieW ( lpszurl : ::windows::core::PCWSTR , lpszcookiename : ::windows::core::PCWSTR , lpszcookiedata : ::windows::core::PWSTR , lpdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetCookieW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), ::core::mem::transmute(lpszcookiedata), lpdwsize)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetLastResponseInfoA(lpdwerror: *mut u32, lpszbuffer: ::windows::core::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetLastResponseInfoA ( lpdwerror : *mut u32 , lpszbuffer : ::windows::core::PSTR , lpdwbufferlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetLastResponseInfoA(lpdwerror, ::core::mem::transmute(lpszbuffer), lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetLastResponseInfoW(lpdwerror: *mut u32, lpszbuffer: ::windows::core::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetLastResponseInfoW ( lpdwerror : *mut u32 , lpszbuffer : ::windows::core::PWSTR , lpdwbufferlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetLastResponseInfoW(lpdwerror, ::core::mem::transmute(lpszbuffer), lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionA<P0>(pchhostname: P0, presult: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetPerSiteCookieDecisionA ( pchhostname : ::windows::core::PCSTR , presult : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetPerSiteCookieDecisionA(pchhostname.into_param().abi(), presult)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionW<P0>(pchhostname: P0, presult: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetPerSiteCookieDecisionW ( pchhostname : ::windows::core::PCWSTR , presult : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetPerSiteCookieDecisionW(pchhostname.into_param().abi(), presult)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetProxyForUrl<P0>(hinternet: *const ::core::ffi::c_void, pcwszurl: P0, pproxyinfolist: *mut WININET_PROXY_INFO_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetProxyForUrl ( hinternet : *const ::core::ffi::c_void , pcwszurl : ::windows::core::PCWSTR , pproxyinfolist : *mut WININET_PROXY_INFO_LIST ) -> u32 );
    InternetGetProxyForUrl(hinternet, pcwszurl.into_param().abi(), pproxyinfolist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURL<P0>(lpszurl: P0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetSecurityInfoByURL ( lpszurl : ::windows::core::PCSTR , ppcertchain : *mut *mut super::super::Security::Cryptography:: CERT_CHAIN_CONTEXT , pdwsecureflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetSecurityInfoByURL(lpszurl.into_param().abi(), ppcertchain, pdwsecureflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURLA<P0>(lpszurl: P0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetSecurityInfoByURLA ( lpszurl : ::windows::core::PCSTR , ppcertchain : *mut *mut super::super::Security::Cryptography:: CERT_CHAIN_CONTEXT , pdwsecureflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetSecurityInfoByURLA(lpszurl.into_param().abi(), ppcertchain, pdwsecureflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURLW<P0>(lpszurl: P0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGetSecurityInfoByURLW ( lpszurl : ::windows::core::PCWSTR , ppcertchain : *mut *mut super::super::Security::Cryptography:: CERT_CHAIN_CONTEXT , pdwsecureflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetGetSecurityInfoByURLW(lpszurl.into_param().abi(), ppcertchain, pdwsecureflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGoOnline<P0, P1>(lpszurl: P0, hwndparent: P1, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGoOnline ( lpszurl : ::windows::core::PCSTR , hwndparent : super::super::Foundation:: HWND , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetGoOnline(lpszurl.into_param().abi(), hwndparent.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGoOnlineA<P0, P1>(lpszurl: P0, hwndparent: P1, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGoOnlineA ( lpszurl : ::windows::core::PCSTR , hwndparent : super::super::Foundation:: HWND , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetGoOnlineA(lpszurl.into_param().abi(), hwndparent.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGoOnlineW<P0, P1>(lpszurl: P0, hwndparent: P1, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetGoOnlineW ( lpszurl : ::windows::core::PCWSTR , hwndparent : super::super::Foundation:: HWND , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetGoOnlineW(lpszurl.into_param().abi(), hwndparent.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetHangUp(dwconnection: usize, dwreserved: u32) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetHangUp ( dwconnection : usize , dwreserved : u32 ) -> u32 );
    InternetHangUp(dwconnection, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetInitializeAutoProxyDll(dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetInitializeAutoProxyDll ( dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetInitializeAutoProxyDll(dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetLockRequestFile(hinternet: *const ::core::ffi::c_void, lphlockrequestinfo: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetLockRequestFile ( hinternet : *const ::core::ffi::c_void , lphlockrequestinfo : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    InternetLockRequestFile(hinternet, lphlockrequestinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetOpenA<P0, P1, P2>(lpszagent: P0, dwaccesstype: u32, lpszproxy: P1, lpszproxybypass: P2, dwflags: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetOpenA ( lpszagent : ::windows::core::PCSTR , dwaccesstype : u32 , lpszproxy : ::windows::core::PCSTR , lpszproxybypass : ::windows::core::PCSTR , dwflags : u32 ) -> *mut ::core::ffi::c_void );
    InternetOpenA(lpszagent.into_param().abi(), dwaccesstype, lpszproxy.into_param().abi(), lpszproxybypass.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetOpenUrlA<P0>(hinternet: *const ::core::ffi::c_void, lpszurl: P0, lpszheaders: ::core::option::Option<&[u8]>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetOpenUrlA ( hinternet : *const ::core::ffi::c_void , lpszurl : ::windows::core::PCSTR , lpszheaders : ::windows::core::PCSTR , dwheaderslength : u32 , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    InternetOpenUrlA(hinternet, lpszurl.into_param().abi(), ::core::mem::transmute(lpszheaders.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len() as _), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetOpenUrlW<P0>(hinternet: *const ::core::ffi::c_void, lpszurl: P0, lpszheaders: ::core::option::Option<&[u16]>, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetOpenUrlW ( hinternet : *const ::core::ffi::c_void , lpszurl : ::windows::core::PCWSTR , lpszheaders : ::windows::core::PCWSTR , dwheaderslength : u32 , dwflags : u32 , dwcontext : usize ) -> *mut ::core::ffi::c_void );
    InternetOpenUrlW(hinternet, lpszurl.into_param().abi(), ::core::mem::transmute(lpszheaders.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len() as _), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetOpenW<P0, P1, P2>(lpszagent: P0, dwaccesstype: u32, lpszproxy: P1, lpszproxybypass: P2, dwflags: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetOpenW ( lpszagent : ::windows::core::PCWSTR , dwaccesstype : u32 , lpszproxy : ::windows::core::PCWSTR , lpszproxybypass : ::windows::core::PCWSTR , dwflags : u32 ) -> *mut ::core::ffi::c_void );
    InternetOpenW(lpszagent.into_param().abi(), dwaccesstype, lpszproxy.into_param().abi(), lpszproxybypass.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryDataAvailable(hfile: *const ::core::ffi::c_void, lpdwnumberofbytesavailable: ::core::option::Option<*mut u32>, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetQueryDataAvailable ( hfile : *const ::core::ffi::c_void , lpdwnumberofbytesavailable : *mut u32 , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    InternetQueryDataAvailable(hfile, ::core::mem::transmute(lpdwnumberofbytesavailable.unwrap_or(::std::ptr::null_mut())), dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryFortezzaStatus(pdwstatus: *mut u32, dwreserved: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetQueryFortezzaStatus ( pdwstatus : *mut u32 , dwreserved : usize ) -> super::super::Foundation:: BOOL );
    InternetQueryFortezzaStatus(pdwstatus, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryOptionA(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetQueryOptionA ( hinternet : *const ::core::ffi::c_void , dwoption : u32 , lpbuffer : *mut ::core::ffi::c_void , lpdwbufferlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetQueryOptionA(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryOptionW(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetQueryOptionW ( hinternet : *const ::core::ffi::c_void , dwoption : u32 , lpbuffer : *mut ::core::ffi::c_void , lpdwbufferlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetQueryOptionW(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetReadFile(hfile: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetReadFile ( hfile : *const ::core::ffi::c_void , lpbuffer : *mut ::core::ffi::c_void , dwnumberofbytestoread : u32 , lpdwnumberofbytesread : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetReadFile(hfile, lpbuffer, dwnumberofbytestoread, lpdwnumberofbytesread)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetReadFileExA(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetReadFileExA ( hfile : *const ::core::ffi::c_void , lpbuffersout : *mut INTERNET_BUFFERSA , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    InternetReadFileExA(hfile, lpbuffersout, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetReadFileExW(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetReadFileExW ( hfile : *const ::core::ffi::c_void , lpbuffersout : *mut INTERNET_BUFFERSW , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    InternetReadFileExW(hfile, lpbuffersout, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSecurityProtocolToStringA(dwprotocol: u32, lpstr: ::windows::core::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSecurityProtocolToStringA ( dwprotocol : u32 , lpstr : ::windows::core::PSTR , lpdwstrlength : *mut u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetSecurityProtocolToStringA(dwprotocol, ::core::mem::transmute(lpstr), lpdwstrlength, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSecurityProtocolToStringW(dwprotocol: u32, lpstr: ::windows::core::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSecurityProtocolToStringW ( dwprotocol : u32 , lpstr : ::windows::core::PWSTR , lpdwstrlength : *mut u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetSecurityProtocolToStringW(dwprotocol, ::core::mem::transmute(lpstr), lpdwstrlength, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetCookieA ( lpszurl : ::windows::core::PCSTR , lpszcookiename : ::windows::core::PCSTR , lpszcookiedata : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    InternetSetCookieA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieEx2<P0, P1>(pcwszurl: P0, pcookie: *const INTERNET_COOKIE2, pcwszp3ppolicy: P1, dwflags: u32, pdwcookiestate: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetCookieEx2 ( pcwszurl : ::windows::core::PCWSTR , pcookie : *const INTERNET_COOKIE2 , pcwszp3ppolicy : ::windows::core::PCWSTR , dwflags : u32 , pdwcookiestate : *mut u32 ) -> u32 );
    InternetSetCookieEx2(pcwszurl.into_param().abi(), pcookie, pcwszp3ppolicy.into_param().abi(), dwflags, pdwcookiestate)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetSetCookieExA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32, dwreserved: usize) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetCookieExA ( lpszurl : ::windows::core::PCSTR , lpszcookiename : ::windows::core::PCSTR , lpszcookiedata : ::windows::core::PCSTR , dwflags : u32 , dwreserved : usize ) -> u32 );
    InternetSetCookieExA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), dwflags, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetSetCookieExW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32, dwreserved: usize) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetCookieExW ( lpszurl : ::windows::core::PCWSTR , lpszcookiename : ::windows::core::PCWSTR , lpszcookiedata : ::windows::core::PCWSTR , dwflags : u32 , dwreserved : usize ) -> u32 );
    InternetSetCookieExW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), dwflags, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetCookieW ( lpszurl : ::windows::core::PCWSTR , lpszcookiename : ::windows::core::PCWSTR , lpszcookiedata : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    InternetSetCookieW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetDialState<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetDialState ( lpszconnectoid : ::windows::core::PCSTR , dwstate : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetDialState(lpszconnectoid.into_param().abi(), dwstate, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetDialStateA<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetDialStateA ( lpszconnectoid : ::windows::core::PCSTR , dwstate : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetDialStateA(lpszconnectoid.into_param().abi(), dwstate, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetDialStateW<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetDialStateW ( lpszconnectoid : ::windows::core::PCWSTR , dwstate : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetDialStateW(lpszconnectoid.into_param().abi(), dwstate, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetSetFilePointer(hfile: *const ::core::ffi::c_void, ldistancetomove: i32, lpdistancetomovehigh: ::core::option::Option<*mut i32>, dwmovemethod: u32, dwcontext: usize) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetFilePointer ( hfile : *const ::core::ffi::c_void , ldistancetomove : i32 , lpdistancetomovehigh : *mut i32 , dwmovemethod : u32 , dwcontext : usize ) -> u32 );
    InternetSetFilePointer(hfile, ldistancetomove, ::core::mem::transmute(lpdistancetomovehigh.unwrap_or(::std::ptr::null_mut())), dwmovemethod, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionA(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwbufferlength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetOptionA ( hinternet : *const ::core::ffi::c_void , dwoption : u32 , lpbuffer : *const ::core::ffi::c_void , dwbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetOptionA(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null())), dwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionExA(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetOptionExA ( hinternet : *const ::core::ffi::c_void , dwoption : u32 , lpbuffer : *const ::core::ffi::c_void , dwbufferlength : u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetOptionExA(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null())), dwbufferlength, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionExW(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetOptionExW ( hinternet : *const ::core::ffi::c_void , dwoption : u32 , lpbuffer : *const ::core::ffi::c_void , dwbufferlength : u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetOptionExW(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null())), dwbufferlength, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionW(hinternet: ::core::option::Option<*const ::core::ffi::c_void>, dwoption: u32, lpbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwbufferlength: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetOptionW ( hinternet : *const ::core::ffi::c_void , dwoption : u32 , lpbuffer : *const ::core::ffi::c_void , dwbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetOptionW(::core::mem::transmute(hinternet.unwrap_or(::std::ptr::null())), dwoption, ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null())), dwbufferlength)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionA<P0>(pchhostname: P0, dwdecision: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetPerSiteCookieDecisionA ( pchhostname : ::windows::core::PCSTR , dwdecision : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetPerSiteCookieDecisionA(pchhostname.into_param().abi(), dwdecision)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionW<P0>(pchhostname: P0, dwdecision: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetPerSiteCookieDecisionW ( pchhostname : ::windows::core::PCWSTR , dwdecision : u32 ) -> super::super::Foundation:: BOOL );
    InternetSetPerSiteCookieDecisionW(pchhostname.into_param().abi(), dwdecision)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetSetStatusCallback(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetStatusCallback ( hinternet : *const ::core::ffi::c_void , lpfninternetcallback : LPINTERNET_STATUS_CALLBACK ) -> LPINTERNET_STATUS_CALLBACK );
    InternetSetStatusCallback(hinternet, lpfninternetcallback)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetSetStatusCallbackA(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetStatusCallbackA ( hinternet : *const ::core::ffi::c_void , lpfninternetcallback : LPINTERNET_STATUS_CALLBACK ) -> LPINTERNET_STATUS_CALLBACK );
    InternetSetStatusCallbackA(hinternet, lpfninternetcallback)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn InternetSetStatusCallbackW(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetSetStatusCallbackW ( hinternet : *const ::core::ffi::c_void , lpfninternetcallback : LPINTERNET_STATUS_CALLBACK ) -> LPINTERNET_STATUS_CALLBACK );
    InternetSetStatusCallbackW(hinternet, lpfninternetcallback)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetShowSecurityInfoByURL<P0, P1>(lpszurl: P0, hwndparent: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetShowSecurityInfoByURL ( lpszurl : ::windows::core::PCSTR , hwndparent : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    InternetShowSecurityInfoByURL(lpszurl.into_param().abi(), hwndparent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetShowSecurityInfoByURLA<P0, P1>(lpszurl: P0, hwndparent: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetShowSecurityInfoByURLA ( lpszurl : ::windows::core::PCSTR , hwndparent : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    InternetShowSecurityInfoByURLA(lpszurl.into_param().abi(), hwndparent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetShowSecurityInfoByURLW<P0, P1>(lpszurl: P0, hwndparent: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetShowSecurityInfoByURLW ( lpszurl : ::windows::core::PCWSTR , hwndparent : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    InternetShowSecurityInfoByURLW(lpszurl.into_param().abi(), hwndparent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: &mut [u8]) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetTimeFromSystemTime ( pst : *const super::super::Foundation:: SYSTEMTIME , dwrfc : u32 , lpsztime : ::windows::core::PSTR , cbtime : u32 ) -> super::super::Foundation:: BOOL );
    InternetTimeFromSystemTime(pst, dwrfc, ::core::mem::transmute(lpsztime.as_ptr()), lpsztime.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeFromSystemTimeA(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: &mut [u8]) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetTimeFromSystemTimeA ( pst : *const super::super::Foundation:: SYSTEMTIME , dwrfc : u32 , lpsztime : ::windows::core::PSTR , cbtime : u32 ) -> super::super::Foundation:: BOOL );
    InternetTimeFromSystemTimeA(pst, dwrfc, ::core::mem::transmute(lpsztime.as_ptr()), lpsztime.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeFromSystemTimeW(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: ::windows::core::PWSTR, cbtime: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetTimeFromSystemTimeW ( pst : *const super::super::Foundation:: SYSTEMTIME , dwrfc : u32 , lpsztime : ::windows::core::PWSTR , cbtime : u32 ) -> super::super::Foundation:: BOOL );
    InternetTimeFromSystemTimeW(pst, dwrfc, ::core::mem::transmute(lpsztime), cbtime)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeToSystemTime<P0>(lpsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetTimeToSystemTime ( lpsztime : ::windows::core::PCSTR , pst : *mut super::super::Foundation:: SYSTEMTIME , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetTimeToSystemTime(lpsztime.into_param().abi(), pst, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeToSystemTimeA<P0>(lpsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetTimeToSystemTimeA ( lpsztime : ::windows::core::PCSTR , pst : *mut super::super::Foundation:: SYSTEMTIME , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetTimeToSystemTimeA(lpsztime.into_param().abi(), pst, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeToSystemTimeW<P0>(lpsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetTimeToSystemTimeW ( lpsztime : ::windows::core::PCWSTR , pst : *mut super::super::Foundation:: SYSTEMTIME , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    InternetTimeToSystemTimeW(lpsztime.into_param().abi(), pst, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetUnlockRequestFile<P0>(hlockrequestinfo: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetUnlockRequestFile ( hlockrequestinfo : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    InternetUnlockRequestFile(hlockrequestinfo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetWriteFile(hfile: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetWriteFile ( hfile : *const ::core::ffi::c_void , lpbuffer : *const ::core::ffi::c_void , dwnumberofbytestowrite : u32 , lpdwnumberofbyteswritten : *mut u32 ) -> super::super::Foundation:: BOOL );
    InternetWriteFile(hfile, lpbuffer, dwnumberofbytestowrite, lpdwnumberofbyteswritten)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetWriteFileExA(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetWriteFileExA ( hfile : *const ::core::ffi::c_void , lpbuffersin : *const INTERNET_BUFFERSA , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    InternetWriteFileExA(hfile, lpbuffersin, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetWriteFileExW(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn InternetWriteFileExW ( hfile : *const ::core::ffi::c_void , lpbuffersin : *const INTERNET_BUFFERSW , dwflags : u32 , dwcontext : usize ) -> super::super::Foundation:: BOOL );
    InternetWriteFileExW(hfile, lpbuffersin, dwflags, dwcontext)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDomainLegalCookieDomainA<P0, P1>(pchdomain: P0, pchfulldomain: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn IsDomainLegalCookieDomainA ( pchdomain : ::windows::core::PCSTR , pchfulldomain : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    IsDomainLegalCookieDomainA(pchdomain.into_param().abi(), pchfulldomain.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDomainLegalCookieDomainW<P0, P1>(pchdomain: P0, pchfulldomain: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn IsDomainLegalCookieDomainW ( pchdomain : ::windows::core::PCWSTR , pchfulldomain : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    IsDomainLegalCookieDomainW(pchdomain.into_param().abi(), pchfulldomain.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsHostInProxyBypassList(tscheme: INTERNET_SCHEME, lpszhost: &[u8]) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn IsHostInProxyBypassList ( tscheme : INTERNET_SCHEME , lpszhost : ::windows::core::PCSTR , cchhost : u32 ) -> super::super::Foundation:: BOOL );
    IsHostInProxyBypassList(tscheme, ::core::mem::transmute(lpszhost.as_ptr()), lpszhost.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProfilesEnabled() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn IsProfilesEnabled ( ) -> super::super::Foundation:: BOOL );
    IsProfilesEnabled()
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUrlCacheEntryExpiredA<P0>(lpszurlname: P0, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn IsUrlCacheEntryExpiredA ( lpszurlname : ::windows::core::PCSTR , dwflags : u32 , pftlastmodified : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    IsUrlCacheEntryExpiredA(lpszurlname.into_param().abi(), dwflags, pftlastmodified)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUrlCacheEntryExpiredW<P0>(lpszurlname: P0, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn IsUrlCacheEntryExpiredW ( lpszurlname : ::windows::core::PCWSTR , dwflags : u32 , pftlastmodified : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    IsUrlCacheEntryExpiredW(lpszurlname.into_param().abi(), dwflags, pftlastmodified)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadUrlCacheContent() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn LoadUrlCacheContent ( ) -> super::super::Foundation:: BOOL );
    LoadUrlCacheContent()
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn ParseX509EncodedCertificateForListBoxEntry(lpcert: &[u8], lpszlistboxentry: ::windows::core::PSTR, lpdwlistboxentry: *mut u32) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn ParseX509EncodedCertificateForListBoxEntry ( lpcert : *const u8 , cbcert : u32 , lpszlistboxentry : ::windows::core::PSTR , lpdwlistboxentry : *mut u32 ) -> u32 );
    ParseX509EncodedCertificateForListBoxEntry(::core::mem::transmute(lpcert.as_ptr()), lpcert.len() as _, ::core::mem::transmute(lpszlistboxentry), lpdwlistboxentry)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerformOperationOverUrlCacheA<P0>(pszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: i64, preserved1: ::core::option::Option<*const ::core::ffi::c_void>, pdwreserved2: ::core::option::Option<*const u32>, preserved3: ::core::option::Option<*const ::core::ffi::c_void>, op: CACHE_OPERATOR, poperatordata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn PerformOperationOverUrlCacheA ( pszurlsearchpattern : ::windows::core::PCSTR , dwflags : u32 , dwfilter : u32 , groupid : i64 , preserved1 : *const ::core::ffi::c_void , pdwreserved2 : *const u32 , preserved3 : *const ::core::ffi::c_void , op : CACHE_OPERATOR , poperatordata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    PerformOperationOverUrlCacheA(pszurlsearchpattern.into_param().abi(), dwflags, dwfilter, groupid, ::core::mem::transmute(preserved1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdwreserved2.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved3.unwrap_or(::std::ptr::null())), op, poperatordata)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn PrivacyGetZonePreferenceW(dwzone: u32, dwtype: u32, pdwtemplate: ::core::option::Option<*mut u32>, pszbuffer: ::windows::core::PWSTR, pdwbufferlength: ::core::option::Option<*mut u32>) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn PrivacyGetZonePreferenceW ( dwzone : u32 , dwtype : u32 , pdwtemplate : *mut u32 , pszbuffer : ::windows::core::PWSTR , pdwbufferlength : *mut u32 ) -> u32 );
    PrivacyGetZonePreferenceW(dwzone, dwtype, ::core::mem::transmute(pdwtemplate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pszbuffer), ::core::mem::transmute(pdwbufferlength.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn PrivacySetZonePreferenceW<P0>(dwzone: u32, dwtype: u32, dwtemplate: u32, pszpreference: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn PrivacySetZonePreferenceW ( dwzone : u32 , dwtype : u32 , dwtemplate : u32 , pszpreference : ::windows::core::PCWSTR ) -> u32 );
    PrivacySetZonePreferenceW(dwzone, dwtype, dwtemplate, pszpreference.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadGuidsForConnectedNetworks(pcnetworks: ::core::option::Option<*mut u32>, pppwsznetworkguids: ::core::option::Option<*mut *mut ::windows::core::PWSTR>, pppbstrnetworknames: ::core::option::Option<*mut *mut ::windows::core::BSTR>, pppwszgwmacs: ::core::option::Option<*mut *mut ::windows::core::PWSTR>, pcgatewaymacs: ::core::option::Option<*mut u32>, pdwflags: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn ReadGuidsForConnectedNetworks ( pcnetworks : *mut u32 , pppwsznetworkguids : *mut *mut ::windows::core::PWSTR , pppbstrnetworknames : *mut *mut ::windows::core::BSTR , pppwszgwmacs : *mut *mut ::windows::core::PWSTR , pcgatewaymacs : *mut u32 , pdwflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    ReadGuidsForConnectedNetworks(::core::mem::transmute(pcnetworks.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pppwsznetworkguids.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pppbstrnetworknames.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pppwszgwmacs.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcgatewaymacs.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwflags.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadUrlCacheEntryStream<P0>(hurlcachestream: P0, dwlocation: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32, reserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ReadUrlCacheEntryStream ( hurlcachestream : super::super::Foundation:: HANDLE , dwlocation : u32 , lpbuffer : *mut ::core::ffi::c_void , lpdwlen : *mut u32 , reserved : u32 ) -> super::super::Foundation:: BOOL );
    ReadUrlCacheEntryStream(hurlcachestream.into_param().abi(), dwlocation, lpbuffer, lpdwlen, reserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadUrlCacheEntryStreamEx<P0>(hurlcachestream: P0, qwlocation: u64, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ReadUrlCacheEntryStreamEx ( hurlcachestream : super::super::Foundation:: HANDLE , qwlocation : u64 , lpbuffer : *mut ::core::ffi::c_void , lpdwlen : *mut u32 ) -> super::super::Foundation:: BOOL );
    ReadUrlCacheEntryStreamEx(hurlcachestream.into_param().abi(), qwlocation, lpbuffer, lpdwlen)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterUrlCacheNotification<P0>(hwnd: P0, umsg: u32, gid: i64, dwopsfilter: u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn RegisterUrlCacheNotification ( hwnd : super::super::Foundation:: HWND , umsg : u32 , gid : i64 , dwopsfilter : u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    RegisterUrlCacheNotification(hwnd.into_param().abi(), umsg, gid, dwopsfilter, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResumeSuspendedDownload(hrequest: *const ::core::ffi::c_void, dwresultcode: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn ResumeSuspendedDownload ( hrequest : *const ::core::ffi::c_void , dwresultcode : u32 ) -> super::super::Foundation:: BOOL );
    ResumeSuspendedDownload(hrequest, dwresultcode)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileA<P0>(lpszurlname: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn RetrieveUrlCacheEntryFileA ( lpszurlname : ::windows::core::PCSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    RetrieveUrlCacheEntryFileA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileW<P0>(lpszurlname: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn RetrieveUrlCacheEntryFileW ( lpszurlname : ::windows::core::PCWSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    RetrieveUrlCacheEntryFileW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamA<P0, P1>(lpszurlname: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, frandomread: P1, dwreserved: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn RetrieveUrlCacheEntryStreamA ( lpszurlname : ::windows::core::PCSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA , lpcbcacheentryinfo : *mut u32 , frandomread : super::super::Foundation:: BOOL , dwreserved : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = RetrieveUrlCacheEntryStreamA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, frandomread.into_param().abi(), dwreserved);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamW<P0, P1>(lpszurlname: P0, lpcacheentryinfo: ::core::option::Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, frandomread: P1, dwreserved: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn RetrieveUrlCacheEntryStreamW ( lpszurlname : ::windows::core::PCWSTR , lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW , lpcbcacheentryinfo : *mut u32 , frandomread : super::super::Foundation:: BOOL , dwreserved : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = RetrieveUrlCacheEntryStreamW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo.unwrap_or(::std::ptr::null_mut())), lpcbcacheentryinfo, frandomread.into_param().abi(), dwreserved);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunOnceUrlCache<P0, P1, P2>(hwnd: P0, hinst: P1, lpszcmd: P2, ncmdshow: i32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn RunOnceUrlCache ( hwnd : super::super::Foundation:: HWND , hinst : super::super::Foundation:: HMODULE , lpszcmd : ::windows::core::PCSTR , ncmdshow : i32 ) -> u32 );
    RunOnceUrlCache(hwnd.into_param().abi(), hinst.into_param().abi(), lpszcmd.into_param().abi(), ncmdshow)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheConfigInfoA(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheConfigInfoA ( lpcacheconfiginfo : *const INTERNET_CACHE_CONFIG_INFOA , dwfieldcontrol : u32 ) -> super::super::Foundation:: BOOL );
    SetUrlCacheConfigInfoA(lpcacheconfiginfo, dwfieldcontrol)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheConfigInfoW(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheConfigInfoW ( lpcacheconfiginfo : *const INTERNET_CACHE_CONFIG_INFOW , dwfieldcontrol : u32 ) -> super::super::Foundation:: BOOL );
    SetUrlCacheConfigInfoW(lpcacheconfiginfo, dwfieldcontrol)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryGroup<P0>(lpszurlname: P0, dwflags: u32, groupid: i64, pbgroupattributes: ::core::option::Option<*const u8>, cbgroupattributes: u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheEntryGroup ( lpszurlname : ::windows::core::PCSTR , dwflags : u32 , groupid : i64 , pbgroupattributes : *const u8 , cbgroupattributes : u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetUrlCacheEntryGroup(lpszurlname.into_param().abi(), dwflags, groupid, ::core::mem::transmute(pbgroupattributes.unwrap_or(::std::ptr::null())), cbgroupattributes, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryGroupA<P0>(lpszurlname: P0, dwflags: u32, groupid: i64, pbgroupattributes: ::core::option::Option<*const u8>, cbgroupattributes: u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheEntryGroupA ( lpszurlname : ::windows::core::PCSTR , dwflags : u32 , groupid : i64 , pbgroupattributes : *const u8 , cbgroupattributes : u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetUrlCacheEntryGroupA(lpszurlname.into_param().abi(), dwflags, groupid, ::core::mem::transmute(pbgroupattributes.unwrap_or(::std::ptr::null())), cbgroupattributes, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryGroupW<P0>(lpszurlname: P0, dwflags: u32, groupid: i64, pbgroupattributes: ::core::option::Option<*const u8>, cbgroupattributes: u32, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheEntryGroupW ( lpszurlname : ::windows::core::PCWSTR , dwflags : u32 , groupid : i64 , pbgroupattributes : *const u8 , cbgroupattributes : u32 , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetUrlCacheEntryGroupW(lpszurlname.into_param().abi(), dwflags, groupid, ::core::mem::transmute(pbgroupattributes.unwrap_or(::std::ptr::null())), cbgroupattributes, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryInfoA<P0>(lpszurlname: P0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheEntryInfoA ( lpszurlname : ::windows::core::PCSTR , lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOA , dwfieldcontrol : u32 ) -> super::super::Foundation:: BOOL );
    SetUrlCacheEntryInfoA(lpszurlname.into_param().abi(), lpcacheentryinfo, dwfieldcontrol)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryInfoW<P0>(lpszurlname: P0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheEntryInfoW ( lpszurlname : ::windows::core::PCWSTR , lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOW , dwfieldcontrol : u32 ) -> super::super::Foundation:: BOOL );
    SetUrlCacheEntryInfoW(lpszurlname.into_param().abi(), lpcacheentryinfo, dwfieldcontrol)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOA, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheGroupAttributeA ( gid : i64 , dwflags : u32 , dwattributes : u32 , lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOA , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetUrlCacheGroupAttributeA(gid, dwflags, dwattributes, lpgroupinfo, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOW, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheGroupAttributeW ( gid : i64 , dwflags : u32 , dwattributes : u32 , lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOW , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetUrlCacheGroupAttributeW(gid, dwflags, dwattributes, lpgroupinfo, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheHeaderData(nidx: u32, dwdata: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "wininet.dll""system" fn SetUrlCacheHeaderData ( nidx : u32 , dwdata : u32 ) -> super::super::Foundation:: BOOL );
    SetUrlCacheHeaderData(nidx, dwdata)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowClientAuthCerts<P0>(hwndparent: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ShowClientAuthCerts ( hwndparent : super::super::Foundation:: HWND ) -> u32 );
    ShowClientAuthCerts(hwndparent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn ShowSecurityInfo<P0>(hwndparent: P0, psecurityinfo: *const INTERNET_SECURITY_INFO) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ShowSecurityInfo ( hwndparent : super::super::Foundation:: HWND , psecurityinfo : *const INTERNET_SECURITY_INFO ) -> u32 );
    ShowSecurityInfo(hwndparent.into_param().abi(), psecurityinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowX509EncodedCertificate<P0>(hwndparent: P0, lpcert: &[u8]) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn ShowX509EncodedCertificate ( hwndparent : super::super::Foundation:: HWND , lpcert : *const u8 , cbcert : u32 ) -> u32 );
    ShowX509EncodedCertificate(hwndparent.into_param().abi(), ::core::mem::transmute(lpcert.as_ptr()), lpcert.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryFile<P0>(lpszurlname: P0, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UnlockUrlCacheEntryFile ( lpszurlname : ::windows::core::PCSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    UnlockUrlCacheEntryFile(lpszurlname.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileA<P0>(lpszurlname: P0, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UnlockUrlCacheEntryFileA ( lpszurlname : ::windows::core::PCSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    UnlockUrlCacheEntryFileA(lpszurlname.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileW<P0>(lpszurlname: P0, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UnlockUrlCacheEntryFileW ( lpszurlname : ::windows::core::PCWSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    UnlockUrlCacheEntryFileW(lpszurlname.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryStream<P0>(hurlcachestream: P0, reserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UnlockUrlCacheEntryStream ( hurlcachestream : super::super::Foundation:: HANDLE , reserved : u32 ) -> super::super::Foundation:: BOOL );
    UnlockUrlCacheEntryStream(hurlcachestream.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateUrlCacheContentPath<P0>(sznewpath: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UpdateUrlCacheContentPath ( sznewpath : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    UpdateUrlCacheContentPath(sznewpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheCheckEntriesExist(rgpwszurls: *const ::windows::core::PCWSTR, centries: u32, rgfexist: *mut super::super::Foundation::BOOL) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheCheckEntriesExist ( rgpwszurls : *const ::windows::core::PCWSTR , centries : u32 , rgfexist : *mut super::super::Foundation:: BOOL ) -> u32 );
    UrlCacheCheckEntriesExist(rgpwszurls, centries, rgfexist)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheCloseEntryHandle(hentryfile: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheCloseEntryHandle ( hentryfile : *const ::core::ffi::c_void ) -> ( ) );
    UrlCacheCloseEntryHandle(hentryfile)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheContainerSetEntryMaximumAge<P0>(pwszprefix: P0, dwentrymaxage: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheContainerSetEntryMaximumAge ( pwszprefix : ::windows::core::PCWSTR , dwentrymaxage : u32 ) -> u32 );
    UrlCacheContainerSetEntryMaximumAge(pwszprefix.into_param().abi(), dwentrymaxage)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheCreateContainer<P0, P1, P2>(pwszname: P0, pwszprefix: P1, pwszdirectory: P2, ulllimit: u64, dwoptions: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheCreateContainer ( pwszname : ::windows::core::PCWSTR , pwszprefix : ::windows::core::PCWSTR , pwszdirectory : ::windows::core::PCWSTR , ulllimit : u64 , dwoptions : u32 ) -> u32 );
    UrlCacheCreateContainer(pwszname.into_param().abi(), pwszprefix.into_param().abi(), pwszdirectory.into_param().abi(), ulllimit, dwoptions)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheFindFirstEntry<P0>(pwszprefix: P0, dwflags: u32, dwfilter: u32, groupid: i64, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phfind: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheFindFirstEntry ( pwszprefix : ::windows::core::PCWSTR , dwflags : u32 , dwfilter : u32 , groupid : i64 , pcacheentryinfo : *mut URLCACHE_ENTRY_INFO , phfind : *mut super::super::Foundation:: HANDLE ) -> u32 );
    UrlCacheFindFirstEntry(pwszprefix.into_param().abi(), dwflags, dwfilter, groupid, pcacheentryinfo, phfind)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheFindNextEntry<P0>(hfind: P0, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheFindNextEntry ( hfind : super::super::Foundation:: HANDLE , pcacheentryinfo : *mut URLCACHE_ENTRY_INFO ) -> u32 );
    UrlCacheFindNextEntry(hfind.into_param().abi(), pcacheentryinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheFreeEntryInfo(pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheFreeEntryInfo ( pcacheentryinfo : *mut URLCACHE_ENTRY_INFO ) -> ( ) );
    UrlCacheFreeEntryInfo(pcacheentryinfo)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheFreeGlobalSpace(ulltargetsize: u64, dwfilter: u32) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheFreeGlobalSpace ( ulltargetsize : u64 , dwfilter : u32 ) -> u32 );
    UrlCacheFreeGlobalSpace(ulltargetsize, dwfilter)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheGetContentPaths(pppwszdirectories: *mut *mut ::windows::core::PWSTR, pcdirectories: *mut u32) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheGetContentPaths ( pppwszdirectories : *mut *mut ::windows::core::PWSTR , pcdirectories : *mut u32 ) -> u32 );
    UrlCacheGetContentPaths(pppwszdirectories, pcdirectories)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheGetEntryInfo<P0>(happcache: ::core::option::Option<*const ::core::ffi::c_void>, pcwszurl: P0, pcacheentryinfo: ::core::option::Option<*mut URLCACHE_ENTRY_INFO>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheGetEntryInfo ( happcache : *const ::core::ffi::c_void , pcwszurl : ::windows::core::PCWSTR , pcacheentryinfo : *mut URLCACHE_ENTRY_INFO ) -> u32 );
    UrlCacheGetEntryInfo(::core::mem::transmute(happcache.unwrap_or(::std::ptr::null())), pcwszurl.into_param().abi(), ::core::mem::transmute(pcacheentryinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheGetGlobalCacheSize(dwfilter: u32, pullsize: *mut u64, pulllimit: *mut u64) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheGetGlobalCacheSize ( dwfilter : u32 , pullsize : *mut u64 , pulllimit : *mut u64 ) -> u32 );
    UrlCacheGetGlobalCacheSize(dwfilter, pullsize, pulllimit)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheGetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, pulllimit: *mut u64) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheGetGlobalLimit ( limittype : URL_CACHE_LIMIT_TYPE , pulllimit : *mut u64 ) -> u32 );
    UrlCacheGetGlobalLimit(limittype, pulllimit)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheReadEntryStream(hurlcachestream: *const ::core::ffi::c_void, ulllocation: u64, pbuffer: *mut ::core::ffi::c_void, dwbufferlen: u32, pdwbufferlen: *mut u32) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheReadEntryStream ( hurlcachestream : *const ::core::ffi::c_void , ulllocation : u64 , pbuffer : *mut ::core::ffi::c_void , dwbufferlen : u32 , pdwbufferlen : *mut u32 ) -> u32 );
    UrlCacheReadEntryStream(hurlcachestream, ulllocation, pbuffer, dwbufferlen, pdwbufferlen)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheReloadSettings() -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheReloadSettings ( ) -> u32 );
    UrlCacheReloadSettings()
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheRetrieveEntryFile<P0>(happcache: ::core::option::Option<*const ::core::ffi::c_void>, pcwszurl: P0, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentryfile: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheRetrieveEntryFile ( happcache : *const ::core::ffi::c_void , pcwszurl : ::windows::core::PCWSTR , pcacheentryinfo : *mut URLCACHE_ENTRY_INFO , phentryfile : *mut *mut ::core::ffi::c_void ) -> u32 );
    UrlCacheRetrieveEntryFile(::core::mem::transmute(happcache.unwrap_or(::std::ptr::null())), pcwszurl.into_param().abi(), pcacheentryinfo, phentryfile)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheRetrieveEntryStream<P0, P1>(happcache: ::core::option::Option<*const ::core::ffi::c_void>, pcwszurl: P0, frandomread: P1, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentrystream: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheRetrieveEntryStream ( happcache : *const ::core::ffi::c_void , pcwszurl : ::windows::core::PCWSTR , frandomread : super::super::Foundation:: BOOL , pcacheentryinfo : *mut URLCACHE_ENTRY_INFO , phentrystream : *mut *mut ::core::ffi::c_void ) -> u32 );
    UrlCacheRetrieveEntryStream(::core::mem::transmute(happcache.unwrap_or(::std::ptr::null())), pcwszurl.into_param().abi(), frandomread.into_param().abi(), pcacheentryinfo, phentrystream)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheServer() -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheServer ( ) -> u32 );
    UrlCacheServer()
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheSetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, ulllimit: u64) -> u32 {
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheSetGlobalLimit ( limittype : URL_CACHE_LIMIT_TYPE , ulllimit : u64 ) -> u32 );
    UrlCacheSetGlobalLimit(limittype, ulllimit)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[inline]
pub unsafe fn UrlCacheUpdateEntryExtraData<P0>(happcache: ::core::option::Option<*const ::core::ffi::c_void>, pcwszurl: P0, pbextradata: &[u8]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wininet.dll""system" fn UrlCacheUpdateEntryExtraData ( happcache : *const ::core::ffi::c_void , pcwszurl : ::windows::core::PCWSTR , pbextradata : *const u8 , cbextradata : u32 ) -> u32 );
    UrlCacheUpdateEntryExtraData(::core::mem::transmute(happcache.unwrap_or(::std::ptr::null())), pcwszurl.into_param().abi(), ::core::mem::transmute(pbextradata.as_ptr()), pbextradata.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
pub struct IDialBranding(::windows::core::IUnknown);
impl IDialBranding {
    pub unsafe fn Initialize<P0>(&self, pwzconnectoid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pwzconnectoid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmap(&self, dwindex: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Gdi::HBITMAP>();
        (::windows::core::Interface::vtable(self).GetBitmap)(::windows::core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDialBranding, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDialBranding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialBranding {}
impl ::core::fmt::Debug for IDialBranding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialBranding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDialBranding {
    type Vtable = IDialBranding_Vtbl;
}
impl ::core::clone::Clone for IDialBranding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDialBranding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8aecafa9_4306_43cc_8c5a_765f2979cc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialBranding_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzconnectoid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetBitmap: usize,
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
pub struct IDialEngine(::windows::core::IUnknown);
impl IDialEngine {
    pub unsafe fn Initialize<P0, P1>(&self, pwzconnectoid: P0, pides: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IDialEventSink>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pwzconnectoid.into_param().abi(), pides.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty<P0, P1>(&self, pwzproperty: P0, pwzvalue: P1, dwbufsize: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), pwzproperty.into_param().abi(), pwzvalue.into_param().abi(), dwbufsize).ok()
    }
    pub unsafe fn SetProperty<P0, P1>(&self, pwzproperty: P0, pwzvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), pwzproperty.into_param().abi(), pwzvalue.into_param().abi()).ok()
    }
    pub unsafe fn Dial(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Dial)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn HangUp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HangUp)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectedState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetConnectedState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectHandle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::windows::core::zeroed::<usize>();
        (::windows::core::Interface::vtable(self).GetConnectHandle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDialEngine, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDialEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialEngine {}
impl ::core::fmt::Debug for IDialEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDialEngine {
    type Vtable = IDialEngine_Vtbl;
}
impl ::core::clone::Clone for IDialEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDialEngine {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39fd782b_7905_40d5_9148_3c9b190423d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialEngine_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzconnectoid: ::windows::core::PCWSTR, pides: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzproperty: ::windows::core::PCWSTR, pwzvalue: ::windows::core::PCWSTR, dwbufsize: u32) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzproperty: ::windows::core::PCWSTR, pwzvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Dial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HangUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConnectedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnectHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhandle: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
pub struct IDialEventSink(::windows::core::IUnknown);
impl IDialEventSink {
    pub unsafe fn OnEvent(&self, dwevent: u32, dwstatus: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEvent)(::windows::core::Interface::as_raw(self), dwevent, dwstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IDialEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDialEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialEventSink {}
impl ::core::fmt::Debug for IDialEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDialEventSink {
    type Vtable = IDialEventSink_Vtbl;
}
impl ::core::clone::Clone for IDialEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDialEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d86f4ff_6e2d_4488_b2e9_6934afd41bea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwevent: u32, dwstatus: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
pub struct IProofOfPossessionCookieInfoManager(::windows::core::IUnknown);
impl IProofOfPossessionCookieInfoManager {
    pub unsafe fn GetCookieInfoForUri<P0>(&self, uri: P0, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCookieInfoForUri)(::windows::core::Interface::as_raw(self), uri.into_param().abi(), cookieinfocount, cookieinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(IProofOfPossessionCookieInfoManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProofOfPossessionCookieInfoManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProofOfPossessionCookieInfoManager {}
impl ::core::fmt::Debug for IProofOfPossessionCookieInfoManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProofOfPossessionCookieInfoManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProofOfPossessionCookieInfoManager {
    type Vtable = IProofOfPossessionCookieInfoManager_Vtbl;
}
impl ::core::clone::Clone for IProofOfPossessionCookieInfoManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProofOfPossessionCookieInfoManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdaece56_4edf_43df_b113_88e4556fa1bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProofOfPossessionCookieInfoManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCookieInfoForUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
pub struct IProofOfPossessionCookieInfoManager2(::windows::core::IUnknown);
impl IProofOfPossessionCookieInfoManager2 {
    pub unsafe fn GetCookieInfoWithUriForAccount<P0, P1>(&self, webaccount: P0, uri: P1, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetCookieInfoWithUriForAccount)(::windows::core::Interface::as_raw(self), webaccount.into_param().abi(), uri.into_param().abi(), cookieinfocount, cookieinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(IProofOfPossessionCookieInfoManager2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProofOfPossessionCookieInfoManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProofOfPossessionCookieInfoManager2 {}
impl ::core::fmt::Debug for IProofOfPossessionCookieInfoManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProofOfPossessionCookieInfoManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProofOfPossessionCookieInfoManager2 {
    type Vtable = IProofOfPossessionCookieInfoManager2_Vtbl;
}
impl ::core::clone::Clone for IProofOfPossessionCookieInfoManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProofOfPossessionCookieInfoManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15e41407_b42f_4ae7_9966_34a087b2d713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProofOfPossessionCookieInfoManager2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCookieInfoWithUriForAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ANY_CACHE_ENTRY: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const APP_CACHE_ENTRY_TYPE_EXPLICIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const APP_CACHE_ENTRY_TYPE_FALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const APP_CACHE_ENTRY_TYPE_FOREIGN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const APP_CACHE_ENTRY_TYPE_MANIFEST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const APP_CACHE_ENTRY_TYPE_MASTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const APP_CACHE_LOOKUP_NO_MASTER_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTH_FLAG_DISABLE_BASIC_CLEARCHANNEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTH_FLAG_DISABLE_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTH_FLAG_DISABLE_SERVER_AUTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTH_FLAG_ENABLE_NEGOTIATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTH_FLAG_RESET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTODIAL_MODE_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTODIAL_MODE_NEVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTODIAL_MODE_NO_NETWORK_PRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_ALWAYS_DETECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_CACHE_INIT_RUN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_DETECTION_RUN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_DETECTION_SUSPECT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_DONT_CACHE_PROXY_RESULT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_MIGRATED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AUTO_PROXY_FLAG_USER_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_GET_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_GROUPNAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_QUOTA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_STORAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ATTRIBUTE_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_FLAG_FLUSHURL_ONDELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_FLAG_GIDONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_FLAG_NONPURGEABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_FLAG_VALID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_ID_BUILTIN_STICKY: u64 = 1152921504606846983u64;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_SEARCH_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_SEARCH_BYURL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHEGROUP_TYPE_INVALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_APPCONTAINER_CONTENT_QUOTA_FC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_APPCONTAINER_TOTAL_CONTENT_QUOTA_FC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_CONTENT_QUOTA_FC: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_TOTAL_CONTENT_QUOTA_FC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_ACCTIME_FC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_ATTRIBUTE_FC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_EXEMPT_DELTA_FC: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_EXPTIME_FC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_HEADERINFO_FC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_HITRATE_FC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_MODIFY_DATA_FC: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_MODTIME_FC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_SYNCTIME_FC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_ENTRY_TYPE_FC: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_FIND_CONTAINER_RETURN_NOCHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_READ_COUNT_SINCE_LAST_SCAVENGE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_12: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_13: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_15: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_17: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_18: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_19: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_20: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_23: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_24: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_25: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_26: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_28: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_29: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_30: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_RESERVED_31: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CACHE_WRITE_COUNT_SINCE_LAST_SCAVENGE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CONLIST_CHANGE_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_COOKIE_CHANGE_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_CURRENT_SETTINGS_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_DOWNLOAD_PARTIAL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_GID_HIGH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_GID_LOW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_HSTS_CHANGE_COUNT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_LAST: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_LAST_SCAVENGE_TIMESTAMP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_NOTIFICATION_FILTER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_NOTIFICATION_HWND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_NOTIFICATION_MESG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_ROOTGROUP_OFFSET: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_ROOT_GROUPLIST_OFFSET: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_ROOT_LEAK_OFFSET: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_HEADER_DATA_SSL_STATE_COUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_ADD_URL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_DELETE_ALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_DELETE_URL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_FILTER_CHANGED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_SET_OFFLINE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_SET_ONLINE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_UPDATE_URL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_URL_SET_STICKY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_NOTIFY_URL_UNSET_STICKY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_ACCEPTED_CACHE_ENTRY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_ALLOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_ALLOW_ALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_CACHE_ENTRY: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_DONT_ALLOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_DONT_ALLOW_ALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_DOWNGRADED_CACHE_ENTRY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_LEASHED_CACHE_ENTRY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_OP_3RD_PARTY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_OP_GET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_OP_MODIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_OP_PERSISTENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_OP_SESSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_OP_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_REJECTED_CACHE_ENTRY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_LB: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_UB: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALENG_OperationComplete: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALENG_RedialAttempt: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALENG_RedialWait: u32 = 65538u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_DOMAIN: ::windows::core::PCWSTR = ::windows::core::w!("Domain");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_LASTERROR: ::windows::core::PCWSTR = ::windows::core::w!("LastError");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_PASSWORD: ::windows::core::PCWSTR = ::windows::core::w!("Password");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_PHONENUMBER: ::windows::core::PCWSTR = ::windows::core::w!("PhoneNumber");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_REDIALCOUNT: ::windows::core::PCWSTR = ::windows::core::w!("RedialCount");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_REDIALINTERVAL: ::windows::core::PCWSTR = ::windows::core::w!("RedialInterval");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_RESOLVEDPHONE: ::windows::core::PCWSTR = ::windows::core::w!("ResolvedPhone");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_SAVEPASSWORD: ::windows::core::PCWSTR = ::windows::core::w!("SavePassword");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DIALPROP_USERNAME: ::windows::core::PCWSTR = ::windows::core::w!("UserName");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DLG_FLAGS_INSECURE_FALLBACK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DLG_FLAGS_INVALID_CA: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DLG_FLAGS_SEC_CERT_CN_INVALID: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DLG_FLAGS_SEC_CERT_DATE_INVALID: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DLG_FLAGS_SEC_CERT_REV_FAILED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DLG_FLAGS_WEAK_SIGNATURE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DOWNLOAD_CACHE_ENTRY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DUO_PROTOCOL_FLAG_SPDY3: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const DUO_PROTOCOL_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const EDITED_CACHE_ENTRY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_FTP_DROPPED: u32 = 12111u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_FTP_NO_PASSIVE_MODE: u32 = 12112u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_FTP_TRANSFER_IN_PROGRESS: u32 = 12110u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_ATTRIBUTE_NOT_FOUND: u32 = 12137u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_DATA_ERROR: u32 = 12132u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_END_OF_DATA: u32 = 12133u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_INCORRECT_LOCATOR_TYPE: u32 = 12135u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_INVALID_LOCATOR: u32 = 12134u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_NOT_FILE: u32 = 12131u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_NOT_GOPHER_PLUS: u32 = 12136u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_PROTOCOL_ERROR: u32 = 12130u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_GOPHER_UNKNOWN_LOCATOR: u32 = 12138u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_COOKIE_DECLINED: u32 = 12162u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION: u32 = 12161u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION_EX: u32 = 12907u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_DOWNLEVEL_SERVER: u32 = 12151u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_HEADER_ALREADY_EXISTS: u32 = 12155u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_HEADER_NOT_FOUND: u32 = 12150u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_HSTS_REDIRECT_REQUIRED: u32 = 12060u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_INVALID_HEADER: u32 = 12153u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_INVALID_QUERY_REQUEST: u32 = 12154u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_INVALID_SERVER_RESPONSE: u32 = 12152u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_NOT_REDIRECTED: u32 = 12160u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_PUSH_ENABLE_FAILED: u32 = 12149u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_PUSH_RETRY_NOT_SUPPORTED: u32 = 12148u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_PUSH_STATUS_CODE_NOT_SUPPORTED: u32 = 12147u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_REDIRECT_FAILED: u32 = 12156u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_HTTP_REDIRECT_NEEDS_CONFIRMATION: u32 = 12168u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_ASYNC_THREAD_FAILED: u32 = 12047u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_BAD_AUTO_PROXY_SCRIPT: u32 = 12166u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_BAD_OPTION_LENGTH: u32 = 12010u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_BAD_REGISTRY_PARAMETER: u32 = 12022u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CACHE_SUCCESS: u32 = 12906u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CANNOT_CONNECT: u32 = 12029u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CHG_POST_IS_NON_SECURE: u32 = 12042u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED: u32 = 12044u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED_PROXY: u32 = 12187u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CLIENT_AUTH_NOT_SETUP: u32 = 12046u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CONNECTION_ABORTED: u32 = 12030u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CONNECTION_AVAILABLE: u32 = 12902u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_CONNECTION_RESET: u32 = 12031u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_DECODING_FAILED: u32 = 12175u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_DIALOG_PENDING: u32 = 12049u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_DISALLOW_INPRIVATE: u32 = 12189u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_DISCONNECTED: u32 = 12163u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_EXTENDED_ERROR: u32 = 12003u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_FAILED_DUETOSECURITYCHECK: u32 = 12171u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_FEATURE_DISABLED: u32 = 12192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_FORCE_RETRY: u32 = 12032u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_FORTEZZA_LOGIN_NEEDED: u32 = 12054u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_GLOBAL_CALLBACK_FAILED: u32 = 12191u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_HANDLE_EXISTS: u32 = 12036u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_HTTPS_HTTP_SUBMIT_REDIR: u32 = 12052u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_HTTPS_TO_HTTP_ON_REDIR: u32 = 12040u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_HTTP_PROTOCOL_MISMATCH: u32 = 12190u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_HTTP_TO_HTTPS_ON_REDIR: u32 = 12039u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INCORRECT_FORMAT: u32 = 12027u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INCORRECT_HANDLE_STATE: u32 = 12019u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INCORRECT_HANDLE_TYPE: u32 = 12018u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INCORRECT_PASSWORD: u32 = 12014u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INCORRECT_USER_NAME: u32 = 12013u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INSECURE_FALLBACK_REQUIRED: u32 = 12059u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INSERT_CDROM: u32 = 12053u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INTERNAL_ERROR: u32 = 12004u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INTERNAL_SOCKET_ERROR: u32 = 12901u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INVALID_CA: u32 = 12045u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INVALID_OPERATION: u32 = 12016u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INVALID_OPTION: u32 = 12009u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INVALID_PROXY_REQUEST: u32 = 12033u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_INVALID_URL: u32 = 12005u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_ITEM_NOT_FOUND: u32 = 12028u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_LOGIN_FAILURE: u32 = 12015u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 12174u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_MIXED_SECURITY: u32 = 12041u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NAME_NOT_RESOLVED: u32 = 12007u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NEED_MSN_SSPI_PKG: u32 = 12173u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NEED_UI: u32 = 12034u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NOT_INITIALIZED: u32 = 12172u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NOT_PROXY_REQUEST: u32 = 12020u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_CALLBACK: u32 = 12025u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_CM_CONNECTION: u32 = 12080u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_CONTEXT: u32 = 12024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_DIRECT_ACCESS: u32 = 12023u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_KNOWN_SERVERS: u32 = 12903u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_NEW_CONTAINERS: u32 = 12051u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_NO_PING_SUPPORT: u32 = 12905u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_OFFLINE: u32 = 12163u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_OPERATION_CANCELLED: u32 = 12017u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_OPTION_NOT_SETTABLE: u32 = 12011u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_OUT_OF_HANDLES: u32 = 12001u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_PING_FAILED: u32 = 12904u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_POST_IS_NON_SECURE: u32 = 12043u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_PROTOCOL_NOT_FOUND: u32 = 12008u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_PROXY_ALERT: u32 = 12061u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_PROXY_SERVER_UNREACHABLE: u32 = 12165u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_REDIRECT_SCHEME_CHANGE: u32 = 12048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_REGISTRY_VALUE_NOT_FOUND: u32 = 12021u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_REQUEST_PENDING: u32 = 12026u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_RETRY_DIALOG: u32 = 12050u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SECURE_FAILURE_PROXY: u32 = 12188u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SECURITY_CHANNEL_ERROR: u32 = 12157u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_CN_INVALID: u32 = 12038u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_DATE_INVALID: u32 = 12037u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_ERRORS: u32 = 12055u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_NO_REV: u32 = 12056u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_REVOKED: u32 = 12170u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_REV_FAILED: u32 = 12057u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_CERT_WEAK_SIGNATURE: u32 = 12062u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SEC_INVALID_CERT: u32 = 12169u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SERVER_UNREACHABLE: u32 = 12164u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SHUTDOWN: u32 = 12012u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_SOURCE_PORT_IN_USE: u32 = 12058u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_TCPIP_NOT_INSTALLED: u32 = 12159u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_TIMEOUT: u32 = 12002u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_UNABLE_TO_CACHE_FILE: u32 = 12158u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_UNABLE_TO_DOWNLOAD_SCRIPT: u32 = 12167u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ERROR_INTERNET_UNRECOGNIZED_SCHEME: u32 = 12006u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAGS_ERROR_UI_FILTER_FOR_ERRORS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAGS_ERROR_UI_FLAGS_CHANGE_OPTIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAGS_ERROR_UI_FLAGS_GENERATE_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAGS_ERROR_UI_FLAGS_NO_UI: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAGS_ERROR_UI_SERIALIZE_DIALOGS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAGS_ERROR_UI_SHOW_IDN_HOSTNAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FLAG_ICC_FORCE_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ABSTRACT_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Abstract");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ABSTRACT_CATEGORY: ::windows::core::PCWSTR = ::windows::core::w!("+ABSTRACT");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ADMIN_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Admin");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ADMIN_CATEGORY: ::windows::core::PCWSTR = ::windows::core::w!("+ADMIN");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_ABSTRACT: u32 = 2882325526u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_ADMIN: u32 = 2882325514u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_ALL: u32 = 2882325513u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_BASE: u32 = 2882325504u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_GEOG: u32 = 2882325522u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_LOCATION: u32 = 2882325521u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_MOD_DATE: u32 = 2882325515u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_ORG: u32 = 2882325520u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_PROVIDER: u32 = 2882325524u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_RANGE: u32 = 2882325518u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_SCORE: u32 = 2882325517u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_SITE: u32 = 2882325519u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_TIMEZONE: u32 = 2882325523u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_TREEWALK: u32 = 2882325528u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_TTL: u32 = 2882325516u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_UNKNOWN: u32 = 2882325529u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_VERSION: u32 = 2882325525u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ATTRIBUTE_ID_VIEW: u32 = 2882325527u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_ABSTRACT: u32 = 2882325509u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_ADMIN: u32 = 2882325507u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_ALL: u32 = 2882325505u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_ASK: u32 = 2882325511u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_INFO: u32 = 2882325506u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_UNKNOWN: u32 = 2882325512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_VERONICA: u32 = 2882325510u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_CATEGORY_ID_VIEWS: u32 = 2882325508u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_GEOG_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Geog");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_INFO_CATEGORY: ::windows::core::PCWSTR = ::windows::core::w!("+INFO");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_LOCATION_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Loc");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_MOD_DATE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Mod-Date");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_ORG_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Org");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_PROVIDER_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Provider");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_RANGE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Score-range");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_SCORE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Score");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_SITE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Site");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TIMEZONE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("TZ");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TREEWALK_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("treewalk");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TTL_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("TTL");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_VERONICA_CATEGORY: ::windows::core::PCWSTR = ::windows::core::w!("+VERONICA");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_VERSION_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("Version");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_VIEWS_CATEGORY: ::windows::core::PCWSTR = ::windows::core::w!("+VIEWS");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_VIEW_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::core::w!("View");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GROUPNAME_MAX_LENGTH: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GROUP_OWNER_STORAGE_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HSR_ASYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HSR_CHUNKED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HSR_DOWNLOAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HSR_INITIATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HSR_SYNC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HSR_USE_CONTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_1_1_CACHE_ENTRY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_ALLOW_EMPTY_VALUES: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_RESPONSE_HEADERS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_INDEX_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE_LAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_COOKIES_SAME_SITE_LEVEL_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_COOKIES_SAME_SITE_LEVEL_SAME_SITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_COOKIES_SAME_SITE_LEVEL_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_PROTOCOL_FLAG_HTTP2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_PROTOCOL_MASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ACCEPT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ACCEPT_CHARSET: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ACCEPT_ENCODING: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ACCEPT_LANGUAGE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ACCEPT_RANGES: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_AGE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ALLOW: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_AUTHENTICATION_INFO: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_AUTHORIZATION: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CACHE_CONTROL: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONNECTION: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_BASE: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_DESCRIPTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_DISPOSITION: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_ENCODING: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_LANGUAGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_LENGTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_LOCATION: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_MD5: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_RANGE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_TRANSFER_ENCODING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CONTENT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_COOKIE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_COST: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_CUSTOM: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_DATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_DEFAULT_STYLE: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_DERIVED_FROM: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_DO_NOT_TRACK: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ECHO_HEADERS: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ECHO_HEADERS_CRLF: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ECHO_REPLY: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ECHO_REQUEST: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ETAG: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_EXPECT: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_EXPIRES: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FLAG_COALESCE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FLAG_COALESCE_WITH_COMMA: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FLAG_NUMBER: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FLAG_NUMBER64: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FLAG_SYSTEMTIME: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FORWARDED: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_FROM: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_HOST: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_HTTP2_SETTINGS: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_IF_MATCH: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_IF_MODIFIED_SINCE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_IF_NONE_MATCH: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_IF_RANGE: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_IF_UNMODIFIED_SINCE: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_INCLUDE_REFERER_TOKEN_BINDING_ID: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_KEEP_ALIVE: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_LAST_MODIFIED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_LINK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_LOCATION: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_MAX: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_MAX_FORWARDS: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_MESSAGE_ID: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_MIME_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_ORIG_URI: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_P3P: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PASSPORT_CONFIG: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PASSPORT_URLS: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PRAGMA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PROXY_AUTHENTICATE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PROXY_AUTHORIZATION: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PROXY_CONNECTION: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PROXY_SUPPORT: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PUBLIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PUBLIC_KEY_PINS: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_PUBLIC_KEY_PINS_REPORT_ONLY: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_RANGE: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_RAW_HEADERS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_REFERER: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_REFRESH: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_REQUEST_METHOD: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_RETRY_AFTER: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_SERVER: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_SET_COOKIE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_SET_COOKIE2: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_STATUS_CODE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_STATUS_TEXT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_STRICT_TRANSPORT_SECURITY: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_TITLE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_TOKEN_BINDING: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_TRANSFER_ENCODING: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_TRANSLATE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_UNLESS_MODIFIED_SINCE: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_UPGRADE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_URI: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_USER_AGENT: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_VARY: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_VERSION: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_VIA: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_WARNING: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_WWW_AUTHENTICATE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_X_CONTENT_TYPE_OPTIONS: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_X_FRAME_OPTIONS: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_X_P2P_PEERDIST: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_X_UA_COMPATIBLE: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_QUERY_X_XSS_PROTECTION: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_STATUS_MISDIRECTED_REQUEST: u32 = 421u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_VERSIONA: ::windows::core::PCSTR = ::windows::core::s!("HTTP/1.0");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_VERSIONW: ::windows::core::PCWSTR = ::windows::core::w!("HTTP/1.0");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ICU_USERNAME: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IDENTITY_CACHE_ENTRY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IDSI_FLAG_KEEP_ALIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IDSI_FLAG_PROXY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IDSI_FLAG_SECURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IDSI_FLAG_TUNNEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IMMUTABLE_CACHE_ENTRY: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INSTALLED_CACHE_ENTRY: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERENT_GOONLINE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERENT_GOONLINE_NOPROMPT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERENT_GOONLINE_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_BASIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_DIGEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_KERBEROS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_NEGOTIATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_PASSPORT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTH_SCHEME_UNKNOWN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTOPROXY_INIT_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTOPROXY_INIT_DOWNLOADSYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTOPROXY_INIT_ONLYQUERY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTOPROXY_INIT_QUERYSTATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_AUTODELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_BLOOM_FILTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_MAP_ENABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_NODESKTOPINIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_NOSUBDIRS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_RESERVED1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_SHARE_READ: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_CONTAINER_SHARE_READ_WRITE: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_FLAG_ADD_FILENAME_ONLY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_FLAG_ALLOW_COLLISIONS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_FLAG_ENTRY_OR_MAPPING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_FLAG_GET_STRUCT_ONLY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_FLAG_INSTALLED_ENTRY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_GROUP_ADD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CACHE_GROUP_REMOVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_ALL_COOKIES: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_APPLY_HOST_ONLY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_APPLY_P3P: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_ECTX_3RDPARTY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_EDGE_COOKIES: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_EVALUATE_P3P: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_HOST_ONLY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_HOST_ONLY_APPLIED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_IE6: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_IS_LEGACY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_IS_RESTRICTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_IS_SECURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_IS_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_NON_SCRIPT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_NO_CALLBACK: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_P3P_ENABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_PERSISTENT_HOST_ONLY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_PROMPT_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_RESTRICTED_ZONE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_SAME_SITE_LAX: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_SAME_SITE_STRICT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_CAN_HANGUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_CONNECT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_DISCONNECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_SAFE_FOR_UNATTENDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_SHOWOFFLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_UNATTENDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CUSTOMDIAL_WILL_SUPPLY_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DEFAULT_FTP_PORT: u16 = 21u16;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DEFAULT_GOPHER_PORT: u16 = 70u16;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DEFAULT_SOCKS_PORT: u16 = 1080u16;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DIALSTATE_DISCONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DIAL_FORCE_PROMPT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DIAL_SHOW_OFFLINE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_DIAL_UNATTENDED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_ERROR_BASE: u32 = 12000u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_ERROR_LAST: u32 = 12192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_ERROR_MASK_COMBINED_SEC_CERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_ERROR_MASK_INSERT_CDROM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_ERROR_MASK_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_ERROR_MASK_NEED_MSN_SSPI_PKG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FIRST_OPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_ASYNC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_BGUPDATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_CACHE_ASYNC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_CACHE_IF_NET_FAIL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_DONT_CACHE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_EXISTING_CONNECT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_FORMS_SUBMIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_FROM_CACHE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_FTP_FOLDER_VIEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_FWD_BACK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_HYPERLINK: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_IDN_DIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_IDN_PROXY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_KEEP_CONNECTION: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_MAKE_PERSISTENT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_MUST_CACHE_REQUEST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_NEED_FILE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_NO_AUTH: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_NO_AUTO_REDIRECT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_NO_CACHE_WRITE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_NO_COOKIES: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_NO_UI: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_OFFLINE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_PASSIVE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_PRAGMA_NOCACHE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_RAW_DATA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_READ_PREFETCH: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_RELOAD: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_RESYNCHRONIZE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_SECURE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_GLOBAL_CALLBACK_DETECTING_PROXY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_GLOBAL_CALLBACK_SENDING_HTTP_HEADERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_CONNECT_FTP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_CONNECT_GOPHER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_CONNECT_HTTP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_FILE_REQUEST: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_FTP_FILE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_FTP_FILE_HTML: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_FTP_FIND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_FTP_FIND_HTML: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE_HTML: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND_HTML: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_HTTP_REQUEST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_HANDLE_TYPE_INTERNET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_IDENTITY_FLAG_CLEAR_CONTENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_IDENTITY_FLAG_CLEAR_COOKIES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_IDENTITY_FLAG_CLEAR_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_IDENTITY_FLAG_CLEAR_HISTORY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_IDENTITY_FLAG_PRIVATE_CACHE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_IDENTITY_FLAG_SHARED_CACHE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_INTERNAL_ERROR_BASE: u32 = 12900u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_INVALID_PORT_NUMBER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_KEEP_ALIVE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_KEEP_ALIVE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_KEEP_ALIVE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_LAST_OPTION: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_LAST_OPTION_INTERNAL: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_MAX_HOST_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_MAX_PASSWORD_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_MAX_PORT_NUMBER_LENGTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_MAX_PORT_NUMBER_VALUE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_MAX_USER_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_NO_CALLBACK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPEN_TYPE_PRECONFIG_WITH_NO_AUTOPROXY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ACTIVATE_WORKER_THREADS: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ACTIVITY_ID: u32 = 185u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ALLOW_FAILED_CONNECT_CONTENT: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ALLOW_INSECURE_FALLBACK: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ALTER_IDENTITY: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_APP_CACHE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ASYNC: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ASYNC_ID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ASYNC_PRIORITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_AUTH_FLAGS: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_AUTH_SCHEME_SELECTED: u32 = 183u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_AUTODIAL_CONNECTION: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_AUTODIAL_HWND: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_AUTODIAL_MODE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_BACKGROUND_CONNECTIONS: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_BYPASS_EDITED_ENTRY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CACHE_ENTRY_EXTRA_DATA: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CACHE_PARTITION: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CACHE_STREAM_HANDLE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CACHE_TIMESTAMPS: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CALLBACK_FILTER: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CALLER_MODULE: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CANCEL_CACHE_WRITE: u32 = 182u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CERT_ERROR_FLAGS: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CHUNK_ENCODE_REQUEST: u32 = 150u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CLIENT_CERT_CONTEXT: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CLIENT_CERT_ISSUER_LIST: u32 = 153u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CM_HANDLE_COPY_REF: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CODEPAGE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CODEPAGE_EXTRA: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CODEPAGE_PATH: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH: u32 = 147u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECTED_STATE: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECTION_FILTER: u32 = 162u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECTION_INFO: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECT_BACKOFF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECT_LIMIT: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECT_TIME: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONTEXT_VALUE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONTEXT_VALUE_OLD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_CONTROL_SEND_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_COOKIES_3RD_PARTY: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_COOKIES_APPLY_HOST_ONLY: u32 = 179u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL: u32 = 187u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DATAFILE_EXT: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DATAFILE_NAME: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DATA_RECEIVE_TIMEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DATA_SEND_TIMEOUT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DEPENDENCY_HANDLE: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DETECT_POST_SEND: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DIAGNOSTIC_SOCKET_INFO: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DIGEST_AUTH_UNLOAD: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISABLE_AUTODIAL: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISABLE_INSECURE_FALLBACK: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISABLE_NTLM_PREAUTH: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISABLE_PASSPORT_AUTH: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISABLE_PROXY_LINK_LOCAL_NAME_RESOLUTION: u32 = 190u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISALLOW_PREMATURE_EOF: u32 = 137u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DISCONNECTED_TIMEOUT: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DOWNLOAD_MODE: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DOWNLOAD_MODE_HANDLE: u32 = 165u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DO_NOT_TRACK: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_DUO_USED: u32 = 149u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_EDGE_COOKIES: u32 = 166u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_EDGE_COOKIES_TEMP: u32 = 175u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_EDGE_MODE: u32 = 180u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_DUO: u32 = 148u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_HEADER_CALLBACKS: u32 = 168u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 148u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_PASSPORT_AUTH: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_REDIRECT_CACHE_READ: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_TEST_SIGNING: u32 = 189u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_WBOEXT: u32 = 158u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENABLE_ZLIB_DEFLATE: u32 = 173u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENCODE_EXTRA: u32 = 155u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENCODE_FALLBACK_FOR_REDIRECT_URI: u32 = 174u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_END_BROWSER_SESSION: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ENTERPRISE_CONTEXT: u32 = 159u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ERROR_MASK: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_EXEMPT_CONNECTION_LIMIT: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_EXTENDED_CALLBACKS: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_EXTENDED_ERROR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_FAIL_ON_CACHE_WRITE_ERROR: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_FALSE_START: u32 = 141u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_FLUSH_STATE: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_FORCE_DECODE: u32 = 178u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_FROM_CACHE_TIMEOUT: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_GLOBAL_CALLBACK: u32 = 188u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HANDLE_TYPE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HIBERNATE_INACTIVE_WORKER_THREADS: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HSTS: u32 = 157u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HTTP_09: u32 = 191u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HTTP_DECODING: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HTTP_PROTOCOL_USED: u32 = 149u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_HTTP_VERSION: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_IDENTITY: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_IDLE_STATE: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_IDN: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_IGNORE_CERT_ERROR_FLAGS: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_IGNORE_OFFLINE: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_KEEP_CONNECTION: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_LINE_STATE: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_LISTEN_TIMEOUT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_MAX_CONNS_PER_1_0_SERVER: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_MAX_CONNS_PER_PROXY: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_MAX_CONNS_PER_SERVER: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_MAX_QUERY_BUFFER_SIZE: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_NET_SPEED: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_NOCACHE_WRITE_IN_PRIVATE: u32 = 184u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_NOTIFY_SENDING_COOKIE: u32 = 152u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_NO_HTTP_SERVER_AUTH: u32 = 167u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_OFFLINE_MODE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_OFFLINE_SEMANTICS: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_OFFLINE_TIMEOUT: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_OPT_IN_WEAK_SIGNATURE: u32 = 176u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_ORIGINAL_CONNECT_FLAGS: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PARENT_HANDLE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PARSE_LINE_FOLDING: u32 = 177u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PASSWORD: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PER_CONNECTION_OPTION: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_POLICY: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PRESERVE_REFERER_ON_HTTPS_TO_HTTP_REDIRECT: u32 = 170u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PRESERVE_REQUEST_SERVER_CREDENTIALS_ON_REDIRECT: u32 = 169u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY_AUTH_SCHEME: u32 = 144u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY_CREDENTIALS: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY_FROM_REQUEST: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY_PASSWORD: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY_SETTINGS_CHANGED: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_PROXY_USERNAME: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_READ_BUFFER_SIZE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_RECEIVE_THROUGHPUT: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_RECEIVE_TIMEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 163u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REFRESH: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REMOVE_IDENTITY: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REQUEST_ANNOTATION: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REQUEST_FLAGS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REQUEST_PRIORITY: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_REQUEST_TIMES: u32 = 186u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_RESET: u32 = 154u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_RESET_URLCACHE_SESSION: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_RESPONSE_RESUMABLE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_RESTORE_WORKER_THREAD_DEFAULTS: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECONDARY_CACHE_KEY: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURE_FAILURE: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURITY_CERTIFICATE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURITY_CERTIFICATE_STRUCT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURITY_CONNECTION_INFO: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURITY_FLAGS: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURITY_KEY_BITNESS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SECURITY_SELECT_CLIENT_CERT: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SEND_THROUGHPUT: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SEND_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SEND_UTF8_SERVERNAME_TO_PROXY: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SERVER_ADDRESS_INFO: u32 = 156u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SERVER_AUTH_SCHEME: u32 = 143u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SERVER_CERT_CHAIN_CONTEXT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SERVER_CREDENTIALS: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SESSION_START_TIME: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SETTINGS_CHANGED: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SET_IN_PRIVATE: u32 = 164u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SOCKET_NODELAY: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SOCKET_NOTIFICATION_IOCTL: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SOCKET_SEND_BUFFER_LENGTH: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SOURCE_PORT: u32 = 146u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SUPPRESS_BEHAVIOR: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SUPPRESS_SERVER_AUTH: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_SYNC_MODE_AUTOMATIC_SESSION_DISABLED: u32 = 172u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_TCP_FAST_OPEN: u32 = 171u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_TIMED_CONNECTION_LIMIT_BYPASS: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_TOKEN_BINDING_PUBLIC_KEY: u32 = 181u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_TUNNEL_ONLY: u32 = 145u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_UNLOAD_NOTIFY_EVENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_UPGRADE_TO_WEB_SOCKET: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_URL: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_USERNAME: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_USER_AGENT: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_USER_PASS_SERVER_ONLY: u32 = 142u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_USE_FIRST_AVAILABLE_CONNECTION: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_USE_MODIFIED_HEADER_FILTER: u32 = 124u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_VERSION: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_WPAD_SLEEP: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_WRITE_BUFFER_SIZE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPTION_WWA_MODE: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_FLAGS_UI: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PREFETCH_ABORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PREFETCH_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PREFETCH_PROGRESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PRIORITY_FOREGROUND: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_CACHE_WRITE_DISABLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_FROM_APP_CACHE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_FROM_CACHE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_NET_TIMEOUT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_NO_HEADERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_PASSIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_REQFLAG_VIA_PROXY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_RFC1123_BUFSIZE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_RFC1123_FORMAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SERVICE_FTP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SERVICE_GOPHER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SERVICE_HTTP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SERVICE_URL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_CLOSING_CONNECTION: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_CONNECTED_TO_SERVER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_CONNECTING_TO_SERVER: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_CONNECTION_CLOSED: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_COOKIE: u32 = 430u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_COOKIE_HISTORY: u32 = 327u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_COOKIE_RECEIVED: u32 = 321u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_COOKIE_SENT: u32 = 320u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_CTL_RESPONSE_RECEIVED: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_DETECTING_PROXY: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_END_BROWSER_SESSION: u32 = 420u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_CLOSED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_CLOSING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_CONNECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_CONNECTING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_HANDLE_CLOSING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_HANDLE_CREATED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_PREFETCH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_RECEIVED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_RECEIVING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_REDIRECT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_RESOLVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_RESOLVING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_SENDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_SENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_FILTER_STATE_CHANGE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_HANDLE_CLOSING: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_HANDLE_CREATED: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_INTERMEDIATE_RESPONSE: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_NAME_RESOLVED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_P3P_HEADER: u32 = 325u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_P3P_POLICYREF: u32 = 326u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_PREFETCH: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_PRIVACY_IMPACTED: u32 = 324u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_PROXY_CREDENTIALS: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_RECEIVING_RESPONSE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_REDIRECT: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_REQUEST_COMPLETE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_REQUEST_HEADERS_SET: u32 = 329u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_REQUEST_SENT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_RESOLVING_NAME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_RESPONSE_HEADERS_SET: u32 = 330u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_RESPONSE_RECEIVED: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_SENDING_COOKIE: u32 = 328u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_SENDING_REQUEST: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_SERVER_CONNECTION_STATE: u32 = 410u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_SERVER_CREDENTIALS: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_STATE_CHANGE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATUS_USER_INPUT_REQUIRED: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SUPPRESS_COOKIE_PERSIST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SUPPRESS_COOKIE_PERSIST_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SUPPRESS_COOKIE_POLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SUPPRESS_COOKIE_POLICY_RESET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SUPPRESS_RESET_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IRF_ASYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IRF_NO_WAIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IRF_SYNC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const IRF_USE_CONTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ISO_FORCE_DISCONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ISO_FORCE_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ISO_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ISO_REGISTRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const LOCAL_NAMESPACE_PREFIX: ::windows::core::PCSTR = ::windows::core::s!("Local\\");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const LOCAL_NAMESPACE_PREFIX_W: ::windows::core::PCWSTR = ::windows::core::w!("Local\\");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MAX_CACHE_ENTRY_INFO_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MAX_GOPHER_ATTRIBUTE_NAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MAX_GOPHER_CATEGORY_NAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MAX_GOPHER_DISPLAY_TEXT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MAX_GOPHER_HOST_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MAX_GOPHER_SELECTOR_TEXT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MIN_GOPHER_ATTRIBUTE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MUST_REVALIDATE_CACHE_ENTRY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const MaxPrivacySettings: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const NORMAL_CACHE_ENTRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const OTHER_USER_CACHE_ENTRY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PENDING_DELETE_CACHE_ENTRY: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const POST_CHECK_CACHE_ENTRY: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const POST_RESPONSE_CACHE_ENTRY: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_IMPACTED_CACHE_ENTRY: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_MODE_CACHE_ENTRY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_ADVANCED: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_CUSTOM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_HIGH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_LOW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_MAX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_MEDIUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_MEDIUM_HIGH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_MEDIUM_LOW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TEMPLATE_NO_COOKIES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TYPE_FIRST_PARTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PRIVACY_TYPE_THIRD_PARTY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PROXY_TYPE_AUTO_DETECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PROXY_TYPE_AUTO_PROXY_URL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PROXY_TYPE_DIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PROXY_TYPE_PROXY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ProofOfPossessionCookieInfoManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9927f85_a304_4390_8b23_a75f1c668600);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const REDIRECT_CACHE_ENTRY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const REGSTR_DIAL_AUTOCONNECT: ::windows::core::PCSTR = ::windows::core::s!("AutoConnect");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const REGSTR_LEASH_LEGACY_COOKIES: ::windows::core::PCSTR = ::windows::core::s!("LeashLegacyCookies");
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_128BIT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_40BIT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_56BIT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_FORTEZZA: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_IETFSSL4: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_IGNORE_REVOCATION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_IGNORE_WEAK_SIGNATURE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_IGNORE_WRONG_USAGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_NORMALBITNESS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_OPT_IN_WEAK_SIGNATURE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_PCT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_PCT4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_SSL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_SSL3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SECURITY_FLAG_UNKNOWNBIT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SHORTPATH_CACHE_ENTRY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const SPARSE_CACHE_ENTRY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const STATIC_CACHE_ENTRY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const STICKY_CACHE_ENTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const TRACK_OFFLINE_CACHE_ENTRY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const TRACK_ONLINE_CACHE_ENTRY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const URLHISTORY_CACHE_ENTRY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_API_FLAG_ASYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_API_FLAG_SYNC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_API_FLAG_USE_CONTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const XDR_CACHE_ENTRY: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APP_CACHE_FINALIZE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheFinalizeStateIncomplete: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheFinalizeStateManifestChange: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheFinalizeStateComplete: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(2i32);
impl ::core::marker::Copy for APP_CACHE_FINALIZE_STATE {}
impl ::core::clone::Clone for APP_CACHE_FINALIZE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APP_CACHE_FINALIZE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APP_CACHE_FINALIZE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APP_CACHE_FINALIZE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APP_CACHE_FINALIZE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APP_CACHE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheStateNoUpdateNeeded: APP_CACHE_STATE = APP_CACHE_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheStateUpdateNeeded: APP_CACHE_STATE = APP_CACHE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheStateUpdateNeededNew: APP_CACHE_STATE = APP_CACHE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const AppCacheStateUpdateNeededMasterOnly: APP_CACHE_STATE = APP_CACHE_STATE(3i32);
impl ::core::marker::Copy for APP_CACHE_STATE {}
impl ::core::clone::Clone for APP_CACHE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APP_CACHE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APP_CACHE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APP_CACHE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APP_CACHE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CACHE_CONFIG(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_FORCE_CLEANUP_FC: CACHE_CONFIG = CACHE_CONFIG(32u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_DISK_CACHE_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(64u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_SYNC_MODE_FC: CACHE_CONFIG = CACHE_CONFIG(128u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_CONTENT_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(256u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_HISTORY_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(1024u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_COOKIES_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(512u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_QUOTA_FC: CACHE_CONFIG = CACHE_CONFIG(2048u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_USER_MODE_FC: CACHE_CONFIG = CACHE_CONFIG(4096u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_CONTENT_USAGE_FC: CACHE_CONFIG = CACHE_CONFIG(8192u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const CACHE_CONFIG_STICKY_CONTENT_USAGE_FC: CACHE_CONFIG = CACHE_CONFIG(16384u32);
impl ::core::marker::Copy for CACHE_CONFIG {}
impl ::core::clone::Clone for CACHE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CACHE_CONFIG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CACHE_CONFIG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CACHE_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CACHE_CONFIG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FORTCMD(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FORTCMD_LOGON: FORTCMD = FORTCMD(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FORTCMD_LOGOFF: FORTCMD = FORTCMD(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FORTCMD_CHG_PERSONALITY: FORTCMD = FORTCMD(3i32);
impl ::core::marker::Copy for FORTCMD {}
impl ::core::clone::Clone for FORTCMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FORTCMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FORTCMD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FORTCMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORTCMD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FORTSTAT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FORTSTAT_INSTALLED: FORTSTAT = FORTSTAT(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FORTSTAT_LOGGEDON: FORTSTAT = FORTSTAT(2i32);
impl ::core::marker::Copy for FORTSTAT {}
impl ::core::clone::Clone for FORTSTAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FORTSTAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FORTSTAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FORTSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORTSTAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FTP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FTP_TRANSFER_TYPE_ASCII: FTP_FLAGS = FTP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FTP_TRANSFER_TYPE_BINARY: FTP_FLAGS = FTP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const FTP_TRANSFER_TYPE_UNKNOWN: FTP_FLAGS = FTP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_TRANSFER_ASCII: FTP_FLAGS = FTP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_TRANSFER_BINARY: FTP_FLAGS = FTP_FLAGS(2u32);
impl ::core::marker::Copy for FTP_FLAGS {}
impl ::core::clone::Clone for FTP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FTP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FTP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FTP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GOPHER_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_ASK: GOPHER_TYPE = GOPHER_TYPE(1073741824u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_BINARY: GOPHER_TYPE = GOPHER_TYPE(512u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_BITMAP: GOPHER_TYPE = GOPHER_TYPE(16384u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_CALENDAR: GOPHER_TYPE = GOPHER_TYPE(524288u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_CSO: GOPHER_TYPE = GOPHER_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_DIRECTORY: GOPHER_TYPE = GOPHER_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_DOS_ARCHIVE: GOPHER_TYPE = GOPHER_TYPE(32u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_ERROR: GOPHER_TYPE = GOPHER_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_GIF: GOPHER_TYPE = GOPHER_TYPE(4096u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_GOPHER_PLUS: GOPHER_TYPE = GOPHER_TYPE(2147483648u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_HTML: GOPHER_TYPE = GOPHER_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_IMAGE: GOPHER_TYPE = GOPHER_TYPE(8192u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_INDEX_SERVER: GOPHER_TYPE = GOPHER_TYPE(128u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_INLINE: GOPHER_TYPE = GOPHER_TYPE(1048576u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_MAC_BINHEX: GOPHER_TYPE = GOPHER_TYPE(16u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_MOVIE: GOPHER_TYPE = GOPHER_TYPE(32768u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_PDF: GOPHER_TYPE = GOPHER_TYPE(262144u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_REDUNDANT: GOPHER_TYPE = GOPHER_TYPE(1024u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_SOUND: GOPHER_TYPE = GOPHER_TYPE(65536u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_TELNET: GOPHER_TYPE = GOPHER_TYPE(256u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_TEXT_FILE: GOPHER_TYPE = GOPHER_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_TN3270: GOPHER_TYPE = GOPHER_TYPE(2048u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_UNIX_UUENCODED: GOPHER_TYPE = GOPHER_TYPE(64u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const GOPHER_TYPE_UNKNOWN: GOPHER_TYPE = GOPHER_TYPE(536870912u32);
impl ::core::marker::Copy for GOPHER_TYPE {}
impl ::core::clone::Clone for GOPHER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GOPHER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GOPHER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GOPHER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GOPHER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_ADDREQ_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_ADD: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(536870912u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_ADD_IF_NEW: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(268435456u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_COALESCE: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(1073741824u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(1073741824u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(16777216u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_ADDREQ_FLAG_REPLACE: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(2147483648u32);
impl ::core::marker::Copy for HTTP_ADDREQ_FLAG {}
impl ::core::clone::Clone for HTTP_ADDREQ_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_ADDREQ_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_ADDREQ_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_ADDREQ_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_ADDREQ_FLAG").field(&self.0).finish()
    }
}
impl HTTP_ADDREQ_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HTTP_ADDREQ_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HTTP_ADDREQ_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_POLICY_EXTENSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const POLICY_EXTENSION_TYPE_NONE: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const POLICY_EXTENSION_TYPE_WINHTTP: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const POLICY_EXTENSION_TYPE_WININET: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(2i32);
impl ::core::marker::Copy for HTTP_POLICY_EXTENSION_TYPE {}
impl ::core::clone::Clone for HTTP_POLICY_EXTENSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_POLICY_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_POLICY_EXTENSION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_POLICY_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_POLICY_EXTENSION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_POLICY_EXTENSION_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const POLICY_EXTENSION_VERSION1: HTTP_POLICY_EXTENSION_VERSION = HTTP_POLICY_EXTENSION_VERSION(1i32);
impl ::core::marker::Copy for HTTP_POLICY_EXTENSION_VERSION {}
impl ::core::clone::Clone for HTTP_POLICY_EXTENSION_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_POLICY_EXTENSION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_POLICY_EXTENSION_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_POLICY_EXTENSION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_POLICY_EXTENSION_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_PUSH_WAIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HttpPushWaitEnableComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HttpPushWaitReceiveComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HttpPushWaitSendComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(2i32);
impl ::core::marker::Copy for HTTP_PUSH_WAIT_TYPE {}
impl ::core::clone::Clone for HTTP_PUSH_WAIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_PUSH_WAIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_PUSH_WAIT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_PUSH_WAIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PUSH_WAIT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_WEB_SOCKET_BUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_BINARY_MESSAGE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_BINARY_FRAGMENT_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_UTF8_MESSAGE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_UTF8_FRAGMENT_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_CLOSE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_PING_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(5i32);
impl ::core::marker::Copy for HTTP_WEB_SOCKET_BUFFER_TYPE {}
impl ::core::clone::Clone for HTTP_WEB_SOCKET_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_WEB_SOCKET_BUFFER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_WEB_SOCKET_CLOSE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1000i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1001i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1002i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1003i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1005i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1006i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1007i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1008i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1009i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1010i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1011i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1015i32);
impl ::core::marker::Copy for HTTP_WEB_SOCKET_CLOSE_STATUS {}
impl ::core::clone::Clone for HTTP_WEB_SOCKET_CLOSE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_CLOSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_WEB_SOCKET_CLOSE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_WEB_SOCKET_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_SEND_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_RECEIVE_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_CLOSE_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HTTP_WEB_SOCKET_SHUTDOWN_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(3i32);
impl ::core::marker::Copy for HTTP_WEB_SOCKET_OPERATION {}
impl ::core::clone::Clone for HTTP_WEB_SOCKET_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HTTP_WEB_SOCKET_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_WEB_SOCKET_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_ACCESS_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPEN_TYPE_DIRECT: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPEN_TYPE_PRECONFIG: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_OPEN_TYPE_PROXY: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(3u32);
impl ::core::marker::Copy for INTERNET_ACCESS_TYPE {}
impl ::core::clone::Clone for INTERNET_ACCESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_ACCESS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_AUTODIAL(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTODIAL_FAILIFSECURITYCHECK: INTERNET_AUTODIAL = INTERNET_AUTODIAL(4u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTODIAL_FORCE_ONLINE: INTERNET_AUTODIAL = INTERNET_AUTODIAL(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTODIAL_FORCE_UNATTENDED: INTERNET_AUTODIAL = INTERNET_AUTODIAL(2u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT: INTERNET_AUTODIAL = INTERNET_AUTODIAL(8u32);
impl ::core::marker::Copy for INTERNET_AUTODIAL {}
impl ::core::clone::Clone for INTERNET_AUTODIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_AUTODIAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_AUTODIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_AUTODIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_AUTODIAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_CONNECTION(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CONNECTION_CONFIGURED: INTERNET_CONNECTION = INTERNET_CONNECTION(64u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CONNECTION_LAN: INTERNET_CONNECTION = INTERNET_CONNECTION(2u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CONNECTION_MODEM: INTERNET_CONNECTION = INTERNET_CONNECTION(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CONNECTION_MODEM_BUSY: INTERNET_CONNECTION = INTERNET_CONNECTION(8u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CONNECTION_OFFLINE: INTERNET_CONNECTION = INTERNET_CONNECTION(32u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_CONNECTION_PROXY: INTERNET_CONNECTION = INTERNET_CONNECTION(4u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_RAS_INSTALLED: INTERNET_CONNECTION = INTERNET_CONNECTION(16u32);
impl ::core::marker::Copy for INTERNET_CONNECTION {}
impl ::core::clone::Clone for INTERNET_CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_CONNECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_CONNECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_CONNECTION").field(&self.0).finish()
    }
}
impl INTERNET_CONNECTION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for INTERNET_CONNECTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_CONNECTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_CONNECTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_CONNECTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INTERNET_CONNECTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_COOKIE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_HTTPONLY: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_COOKIE_THIRD_PARTY: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_FLAG_RESTRICTED_ZONE: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(131072u32);
impl ::core::marker::Copy for INTERNET_COOKIE_FLAGS {}
impl ::core::clone::Clone for INTERNET_COOKIE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_COOKIE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_COOKIE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_COOKIE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_COOKIE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_PER_CONN(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_AUTOCONFIG_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(4u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_AUTODISCOVERY_FLAGS: INTERNET_PER_CONN = INTERNET_PER_CONN(5u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_FLAGS: INTERNET_PER_CONN = INTERNET_PER_CONN(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_PROXY_BYPASS: INTERNET_PER_CONN = INTERNET_PER_CONN(3u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_PROXY_SERVER: INTERNET_PER_CONN = INTERNET_PER_CONN(2u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_AUTOCONFIG_SECONDARY_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(6u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_AUTOCONFIG_RELOAD_DELAY_MINS: INTERNET_PER_CONN = INTERNET_PER_CONN(7u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_TIME: INTERNET_PER_CONN = INTERNET_PER_CONN(8u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(9u32);
impl ::core::marker::Copy for INTERNET_PER_CONN {}
impl ::core::clone::Clone for INTERNET_PER_CONN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_PER_CONN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_PER_CONN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_PER_CONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_PER_CONN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_SCHEME(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_PARTIAL: INTERNET_SCHEME = INTERNET_SCHEME(-2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_UNKNOWN: INTERNET_SCHEME = INTERNET_SCHEME(-1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_DEFAULT: INTERNET_SCHEME = INTERNET_SCHEME(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_FTP: INTERNET_SCHEME = INTERNET_SCHEME(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_GOPHER: INTERNET_SCHEME = INTERNET_SCHEME(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_HTTP: INTERNET_SCHEME = INTERNET_SCHEME(3i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_HTTPS: INTERNET_SCHEME = INTERNET_SCHEME(4i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_FILE: INTERNET_SCHEME = INTERNET_SCHEME(5i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_NEWS: INTERNET_SCHEME = INTERNET_SCHEME(6i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_MAILTO: INTERNET_SCHEME = INTERNET_SCHEME(7i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_SOCKS: INTERNET_SCHEME = INTERNET_SCHEME(8i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_JAVASCRIPT: INTERNET_SCHEME = INTERNET_SCHEME(9i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_VBSCRIPT: INTERNET_SCHEME = INTERNET_SCHEME(10i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_RES: INTERNET_SCHEME = INTERNET_SCHEME(11i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_FIRST: INTERNET_SCHEME = INTERNET_SCHEME(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_SCHEME_LAST: INTERNET_SCHEME = INTERNET_SCHEME(11i32);
impl ::core::marker::Copy for INTERNET_SCHEME {}
impl ::core::clone::Clone for INTERNET_SCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_SCHEME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_SCHEME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNET_STATE(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATE_CONNECTED: INTERNET_STATE = INTERNET_STATE(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATE_DISCONNECTED: INTERNET_STATE = INTERNET_STATE(2u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATE_DISCONNECTED_BY_USER: INTERNET_STATE = INTERNET_STATE(16u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATE_IDLE: INTERNET_STATE = INTERNET_STATE(256u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const INTERNET_STATE_BUSY: INTERNET_STATE = INTERNET_STATE(512u32);
impl ::core::marker::Copy for INTERNET_STATE {}
impl ::core::clone::Clone for INTERNET_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNET_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InternetCookieState(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_UNKNOWN: InternetCookieState = InternetCookieState(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_ACCEPT: InternetCookieState = InternetCookieState(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_PROMPT: InternetCookieState = InternetCookieState(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_LEASH: InternetCookieState = InternetCookieState(3i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_DOWNGRADE: InternetCookieState = InternetCookieState(4i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_REJECT: InternetCookieState = InternetCookieState(5i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const COOKIE_STATE_MAX: InternetCookieState = InternetCookieState(5i32);
impl ::core::marker::Copy for InternetCookieState {}
impl ::core::clone::Clone for InternetCookieState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InternetCookieState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for InternetCookieState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for InternetCookieState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InternetCookieState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROXY_AUTO_DETECT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PROXY_AUTO_DETECT_TYPE_DHCP: PROXY_AUTO_DETECT_TYPE = PROXY_AUTO_DETECT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const PROXY_AUTO_DETECT_TYPE_DNS_A: PROXY_AUTO_DETECT_TYPE = PROXY_AUTO_DETECT_TYPE(2u32);
impl ::core::marker::Copy for PROXY_AUTO_DETECT_TYPE {}
impl ::core::clone::Clone for PROXY_AUTO_DETECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROXY_AUTO_DETECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROXY_AUTO_DETECT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROXY_AUTO_DETECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXY_AUTO_DETECT_TYPE").field(&self.0).finish()
    }
}
impl PROXY_AUTO_DETECT_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REQUEST_TIMES(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const NameResolutionStart: REQUEST_TIMES = REQUEST_TIMES(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const NameResolutionEnd: REQUEST_TIMES = REQUEST_TIMES(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ConnectionEstablishmentStart: REQUEST_TIMES = REQUEST_TIMES(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const ConnectionEstablishmentEnd: REQUEST_TIMES = REQUEST_TIMES(3i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const TLSHandshakeStart: REQUEST_TIMES = REQUEST_TIMES(4i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const TLSHandshakeEnd: REQUEST_TIMES = REQUEST_TIMES(5i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const HttpRequestTimeMax: REQUEST_TIMES = REQUEST_TIMES(32i32);
impl ::core::marker::Copy for REQUEST_TIMES {}
impl ::core::clone::Clone for REQUEST_TIMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REQUEST_TIMES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REQUEST_TIMES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REQUEST_TIMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REQUEST_TIMES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct URL_CACHE_LIMIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const UrlCacheLimitTypeIE: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const UrlCacheLimitTypeIETotal: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const UrlCacheLimitTypeAppContainer: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const UrlCacheLimitTypeAppContainerTotal: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const UrlCacheLimitTypeNum: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(4i32);
impl ::core::marker::Copy for URL_CACHE_LIMIT_TYPE {}
impl ::core::clone::Clone for URL_CACHE_LIMIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URL_CACHE_LIMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for URL_CACHE_LIMIT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for URL_CACHE_LIMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URL_CACHE_LIMIT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WININET_SYNC_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_SYNC_MODE_NEVER: WININET_SYNC_MODE = WININET_SYNC_MODE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_SYNC_MODE_ON_EXPIRY: WININET_SYNC_MODE = WININET_SYNC_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_SYNC_MODE_ONCE_PER_SESSION: WININET_SYNC_MODE = WININET_SYNC_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_SYNC_MODE_ALWAYS: WININET_SYNC_MODE = WININET_SYNC_MODE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_SYNC_MODE_AUTOMATIC: WININET_SYNC_MODE = WININET_SYNC_MODE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WININET_SYNC_MODE_DEFAULT: WININET_SYNC_MODE = WININET_SYNC_MODE(4i32);
impl ::core::marker::Copy for WININET_SYNC_MODE {}
impl ::core::clone::Clone for WININET_SYNC_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WININET_SYNC_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WININET_SYNC_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WININET_SYNC_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WININET_SYNC_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPAD_CACHE_DELETE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WPAD_CACHE_DELETE_CURRENT: WPAD_CACHE_DELETE = WPAD_CACHE_DELETE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub const WPAD_CACHE_DELETE_ALL: WPAD_CACHE_DELETE = WPAD_CACHE_DELETE(1i32);
impl ::core::marker::Copy for WPAD_CACHE_DELETE {}
impl ::core::clone::Clone for WPAD_CACHE_DELETE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPAD_CACHE_DELETE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPAD_CACHE_DELETE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPAD_CACHE_DELETE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPAD_CACHE_DELETE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct APP_CACHE_DOWNLOAD_ENTRY {
    pub pwszUrl: ::windows::core::PWSTR,
    pub dwEntryType: u32,
}
impl ::core::marker::Copy for APP_CACHE_DOWNLOAD_ENTRY {}
impl ::core::clone::Clone for APP_CACHE_DOWNLOAD_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APP_CACHE_DOWNLOAD_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_DOWNLOAD_ENTRY").field("pwszUrl", &self.pwszUrl).field("dwEntryType", &self.dwEntryType).finish()
    }
}
impl ::windows::core::TypeKind for APP_CACHE_DOWNLOAD_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APP_CACHE_DOWNLOAD_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszUrl == other.pwszUrl && self.dwEntryType == other.dwEntryType
    }
}
impl ::core::cmp::Eq for APP_CACHE_DOWNLOAD_ENTRY {}
impl ::core::default::Default for APP_CACHE_DOWNLOAD_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct APP_CACHE_DOWNLOAD_LIST {
    pub dwEntryCount: u32,
    pub pEntries: *mut APP_CACHE_DOWNLOAD_ENTRY,
}
impl ::core::marker::Copy for APP_CACHE_DOWNLOAD_LIST {}
impl ::core::clone::Clone for APP_CACHE_DOWNLOAD_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APP_CACHE_DOWNLOAD_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_DOWNLOAD_LIST").field("dwEntryCount", &self.dwEntryCount).field("pEntries", &self.pEntries).finish()
    }
}
impl ::windows::core::TypeKind for APP_CACHE_DOWNLOAD_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APP_CACHE_DOWNLOAD_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwEntryCount == other.dwEntryCount && self.pEntries == other.pEntries
    }
}
impl ::core::cmp::Eq for APP_CACHE_DOWNLOAD_LIST {}
impl ::core::default::Default for APP_CACHE_DOWNLOAD_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct APP_CACHE_GROUP_INFO {
    pub pwszManifestUrl: ::windows::core::PWSTR,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ullSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for APP_CACHE_GROUP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for APP_CACHE_GROUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_GROUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_GROUP_INFO").field("pwszManifestUrl", &self.pwszManifestUrl).field("ftLastAccessTime", &self.ftLastAccessTime).field("ullSize", &self.ullSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for APP_CACHE_GROUP_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszManifestUrl == other.pwszManifestUrl && self.ftLastAccessTime == other.ftLastAccessTime && self.ullSize == other.ullSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_GROUP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct APP_CACHE_GROUP_LIST {
    pub dwAppCacheGroupCount: u32,
    pub pAppCacheGroups: *mut APP_CACHE_GROUP_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for APP_CACHE_GROUP_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for APP_CACHE_GROUP_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_GROUP_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_GROUP_LIST").field("dwAppCacheGroupCount", &self.dwAppCacheGroupCount).field("pAppCacheGroups", &self.pAppCacheGroups).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for APP_CACHE_GROUP_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_GROUP_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwAppCacheGroupCount == other.dwAppCacheGroupCount && self.pAppCacheGroups == other.pAppCacheGroups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_GROUP_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_GROUP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct AUTO_PROXY_SCRIPT_BUFFER {
    pub dwStructSize: u32,
    pub lpszScriptBuffer: ::windows::core::PSTR,
    pub dwScriptBufferSize: u32,
}
impl ::core::marker::Copy for AUTO_PROXY_SCRIPT_BUFFER {}
impl ::core::clone::Clone for AUTO_PROXY_SCRIPT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTO_PROXY_SCRIPT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTO_PROXY_SCRIPT_BUFFER").field("dwStructSize", &self.dwStructSize).field("lpszScriptBuffer", &self.lpszScriptBuffer).field("dwScriptBufferSize", &self.dwScriptBufferSize).finish()
    }
}
impl ::windows::core::TypeKind for AUTO_PROXY_SCRIPT_BUFFER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTO_PROXY_SCRIPT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScriptBuffer == other.lpszScriptBuffer && self.dwScriptBufferSize == other.dwScriptBufferSize
    }
}
impl ::core::cmp::Eq for AUTO_PROXY_SCRIPT_BUFFER {}
impl ::core::default::Default for AUTO_PROXY_SCRIPT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct AutoProxyHelperFunctions {
    pub lpVtbl: *const AutoProxyHelperVtbl,
}
impl ::core::marker::Copy for AutoProxyHelperFunctions {}
impl ::core::clone::Clone for AutoProxyHelperFunctions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AutoProxyHelperFunctions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutoProxyHelperFunctions").field("lpVtbl", &self.lpVtbl).finish()
    }
}
impl ::windows::core::TypeKind for AutoProxyHelperFunctions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AutoProxyHelperFunctions {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl
    }
}
impl ::core::cmp::Eq for AutoProxyHelperFunctions {}
impl ::core::default::Default for AutoProxyHelperFunctions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct AutoProxyHelperVtbl {
    pub IsResolvable: isize,
    pub GetIPAddress: isize,
    pub ResolveHostName: isize,
    pub IsInNet: isize,
    pub IsResolvableEx: isize,
    pub GetIPAddressEx: isize,
    pub ResolveHostNameEx: isize,
    pub IsInNetEx: isize,
    pub SortIpList: isize,
}
impl ::core::marker::Copy for AutoProxyHelperVtbl {}
impl ::core::clone::Clone for AutoProxyHelperVtbl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AutoProxyHelperVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutoProxyHelperVtbl").field("IsResolvable", &self.IsResolvable).field("GetIPAddress", &self.GetIPAddress).field("ResolveHostName", &self.ResolveHostName).field("IsInNet", &self.IsInNet).field("IsResolvableEx", &self.IsResolvableEx).field("GetIPAddressEx", &self.GetIPAddressEx).field("ResolveHostNameEx", &self.ResolveHostNameEx).field("IsInNetEx", &self.IsInNetEx).field("SortIpList", &self.SortIpList).finish()
    }
}
impl ::windows::core::TypeKind for AutoProxyHelperVtbl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AutoProxyHelperVtbl {
    fn eq(&self, other: &Self) -> bool {
        self.IsResolvable == other.IsResolvable && self.GetIPAddress == other.GetIPAddress && self.ResolveHostName == other.ResolveHostName && self.IsInNet == other.IsInNet && self.IsResolvableEx == other.IsResolvableEx && self.GetIPAddressEx == other.GetIPAddressEx && self.ResolveHostNameEx == other.ResolveHostNameEx && self.IsInNetEx == other.IsInNetEx && self.SortIpList == other.SortIpList
    }
}
impl ::core::cmp::Eq for AutoProxyHelperVtbl {}
impl ::core::default::Default for AutoProxyHelperVtbl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COOKIE_DLG_INFO {
    pub pszServer: ::windows::core::PWSTR,
    pub pic: *mut INTERNET_COOKIE,
    pub dwStopWarning: u32,
    pub cx: i32,
    pub cy: i32,
    pub pszHeader: ::windows::core::PWSTR,
    pub dwOperation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COOKIE_DLG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COOKIE_DLG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COOKIE_DLG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COOKIE_DLG_INFO").field("pszServer", &self.pszServer).field("pic", &self.pic).field("dwStopWarning", &self.dwStopWarning).field("cx", &self.cx).field("cy", &self.cy).field("pszHeader", &self.pszHeader).field("dwOperation", &self.dwOperation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COOKIE_DLG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COOKIE_DLG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszServer == other.pszServer && self.pic == other.pic && self.dwStopWarning == other.dwStopWarning && self.cx == other.cx && self.cy == other.cy && self.pszHeader == other.pszHeader && self.dwOperation == other.dwOperation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COOKIE_DLG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COOKIE_DLG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CookieDecision {
    pub dwCookieState: u32,
    pub fAllowSession: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CookieDecision {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CookieDecision {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CookieDecision {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CookieDecision").field("dwCookieState", &self.dwCookieState).field("fAllowSession", &self.fAllowSession).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CookieDecision {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CookieDecision {
    fn eq(&self, other: &Self) -> bool {
        self.dwCookieState == other.dwCookieState && self.fAllowSession == other.fAllowSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CookieDecision {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CookieDecision {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    pub ShortAbstract: *mut i8,
    pub AbstractFile: *mut i8,
}
impl ::core::marker::Copy for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ABSTRACT_ATTRIBUTE_TYPE").field("ShortAbstract", &self.ShortAbstract).field("AbstractFile", &self.AbstractFile).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ShortAbstract == other.ShortAbstract && self.AbstractFile == other.AbstractFile
    }
}
impl ::core::cmp::Eq for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_ADMIN_ATTRIBUTE_TYPE {
    pub Comment: *mut i8,
    pub EmailAddress: *mut i8,
}
impl ::core::marker::Copy for GOPHER_ADMIN_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ADMIN_ATTRIBUTE_TYPE").field("Comment", &self.Comment).field("EmailAddress", &self.EmailAddress).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Comment == other.Comment && self.EmailAddress == other.EmailAddress
    }
}
impl ::core::cmp::Eq for GOPHER_ADMIN_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_ASK_ATTRIBUTE_TYPE {
    pub QuestionType: *mut i8,
    pub QuestionText: *mut i8,
}
impl ::core::marker::Copy for GOPHER_ASK_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ASK_ATTRIBUTE_TYPE").field("QuestionType", &self.QuestionType).field("QuestionText", &self.QuestionText).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_ASK_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.QuestionType == other.QuestionType && self.QuestionText == other.QuestionText
    }
}
impl ::core::cmp::Eq for GOPHER_ASK_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_ATTRIBUTE_TYPE {
    pub CategoryId: u32,
    pub AttributeId: u32,
    pub AttributeType: GOPHER_ATTRIBUTE_TYPE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GOPHER_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GOPHER_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GOPHER_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union GOPHER_ATTRIBUTE_TYPE_0 {
    pub Admin: GOPHER_ADMIN_ATTRIBUTE_TYPE,
    pub ModDate: GOPHER_MOD_DATE_ATTRIBUTE_TYPE,
    pub Ttl: GOPHER_TTL_ATTRIBUTE_TYPE,
    pub Score: GOPHER_SCORE_ATTRIBUTE_TYPE,
    pub ScoreRange: GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE,
    pub Site: GOPHER_SITE_ATTRIBUTE_TYPE,
    pub Organization: GOPHER_ORGANIZATION_ATTRIBUTE_TYPE,
    pub Location: GOPHER_LOCATION_ATTRIBUTE_TYPE,
    pub GeographicalLocation: GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE,
    pub TimeZone: GOPHER_TIMEZONE_ATTRIBUTE_TYPE,
    pub Provider: GOPHER_PROVIDER_ATTRIBUTE_TYPE,
    pub Version: GOPHER_VERSION_ATTRIBUTE_TYPE,
    pub Abstract: GOPHER_ABSTRACT_ATTRIBUTE_TYPE,
    pub View: GOPHER_VIEW_ATTRIBUTE_TYPE,
    pub Veronica: GOPHER_VERONICA_ATTRIBUTE_TYPE,
    pub Ask: GOPHER_ASK_ATTRIBUTE_TYPE,
    pub Unknown: GOPHER_UNKNOWN_ATTRIBUTE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GOPHER_ATTRIBUTE_TYPE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GOPHER_ATTRIBUTE_TYPE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GOPHER_ATTRIBUTE_TYPE_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_ATTRIBUTE_TYPE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_FIND_DATAA {
    pub DisplayString: [u8; 129],
    pub GopherType: GOPHER_TYPE,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::super::Foundation::FILETIME,
    pub Locator: [u8; 654],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GOPHER_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GOPHER_FIND_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_FIND_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_FIND_DATAA").field("DisplayString", &self.DisplayString).field("GopherType", &self.GopherType).field("SizeLow", &self.SizeLow).field("SizeHigh", &self.SizeHigh).field("LastModificationTime", &self.LastModificationTime).field("Locator", &self.Locator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GOPHER_FIND_DATAA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayString == other.DisplayString && self.GopherType == other.GopherType && self.SizeLow == other.SizeLow && self.SizeHigh == other.SizeHigh && self.LastModificationTime == other.LastModificationTime && self.Locator == other.Locator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_FIND_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_FIND_DATAW {
    pub DisplayString: [u16; 129],
    pub GopherType: GOPHER_TYPE,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::super::Foundation::FILETIME,
    pub Locator: [u16; 654],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GOPHER_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GOPHER_FIND_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_FIND_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_FIND_DATAW").field("DisplayString", &self.DisplayString).field("GopherType", &self.GopherType).field("SizeLow", &self.SizeLow).field("SizeHigh", &self.SizeHigh).field("LastModificationTime", &self.LastModificationTime).field("Locator", &self.Locator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GOPHER_FIND_DATAW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayString == other.DisplayString && self.GopherType == other.GopherType && self.SizeLow == other.SizeLow && self.SizeHigh == other.SizeHigh && self.LastModificationTime == other.LastModificationTime && self.Locator == other.Locator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    pub DegreesNorth: i32,
    pub MinutesNorth: i32,
    pub SecondsNorth: i32,
    pub DegreesEast: i32,
    pub MinutesEast: i32,
    pub SecondsEast: i32,
}
impl ::core::marker::Copy for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE").field("DegreesNorth", &self.DegreesNorth).field("MinutesNorth", &self.MinutesNorth).field("SecondsNorth", &self.SecondsNorth).field("DegreesEast", &self.DegreesEast).field("MinutesEast", &self.MinutesEast).field("SecondsEast", &self.SecondsEast).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DegreesNorth == other.DegreesNorth && self.MinutesNorth == other.MinutesNorth && self.SecondsNorth == other.SecondsNorth && self.DegreesEast == other.DegreesEast && self.MinutesEast == other.MinutesEast && self.SecondsEast == other.SecondsEast
    }
}
impl ::core::cmp::Eq for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_LOCATION_ATTRIBUTE_TYPE {
    pub Location: *mut i8,
}
impl ::core::marker::Copy for GOPHER_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_LOCATION_ATTRIBUTE_TYPE").field("Location", &self.Location).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
    }
}
impl ::core::cmp::Eq for GOPHER_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    pub DateAndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_MOD_DATE_ATTRIBUTE_TYPE").field("DateAndTime", &self.DateAndTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DateAndTime == other.DateAndTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    pub Organization: *mut i8,
}
impl ::core::marker::Copy for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ORGANIZATION_ATTRIBUTE_TYPE").field("Organization", &self.Organization).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Organization == other.Organization
    }
}
impl ::core::cmp::Eq for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    pub Provider: *mut i8,
}
impl ::core::marker::Copy for GOPHER_PROVIDER_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_PROVIDER_ATTRIBUTE_TYPE").field("Provider", &self.Provider).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Provider == other.Provider
    }
}
impl ::core::cmp::Eq for GOPHER_PROVIDER_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_SCORE_ATTRIBUTE_TYPE {
    pub Score: i32,
}
impl ::core::marker::Copy for GOPHER_SCORE_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_SCORE_ATTRIBUTE_TYPE").field("Score", &self.Score).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_SCORE_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Score == other.Score
    }
}
impl ::core::cmp::Eq for GOPHER_SCORE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    pub LowerBound: i32,
    pub UpperBound: i32,
}
impl ::core::marker::Copy for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE").field("LowerBound", &self.LowerBound).field("UpperBound", &self.UpperBound).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.LowerBound == other.LowerBound && self.UpperBound == other.UpperBound
    }
}
impl ::core::cmp::Eq for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_SITE_ATTRIBUTE_TYPE {
    pub Site: *mut i8,
}
impl ::core::marker::Copy for GOPHER_SITE_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_SITE_ATTRIBUTE_TYPE").field("Site", &self.Site).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_SITE_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Site == other.Site
    }
}
impl ::core::cmp::Eq for GOPHER_SITE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    pub Zone: i32,
}
impl ::core::marker::Copy for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_TIMEZONE_ATTRIBUTE_TYPE").field("Zone", &self.Zone).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Zone == other.Zone
    }
}
impl ::core::cmp::Eq for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_TTL_ATTRIBUTE_TYPE {
    pub Ttl: u32,
}
impl ::core::marker::Copy for GOPHER_TTL_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_TTL_ATTRIBUTE_TYPE").field("Ttl", &self.Ttl).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_TTL_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Ttl == other.Ttl
    }
}
impl ::core::cmp::Eq for GOPHER_TTL_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    pub Text: *mut i8,
}
impl ::core::marker::Copy for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_UNKNOWN_ATTRIBUTE_TYPE").field("Text", &self.Text).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
impl ::core::cmp::Eq for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_VERONICA_ATTRIBUTE_TYPE {
    pub TreeWalk: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GOPHER_VERONICA_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_VERONICA_ATTRIBUTE_TYPE").field("TreeWalk", &self.TreeWalk).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TreeWalk == other.TreeWalk
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_VERONICA_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_VERSION_ATTRIBUTE_TYPE {
    pub Version: *mut i8,
}
impl ::core::marker::Copy for GOPHER_VERSION_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_VERSION_ATTRIBUTE_TYPE").field("Version", &self.Version).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_VERSION_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
    }
}
impl ::core::cmp::Eq for GOPHER_VERSION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct GOPHER_VIEW_ATTRIBUTE_TYPE {
    pub ContentType: *mut i8,
    pub Language: *mut i8,
    pub Size: u32,
}
impl ::core::marker::Copy for GOPHER_VIEW_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_VIEW_ATTRIBUTE_TYPE").field("ContentType", &self.ContentType).field("Language", &self.Language).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for GOPHER_VIEW_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ContentType == other.ContentType && self.Language == other.Language && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for GOPHER_VIEW_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_PUSH_NOTIFICATION_STATUS {
    pub ChannelStatusValid: super::super::Foundation::BOOL,
    pub ChannelStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_PUSH_NOTIFICATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_PUSH_NOTIFICATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_PUSH_NOTIFICATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PUSH_NOTIFICATION_STATUS").field("ChannelStatusValid", &self.ChannelStatusValid).field("ChannelStatus", &self.ChannelStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HTTP_PUSH_NOTIFICATION_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_PUSH_NOTIFICATION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelStatusValid == other.ChannelStatusValid && self.ChannelStatus == other.ChannelStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_PUSH_NOTIFICATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_PUSH_NOTIFICATION_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct HTTP_PUSH_TRANSPORT_SETTING {
    pub TransportSettingId: ::windows::core::GUID,
    pub BrokerEventId: ::windows::core::GUID,
}
impl ::core::marker::Copy for HTTP_PUSH_TRANSPORT_SETTING {}
impl ::core::clone::Clone for HTTP_PUSH_TRANSPORT_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_PUSH_TRANSPORT_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PUSH_TRANSPORT_SETTING").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventId", &self.BrokerEventId).finish()
    }
}
impl ::windows::core::TypeKind for HTTP_PUSH_TRANSPORT_SETTING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_PUSH_TRANSPORT_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventId == other.BrokerEventId
    }
}
impl ::core::cmp::Eq for HTTP_PUSH_TRANSPORT_SETTING {}
impl ::core::default::Default for HTTP_PUSH_TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTTP_PUSH_WAIT_HANDLE(pub isize);
impl HTTP_PUSH_WAIT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HTTP_PUSH_WAIT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTTP_PUSH_WAIT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTTP_PUSH_WAIT_HANDLE {}
impl ::core::fmt::Debug for HTTP_PUSH_WAIT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PUSH_WAIT_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HTTP_PUSH_WAIT_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct HTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgTimes: [u64; 32],
}
impl ::core::marker::Copy for HTTP_REQUEST_TIMES {}
impl ::core::clone::Clone for HTTP_REQUEST_TIMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_TIMES").field("cTimes", &self.cTimes).field("rgTimes", &self.rgTimes).finish()
    }
}
impl ::windows::core::TypeKind for HTTP_REQUEST_TIMES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TIMES {
    fn eq(&self, other: &Self) -> bool {
        self.cTimes == other.cTimes && self.rgTimes == other.rgTimes
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TIMES {}
impl ::core::default::Default for HTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct HTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: INTERNET_ASYNC_RESULT,
    pub Operation: HTTP_WEB_SOCKET_OPERATION,
    pub BufferType: HTTP_WEB_SOCKET_BUFFER_TYPE,
    pub dwBytesTransferred: u32,
}
impl ::core::marker::Copy for HTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::clone::Clone for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_WEB_SOCKET_ASYNC_RESULT").field("AsyncResult", &self.AsyncResult).field("Operation", &self.Operation).field("BufferType", &self.BufferType).field("dwBytesTransferred", &self.dwBytesTransferred).finish()
    }
}
impl ::windows::core::TypeKind for HTTP_WEB_SOCKET_ASYNC_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.AsyncResult == other.AsyncResult && self.Operation == other.Operation && self.BufferType == other.BufferType && self.dwBytesTransferred == other.dwBytesTransferred
    }
}
impl ::core::cmp::Eq for HTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::default::Default for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
impl ::core::marker::Copy for INTERNET_ASYNC_RESULT {}
impl ::core::clone::Clone for INTERNET_ASYNC_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_ASYNC_RESULT").field("dwResult", &self.dwResult).field("dwError", &self.dwError).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_ASYNC_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwResult == other.dwResult && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for INTERNET_ASYNC_RESULT {}
impl ::core::default::Default for INTERNET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_AUTH_NOTIFY_DATA {
    pub cbStruct: u32,
    pub dwOptions: u32,
    pub pfnNotify: PFN_AUTH_NOTIFY,
    pub dwContext: usize,
}
impl ::core::marker::Copy for INTERNET_AUTH_NOTIFY_DATA {}
impl ::core::clone::Clone for INTERNET_AUTH_NOTIFY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_AUTH_NOTIFY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_AUTH_NOTIFY_DATA").field("cbStruct", &self.cbStruct).field("dwOptions", &self.dwOptions).field("dwContext", &self.dwContext).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_AUTH_NOTIFY_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for INTERNET_AUTH_NOTIFY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_BUFFERSA {
    pub dwStructSize: u32,
    pub Next: *mut INTERNET_BUFFERSA,
    pub lpcszHeader: ::windows::core::PCSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut ::core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
impl ::core::marker::Copy for INTERNET_BUFFERSA {}
impl ::core::clone::Clone for INTERNET_BUFFERSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_BUFFERSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_BUFFERSA").field("dwStructSize", &self.dwStructSize).field("Next", &self.Next).field("lpcszHeader", &self.lpcszHeader).field("dwHeadersLength", &self.dwHeadersLength).field("dwHeadersTotal", &self.dwHeadersTotal).field("lpvBuffer", &self.lpvBuffer).field("dwBufferLength", &self.dwBufferLength).field("dwBufferTotal", &self.dwBufferTotal).field("dwOffsetLow", &self.dwOffsetLow).field("dwOffsetHigh", &self.dwOffsetHigh).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_BUFFERSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_BUFFERSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.Next == other.Next && self.lpcszHeader == other.lpcszHeader && self.dwHeadersLength == other.dwHeadersLength && self.dwHeadersTotal == other.dwHeadersTotal && self.lpvBuffer == other.lpvBuffer && self.dwBufferLength == other.dwBufferLength && self.dwBufferTotal == other.dwBufferTotal && self.dwOffsetLow == other.dwOffsetLow && self.dwOffsetHigh == other.dwOffsetHigh
    }
}
impl ::core::cmp::Eq for INTERNET_BUFFERSA {}
impl ::core::default::Default for INTERNET_BUFFERSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_BUFFERSW {
    pub dwStructSize: u32,
    pub Next: *mut INTERNET_BUFFERSW,
    pub lpcszHeader: ::windows::core::PCWSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut ::core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
impl ::core::marker::Copy for INTERNET_BUFFERSW {}
impl ::core::clone::Clone for INTERNET_BUFFERSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_BUFFERSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_BUFFERSW").field("dwStructSize", &self.dwStructSize).field("Next", &self.Next).field("lpcszHeader", &self.lpcszHeader).field("dwHeadersLength", &self.dwHeadersLength).field("dwHeadersTotal", &self.dwHeadersTotal).field("lpvBuffer", &self.lpvBuffer).field("dwBufferLength", &self.dwBufferLength).field("dwBufferTotal", &self.dwBufferTotal).field("dwOffsetLow", &self.dwOffsetLow).field("dwOffsetHigh", &self.dwOffsetHigh).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_BUFFERSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_BUFFERSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.Next == other.Next && self.lpcszHeader == other.lpcszHeader && self.dwHeadersLength == other.dwHeadersLength && self.dwHeadersTotal == other.dwHeadersTotal && self.lpvBuffer == other.lpvBuffer && self.dwBufferLength == other.dwBufferLength && self.dwBufferTotal == other.dwBufferTotal && self.dwOffsetLow == other.dwOffsetLow && self.dwOffsetHigh == other.dwOffsetHigh
    }
}
impl ::core::cmp::Eq for INTERNET_BUFFERSW {}
impl ::core::default::Default for INTERNET_BUFFERSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOA {
    pub dwStructSize: u32,
    pub dwContainer: u32,
    pub dwQuota: u32,
    pub dwReserved4: u32,
    pub fPerUser: super::super::Foundation::BOOL,
    pub dwSyncMode: u32,
    pub dwNumCachePaths: u32,
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOA_0,
    pub dwNormalUsage: u32,
    pub dwExemptUsage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_CONFIG_INFOA_0 {
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOA_0_0,
    pub CachePaths: [INTERNET_CACHE_CONFIG_PATH_ENTRYA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_INFOA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_INFOA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_INFOA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOA_0_0 {
    pub CachePath: [u8; 260],
    pub dwCacheSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_INFOA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONFIG_INFOA_0_0").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOW {
    pub dwStructSize: u32,
    pub dwContainer: u32,
    pub dwQuota: u32,
    pub dwReserved4: u32,
    pub fPerUser: super::super::Foundation::BOOL,
    pub dwSyncMode: u32,
    pub dwNumCachePaths: u32,
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOW_0,
    pub dwNormalUsage: u32,
    pub dwExemptUsage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_CONFIG_INFOW_0 {
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOW_0_0,
    pub CachePaths: [INTERNET_CACHE_CONFIG_PATH_ENTRYW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_INFOW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_INFOW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_INFOW_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOW_0_0 {
    pub CachePath: [u16; 260],
    pub dwCacheSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_INFOW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONFIG_INFOW_0_0").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    pub CachePath: [u8; 260],
    pub dwCacheSize: u32,
}
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_PATH_ENTRYA {}
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONFIG_PATH_ENTRYA").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_PATH_ENTRYA {}
impl ::core::default::Default for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    pub CachePath: [u16; 260],
    pub dwCacheSize: u32,
}
impl ::core::marker::Copy for INTERNET_CACHE_CONFIG_PATH_ENTRYW {}
impl ::core::clone::Clone for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONFIG_PATH_ENTRYW").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_PATH_ENTRYW {}
impl ::core::default::Default for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CACHE_CONTAINER_INFOA {
    pub dwCacheVersion: u32,
    pub lpszName: ::windows::core::PSTR,
    pub lpszCachePrefix: ::windows::core::PSTR,
    pub lpszVolumeLabel: ::windows::core::PSTR,
    pub lpszVolumeTitle: ::windows::core::PSTR,
}
impl ::core::marker::Copy for INTERNET_CACHE_CONTAINER_INFOA {}
impl ::core::clone::Clone for INTERNET_CACHE_CONTAINER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_CONTAINER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONTAINER_INFOA").field("dwCacheVersion", &self.dwCacheVersion).field("lpszName", &self.lpszName).field("lpszCachePrefix", &self.lpszCachePrefix).field("lpszVolumeLabel", &self.lpszVolumeLabel).field("lpszVolumeTitle", &self.lpszVolumeTitle).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CACHE_CONTAINER_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONTAINER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCacheVersion == other.dwCacheVersion && self.lpszName == other.lpszName && self.lpszCachePrefix == other.lpszCachePrefix && self.lpszVolumeLabel == other.lpszVolumeLabel && self.lpszVolumeTitle == other.lpszVolumeTitle
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONTAINER_INFOA {}
impl ::core::default::Default for INTERNET_CACHE_CONTAINER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CACHE_CONTAINER_INFOW {
    pub dwCacheVersion: u32,
    pub lpszName: ::windows::core::PWSTR,
    pub lpszCachePrefix: ::windows::core::PWSTR,
    pub lpszVolumeLabel: ::windows::core::PWSTR,
    pub lpszVolumeTitle: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for INTERNET_CACHE_CONTAINER_INFOW {}
impl ::core::clone::Clone for INTERNET_CACHE_CONTAINER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_CONTAINER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONTAINER_INFOW").field("dwCacheVersion", &self.dwCacheVersion).field("lpszName", &self.lpszName).field("lpszCachePrefix", &self.lpszCachePrefix).field("lpszVolumeLabel", &self.lpszVolumeLabel).field("lpszVolumeTitle", &self.lpszVolumeTitle).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CACHE_CONTAINER_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONTAINER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCacheVersion == other.dwCacheVersion && self.lpszName == other.lpszName && self.lpszCachePrefix == other.lpszCachePrefix && self.lpszVolumeLabel == other.lpszVolumeLabel && self.lpszVolumeTitle == other.lpszVolumeTitle
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONTAINER_INFOW {}
impl ::core::default::Default for INTERNET_CACHE_CONTAINER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_ENTRY_INFOA {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: ::windows::core::PSTR,
    pub lpszLocalFileName: ::windows::core::PSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub ExpireTime: super::super::Foundation::FILETIME,
    pub LastAccessTime: super::super::Foundation::FILETIME,
    pub LastSyncTime: super::super::Foundation::FILETIME,
    pub lpHeaderInfo: ::windows::core::PSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: ::windows::core::PSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_ENTRY_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_ENTRY_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_ENTRY_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_ENTRY_INFOA_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_ENTRY_INFOA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_ENTRY_INFOA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_ENTRY_INFOA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_ENTRY_INFOW {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: ::windows::core::PWSTR,
    pub lpszLocalFileName: ::windows::core::PWSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub ExpireTime: super::super::Foundation::FILETIME,
    pub LastAccessTime: super::super::Foundation::FILETIME,
    pub LastSyncTime: super::super::Foundation::FILETIME,
    pub lpHeaderInfo: ::windows::core::PWSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: ::windows::core::PWSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_ENTRY_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_ENTRY_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_ENTRY_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_ENTRY_INFOW_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_ENTRY_INFOW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_ENTRY_INFOW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_ENTRY_INFOW_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CACHE_GROUP_INFOA {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [u8; 120],
}
impl ::core::marker::Copy for INTERNET_CACHE_GROUP_INFOA {}
impl ::core::clone::Clone for INTERNET_CACHE_GROUP_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_GROUP_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_GROUP_INFOA").field("dwGroupSize", &self.dwGroupSize).field("dwGroupFlags", &self.dwGroupFlags).field("dwGroupType", &self.dwGroupType).field("dwDiskUsage", &self.dwDiskUsage).field("dwDiskQuota", &self.dwDiskQuota).field("dwOwnerStorage", &self.dwOwnerStorage).field("szGroupName", &self.szGroupName).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CACHE_GROUP_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_GROUP_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupSize == other.dwGroupSize && self.dwGroupFlags == other.dwGroupFlags && self.dwGroupType == other.dwGroupType && self.dwDiskUsage == other.dwDiskUsage && self.dwDiskQuota == other.dwDiskQuota && self.dwOwnerStorage == other.dwOwnerStorage && self.szGroupName == other.szGroupName
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_GROUP_INFOA {}
impl ::core::default::Default for INTERNET_CACHE_GROUP_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CACHE_GROUP_INFOW {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [u16; 120],
}
impl ::core::marker::Copy for INTERNET_CACHE_GROUP_INFOW {}
impl ::core::clone::Clone for INTERNET_CACHE_GROUP_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_GROUP_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_GROUP_INFOW").field("dwGroupSize", &self.dwGroupSize).field("dwGroupFlags", &self.dwGroupFlags).field("dwGroupType", &self.dwGroupType).field("dwDiskUsage", &self.dwDiskUsage).field("dwDiskQuota", &self.dwDiskQuota).field("dwOwnerStorage", &self.dwOwnerStorage).field("szGroupName", &self.szGroupName).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CACHE_GROUP_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_GROUP_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupSize == other.dwGroupSize && self.dwGroupFlags == other.dwGroupFlags && self.dwGroupType == other.dwGroupType && self.dwDiskUsage == other.dwDiskUsage && self.dwDiskQuota == other.dwDiskQuota && self.dwOwnerStorage == other.dwOwnerStorage && self.szGroupName == other.szGroupName
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_GROUP_INFOW {}
impl ::core::default::Default for INTERNET_CACHE_GROUP_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_TIMESTAMPS {
    pub ftExpires: super::super::Foundation::FILETIME,
    pub ftLastModified: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CACHE_TIMESTAMPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CACHE_TIMESTAMPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_TIMESTAMPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_TIMESTAMPS").field("ftExpires", &self.ftExpires).field("ftLastModified", &self.ftLastModified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CACHE_TIMESTAMPS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpires == other.ftExpires && self.ftLastModified == other.ftLastModified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_TIMESTAMPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CALLBACK_COOKIE {
    pub pcwszName: ::windows::core::PCWSTR,
    pub pcwszValue: ::windows::core::PCWSTR,
    pub pcwszDomain: ::windows::core::PCWSTR,
    pub pcwszPath: ::windows::core::PCWSTR,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CALLBACK_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CALLBACK_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CALLBACK_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CALLBACK_COOKIE").field("pcwszName", &self.pcwszName).field("pcwszValue", &self.pcwszValue).field("pcwszDomain", &self.pcwszDomain).field("pcwszPath", &self.pcwszPath).field("ftExpires", &self.ftExpires).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CALLBACK_COOKIE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CALLBACK_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.pcwszName == other.pcwszName && self.pcwszValue == other.pcwszValue && self.pcwszDomain == other.pcwszDomain && self.pcwszPath == other.pcwszPath && self.ftExpires == other.ftExpires && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CALLBACK_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CALLBACK_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CERTIFICATE_INFO {
    pub ftExpiry: super::super::Foundation::FILETIME,
    pub ftStart: super::super::Foundation::FILETIME,
    pub lpszSubjectInfo: *mut i8,
    pub lpszIssuerInfo: *mut i8,
    pub lpszProtocolName: *mut i8,
    pub lpszSignatureAlgName: *mut i8,
    pub lpszEncryptionAlgName: *mut i8,
    pub dwKeySize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CERTIFICATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CERTIFICATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CERTIFICATE_INFO").field("ftExpiry", &self.ftExpiry).field("ftStart", &self.ftStart).field("lpszSubjectInfo", &self.lpszSubjectInfo).field("lpszIssuerInfo", &self.lpszIssuerInfo).field("lpszProtocolName", &self.lpszProtocolName).field("lpszSignatureAlgName", &self.lpszSignatureAlgName).field("lpszEncryptionAlgName", &self.lpszEncryptionAlgName).field("dwKeySize", &self.dwKeySize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CERTIFICATE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CERTIFICATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpiry == other.ftExpiry && self.ftStart == other.ftStart && self.lpszSubjectInfo == other.lpszSubjectInfo && self.lpszIssuerInfo == other.lpszIssuerInfo && self.lpszProtocolName == other.lpszProtocolName && self.lpszSignatureAlgName == other.lpszSignatureAlgName && self.lpszEncryptionAlgName == other.lpszEncryptionAlgName && self.dwKeySize == other.dwKeySize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_CONNECTED_INFO {
    pub dwConnectedState: INTERNET_STATE,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for INTERNET_CONNECTED_INFO {}
impl ::core::clone::Clone for INTERNET_CONNECTED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_CONNECTED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CONNECTED_INFO").field("dwConnectedState", &self.dwConnectedState).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_CONNECTED_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_CONNECTED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwConnectedState == other.dwConnectedState && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for INTERNET_CONNECTED_INFO {}
impl ::core::default::Default for INTERNET_CONNECTED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_COOKIE {
    pub cbSize: u32,
    pub pszName: ::windows::core::PSTR,
    pub pszData: ::windows::core::PSTR,
    pub pszDomain: ::windows::core::PSTR,
    pub pszPath: ::windows::core::PSTR,
    pub pftExpires: *mut super::super::Foundation::FILETIME,
    pub dwFlags: u32,
    pub pszUrl: ::windows::core::PSTR,
    pub pszP3PPolicy: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_COOKIE").field("cbSize", &self.cbSize).field("pszName", &self.pszName).field("pszData", &self.pszData).field("pszDomain", &self.pszDomain).field("pszPath", &self.pszPath).field("pftExpires", &self.pftExpires).field("dwFlags", &self.dwFlags).field("pszUrl", &self.pszUrl).field("pszP3PPolicy", &self.pszP3PPolicy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_COOKIE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszName == other.pszName && self.pszData == other.pszData && self.pszDomain == other.pszDomain && self.pszPath == other.pszPath && self.pftExpires == other.pftExpires && self.dwFlags == other.dwFlags && self.pszUrl == other.pszUrl && self.pszP3PPolicy == other.pszP3PPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_COOKIE2 {
    pub pwszName: ::windows::core::PWSTR,
    pub pwszValue: ::windows::core::PWSTR,
    pub pwszDomain: ::windows::core::PWSTR,
    pub pwszPath: ::windows::core::PWSTR,
    pub dwFlags: u32,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub fExpiresSet: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_COOKIE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_COOKIE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_COOKIE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_COOKIE2").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).field("pwszDomain", &self.pwszDomain).field("pwszPath", &self.pwszPath).field("dwFlags", &self.dwFlags).field("ftExpires", &self.ftExpires).field("fExpiresSet", &self.fExpiresSet).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_COOKIE2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_COOKIE2 {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pwszValue == other.pwszValue && self.pwszDomain == other.pwszDomain && self.pwszPath == other.pwszPath && self.dwFlags == other.dwFlags && self.ftExpires == other.ftExpires && self.fExpiresSet == other.fExpiresSet
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_COOKIE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_COOKIE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CREDENTIALS {
    pub lpcwszHostName: ::windows::core::PCWSTR,
    pub dwPort: u32,
    pub dwScheme: u32,
    pub lpcwszUrl: ::windows::core::PCWSTR,
    pub lpcwszRealm: ::windows::core::PCWSTR,
    pub fAuthIdentity: super::super::Foundation::BOOL,
    pub Anonymous: INTERNET_CREDENTIALS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CREDENTIALS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CREDENTIALS_0 {
    pub Anonymous: INTERNET_CREDENTIALS_0_0,
    pub pAuthIdentityOpaque: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CREDENTIALS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CREDENTIALS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CREDENTIALS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CREDENTIALS_0_0 {
    pub lpcwszUserName: ::windows::core::PCWSTR,
    pub lpcwszPassword: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_CREDENTIALS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_CREDENTIALS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CREDENTIALS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CREDENTIALS_0_0").field("lpcwszUserName", &self.lpcwszUserName).field("lpcwszPassword", &self.lpcwszPassword).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_CREDENTIALS_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CREDENTIALS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lpcwszUserName == other.lpcwszUserName && self.lpcwszPassword == other.lpcwszPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CREDENTIALS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_DIAGNOSTIC_SOCKET_INFO {
    pub Socket: usize,
    pub SourcePort: u32,
    pub DestPort: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for INTERNET_DIAGNOSTIC_SOCKET_INFO {}
impl ::core::clone::Clone for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_DIAGNOSTIC_SOCKET_INFO").field("Socket", &self.Socket).field("SourcePort", &self.SourcePort).field("DestPort", &self.DestPort).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Socket == other.Socket && self.SourcePort == other.SourcePort && self.DestPort == other.DestPort && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for INTERNET_DIAGNOSTIC_SOCKET_INFO {}
impl ::core::default::Default for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_DOWNLOAD_MODE_HANDLE {
    pub pcwszFileName: ::windows::core::PCWSTR,
    pub phFile: *mut super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_DOWNLOAD_MODE_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_DOWNLOAD_MODE_HANDLE").field("pcwszFileName", &self.pcwszFileName).field("phFile", &self.phFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_DOWNLOAD_MODE_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.pcwszFileName == other.pcwszFileName && self.phFile == other.phFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_DOWNLOAD_MODE_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_END_BROWSER_SESSION_DATA {
    pub lpBuffer: *mut ::core::ffi::c_void,
    pub dwBufferLength: u32,
}
impl ::core::marker::Copy for INTERNET_END_BROWSER_SESSION_DATA {}
impl ::core::clone::Clone for INTERNET_END_BROWSER_SESSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_END_BROWSER_SESSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_END_BROWSER_SESSION_DATA").field("lpBuffer", &self.lpBuffer).field("dwBufferLength", &self.dwBufferLength).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_END_BROWSER_SESSION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_END_BROWSER_SESSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpBuffer == other.lpBuffer && self.dwBufferLength == other.dwBufferLength
    }
}
impl ::core::cmp::Eq for INTERNET_END_BROWSER_SESSION_DATA {}
impl ::core::default::Default for INTERNET_END_BROWSER_SESSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTIONA {
    pub dwOption: INTERNET_PER_CONN,
    pub Value: INTERNET_PER_CONN_OPTIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_PER_CONN_OPTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_PER_CONN_OPTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_PER_CONN_OPTIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_PER_CONN_OPTIONA_0 {
    pub dwValue: u32,
    pub pszValue: ::windows::core::PSTR,
    pub ftValue: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_PER_CONN_OPTIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_PER_CONN_OPTIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_PER_CONN_OPTIONA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTIONW {
    pub dwOption: INTERNET_PER_CONN,
    pub Value: INTERNET_PER_CONN_OPTIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_PER_CONN_OPTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_PER_CONN_OPTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_PER_CONN_OPTIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_PER_CONN_OPTIONW_0 {
    pub dwValue: u32,
    pub pszValue: ::windows::core::PWSTR,
    pub ftValue: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_PER_CONN_OPTIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_PER_CONN_OPTIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_PER_CONN_OPTIONW_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTION_LISTA {
    pub dwSize: u32,
    pub pszConnection: ::windows::core::PSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: *mut INTERNET_PER_CONN_OPTIONA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_PER_CONN_OPTION_LISTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_PER_CONN_OPTION_LISTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_PER_CONN_OPTION_LISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PER_CONN_OPTION_LISTA").field("dwSize", &self.dwSize).field("pszConnection", &self.pszConnection).field("dwOptionCount", &self.dwOptionCount).field("dwOptionError", &self.dwOptionError).field("pOptions", &self.pOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_PER_CONN_OPTION_LISTA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTION_LISTA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pszConnection == other.pszConnection && self.dwOptionCount == other.dwOptionCount && self.dwOptionError == other.dwOptionError && self.pOptions == other.pOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTION_LISTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTION_LISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTION_LISTW {
    pub dwSize: u32,
    pub pszConnection: ::windows::core::PWSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: *mut INTERNET_PER_CONN_OPTIONW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_PER_CONN_OPTION_LISTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_PER_CONN_OPTION_LISTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_PER_CONN_OPTION_LISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PER_CONN_OPTION_LISTW").field("dwSize", &self.dwSize).field("pszConnection", &self.pszConnection).field("dwOptionCount", &self.dwOptionCount).field("dwOptionError", &self.dwOptionError).field("pOptions", &self.pOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_PER_CONN_OPTION_LISTW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTION_LISTW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pszConnection == other.pszConnection && self.dwOptionCount == other.dwOptionCount && self.dwOptionError == other.dwOptionError && self.pOptions == other.pOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTION_LISTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTION_LISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_PREFETCH_STATUS {
    pub dwStatus: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for INTERNET_PREFETCH_STATUS {}
impl ::core::clone::Clone for INTERNET_PREFETCH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_PREFETCH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PREFETCH_STATUS").field("dwStatus", &self.dwStatus).field("dwSize", &self.dwSize).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_PREFETCH_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_PREFETCH_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for INTERNET_PREFETCH_STATUS {}
impl ::core::default::Default for INTERNET_PREFETCH_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_PROXY_INFO {
    pub dwAccessType: INTERNET_ACCESS_TYPE,
    pub lpszProxy: *mut i8,
    pub lpszProxyBypass: *mut i8,
}
impl ::core::marker::Copy for INTERNET_PROXY_INFO {}
impl ::core::clone::Clone for INTERNET_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PROXY_INFO").field("dwAccessType", &self.dwAccessType).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_PROXY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAccessType == other.dwAccessType && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
impl ::core::cmp::Eq for INTERNET_PROXY_INFO {}
impl ::core::default::Default for INTERNET_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
pub struct INTERNET_SECURITY_CONNECTION_INFO {
    pub dwSize: u32,
    pub fSecure: super::super::Foundation::BOOL,
    pub connectionInfo: super::super::Security::Authentication::Identity::SecPkgContext_ConnectionInfo,
    pub cipherInfo: super::super::Security::Authentication::Identity::SecPkgContext_CipherInfo,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::marker::Copy for INTERNET_SECURITY_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::clone::Clone for INTERNET_SECURITY_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::fmt::Debug for INTERNET_SECURITY_CONNECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_SECURITY_CONNECTION_INFO").field("dwSize", &self.dwSize).field("fSecure", &self.fSecure).field("connectionInfo", &self.connectionInfo).field("cipherInfo", &self.cipherInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::windows::core::TypeKind for INTERNET_SECURITY_CONNECTION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::cmp::PartialEq for INTERNET_SECURITY_CONNECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fSecure == other.fSecure && self.connectionInfo == other.connectionInfo && self.cipherInfo == other.cipherInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::cmp::Eq for INTERNET_SECURITY_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::default::Default for INTERNET_SECURITY_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub struct INTERNET_SECURITY_INFO {
    pub dwSize: u32,
    pub pCertificate: *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub pcCertChain: *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT,
    pub connectionInfo: super::super::Security::Authentication::Identity::SecPkgContext_ConnectionInfo,
    pub cipherInfo: super::super::Security::Authentication::Identity::SecPkgContext_CipherInfo,
    pub pcUnverifiedCertChain: *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT,
    pub channelBindingToken: super::super::Security::Authentication::Identity::SecPkgContext_Bindings,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for INTERNET_SECURITY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for INTERNET_SECURITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for INTERNET_SECURITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_SECURITY_INFO").field("dwSize", &self.dwSize).field("pCertificate", &self.pCertificate).field("pcCertChain", &self.pcCertChain).field("connectionInfo", &self.connectionInfo).field("cipherInfo", &self.cipherInfo).field("pcUnverifiedCertChain", &self.pcUnverifiedCertChain).field("channelBindingToken", &self.channelBindingToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for INTERNET_SECURITY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for INTERNET_SECURITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pCertificate == other.pCertificate && self.pcCertChain == other.pcCertChain && self.connectionInfo == other.connectionInfo && self.cipherInfo == other.cipherInfo && self.pcUnverifiedCertChain == other.pcUnverifiedCertChain && self.channelBindingToken == other.channelBindingToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for INTERNET_SECURITY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for INTERNET_SECURITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_SERVER_CONNECTION_STATE {
    pub lpcwszHostName: ::windows::core::PCWSTR,
    pub fProxy: super::super::Foundation::BOOL,
    pub dwCounter: u32,
    pub dwConnectionLimit: u32,
    pub dwAvailableCreates: u32,
    pub dwAvailableKeepAlives: u32,
    pub dwActiveConnections: u32,
    pub dwWaiters: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERNET_SERVER_CONNECTION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERNET_SERVER_CONNECTION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_SERVER_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_SERVER_CONNECTION_STATE").field("lpcwszHostName", &self.lpcwszHostName).field("fProxy", &self.fProxy).field("dwCounter", &self.dwCounter).field("dwConnectionLimit", &self.dwConnectionLimit).field("dwAvailableCreates", &self.dwAvailableCreates).field("dwAvailableKeepAlives", &self.dwAvailableKeepAlives).field("dwActiveConnections", &self.dwActiveConnections).field("dwWaiters", &self.dwWaiters).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for INTERNET_SERVER_CONNECTION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_SERVER_CONNECTION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.lpcwszHostName == other.lpcwszHostName && self.fProxy == other.fProxy && self.dwCounter == other.dwCounter && self.dwConnectionLimit == other.dwConnectionLimit && self.dwAvailableCreates == other.dwAvailableCreates && self.dwAvailableKeepAlives == other.dwAvailableKeepAlives && self.dwActiveConnections == other.dwActiveConnections && self.dwWaiters == other.dwWaiters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_SERVER_CONNECTION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_SERVER_CONNECTION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct INTERNET_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
impl ::core::marker::Copy for INTERNET_VERSION_INFO {}
impl ::core::clone::Clone for INTERNET_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERNET_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_VERSION_INFO").field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
impl ::windows::core::TypeKind for INTERNET_VERSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INTERNET_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion
    }
}
impl ::core::cmp::Eq for INTERNET_VERSION_INFO {}
impl ::core::default::Default for INTERNET_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct IncomingCookieState {
    pub cSession: i32,
    pub cPersistent: i32,
    pub cAccepted: i32,
    pub cLeashed: i32,
    pub cDowngraded: i32,
    pub cBlocked: i32,
    pub pszLocation: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for IncomingCookieState {}
impl ::core::clone::Clone for IncomingCookieState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IncomingCookieState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IncomingCookieState").field("cSession", &self.cSession).field("cPersistent", &self.cPersistent).field("cAccepted", &self.cAccepted).field("cLeashed", &self.cLeashed).field("cDowngraded", &self.cDowngraded).field("cBlocked", &self.cBlocked).field("pszLocation", &self.pszLocation).finish()
    }
}
impl ::windows::core::TypeKind for IncomingCookieState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IncomingCookieState {
    fn eq(&self, other: &Self) -> bool {
        self.cSession == other.cSession && self.cPersistent == other.cPersistent && self.cAccepted == other.cAccepted && self.cLeashed == other.cLeashed && self.cDowngraded == other.cDowngraded && self.cBlocked == other.cBlocked && self.pszLocation == other.pszLocation
    }
}
impl ::core::cmp::Eq for IncomingCookieState {}
impl ::core::default::Default for IncomingCookieState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct InternetCookieHistory {
    pub fAccepted: super::super::Foundation::BOOL,
    pub fLeashed: super::super::Foundation::BOOL,
    pub fDowngraded: super::super::Foundation::BOOL,
    pub fRejected: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for InternetCookieHistory {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for InternetCookieHistory {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for InternetCookieHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InternetCookieHistory").field("fAccepted", &self.fAccepted).field("fLeashed", &self.fLeashed).field("fDowngraded", &self.fDowngraded).field("fRejected", &self.fRejected).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for InternetCookieHistory {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for InternetCookieHistory {
    fn eq(&self, other: &Self) -> bool {
        self.fAccepted == other.fAccepted && self.fLeashed == other.fLeashed && self.fDowngraded == other.fDowngraded && self.fRejected == other.fRejected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for InternetCookieHistory {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for InternetCookieHistory {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct OutgoingCookieState {
    pub cSent: i32,
    pub cSuppressed: i32,
    pub pszLocation: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for OutgoingCookieState {}
impl ::core::clone::Clone for OutgoingCookieState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OutgoingCookieState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OutgoingCookieState").field("cSent", &self.cSent).field("cSuppressed", &self.cSuppressed).field("pszLocation", &self.pszLocation).finish()
    }
}
impl ::windows::core::TypeKind for OutgoingCookieState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OutgoingCookieState {
    fn eq(&self, other: &Self) -> bool {
        self.cSent == other.cSent && self.cSuppressed == other.cSuppressed && self.pszLocation == other.pszLocation
    }
}
impl ::core::cmp::Eq for OutgoingCookieState {}
impl ::core::default::Default for OutgoingCookieState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct ProofOfPossessionCookieInfo {
    pub name: ::windows::core::PWSTR,
    pub data: ::windows::core::PWSTR,
    pub flags: u32,
    pub p3pHeader: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ProofOfPossessionCookieInfo {}
impl ::core::clone::Clone for ProofOfPossessionCookieInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ProofOfPossessionCookieInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProofOfPossessionCookieInfo").field("name", &self.name).field("data", &self.data).field("flags", &self.flags).field("p3pHeader", &self.p3pHeader).finish()
    }
}
impl ::windows::core::TypeKind for ProofOfPossessionCookieInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ProofOfPossessionCookieInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data && self.flags == other.flags && self.p3pHeader == other.p3pHeader
    }
}
impl ::core::cmp::Eq for ProofOfPossessionCookieInfo {}
impl ::core::default::Default for ProofOfPossessionCookieInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct URLCACHE_ENTRY_INFO {
    pub pwszSourceUrlName: ::windows::core::PWSTR,
    pub pwszLocalFileName: ::windows::core::PWSTR,
    pub dwCacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub ftLastModifiedTime: super::super::Foundation::FILETIME,
    pub ftExpireTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastSyncTime: super::super::Foundation::FILETIME,
    pub pbHeaderInfo: *mut u8,
    pub cbHeaderInfoSize: u32,
    pub pbExtraData: *mut u8,
    pub cbExtraDataSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for URLCACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for URLCACHE_ENTRY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URLCACHE_ENTRY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URLCACHE_ENTRY_INFO")
            .field("pwszSourceUrlName", &self.pwszSourceUrlName)
            .field("pwszLocalFileName", &self.pwszLocalFileName)
            .field("dwCacheEntryType", &self.dwCacheEntryType)
            .field("dwUseCount", &self.dwUseCount)
            .field("dwHitRate", &self.dwHitRate)
            .field("dwSizeLow", &self.dwSizeLow)
            .field("dwSizeHigh", &self.dwSizeHigh)
            .field("ftLastModifiedTime", &self.ftLastModifiedTime)
            .field("ftExpireTime", &self.ftExpireTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastSyncTime", &self.ftLastSyncTime)
            .field("pbHeaderInfo", &self.pbHeaderInfo)
            .field("cbHeaderInfoSize", &self.cbHeaderInfoSize)
            .field("pbExtraData", &self.pbExtraData)
            .field("cbExtraDataSize", &self.cbExtraDataSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for URLCACHE_ENTRY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URLCACHE_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszSourceUrlName == other.pwszSourceUrlName && self.pwszLocalFileName == other.pwszLocalFileName && self.dwCacheEntryType == other.dwCacheEntryType && self.dwUseCount == other.dwUseCount && self.dwHitRate == other.dwHitRate && self.dwSizeLow == other.dwSizeLow && self.dwSizeHigh == other.dwSizeHigh && self.ftLastModifiedTime == other.ftLastModifiedTime && self.ftExpireTime == other.ftExpireTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastSyncTime == other.ftLastSyncTime && self.pbHeaderInfo == other.pbHeaderInfo && self.cbHeaderInfoSize == other.cbHeaderInfoSize && self.pbExtraData == other.pbExtraData && self.cbExtraDataSize == other.cbExtraDataSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URLCACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URLCACHE_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct URL_COMPONENTSA {
    pub dwStructSize: u32,
    pub lpszScheme: ::windows::core::PSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: ::windows::core::PSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: ::windows::core::PSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: ::windows::core::PSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: ::windows::core::PSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: ::windows::core::PSTR,
    pub dwExtraInfoLength: u32,
}
impl ::core::marker::Copy for URL_COMPONENTSA {}
impl ::core::clone::Clone for URL_COMPONENTSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for URL_COMPONENTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URL_COMPONENTSA")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpszScheme", &self.lpszScheme)
            .field("dwSchemeLength", &self.dwSchemeLength)
            .field("nScheme", &self.nScheme)
            .field("lpszHostName", &self.lpszHostName)
            .field("dwHostNameLength", &self.dwHostNameLength)
            .field("nPort", &self.nPort)
            .field("lpszUserName", &self.lpszUserName)
            .field("dwUserNameLength", &self.dwUserNameLength)
            .field("lpszPassword", &self.lpszPassword)
            .field("dwPasswordLength", &self.dwPasswordLength)
            .field("lpszUrlPath", &self.lpszUrlPath)
            .field("dwUrlPathLength", &self.dwUrlPathLength)
            .field("lpszExtraInfo", &self.lpszExtraInfo)
            .field("dwExtraInfoLength", &self.dwExtraInfoLength)
            .finish()
    }
}
impl ::windows::core::TypeKind for URL_COMPONENTSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for URL_COMPONENTSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScheme == other.lpszScheme && self.dwSchemeLength == other.dwSchemeLength && self.nScheme == other.nScheme && self.lpszHostName == other.lpszHostName && self.dwHostNameLength == other.dwHostNameLength && self.nPort == other.nPort && self.lpszUserName == other.lpszUserName && self.dwUserNameLength == other.dwUserNameLength && self.lpszPassword == other.lpszPassword && self.dwPasswordLength == other.dwPasswordLength && self.lpszUrlPath == other.lpszUrlPath && self.dwUrlPathLength == other.dwUrlPathLength && self.lpszExtraInfo == other.lpszExtraInfo && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
impl ::core::cmp::Eq for URL_COMPONENTSA {}
impl ::core::default::Default for URL_COMPONENTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub struct URL_COMPONENTSW {
    pub dwStructSize: u32,
    pub lpszScheme: ::windows::core::PWSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: ::windows::core::PWSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: ::windows::core::PWSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: ::windows::core::PWSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: ::windows::core::PWSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: ::windows::core::PWSTR,
    pub dwExtraInfoLength: u32,
}
impl ::core::marker::Copy for URL_COMPONENTSW {}
impl ::core::clone::Clone for URL_COMPONENTSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for URL_COMPONENTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URL_COMPONENTSW")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpszScheme", &self.lpszScheme)
            .field("dwSchemeLength", &self.dwSchemeLength)
            .field("nScheme", &self.nScheme)
            .field("lpszHostName", &self.lpszHostName)
            .field("dwHostNameLength", &self.dwHostNameLength)
            .field("nPort", &self.nPort)
            .field("lpszUserName", &self.lpszUserName)
            .field("dwUserNameLength", &self.dwUserNameLength)
            .field("lpszPassword", &self.lpszPassword)
            .field("dwPasswordLength", &self.dwPasswordLength)
            .field("lpszUrlPath", &self.lpszUrlPath)
            .field("dwUrlPathLength", &self.dwUrlPathLength)
            .field("lpszExtraInfo", &self.lpszExtraInfo)
            .field("dwExtraInfoLength", &self.dwExtraInfoLength)
            .finish()
    }
}
impl ::windows::core::TypeKind for URL_COMPONENTSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for URL_COMPONENTSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScheme == other.lpszScheme && self.dwSchemeLength == other.dwSchemeLength && self.nScheme == other.nScheme && self.lpszHostName == other.lpszHostName && self.dwHostNameLength == other.dwHostNameLength && self.nPort == other.nPort && self.lpszUserName == other.lpszUserName && self.dwUserNameLength == other.dwUserNameLength && self.lpszPassword == other.lpszPassword && self.dwPasswordLength == other.dwPasswordLength && self.lpszUrlPath == other.lpszUrlPath && self.dwUrlPathLength == other.dwUrlPathLength && self.lpszExtraInfo == other.lpszExtraInfo && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
impl ::core::cmp::Eq for URL_COMPONENTSW {}
impl ::core::default::Default for URL_COMPONENTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WININET_PROXY_INFO {
    pub fProxy: super::super::Foundation::BOOL,
    pub fBypass: super::super::Foundation::BOOL,
    pub ProxyScheme: INTERNET_SCHEME,
    pub pwszProxy: ::windows::core::PWSTR,
    pub ProxyPort: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WININET_PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WININET_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WININET_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WININET_PROXY_INFO").field("fProxy", &self.fProxy).field("fBypass", &self.fBypass).field("ProxyScheme", &self.ProxyScheme).field("pwszProxy", &self.pwszProxy).field("ProxyPort", &self.ProxyPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WININET_PROXY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WININET_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fProxy == other.fProxy && self.fBypass == other.fBypass && self.ProxyScheme == other.ProxyScheme && self.pwszProxy == other.pwszProxy && self.ProxyPort == other.ProxyPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WININET_PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WININET_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WININET_PROXY_INFO_LIST {
    pub dwProxyInfoCount: u32,
    pub pProxyInfo: *mut WININET_PROXY_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WININET_PROXY_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WININET_PROXY_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WININET_PROXY_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WININET_PROXY_INFO_LIST").field("dwProxyInfoCount", &self.dwProxyInfoCount).field("pProxyInfo", &self.pProxyInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WININET_PROXY_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WININET_PROXY_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwProxyInfoCount == other.dwProxyInfoCount && self.pProxyInfo == other.pProxyInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WININET_PROXY_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WININET_PROXY_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CACHE_OPERATOR = ::core::option::Option<unsafe extern "system" fn(pcei: *mut INTERNET_CACHE_ENTRY_INFOA, pcbcei: *mut u32, popdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type GOPHER_ATTRIBUTE_ENUMERATOR = ::core::option::Option<unsafe extern "system" fn(lpattributeinfo: *const GOPHER_ATTRIBUTE_TYPE, dwerror: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub type HTTP_POLICY_EXTENSION_INIT = ::core::option::Option<unsafe extern "system" fn(version: HTTP_POLICY_EXTENSION_VERSION, r#type: HTTP_POLICY_EXTENSION_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub type HTTP_POLICY_EXTENSION_SHUTDOWN = ::core::option::Option<unsafe extern "system" fn(r#type: HTTP_POLICY_EXTENSION_TYPE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub type LPINTERNET_STATUS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hinternet: *const ::core::ffi::c_void, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *const ::core::ffi::c_void, dwstatusinformationlength: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`*"]
pub type PFN_AUTH_NOTIFY = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DIAL_HANDLER = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows::core::PCSTR, param2: u32, param3: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfnInternetDeInitializeAutoProxyDll = ::core::option::Option<unsafe extern "system" fn(lpszmime: ::windows::core::PCSTR, dwreserved: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfnInternetGetProxyInfo = ::core::option::Option<unsafe extern "system" fn(lpszurl: ::windows::core::PCSTR, dwurllength: u32, lpszurlhostname: ::windows::core::PCSTR, dwurlhostnamelength: u32, lplpszproxyhostname: *mut ::windows::core::PSTR, lpdwproxyhostnamelength: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_WinInet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfnInternetInitializeAutoProxyDll = ::core::option::Option<unsafe extern "system" fn(dwversion: u32, lpszdownloadedtempfile: ::windows::core::PCSTR, lpszmime: ::windows::core::PCSTR, lpautoproxycallbacks: *mut AutoProxyHelperFunctions, lpautoproxyscriptbuffer: *mut AUTO_PROXY_SCRIPT_BUFFER) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
