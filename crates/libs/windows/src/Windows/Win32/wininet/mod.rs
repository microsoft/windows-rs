#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CommitUrlCacheEntryA<P0, P1, P7, P8>(lpszurlname: P0, lpszlocalfilename: P1, expiretime: super::minwindef::FILETIME, lastmodifiedtime: super::minwindef::FILETIME, cacheentrytype: u32, lpheaderinfo: Option<&[u8]>, lpszfileextension: P7, lpszoriginalurl: P8) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CommitUrlCacheEntryA(lpszurlname : windows_core::PCSTR, lpszlocalfilename : windows_core::PCSTR, expiretime : super::minwindef::FILETIME, lastmodifiedtime : super::minwindef::FILETIME, cacheentrytype : u32, lpheaderinfo : *const u8, cchheaderinfo : u32, lpszfileextension : windows_core::PCSTR, lpszoriginalurl : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CommitUrlCacheEntryA(lpszurlname.param().abi(), lpszlocalfilename.param().abi(), core::mem::transmute(expiretime), core::mem::transmute(lastmodifiedtime), cacheentrytype, core::mem::transmute(lpheaderinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpheaderinfo.map_or(0, |slice| slice.len().try_into().unwrap()), lpszfileextension.param().abi(), lpszoriginalurl.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CommitUrlCacheEntryW<P0, P1, P7, P8>(lpszurlname: P0, lpszlocalfilename: P1, expiretime: super::minwindef::FILETIME, lastmodifiedtime: super::minwindef::FILETIME, cacheentrytype: u32, lpszheaderinfo: Option<&[u16]>, lpszfileextension: P7, lpszoriginalurl: P8) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CommitUrlCacheEntryW(lpszurlname : windows_core::PCWSTR, lpszlocalfilename : windows_core::PCWSTR, expiretime : super::minwindef::FILETIME, lastmodifiedtime : super::minwindef::FILETIME, cacheentrytype : u32, lpszheaderinfo : windows_core::PCWSTR, cchheaderinfo : u32, lpszfileextension : windows_core::PCWSTR, lpszoriginalurl : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CommitUrlCacheEntryW(lpszurlname.param().abi(), lpszlocalfilename.param().abi(), core::mem::transmute(expiretime), core::mem::transmute(lastmodifiedtime), cacheentrytype, core::mem::transmute(lpszheaderinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaderinfo.map_or(0, |slice| slice.len().try_into().unwrap()), lpszfileextension.param().abi(), lpszoriginalurl.param().abi()) }
}
#[inline]
pub unsafe fn CreateMD5SSOHash<P0, P1, P2>(pszchallengeinfo: P0, pwszrealm: P1, pwsztarget: P2, pbhexhash: *mut u8) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateMD5SSOHash(pszchallengeinfo : windows_core::PCWSTR, pwszrealm : windows_core::PCWSTR, pwsztarget : windows_core::PCWSTR, pbhexhash : *mut u8) -> windows_core::BOOL);
    unsafe { CreateMD5SSOHash(pszchallengeinfo.param().abi(), pwszrealm.param().abi(), pwsztarget.param().abi(), pbhexhash as _) }
}
#[inline]
pub unsafe fn CreateUrlCacheEntryA<P0, P2>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P2, lpszfilename: windows_core::PSTR, dwreserved: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheEntryA(lpszurlname : windows_core::PCSTR, dwexpectedfilesize : u32, lpszfileextension : windows_core::PCSTR, lpszfilename : windows_core::PSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { CreateUrlCacheEntryA(lpszurlname.param().abi(), dwexpectedfilesize, lpszfileextension.param().abi(), core::mem::transmute(lpszfilename), dwreserved) }
}
#[inline]
pub unsafe fn CreateUrlCacheEntryW<P0, P2>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P2, lpszfilename: windows_core::PWSTR, dwreserved: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheEntryW(lpszurlname : windows_core::PCWSTR, dwexpectedfilesize : u32, lpszfileextension : windows_core::PCWSTR, lpszfilename : windows_core::PWSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { CreateUrlCacheEntryW(lpszurlname.param().abi(), dwexpectedfilesize, lpszfileextension.param().abi(), core::mem::transmute(lpszfilename), dwreserved) }
}
#[inline]
pub unsafe fn CreateUrlCacheGroup(dwflags: u32, lpreserved: Option<*const core::ffi::c_void>) -> GROUPID {
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheGroup(dwflags : u32, lpreserved : *const core::ffi::c_void) -> GROUPID);
    unsafe { CreateUrlCacheGroup(dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteUrlCacheEntry<P0>(lpszurlname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheEntry(lpszurlname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheEntry(lpszurlname.param().abi()) }
}
#[inline]
pub unsafe fn DeleteUrlCacheEntryA<P0>(lpszurlname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheEntryA(lpszurlname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheEntryA(lpszurlname.param().abi()) }
}
#[inline]
pub unsafe fn DeleteUrlCacheEntryW<P0>(lpszurlname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheEntryW(lpszurlname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheEntryW(lpszurlname.param().abi()) }
}
#[inline]
pub unsafe fn DeleteUrlCacheGroup(groupid: GROUPID, dwflags: u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheGroup(groupid : GROUPID, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheGroup(groupid, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteWpadCacheForNetworks(param0: WPAD_CACHE_DELETE) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn DeleteWpadCacheForNetworks(param0 : WPAD_CACHE_DELETE) -> windows_core::BOOL);
    unsafe { DeleteWpadCacheForNetworks(param0) }
}
#[inline]
pub unsafe fn DetectAutoProxyUrl(pszautoproxyurl: &mut [u8], dwdetectflags: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn DetectAutoProxyUrl(pszautoproxyurl : windows_core::PSTR, cchautoproxyurl : u32, dwdetectflags : u32) -> windows_core::BOOL);
    unsafe { DetectAutoProxyUrl(core::mem::transmute(pszautoproxyurl.as_ptr()), pszautoproxyurl.len().try_into().unwrap(), dwdetectflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindCloseUrlCache(henumhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindCloseUrlCache(henumhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { FindCloseUrlCache(henumhandle) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryA<P0>(lpszurlsearchpattern: P0, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryA(lpszurlsearchpattern : windows_core::PCSTR, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> super::winnt::HANDLE);
    unsafe { FindFirstUrlCacheEntryA(lpszurlsearchpattern.param().abi(), lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExA<P0>(lpszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: GROUPID, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryExA(lpszurlsearchpattern : windows_core::PCSTR, dwflags : u32, dwfilter : u32, groupid : GROUPID, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindFirstUrlCacheEntryExA(lpszurlsearchpattern.param().abi(), dwflags, dwfilter, groupid, lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExW<P0>(lpszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: GROUPID, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryExW(lpszurlsearchpattern : windows_core::PCWSTR, dwflags : u32, dwfilter : u32, groupid : GROUPID, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindFirstUrlCacheEntryExW(lpszurlsearchpattern.param().abi(), dwflags, dwfilter, groupid, lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryW<P0>(lpszurlsearchpattern: P0, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryW(lpszurlsearchpattern : windows_core::PCWSTR, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> super::winnt::HANDLE);
    unsafe { FindFirstUrlCacheEntryW(lpszurlsearchpattern.param().abi(), lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstUrlCacheGroup(dwflags: u32, dwfilter: u32, lpsearchcondition: Option<*const core::ffi::c_void>, dwsearchcondition: Option<u32>, lpgroupid: *mut GROUPID, lpreserved: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE {
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheGroup(dwflags : u32, dwfilter : u32, lpsearchcondition : *const core::ffi::c_void, dwsearchcondition : u32, lpgroupid : *mut GROUPID, lpreserved : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindFirstUrlCacheGroup(dwflags, dwfilter, lpsearchcondition.unwrap_or(core::mem::zeroed()) as _, dwsearchcondition.unwrap_or(core::mem::zeroed()) as _, lpgroupid as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextUrlCacheEntryA(henumhandle: super::winnt::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryA(henumhandle : super::winnt::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryA(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextUrlCacheEntryExA(henumhandle: super::winnt::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryExA(henumhandle : super::winnt::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryExA(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextUrlCacheEntryExW(henumhandle: super::winnt::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryExW(henumhandle : super::winnt::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryExW(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextUrlCacheEntryW(henumhandle: super::winnt::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryW(henumhandle : super::winnt::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryW(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindNextUrlCacheGroup(hfind: super::winnt::HANDLE, lpgroupid: *mut GROUPID, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheGroup(hfind : super::winnt::HANDLE, lpgroupid : *mut GROUPID, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheGroup(hfind, lpgroupid as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpCommandA<P3>(hconnect: super::winhttp::HINTERNET, fexpectresponse: bool, dwflags: u32, lpszcommand: P3, dwcontext: Option<usize>, phftpcommand: Option<*mut super::winhttp::HINTERNET>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCommandA(hconnect : super::winhttp::HINTERNET, fexpectresponse : windows_core::BOOL, dwflags : u32, lpszcommand : windows_core::PCSTR, dwcontext : usize, phftpcommand : *mut super::winhttp::HINTERNET) -> windows_core::BOOL);
    unsafe { FtpCommandA(hconnect, fexpectresponse.into(), dwflags, lpszcommand.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, phftpcommand.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpCommandW<P3>(hconnect: super::winhttp::HINTERNET, fexpectresponse: bool, dwflags: u32, lpszcommand: P3, dwcontext: Option<usize>, phftpcommand: Option<*mut super::winhttp::HINTERNET>) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCommandW(hconnect : super::winhttp::HINTERNET, fexpectresponse : windows_core::BOOL, dwflags : u32, lpszcommand : windows_core::PCWSTR, dwcontext : usize, phftpcommand : *mut super::winhttp::HINTERNET) -> windows_core::BOOL);
    unsafe { FtpCommandW(hconnect, fexpectresponse.into(), dwflags, lpszcommand.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, phftpcommand.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpCreateDirectoryA<P1>(hconnect: super::winhttp::HINTERNET, lpszdirectory: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCreateDirectoryA(hconnect : super::winhttp::HINTERNET, lpszdirectory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpCreateDirectoryA(hconnect, lpszdirectory.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpCreateDirectoryW<P1>(hconnect: super::winhttp::HINTERNET, lpszdirectory: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCreateDirectoryW(hconnect : super::winhttp::HINTERNET, lpszdirectory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpCreateDirectoryW(hconnect, lpszdirectory.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpDeleteFileA<P1>(hconnect: super::winhttp::HINTERNET, lpszfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpDeleteFileA(hconnect : super::winhttp::HINTERNET, lpszfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpDeleteFileA(hconnect, lpszfilename.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpDeleteFileW<P1>(hconnect: super::winhttp::HINTERNET, lpszfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpDeleteFileW(hconnect : super::winhttp::HINTERNET, lpszfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpDeleteFileW(hconnect, lpszfilename.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winhttp"))]
#[inline]
pub unsafe fn FtpFindFirstFileA<P1>(hconnect: super::winhttp::HINTERNET, lpszsearchfile: P1, lpfindfiledata: Option<*mut super::minwinbase::WIN32_FIND_DATAA>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpFindFirstFileA(hconnect : super::winhttp::HINTERNET, lpszsearchfile : windows_core::PCSTR, lpfindfiledata : *mut super::minwinbase::WIN32_FIND_DATAA, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { FtpFindFirstFileA(hconnect, lpszsearchfile.param().abi(), lpfindfiledata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winhttp"))]
#[inline]
pub unsafe fn FtpFindFirstFileW<P1>(hconnect: super::winhttp::HINTERNET, lpszsearchfile: P1, lpfindfiledata: Option<*mut super::minwinbase::WIN32_FIND_DATAW>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpFindFirstFileW(hconnect : super::winhttp::HINTERNET, lpszsearchfile : windows_core::PCWSTR, lpfindfiledata : *mut super::minwinbase::WIN32_FIND_DATAW, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { FtpFindFirstFileW(hconnect, lpszsearchfile.param().abi(), lpfindfiledata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpGetCurrentDirectoryA(hconnect: super::winhttp::HINTERNET, lpszcurrentdirectory: windows_core::PSTR, lpdwcurrentdirectory: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FtpGetCurrentDirectoryA(hconnect : super::winhttp::HINTERNET, lpszcurrentdirectory : windows_core::PSTR, lpdwcurrentdirectory : *mut u32) -> windows_core::BOOL);
    unsafe { FtpGetCurrentDirectoryA(hconnect, core::mem::transmute(lpszcurrentdirectory), lpdwcurrentdirectory as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpGetCurrentDirectoryW(hconnect: super::winhttp::HINTERNET, lpszcurrentdirectory: windows_core::PWSTR, lpdwcurrentdirectory: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FtpGetCurrentDirectoryW(hconnect : super::winhttp::HINTERNET, lpszcurrentdirectory : windows_core::PWSTR, lpdwcurrentdirectory : *mut u32) -> windows_core::BOOL);
    unsafe { FtpGetCurrentDirectoryW(hconnect, core::mem::transmute(lpszcurrentdirectory), lpdwcurrentdirectory as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpGetFileA<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszremotefile: P1, lpsznewfile: P2, ffailifexists: bool, dwflagsandattributes: u32, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpGetFileA(hconnect : super::winhttp::HINTERNET, lpszremotefile : windows_core::PCSTR, lpsznewfile : windows_core::PCSTR, ffailifexists : windows_core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpGetFileA(hconnect, lpszremotefile.param().abi(), lpsznewfile.param().abi(), ffailifexists.into(), dwflagsandattributes, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpGetFileEx<P1, P2>(hftpsession: super::winhttp::HINTERNET, lpszremotefile: P1, lpsznewfile: P2, ffailifexists: bool, dwflagsandattributes: u32, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpGetFileEx(hftpsession : super::winhttp::HINTERNET, lpszremotefile : windows_core::PCSTR, lpsznewfile : windows_core::PCWSTR, ffailifexists : windows_core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpGetFileEx(hftpsession, lpszremotefile.param().abi(), lpsznewfile.param().abi(), ffailifexists.into(), dwflagsandattributes, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpGetFileSize(hfile: super::winhttp::HINTERNET, lpdwfilesizehigh: Option<*mut u32>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn FtpGetFileSize(hfile : super::winhttp::HINTERNET, lpdwfilesizehigh : *mut u32) -> u32);
    unsafe { FtpGetFileSize(hfile, lpdwfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpGetFileW<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszremotefile: P1, lpsznewfile: P2, ffailifexists: bool, dwflagsandattributes: u32, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpGetFileW(hconnect : super::winhttp::HINTERNET, lpszremotefile : windows_core::PCWSTR, lpsznewfile : windows_core::PCWSTR, ffailifexists : windows_core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpGetFileW(hconnect, lpszremotefile.param().abi(), lpsznewfile.param().abi(), ffailifexists.into(), dwflagsandattributes, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpOpenFileA<P1>(hconnect: super::winhttp::HINTERNET, lpszfilename: P1, dwaccess: u32, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpOpenFileA(hconnect : super::winhttp::HINTERNET, lpszfilename : windows_core::PCSTR, dwaccess : u32, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { FtpOpenFileA(hconnect, lpszfilename.param().abi(), dwaccess, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpOpenFileW<P1>(hconnect: super::winhttp::HINTERNET, lpszfilename: P1, dwaccess: u32, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpOpenFileW(hconnect : super::winhttp::HINTERNET, lpszfilename : windows_core::PCWSTR, dwaccess : u32, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { FtpOpenFileW(hconnect, lpszfilename.param().abi(), dwaccess, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpPutFileA<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocalfile: P1, lpsznewremotefile: P2, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpPutFileA(hconnect : super::winhttp::HINTERNET, lpszlocalfile : windows_core::PCSTR, lpsznewremotefile : windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpPutFileA(hconnect, lpszlocalfile.param().abi(), lpsznewremotefile.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpPutFileEx<P1, P2>(hftpsession: super::winhttp::HINTERNET, lpszlocalfile: P1, lpsznewremotefile: P2, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpPutFileEx(hftpsession : super::winhttp::HINTERNET, lpszlocalfile : windows_core::PCWSTR, lpsznewremotefile : windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpPutFileEx(hftpsession, lpszlocalfile.param().abi(), lpsznewremotefile.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpPutFileW<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocalfile: P1, lpsznewremotefile: P2, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpPutFileW(hconnect : super::winhttp::HINTERNET, lpszlocalfile : windows_core::PCWSTR, lpsznewremotefile : windows_core::PCWSTR, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpPutFileW(hconnect, lpszlocalfile.param().abi(), lpsznewremotefile.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpRemoveDirectoryA<P1>(hconnect: super::winhttp::HINTERNET, lpszdirectory: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRemoveDirectoryA(hconnect : super::winhttp::HINTERNET, lpszdirectory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpRemoveDirectoryA(hconnect, lpszdirectory.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpRemoveDirectoryW<P1>(hconnect: super::winhttp::HINTERNET, lpszdirectory: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRemoveDirectoryW(hconnect : super::winhttp::HINTERNET, lpszdirectory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpRemoveDirectoryW(hconnect, lpszdirectory.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpRenameFileA<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszexisting: P1, lpsznew: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRenameFileA(hconnect : super::winhttp::HINTERNET, lpszexisting : windows_core::PCSTR, lpsznew : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpRenameFileA(hconnect, lpszexisting.param().abi(), lpsznew.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpRenameFileW<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszexisting: P1, lpsznew: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRenameFileW(hconnect : super::winhttp::HINTERNET, lpszexisting : windows_core::PCWSTR, lpsznew : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpRenameFileW(hconnect, lpszexisting.param().abi(), lpsznew.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpSetCurrentDirectoryA<P1>(hconnect: super::winhttp::HINTERNET, lpszdirectory: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpSetCurrentDirectoryA(hconnect : super::winhttp::HINTERNET, lpszdirectory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpSetCurrentDirectoryA(hconnect, lpszdirectory.param().abi()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn FtpSetCurrentDirectoryW<P1>(hconnect: super::winhttp::HINTERNET, lpszdirectory: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpSetCurrentDirectoryW(hconnect : super::winhttp::HINTERNET, lpszdirectory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpSetCurrentDirectoryW(hconnect, lpszdirectory.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoA<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoA(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExA<P0, P3>(lpszurl: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: Option<*mut u32>, lpszredirecturl: P3, lpcbredirecturl: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoExA(lpszurl : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpszredirecturl : windows_core::PCSTR, lpcbredirecturl : *const u32, lpreserved : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoExA(lpszurl.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpszredirecturl.param().abi(), lpcbredirecturl.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExW<P0, P3>(lpszurl: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: Option<*mut u32>, lpszredirecturl: P3, lpcbredirecturl: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoExW(lpszurl : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpszredirecturl : windows_core::PCWSTR, lpcbredirecturl : *const u32, lpreserved : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoExW(lpszurl.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpszredirecturl.param().abi(), lpcbredirecturl.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoW<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoW(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeA(gid: GROUPID, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo: *mut u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheGroupAttributeA(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetUrlCacheGroupAttributeA(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo as _, lpcbgroupinfo as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeW(gid: GROUPID, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo: *mut u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheGroupAttributeW(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetUrlCacheGroupAttributeW(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo as _, lpcbgroupinfo as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn GopherCreateLocatorA<P0, P2, P3>(lpszhost: P0, nserverport: super::winhttp::INTERNET_PORT, lpszdisplaystring: P2, lpszselectorstring: P3, dwgophertype: u32, lpszlocator: Option<windows_core::PSTR>, lpdwbufferlength: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherCreateLocatorA(lpszhost : windows_core::PCSTR, nserverport : super::winhttp::INTERNET_PORT, lpszdisplaystring : windows_core::PCSTR, lpszselectorstring : windows_core::PCSTR, dwgophertype : u32, lpszlocator : windows_core::PSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { GopherCreateLocatorA(lpszhost.param().abi(), nserverport, lpszdisplaystring.param().abi(), lpszselectorstring.param().abi(), dwgophertype, lpszlocator.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn GopherCreateLocatorW<P0, P2, P3>(lpszhost: P0, nserverport: super::winhttp::INTERNET_PORT, lpszdisplaystring: P2, lpszselectorstring: P3, dwgophertype: u32, lpszlocator: Option<windows_core::PWSTR>, lpdwbufferlength: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherCreateLocatorW(lpszhost : windows_core::PCWSTR, nserverport : super::winhttp::INTERNET_PORT, lpszdisplaystring : windows_core::PCWSTR, lpszselectorstring : windows_core::PCWSTR, dwgophertype : u32, lpszlocator : windows_core::PWSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { GopherCreateLocatorW(lpszhost.param().abi(), nserverport, lpszdisplaystring.param().abi(), lpszselectorstring.param().abi(), dwgophertype, lpszlocator.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winhttp"))]
#[inline]
pub unsafe fn GopherFindFirstFileA<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocator: P1, lpszsearchstring: P2, lpfinddata: Option<*mut GOPHER_FIND_DATAA>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherFindFirstFileA(hconnect : super::winhttp::HINTERNET, lpszlocator : windows_core::PCSTR, lpszsearchstring : windows_core::PCSTR, lpfinddata : *mut GOPHER_FIND_DATAA, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { GopherFindFirstFileA(hconnect, lpszlocator.param().abi(), lpszsearchstring.param().abi(), lpfinddata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winhttp"))]
#[inline]
pub unsafe fn GopherFindFirstFileW<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocator: P1, lpszsearchstring: P2, lpfinddata: Option<*mut GOPHER_FIND_DATAW>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherFindFirstFileW(hconnect : super::winhttp::HINTERNET, lpszlocator : windows_core::PCWSTR, lpszsearchstring : windows_core::PCWSTR, lpfinddata : *mut GOPHER_FIND_DATAW, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { GopherFindFirstFileW(hconnect, lpszlocator.param().abi(), lpszsearchstring.param().abi(), lpfinddata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winhttp", feature = "winnt"))]
#[inline]
pub unsafe fn GopherGetAttributeA<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocator: P1, lpszattributename: P2, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetAttributeA(hconnect : super::winhttp::HINTERNET, lpszlocator : windows_core::PCSTR, lpszattributename : windows_core::PCSTR, lpbuffer : *mut u8, dwbufferlength : u32, lpdwcharactersreturned : *mut u32, lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext : usize) -> windows_core::BOOL);
    unsafe { GopherGetAttributeA(hconnect, lpszlocator.param().abi(), lpszattributename.param().abi(), lpbuffer as _, dwbufferlength, lpdwcharactersreturned as _, lpfnenumerator, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winhttp", feature = "winnt"))]
#[inline]
pub unsafe fn GopherGetAttributeW<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocator: P1, lpszattributename: P2, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetAttributeW(hconnect : super::winhttp::HINTERNET, lpszlocator : windows_core::PCWSTR, lpszattributename : windows_core::PCWSTR, lpbuffer : *mut u8, dwbufferlength : u32, lpdwcharactersreturned : *mut u32, lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext : usize) -> windows_core::BOOL);
    unsafe { GopherGetAttributeW(hconnect, lpszlocator.param().abi(), lpszattributename.param().abi(), lpbuffer as _, dwbufferlength, lpdwcharactersreturned as _, lpfnenumerator, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GopherGetLocatorTypeA<P0>(lpszlocator: P0, lpdwgophertype: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetLocatorTypeA(lpszlocator : windows_core::PCSTR, lpdwgophertype : *mut u32) -> windows_core::BOOL);
    unsafe { GopherGetLocatorTypeA(lpszlocator.param().abi(), lpdwgophertype as _) }
}
#[inline]
pub unsafe fn GopherGetLocatorTypeW<P0>(lpszlocator: P0, lpdwgophertype: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetLocatorTypeW(lpszlocator : windows_core::PCWSTR, lpdwgophertype : *mut u32) -> windows_core::BOOL);
    unsafe { GopherGetLocatorTypeW(lpszlocator.param().abi(), lpdwgophertype as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn GopherOpenFileA<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocator: P1, lpszview: P2, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherOpenFileA(hconnect : super::winhttp::HINTERNET, lpszlocator : windows_core::PCSTR, lpszview : windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { GopherOpenFileA(hconnect, lpszlocator.param().abi(), lpszview.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn GopherOpenFileW<P1, P2>(hconnect: super::winhttp::HINTERNET, lpszlocator: P1, lpszview: P2, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherOpenFileW(hconnect : super::winhttp::HINTERNET, lpszlocator : windows_core::PCWSTR, lpszview : windows_core::PCWSTR, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { GopherOpenFileW(hconnect, lpszlocator.param().abi(), lpszview.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpAddRequestHeadersA<P1>(hrequest: super::winhttp::HINTERNET, lpszheaders: P1, dwheaderslength: u32, dwmodifiers: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpAddRequestHeadersA(hrequest : super::winhttp::HINTERNET, lpszheaders : windows_core::PCSTR, dwheaderslength : u32, dwmodifiers : u32) -> windows_core::BOOL);
    unsafe { HttpAddRequestHeadersA(hrequest, lpszheaders.param().abi(), dwheaderslength, dwmodifiers) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpAddRequestHeadersW<P1>(hrequest: super::winhttp::HINTERNET, lpszheaders: P1, dwheaderslength: u32, dwmodifiers: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpAddRequestHeadersW(hrequest : super::winhttp::HINTERNET, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, dwmodifiers : u32) -> windows_core::BOOL);
    unsafe { HttpAddRequestHeadersW(hrequest, lpszheaders.param().abi(), dwheaderslength, dwmodifiers) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpEndRequestA(hrequest: super::winhttp::HINTERNET, lpbuffersout: Option<*mut INTERNET_BUFFERSA>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpEndRequestA(hrequest : super::winhttp::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpEndRequestA(hrequest, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpEndRequestW(hrequest: super::winhttp::HINTERNET, lpbuffersout: Option<*mut INTERNET_BUFFERSW>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpEndRequestW(hrequest : super::winhttp::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpEndRequestW(hrequest, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HttpIsHostHstsEnabled<P0>(pcwszurl: P0, pfishsts: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpIsHostHstsEnabled(pcwszurl : windows_core::PCWSTR, pfishsts : *mut windows_core::BOOL) -> u32);
    unsafe { HttpIsHostHstsEnabled(pcwszurl.param().abi(), pfishsts as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpOpenRequestA<P1, P2, P3, P4>(hconnect: super::winhttp::HINTERNET, lpszverb: P1, lpszobjectname: P2, lpszversion: P3, lpszreferrer: P4, lplpszaccepttypes: Option<*const windows_core::PCSTR>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpOpenRequestA(hconnect : super::winhttp::HINTERNET, lpszverb : windows_core::PCSTR, lpszobjectname : windows_core::PCSTR, lpszversion : windows_core::PCSTR, lpszreferrer : windows_core::PCSTR, lplpszaccepttypes : *const windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { HttpOpenRequestA(hconnect, lpszverb.param().abi(), lpszobjectname.param().abi(), lpszversion.param().abi(), lpszreferrer.param().abi(), lplpszaccepttypes.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpOpenRequestW<P1, P2, P3, P4>(hconnect: super::winhttp::HINTERNET, lpszverb: P1, lpszobjectname: P2, lpszversion: P3, lpszreferrer: P4, lplpszaccepttypes: Option<*const windows_core::PCWSTR>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpOpenRequestW(hconnect : super::winhttp::HINTERNET, lpszverb : windows_core::PCWSTR, lpszobjectname : windows_core::PCWSTR, lpszversion : windows_core::PCWSTR, lpszreferrer : windows_core::PCWSTR, lplpszaccepttypes : *const windows_core::PCWSTR, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { HttpOpenRequestW(hconnect, lpszverb.param().abi(), lpszobjectname.param().abi(), lpszversion.param().abi(), lpszreferrer.param().abi(), lplpszaccepttypes.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpQueryInfoA(hrequest: super::winhttp::HINTERNET, dwinfolevel: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpQueryInfoA(hrequest : super::winhttp::HINTERNET, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_core::BOOL);
    unsafe { HttpQueryInfoA(hrequest, dwinfolevel, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _, lpdwindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpQueryInfoW(hrequest: super::winhttp::HINTERNET, dwinfolevel: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpQueryInfoW(hrequest : super::winhttp::HINTERNET, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_core::BOOL);
    unsafe { HttpQueryInfoW(hrequest, dwinfolevel, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _, lpdwindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpSendRequestA(hrequest: super::winhttp::HINTERNET, lpszheaders: Option<&[u8]>, lpoptional: Option<*const core::ffi::c_void>, dwoptionallength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestA(hrequest : super::winhttp::HINTERNET, lpszheaders : windows_core::PCSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32) -> windows_core::BOOL);
    unsafe { HttpSendRequestA(hrequest, core::mem::transmute(lpszheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.map_or(0, |slice| slice.len().try_into().unwrap()), lpoptional.unwrap_or(core::mem::zeroed()) as _, dwoptionallength) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpSendRequestExA(hrequest: super::winhttp::HINTERNET, lpbuffersin: Option<*const INTERNET_BUFFERSA>, lpbuffersout: Option<*mut INTERNET_BUFFERSA>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestExA(hrequest : super::winhttp::HINTERNET, lpbuffersin : *const INTERNET_BUFFERSA, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpSendRequestExA(hrequest, lpbuffersin.unwrap_or(core::mem::zeroed()) as _, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpSendRequestExW(hrequest: super::winhttp::HINTERNET, lpbuffersin: Option<*const INTERNET_BUFFERSW>, lpbuffersout: Option<*mut INTERNET_BUFFERSW>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestExW(hrequest : super::winhttp::HINTERNET, lpbuffersin : *const INTERNET_BUFFERSW, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpSendRequestExW(hrequest, lpbuffersin.unwrap_or(core::mem::zeroed()) as _, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn HttpSendRequestW(hrequest: super::winhttp::HINTERNET, lpszheaders: Option<&[u16]>, lpoptional: Option<*const core::ffi::c_void>, dwoptionallength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestW(hrequest : super::winhttp::HINTERNET, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32) -> windows_core::BOOL);
    unsafe { HttpSendRequestW(hrequest, core::mem::transmute(lpszheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.map_or(0, |slice| slice.len().try_into().unwrap()), lpoptional.unwrap_or(core::mem::zeroed()) as _, dwoptionallength) }
}
#[inline]
pub unsafe fn InternetAttemptConnect(dwreserved: u32) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetAttemptConnect(dwreserved : u32) -> u32);
    unsafe { InternetAttemptConnect(dwreserved) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetAutodial(dwflags: u32, hwndparent: Option<super::windef::HWND>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetAutodial(dwflags : u32, hwndparent : super::windef::HWND) -> windows_core::BOOL);
    unsafe { InternetAutodial(dwflags, hwndparent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetAutodialHangup(dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetAutodialHangup(dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetAutodialHangup(dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetCanonicalizeUrlA<P0>(lpszurl: P0, lpszbuffer: windows_core::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCanonicalizeUrlA(lpszurl : windows_core::PCSTR, lpszbuffer : windows_core::PSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCanonicalizeUrlA(lpszurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags) }
}
#[inline]
pub unsafe fn InternetCanonicalizeUrlW<P0>(lpszurl: P0, lpszbuffer: windows_core::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCanonicalizeUrlW(lpszurl : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCanonicalizeUrlW(lpszurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags) }
}
#[inline]
pub unsafe fn InternetCheckConnectionA<P0>(lpszurl: P0, dwflags: u32, dwreserved: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCheckConnectionA(lpszurl : windows_core::PCSTR, dwflags : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetCheckConnectionA(lpszurl.param().abi(), dwflags, dwreserved) }
}
#[inline]
pub unsafe fn InternetCheckConnectionW<P0>(lpszurl: P0, dwflags: u32, dwreserved: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCheckConnectionW(lpszurl : windows_core::PCWSTR, dwflags : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetCheckConnectionW(lpszurl.param().abi(), dwflags, dwreserved) }
}
#[inline]
pub unsafe fn InternetClearAllPerSiteCookieDecisions() -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetClearAllPerSiteCookieDecisions() -> windows_core::BOOL);
    unsafe { InternetClearAllPerSiteCookieDecisions() }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetCloseHandle(hinternet: super::winhttp::HINTERNET) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetCloseHandle(hinternet : super::winhttp::HINTERNET) -> windows_core::BOOL);
    unsafe { InternetCloseHandle(hinternet) }
}
#[inline]
pub unsafe fn InternetCombineUrlA<P0, P1>(lpszbaseurl: P0, lpszrelativeurl: P1, lpszbuffer: windows_core::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCombineUrlA(lpszbaseurl : windows_core::PCSTR, lpszrelativeurl : windows_core::PCSTR, lpszbuffer : windows_core::PSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCombineUrlA(lpszbaseurl.param().abi(), lpszrelativeurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags) }
}
#[inline]
pub unsafe fn InternetCombineUrlW<P0, P1>(lpszbaseurl: P0, lpszrelativeurl: P1, lpszbuffer: windows_core::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCombineUrlW(lpszbaseurl : windows_core::PCWSTR, lpszrelativeurl : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCombineUrlW(lpszbaseurl.param().abi(), lpszrelativeurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossing<P1, P2>(hwnd: super::windef::HWND, szurlprev: P1, szurlnew: P2, bpost: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConfirmZoneCrossing(hwnd : super::windef::HWND, szurlprev : windows_core::PCSTR, szurlnew : windows_core::PCSTR, bpost : windows_core::BOOL) -> u32);
    unsafe { InternetConfirmZoneCrossing(hwnd, szurlprev.param().abi(), szurlnew.param().abi(), bpost.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossingA<P1, P2>(hwnd: super::windef::HWND, szurlprev: P1, szurlnew: P2, bpost: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConfirmZoneCrossingA(hwnd : super::windef::HWND, szurlprev : windows_core::PCSTR, szurlnew : windows_core::PCSTR, bpost : windows_core::BOOL) -> u32);
    unsafe { InternetConfirmZoneCrossingA(hwnd, szurlprev.param().abi(), szurlnew.param().abi(), bpost.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossingW<P1, P2>(hwnd: super::windef::HWND, szurlprev: P1, szurlnew: P2, bpost: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConfirmZoneCrossingW(hwnd : super::windef::HWND, szurlprev : windows_core::PCWSTR, szurlnew : windows_core::PCWSTR, bpost : windows_core::BOOL) -> u32);
    unsafe { InternetConfirmZoneCrossingW(hwnd, szurlprev.param().abi(), szurlnew.param().abi(), bpost.into()) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetConnectA<P1, P3, P4>(hinternet: super::winhttp::HINTERNET, lpszservername: P1, nserverport: super::winhttp::INTERNET_PORT, lpszusername: P3, lpszpassword: P4, dwservice: u32, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConnectA(hinternet : super::winhttp::HINTERNET, lpszservername : windows_core::PCSTR, nserverport : super::winhttp::INTERNET_PORT, lpszusername : windows_core::PCSTR, lpszpassword : windows_core::PCSTR, dwservice : u32, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { InternetConnectA(hinternet, lpszservername.param().abi(), nserverport, lpszusername.param().abi(), lpszpassword.param().abi(), dwservice, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetConnectW<P1, P3, P4>(hinternet: super::winhttp::HINTERNET, lpszservername: P1, nserverport: super::winhttp::INTERNET_PORT, lpszusername: P3, lpszpassword: P4, dwservice: u32, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConnectW(hinternet : super::winhttp::HINTERNET, lpszservername : windows_core::PCWSTR, nserverport : super::winhttp::INTERNET_PORT, lpszusername : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwservice : u32, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { InternetConnectW(hinternet, lpszservername.param().abi(), nserverport, lpszusername.param().abi(), lpszpassword.param().abi(), dwservice, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetCrackUrlA(lpszurl: &[u8], dwflags: u32, lpurlcomponents: *mut URL_COMPONENTSA) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetCrackUrlA(lpszurl : windows_core::PCSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : *mut URL_COMPONENTSA) -> windows_core::BOOL);
    unsafe { InternetCrackUrlA(core::mem::transmute(lpszurl.as_ptr()), lpszurl.len().try_into().unwrap(), dwflags, lpurlcomponents as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetCrackUrlW(lpszurl: &[u16], dwflags: u32, lpurlcomponents: super::winhttp::LPURL_COMPONENTSW) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetCrackUrlW(lpszurl : windows_core::PCWSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : super::winhttp::LPURL_COMPONENTSW) -> windows_core::BOOL);
    unsafe { InternetCrackUrlW(core::mem::transmute(lpszurl.as_ptr()), lpszurl.len().try_into().unwrap(), dwflags, lpurlcomponents as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetCreateUrlA(lpurlcomponents: *const URL_COMPONENTSA, dwflags: u32, lpszurl: Option<windows_core::PSTR>, lpdwurllength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetCreateUrlA(lpurlcomponents : *const URL_COMPONENTSA, dwflags : u32, lpszurl : windows_core::PSTR, lpdwurllength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetCreateUrlA(lpurlcomponents, dwflags, lpszurl.unwrap_or(core::mem::zeroed()) as _, lpdwurllength as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetCreateUrlW(lpurlcomponents: super::winhttp::LPURL_COMPONENTSW, dwflags: u32, lpszurl: Option<windows_core::PWSTR>, lpdwurllength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetCreateUrlW(lpurlcomponents : super::winhttp::LPURL_COMPONENTSW, dwflags : u32, lpszurl : windows_core::PWSTR, lpdwurllength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetCreateUrlW(lpurlcomponents, dwflags, lpszurl.unwrap_or(core::mem::zeroed()) as _, lpdwurllength as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetDial<P1>(hwndparent: super::windef::HWND, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut u32, dwreserved: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetDial(hwndparent : super::windef::HWND, lpszconnectoid : windows_core::PCSTR, dwflags : u32, lpdwconnection : *mut u32, dwreserved : u32) -> u32);
    unsafe { InternetDial(hwndparent, lpszconnectoid.param().abi(), dwflags, lpdwconnection as _, dwreserved) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetDialA<P1>(hwndparent: super::windef::HWND, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: Option<u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetDialA(hwndparent : super::windef::HWND, lpszconnectoid : windows_core::PCSTR, dwflags : u32, lpdwconnection : *mut usize, dwreserved : u32) -> u32);
    unsafe { InternetDialA(hwndparent, lpszconnectoid.param().abi(), dwflags, lpdwconnection as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetDialW<P1>(hwndparent: super::windef::HWND, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: Option<u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetDialW(hwndparent : super::windef::HWND, lpszconnectoid : windows_core::PCWSTR, dwflags : u32, lpdwconnection : *mut usize, dwreserved : u32) -> u32);
    unsafe { InternetDialW(hwndparent, lpszconnectoid.param().abi(), dwflags, lpdwconnection as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionA(pszsitename: windows_core::PSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetEnumPerSiteCookieDecisionA(pszsitename : windows_core::PSTR, pcsitenamesize : *mut u32, pdwdecision : *mut u32, dwindex : u32) -> windows_core::BOOL);
    unsafe { InternetEnumPerSiteCookieDecisionA(core::mem::transmute(pszsitename), pcsitenamesize as _, pdwdecision as _, dwindex) }
}
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionW(pszsitename: windows_core::PWSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetEnumPerSiteCookieDecisionW(pszsitename : windows_core::PWSTR, pcsitenamesize : *mut u32, pdwdecision : *mut u32, dwindex : u32) -> windows_core::BOOL);
    unsafe { InternetEnumPerSiteCookieDecisionW(core::mem::transmute(pszsitename), pcsitenamesize as _, pdwdecision as _, dwindex) }
}
#[cfg(all(feature = "windef", feature = "winhttp"))]
#[inline]
pub unsafe fn InternetErrorDlg(hwnd: super::windef::HWND, hrequest: Option<super::winhttp::HINTERNET>, dwerror: u32, dwflags: u32, lppvdata: Option<*mut *mut core::ffi::c_void>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetErrorDlg(hwnd : super::windef::HWND, hrequest : super::winhttp::HINTERNET, dwerror : u32, dwflags : u32, lppvdata : *mut *mut core::ffi::c_void) -> u32);
    unsafe { InternetErrorDlg(hwnd, hrequest.unwrap_or(core::mem::zeroed()) as _, dwerror, dwflags, lppvdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetFindNextFileA(hfind: super::winhttp::HINTERNET, lpvfinddata: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetFindNextFileA(hfind : super::winhttp::HINTERNET, lpvfinddata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetFindNextFileA(hfind, lpvfinddata as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetFindNextFileW(hfind: super::winhttp::HINTERNET, lpvfinddata: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetFindNextFileW(hfind : super::winhttp::HINTERNET, lpvfinddata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetFindNextFileW(hfind, lpvfinddata as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn InternetFreeCookies(pcookies: Option<*mut INTERNET_COOKIE2>, dwcookiecount: u32) {
    windows_core::link!("wininet.dll" "system" fn InternetFreeCookies(pcookies : *mut INTERNET_COOKIE2, dwcookiecount : u32));
    unsafe { InternetFreeCookies(pcookies.unwrap_or(core::mem::zeroed()) as _, dwcookiecount) }
}
#[inline]
pub unsafe fn InternetGetConnectedState(lpdwflags: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedState(lpdwflags : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedState(lpdwflags as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetGetConnectedStateEx(lpdwflags: *mut u32, lpszconnectionname: Option<&mut [u8]>, dwreserved: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedStateEx(lpdwflags : *mut u32, lpszconnectionname : windows_core::PSTR, dwnamelen : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedStateEx(lpdwflags as _, core::mem::transmute(lpszconnectionname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwreserved) }
}
#[inline]
pub unsafe fn InternetGetConnectedStateExA(lpdwflags: Option<*mut u32>, lpszconnectionname: Option<&mut [u8]>, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedStateExA(lpdwflags : *mut u32, lpszconnectionname : windows_core::PSTR, cchnamelen : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedStateExA(lpdwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpszconnectionname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetGetConnectedStateExW(lpdwflags: Option<*mut u32>, lpszconnectionname: Option<&mut [u16]>, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedStateExW(lpdwflags : *mut u32, lpszconnectionname : windows_core::PWSTR, cchnamelen : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedStateExW(lpdwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpszconnectionname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetGetCookieA<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: Option<windows_core::PSTR>, lpdwsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PSTR, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetCookieA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn InternetGetCookieEx2<P0, P1>(pcwszurl: P0, pcwszcookiename: P1, dwflags: u32, ppcookies: *mut *mut INTERNET_COOKIE2, pdwcookiecount: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieEx2(pcwszurl : windows_core::PCWSTR, pcwszcookiename : windows_core::PCWSTR, dwflags : u32, ppcookies : *mut *mut INTERNET_COOKIE2, pdwcookiecount : *mut u32) -> u32);
    unsafe { InternetGetCookieEx2(pcwszurl.param().abi(), pcwszcookiename.param().abi(), dwflags, ppcookies as _, pdwcookiecount as _) }
}
#[inline]
pub unsafe fn InternetGetCookieExA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, lpdwsize: *mut u32, dwflags: u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieExA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PCSTR, lpdwsize : *mut u32, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetGetCookieExA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), lpdwsize as _, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetGetCookieExW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, lpdwsize: *mut u32, dwflags: u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieExW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR, lpdwsize : *mut u32, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetGetCookieExW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), lpdwsize as _, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetGetCookieW<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: Option<windows_core::PWSTR>, lpdwsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PWSTR, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetCookieW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _) }
}
#[inline]
pub unsafe fn InternetGetLastResponseInfoA(lpdwerror: *mut u32, lpszbuffer: Option<windows_core::PSTR>, lpdwbufferlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetGetLastResponseInfoA(lpdwerror : *mut u32, lpszbuffer : windows_core::PSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetLastResponseInfoA(lpdwerror as _, lpszbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[inline]
pub unsafe fn InternetGetLastResponseInfoW(lpdwerror: *mut u32, lpszbuffer: Option<windows_core::PWSTR>, lpdwbufferlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetGetLastResponseInfoW(lpdwerror : *mut u32, lpszbuffer : windows_core::PWSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetLastResponseInfoW(lpdwerror as _, lpszbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionA<P0>(pchhostname: P0, presult: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetPerSiteCookieDecisionA(pchhostname : windows_core::PCSTR, presult : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetPerSiteCookieDecisionA(pchhostname.param().abi(), presult as _) }
}
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionW<P0>(pchhostname: P0, presult: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetPerSiteCookieDecisionW(pchhostname : windows_core::PCWSTR, presult : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetPerSiteCookieDecisionW(pchhostname.param().abi(), presult as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetGoOnline<P0>(lpszurl: P0, hwndparent: super::windef::HWND, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGoOnline(lpszurl : windows_core::PCSTR, hwndparent : super::windef::HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetGoOnline(lpszurl.param().abi(), hwndparent, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetGoOnlineA<P0>(lpszurl: P0, hwndparent: super::windef::HWND, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGoOnlineA(lpszurl : windows_core::PCSTR, hwndparent : super::windef::HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetGoOnlineA(lpszurl.param().abi(), hwndparent, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternetGoOnlineW<P0>(lpszurl: P0, hwndparent: super::windef::HWND, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGoOnlineW(lpszurl : windows_core::PCWSTR, hwndparent : super::windef::HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetGoOnlineW(lpszurl.param().abi(), hwndparent, dwflags) }
}
#[inline]
pub unsafe fn InternetHangUp(dwconnection: usize, dwreserved: Option<u32>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetHangUp(dwconnection : usize, dwreserved : u32) -> u32);
    unsafe { InternetHangUp(dwconnection, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetInitializeAutoProxyDll(dwreserved: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetInitializeAutoProxyDll(dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetInitializeAutoProxyDll(dwreserved) }
}
#[cfg(all(feature = "winhttp", feature = "winnt"))]
#[inline]
pub unsafe fn InternetLockRequestFile(hinternet: super::winhttp::HINTERNET, lphlockrequestinfo: *mut super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetLockRequestFile(hinternet : super::winhttp::HINTERNET, lphlockrequestinfo : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { InternetLockRequestFile(hinternet, lphlockrequestinfo as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetOpenA<P0, P2, P3>(lpszagent: P0, dwaccesstype: u32, lpszproxy: P2, lpszproxybypass: P3, dwflags: u32) -> super::winhttp::HINTERNET
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenA(lpszagent : windows_core::PCSTR, dwaccesstype : u32, lpszproxy : windows_core::PCSTR, lpszproxybypass : windows_core::PCSTR, dwflags : u32) -> super::winhttp::HINTERNET);
    unsafe { InternetOpenA(lpszagent.param().abi(), dwaccesstype, lpszproxy.param().abi(), lpszproxybypass.param().abi(), dwflags) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetOpenUrlA<P1>(hinternet: super::winhttp::HINTERNET, lpszurl: P1, lpszheaders: Option<&[u8]>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenUrlA(hinternet : super::winhttp::HINTERNET, lpszurl : windows_core::PCSTR, lpszheaders : windows_core::PCSTR, dwheaderslength : u32, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { InternetOpenUrlA(hinternet, lpszurl.param().abi(), core::mem::transmute(lpszheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.map_or(0, |slice| slice.len().try_into().unwrap()), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetOpenUrlW<P1>(hinternet: super::winhttp::HINTERNET, lpszurl: P1, lpszheaders: Option<&[u16]>, dwflags: u32, dwcontext: Option<usize>) -> super::winhttp::HINTERNET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenUrlW(hinternet : super::winhttp::HINTERNET, lpszurl : windows_core::PCWSTR, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, dwflags : u32, dwcontext : usize) -> super::winhttp::HINTERNET);
    unsafe { InternetOpenUrlW(hinternet, lpszurl.param().abi(), core::mem::transmute(lpszheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.map_or(0, |slice| slice.len().try_into().unwrap()), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetOpenW<P0, P2, P3>(lpszagent: P0, dwaccesstype: u32, lpszproxy: P2, lpszproxybypass: P3, dwflags: u32) -> super::winhttp::HINTERNET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenW(lpszagent : windows_core::PCWSTR, dwaccesstype : u32, lpszproxy : windows_core::PCWSTR, lpszproxybypass : windows_core::PCWSTR, dwflags : u32) -> super::winhttp::HINTERNET);
    unsafe { InternetOpenW(lpszagent.param().abi(), dwaccesstype, lpszproxy.param().abi(), lpszproxybypass.param().abi(), dwflags) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetQueryDataAvailable(hfile: super::winhttp::HINTERNET, lpdwnumberofbytesavailable: Option<*mut u32>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetQueryDataAvailable(hfile : super::winhttp::HINTERNET, lpdwnumberofbytesavailable : *mut u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetQueryDataAvailable(hfile, lpdwnumberofbytesavailable.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetQueryOptionA(hinternet: Option<super::winhttp::HINTERNET>, dwoption: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetQueryOptionA(hinternet : super::winhttp::HINTERNET, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetQueryOptionA(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetQueryOptionW(hinternet: Option<super::winhttp::HINTERNET>, dwoption: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetQueryOptionW(hinternet : super::winhttp::HINTERNET, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetQueryOptionW(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetReadFile(hfile: super::winhttp::HINTERNET, lpbuffer: *mut core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetReadFile(hfile : super::winhttp::HINTERNET, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32) -> windows_core::BOOL);
    unsafe { InternetReadFile(hfile, lpbuffer as _, dwnumberofbytestoread, lpdwnumberofbytesread as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetReadFileExA(hfile: super::winhttp::HINTERNET, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetReadFileExA(hfile : super::winhttp::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetReadFileExA(hfile, lpbuffersout as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetReadFileExW(hfile: super::winhttp::HINTERNET, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetReadFileExW(hfile : super::winhttp::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetReadFileExW(hfile, lpbuffersout as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetCookieA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { InternetSetCookieA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn InternetSetCookieEx2<P0, P2>(pcwszurl: P0, pcookie: *const INTERNET_COOKIE2, pcwszp3ppolicy: P2, dwflags: u32, pdwcookiestate: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieEx2(pcwszurl : windows_core::PCWSTR, pcookie : *const INTERNET_COOKIE2, pcwszp3ppolicy : windows_core::PCWSTR, dwflags : u32, pdwcookiestate : *mut u32) -> u32);
    unsafe { InternetSetCookieEx2(pcwszurl.param().abi(), pcookie, pcwszp3ppolicy.param().abi(), dwflags, pdwcookiestate as _) }
}
#[inline]
pub unsafe fn InternetSetCookieExA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32, dwreserved: Option<usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieExA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PCSTR, dwflags : u32, dwreserved : usize) -> u32);
    unsafe { InternetSetCookieExA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetCookieExW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32, dwreserved: Option<usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieExW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR, dwflags : u32, dwreserved : usize) -> u32);
    unsafe { InternetSetCookieExW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetCookieW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { InternetSetCookieW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi()) }
}
#[inline]
pub unsafe fn InternetSetDialState<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetDialState(lpszconnectoid : windows_core::PCSTR, dwstate : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSetDialState(lpszconnectoid.param().abi(), dwstate, dwreserved) }
}
#[inline]
pub unsafe fn InternetSetDialStateA<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetDialStateA(lpszconnectoid : windows_core::PCSTR, dwstate : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSetDialStateA(lpszconnectoid.param().abi(), dwstate, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetDialStateW<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetDialStateW(lpszconnectoid : windows_core::PCWSTR, dwstate : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSetDialStateW(lpszconnectoid.param().abi(), dwstate, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetFilePointer(hfile: super::winhttp::HINTERNET, ldistancetomove: i32, lpdistancetomovehigh: Option<*mut i32>, dwmovemethod: u32, dwcontext: Option<usize>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetSetFilePointer(hfile : super::winhttp::HINTERNET, ldistancetomove : i32, lpdistancetomovehigh : *mut i32, dwmovemethod : u32, dwcontext : usize) -> u32);
    unsafe { InternetSetFilePointer(hfile, ldistancetomove, lpdistancetomovehigh.unwrap_or(core::mem::zeroed()) as _, dwmovemethod, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetOptionA(hinternet: Option<super::winhttp::HINTERNET>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionA(hinternet : super::winhttp::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionA(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetOptionExA(hinternet: Option<super::winhttp::HINTERNET>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionExA(hinternet : super::winhttp::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionExA(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength, dwflags) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetOptionExW(hinternet: Option<super::winhttp::HINTERNET>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionExW(hinternet : super::winhttp::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionExW(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength, dwflags) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetOptionW(hinternet: Option<super::winhttp::HINTERNET>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionW(hinternet : super::winhttp::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionW(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength) }
}
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionA<P0>(pchhostname: P0, dwdecision: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetPerSiteCookieDecisionA(pchhostname : windows_core::PCSTR, dwdecision : u32) -> windows_core::BOOL);
    unsafe { InternetSetPerSiteCookieDecisionA(pchhostname.param().abi(), dwdecision) }
}
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionW<P0>(pchhostname: P0, dwdecision: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetPerSiteCookieDecisionW(pchhostname : windows_core::PCWSTR, dwdecision : u32) -> windows_core::BOOL);
    unsafe { InternetSetPerSiteCookieDecisionW(pchhostname.param().abi(), dwdecision) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetStatusCallback(hinternet: super::winhttp::HINTERNET, lpfninternetcallback: INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK {
    windows_core::link!("wininet.dll" "system" fn InternetSetStatusCallback(hinternet : super::winhttp::HINTERNET, lpfninternetcallback : INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK);
    unsafe { InternetSetStatusCallback(hinternet, lpfninternetcallback) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetStatusCallbackA(hinternet: super::winhttp::HINTERNET, lpfninternetcallback: INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK {
    windows_core::link!("wininet.dll" "system" fn InternetSetStatusCallbackA(hinternet : super::winhttp::HINTERNET, lpfninternetcallback : INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK);
    unsafe { InternetSetStatusCallbackA(hinternet, lpfninternetcallback) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetSetStatusCallbackW(hinternet: super::winhttp::HINTERNET, lpfninternetcallback: INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK {
    windows_core::link!("wininet.dll" "system" fn InternetSetStatusCallbackW(hinternet : super::winhttp::HINTERNET, lpfninternetcallback : INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK);
    unsafe { InternetSetStatusCallbackW(hinternet, lpfninternetcallback) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn InternetTimeFromSystemTime(pst: *const super::minwinbase::SYSTEMTIME, dwrfc: u32, lpsztime: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetTimeFromSystemTime(pst : *const super::minwinbase::SYSTEMTIME, dwrfc : u32, lpsztime : windows_core::PSTR, cbtime : u32) -> windows_core::BOOL);
    unsafe { InternetTimeFromSystemTime(pst, dwrfc, core::mem::transmute(lpsztime.as_ptr()), lpsztime.len().try_into().unwrap()) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn InternetTimeFromSystemTimeA(pst: *const super::minwinbase::SYSTEMTIME, dwrfc: u32, lpsztime: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetTimeFromSystemTimeA(pst : *const super::minwinbase::SYSTEMTIME, dwrfc : u32, lpsztime : windows_core::PSTR, cbtime : u32) -> windows_core::BOOL);
    unsafe { InternetTimeFromSystemTimeA(pst, dwrfc, core::mem::transmute(lpsztime.as_ptr()), lpsztime.len().try_into().unwrap()) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn InternetTimeFromSystemTimeW(pst: *const super::minwinbase::SYSTEMTIME, dwrfc: u32, lpsztime: windows_core::PWSTR, cbtime: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetTimeFromSystemTimeW(pst : *const super::minwinbase::SYSTEMTIME, dwrfc : u32, lpsztime : windows_core::PWSTR, cbtime : u32) -> windows_core::BOOL);
    unsafe { InternetTimeFromSystemTimeW(pst, dwrfc, core::mem::transmute(lpsztime), cbtime) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn InternetTimeToSystemTime<P0>(lpsztime: P0, pst: *mut super::minwinbase::SYSTEMTIME, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetTimeToSystemTime(lpsztime : windows_core::PCSTR, pst : *mut super::minwinbase::SYSTEMTIME, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetTimeToSystemTime(lpsztime.param().abi(), pst as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn InternetTimeToSystemTimeA<P0>(lpsztime: P0, pst: *mut super::minwinbase::SYSTEMTIME, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetTimeToSystemTimeA(lpsztime : windows_core::PCSTR, pst : *mut super::minwinbase::SYSTEMTIME, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetTimeToSystemTimeA(lpsztime.param().abi(), pst as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn InternetTimeToSystemTimeW<P0>(lpsztime: P0, pst: *mut super::minwinbase::SYSTEMTIME, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetTimeToSystemTimeW(lpsztime : windows_core::PCWSTR, pst : *mut super::minwinbase::SYSTEMTIME, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetTimeToSystemTimeW(lpsztime.param().abi(), pst as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InternetUnlockRequestFile(hlockrequestinfo: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetUnlockRequestFile(hlockrequestinfo : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { InternetUnlockRequestFile(hlockrequestinfo as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn InternetWriteFile(hfile: super::winhttp::HINTERNET, lpbuffer: *const core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetWriteFile(hfile : super::winhttp::HINTERNET, lpbuffer : *const core::ffi::c_void, dwnumberofbytestowrite : u32, lpdwnumberofbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { InternetWriteFile(hfile, lpbuffer, dwnumberofbytestowrite, lpdwnumberofbyteswritten as _) }
}
#[inline]
pub unsafe fn PrivacyGetZonePreferenceW(dwzone: u32, dwtype: u32, pdwtemplate: Option<*mut u32>, pszbuffer: Option<windows_core::PWSTR>, pdwbufferlength: Option<*mut u32>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn PrivacyGetZonePreferenceW(dwzone : u32, dwtype : u32, pdwtemplate : *mut u32, pszbuffer : windows_core::PWSTR, pdwbufferlength : *mut u32) -> u32);
    unsafe { PrivacyGetZonePreferenceW(dwzone, dwtype, pdwtemplate.unwrap_or(core::mem::zeroed()) as _, pszbuffer.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PrivacySetZonePreferenceW<P3>(dwzone: u32, dwtype: u32, dwtemplate: u32, pszpreference: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn PrivacySetZonePreferenceW(dwzone : u32, dwtype : u32, dwtemplate : u32, pszpreference : windows_core::PCWSTR) -> u32);
    unsafe { PrivacySetZonePreferenceW(dwzone, dwtype, dwtemplate, pszpreference.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadUrlCacheEntryStream(hurlcachestream: super::winnt::HANDLE, dwlocation: u32, lpbuffer: *mut core::ffi::c_void, lpdwlen: *mut u32, reserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn ReadUrlCacheEntryStream(hurlcachestream : super::winnt::HANDLE, dwlocation : u32, lpbuffer : *mut core::ffi::c_void, lpdwlen : *mut u32, reserved : u32) -> windows_core::BOOL);
    unsafe { ReadUrlCacheEntryStream(hurlcachestream, dwlocation, lpbuffer as _, lpdwlen as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadUrlCacheEntryStreamEx(hurlcachestream: super::winnt::HANDLE, qwlocation: super::winnt::DWORDLONG, lpbuffer: *mut core::ffi::c_void, lpdwlen: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn ReadUrlCacheEntryStreamEx(hurlcachestream : super::winnt::HANDLE, qwlocation : super::winnt::DWORDLONG, lpbuffer : *mut core::ffi::c_void, lpdwlen : *mut u32) -> windows_core::BOOL);
    unsafe { ReadUrlCacheEntryStreamEx(hurlcachestream, qwlocation, lpbuffer as _, lpdwlen as _) }
}
#[cfg(feature = "winhttp")]
#[inline]
pub unsafe fn ResumeSuspendedDownload(hrequest: super::winhttp::HINTERNET, dwresultcode: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn ResumeSuspendedDownload(hrequest : super::winhttp::HINTERNET, dwresultcode : u32) -> windows_core::BOOL);
    unsafe { ResumeSuspendedDownload(hrequest, dwresultcode) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileA<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryFileA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { RetrieveUrlCacheEntryFileA(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileW<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryFileW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { RetrieveUrlCacheEntryFileW(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamA<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, frandomread: bool, dwreserved: Option<u32>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryStreamA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, frandomread : windows_core::BOOL, dwreserved : u32) -> super::winnt::HANDLE);
    unsafe { RetrieveUrlCacheEntryStreamA(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, frandomread.into(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamW<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, frandomread: bool, dwreserved: Option<u32>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryStreamW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, frandomread : windows_core::BOOL, dwreserved : u32) -> super::winnt::HANDLE);
    unsafe { RetrieveUrlCacheEntryStreamW(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, frandomread.into(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryGroup<P0>(lpszurlname: P0, dwflags: u32, groupid: GROUPID, pbgroupattributes: Option<*const u8>, cbgroupattributes: Option<u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryGroup(lpszurlname : windows_core::PCSTR, dwflags : u32, groupid : GROUPID, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryGroup(lpszurlname.param().abi(), dwflags, groupid, pbgroupattributes.unwrap_or(core::mem::zeroed()) as _, cbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryGroupA<P0>(lpszurlname: P0, dwflags: u32, groupid: GROUPID, pbgroupattributes: Option<*const u8>, cbgroupattributes: Option<u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryGroupA(lpszurlname : windows_core::PCSTR, dwflags : u32, groupid : GROUPID, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryGroupA(lpszurlname.param().abi(), dwflags, groupid, pbgroupattributes.unwrap_or(core::mem::zeroed()) as _, cbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryGroupW<P0>(lpszurlname: P0, dwflags: u32, groupid: GROUPID, pbgroupattributes: Option<*const u8>, cbgroupattributes: Option<u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryGroupW(lpszurlname : windows_core::PCWSTR, dwflags : u32, groupid : GROUPID, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryGroupW(lpszurlname.param().abi(), dwflags, groupid, pbgroupattributes.unwrap_or(core::mem::zeroed()) as _, cbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetUrlCacheEntryInfoA<P0>(lpszurlname: P0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryInfoA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryInfoA(lpszurlname.param().abi(), lpcacheentryinfo, dwfieldcontrol) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetUrlCacheEntryInfoW<P0>(lpszurlname: P0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryInfoW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryInfoW(lpszurlname.param().abi(), lpcacheentryinfo, dwfieldcontrol) }
}
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeA(gid: GROUPID, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOA, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheGroupAttributeA(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOA, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheGroupAttributeA(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeW(gid: GROUPID, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOW, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheGroupAttributeW(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOW, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheGroupAttributeW(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryFile<P0>(lpszurlname: P0, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFile(lpszurlname : windows_core::PCSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryFile(lpszurlname.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileA<P0>(lpszurlname: P0, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFileA(lpszurlname : windows_core::PCSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryFileA(lpszurlname.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileW<P0>(lpszurlname: P0, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFileW(lpszurlname : windows_core::PCWSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryFileW(lpszurlname.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryStream(hurlcachestream: super::winnt::HANDLE, reserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryStream(hurlcachestream : super::winnt::HANDLE, reserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryStream(hurlcachestream, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
pub const AUTH_FLAG_DISABLE_BASIC_CLEARCHANNEL: u32 = 4;
pub const AUTH_FLAG_DISABLE_NEGOTIATE: u32 = 1;
pub const AUTH_FLAG_DISABLE_SERVER_AUTH: u32 = 8;
pub const AUTH_FLAG_ENABLE_NEGOTIATE: u32 = 2;
pub const AUTODIAL_MODE_ALWAYS: u32 = 2;
pub const AUTODIAL_MODE_NEVER: u32 = 1;
pub const AUTODIAL_MODE_NO_NETWORK_PRESENT: u32 = 4;
pub const AUTO_PROXY_FLAG_ALWAYS_DETECT: u32 = 2;
pub const AUTO_PROXY_FLAG_CACHE_INIT_RUN: u32 = 32;
pub const AUTO_PROXY_FLAG_DETECTION_RUN: u32 = 4;
pub const AUTO_PROXY_FLAG_DETECTION_SUSPECT: u32 = 64;
pub const AUTO_PROXY_FLAG_DONT_CACHE_PROXY_RESULT: u32 = 16;
pub const AUTO_PROXY_FLAG_MIGRATED: u32 = 8;
pub const AUTO_PROXY_FLAG_USER_SET: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTO_PROXY_SCRIPT_BUFFER {
    pub dwStructSize: u32,
    pub lpszScriptBuffer: windows_core::PSTR,
    pub dwScriptBufferSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoProxyHelperFunctions {
    pub lpVtbl: *const AutoProxyHelperVtbl,
}
impl Default for AutoProxyHelperFunctions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoProxyHelperVtbl {
    pub IsResolvable: *mut u8,
    pub GetIPAddress: *mut u8,
    pub ResolveHostName: *mut u8,
    pub IsInNet: *mut u8,
    pub IsResolvableEx: *mut u8,
    pub GetIPAddressEx: *mut u8,
    pub ResolveHostNameEx: *mut u8,
    pub IsInNetEx: *mut u8,
    pub SortIpList: *mut u8,
}
impl Default for AutoProxyHelperVtbl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CACHEGROUP_ATTRIBUTE_BASIC: u32 = 1;
pub const CACHEGROUP_ATTRIBUTE_FLAG: u32 = 2;
pub const CACHEGROUP_ATTRIBUTE_GET_ALL: u32 = 4294967295;
pub const CACHEGROUP_ATTRIBUTE_GROUPNAME: u32 = 16;
pub const CACHEGROUP_ATTRIBUTE_QUOTA: u32 = 8;
pub const CACHEGROUP_ATTRIBUTE_STORAGE: u32 = 32;
pub const CACHEGROUP_ATTRIBUTE_TYPE: u32 = 4;
pub const CACHEGROUP_FLAG_FLUSHURL_ONDELETE: u32 = 2;
pub const CACHEGROUP_FLAG_GIDONLY: u32 = 4;
pub const CACHEGROUP_FLAG_NONPURGEABLE: u32 = 1;
pub const CACHEGROUP_READWRITE_MASK: u32 = 60;
pub const CACHEGROUP_SEARCH_ALL: u32 = 0;
pub const CACHEGROUP_SEARCH_BYURL: u32 = 1;
pub const CACHEGROUP_TYPE_INVALID: u32 = 1;
pub const CACHE_ENTRY_ACCTIME_FC: u32 = 256;
pub const CACHE_ENTRY_ATTRIBUTE_FC: u32 = 4;
pub const CACHE_ENTRY_EXEMPT_DELTA_FC: u32 = 2048;
pub const CACHE_ENTRY_EXPTIME_FC: u32 = 128;
pub const CACHE_ENTRY_HEADERINFO_FC: u32 = 1024;
pub const CACHE_ENTRY_HITRATE_FC: u32 = 16;
pub const CACHE_ENTRY_MODTIME_FC: u32 = 64;
pub const CACHE_ENTRY_SYNCTIME_FC: u32 = 512;
pub const CERN_PROXY_INTERNET_ACCESS: u32 = 3;
pub const COOKIE_CACHE_ENTRY: u32 = 1048576;
pub const COOKIE_STATE_ACCEPT: InternetCookieState = 1;
pub const COOKIE_STATE_DOWNGRADE: InternetCookieState = 4;
pub const COOKIE_STATE_LEASH: InternetCookieState = 3;
pub const COOKIE_STATE_MAX: InternetCookieState = 5;
pub const COOKIE_STATE_PROMPT: InternetCookieState = 2;
pub const COOKIE_STATE_REJECT: InternetCookieState = 5;
pub const COOKIE_STATE_UNKNOWN: InternetCookieState = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CookieDecision {
    pub dwCookieState: u32,
    pub fAllowSession: windows_core::BOOL,
}
pub const EDITED_CACHE_ENTRY: u32 = 8;
pub const ERROR_FTP_DROPPED: u32 = 12111;
pub const ERROR_FTP_NO_PASSIVE_MODE: u32 = 12112;
pub const ERROR_FTP_TRANSFER_IN_PROGRESS: u32 = 12110;
pub const ERROR_GOPHER_ATTRIBUTE_NOT_FOUND: u32 = 12137;
pub const ERROR_GOPHER_DATA_ERROR: u32 = 12132;
pub const ERROR_GOPHER_END_OF_DATA: u32 = 12133;
pub const ERROR_GOPHER_INCORRECT_LOCATOR_TYPE: u32 = 12135;
pub const ERROR_GOPHER_INVALID_LOCATOR: u32 = 12134;
pub const ERROR_GOPHER_NOT_FILE: u32 = 12131;
pub const ERROR_GOPHER_NOT_GOPHER_PLUS: u32 = 12136;
pub const ERROR_GOPHER_PROTOCOL_ERROR: u32 = 12130;
pub const ERROR_GOPHER_UNKNOWN_LOCATOR: u32 = 12138;
pub const ERROR_HTTP_COOKIE_DECLINED: u32 = 12162;
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION: u32 = 12161;
pub const ERROR_HTTP_DOWNLEVEL_SERVER: u32 = 12151;
pub const ERROR_HTTP_HEADER_ALREADY_EXISTS: u32 = 12155;
pub const ERROR_HTTP_HEADER_NOT_FOUND: u32 = 12150;
pub const ERROR_HTTP_HSTS_REDIRECT_REQUIRED: u32 = 12060;
pub const ERROR_HTTP_INVALID_HEADER: u32 = 12153;
pub const ERROR_HTTP_INVALID_QUERY_REQUEST: u32 = 12154;
pub const ERROR_HTTP_INVALID_SERVER_RESPONSE: u32 = 12152;
pub const ERROR_HTTP_NOT_REDIRECTED: u32 = 12160;
pub const ERROR_HTTP_REDIRECT_FAILED: u32 = 12156;
pub const ERROR_HTTP_REDIRECT_NEEDS_CONFIRMATION: u32 = 12168;
pub const ERROR_INTERNET_ASYNC_THREAD_FAILED: u32 = 12047;
pub const ERROR_INTERNET_BAD_AUTO_PROXY_SCRIPT: u32 = 12166;
pub const ERROR_INTERNET_BAD_OPTION_LENGTH: u32 = 12010;
pub const ERROR_INTERNET_BAD_REGISTRY_PARAMETER: u32 = 12022;
pub const ERROR_INTERNET_CANNOT_CONNECT: u32 = 12029;
pub const ERROR_INTERNET_CHG_POST_IS_NON_SECURE: u32 = 12042;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED: u32 = 12044;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED_PROXY: u32 = 12187;
pub const ERROR_INTERNET_CLIENT_AUTH_NOT_SETUP: u32 = 12046;
pub const ERROR_INTERNET_CONNECTION_ABORTED: u32 = 12030;
pub const ERROR_INTERNET_CONNECTION_RESET: u32 = 12031;
pub const ERROR_INTERNET_DECODING_FAILED: u32 = 12175;
pub const ERROR_INTERNET_DIALOG_PENDING: u32 = 12049;
pub const ERROR_INTERNET_DISCONNECTED: u32 = 12163;
pub const ERROR_INTERNET_EXTENDED_ERROR: u32 = 12003;
pub const ERROR_INTERNET_FAILED_DUETOSECURITYCHECK: u32 = 12171;
pub const ERROR_INTERNET_FEATURE_DISABLED: u32 = 12192;
pub const ERROR_INTERNET_FORCE_RETRY: u32 = 12032;
pub const ERROR_INTERNET_FORTEZZA_LOGIN_NEEDED: u32 = 12054;
pub const ERROR_INTERNET_GLOBAL_CALLBACK_FAILED: u32 = 12191;
pub const ERROR_INTERNET_HANDLE_EXISTS: u32 = 12036;
pub const ERROR_INTERNET_HTTPS_HTTP_SUBMIT_REDIR: u32 = 12052;
pub const ERROR_INTERNET_HTTPS_TO_HTTP_ON_REDIR: u32 = 12040;
pub const ERROR_INTERNET_HTTP_PROTOCOL_MISMATCH: u32 = 12190;
pub const ERROR_INTERNET_HTTP_TO_HTTPS_ON_REDIR: u32 = 12039;
pub const ERROR_INTERNET_INCORRECT_FORMAT: u32 = 12027;
pub const ERROR_INTERNET_INCORRECT_HANDLE_STATE: u32 = 12019;
pub const ERROR_INTERNET_INCORRECT_HANDLE_TYPE: u32 = 12018;
pub const ERROR_INTERNET_INCORRECT_PASSWORD: u32 = 12014;
pub const ERROR_INTERNET_INCORRECT_USER_NAME: u32 = 12013;
pub const ERROR_INTERNET_INSERT_CDROM: u32 = 12053;
pub const ERROR_INTERNET_INTERNAL_ERROR: u32 = 12004;
pub const ERROR_INTERNET_INVALID_CA: u32 = 12045;
pub const ERROR_INTERNET_INVALID_OPERATION: u32 = 12016;
pub const ERROR_INTERNET_INVALID_OPTION: u32 = 12009;
pub const ERROR_INTERNET_INVALID_PROXY_REQUEST: u32 = 12033;
pub const ERROR_INTERNET_INVALID_URL: u32 = 12005;
pub const ERROR_INTERNET_ITEM_NOT_FOUND: u32 = 12028;
pub const ERROR_INTERNET_LOGIN_FAILURE: u32 = 12015;
pub const ERROR_INTERNET_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 12174;
pub const ERROR_INTERNET_MIXED_SECURITY: u32 = 12041;
pub const ERROR_INTERNET_NAME_NOT_RESOLVED: u32 = 12007;
pub const ERROR_INTERNET_NEED_MSN_SSPI_PKG: u32 = 12173;
pub const ERROR_INTERNET_NEED_UI: u32 = 12034;
pub const ERROR_INTERNET_NOT_INITIALIZED: u32 = 12172;
pub const ERROR_INTERNET_NOT_PROXY_REQUEST: u32 = 12020;
pub const ERROR_INTERNET_NO_CALLBACK: u32 = 12025;
pub const ERROR_INTERNET_NO_CONTEXT: u32 = 12024;
pub const ERROR_INTERNET_NO_DIRECT_ACCESS: u32 = 12023;
pub const ERROR_INTERNET_OPERATION_CANCELLED: u32 = 12017;
pub const ERROR_INTERNET_OPTION_NOT_SETTABLE: u32 = 12011;
pub const ERROR_INTERNET_OUT_OF_HANDLES: u32 = 12001;
pub const ERROR_INTERNET_POST_IS_NON_SECURE: u32 = 12043;
pub const ERROR_INTERNET_PROTOCOL_NOT_FOUND: u32 = 12008;
pub const ERROR_INTERNET_PROXY_SERVER_UNREACHABLE: u32 = 12165;
pub const ERROR_INTERNET_REDIRECT_SCHEME_CHANGE: u32 = 12048;
pub const ERROR_INTERNET_REGISTRY_VALUE_NOT_FOUND: u32 = 12021;
pub const ERROR_INTERNET_REQUEST_PENDING: u32 = 12026;
pub const ERROR_INTERNET_RETRY_DIALOG: u32 = 12050;
pub const ERROR_INTERNET_SECURE_FAILURE_PROXY: u32 = 12188;
pub const ERROR_INTERNET_SECURITY_CHANNEL_ERROR: u32 = 12157;
pub const ERROR_INTERNET_SEC_CERT_CN_INVALID: u32 = 12038;
pub const ERROR_INTERNET_SEC_CERT_DATE_INVALID: u32 = 12037;
pub const ERROR_INTERNET_SEC_CERT_ERRORS: u32 = 12055;
pub const ERROR_INTERNET_SEC_CERT_NO_REV: u32 = 12056;
pub const ERROR_INTERNET_SEC_CERT_REVOKED: u32 = 12170;
pub const ERROR_INTERNET_SEC_CERT_REV_FAILED: u32 = 12057;
pub const ERROR_INTERNET_SEC_CERT_WEAK_SIGNATURE: u32 = 12062;
pub const ERROR_INTERNET_SEC_INVALID_CERT: u32 = 12169;
pub const ERROR_INTERNET_SERVER_UNREACHABLE: u32 = 12164;
pub const ERROR_INTERNET_SHUTDOWN: u32 = 12012;
pub const ERROR_INTERNET_TCPIP_NOT_INSTALLED: u32 = 12159;
pub const ERROR_INTERNET_TIMEOUT: u32 = 12002;
pub const ERROR_INTERNET_UNABLE_TO_CACHE_FILE: u32 = 12158;
pub const ERROR_INTERNET_UNABLE_TO_DOWNLOAD_SCRIPT: u32 = 12167;
pub const ERROR_INTERNET_UNRECOGNIZED_SCHEME: u32 = 12006;
pub const FLAGS_ERROR_UI_FILTER_FOR_ERRORS: u32 = 1;
pub const FLAGS_ERROR_UI_FLAGS_CHANGE_OPTIONS: u32 = 2;
pub const FLAGS_ERROR_UI_FLAGS_GENERATE_DATA: u32 = 4;
pub const FLAGS_ERROR_UI_FLAGS_NO_UI: u32 = 8;
pub const FLAGS_ERROR_UI_SERIALIZE_DIALOGS: u32 = 16;
pub const FLAG_ICC_FORCE_CONNECTION: u32 = 1;
pub const FTP_TRANSFER_TYPE_ASCII: u32 = 1;
pub const FTP_TRANSFER_TYPE_BINARY: u32 = 2;
pub const FTP_TRANSFER_TYPE_MASK: u32 = 3;
pub const FTP_TRANSFER_TYPE_UNKNOWN: u32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    pub ShortAbstract: super::winnt::LPCTSTR,
    pub AbstractFile: super::winnt::LPCTSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_ADMIN_ATTRIBUTE_TYPE {
    pub Comment: super::winnt::LPCTSTR,
    pub EmailAddress: super::winnt::LPCTSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_ASK_ATTRIBUTE_TYPE {
    pub QuestionType: super::winnt::LPCTSTR,
    pub QuestionText: super::winnt::LPCTSTR,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type GOPHER_ATTRIBUTE_ENUMERATOR = Option<unsafe extern "system" fn(lpattributeinfo: *const GOPHER_ATTRIBUTE_TYPE, dwerror: u32) -> windows_core::BOOL>;
pub const GOPHER_ATTRIBUTE_ID_ABSTRACT: i32 = -1412641770;
pub const GOPHER_ATTRIBUTE_ID_ADMIN: i32 = -1412641782;
pub const GOPHER_ATTRIBUTE_ID_ALL: i32 = -1412641783;
pub const GOPHER_ATTRIBUTE_ID_BASE: u32 = 2882325504;
pub const GOPHER_ATTRIBUTE_ID_GEOG: i32 = -1412641774;
pub const GOPHER_ATTRIBUTE_ID_LOCATION: i32 = -1412641775;
pub const GOPHER_ATTRIBUTE_ID_MOD_DATE: i32 = -1412641781;
pub const GOPHER_ATTRIBUTE_ID_ORG: i32 = -1412641776;
pub const GOPHER_ATTRIBUTE_ID_PROVIDER: i32 = -1412641772;
pub const GOPHER_ATTRIBUTE_ID_RANGE: i32 = -1412641778;
pub const GOPHER_ATTRIBUTE_ID_SCORE: i32 = -1412641779;
pub const GOPHER_ATTRIBUTE_ID_SITE: i32 = -1412641777;
pub const GOPHER_ATTRIBUTE_ID_TIMEZONE: i32 = -1412641773;
pub const GOPHER_ATTRIBUTE_ID_TREEWALK: i32 = -1412641768;
pub const GOPHER_ATTRIBUTE_ID_TTL: i32 = -1412641780;
pub const GOPHER_ATTRIBUTE_ID_UNKNOWN: i32 = -1412641767;
pub const GOPHER_ATTRIBUTE_ID_VERSION: i32 = -1412641771;
pub const GOPHER_ATTRIBUTE_ID_VIEW: i32 = -1412641769;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GOPHER_ATTRIBUTE_TYPE {
    pub CategoryId: u32,
    pub AttributeId: u32,
    pub AttributeType: GOPHER_ATTRIBUTE_TYPE_0,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for GOPHER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
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
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for GOPHER_ATTRIBUTE_TYPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_CATEGORY_ID_ABSTRACT: i32 = -1412641787;
pub const GOPHER_CATEGORY_ID_ADMIN: i32 = -1412641789;
pub const GOPHER_CATEGORY_ID_ALL: i32 = -1412641791;
pub const GOPHER_CATEGORY_ID_ASK: i32 = -1412641785;
pub const GOPHER_CATEGORY_ID_INFO: i32 = -1412641790;
pub const GOPHER_CATEGORY_ID_UNKNOWN: i32 = -1412641784;
pub const GOPHER_CATEGORY_ID_VERONICA: i32 = -1412641786;
pub const GOPHER_CATEGORY_ID_VIEWS: i32 = -1412641788;
#[cfg(feature = "minwindef")]
pub type GOPHER_FIND_DATA = GOPHER_FIND_DATAA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_FIND_DATAA {
    pub DisplayString: [i8; 129],
    pub GopherType: u32,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::minwindef::FILETIME,
    pub Locator: [i8; 654],
}
#[cfg(feature = "minwindef")]
impl Default for GOPHER_FIND_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_FIND_DATAW {
    pub DisplayString: [u16; 129],
    pub GopherType: u32,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::minwindef::FILETIME,
    pub Locator: [u16; 654],
}
#[cfg(feature = "minwindef")]
impl Default for GOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    pub DegreesNorth: i32,
    pub MinutesNorth: i32,
    pub SecondsNorth: i32,
    pub DegreesEast: i32,
    pub MinutesEast: i32,
    pub SecondsEast: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_LOCATION_ATTRIBUTE_TYPE {
    pub Location: super::winnt::LPCTSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    pub DateAndTime: super::minwindef::FILETIME,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    pub Organization: super::winnt::LPCTSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    pub Provider: super::winnt::LPCTSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_SCORE_ATTRIBUTE_TYPE {
    pub Score: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    pub LowerBound: i32,
    pub UpperBound: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_SITE_ATTRIBUTE_TYPE {
    pub Site: super::winnt::LPCTSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    pub Zone: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_TTL_ATTRIBUTE_TYPE {
    pub Ttl: u32,
}
pub const GOPHER_TYPE_ASK: u32 = 1073741824;
pub const GOPHER_TYPE_BINARY: u32 = 512;
pub const GOPHER_TYPE_BITMAP: u32 = 16384;
pub const GOPHER_TYPE_CALENDAR: u32 = 524288;
pub const GOPHER_TYPE_CSO: u32 = 4;
pub const GOPHER_TYPE_DIRECTORY: u32 = 2;
pub const GOPHER_TYPE_DOS_ARCHIVE: u32 = 32;
pub const GOPHER_TYPE_ERROR: u32 = 8;
pub const GOPHER_TYPE_FILE_MASK: u32 = 2093681;
pub const GOPHER_TYPE_GIF: u32 = 4096;
pub const GOPHER_TYPE_GOPHER_PLUS: u32 = 2147483648;
pub const GOPHER_TYPE_HTML: u32 = 131072;
pub const GOPHER_TYPE_IMAGE: u32 = 8192;
pub const GOPHER_TYPE_INDEX_SERVER: u32 = 128;
pub const GOPHER_TYPE_INLINE: u32 = 1048576;
pub const GOPHER_TYPE_MAC_BINHEX: u32 = 16;
pub const GOPHER_TYPE_MOVIE: u32 = 32768;
pub const GOPHER_TYPE_PDF: u32 = 262144;
pub const GOPHER_TYPE_REDUNDANT: u32 = 1024;
pub const GOPHER_TYPE_SOUND: u32 = 65536;
pub const GOPHER_TYPE_TELNET: u32 = 256;
pub const GOPHER_TYPE_TEXT_FILE: u32 = 1;
pub const GOPHER_TYPE_TN3270: u32 = 2048;
pub const GOPHER_TYPE_UNIX_UUENCODED: u32 = 64;
pub const GOPHER_TYPE_UNKNOWN: u32 = 536870912;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    pub Text: super::winnt::LPCTSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_VERONICA_ATTRIBUTE_TYPE {
    pub TreeWalk: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_VERSION_ATTRIBUTE_TYPE {
    pub Version: super::winnt::LPCTSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_VIEW_ATTRIBUTE_TYPE {
    pub ContentType: super::winnt::LPCTSTR,
    pub Language: super::winnt::LPCTSTR,
    pub Size: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GROUPID(pub i64);
pub const GROUPNAME_MAX_LENGTH: u32 = 120;
pub const GROUP_OWNER_STORAGE_SIZE: u32 = 4;
pub const HSR_ASYNC: u32 = 1;
pub const HSR_CHUNKED: u32 = 32;
pub const HSR_DOWNLOAD: u32 = 16;
pub const HSR_INITIATE: u32 = 8;
pub const HSR_SYNC: u32 = 4;
pub const HSR_USE_CONTEXT: u32 = 8;
pub const HTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760;
pub const HTTP_ADDREQ_FLAG_ADD: u32 = 536870912;
pub const HTTP_ADDREQ_FLAG_ADD_IF_NEW: u32 = 268435456;
pub const HTTP_ADDREQ_FLAG_COALESCE: u32 = 1073741824;
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: u32 = 1073741824;
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: u32 = 16777216;
pub const HTTP_ADDREQ_FLAG_REPLACE: u32 = 2147483648;
pub const HTTP_ADDREQ_INDEX_MASK: u32 = 65535;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE: u32 = 3;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE_LAX: u32 = 2;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_MAX: u32 = 3;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_SAME_SITE: u32 = 1;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_UNKNOWN: u32 = 0;
pub const HTTP_MAJOR_VERSION: u32 = 1;
pub const HTTP_MINOR_VERSION: u32 = 0;
pub const HTTP_PROTOCOL_FLAG_HTTP2: u32 = 2;
pub const HTTP_PROTOCOL_MASK: u32 = 2;
pub const HTTP_QUERY_ACCEPT: u32 = 24;
pub const HTTP_QUERY_ACCEPT_CHARSET: u32 = 25;
pub const HTTP_QUERY_ACCEPT_ENCODING: u32 = 26;
pub const HTTP_QUERY_ACCEPT_LANGUAGE: u32 = 27;
pub const HTTP_QUERY_ACCEPT_RANGES: u32 = 42;
pub const HTTP_QUERY_AGE: u32 = 48;
pub const HTTP_QUERY_ALLOW: u32 = 7;
pub const HTTP_QUERY_AUTHENTICATION_INFO: u32 = 76;
pub const HTTP_QUERY_AUTHORIZATION: u32 = 28;
pub const HTTP_QUERY_CACHE_CONTROL: u32 = 49;
pub const HTTP_QUERY_CONNECTION: u32 = 23;
pub const HTTP_QUERY_CONTENT_BASE: u32 = 50;
pub const HTTP_QUERY_CONTENT_DESCRIPTION: u32 = 4;
pub const HTTP_QUERY_CONTENT_DISPOSITION: u32 = 47;
pub const HTTP_QUERY_CONTENT_ENCODING: u32 = 29;
pub const HTTP_QUERY_CONTENT_ID: u32 = 3;
pub const HTTP_QUERY_CONTENT_LANGUAGE: u32 = 6;
pub const HTTP_QUERY_CONTENT_LENGTH: u32 = 5;
pub const HTTP_QUERY_CONTENT_LOCATION: u32 = 51;
pub const HTTP_QUERY_CONTENT_MD5: u32 = 52;
pub const HTTP_QUERY_CONTENT_RANGE: u32 = 53;
pub const HTTP_QUERY_CONTENT_TRANSFER_ENCODING: u32 = 2;
pub const HTTP_QUERY_CONTENT_TYPE: u32 = 1;
pub const HTTP_QUERY_COOKIE: u32 = 44;
pub const HTTP_QUERY_COST: u32 = 15;
pub const HTTP_QUERY_CUSTOM: u32 = 65535;
pub const HTTP_QUERY_DATE: u32 = 9;
pub const HTTP_QUERY_DEFAULT_STYLE: u32 = 84;
pub const HTTP_QUERY_DERIVED_FROM: u32 = 14;
pub const HTTP_QUERY_DO_NOT_TRACK: u32 = 88;
pub const HTTP_QUERY_ECHO_HEADERS: u32 = 73;
pub const HTTP_QUERY_ECHO_HEADERS_CRLF: u32 = 74;
pub const HTTP_QUERY_ECHO_REPLY: u32 = 72;
pub const HTTP_QUERY_ECHO_REQUEST: u32 = 71;
pub const HTTP_QUERY_ETAG: u32 = 54;
pub const HTTP_QUERY_EXPECT: u32 = 68;
pub const HTTP_QUERY_EXPIRES: u32 = 10;
pub const HTTP_QUERY_FLAG_COALESCE: u32 = 268435456;
pub const HTTP_QUERY_FLAG_COALESCE_WITH_COMMA: u32 = 67108864;
pub const HTTP_QUERY_FLAG_NUMBER: u32 = 536870912;
pub const HTTP_QUERY_FLAG_NUMBER64: u32 = 134217728;
pub const HTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648;
pub const HTTP_QUERY_FLAG_SYSTEMTIME: u32 = 1073741824;
pub const HTTP_QUERY_FORWARDED: u32 = 30;
pub const HTTP_QUERY_FROM: u32 = 31;
pub const HTTP_QUERY_HEADER_MASK: u32 = 67108863;
pub const HTTP_QUERY_HOST: u32 = 55;
pub const HTTP_QUERY_HTTP2_SETTINGS: u32 = 90;
pub const HTTP_QUERY_IF_MATCH: u32 = 56;
pub const HTTP_QUERY_IF_MODIFIED_SINCE: u32 = 32;
pub const HTTP_QUERY_IF_NONE_MATCH: u32 = 57;
pub const HTTP_QUERY_IF_RANGE: u32 = 58;
pub const HTTP_QUERY_IF_UNMODIFIED_SINCE: u32 = 59;
pub const HTTP_QUERY_INCLUDE_REFERER_TOKEN_BINDING_ID: u32 = 93;
pub const HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID: u32 = 93;
pub const HTTP_QUERY_KEEP_ALIVE: u32 = 89;
pub const HTTP_QUERY_LAST_MODIFIED: u32 = 11;
pub const HTTP_QUERY_LINK: u32 = 16;
pub const HTTP_QUERY_LOCATION: u32 = 33;
pub const HTTP_QUERY_MAX: u32 = 95;
pub const HTTP_QUERY_MAX_FORWARDS: u32 = 60;
pub const HTTP_QUERY_MESSAGE_ID: u32 = 12;
pub const HTTP_QUERY_MIME_VERSION: u32 = 0;
pub const HTTP_QUERY_MODIFIER_FLAGS_MASK: i32 = -67108864;
pub const HTTP_QUERY_ORIG_URI: u32 = 34;
pub const HTTP_QUERY_P3P: u32 = 80;
pub const HTTP_QUERY_PASSPORT_CONFIG: u32 = 78;
pub const HTTP_QUERY_PASSPORT_URLS: u32 = 77;
pub const HTTP_QUERY_PRAGMA: u32 = 17;
pub const HTTP_QUERY_PROXY_AUTHENTICATE: u32 = 41;
pub const HTTP_QUERY_PROXY_AUTHORIZATION: u32 = 61;
pub const HTTP_QUERY_PROXY_CONNECTION: u32 = 69;
pub const HTTP_QUERY_PROXY_SUPPORT: u32 = 75;
pub const HTTP_QUERY_PUBLIC: u32 = 8;
pub const HTTP_QUERY_PUBLIC_KEY_PINS: u32 = 94;
pub const HTTP_QUERY_PUBLIC_KEY_PINS_REPORT_ONLY: u32 = 95;
pub const HTTP_QUERY_RANGE: u32 = 62;
pub const HTTP_QUERY_RAW_HEADERS: u32 = 21;
pub const HTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22;
pub const HTTP_QUERY_REFERER: u32 = 35;
pub const HTTP_QUERY_REFRESH: u32 = 46;
pub const HTTP_QUERY_REQUEST_METHOD: u32 = 45;
pub const HTTP_QUERY_RETRY_AFTER: u32 = 36;
pub const HTTP_QUERY_SERVER: u32 = 37;
pub const HTTP_QUERY_SET_COOKIE: u32 = 43;
pub const HTTP_QUERY_SET_COOKIE2: u32 = 87;
pub const HTTP_QUERY_STATUS_CODE: u32 = 19;
pub const HTTP_QUERY_STATUS_TEXT: u32 = 20;
pub const HTTP_QUERY_STRICT_TRANSPORT_SECURITY: u32 = 91;
pub const HTTP_QUERY_TITLE: u32 = 38;
pub const HTTP_QUERY_TOKEN_BINDING: u32 = 92;
pub const HTTP_QUERY_TRANSFER_ENCODING: u32 = 63;
pub const HTTP_QUERY_TRANSLATE: u32 = 82;
pub const HTTP_QUERY_UNLESS_MODIFIED_SINCE: u32 = 70;
pub const HTTP_QUERY_UPGRADE: u32 = 64;
pub const HTTP_QUERY_URI: u32 = 13;
pub const HTTP_QUERY_USER_AGENT: u32 = 39;
pub const HTTP_QUERY_VARY: u32 = 65;
pub const HTTP_QUERY_VERSION: u32 = 18;
pub const HTTP_QUERY_VIA: u32 = 66;
pub const HTTP_QUERY_WARNING: u32 = 67;
pub const HTTP_QUERY_WWW_AUTHENTICATE: u32 = 40;
pub const HTTP_QUERY_X_CONTENT_TYPE_OPTIONS: u32 = 79;
pub const HTTP_QUERY_X_FRAME_OPTIONS: u32 = 85;
pub const HTTP_QUERY_X_P2P_PEERDIST: u32 = 81;
pub const HTTP_QUERY_X_UA_COMPATIBLE: u32 = 83;
pub const HTTP_QUERY_X_XSS_PROTECTION: u32 = 86;
pub const HTTP_STATUS_MISDIRECTED_REQUEST: u32 = 421;
pub const HTTP_VERSIONA: windows_core::PCSTR = windows_core::s!("HTTP/1.0");
pub const HTTP_VERSIONW: windows_core::PCWSTR = windows_core::w!("HTTP/1.0");
pub const ICU_USERNAME: u32 = 1073741824;
pub const IDSI_FLAG_KEEP_ALIVE: u32 = 1;
pub const IDSI_FLAG_PROXY: u32 = 4;
pub const IDSI_FLAG_SECURE: u32 = 2;
pub const IDSI_FLAG_TUNNEL: u32 = 8;
pub const INTERENT_GOONLINE_MASK: u32 = 3;
pub const INTERENT_GOONLINE_NOPROMPT: u32 = 2;
pub const INTERENT_GOONLINE_REFRESH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct INTERNET_AUTH_NOTIFY_DATA {
    pub cbStruct: u32,
    pub dwOptions: u32,
    pub pfnNotify: PFN_AUTH_NOTIFY,
    pub dwContext: usize,
}
pub const INTERNET_AUTODIAL_FAILIFSECURITYCHECK: u32 = 4;
pub const INTERNET_AUTODIAL_FLAGS_MASK: u32 = 15;
pub const INTERNET_AUTODIAL_FORCE_ONLINE: u32 = 1;
pub const INTERNET_AUTODIAL_FORCE_UNATTENDED: u32 = 2;
pub const INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT: u32 = 8;
pub type INTERNET_BUFFERS = INTERNET_BUFFERSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_BUFFERSA {
    pub dwStructSize: u32,
    pub Next: *mut Self,
    pub lpcszHeader: windows_core::PCSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
impl Default for INTERNET_BUFFERSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_BUFFERSW {
    pub dwStructSize: u32,
    pub Next: *mut Self,
    pub lpcszHeader: windows_core::PCWSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
impl Default for INTERNET_BUFFERSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type INTERNET_CACHE_ENTRY_INFO = INTERNET_CACHE_ENTRY_INFOA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct INTERNET_CACHE_ENTRY_INFOA {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: windows_core::PSTR,
    pub lpszLocalFileName: windows_core::PSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::minwindef::FILETIME,
    pub ExpireTime: super::minwindef::FILETIME,
    pub LastAccessTime: super::minwindef::FILETIME,
    pub LastSyncTime: super::minwindef::FILETIME,
    pub lpHeaderInfo: windows_core::PSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: windows_core::PSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOA_0,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_CACHE_ENTRY_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union INTERNET_CACHE_ENTRY_INFOA_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_CACHE_ENTRY_INFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct INTERNET_CACHE_ENTRY_INFOW {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: windows_core::PWSTR,
    pub lpszLocalFileName: windows_core::PWSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::minwindef::FILETIME,
    pub ExpireTime: super::minwindef::FILETIME,
    pub LastAccessTime: super::minwindef::FILETIME,
    pub LastSyncTime: super::minwindef::FILETIME,
    pub lpHeaderInfo: windows_core::PWSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: windows_core::PWSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOW_0,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_CACHE_ENTRY_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union INTERNET_CACHE_ENTRY_INFOW_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_CACHE_ENTRY_INFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_CACHE_GROUP_ADD: u32 = 0;
pub type INTERNET_CACHE_GROUP_INFO = INTERNET_CACHE_GROUP_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_GROUP_INFOA {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [i8; 120],
}
impl Default for INTERNET_CACHE_GROUP_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_GROUP_INFOW {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [u16; 120],
}
impl Default for INTERNET_CACHE_GROUP_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_CACHE_GROUP_REMOVE: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CACHE_TIMESTAMPS {
    pub ftExpires: super::minwindef::FILETIME,
    pub ftLastModified: super::minwindef::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CERTIFICATE_INFO {
    pub ftExpiry: super::minwindef::FILETIME,
    pub ftStart: super::minwindef::FILETIME,
    pub lpszSubjectInfo: super::winnt::LPTSTR,
    pub lpszIssuerInfo: super::winnt::LPTSTR,
    pub lpszProtocolName: super::winnt::LPTSTR,
    pub lpszSignatureAlgName: super::winnt::LPTSTR,
    pub lpszEncryptionAlgName: super::winnt::LPTSTR,
    pub dwKeySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CONNECTED_INFO {
    pub dwConnectedState: u32,
    pub dwFlags: u32,
}
pub const INTERNET_CONNECTION_CONFIGURED: u32 = 64;
pub const INTERNET_CONNECTION_LAN: u32 = 2;
pub const INTERNET_CONNECTION_MODEM: u32 = 1;
pub const INTERNET_CONNECTION_MODEM_BUSY: u32 = 8;
pub const INTERNET_CONNECTION_OFFLINE: u32 = 32;
pub const INTERNET_CONNECTION_PROXY: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_COOKIE2 {
    pub pwszName: windows_core::PWSTR,
    pub pwszValue: windows_core::PWSTR,
    pub pwszDomain: windows_core::PWSTR,
    pub pwszPath: windows_core::PWSTR,
    pub dwFlags: u32,
    pub ftExpires: super::minwindef::FILETIME,
    pub fExpiresSet: windows_core::BOOL,
}
pub const INTERNET_COOKIE_APPLY_HOST_ONLY: u32 = 32768;
pub const INTERNET_COOKIE_APPLY_P3P: u32 = 128;
pub const INTERNET_COOKIE_EVALUATE_P3P: u32 = 64;
pub const INTERNET_COOKIE_HOST_ONLY: u32 = 16384;
pub const INTERNET_COOKIE_HOST_ONLY_APPLIED: u32 = 524288;
pub const INTERNET_COOKIE_HTTPONLY: u32 = 8192;
pub const INTERNET_COOKIE_IE6: u32 = 1024;
pub const INTERNET_COOKIE_IS_LEGACY: u32 = 2048;
pub const INTERNET_COOKIE_IS_RESTRICTED: u32 = 512;
pub const INTERNET_COOKIE_IS_SECURE: u32 = 1;
pub const INTERNET_COOKIE_IS_SESSION: u32 = 2;
pub const INTERNET_COOKIE_NON_SCRIPT: u32 = 4096;
pub const INTERNET_COOKIE_P3P_ENABLED: u32 = 256;
pub const INTERNET_COOKIE_PROMPT_REQUIRED: u32 = 32;
pub const INTERNET_COOKIE_SAME_SITE_LAX: u32 = 2097152;
pub const INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE: u32 = 4194304;
pub const INTERNET_COOKIE_SAME_SITE_STRICT: u32 = 1048576;
pub const INTERNET_COOKIE_THIRD_PARTY: u32 = 16;
pub const INTERNET_CUSTOMDIAL_CAN_HANGUP: u32 = 4;
pub const INTERNET_CUSTOMDIAL_CONNECT: u32 = 0;
pub const INTERNET_CUSTOMDIAL_DISCONNECT: u32 = 2;
pub const INTERNET_CUSTOMDIAL_SAFE_FOR_UNATTENDED: u32 = 1;
pub const INTERNET_CUSTOMDIAL_SHOWOFFLINE: u32 = 4;
pub const INTERNET_CUSTOMDIAL_UNATTENDED: u32 = 1;
pub const INTERNET_CUSTOMDIAL_WILL_SUPPLY_STATE: u32 = 2;
pub const INTERNET_DEFAULT_FTP_PORT: u32 = 21;
pub const INTERNET_DEFAULT_GOPHER_PORT: u32 = 70;
pub const INTERNET_DEFAULT_SOCKS_PORT: u32 = 1080;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_DIAGNOSTIC_SOCKET_INFO {
    pub Socket: usize,
    pub SourcePort: u32,
    pub DestPort: u32,
    pub Flags: u32,
}
pub const INTERNET_DIALSTATE_DISCONNECTED: u32 = 1;
pub const INTERNET_DIAL_FORCE_PROMPT: u32 = 8192;
pub const INTERNET_DIAL_SHOW_OFFLINE: u32 = 16384;
pub const INTERNET_DIAL_UNATTENDED: u32 = 32768;
pub const INTERNET_ERROR_BASE: u32 = 12000;
pub const INTERNET_ERROR_LAST: u32 = 12192;
pub const INTERNET_ERROR_MASK_COMBINED_SEC_CERT: u32 = 2;
pub const INTERNET_ERROR_MASK_INSERT_CDROM: u32 = 1;
pub const INTERNET_ERROR_MASK_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 8;
pub const INTERNET_ERROR_MASK_NEED_MSN_SSPI_PKG: u32 = 4;
pub const INTERNET_FIRST_OPTION: u32 = 1;
pub const INTERNET_FLAG_ASYNC: u32 = 268435456;
pub const INTERNET_FLAG_CACHE_ASYNC: u32 = 128;
pub const INTERNET_FLAG_CACHE_IF_NET_FAIL: u32 = 65536;
pub const INTERNET_FLAG_DONT_CACHE: u32 = 67108864;
pub const INTERNET_FLAG_EXISTING_CONNECT: u32 = 536870912;
pub const INTERNET_FLAG_FORMS_SUBMIT: u32 = 64;
pub const INTERNET_FLAG_FROM_CACHE: u32 = 16777216;
pub const INTERNET_FLAG_FWD_BACK: u32 = 32;
pub const INTERNET_FLAG_HYPERLINK: u32 = 1024;
pub const INTERNET_FLAG_IDN_DIRECT: u32 = 1;
pub const INTERNET_FLAG_IDN_PROXY: u32 = 2;
pub const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: u32 = 4096;
pub const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 8192;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384;
pub const INTERNET_FLAG_KEEP_CONNECTION: u32 = 4194304;
pub const INTERNET_FLAG_MAKE_PERSISTENT: u32 = 33554432;
pub const INTERNET_FLAG_MUST_CACHE_REQUEST: u32 = 16;
pub const INTERNET_FLAG_NEED_FILE: u32 = 16;
pub const INTERNET_FLAG_NO_AUTH: u32 = 262144;
pub const INTERNET_FLAG_NO_AUTO_REDIRECT: u32 = 2097152;
pub const INTERNET_FLAG_NO_CACHE_WRITE: u32 = 67108864;
pub const INTERNET_FLAG_NO_COOKIES: u32 = 524288;
pub const INTERNET_FLAG_NO_UI: u32 = 512;
pub const INTERNET_FLAG_OFFLINE: u32 = 16777216;
pub const INTERNET_FLAG_PASSIVE: u32 = 134217728;
pub const INTERNET_FLAG_PRAGMA_NOCACHE: u32 = 256;
pub const INTERNET_FLAG_RAW_DATA: u32 = 1073741824;
pub const INTERNET_FLAG_READ_PREFETCH: u32 = 1048576;
pub const INTERNET_FLAG_RELOAD: u32 = 2147483648;
pub const INTERNET_FLAG_RESTRICTED_ZONE: u32 = 131072;
pub const INTERNET_FLAG_RESYNCHRONIZE: u32 = 2048;
pub const INTERNET_FLAG_SECURE: u32 = 8388608;
pub const INTERNET_FLAG_TRANSFER_ASCII: u32 = 1;
pub const INTERNET_FLAG_TRANSFER_BINARY: u32 = 2;
pub const INTERNET_HANDLE_TYPE_CONNECT_FTP: u32 = 2;
pub const INTERNET_HANDLE_TYPE_CONNECT_GOPHER: u32 = 3;
pub const INTERNET_HANDLE_TYPE_CONNECT_HTTP: u32 = 4;
pub const INTERNET_HANDLE_TYPE_FILE_REQUEST: u32 = 14;
pub const INTERNET_HANDLE_TYPE_FTP_FILE: u32 = 7;
pub const INTERNET_HANDLE_TYPE_FTP_FILE_HTML: u32 = 8;
pub const INTERNET_HANDLE_TYPE_FTP_FIND: u32 = 5;
pub const INTERNET_HANDLE_TYPE_FTP_FIND_HTML: u32 = 6;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE: u32 = 11;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE_HTML: u32 = 12;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND: u32 = 9;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND_HTML: u32 = 10;
pub const INTERNET_HANDLE_TYPE_HTTP_REQUEST: u32 = 13;
pub const INTERNET_HANDLE_TYPE_INTERNET: u32 = 1;
pub const INTERNET_IDENTITY_FLAG_CLEAR_CONTENT: u32 = 32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_COOKIES: u32 = 8;
pub const INTERNET_IDENTITY_FLAG_CLEAR_DATA: u32 = 4;
pub const INTERNET_IDENTITY_FLAG_CLEAR_HISTORY: u32 = 16;
pub const INTERNET_IDENTITY_FLAG_PRIVATE_CACHE: u32 = 1;
pub const INTERNET_IDENTITY_FLAG_SHARED_CACHE: u32 = 2;
pub const INTERNET_INVALID_PORT_NUMBER: u32 = 0;
pub const INTERNET_KEEP_ALIVE_DISABLED: u32 = 0;
pub const INTERNET_KEEP_ALIVE_ENABLED: u32 = 1;
pub const INTERNET_KEEP_ALIVE_UNKNOWN: u32 = 4294967295;
pub const INTERNET_LAST_OPTION: u32 = 193;
pub const INTERNET_MAX_HOST_NAME_LENGTH: u32 = 256;
pub const INTERNET_MAX_PASSWORD_LENGTH: u32 = 128;
pub const INTERNET_MAX_PATH_LENGTH: u32 = 2048;
pub const INTERNET_MAX_PORT_NUMBER_LENGTH: u32 = 5;
pub const INTERNET_MAX_PORT_NUMBER_VALUE: u32 = 65535;
pub const INTERNET_MAX_SCHEME_LENGTH: u32 = 32;
pub const INTERNET_MAX_USER_NAME_LENGTH: u32 = 128;
pub const INTERNET_NO_CALLBACK: u32 = 0;
pub const INTERNET_OPEN_TYPE_DIRECT: u32 = 1;
pub const INTERNET_OPEN_TYPE_PRECONFIG: u32 = 0;
pub const INTERNET_OPEN_TYPE_PRECONFIG_WITH_NO_AUTOPROXY: u32 = 4;
pub const INTERNET_OPEN_TYPE_PROXY: u32 = 3;
pub const INTERNET_OPTION_ACTIVATE_WORKER_THREADS: u32 = 92;
pub const INTERNET_OPTION_ALTER_IDENTITY: u32 = 80;
pub const INTERNET_OPTION_ASYNC: u32 = 30;
pub const INTERNET_OPTION_ASYNC_ID: u32 = 15;
pub const INTERNET_OPTION_ASYNC_PRIORITY: u32 = 16;
pub const INTERNET_OPTION_AUTH_FLAGS: u32 = 85;
pub const INTERNET_OPTION_AUTODIAL_CONNECTION: u32 = 83;
pub const INTERNET_OPTION_AUTODIAL_MODE: u32 = 82;
pub const INTERNET_OPTION_BYPASS_EDITED_ENTRY: u32 = 64;
pub const INTERNET_OPTION_CACHE_STREAM_HANDLE: u32 = 27;
pub const INTERNET_OPTION_CACHE_TIMESTAMPS: u32 = 69;
pub const INTERNET_OPTION_CALLBACK: u32 = 1;
pub const INTERNET_OPTION_CALLBACK_FILTER: u32 = 54;
pub const INTERNET_OPTION_CLIENT_CERT_CONTEXT: u32 = 84;
pub const INTERNET_OPTION_CODEPAGE: u32 = 68;
pub const INTERNET_OPTION_CODEPAGE_EXTRA: u32 = 101;
pub const INTERNET_OPTION_CODEPAGE_PATH: u32 = 100;
pub const INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH: u32 = 147;
pub const INTERNET_OPTION_CONNECTED_STATE: u32 = 50;
pub const INTERNET_OPTION_CONNECTION_FILTER: u32 = 162;
pub const INTERNET_OPTION_CONNECT_BACKOFF: u32 = 4;
pub const INTERNET_OPTION_CONNECT_LIMIT: u32 = 46;
pub const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3;
pub const INTERNET_OPTION_CONNECT_TIME: u32 = 55;
pub const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2;
pub const INTERNET_OPTION_CONTEXT_VALUE: u32 = 45;
pub const INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT: u32 = 6;
pub const INTERNET_OPTION_CONTROL_SEND_TIMEOUT: u32 = 5;
pub const INTERNET_OPTION_COOKIES_3RD_PARTY: u32 = 86;
pub const INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL: u32 = 187;
pub const INTERNET_OPTION_DATAFILE_EXT: u32 = 96;
pub const INTERNET_OPTION_DATAFILE_NAME: u32 = 33;
pub const INTERNET_OPTION_DATA_RECEIVE_TIMEOUT: u32 = 8;
pub const INTERNET_OPTION_DATA_SEND_TIMEOUT: u32 = 7;
pub const INTERNET_OPTION_DIAGNOSTIC_SOCKET_INFO: u32 = 67;
pub const INTERNET_OPTION_DIGEST_AUTH_UNLOAD: u32 = 76;
pub const INTERNET_OPTION_DISABLE_AUTODIAL: u32 = 70;
pub const INTERNET_OPTION_DISABLE_PASSPORT_AUTH: u32 = 87;
pub const INTERNET_OPTION_DISCONNECTED_TIMEOUT: u32 = 49;
pub const INTERNET_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 148;
pub const INTERNET_OPTION_ENABLE_PASSPORT_AUTH: u32 = 90;
pub const INTERNET_OPTION_ENABLE_REDIRECT_CACHE_READ: u32 = 122;
pub const INTERNET_OPTION_ENCODE_EXTRA: u32 = 155;
pub const INTERNET_OPTION_END_BROWSER_SESSION: u32 = 42;
pub const INTERNET_OPTION_ENTERPRISE_CONTEXT: u32 = 159;
pub const INTERNET_OPTION_ERROR_MASK: u32 = 62;
pub const INTERNET_OPTION_EXEMPT_CONNECTION_LIMIT: u32 = 89;
pub const INTERNET_OPTION_EXTENDED_ERROR: u32 = 24;
pub const INTERNET_OPTION_FROM_CACHE_TIMEOUT: u32 = 63;
pub const INTERNET_OPTION_HANDLE_TYPE: u32 = 9;
pub const INTERNET_OPTION_HIBERNATE_INACTIVE_WORKER_THREADS: u32 = 91;
pub const INTERNET_OPTION_HSTS: u32 = 157;
pub const INTERNET_OPTION_HTTP_DECODING: u32 = 65;
pub const INTERNET_OPTION_HTTP_PROTOCOL_USED: u32 = 149;
pub const INTERNET_OPTION_HTTP_VERSION: u32 = 59;
pub const INTERNET_OPTION_IDENTITY: u32 = 78;
pub const INTERNET_OPTION_IDLE_STATE: u32 = 51;
pub const INTERNET_OPTION_IDN: u32 = 102;
pub const INTERNET_OPTION_IGNORE_OFFLINE: u32 = 77;
pub const INTERNET_OPTION_KEEP_CONNECTION: u32 = 22;
pub const INTERNET_OPTION_LISTEN_TIMEOUT: u32 = 11;
pub const INTERNET_OPTION_MAX_CONNS_PER_1_0_SERVER: u32 = 74;
pub const INTERNET_OPTION_MAX_CONNS_PER_PROXY: u32 = 103;
pub const INTERNET_OPTION_MAX_CONNS_PER_SERVER: u32 = 73;
pub const INTERNET_OPTION_OFFLINE_MODE: u32 = 26;
pub const INTERNET_OPTION_OFFLINE_SEMANTICS: u32 = 52;
pub const INTERNET_OPTION_PARENT_HANDLE: u32 = 21;
pub const INTERNET_OPTION_PASSWORD: u32 = 29;
pub const INTERNET_OPTION_PER_CONNECTION_OPTION: u32 = 75;
pub const INTERNET_OPTION_POLICY: u32 = 48;
pub const INTERNET_OPTION_PROXY: u32 = 38;
pub const INTERNET_OPTION_PROXY_PASSWORD: u32 = 44;
pub const INTERNET_OPTION_PROXY_SETTINGS_CHANGED: u32 = 95;
pub const INTERNET_OPTION_PROXY_USERNAME: u32 = 43;
pub const INTERNET_OPTION_READ_BUFFER_SIZE: u32 = 12;
pub const INTERNET_OPTION_RECEIVE_THROUGHPUT: u32 = 57;
pub const INTERNET_OPTION_RECEIVE_TIMEOUT: u32 = 6;
pub const INTERNET_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 163;
pub const INTERNET_OPTION_REFRESH: u32 = 37;
pub const INTERNET_OPTION_REMOVE_IDENTITY: u32 = 79;
pub const INTERNET_OPTION_REQUEST_ANNOTATION: u32 = 193;
pub const INTERNET_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: u32 = 64000;
pub const INTERNET_OPTION_REQUEST_FLAGS: u32 = 23;
pub const INTERNET_OPTION_REQUEST_PRIORITY: u32 = 58;
pub const INTERNET_OPTION_RESET_URLCACHE_SESSION: u32 = 60;
pub const INTERNET_OPTION_RESTORE_WORKER_THREAD_DEFAULTS: u32 = 93;
pub const INTERNET_OPTION_SECONDARY_CACHE_KEY: u32 = 53;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE: u32 = 35;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE_STRUCT: u32 = 32;
pub const INTERNET_OPTION_SECURITY_FLAGS: u32 = 31;
pub const INTERNET_OPTION_SECURITY_KEY_BITNESS: u32 = 36;
pub const INTERNET_OPTION_SECURITY_SELECT_CLIENT_CERT: u32 = 47;
pub const INTERNET_OPTION_SEND_THROUGHPUT: u32 = 56;
pub const INTERNET_OPTION_SEND_TIMEOUT: u32 = 5;
pub const INTERNET_OPTION_SEND_UTF8_SERVERNAME_TO_PROXY: u32 = 88;
pub const INTERNET_OPTION_SERVER_CERT_CHAIN_CONTEXT: u32 = 105;
pub const INTERNET_OPTION_SETTINGS_CHANGED: u32 = 39;
pub const INTERNET_OPTION_SOCKET_SEND_BUFFER_LENGTH: u32 = 94;
pub const INTERNET_OPTION_SUPPRESS_BEHAVIOR: u32 = 81;
pub const INTERNET_OPTION_SUPPRESS_SERVER_AUTH: u32 = 104;
pub const INTERNET_OPTION_TOKEN_BINDING_PUBLIC_KEY: u32 = 181;
pub const INTERNET_OPTION_URL: u32 = 34;
pub const INTERNET_OPTION_USERNAME: u32 = 28;
pub const INTERNET_OPTION_USER_AGENT: u32 = 41;
pub const INTERNET_OPTION_VERSION: u32 = 40;
pub const INTERNET_OPTION_WRITE_BUFFER_SIZE: u32 = 13;
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_TIME: u32 = 8;
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_URL: u32 = 9;
pub const INTERNET_PER_CONN_AUTOCONFIG_RELOAD_DELAY_MINS: u32 = 7;
pub const INTERNET_PER_CONN_AUTOCONFIG_SECONDARY_URL: u32 = 6;
pub const INTERNET_PER_CONN_AUTOCONFIG_URL: u32 = 4;
pub const INTERNET_PER_CONN_AUTODISCOVERY_FLAGS: u32 = 5;
pub const INTERNET_PER_CONN_FLAGS: u32 = 1;
pub const INTERNET_PER_CONN_FLAGS_UI: u32 = 10;
#[cfg(feature = "minwindef")]
pub type INTERNET_PER_CONN_OPTION = INTERNET_PER_CONN_OPTIONA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct INTERNET_PER_CONN_OPTIONA {
    pub dwOption: u32,
    pub Value: INTERNET_PER_CONN_OPTIONA_0,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_PER_CONN_OPTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union INTERNET_PER_CONN_OPTIONA_0 {
    pub dwValue: u32,
    pub pszValue: windows_core::PSTR,
    pub ftValue: super::minwindef::FILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_PER_CONN_OPTIONA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct INTERNET_PER_CONN_OPTIONW {
    pub dwOption: u32,
    pub Value: INTERNET_PER_CONN_OPTIONW_0,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_PER_CONN_OPTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union INTERNET_PER_CONN_OPTIONW_0 {
    pub dwValue: u32,
    pub pszValue: windows_core::PWSTR,
    pub ftValue: super::minwindef::FILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_PER_CONN_OPTIONW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type INTERNET_PER_CONN_OPTION_LIST = INTERNET_PER_CONN_OPTION_LISTA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_PER_CONN_OPTION_LISTA {
    pub dwSize: u32,
    pub pszConnection: windows_core::PSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: LPINTERNET_PER_CONN_OPTIONA,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_PER_CONN_OPTION_LISTW {
    pub dwSize: u32,
    pub pszConnection: windows_core::PWSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: LPINTERNET_PER_CONN_OPTIONW,
}
pub const INTERNET_PER_CONN_PROXY_BYPASS: u32 = 3;
pub const INTERNET_PER_CONN_PROXY_SERVER: u32 = 2;
pub const INTERNET_PRIORITY_FOREGROUND: u32 = 1000;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_PROXY_INFO {
    pub dwAccessType: u32,
    pub lpszProxy: super::winnt::LPCTSTR,
    pub lpszProxyBypass: super::winnt::LPCTSTR,
}
pub const INTERNET_RAS_INSTALLED: u32 = 16;
pub const INTERNET_REQFLAG_ASYNC: u32 = 2;
pub const INTERNET_REQFLAG_CACHE_WRITE_DISABLED: u32 = 64;
pub const INTERNET_REQFLAG_FROM_CACHE: u32 = 1;
pub const INTERNET_REQFLAG_NET_TIMEOUT: u32 = 128;
pub const INTERNET_REQFLAG_NO_HEADERS: u32 = 8;
pub const INTERNET_REQFLAG_PASSIVE: u32 = 16;
pub const INTERNET_REQFLAG_VIA_PROXY: u32 = 4;
pub const INTERNET_RFC1123_BUFSIZE: u32 = 30;
pub const INTERNET_RFC1123_FORMAT: u32 = 0;
pub const INTERNET_SERVICE_FTP: u32 = 1;
pub const INTERNET_SERVICE_GOPHER: u32 = 2;
pub const INTERNET_SERVICE_HTTP: u32 = 3;
pub const INTERNET_STATE_BUSY: u32 = 512;
pub const INTERNET_STATE_CONNECTED: u32 = 1;
pub const INTERNET_STATE_DISCONNECTED: u32 = 2;
pub const INTERNET_STATE_DISCONNECTED_BY_USER: u32 = 16;
pub const INTERNET_STATE_IDLE: u32 = 256;
#[cfg(feature = "winhttp")]
pub type INTERNET_STATUS_CALLBACK = Option<unsafe extern "system" fn(hinternet: super::winhttp::HINTERNET, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *const core::ffi::c_void, dwstatusinformationlength: u32)>;
pub const INTERNET_STATUS_CLOSING_CONNECTION: u32 = 50;
pub const INTERNET_STATUS_CONNECTED_TO_SERVER: u32 = 21;
pub const INTERNET_STATUS_CONNECTING_TO_SERVER: u32 = 20;
pub const INTERNET_STATUS_CONNECTION_CLOSED: u32 = 51;
pub const INTERNET_STATUS_COOKIE_HISTORY: u32 = 327;
pub const INTERNET_STATUS_COOKIE_RECEIVED: u32 = 321;
pub const INTERNET_STATUS_COOKIE_SENT: u32 = 320;
pub const INTERNET_STATUS_CTL_RESPONSE_RECEIVED: u32 = 42;
pub const INTERNET_STATUS_DETECTING_PROXY: u32 = 80;
pub const INTERNET_STATUS_HANDLE_CLOSING: u32 = 70;
pub const INTERNET_STATUS_HANDLE_CREATED: u32 = 60;
pub const INTERNET_STATUS_INTERMEDIATE_RESPONSE: u32 = 120;
pub const INTERNET_STATUS_NAME_RESOLVED: u32 = 11;
pub const INTERNET_STATUS_P3P_HEADER: u32 = 325;
pub const INTERNET_STATUS_P3P_POLICYREF: u32 = 326;
pub const INTERNET_STATUS_PREFETCH: u32 = 43;
pub const INTERNET_STATUS_PRIVACY_IMPACTED: u32 = 324;
pub const INTERNET_STATUS_RECEIVING_RESPONSE: u32 = 40;
pub const INTERNET_STATUS_REDIRECT: u32 = 110;
pub const INTERNET_STATUS_REQUEST_COMPLETE: u32 = 100;
pub const INTERNET_STATUS_REQUEST_SENT: u32 = 31;
pub const INTERNET_STATUS_RESOLVING_NAME: u32 = 10;
pub const INTERNET_STATUS_RESPONSE_RECEIVED: u32 = 41;
pub const INTERNET_STATUS_SENDING_REQUEST: u32 = 30;
pub const INTERNET_STATUS_STATE_CHANGE: u32 = 200;
pub const INTERNET_STATUS_USER_INPUT_REQUIRED: u32 = 140;
pub const INTERNET_SUPPRESS_COOKIE_POLICY: u32 = 1;
pub const INTERNET_SUPPRESS_COOKIE_POLICY_RESET: u32 = 2;
pub const INTERNET_SUPPRESS_RESET_ALL: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
pub const IRF_ASYNC: u32 = 1;
pub const IRF_NO_WAIT: u32 = 8;
pub const IRF_SYNC: u32 = 4;
pub const IRF_USE_CONTEXT: u32 = 8;
pub const ISO_FORCE_DISCONNECTED: u32 = 1;
pub const ISO_GLOBAL: u32 = 1;
pub const ISO_REGISTRY: u32 = 2;
pub const ISO_VALID_FLAGS: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IncomingCookieState {
    pub cSession: i32,
    pub cPersistent: i32,
    pub cAccepted: i32,
    pub cLeashed: i32,
    pub cDowngraded: i32,
    pub cBlocked: i32,
    pub pszLocation: *const i8,
}
impl Default for IncomingCookieState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InternetCookieHistory {
    pub fAccepted: windows_core::BOOL,
    pub fLeashed: windows_core::BOOL,
    pub fDowngraded: windows_core::BOOL,
    pub fRejected: windows_core::BOOL,
}
pub type InternetCookieState = i32;
pub const LOCAL_INTERNET_ACCESS: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUTO_PROXY_SCRIPT_BUFFER(pub *mut AUTO_PROXY_SCRIPT_BUFFER);
impl LPAUTO_PROXY_SCRIPT_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPAUTO_PROXY_SCRIPT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_ABSTRACT_ATTRIBUTE_TYPE(pub *mut GOPHER_ABSTRACT_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_ADMIN_ATTRIBUTE_TYPE(pub *mut GOPHER_ADMIN_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_ADMIN_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_ASK_ATTRIBUTE_TYPE(pub *mut GOPHER_ASK_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_ASK_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_ASK_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_ATTRIBUTE_TYPE(pub *mut GOPHER_ATTRIBUTE_TYPE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl LPGOPHER_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for LPGOPHER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPGOPHER_FIND_DATA(pub LPGOPHER_FIND_DATAA);
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_FIND_DATAA(pub *mut GOPHER_FIND_DATAA);
#[cfg(feature = "minwindef")]
impl LPGOPHER_FIND_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPGOPHER_FIND_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_FIND_DATAW(pub *mut GOPHER_FIND_DATAW);
#[cfg(feature = "minwindef")]
impl LPGOPHER_FIND_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPGOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE(pub *mut GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE);
impl LPGOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_LOCATION_ATTRIBUTE_TYPE(pub *mut GOPHER_LOCATION_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_LOCATION_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_MOD_DATE_ATTRIBUTE_TYPE(pub *mut GOPHER_MOD_DATE_ATTRIBUTE_TYPE);
#[cfg(feature = "minwindef")]
impl LPGOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPGOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_ORGANIZATION_ATTRIBUTE_TYPE(pub *mut GOPHER_ORGANIZATION_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_PROVIDER_ATTRIBUTE_TYPE(pub *mut GOPHER_PROVIDER_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_PROVIDER_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_SCORE_ATTRIBUTE_TYPE(pub *mut GOPHER_SCORE_ATTRIBUTE_TYPE);
impl LPGOPHER_SCORE_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGOPHER_SCORE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_SCORE_RANGE_ATTRIBUTE_TYPE(pub *mut GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE);
impl LPGOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_SITE_ATTRIBUTE_TYPE(pub *mut GOPHER_SITE_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_SITE_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_SITE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_TIMEZONE_ATTRIBUTE_TYPE(pub *mut GOPHER_TIMEZONE_ATTRIBUTE_TYPE);
impl LPGOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_TTL_ATTRIBUTE_TYPE(pub *mut GOPHER_TTL_ATTRIBUTE_TYPE);
impl LPGOPHER_TTL_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGOPHER_TTL_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_UNKNOWN_ATTRIBUTE_TYPE(pub *mut GOPHER_UNKNOWN_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_VERONICA_ATTRIBUTE_TYPE(pub *mut GOPHER_VERONICA_ATTRIBUTE_TYPE);
impl LPGOPHER_VERONICA_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_VERSION_ATTRIBUTE_TYPE(pub *mut GOPHER_VERSION_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_VERSION_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_VERSION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGOPHER_VIEW_ATTRIBUTE_TYPE(pub *mut GOPHER_VIEW_ATTRIBUTE_TYPE);
#[cfg(feature = "winnt")]
impl LPGOPHER_VIEW_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPGOPHER_VIEW_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_ASYNC_RESULT(pub *mut INTERNET_ASYNC_RESULT);
impl LPINTERNET_ASYNC_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPINTERNET_BUFFERS(pub LPINTERNET_BUFFERSA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_BUFFERSA(pub *mut INTERNET_BUFFERSA);
impl LPINTERNET_BUFFERSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_BUFFERSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_BUFFERSW(pub *mut INTERNET_BUFFERSW);
impl LPINTERNET_BUFFERSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_BUFFERSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPINTERNET_CACHE_ENTRY_INFO(pub LPINTERNET_CACHE_ENTRY_INFOA);
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CACHE_ENTRY_INFOA(pub *mut INTERNET_CACHE_ENTRY_INFOA);
#[cfg(feature = "minwindef")]
impl LPINTERNET_CACHE_ENTRY_INFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_CACHE_ENTRY_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CACHE_ENTRY_INFOW(pub *mut INTERNET_CACHE_ENTRY_INFOW);
#[cfg(feature = "minwindef")]
impl LPINTERNET_CACHE_ENTRY_INFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_CACHE_ENTRY_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPINTERNET_CACHE_GROUP_INFO(pub LPINTERNET_CACHE_GROUP_INFOA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CACHE_GROUP_INFOA(pub *mut INTERNET_CACHE_GROUP_INFOA);
impl LPINTERNET_CACHE_GROUP_INFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_CACHE_GROUP_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CACHE_GROUP_INFOW(pub *mut INTERNET_CACHE_GROUP_INFOW);
impl LPINTERNET_CACHE_GROUP_INFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_CACHE_GROUP_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CACHE_TIMESTAMPS(pub *mut INTERNET_CACHE_TIMESTAMPS);
#[cfg(feature = "minwindef")]
impl LPINTERNET_CACHE_TIMESTAMPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_CACHE_TIMESTAMPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CERTIFICATE_INFO(pub *mut INTERNET_CERTIFICATE_INFO);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl LPINTERNET_CERTIFICATE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for LPINTERNET_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_CONNECTED_INFO(pub *mut INTERNET_CONNECTED_INFO);
impl LPINTERNET_CONNECTED_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_CONNECTED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_DIAGNOSTIC_SOCKET_INFO(pub *mut INTERNET_DIAGNOSTIC_SOCKET_INFO);
impl LPINTERNET_DIAGNOSTIC_SOCKET_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPINTERNET_PER_CONN_OPTION(pub LPINTERNET_PER_CONN_OPTIONA);
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_PER_CONN_OPTIONA(pub *mut INTERNET_PER_CONN_OPTIONA);
#[cfg(feature = "minwindef")]
impl LPINTERNET_PER_CONN_OPTIONA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_PER_CONN_OPTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_PER_CONN_OPTIONW(pub *mut INTERNET_PER_CONN_OPTIONW);
#[cfg(feature = "minwindef")]
impl LPINTERNET_PER_CONN_OPTIONW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_PER_CONN_OPTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPINTERNET_PER_CONN_OPTION_LIST(pub LPINTERNET_PER_CONN_OPTION_LISTA);
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_PER_CONN_OPTION_LISTA(pub *mut INTERNET_PER_CONN_OPTION_LISTA);
#[cfg(feature = "minwindef")]
impl LPINTERNET_PER_CONN_OPTION_LISTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_PER_CONN_OPTION_LISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_PER_CONN_OPTION_LISTW(pub *mut INTERNET_PER_CONN_OPTION_LISTW);
#[cfg(feature = "minwindef")]
impl LPINTERNET_PER_CONN_OPTION_LISTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPINTERNET_PER_CONN_OPTION_LISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_PROXY_INFO(pub *mut INTERNET_PROXY_INFO);
#[cfg(feature = "winnt")]
impl LPINTERNET_PROXY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPINTERNET_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winhttp")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_STATUS_CALLBACK(pub *mut INTERNET_STATUS_CALLBACK);
#[cfg(feature = "winhttp")]
impl LPINTERNET_STATUS_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winhttp")]
impl Default for LPINTERNET_STATUS_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERNET_VERSION_INFO(pub *mut INTERNET_VERSION_INFO);
impl LPINTERNET_VERSION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERNET_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winhttp")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPURL_COMPONENTSA(pub *mut URL_COMPONENTSA);
#[cfg(feature = "winhttp")]
impl LPURL_COMPONENTSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winhttp")]
impl Default for LPURL_COMPONENTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_GOPHER_ATTRIBUTE_NAME: u32 = 128;
pub const MAX_GOPHER_CATEGORY_NAME: u32 = 128;
pub const MAX_GOPHER_DISPLAY_TEXT: u32 = 128;
pub const MAX_GOPHER_HOST_NAME: u32 = 256;
pub const MAX_GOPHER_LOCATOR_LENGTH: u32 = 653;
pub const MAX_GOPHER_SELECTOR_TEXT: u32 = 256;
pub const MIN_GOPHER_ATTRIBUTE_LENGTH: u32 = 256;
pub const NORMAL_CACHE_ENTRY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutgoingCookieState {
    pub cSent: i32,
    pub cSuppressed: i32,
    pub pszLocation: *const i8,
}
impl Default for OutgoingCookieState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_AUTH_NOTIFY = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "windef")]
pub type PFN_DIAL_HANDLER = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: windows_core::PCSTR, param2: u32, param3: *mut u32) -> u32>;
pub const PRE_CONFIG_INTERNET_ACCESS: u32 = 0;
pub const PRIVACY_TEMPLATE_ADVANCED: u32 = 101;
pub const PRIVACY_TEMPLATE_CUSTOM: u32 = 100;
pub const PRIVACY_TEMPLATE_HIGH: u32 = 1;
pub const PRIVACY_TEMPLATE_LOW: u32 = 5;
pub const PRIVACY_TEMPLATE_MAX: u32 = 5;
pub const PRIVACY_TEMPLATE_MEDIUM: u32 = 3;
pub const PRIVACY_TEMPLATE_MEDIUM_HIGH: u32 = 2;
pub const PRIVACY_TEMPLATE_MEDIUM_LOW: u32 = 4;
pub const PRIVACY_TEMPLATE_NO_COOKIES: u32 = 0;
pub const PRIVACY_TYPE_FIRST_PARTY: u32 = 0;
pub const PRIVACY_TYPE_THIRD_PARTY: u32 = 1;
pub const PROXY_AUTO_DETECT_TYPE_DHCP: u32 = 1;
pub const PROXY_AUTO_DETECT_TYPE_DNS_A: u32 = 2;
pub const PROXY_TYPE_AUTO_DETECT: u32 = 8;
pub const PROXY_TYPE_AUTO_PROXY_URL: u32 = 4;
pub const PROXY_TYPE_DIRECT: u32 = 1;
pub const PROXY_TYPE_PROXY: u32 = 2;
pub const SECURITY_FLAG_128BIT: u32 = 536870912;
pub const SECURITY_FLAG_40BIT: u32 = 268435456;
pub const SECURITY_FLAG_56BIT: u32 = 1073741824;
pub const SECURITY_FLAG_FORTEZZA: u32 = 134217728;
pub const SECURITY_FLAG_IETFSSL4: u32 = 32;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384;
pub const SECURITY_FLAG_IGNORE_REVOCATION: u32 = 128;
pub const SECURITY_FLAG_IGNORE_WEAK_SIGNATURE: u32 = 65536;
pub const SECURITY_FLAG_IGNORE_WRONG_USAGE: u32 = 512;
pub const SECURITY_FLAG_NORMALBITNESS: u32 = 268435456;
pub const SECURITY_FLAG_OPT_IN_WEAK_SIGNATURE: u32 = 131072;
pub const SECURITY_FLAG_PCT: u32 = 8;
pub const SECURITY_FLAG_PCT4: u32 = 16;
pub const SECURITY_FLAG_SSL: u32 = 2;
pub const SECURITY_FLAG_SSL3: u32 = 4;
pub const SECURITY_FLAG_UNKNOWNBIT: u32 = 2147483648;
pub const SECURITY_IGNORE_ERROR_MASK: u32 = 78208;
pub const SECURITY_INTERNET_MASK: u32 = 61440;
pub const SECURITY_SET_MASK: u32 = 78720;
pub const SPARSE_CACHE_ENTRY: u32 = 65536;
pub const STICKY_CACHE_ENTRY: u32 = 4;
pub const TRACK_OFFLINE_CACHE_ENTRY: u32 = 16;
pub const TRACK_ONLINE_CACHE_ENTRY: u32 = 32;
pub const URLCACHE_FIND_DEFAULT_FILTER: u32 = 3145781;
pub const URLHISTORY_CACHE_ENTRY: u32 = 2097152;
#[repr(C)]
#[cfg(feature = "winhttp")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URL_COMPONENTSA {
    pub dwStructSize: u32,
    pub lpszScheme: windows_core::PSTR,
    pub dwSchemeLength: u32,
    pub nScheme: super::winhttp::INTERNET_SCHEME,
    pub lpszHostName: windows_core::PSTR,
    pub dwHostNameLength: u32,
    pub nPort: super::winhttp::INTERNET_PORT,
    pub lpszUserName: windows_core::PSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: windows_core::PSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: windows_core::PSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: windows_core::PSTR,
    pub dwExtraInfoLength: u32,
}
pub const WININET_API_FLAG_ASYNC: u32 = 1;
pub const WININET_API_FLAG_SYNC: u32 = 4;
pub const WININET_API_FLAG_USE_CONTEXT: u32 = 8;
pub type WPAD_CACHE_DELETE = i32;
pub const WPAD_CACHE_DELETE_ALL: WPAD_CACHE_DELETE = 1;
pub const WPAD_CACHE_DELETE_CURRENT: WPAD_CACHE_DELETE = 0;
pub type pfnInternetDeInitializeAutoProxyDll = Option<unsafe extern "system" fn(lpszmime: windows_core::PCSTR, dwreserved: u32) -> windows_core::BOOL>;
pub type pfnInternetGetProxyInfo = Option<unsafe extern "system" fn(lpszurl: windows_core::PCSTR, dwurllength: u32, lpszurlhostname: windows_core::PCSTR, dwurlhostnamelength: u32, lplpszproxyhostname: *mut windows_core::PSTR, lpdwproxyhostnamelength: *mut u32) -> windows_core::BOOL>;
pub type pfnInternetInitializeAutoProxyDll = Option<unsafe extern "system" fn(dwversion: u32, lpszdownloadedtempfile: windows_core::PCSTR, lpszmime: windows_core::PCSTR, lpautoproxycallbacks: *mut AutoProxyHelperFunctions, lpautoproxyscriptbuffer: *mut AUTO_PROXY_SCRIPT_BUFFER) -> windows_core::BOOL>;
