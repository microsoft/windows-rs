#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn CommitUrlCacheEntryA(lpszurlname : windows_sys::core::PCSTR, lpszlocalfilename : windows_sys::core::PCSTR, expiretime : super::FILETIME, lastmodifiedtime : super::FILETIME, cacheentrytype : u32, lpheaderinfo : *const u8, cchheaderinfo : u32, lpszfileextension : windows_sys::core::PCSTR, lpszoriginalurl : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn CommitUrlCacheEntryW(lpszurlname : windows_sys::core::PCWSTR, lpszlocalfilename : windows_sys::core::PCWSTR, expiretime : super::FILETIME, lastmodifiedtime : super::FILETIME, cacheentrytype : u32, lpszheaderinfo : windows_sys::core::PCWSTR, cchheaderinfo : u32, lpszfileextension : windows_sys::core::PCWSTR, lpszoriginalurl : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn CreateMD5SSOHash(pszchallengeinfo : windows_sys::core::PCWSTR, pwszrealm : windows_sys::core::PCWSTR, pwsztarget : windows_sys::core::PCWSTR, pbhexhash : *mut u8) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn CreateUrlCacheEntryA(lpszurlname : windows_sys::core::PCSTR, dwexpectedfilesize : u32, lpszfileextension : windows_sys::core::PCSTR, lpszfilename : windows_sys::core::PSTR, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn CreateUrlCacheEntryW(lpszurlname : windows_sys::core::PCWSTR, dwexpectedfilesize : u32, lpszfileextension : windows_sys::core::PCWSTR, lpszfilename : windows_sys::core::PWSTR, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn CreateUrlCacheGroup(dwflags : u32, lpreserved : *const core::ffi::c_void) -> GROUPID);
windows_link::link!("wininet.dll" "system" fn DeleteUrlCacheEntry(lpszurlname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn DeleteUrlCacheEntryA(lpszurlname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn DeleteUrlCacheEntryW(lpszurlname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn DeleteUrlCacheGroup(groupid : GROUPID, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn DeleteWpadCacheForNetworks(param0 : WPAD_CACHE_DELETE) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn DetectAutoProxyUrl(pszautoproxyurl : windows_sys::core::PSTR, cchautoproxyurl : u32, dwdetectflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn FindCloseUrlCache(henumhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryA(lpszurlsearchpattern : windows_sys::core::PCSTR, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryExA(lpszurlsearchpattern : windows_sys::core::PCSTR, dwflags : u32, dwfilter : u32, groupid : GROUPID, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryExW(lpszurlsearchpattern : windows_sys::core::PCWSTR, dwflags : u32, dwfilter : u32, groupid : GROUPID, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryW(lpszurlsearchpattern : windows_sys::core::PCWSTR, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn FindFirstUrlCacheGroup(dwflags : u32, dwfilter : u32, lpsearchcondition : *const core::ffi::c_void, dwsearchcondition : u32, lpgroupid : *mut GROUPID, lpreserved : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindNextUrlCacheEntryA(henumhandle : super::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindNextUrlCacheEntryExA(henumhandle : super::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindNextUrlCacheEntryExW(henumhandle : super::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn FindNextUrlCacheEntryW(henumhandle : super::HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn FindNextUrlCacheGroup(hfind : super::HANDLE, lpgroupid : *mut GROUPID, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpCommandA(hconnect : super::HINTERNET, fexpectresponse : windows_sys::core::BOOL, dwflags : u32, lpszcommand : windows_sys::core::PCSTR, dwcontext : usize, phftpcommand : *mut super::HINTERNET) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpCommandW(hconnect : super::HINTERNET, fexpectresponse : windows_sys::core::BOOL, dwflags : u32, lpszcommand : windows_sys::core::PCWSTR, dwcontext : usize, phftpcommand : *mut super::HINTERNET) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpCreateDirectoryA(hconnect : super::HINTERNET, lpszdirectory : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpCreateDirectoryW(hconnect : super::HINTERNET, lpszdirectory : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpDeleteFileA(hconnect : super::HINTERNET, lpszfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpDeleteFileW(hconnect : super::HINTERNET, lpszfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winhttp"))]
windows_link::link!("wininet.dll" "system" fn FtpFindFirstFileA(hconnect : super::HINTERNET, lpszsearchfile : windows_sys::core::PCSTR, lpfindfiledata : *mut super::WIN32_FIND_DATAA, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winhttp"))]
windows_link::link!("wininet.dll" "system" fn FtpFindFirstFileW(hconnect : super::HINTERNET, lpszsearchfile : windows_sys::core::PCWSTR, lpfindfiledata : *mut super::WIN32_FIND_DATAW, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpGetCurrentDirectoryA(hconnect : super::HINTERNET, lpszcurrentdirectory : windows_sys::core::PSTR, lpdwcurrentdirectory : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpGetCurrentDirectoryW(hconnect : super::HINTERNET, lpszcurrentdirectory : windows_sys::core::PWSTR, lpdwcurrentdirectory : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpGetFileA(hconnect : super::HINTERNET, lpszremotefile : windows_sys::core::PCSTR, lpsznewfile : windows_sys::core::PCSTR, ffailifexists : windows_sys::core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpGetFileEx(hftpsession : super::HINTERNET, lpszremotefile : windows_sys::core::PCSTR, lpsznewfile : windows_sys::core::PCWSTR, ffailifexists : windows_sys::core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpGetFileSize(hfile : super::HINTERNET, lpdwfilesizehigh : *mut u32) -> u32);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpGetFileW(hconnect : super::HINTERNET, lpszremotefile : windows_sys::core::PCWSTR, lpsznewfile : windows_sys::core::PCWSTR, ffailifexists : windows_sys::core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpOpenFileA(hconnect : super::HINTERNET, lpszfilename : windows_sys::core::PCSTR, dwaccess : u32, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpOpenFileW(hconnect : super::HINTERNET, lpszfilename : windows_sys::core::PCWSTR, dwaccess : u32, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpPutFileA(hconnect : super::HINTERNET, lpszlocalfile : windows_sys::core::PCSTR, lpsznewremotefile : windows_sys::core::PCSTR, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpPutFileEx(hftpsession : super::HINTERNET, lpszlocalfile : windows_sys::core::PCWSTR, lpsznewremotefile : windows_sys::core::PCSTR, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpPutFileW(hconnect : super::HINTERNET, lpszlocalfile : windows_sys::core::PCWSTR, lpsznewremotefile : windows_sys::core::PCWSTR, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpRemoveDirectoryA(hconnect : super::HINTERNET, lpszdirectory : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpRemoveDirectoryW(hconnect : super::HINTERNET, lpszdirectory : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpRenameFileA(hconnect : super::HINTERNET, lpszexisting : windows_sys::core::PCSTR, lpsznew : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpRenameFileW(hconnect : super::HINTERNET, lpszexisting : windows_sys::core::PCWSTR, lpsznew : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpSetCurrentDirectoryA(hconnect : super::HINTERNET, lpszdirectory : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn FtpSetCurrentDirectoryW(hconnect : super::HINTERNET, lpszdirectory : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoA(lpszurlname : windows_sys::core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoExA(lpszurl : windows_sys::core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpszredirecturl : windows_sys::core::PCSTR, lpcbredirecturl : *const u32, lpreserved : *const core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoExW(lpszurl : windows_sys::core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpszredirecturl : windows_sys::core::PCWSTR, lpcbredirecturl : *const u32, lpreserved : *const core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoW(lpszurlname : windows_sys::core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn GetUrlCacheGroupAttributeA(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn GetUrlCacheGroupAttributeW(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn GopherCreateLocatorA(lpszhost : windows_sys::core::PCSTR, nserverport : super::INTERNET_PORT, lpszdisplaystring : windows_sys::core::PCSTR, lpszselectorstring : windows_sys::core::PCSTR, dwgophertype : u32, lpszlocator : windows_sys::core::PSTR, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn GopherCreateLocatorW(lpszhost : windows_sys::core::PCWSTR, nserverport : super::INTERNET_PORT, lpszdisplaystring : windows_sys::core::PCWSTR, lpszselectorstring : windows_sys::core::PCWSTR, dwgophertype : u32, lpszlocator : windows_sys::core::PWSTR, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winhttp"))]
windows_link::link!("wininet.dll" "system" fn GopherFindFirstFileA(hconnect : super::HINTERNET, lpszlocator : windows_sys::core::PCSTR, lpszsearchstring : windows_sys::core::PCSTR, lpfinddata : *mut GOPHER_FIND_DATAA, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(all(feature = "minwindef", feature = "winhttp"))]
windows_link::link!("wininet.dll" "system" fn GopherFindFirstFileW(hconnect : super::HINTERNET, lpszlocator : windows_sys::core::PCWSTR, lpszsearchstring : windows_sys::core::PCWSTR, lpfinddata : *mut GOPHER_FIND_DATAW, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(all(feature = "minwindef", feature = "winhttp", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn GopherGetAttributeA(hconnect : super::HINTERNET, lpszlocator : windows_sys::core::PCSTR, lpszattributename : windows_sys::core::PCSTR, lpbuffer : *mut u8, dwbufferlength : u32, lpdwcharactersreturned : *mut u32, lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winhttp", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn GopherGetAttributeW(hconnect : super::HINTERNET, lpszlocator : windows_sys::core::PCWSTR, lpszattributename : windows_sys::core::PCWSTR, lpbuffer : *mut u8, dwbufferlength : u32, lpdwcharactersreturned : *mut u32, lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext : usize) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn GopherGetLocatorTypeA(lpszlocator : windows_sys::core::PCSTR, lpdwgophertype : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn GopherGetLocatorTypeW(lpszlocator : windows_sys::core::PCWSTR, lpdwgophertype : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn GopherOpenFileA(hconnect : super::HINTERNET, lpszlocator : windows_sys::core::PCSTR, lpszview : windows_sys::core::PCSTR, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn GopherOpenFileW(hconnect : super::HINTERNET, lpszlocator : windows_sys::core::PCWSTR, lpszview : windows_sys::core::PCWSTR, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpAddRequestHeadersA(hrequest : super::HINTERNET, lpszheaders : windows_sys::core::PCSTR, dwheaderslength : u32, dwmodifiers : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpAddRequestHeadersW(hrequest : super::HINTERNET, lpszheaders : windows_sys::core::PCWSTR, dwheaderslength : u32, dwmodifiers : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpEndRequestA(hrequest : super::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpEndRequestW(hrequest : super::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn HttpIsHostHstsEnabled(pcwszurl : windows_sys::core::PCWSTR, pfishsts : *mut windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpOpenRequestA(hconnect : super::HINTERNET, lpszverb : windows_sys::core::PCSTR, lpszobjectname : windows_sys::core::PCSTR, lpszversion : windows_sys::core::PCSTR, lpszreferrer : windows_sys::core::PCSTR, lplpszaccepttypes : *const windows_sys::core::PCSTR, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpOpenRequestW(hconnect : super::HINTERNET, lpszverb : windows_sys::core::PCWSTR, lpszobjectname : windows_sys::core::PCWSTR, lpszversion : windows_sys::core::PCWSTR, lpszreferrer : windows_sys::core::PCWSTR, lplpszaccepttypes : *const windows_sys::core::PCWSTR, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpQueryInfoA(hrequest : super::HINTERNET, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpQueryInfoW(hrequest : super::HINTERNET, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpSendRequestA(hrequest : super::HINTERNET, lpszheaders : windows_sys::core::PCSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpSendRequestExA(hrequest : super::HINTERNET, lpbuffersin : *const INTERNET_BUFFERSA, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpSendRequestExW(hrequest : super::HINTERNET, lpbuffersin : *const INTERNET_BUFFERSW, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn HttpSendRequestW(hrequest : super::HINTERNET, lpszheaders : windows_sys::core::PCWSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetAttemptConnect(dwreserved : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetAutodial(dwflags : u32, hwndparent : super::HWND) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetAutodialHangup(dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetCanonicalizeUrlA(lpszurl : windows_sys::core::PCSTR, lpszbuffer : windows_sys::core::PSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetCanonicalizeUrlW(lpszurl : windows_sys::core::PCWSTR, lpszbuffer : windows_sys::core::PWSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetCheckConnectionA(lpszurl : windows_sys::core::PCSTR, dwflags : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetCheckConnectionW(lpszurl : windows_sys::core::PCWSTR, dwflags : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetClearAllPerSiteCookieDecisions() -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetCloseHandle(hinternet : super::HINTERNET) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetCombineUrlA(lpszbaseurl : windows_sys::core::PCSTR, lpszrelativeurl : windows_sys::core::PCSTR, lpszbuffer : windows_sys::core::PSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetCombineUrlW(lpszbaseurl : windows_sys::core::PCWSTR, lpszrelativeurl : windows_sys::core::PCWSTR, lpszbuffer : windows_sys::core::PWSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetConfirmZoneCrossing(hwnd : super::HWND, szurlprev : windows_sys::core::PCSTR, szurlnew : windows_sys::core::PCSTR, bpost : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetConfirmZoneCrossingA(hwnd : super::HWND, szurlprev : windows_sys::core::PCSTR, szurlnew : windows_sys::core::PCSTR, bpost : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetConfirmZoneCrossingW(hwnd : super::HWND, szurlprev : windows_sys::core::PCWSTR, szurlnew : windows_sys::core::PCWSTR, bpost : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetConnectA(hinternet : super::HINTERNET, lpszservername : windows_sys::core::PCSTR, nserverport : super::INTERNET_PORT, lpszusername : windows_sys::core::PCSTR, lpszpassword : windows_sys::core::PCSTR, dwservice : u32, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetConnectW(hinternet : super::HINTERNET, lpszservername : windows_sys::core::PCWSTR, nserverport : super::INTERNET_PORT, lpszusername : windows_sys::core::PCWSTR, lpszpassword : windows_sys::core::PCWSTR, dwservice : u32, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetCrackUrlA(lpszurl : windows_sys::core::PCSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : *mut URL_COMPONENTSA) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetCrackUrlW(lpszurl : windows_sys::core::PCWSTR, dwurllength : u32, dwflags : u32, lpurlcomponents : super::LPURL_COMPONENTSW) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetCreateUrlA(lpurlcomponents : *const URL_COMPONENTSA, dwflags : u32, lpszurl : windows_sys::core::PSTR, lpdwurllength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetCreateUrlW(lpurlcomponents : super::LPURL_COMPONENTSW, dwflags : u32, lpszurl : windows_sys::core::PWSTR, lpdwurllength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetDial(hwndparent : super::HWND, lpszconnectoid : windows_sys::core::PCSTR, dwflags : u32, lpdwconnection : *mut u32, dwreserved : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetDialA(hwndparent : super::HWND, lpszconnectoid : windows_sys::core::PCSTR, dwflags : u32, lpdwconnection : *mut usize, dwreserved : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetDialW(hwndparent : super::HWND, lpszconnectoid : windows_sys::core::PCWSTR, dwflags : u32, lpdwconnection : *mut usize, dwreserved : u32) -> u32);
windows_link::link!("wininet.dll" "system" fn InternetEnumPerSiteCookieDecisionA(pszsitename : windows_sys::core::PSTR, pcsitenamesize : *mut u32, pdwdecision : *mut u32, dwindex : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetEnumPerSiteCookieDecisionW(pszsitename : windows_sys::core::PWSTR, pcsitenamesize : *mut u32, pdwdecision : *mut u32, dwindex : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winhttp"))]
windows_link::link!("wininet.dll" "system" fn InternetErrorDlg(hwnd : super::HWND, hrequest : super::HINTERNET, dwerror : u32, dwflags : u32, lppvdata : *mut *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetFindNextFileA(hfind : super::HINTERNET, lpvfinddata : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetFindNextFileW(hfind : super::HINTERNET, lpvfinddata : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn InternetFreeCookies(pcookies : *mut INTERNET_COOKIE2, dwcookiecount : u32));
windows_link::link!("wininet.dll" "system" fn InternetGetConnectedState(lpdwflags : *mut u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetConnectedStateEx(lpdwflags : *mut u32, lpszconnectionname : windows_sys::core::PSTR, dwnamelen : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetConnectedStateExA(lpdwflags : *mut u32, lpszconnectionname : windows_sys::core::PSTR, cchnamelen : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetConnectedStateExW(lpdwflags : *mut u32, lpszconnectionname : windows_sys::core::PWSTR, cchnamelen : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetCookieA(lpszurl : windows_sys::core::PCSTR, lpszcookiename : windows_sys::core::PCSTR, lpszcookiedata : windows_sys::core::PSTR, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn InternetGetCookieEx2(pcwszurl : windows_sys::core::PCWSTR, pcwszcookiename : windows_sys::core::PCWSTR, dwflags : u32, ppcookies : *mut *mut INTERNET_COOKIE2, pdwcookiecount : *mut u32) -> u32);
windows_link::link!("wininet.dll" "system" fn InternetGetCookieExA(lpszurl : windows_sys::core::PCSTR, lpszcookiename : windows_sys::core::PCSTR, lpszcookiedata : windows_sys::core::PCSTR, lpdwsize : *mut u32, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetCookieExW(lpszurl : windows_sys::core::PCWSTR, lpszcookiename : windows_sys::core::PCWSTR, lpszcookiedata : windows_sys::core::PCWSTR, lpdwsize : *mut u32, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetCookieW(lpszurl : windows_sys::core::PCWSTR, lpszcookiename : windows_sys::core::PCWSTR, lpszcookiedata : windows_sys::core::PWSTR, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetLastResponseInfoA(lpdwerror : *mut u32, lpszbuffer : windows_sys::core::PSTR, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetLastResponseInfoW(lpdwerror : *mut u32, lpszbuffer : windows_sys::core::PWSTR, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetPerSiteCookieDecisionA(pchhostname : windows_sys::core::PCSTR, presult : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetGetPerSiteCookieDecisionW(pchhostname : windows_sys::core::PCWSTR, presult : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetGoOnline(lpszurl : windows_sys::core::PCSTR, hwndparent : super::HWND, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetGoOnlineA(lpszurl : windows_sys::core::PCSTR, hwndparent : super::HWND, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("wininet.dll" "system" fn InternetGoOnlineW(lpszurl : windows_sys::core::PCWSTR, hwndparent : super::HWND, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetHangUp(dwconnection : usize, dwreserved : u32) -> u32);
windows_link::link!("wininet.dll" "system" fn InternetInitializeAutoProxyDll(dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "winhttp", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn InternetLockRequestFile(hinternet : super::HINTERNET, lphlockrequestinfo : *mut super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetOpenA(lpszagent : windows_sys::core::PCSTR, dwaccesstype : u32, lpszproxy : windows_sys::core::PCSTR, lpszproxybypass : windows_sys::core::PCSTR, dwflags : u32) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetOpenUrlA(hinternet : super::HINTERNET, lpszurl : windows_sys::core::PCSTR, lpszheaders : windows_sys::core::PCSTR, dwheaderslength : u32, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetOpenUrlW(hinternet : super::HINTERNET, lpszurl : windows_sys::core::PCWSTR, lpszheaders : windows_sys::core::PCWSTR, dwheaderslength : u32, dwflags : u32, dwcontext : usize) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetOpenW(lpszagent : windows_sys::core::PCWSTR, dwaccesstype : u32, lpszproxy : windows_sys::core::PCWSTR, lpszproxybypass : windows_sys::core::PCWSTR, dwflags : u32) -> super::HINTERNET);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetQueryDataAvailable(hfile : super::HINTERNET, lpdwnumberofbytesavailable : *mut u32, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetQueryOptionA(hinternet : super::HINTERNET, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetQueryOptionW(hinternet : super::HINTERNET, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetReadFile(hfile : super::HINTERNET, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetReadFileExA(hfile : super::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetReadFileExW(hfile : super::HINTERNET, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetSetCookieA(lpszurl : windows_sys::core::PCSTR, lpszcookiename : windows_sys::core::PCSTR, lpszcookiedata : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn InternetSetCookieEx2(pcwszurl : windows_sys::core::PCWSTR, pcookie : *const INTERNET_COOKIE2, pcwszp3ppolicy : windows_sys::core::PCWSTR, dwflags : u32, pdwcookiestate : *mut u32) -> u32);
windows_link::link!("wininet.dll" "system" fn InternetSetCookieExA(lpszurl : windows_sys::core::PCSTR, lpszcookiename : windows_sys::core::PCSTR, lpszcookiedata : windows_sys::core::PCSTR, dwflags : u32, dwreserved : usize) -> u32);
windows_link::link!("wininet.dll" "system" fn InternetSetCookieExW(lpszurl : windows_sys::core::PCWSTR, lpszcookiename : windows_sys::core::PCWSTR, lpszcookiedata : windows_sys::core::PCWSTR, dwflags : u32, dwreserved : usize) -> u32);
windows_link::link!("wininet.dll" "system" fn InternetSetCookieW(lpszurl : windows_sys::core::PCWSTR, lpszcookiename : windows_sys::core::PCWSTR, lpszcookiedata : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetSetDialState(lpszconnectoid : windows_sys::core::PCSTR, dwstate : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetSetDialStateA(lpszconnectoid : windows_sys::core::PCSTR, dwstate : u32, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetSetDialStateW(lpszconnectoid : windows_sys::core::PCWSTR, dwstate : u32, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetFilePointer(hfile : super::HINTERNET, ldistancetomove : i32, lpdistancetomovehigh : *mut i32, dwmovemethod : u32, dwcontext : usize) -> u32);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetOptionA(hinternet : super::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetOptionExA(hinternet : super::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetOptionExW(hinternet : super::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetOptionW(hinternet : super::HINTERNET, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetSetPerSiteCookieDecisionA(pchhostname : windows_sys::core::PCSTR, dwdecision : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn InternetSetPerSiteCookieDecisionW(pchhostname : windows_sys::core::PCWSTR, dwdecision : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetStatusCallback(hinternet : super::HINTERNET, lpfninternetcallback : INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetStatusCallbackA(hinternet : super::HINTERNET, lpfninternetcallback : INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetSetStatusCallbackW(hinternet : super::HINTERNET, lpfninternetcallback : INTERNET_STATUS_CALLBACK) -> INTERNET_STATUS_CALLBACK);
#[cfg(feature = "minwinbase")]
windows_link::link!("wininet.dll" "system" fn InternetTimeFromSystemTime(pst : *const super::SYSTEMTIME, dwrfc : u32, lpsztime : windows_sys::core::PSTR, cbtime : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("wininet.dll" "system" fn InternetTimeFromSystemTimeA(pst : *const super::SYSTEMTIME, dwrfc : u32, lpsztime : windows_sys::core::PSTR, cbtime : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("wininet.dll" "system" fn InternetTimeFromSystemTimeW(pst : *const super::SYSTEMTIME, dwrfc : u32, lpsztime : windows_sys::core::PWSTR, cbtime : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("wininet.dll" "system" fn InternetTimeToSystemTime(lpsztime : windows_sys::core::PCSTR, pst : *mut super::SYSTEMTIME, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("wininet.dll" "system" fn InternetTimeToSystemTimeA(lpsztime : windows_sys::core::PCSTR, pst : *mut super::SYSTEMTIME, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("wininet.dll" "system" fn InternetTimeToSystemTimeW(lpsztime : windows_sys::core::PCWSTR, pst : *mut super::SYSTEMTIME, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn InternetUnlockRequestFile(hlockrequestinfo : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn InternetWriteFile(hfile : super::HINTERNET, lpbuffer : *const core::ffi::c_void, dwnumberofbytestowrite : u32, lpdwnumberofbyteswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn PrivacyGetZonePreferenceW(dwzone : u32, dwtype : u32, pdwtemplate : *mut u32, pszbuffer : windows_sys::core::PWSTR, pdwbufferlength : *mut u32) -> u32);
windows_link::link!("wininet.dll" "system" fn PrivacySetZonePreferenceW(dwzone : u32, dwtype : u32, dwtemplate : u32, pszpreference : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn ReadUrlCacheEntryStream(hurlcachestream : super::HANDLE, dwlocation : u32, lpbuffer : *mut core::ffi::c_void, lpdwlen : *mut u32, reserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn ReadUrlCacheEntryStreamEx(hurlcachestream : super::HANDLE, qwlocation : super::DWORDLONG, lpbuffer : *mut core::ffi::c_void, lpdwlen : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winhttp")]
windows_link::link!("wininet.dll" "system" fn ResumeSuspendedDownload(hrequest : super::HINTERNET, dwresultcode : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryFileA(lpszurlname : windows_sys::core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryFileW(lpszurlname : windows_sys::core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryStreamA(lpszurlname : windows_sys::core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, frandomread : windows_sys::core::BOOL, dwreserved : u32) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryStreamW(lpszurlname : windows_sys::core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, frandomread : windows_sys::core::BOOL, dwreserved : u32) -> super::HANDLE);
windows_link::link!("wininet.dll" "system" fn SetUrlCacheEntryGroup(lpszurlname : windows_sys::core::PCSTR, dwflags : u32, groupid : GROUPID, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn SetUrlCacheEntryGroupA(lpszurlname : windows_sys::core::PCSTR, dwflags : u32, groupid : GROUPID, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn SetUrlCacheEntryGroupW(lpszurlname : windows_sys::core::PCWSTR, dwflags : u32, groupid : GROUPID, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn SetUrlCacheEntryInfoA(lpszurlname : windows_sys::core::PCSTR, lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wininet.dll" "system" fn SetUrlCacheEntryInfoW(lpszurlname : windows_sys::core::PCWSTR, lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn SetUrlCacheGroupAttributeA(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOA, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn SetUrlCacheGroupAttributeW(gid : GROUPID, dwflags : u32, dwattributes : u32, lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOW, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFile(lpszurlname : windows_sys::core::PCSTR, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFileA(lpszurlname : windows_sys::core::PCSTR, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFileW(lpszurlname : windows_sys::core::PCWSTR, dwreserved : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wininet.dll" "system" fn UnlockUrlCacheEntryStream(hurlcachestream : super::HANDLE, reserved : u32) -> windows_sys::core::BOOL);
pub const AUTH_FLAG_DISABLE_BASIC_CLEARCHANNEL: i32 = 4;
pub const AUTH_FLAG_DISABLE_NEGOTIATE: i32 = 1;
pub const AUTH_FLAG_DISABLE_SERVER_AUTH: i32 = 8;
pub const AUTH_FLAG_ENABLE_NEGOTIATE: i32 = 2;
pub const AUTODIAL_MODE_ALWAYS: i32 = 2;
pub const AUTODIAL_MODE_NEVER: i32 = 1;
pub const AUTODIAL_MODE_NO_NETWORK_PRESENT: i32 = 4;
pub const AUTO_PROXY_FLAG_ALWAYS_DETECT: i32 = 2;
pub const AUTO_PROXY_FLAG_CACHE_INIT_RUN: i32 = 32;
pub const AUTO_PROXY_FLAG_DETECTION_RUN: i32 = 4;
pub const AUTO_PROXY_FLAG_DETECTION_SUSPECT: i32 = 64;
pub const AUTO_PROXY_FLAG_DONT_CACHE_PROXY_RESULT: i32 = 16;
pub const AUTO_PROXY_FLAG_MIGRATED: i32 = 8;
pub const AUTO_PROXY_FLAG_USER_SET: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTO_PROXY_SCRIPT_BUFFER {
    pub dwStructSize: u32,
    pub lpszScriptBuffer: windows_sys::core::PSTR,
    pub dwScriptBufferSize: u32,
}
impl Default for AUTO_PROXY_SCRIPT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AutoProxyHelperFunctions {
    pub lpVtbl: *const AutoProxyHelperVtbl,
}
impl Default for AutoProxyHelperFunctions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const CACHEGROUP_ATTRIBUTE_BASIC: i32 = 1;
pub const CACHEGROUP_ATTRIBUTE_FLAG: i32 = 2;
pub const CACHEGROUP_ATTRIBUTE_GET_ALL: u32 = 4294967295;
pub const CACHEGROUP_ATTRIBUTE_GROUPNAME: i32 = 16;
pub const CACHEGROUP_ATTRIBUTE_QUOTA: i32 = 8;
pub const CACHEGROUP_ATTRIBUTE_STORAGE: i32 = 32;
pub const CACHEGROUP_ATTRIBUTE_TYPE: i32 = 4;
pub const CACHEGROUP_FLAG_FLUSHURL_ONDELETE: i32 = 2;
pub const CACHEGROUP_FLAG_GIDONLY: i32 = 4;
pub const CACHEGROUP_FLAG_NONPURGEABLE: i32 = 1;
pub const CACHEGROUP_READWRITE_MASK: i32 = 60;
pub const CACHEGROUP_SEARCH_ALL: i32 = 0;
pub const CACHEGROUP_SEARCH_BYURL: i32 = 1;
pub const CACHEGROUP_TYPE_INVALID: i32 = 1;
pub const CACHE_ENTRY_ACCTIME_FC: i32 = 256;
pub const CACHE_ENTRY_ATTRIBUTE_FC: i32 = 4;
pub const CACHE_ENTRY_EXEMPT_DELTA_FC: i32 = 2048;
pub const CACHE_ENTRY_EXPTIME_FC: i32 = 128;
pub const CACHE_ENTRY_HEADERINFO_FC: i32 = 1024;
pub const CACHE_ENTRY_HITRATE_FC: i32 = 16;
pub const CACHE_ENTRY_MODTIME_FC: i32 = 64;
pub const CACHE_ENTRY_SYNCTIME_FC: i32 = 512;
pub const CERN_PROXY_INTERNET_ACCESS: i32 = 3;
pub const COOKIE_CACHE_ENTRY: i32 = 1048576;
pub const COOKIE_STATE_ACCEPT: InternetCookieState = 1;
pub const COOKIE_STATE_DOWNGRADE: InternetCookieState = 4;
pub const COOKIE_STATE_LEASH: InternetCookieState = 3;
pub const COOKIE_STATE_MAX: InternetCookieState = 5;
pub const COOKIE_STATE_PROMPT: InternetCookieState = 2;
pub const COOKIE_STATE_REJECT: InternetCookieState = 5;
pub const COOKIE_STATE_UNKNOWN: InternetCookieState = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CookieDecision {
    pub dwCookieState: u32,
    pub fAllowSession: windows_sys::core::BOOL,
}
pub const EDITED_CACHE_ENTRY: i32 = 8;
pub const ERROR_FTP_DROPPED: i32 = 12111;
pub const ERROR_FTP_NO_PASSIVE_MODE: i32 = 12112;
pub const ERROR_FTP_TRANSFER_IN_PROGRESS: i32 = 12110;
pub const ERROR_GOPHER_ATTRIBUTE_NOT_FOUND: i32 = 12137;
pub const ERROR_GOPHER_DATA_ERROR: i32 = 12132;
pub const ERROR_GOPHER_END_OF_DATA: i32 = 12133;
pub const ERROR_GOPHER_INCORRECT_LOCATOR_TYPE: i32 = 12135;
pub const ERROR_GOPHER_INVALID_LOCATOR: i32 = 12134;
pub const ERROR_GOPHER_NOT_FILE: i32 = 12131;
pub const ERROR_GOPHER_NOT_GOPHER_PLUS: i32 = 12136;
pub const ERROR_GOPHER_PROTOCOL_ERROR: i32 = 12130;
pub const ERROR_GOPHER_UNKNOWN_LOCATOR: i32 = 12138;
pub const ERROR_HTTP_COOKIE_DECLINED: i32 = 12162;
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION: i32 = 12161;
pub const ERROR_HTTP_DOWNLEVEL_SERVER: i32 = 12151;
pub const ERROR_HTTP_HEADER_ALREADY_EXISTS: i32 = 12155;
pub const ERROR_HTTP_HEADER_NOT_FOUND: i32 = 12150;
pub const ERROR_HTTP_HSTS_REDIRECT_REQUIRED: i32 = 12060;
pub const ERROR_HTTP_INVALID_HEADER: i32 = 12153;
pub const ERROR_HTTP_INVALID_QUERY_REQUEST: i32 = 12154;
pub const ERROR_HTTP_INVALID_SERVER_RESPONSE: i32 = 12152;
pub const ERROR_HTTP_NOT_REDIRECTED: i32 = 12160;
pub const ERROR_HTTP_REDIRECT_FAILED: i32 = 12156;
pub const ERROR_HTTP_REDIRECT_NEEDS_CONFIRMATION: i32 = 12168;
pub const ERROR_INTERNET_ASYNC_THREAD_FAILED: i32 = 12047;
pub const ERROR_INTERNET_BAD_AUTO_PROXY_SCRIPT: i32 = 12166;
pub const ERROR_INTERNET_BAD_OPTION_LENGTH: i32 = 12010;
pub const ERROR_INTERNET_BAD_REGISTRY_PARAMETER: i32 = 12022;
pub const ERROR_INTERNET_CANNOT_CONNECT: i32 = 12029;
pub const ERROR_INTERNET_CHG_POST_IS_NON_SECURE: i32 = 12042;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED: i32 = 12044;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED_PROXY: i32 = 12187;
pub const ERROR_INTERNET_CLIENT_AUTH_NOT_SETUP: i32 = 12046;
pub const ERROR_INTERNET_CONNECTION_ABORTED: i32 = 12030;
pub const ERROR_INTERNET_CONNECTION_RESET: i32 = 12031;
pub const ERROR_INTERNET_DECODING_FAILED: i32 = 12175;
pub const ERROR_INTERNET_DIALOG_PENDING: i32 = 12049;
pub const ERROR_INTERNET_DISCONNECTED: i32 = 12163;
pub const ERROR_INTERNET_EXTENDED_ERROR: i32 = 12003;
pub const ERROR_INTERNET_FAILED_DUETOSECURITYCHECK: i32 = 12171;
pub const ERROR_INTERNET_FEATURE_DISABLED: i32 = 12192;
pub const ERROR_INTERNET_FORCE_RETRY: i32 = 12032;
pub const ERROR_INTERNET_FORTEZZA_LOGIN_NEEDED: i32 = 12054;
pub const ERROR_INTERNET_GLOBAL_CALLBACK_FAILED: i32 = 12191;
pub const ERROR_INTERNET_HANDLE_EXISTS: i32 = 12036;
pub const ERROR_INTERNET_HTTPS_HTTP_SUBMIT_REDIR: i32 = 12052;
pub const ERROR_INTERNET_HTTPS_TO_HTTP_ON_REDIR: i32 = 12040;
pub const ERROR_INTERNET_HTTP_PROTOCOL_MISMATCH: i32 = 12190;
pub const ERROR_INTERNET_HTTP_TO_HTTPS_ON_REDIR: i32 = 12039;
pub const ERROR_INTERNET_INCORRECT_FORMAT: i32 = 12027;
pub const ERROR_INTERNET_INCORRECT_HANDLE_STATE: i32 = 12019;
pub const ERROR_INTERNET_INCORRECT_HANDLE_TYPE: i32 = 12018;
pub const ERROR_INTERNET_INCORRECT_PASSWORD: i32 = 12014;
pub const ERROR_INTERNET_INCORRECT_USER_NAME: i32 = 12013;
pub const ERROR_INTERNET_INSERT_CDROM: i32 = 12053;
pub const ERROR_INTERNET_INTERNAL_ERROR: i32 = 12004;
pub const ERROR_INTERNET_INVALID_CA: i32 = 12045;
pub const ERROR_INTERNET_INVALID_OPERATION: i32 = 12016;
pub const ERROR_INTERNET_INVALID_OPTION: i32 = 12009;
pub const ERROR_INTERNET_INVALID_PROXY_REQUEST: i32 = 12033;
pub const ERROR_INTERNET_INVALID_URL: i32 = 12005;
pub const ERROR_INTERNET_ITEM_NOT_FOUND: i32 = 12028;
pub const ERROR_INTERNET_LOGIN_FAILURE: i32 = 12015;
pub const ERROR_INTERNET_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: i32 = 12174;
pub const ERROR_INTERNET_MIXED_SECURITY: i32 = 12041;
pub const ERROR_INTERNET_NAME_NOT_RESOLVED: i32 = 12007;
pub const ERROR_INTERNET_NEED_MSN_SSPI_PKG: i32 = 12173;
pub const ERROR_INTERNET_NEED_UI: i32 = 12034;
pub const ERROR_INTERNET_NOT_INITIALIZED: i32 = 12172;
pub const ERROR_INTERNET_NOT_PROXY_REQUEST: i32 = 12020;
pub const ERROR_INTERNET_NO_CALLBACK: i32 = 12025;
pub const ERROR_INTERNET_NO_CONTEXT: i32 = 12024;
pub const ERROR_INTERNET_NO_DIRECT_ACCESS: i32 = 12023;
pub const ERROR_INTERNET_OPERATION_CANCELLED: i32 = 12017;
pub const ERROR_INTERNET_OPTION_NOT_SETTABLE: i32 = 12011;
pub const ERROR_INTERNET_OUT_OF_HANDLES: i32 = 12001;
pub const ERROR_INTERNET_POST_IS_NON_SECURE: i32 = 12043;
pub const ERROR_INTERNET_PROTOCOL_NOT_FOUND: i32 = 12008;
pub const ERROR_INTERNET_PROXY_SERVER_UNREACHABLE: i32 = 12165;
pub const ERROR_INTERNET_REDIRECT_SCHEME_CHANGE: i32 = 12048;
pub const ERROR_INTERNET_REGISTRY_VALUE_NOT_FOUND: i32 = 12021;
pub const ERROR_INTERNET_REQUEST_PENDING: i32 = 12026;
pub const ERROR_INTERNET_RETRY_DIALOG: i32 = 12050;
pub const ERROR_INTERNET_SECURE_FAILURE_PROXY: i32 = 12188;
pub const ERROR_INTERNET_SECURITY_CHANNEL_ERROR: i32 = 12157;
pub const ERROR_INTERNET_SEC_CERT_CN_INVALID: i32 = 12038;
pub const ERROR_INTERNET_SEC_CERT_DATE_INVALID: i32 = 12037;
pub const ERROR_INTERNET_SEC_CERT_ERRORS: i32 = 12055;
pub const ERROR_INTERNET_SEC_CERT_NO_REV: i32 = 12056;
pub const ERROR_INTERNET_SEC_CERT_REVOKED: i32 = 12170;
pub const ERROR_INTERNET_SEC_CERT_REV_FAILED: i32 = 12057;
pub const ERROR_INTERNET_SEC_CERT_WEAK_SIGNATURE: i32 = 12062;
pub const ERROR_INTERNET_SEC_INVALID_CERT: i32 = 12169;
pub const ERROR_INTERNET_SERVER_UNREACHABLE: i32 = 12164;
pub const ERROR_INTERNET_SHUTDOWN: i32 = 12012;
pub const ERROR_INTERNET_TCPIP_NOT_INSTALLED: i32 = 12159;
pub const ERROR_INTERNET_TIMEOUT: i32 = 12002;
pub const ERROR_INTERNET_UNABLE_TO_CACHE_FILE: i32 = 12158;
pub const ERROR_INTERNET_UNABLE_TO_DOWNLOAD_SCRIPT: i32 = 12167;
pub const ERROR_INTERNET_UNRECOGNIZED_SCHEME: i32 = 12006;
pub const FLAGS_ERROR_UI_FILTER_FOR_ERRORS: i32 = 1;
pub const FLAGS_ERROR_UI_FLAGS_CHANGE_OPTIONS: i32 = 2;
pub const FLAGS_ERROR_UI_FLAGS_GENERATE_DATA: i32 = 4;
pub const FLAGS_ERROR_UI_FLAGS_NO_UI: i32 = 8;
pub const FLAGS_ERROR_UI_SERIALIZE_DIALOGS: i32 = 16;
pub const FLAG_ICC_FORCE_CONNECTION: i32 = 1;
pub const FTP_TRANSFER_TYPE_ASCII: i32 = 1;
pub const FTP_TRANSFER_TYPE_BINARY: i32 = 2;
pub const FTP_TRANSFER_TYPE_MASK: i32 = 3;
pub const FTP_TRANSFER_TYPE_UNKNOWN: i32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    pub ShortAbstract: super::LPCTSTR,
    pub AbstractFile: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_ADMIN_ATTRIBUTE_TYPE {
    pub Comment: super::LPCTSTR,
    pub EmailAddress: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_ASK_ATTRIBUTE_TYPE {
    pub QuestionType: super::LPCTSTR,
    pub QuestionText: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type GOPHER_ATTRIBUTE_ENUMERATOR = Option<unsafe extern "system" fn(lpattributeinfo: *const GOPHER_ATTRIBUTE_TYPE, dwerror: u32) -> windows_sys::core::BOOL>;
pub const GOPHER_ATTRIBUTE_ID_ABSTRACT: u32 = 2882325526;
pub const GOPHER_ATTRIBUTE_ID_ADMIN: u32 = 2882325514;
pub const GOPHER_ATTRIBUTE_ID_ALL: u32 = 2882325513;
pub const GOPHER_ATTRIBUTE_ID_BASE: u32 = 2882325504;
pub const GOPHER_ATTRIBUTE_ID_GEOG: u32 = 2882325522;
pub const GOPHER_ATTRIBUTE_ID_LOCATION: u32 = 2882325521;
pub const GOPHER_ATTRIBUTE_ID_MOD_DATE: u32 = 2882325515;
pub const GOPHER_ATTRIBUTE_ID_ORG: u32 = 2882325520;
pub const GOPHER_ATTRIBUTE_ID_PROVIDER: u32 = 2882325524;
pub const GOPHER_ATTRIBUTE_ID_RANGE: u32 = 2882325518;
pub const GOPHER_ATTRIBUTE_ID_SCORE: u32 = 2882325517;
pub const GOPHER_ATTRIBUTE_ID_SITE: u32 = 2882325519;
pub const GOPHER_ATTRIBUTE_ID_TIMEZONE: u32 = 2882325523;
pub const GOPHER_ATTRIBUTE_ID_TREEWALK: u32 = 2882325528;
pub const GOPHER_ATTRIBUTE_ID_TTL: u32 = 2882325516;
pub const GOPHER_ATTRIBUTE_ID_UNKNOWN: u32 = 2882325529;
pub const GOPHER_ATTRIBUTE_ID_VERSION: u32 = 2882325525;
pub const GOPHER_ATTRIBUTE_ID_VIEW: u32 = 2882325527;
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
pub const GOPHER_CATEGORY_ID_ABSTRACT: u32 = 2882325509;
pub const GOPHER_CATEGORY_ID_ADMIN: u32 = 2882325507;
pub const GOPHER_CATEGORY_ID_ALL: u32 = 2882325505;
pub const GOPHER_CATEGORY_ID_ASK: u32 = 2882325511;
pub const GOPHER_CATEGORY_ID_INFO: u32 = 2882325506;
pub const GOPHER_CATEGORY_ID_UNKNOWN: u32 = 2882325512;
pub const GOPHER_CATEGORY_ID_VERONICA: u32 = 2882325510;
pub const GOPHER_CATEGORY_ID_VIEWS: u32 = 2882325508;
#[cfg(feature = "minwindef")]
pub type GOPHER_FIND_DATA = GOPHER_FIND_DATAA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct GOPHER_FIND_DATAA {
    pub DisplayString: [i8; 129],
    pub GopherType: u32,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::FILETIME,
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
#[derive(Clone, Copy)]
pub struct GOPHER_FIND_DATAW {
    pub DisplayString: [u16; 129],
    pub GopherType: u32,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::FILETIME,
    pub Locator: [u16; 654],
}
#[cfg(feature = "minwindef")]
impl Default for GOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct GOPHER_LOCATION_ATTRIBUTE_TYPE {
    pub Location: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    pub DateAndTime: super::FILETIME,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    pub Organization: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    pub Provider: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GOPHER_SCORE_ATTRIBUTE_TYPE {
    pub Score: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    pub LowerBound: i32,
    pub UpperBound: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_SITE_ATTRIBUTE_TYPE {
    pub Site: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    pub Zone: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GOPHER_TTL_ATTRIBUTE_TYPE {
    pub Ttl: u32,
}
pub const GOPHER_TYPE_ASK: i32 = 1073741824;
pub const GOPHER_TYPE_BINARY: i32 = 512;
pub const GOPHER_TYPE_BITMAP: i32 = 16384;
pub const GOPHER_TYPE_CALENDAR: i32 = 524288;
pub const GOPHER_TYPE_CSO: i32 = 4;
pub const GOPHER_TYPE_DIRECTORY: i32 = 2;
pub const GOPHER_TYPE_DOS_ARCHIVE: i32 = 32;
pub const GOPHER_TYPE_ERROR: i32 = 8;
pub const GOPHER_TYPE_FILE_MASK: i32 = 2093681;
pub const GOPHER_TYPE_GIF: i32 = 4096;
pub const GOPHER_TYPE_GOPHER_PLUS: u32 = 2147483648;
pub const GOPHER_TYPE_HTML: i32 = 131072;
pub const GOPHER_TYPE_IMAGE: i32 = 8192;
pub const GOPHER_TYPE_INDEX_SERVER: i32 = 128;
pub const GOPHER_TYPE_INLINE: i32 = 1048576;
pub const GOPHER_TYPE_MAC_BINHEX: i32 = 16;
pub const GOPHER_TYPE_MOVIE: i32 = 32768;
pub const GOPHER_TYPE_PDF: i32 = 262144;
pub const GOPHER_TYPE_REDUNDANT: i32 = 1024;
pub const GOPHER_TYPE_SOUND: i32 = 65536;
pub const GOPHER_TYPE_TELNET: i32 = 256;
pub const GOPHER_TYPE_TEXT_FILE: i32 = 1;
pub const GOPHER_TYPE_TN3270: i32 = 2048;
pub const GOPHER_TYPE_UNIX_UUENCODED: i32 = 64;
pub const GOPHER_TYPE_UNKNOWN: i32 = 536870912;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    pub Text: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GOPHER_VERONICA_ATTRIBUTE_TYPE {
    pub TreeWalk: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_VERSION_ATTRIBUTE_TYPE {
    pub Version: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GOPHER_VIEW_ATTRIBUTE_TYPE {
    pub ContentType: super::LPCTSTR,
    pub Language: super::LPCTSTR,
    pub Size: u32,
}
#[cfg(feature = "winnt")]
impl Default for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type GROUPID = i64;
pub const GROUPNAME_MAX_LENGTH: i32 = 120;
pub const GROUP_OWNER_STORAGE_SIZE: i32 = 4;
pub const HSR_ASYNC: i32 = 1;
pub const HSR_CHUNKED: i32 = 32;
pub const HSR_DOWNLOAD: i32 = 16;
pub const HSR_INITIATE: i32 = 8;
pub const HSR_SYNC: i32 = 4;
pub const HSR_USE_CONTEXT: i32 = 8;
pub const HTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760;
pub const HTTP_ADDREQ_FLAG_ADD: i32 = 536870912;
pub const HTTP_ADDREQ_FLAG_ADD_IF_NEW: i32 = 268435456;
pub const HTTP_ADDREQ_FLAG_COALESCE: i32 = 1073741824;
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: i32 = 1073741824;
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: i32 = 16777216;
pub const HTTP_ADDREQ_FLAG_REPLACE: u32 = 2147483648;
pub const HTTP_ADDREQ_INDEX_MASK: i32 = 65535;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE: i32 = 3;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE_LAX: i32 = 2;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_MAX: i32 = 3;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_SAME_SITE: i32 = 1;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_UNKNOWN: i32 = 0;
pub const HTTP_MAJOR_VERSION: i32 = 1;
pub const HTTP_MINOR_VERSION: i32 = 0;
pub const HTTP_PROTOCOL_FLAG_HTTP2: i32 = 2;
pub const HTTP_PROTOCOL_MASK: i32 = 2;
pub const HTTP_QUERY_ACCEPT: i32 = 24;
pub const HTTP_QUERY_ACCEPT_CHARSET: i32 = 25;
pub const HTTP_QUERY_ACCEPT_ENCODING: i32 = 26;
pub const HTTP_QUERY_ACCEPT_LANGUAGE: i32 = 27;
pub const HTTP_QUERY_ACCEPT_RANGES: i32 = 42;
pub const HTTP_QUERY_AGE: i32 = 48;
pub const HTTP_QUERY_ALLOW: i32 = 7;
pub const HTTP_QUERY_AUTHENTICATION_INFO: i32 = 76;
pub const HTTP_QUERY_AUTHORIZATION: i32 = 28;
pub const HTTP_QUERY_CACHE_CONTROL: i32 = 49;
pub const HTTP_QUERY_CONNECTION: i32 = 23;
pub const HTTP_QUERY_CONTENT_BASE: i32 = 50;
pub const HTTP_QUERY_CONTENT_DESCRIPTION: i32 = 4;
pub const HTTP_QUERY_CONTENT_DISPOSITION: i32 = 47;
pub const HTTP_QUERY_CONTENT_ENCODING: i32 = 29;
pub const HTTP_QUERY_CONTENT_ID: i32 = 3;
pub const HTTP_QUERY_CONTENT_LANGUAGE: i32 = 6;
pub const HTTP_QUERY_CONTENT_LENGTH: i32 = 5;
pub const HTTP_QUERY_CONTENT_LOCATION: i32 = 51;
pub const HTTP_QUERY_CONTENT_MD5: i32 = 52;
pub const HTTP_QUERY_CONTENT_RANGE: i32 = 53;
pub const HTTP_QUERY_CONTENT_TRANSFER_ENCODING: i32 = 2;
pub const HTTP_QUERY_CONTENT_TYPE: i32 = 1;
pub const HTTP_QUERY_COOKIE: i32 = 44;
pub const HTTP_QUERY_COST: i32 = 15;
pub const HTTP_QUERY_CUSTOM: i32 = 65535;
pub const HTTP_QUERY_DATE: i32 = 9;
pub const HTTP_QUERY_DEFAULT_STYLE: i32 = 84;
pub const HTTP_QUERY_DERIVED_FROM: i32 = 14;
pub const HTTP_QUERY_DO_NOT_TRACK: i32 = 88;
pub const HTTP_QUERY_ECHO_HEADERS: i32 = 73;
pub const HTTP_QUERY_ECHO_HEADERS_CRLF: i32 = 74;
pub const HTTP_QUERY_ECHO_REPLY: i32 = 72;
pub const HTTP_QUERY_ECHO_REQUEST: i32 = 71;
pub const HTTP_QUERY_ETAG: i32 = 54;
pub const HTTP_QUERY_EXPECT: i32 = 68;
pub const HTTP_QUERY_EXPIRES: i32 = 10;
pub const HTTP_QUERY_FLAG_COALESCE: i32 = 268435456;
pub const HTTP_QUERY_FLAG_COALESCE_WITH_COMMA: i32 = 67108864;
pub const HTTP_QUERY_FLAG_NUMBER: i32 = 536870912;
pub const HTTP_QUERY_FLAG_NUMBER64: i32 = 134217728;
pub const HTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648;
pub const HTTP_QUERY_FLAG_SYSTEMTIME: i32 = 1073741824;
pub const HTTP_QUERY_FORWARDED: i32 = 30;
pub const HTTP_QUERY_FROM: i32 = 31;
pub const HTTP_QUERY_HEADER_MASK: u32 = 67108863;
pub const HTTP_QUERY_HOST: i32 = 55;
pub const HTTP_QUERY_HTTP2_SETTINGS: i32 = 90;
pub const HTTP_QUERY_IF_MATCH: i32 = 56;
pub const HTTP_QUERY_IF_MODIFIED_SINCE: i32 = 32;
pub const HTTP_QUERY_IF_NONE_MATCH: i32 = 57;
pub const HTTP_QUERY_IF_RANGE: i32 = 58;
pub const HTTP_QUERY_IF_UNMODIFIED_SINCE: i32 = 59;
pub const HTTP_QUERY_INCLUDE_REFERER_TOKEN_BINDING_ID: i32 = 93;
pub const HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID: i32 = 93;
pub const HTTP_QUERY_KEEP_ALIVE: i32 = 89;
pub const HTTP_QUERY_LAST_MODIFIED: i32 = 11;
pub const HTTP_QUERY_LINK: i32 = 16;
pub const HTTP_QUERY_LOCATION: i32 = 33;
pub const HTTP_QUERY_MAX: i32 = 95;
pub const HTTP_QUERY_MAX_FORWARDS: i32 = 60;
pub const HTTP_QUERY_MESSAGE_ID: i32 = 12;
pub const HTTP_QUERY_MIME_VERSION: i32 = 0;
pub const HTTP_QUERY_MODIFIER_FLAGS_MASK: u32 = 4227858432;
pub const HTTP_QUERY_ORIG_URI: i32 = 34;
pub const HTTP_QUERY_P3P: i32 = 80;
pub const HTTP_QUERY_PASSPORT_CONFIG: i32 = 78;
pub const HTTP_QUERY_PASSPORT_URLS: i32 = 77;
pub const HTTP_QUERY_PRAGMA: i32 = 17;
pub const HTTP_QUERY_PROXY_AUTHENTICATE: i32 = 41;
pub const HTTP_QUERY_PROXY_AUTHORIZATION: i32 = 61;
pub const HTTP_QUERY_PROXY_CONNECTION: i32 = 69;
pub const HTTP_QUERY_PROXY_SUPPORT: i32 = 75;
pub const HTTP_QUERY_PUBLIC: i32 = 8;
pub const HTTP_QUERY_PUBLIC_KEY_PINS: i32 = 94;
pub const HTTP_QUERY_PUBLIC_KEY_PINS_REPORT_ONLY: i32 = 95;
pub const HTTP_QUERY_RANGE: i32 = 62;
pub const HTTP_QUERY_RAW_HEADERS: i32 = 21;
pub const HTTP_QUERY_RAW_HEADERS_CRLF: i32 = 22;
pub const HTTP_QUERY_REFERER: i32 = 35;
pub const HTTP_QUERY_REFRESH: i32 = 46;
pub const HTTP_QUERY_REQUEST_METHOD: i32 = 45;
pub const HTTP_QUERY_RETRY_AFTER: i32 = 36;
pub const HTTP_QUERY_SERVER: i32 = 37;
pub const HTTP_QUERY_SET_COOKIE: i32 = 43;
pub const HTTP_QUERY_SET_COOKIE2: i32 = 87;
pub const HTTP_QUERY_STATUS_CODE: i32 = 19;
pub const HTTP_QUERY_STATUS_TEXT: i32 = 20;
pub const HTTP_QUERY_STRICT_TRANSPORT_SECURITY: i32 = 91;
pub const HTTP_QUERY_TITLE: i32 = 38;
pub const HTTP_QUERY_TOKEN_BINDING: i32 = 92;
pub const HTTP_QUERY_TRANSFER_ENCODING: i32 = 63;
pub const HTTP_QUERY_TRANSLATE: i32 = 82;
pub const HTTP_QUERY_UNLESS_MODIFIED_SINCE: i32 = 70;
pub const HTTP_QUERY_UPGRADE: i32 = 64;
pub const HTTP_QUERY_URI: i32 = 13;
pub const HTTP_QUERY_USER_AGENT: i32 = 39;
pub const HTTP_QUERY_VARY: i32 = 65;
pub const HTTP_QUERY_VERSION: i32 = 18;
pub const HTTP_QUERY_VIA: i32 = 66;
pub const HTTP_QUERY_WARNING: i32 = 67;
pub const HTTP_QUERY_WWW_AUTHENTICATE: i32 = 40;
pub const HTTP_QUERY_X_CONTENT_TYPE_OPTIONS: i32 = 79;
pub const HTTP_QUERY_X_FRAME_OPTIONS: i32 = 85;
pub const HTTP_QUERY_X_P2P_PEERDIST: i32 = 81;
pub const HTTP_QUERY_X_UA_COMPATIBLE: i32 = 83;
pub const HTTP_QUERY_X_XSS_PROTECTION: i32 = 86;
pub const HTTP_STATUS_MISDIRECTED_REQUEST: i32 = 421;
pub const HTTP_VERSIONA: windows_sys::core::PCSTR = windows_sys::core::s!("HTTP/1.0");
pub const HTTP_VERSIONW: windows_sys::core::PCWSTR = windows_sys::core::w!("HTTP/1.0");
pub const ICU_USERNAME: i32 = 1073741824;
pub const IDSI_FLAG_KEEP_ALIVE: i32 = 1;
pub const IDSI_FLAG_PROXY: i32 = 4;
pub const IDSI_FLAG_SECURE: i32 = 2;
pub const IDSI_FLAG_TUNNEL: i32 = 8;
pub const INTERENT_GOONLINE_MASK: i32 = 3;
pub const INTERENT_GOONLINE_NOPROMPT: i32 = 2;
pub const INTERENT_GOONLINE_REFRESH: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERNET_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERNET_AUTH_NOTIFY_DATA {
    pub cbStruct: u32,
    pub dwOptions: u32,
    pub pfnNotify: PFN_AUTH_NOTIFY,
    pub dwContext: usize,
}
pub const INTERNET_AUTODIAL_FAILIFSECURITYCHECK: i32 = 4;
pub const INTERNET_AUTODIAL_FLAGS_MASK: i32 = 15;
pub const INTERNET_AUTODIAL_FORCE_ONLINE: i32 = 1;
pub const INTERNET_AUTODIAL_FORCE_UNATTENDED: i32 = 2;
pub const INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT: i32 = 8;
pub type INTERNET_BUFFERS = INTERNET_BUFFERSA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_BUFFERSA {
    pub dwStructSize: u32,
    pub Next: *mut Self,
    pub lpcszHeader: windows_sys::core::PCSTR,
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
#[derive(Clone, Copy)]
pub struct INTERNET_BUFFERSW {
    pub dwStructSize: u32,
    pub Next: *mut Self,
    pub lpcszHeader: windows_sys::core::PCWSTR,
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
    pub lpszSourceUrlName: windows_sys::core::PSTR,
    pub lpszLocalFileName: windows_sys::core::PSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::FILETIME,
    pub ExpireTime: super::FILETIME,
    pub LastAccessTime: super::FILETIME,
    pub LastSyncTime: super::FILETIME,
    pub lpHeaderInfo: windows_sys::core::PSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: windows_sys::core::PSTR,
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
    pub lpszSourceUrlName: windows_sys::core::PWSTR,
    pub lpszLocalFileName: windows_sys::core::PWSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::FILETIME,
    pub ExpireTime: super::FILETIME,
    pub LastAccessTime: super::FILETIME,
    pub LastSyncTime: super::FILETIME,
    pub lpHeaderInfo: windows_sys::core::PWSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: windows_sys::core::PWSTR,
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
pub const INTERNET_CACHE_GROUP_ADD: i32 = 0;
pub type INTERNET_CACHE_GROUP_INFO = INTERNET_CACHE_GROUP_INFOA;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
pub const INTERNET_CACHE_GROUP_REMOVE: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct INTERNET_CACHE_TIMESTAMPS {
    pub ftExpires: super::FILETIME,
    pub ftLastModified: super::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct INTERNET_CERTIFICATE_INFO {
    pub ftExpiry: super::FILETIME,
    pub ftStart: super::FILETIME,
    pub lpszSubjectInfo: super::LPTSTR,
    pub lpszIssuerInfo: super::LPTSTR,
    pub lpszProtocolName: super::LPTSTR,
    pub lpszSignatureAlgName: super::LPTSTR,
    pub lpszEncryptionAlgName: super::LPTSTR,
    pub dwKeySize: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for INTERNET_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERNET_CONNECTED_INFO {
    pub dwConnectedState: u32,
    pub dwFlags: u32,
}
pub const INTERNET_CONNECTION_CONFIGURED: i32 = 64;
pub const INTERNET_CONNECTION_LAN: i32 = 2;
pub const INTERNET_CONNECTION_MODEM: i32 = 1;
pub const INTERNET_CONNECTION_MODEM_BUSY: i32 = 8;
pub const INTERNET_CONNECTION_OFFLINE: i32 = 32;
pub const INTERNET_CONNECTION_PROXY: i32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct INTERNET_COOKIE2 {
    pub pwszName: windows_sys::core::PWSTR,
    pub pwszValue: windows_sys::core::PWSTR,
    pub pwszDomain: windows_sys::core::PWSTR,
    pub pwszPath: windows_sys::core::PWSTR,
    pub dwFlags: u32,
    pub ftExpires: super::FILETIME,
    pub fExpiresSet: windows_sys::core::BOOL,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_COOKIE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_COOKIE_APPLY_HOST_ONLY: i32 = 32768;
pub const INTERNET_COOKIE_APPLY_P3P: i32 = 128;
pub const INTERNET_COOKIE_EVALUATE_P3P: i32 = 64;
pub const INTERNET_COOKIE_HOST_ONLY: i32 = 16384;
pub const INTERNET_COOKIE_HOST_ONLY_APPLIED: i32 = 524288;
pub const INTERNET_COOKIE_HTTPONLY: i32 = 8192;
pub const INTERNET_COOKIE_IE6: i32 = 1024;
pub const INTERNET_COOKIE_IS_LEGACY: i32 = 2048;
pub const INTERNET_COOKIE_IS_RESTRICTED: i32 = 512;
pub const INTERNET_COOKIE_IS_SECURE: i32 = 1;
pub const INTERNET_COOKIE_IS_SESSION: i32 = 2;
pub const INTERNET_COOKIE_NON_SCRIPT: i32 = 4096;
pub const INTERNET_COOKIE_P3P_ENABLED: i32 = 256;
pub const INTERNET_COOKIE_PROMPT_REQUIRED: i32 = 32;
pub const INTERNET_COOKIE_SAME_SITE_LAX: i32 = 2097152;
pub const INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE: i32 = 4194304;
pub const INTERNET_COOKIE_SAME_SITE_STRICT: i32 = 1048576;
pub const INTERNET_COOKIE_THIRD_PARTY: i32 = 16;
pub const INTERNET_CUSTOMDIAL_CAN_HANGUP: i32 = 4;
pub const INTERNET_CUSTOMDIAL_CONNECT: i32 = 0;
pub const INTERNET_CUSTOMDIAL_DISCONNECT: i32 = 2;
pub const INTERNET_CUSTOMDIAL_SAFE_FOR_UNATTENDED: i32 = 1;
pub const INTERNET_CUSTOMDIAL_SHOWOFFLINE: i32 = 4;
pub const INTERNET_CUSTOMDIAL_UNATTENDED: i32 = 1;
pub const INTERNET_CUSTOMDIAL_WILL_SUPPLY_STATE: i32 = 2;
pub const INTERNET_DEFAULT_FTP_PORT: i32 = 21;
pub const INTERNET_DEFAULT_GOPHER_PORT: i32 = 70;
pub const INTERNET_DEFAULT_SOCKS_PORT: i32 = 1080;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERNET_DIAGNOSTIC_SOCKET_INFO {
    pub Socket: usize,
    pub SourcePort: u32,
    pub DestPort: u32,
    pub Flags: u32,
}
pub const INTERNET_DIALSTATE_DISCONNECTED: i32 = 1;
pub const INTERNET_DIAL_FORCE_PROMPT: i32 = 8192;
pub const INTERNET_DIAL_SHOW_OFFLINE: i32 = 16384;
pub const INTERNET_DIAL_UNATTENDED: i32 = 32768;
pub const INTERNET_ERROR_BASE: i32 = 12000;
pub const INTERNET_ERROR_LAST: i32 = 12192;
pub const INTERNET_ERROR_MASK_COMBINED_SEC_CERT: i32 = 2;
pub const INTERNET_ERROR_MASK_INSERT_CDROM: i32 = 1;
pub const INTERNET_ERROR_MASK_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: i32 = 8;
pub const INTERNET_ERROR_MASK_NEED_MSN_SSPI_PKG: i32 = 4;
pub const INTERNET_FIRST_OPTION: i32 = 1;
pub const INTERNET_FLAG_ASYNC: i32 = 268435456;
pub const INTERNET_FLAG_CACHE_ASYNC: i32 = 128;
pub const INTERNET_FLAG_CACHE_IF_NET_FAIL: i32 = 65536;
pub const INTERNET_FLAG_DONT_CACHE: i32 = 67108864;
pub const INTERNET_FLAG_EXISTING_CONNECT: i32 = 536870912;
pub const INTERNET_FLAG_FORMS_SUBMIT: i32 = 64;
pub const INTERNET_FLAG_FROM_CACHE: i32 = 16777216;
pub const INTERNET_FLAG_FWD_BACK: i32 = 32;
pub const INTERNET_FLAG_HYPERLINK: i32 = 1024;
pub const INTERNET_FLAG_IDN_DIRECT: i32 = 1;
pub const INTERNET_FLAG_IDN_PROXY: i32 = 2;
pub const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: i32 = 4096;
pub const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: i32 = 8192;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP: i32 = 32768;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS: i32 = 16384;
pub const INTERNET_FLAG_KEEP_CONNECTION: i32 = 4194304;
pub const INTERNET_FLAG_MAKE_PERSISTENT: i32 = 33554432;
pub const INTERNET_FLAG_MUST_CACHE_REQUEST: i32 = 16;
pub const INTERNET_FLAG_NEED_FILE: i32 = 16;
pub const INTERNET_FLAG_NO_AUTH: i32 = 262144;
pub const INTERNET_FLAG_NO_AUTO_REDIRECT: i32 = 2097152;
pub const INTERNET_FLAG_NO_CACHE_WRITE: i32 = 67108864;
pub const INTERNET_FLAG_NO_COOKIES: i32 = 524288;
pub const INTERNET_FLAG_NO_UI: i32 = 512;
pub const INTERNET_FLAG_OFFLINE: i32 = 16777216;
pub const INTERNET_FLAG_PASSIVE: i32 = 134217728;
pub const INTERNET_FLAG_PRAGMA_NOCACHE: i32 = 256;
pub const INTERNET_FLAG_RAW_DATA: i32 = 1073741824;
pub const INTERNET_FLAG_READ_PREFETCH: i32 = 1048576;
pub const INTERNET_FLAG_RELOAD: u32 = 2147483648;
pub const INTERNET_FLAG_RESTRICTED_ZONE: i32 = 131072;
pub const INTERNET_FLAG_RESYNCHRONIZE: i32 = 2048;
pub const INTERNET_FLAG_SECURE: i32 = 8388608;
pub const INTERNET_FLAG_TRANSFER_ASCII: i32 = 1;
pub const INTERNET_FLAG_TRANSFER_BINARY: i32 = 2;
pub const INTERNET_HANDLE_TYPE_CONNECT_FTP: i32 = 2;
pub const INTERNET_HANDLE_TYPE_CONNECT_GOPHER: i32 = 3;
pub const INTERNET_HANDLE_TYPE_CONNECT_HTTP: i32 = 4;
pub const INTERNET_HANDLE_TYPE_FILE_REQUEST: i32 = 14;
pub const INTERNET_HANDLE_TYPE_FTP_FILE: i32 = 7;
pub const INTERNET_HANDLE_TYPE_FTP_FILE_HTML: i32 = 8;
pub const INTERNET_HANDLE_TYPE_FTP_FIND: i32 = 5;
pub const INTERNET_HANDLE_TYPE_FTP_FIND_HTML: i32 = 6;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE: i32 = 11;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE_HTML: i32 = 12;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND: i32 = 9;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND_HTML: i32 = 10;
pub const INTERNET_HANDLE_TYPE_HTTP_REQUEST: i32 = 13;
pub const INTERNET_HANDLE_TYPE_INTERNET: i32 = 1;
pub const INTERNET_IDENTITY_FLAG_CLEAR_CONTENT: i32 = 32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_COOKIES: i32 = 8;
pub const INTERNET_IDENTITY_FLAG_CLEAR_DATA: i32 = 4;
pub const INTERNET_IDENTITY_FLAG_CLEAR_HISTORY: i32 = 16;
pub const INTERNET_IDENTITY_FLAG_PRIVATE_CACHE: i32 = 1;
pub const INTERNET_IDENTITY_FLAG_SHARED_CACHE: i32 = 2;
pub const INTERNET_INVALID_PORT_NUMBER: i32 = 0;
pub const INTERNET_KEEP_ALIVE_DISABLED: i32 = 0;
pub const INTERNET_KEEP_ALIVE_ENABLED: i32 = 1;
pub const INTERNET_KEEP_ALIVE_UNKNOWN: u32 = 4294967295;
pub const INTERNET_LAST_OPTION: i32 = 193;
pub const INTERNET_MAX_HOST_NAME_LENGTH: i32 = 256;
pub const INTERNET_MAX_PASSWORD_LENGTH: i32 = 128;
pub const INTERNET_MAX_PATH_LENGTH: i32 = 2048;
pub const INTERNET_MAX_PORT_NUMBER_LENGTH: i32 = 5;
pub const INTERNET_MAX_PORT_NUMBER_VALUE: i32 = 65535;
pub const INTERNET_MAX_SCHEME_LENGTH: i32 = 32;
pub const INTERNET_MAX_USER_NAME_LENGTH: i32 = 128;
pub const INTERNET_NO_CALLBACK: i32 = 0;
pub const INTERNET_OPEN_TYPE_DIRECT: i32 = 1;
pub const INTERNET_OPEN_TYPE_PRECONFIG: i32 = 0;
pub const INTERNET_OPEN_TYPE_PRECONFIG_WITH_NO_AUTOPROXY: i32 = 4;
pub const INTERNET_OPEN_TYPE_PROXY: i32 = 3;
pub const INTERNET_OPTION_ACTIVATE_WORKER_THREADS: i32 = 92;
pub const INTERNET_OPTION_ALTER_IDENTITY: i32 = 80;
pub const INTERNET_OPTION_ASYNC: i32 = 30;
pub const INTERNET_OPTION_ASYNC_ID: i32 = 15;
pub const INTERNET_OPTION_ASYNC_PRIORITY: i32 = 16;
pub const INTERNET_OPTION_AUTH_FLAGS: i32 = 85;
pub const INTERNET_OPTION_AUTODIAL_CONNECTION: i32 = 83;
pub const INTERNET_OPTION_AUTODIAL_MODE: i32 = 82;
pub const INTERNET_OPTION_BYPASS_EDITED_ENTRY: i32 = 64;
pub const INTERNET_OPTION_CACHE_STREAM_HANDLE: i32 = 27;
pub const INTERNET_OPTION_CACHE_TIMESTAMPS: i32 = 69;
pub const INTERNET_OPTION_CALLBACK: i32 = 1;
pub const INTERNET_OPTION_CALLBACK_FILTER: i32 = 54;
pub const INTERNET_OPTION_CLIENT_CERT_CONTEXT: i32 = 84;
pub const INTERNET_OPTION_CODEPAGE: i32 = 68;
pub const INTERNET_OPTION_CODEPAGE_EXTRA: i32 = 101;
pub const INTERNET_OPTION_CODEPAGE_PATH: i32 = 100;
pub const INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH: i32 = 147;
pub const INTERNET_OPTION_CONNECTED_STATE: i32 = 50;
pub const INTERNET_OPTION_CONNECTION_FILTER: i32 = 162;
pub const INTERNET_OPTION_CONNECT_BACKOFF: i32 = 4;
pub const INTERNET_OPTION_CONNECT_LIMIT: i32 = 46;
pub const INTERNET_OPTION_CONNECT_RETRIES: i32 = 3;
pub const INTERNET_OPTION_CONNECT_TIME: i32 = 55;
pub const INTERNET_OPTION_CONNECT_TIMEOUT: i32 = 2;
pub const INTERNET_OPTION_CONTEXT_VALUE: i32 = 45;
pub const INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT: i32 = 6;
pub const INTERNET_OPTION_CONTROL_SEND_TIMEOUT: i32 = 5;
pub const INTERNET_OPTION_COOKIES_3RD_PARTY: i32 = 86;
pub const INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL: i32 = 187;
pub const INTERNET_OPTION_DATAFILE_EXT: i32 = 96;
pub const INTERNET_OPTION_DATAFILE_NAME: i32 = 33;
pub const INTERNET_OPTION_DATA_RECEIVE_TIMEOUT: i32 = 8;
pub const INTERNET_OPTION_DATA_SEND_TIMEOUT: i32 = 7;
pub const INTERNET_OPTION_DIAGNOSTIC_SOCKET_INFO: i32 = 67;
pub const INTERNET_OPTION_DIGEST_AUTH_UNLOAD: i32 = 76;
pub const INTERNET_OPTION_DISABLE_AUTODIAL: i32 = 70;
pub const INTERNET_OPTION_DISABLE_PASSPORT_AUTH: i32 = 87;
pub const INTERNET_OPTION_DISCONNECTED_TIMEOUT: i32 = 49;
pub const INTERNET_OPTION_ENABLE_HTTP_PROTOCOL: i32 = 148;
pub const INTERNET_OPTION_ENABLE_PASSPORT_AUTH: i32 = 90;
pub const INTERNET_OPTION_ENABLE_REDIRECT_CACHE_READ: i32 = 122;
pub const INTERNET_OPTION_ENCODE_EXTRA: i32 = 155;
pub const INTERNET_OPTION_END_BROWSER_SESSION: i32 = 42;
pub const INTERNET_OPTION_ENTERPRISE_CONTEXT: i32 = 159;
pub const INTERNET_OPTION_ERROR_MASK: i32 = 62;
pub const INTERNET_OPTION_EXEMPT_CONNECTION_LIMIT: i32 = 89;
pub const INTERNET_OPTION_EXTENDED_ERROR: i32 = 24;
pub const INTERNET_OPTION_FROM_CACHE_TIMEOUT: i32 = 63;
pub const INTERNET_OPTION_HANDLE_TYPE: i32 = 9;
pub const INTERNET_OPTION_HIBERNATE_INACTIVE_WORKER_THREADS: i32 = 91;
pub const INTERNET_OPTION_HSTS: i32 = 157;
pub const INTERNET_OPTION_HTTP_DECODING: i32 = 65;
pub const INTERNET_OPTION_HTTP_PROTOCOL_USED: i32 = 149;
pub const INTERNET_OPTION_HTTP_VERSION: i32 = 59;
pub const INTERNET_OPTION_IDENTITY: i32 = 78;
pub const INTERNET_OPTION_IDLE_STATE: i32 = 51;
pub const INTERNET_OPTION_IDN: i32 = 102;
pub const INTERNET_OPTION_IGNORE_OFFLINE: i32 = 77;
pub const INTERNET_OPTION_KEEP_CONNECTION: i32 = 22;
pub const INTERNET_OPTION_LISTEN_TIMEOUT: i32 = 11;
pub const INTERNET_OPTION_MAX_CONNS_PER_1_0_SERVER: i32 = 74;
pub const INTERNET_OPTION_MAX_CONNS_PER_PROXY: i32 = 103;
pub const INTERNET_OPTION_MAX_CONNS_PER_SERVER: i32 = 73;
pub const INTERNET_OPTION_OFFLINE_MODE: i32 = 26;
pub const INTERNET_OPTION_OFFLINE_SEMANTICS: i32 = 52;
pub const INTERNET_OPTION_PARENT_HANDLE: i32 = 21;
pub const INTERNET_OPTION_PASSWORD: i32 = 29;
pub const INTERNET_OPTION_PER_CONNECTION_OPTION: i32 = 75;
pub const INTERNET_OPTION_POLICY: i32 = 48;
pub const INTERNET_OPTION_PROXY: i32 = 38;
pub const INTERNET_OPTION_PROXY_PASSWORD: i32 = 44;
pub const INTERNET_OPTION_PROXY_SETTINGS_CHANGED: i32 = 95;
pub const INTERNET_OPTION_PROXY_USERNAME: i32 = 43;
pub const INTERNET_OPTION_READ_BUFFER_SIZE: i32 = 12;
pub const INTERNET_OPTION_RECEIVE_THROUGHPUT: i32 = 57;
pub const INTERNET_OPTION_RECEIVE_TIMEOUT: i32 = 6;
pub const INTERNET_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: i32 = 163;
pub const INTERNET_OPTION_REFRESH: i32 = 37;
pub const INTERNET_OPTION_REMOVE_IDENTITY: i32 = 79;
pub const INTERNET_OPTION_REQUEST_ANNOTATION: i32 = 193;
pub const INTERNET_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: i32 = 64000;
pub const INTERNET_OPTION_REQUEST_FLAGS: i32 = 23;
pub const INTERNET_OPTION_REQUEST_PRIORITY: i32 = 58;
pub const INTERNET_OPTION_RESET_URLCACHE_SESSION: i32 = 60;
pub const INTERNET_OPTION_RESTORE_WORKER_THREAD_DEFAULTS: i32 = 93;
pub const INTERNET_OPTION_SECONDARY_CACHE_KEY: i32 = 53;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE: i32 = 35;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE_STRUCT: i32 = 32;
pub const INTERNET_OPTION_SECURITY_FLAGS: i32 = 31;
pub const INTERNET_OPTION_SECURITY_KEY_BITNESS: i32 = 36;
pub const INTERNET_OPTION_SECURITY_SELECT_CLIENT_CERT: i32 = 47;
pub const INTERNET_OPTION_SEND_THROUGHPUT: i32 = 56;
pub const INTERNET_OPTION_SEND_TIMEOUT: i32 = 5;
pub const INTERNET_OPTION_SEND_UTF8_SERVERNAME_TO_PROXY: i32 = 88;
pub const INTERNET_OPTION_SERVER_CERT_CHAIN_CONTEXT: i32 = 105;
pub const INTERNET_OPTION_SETTINGS_CHANGED: i32 = 39;
pub const INTERNET_OPTION_SOCKET_SEND_BUFFER_LENGTH: i32 = 94;
pub const INTERNET_OPTION_SUPPRESS_BEHAVIOR: i32 = 81;
pub const INTERNET_OPTION_SUPPRESS_SERVER_AUTH: i32 = 104;
pub const INTERNET_OPTION_TOKEN_BINDING_PUBLIC_KEY: i32 = 181;
pub const INTERNET_OPTION_URL: i32 = 34;
pub const INTERNET_OPTION_USERNAME: i32 = 28;
pub const INTERNET_OPTION_USER_AGENT: i32 = 41;
pub const INTERNET_OPTION_VERSION: i32 = 40;
pub const INTERNET_OPTION_WRITE_BUFFER_SIZE: i32 = 13;
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_TIME: i32 = 8;
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_URL: i32 = 9;
pub const INTERNET_PER_CONN_AUTOCONFIG_RELOAD_DELAY_MINS: i32 = 7;
pub const INTERNET_PER_CONN_AUTOCONFIG_SECONDARY_URL: i32 = 6;
pub const INTERNET_PER_CONN_AUTOCONFIG_URL: i32 = 4;
pub const INTERNET_PER_CONN_AUTODISCOVERY_FLAGS: i32 = 5;
pub const INTERNET_PER_CONN_FLAGS: i32 = 1;
pub const INTERNET_PER_CONN_FLAGS_UI: i32 = 10;
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
    pub pszValue: windows_sys::core::PSTR,
    pub ftValue: super::FILETIME,
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
    pub pszValue: windows_sys::core::PWSTR,
    pub ftValue: super::FILETIME,
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
#[derive(Clone, Copy)]
pub struct INTERNET_PER_CONN_OPTION_LISTA {
    pub dwSize: u32,
    pub pszConnection: windows_sys::core::PSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: LPINTERNET_PER_CONN_OPTIONA,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_PER_CONN_OPTION_LISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct INTERNET_PER_CONN_OPTION_LISTW {
    pub dwSize: u32,
    pub pszConnection: windows_sys::core::PWSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: LPINTERNET_PER_CONN_OPTIONW,
}
#[cfg(feature = "minwindef")]
impl Default for INTERNET_PER_CONN_OPTION_LISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_PER_CONN_PROXY_BYPASS: i32 = 3;
pub const INTERNET_PER_CONN_PROXY_SERVER: i32 = 2;
pub const INTERNET_PRIORITY_FOREGROUND: i32 = 1000;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct INTERNET_PROXY_INFO {
    pub dwAccessType: u32,
    pub lpszProxy: super::LPCTSTR,
    pub lpszProxyBypass: super::LPCTSTR,
}
#[cfg(feature = "winnt")]
impl Default for INTERNET_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_RAS_INSTALLED: i32 = 16;
pub const INTERNET_REQFLAG_ASYNC: i32 = 2;
pub const INTERNET_REQFLAG_CACHE_WRITE_DISABLED: i32 = 64;
pub const INTERNET_REQFLAG_FROM_CACHE: i32 = 1;
pub const INTERNET_REQFLAG_NET_TIMEOUT: i32 = 128;
pub const INTERNET_REQFLAG_NO_HEADERS: i32 = 8;
pub const INTERNET_REQFLAG_PASSIVE: i32 = 16;
pub const INTERNET_REQFLAG_VIA_PROXY: i32 = 4;
pub const INTERNET_RFC1123_BUFSIZE: i32 = 30;
pub const INTERNET_RFC1123_FORMAT: i32 = 0;
pub const INTERNET_SERVICE_FTP: i32 = 1;
pub const INTERNET_SERVICE_GOPHER: i32 = 2;
pub const INTERNET_SERVICE_HTTP: i32 = 3;
pub const INTERNET_STATE_BUSY: i32 = 512;
pub const INTERNET_STATE_CONNECTED: i32 = 1;
pub const INTERNET_STATE_DISCONNECTED: i32 = 2;
pub const INTERNET_STATE_DISCONNECTED_BY_USER: i32 = 16;
pub const INTERNET_STATE_IDLE: i32 = 256;
#[cfg(feature = "winhttp")]
pub type INTERNET_STATUS_CALLBACK = Option<unsafe extern "system" fn(hinternet: super::HINTERNET, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *const core::ffi::c_void, dwstatusinformationlength: u32)>;
pub const INTERNET_STATUS_CLOSING_CONNECTION: i32 = 50;
pub const INTERNET_STATUS_CONNECTED_TO_SERVER: i32 = 21;
pub const INTERNET_STATUS_CONNECTING_TO_SERVER: i32 = 20;
pub const INTERNET_STATUS_CONNECTION_CLOSED: i32 = 51;
pub const INTERNET_STATUS_COOKIE_HISTORY: i32 = 327;
pub const INTERNET_STATUS_COOKIE_RECEIVED: i32 = 321;
pub const INTERNET_STATUS_COOKIE_SENT: i32 = 320;
pub const INTERNET_STATUS_CTL_RESPONSE_RECEIVED: i32 = 42;
pub const INTERNET_STATUS_DETECTING_PROXY: i32 = 80;
pub const INTERNET_STATUS_HANDLE_CLOSING: i32 = 70;
pub const INTERNET_STATUS_HANDLE_CREATED: i32 = 60;
pub const INTERNET_STATUS_INTERMEDIATE_RESPONSE: i32 = 120;
pub const INTERNET_STATUS_NAME_RESOLVED: i32 = 11;
pub const INTERNET_STATUS_P3P_HEADER: i32 = 325;
pub const INTERNET_STATUS_P3P_POLICYREF: i32 = 326;
pub const INTERNET_STATUS_PREFETCH: i32 = 43;
pub const INTERNET_STATUS_PRIVACY_IMPACTED: i32 = 324;
pub const INTERNET_STATUS_RECEIVING_RESPONSE: i32 = 40;
pub const INTERNET_STATUS_REDIRECT: i32 = 110;
pub const INTERNET_STATUS_REQUEST_COMPLETE: i32 = 100;
pub const INTERNET_STATUS_REQUEST_SENT: i32 = 31;
pub const INTERNET_STATUS_RESOLVING_NAME: i32 = 10;
pub const INTERNET_STATUS_RESPONSE_RECEIVED: i32 = 41;
pub const INTERNET_STATUS_SENDING_REQUEST: i32 = 30;
pub const INTERNET_STATUS_STATE_CHANGE: i32 = 200;
pub const INTERNET_STATUS_USER_INPUT_REQUIRED: i32 = 140;
pub const INTERNET_SUPPRESS_COOKIE_POLICY: i32 = 1;
pub const INTERNET_SUPPRESS_COOKIE_POLICY_RESET: i32 = 2;
pub const INTERNET_SUPPRESS_RESET_ALL: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERNET_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
pub const IRF_ASYNC: i32 = 1;
pub const IRF_NO_WAIT: i32 = 8;
pub const IRF_SYNC: i32 = 4;
pub const IRF_USE_CONTEXT: i32 = 8;
pub const ISO_FORCE_DISCONNECTED: i32 = 1;
pub const ISO_GLOBAL: i32 = 1;
pub const ISO_REGISTRY: i32 = 2;
pub const ISO_VALID_FLAGS: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct InternetCookieHistory {
    pub fAccepted: windows_sys::core::BOOL,
    pub fLeashed: windows_sys::core::BOOL,
    pub fDowngraded: windows_sys::core::BOOL,
    pub fRejected: windows_sys::core::BOOL,
}
pub type InternetCookieState = i32;
pub const LOCAL_INTERNET_ACCESS: i32 = 1;
pub type LPAUTO_PROXY_SCRIPT_BUFFER = *mut AUTO_PROXY_SCRIPT_BUFFER;
#[cfg(feature = "winnt")]
pub type LPGOPHER_ABSTRACT_ATTRIBUTE_TYPE = *mut GOPHER_ABSTRACT_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_ADMIN_ATTRIBUTE_TYPE = *mut GOPHER_ADMIN_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_ASK_ATTRIBUTE_TYPE = *mut GOPHER_ASK_ATTRIBUTE_TYPE;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPGOPHER_ATTRIBUTE_TYPE = *mut GOPHER_ATTRIBUTE_TYPE;
#[cfg(feature = "minwindef")]
pub type LPGOPHER_FIND_DATA = LPGOPHER_FIND_DATAA;
#[cfg(feature = "minwindef")]
pub type LPGOPHER_FIND_DATAA = *mut GOPHER_FIND_DATAA;
#[cfg(feature = "minwindef")]
pub type LPGOPHER_FIND_DATAW = *mut GOPHER_FIND_DATAW;
pub type LPGOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE = *mut GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_LOCATION_ATTRIBUTE_TYPE = *mut GOPHER_LOCATION_ATTRIBUTE_TYPE;
#[cfg(feature = "minwindef")]
pub type LPGOPHER_MOD_DATE_ATTRIBUTE_TYPE = *mut GOPHER_MOD_DATE_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_ORGANIZATION_ATTRIBUTE_TYPE = *mut GOPHER_ORGANIZATION_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_PROVIDER_ATTRIBUTE_TYPE = *mut GOPHER_PROVIDER_ATTRIBUTE_TYPE;
pub type LPGOPHER_SCORE_ATTRIBUTE_TYPE = *mut GOPHER_SCORE_ATTRIBUTE_TYPE;
pub type LPGOPHER_SCORE_RANGE_ATTRIBUTE_TYPE = *mut GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_SITE_ATTRIBUTE_TYPE = *mut GOPHER_SITE_ATTRIBUTE_TYPE;
pub type LPGOPHER_TIMEZONE_ATTRIBUTE_TYPE = *mut GOPHER_TIMEZONE_ATTRIBUTE_TYPE;
pub type LPGOPHER_TTL_ATTRIBUTE_TYPE = *mut GOPHER_TTL_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_UNKNOWN_ATTRIBUTE_TYPE = *mut GOPHER_UNKNOWN_ATTRIBUTE_TYPE;
pub type LPGOPHER_VERONICA_ATTRIBUTE_TYPE = *mut GOPHER_VERONICA_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_VERSION_ATTRIBUTE_TYPE = *mut GOPHER_VERSION_ATTRIBUTE_TYPE;
#[cfg(feature = "winnt")]
pub type LPGOPHER_VIEW_ATTRIBUTE_TYPE = *mut GOPHER_VIEW_ATTRIBUTE_TYPE;
pub type LPINTERNET_ASYNC_RESULT = *mut INTERNET_ASYNC_RESULT;
pub type LPINTERNET_BUFFERS = LPINTERNET_BUFFERSA;
pub type LPINTERNET_BUFFERSA = *mut INTERNET_BUFFERSA;
pub type LPINTERNET_BUFFERSW = *mut INTERNET_BUFFERSW;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_CACHE_ENTRY_INFO = LPINTERNET_CACHE_ENTRY_INFOA;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_CACHE_ENTRY_INFOA = *mut INTERNET_CACHE_ENTRY_INFOA;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_CACHE_ENTRY_INFOW = *mut INTERNET_CACHE_ENTRY_INFOW;
pub type LPINTERNET_CACHE_GROUP_INFO = LPINTERNET_CACHE_GROUP_INFOA;
pub type LPINTERNET_CACHE_GROUP_INFOA = *mut INTERNET_CACHE_GROUP_INFOA;
pub type LPINTERNET_CACHE_GROUP_INFOW = *mut INTERNET_CACHE_GROUP_INFOW;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_CACHE_TIMESTAMPS = *mut INTERNET_CACHE_TIMESTAMPS;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPINTERNET_CERTIFICATE_INFO = *mut INTERNET_CERTIFICATE_INFO;
pub type LPINTERNET_CONNECTED_INFO = *mut INTERNET_CONNECTED_INFO;
pub type LPINTERNET_DIAGNOSTIC_SOCKET_INFO = *mut INTERNET_DIAGNOSTIC_SOCKET_INFO;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_PER_CONN_OPTION = LPINTERNET_PER_CONN_OPTIONA;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_PER_CONN_OPTIONA = *mut INTERNET_PER_CONN_OPTIONA;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_PER_CONN_OPTIONW = *mut INTERNET_PER_CONN_OPTIONW;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_PER_CONN_OPTION_LIST = LPINTERNET_PER_CONN_OPTION_LISTA;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_PER_CONN_OPTION_LISTA = *mut INTERNET_PER_CONN_OPTION_LISTA;
#[cfg(feature = "minwindef")]
pub type LPINTERNET_PER_CONN_OPTION_LISTW = *mut INTERNET_PER_CONN_OPTION_LISTW;
#[cfg(feature = "winnt")]
pub type LPINTERNET_PROXY_INFO = *mut INTERNET_PROXY_INFO;
#[cfg(feature = "winhttp")]
pub type LPINTERNET_STATUS_CALLBACK = *mut INTERNET_STATUS_CALLBACK;
pub type LPINTERNET_VERSION_INFO = *mut INTERNET_VERSION_INFO;
#[cfg(feature = "winhttp")]
pub type LPURL_COMPONENTSA = *mut URL_COMPONENTSA;
pub const MAX_GOPHER_ATTRIBUTE_NAME: i32 = 128;
pub const MAX_GOPHER_CATEGORY_NAME: i32 = 128;
pub const MAX_GOPHER_DISPLAY_TEXT: i32 = 128;
pub const MAX_GOPHER_HOST_NAME: i32 = 256;
pub const MAX_GOPHER_LOCATOR_LENGTH: i32 = 653;
pub const MAX_GOPHER_SELECTOR_TEXT: i32 = 256;
pub const MIN_GOPHER_ATTRIBUTE_LENGTH: i32 = 256;
pub const NORMAL_CACHE_ENTRY: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type PFN_DIAL_HANDLER = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PCSTR, param2: u32, param3: *mut u32) -> u32>;
pub const PRE_CONFIG_INTERNET_ACCESS: i32 = 0;
pub const PRIVACY_TEMPLATE_ADVANCED: i32 = 101;
pub const PRIVACY_TEMPLATE_CUSTOM: i32 = 100;
pub const PRIVACY_TEMPLATE_HIGH: i32 = 1;
pub const PRIVACY_TEMPLATE_LOW: i32 = 5;
pub const PRIVACY_TEMPLATE_MAX: i32 = 5;
pub const PRIVACY_TEMPLATE_MEDIUM: i32 = 3;
pub const PRIVACY_TEMPLATE_MEDIUM_HIGH: i32 = 2;
pub const PRIVACY_TEMPLATE_MEDIUM_LOW: i32 = 4;
pub const PRIVACY_TEMPLATE_NO_COOKIES: i32 = 0;
pub const PRIVACY_TYPE_FIRST_PARTY: i32 = 0;
pub const PRIVACY_TYPE_THIRD_PARTY: i32 = 1;
pub const PROXY_AUTO_DETECT_TYPE_DHCP: i32 = 1;
pub const PROXY_AUTO_DETECT_TYPE_DNS_A: i32 = 2;
pub const PROXY_TYPE_AUTO_DETECT: i32 = 8;
pub const PROXY_TYPE_AUTO_PROXY_URL: i32 = 4;
pub const PROXY_TYPE_DIRECT: i32 = 1;
pub const PROXY_TYPE_PROXY: i32 = 2;
pub const SECURITY_FLAG_128BIT: i32 = 536870912;
pub const SECURITY_FLAG_40BIT: i32 = 268435456;
pub const SECURITY_FLAG_56BIT: i32 = 1073741824;
pub const SECURITY_FLAG_FORTEZZA: i32 = 134217728;
pub const SECURITY_FLAG_IETFSSL4: i32 = 32;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTP: i32 = 32768;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTPS: i32 = 16384;
pub const SECURITY_FLAG_IGNORE_REVOCATION: i32 = 128;
pub const SECURITY_FLAG_IGNORE_WEAK_SIGNATURE: i32 = 65536;
pub const SECURITY_FLAG_IGNORE_WRONG_USAGE: i32 = 512;
pub const SECURITY_FLAG_NORMALBITNESS: i32 = 268435456;
pub const SECURITY_FLAG_OPT_IN_WEAK_SIGNATURE: i32 = 131072;
pub const SECURITY_FLAG_PCT: i32 = 8;
pub const SECURITY_FLAG_PCT4: i32 = 16;
pub const SECURITY_FLAG_SSL: i32 = 2;
pub const SECURITY_FLAG_SSL3: i32 = 4;
pub const SECURITY_FLAG_UNKNOWNBIT: u32 = 2147483648;
pub const SECURITY_IGNORE_ERROR_MASK: i32 = 78208;
pub const SECURITY_INTERNET_MASK: i32 = 61440;
pub const SECURITY_SET_MASK: i32 = 78720;
pub const SPARSE_CACHE_ENTRY: i32 = 65536;
pub const STICKY_CACHE_ENTRY: i32 = 4;
pub const TRACK_OFFLINE_CACHE_ENTRY: i32 = 16;
pub const TRACK_ONLINE_CACHE_ENTRY: i32 = 32;
pub const URLCACHE_FIND_DEFAULT_FILTER: i32 = 3145781;
pub const URLHISTORY_CACHE_ENTRY: i32 = 2097152;
#[repr(C)]
#[cfg(feature = "winhttp")]
#[derive(Clone, Copy)]
pub struct URL_COMPONENTSA {
    pub dwStructSize: u32,
    pub lpszScheme: windows_sys::core::PSTR,
    pub dwSchemeLength: u32,
    pub nScheme: super::INTERNET_SCHEME,
    pub lpszHostName: windows_sys::core::PSTR,
    pub dwHostNameLength: u32,
    pub nPort: super::INTERNET_PORT,
    pub lpszUserName: windows_sys::core::PSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: windows_sys::core::PSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: windows_sys::core::PSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: windows_sys::core::PSTR,
    pub dwExtraInfoLength: u32,
}
#[cfg(feature = "winhttp")]
impl Default for URL_COMPONENTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WININET_API_FLAG_ASYNC: i32 = 1;
pub const WININET_API_FLAG_SYNC: i32 = 4;
pub const WININET_API_FLAG_USE_CONTEXT: i32 = 8;
pub type WPAD_CACHE_DELETE = i32;
pub const WPAD_CACHE_DELETE_ALL: WPAD_CACHE_DELETE = 1;
pub const WPAD_CACHE_DELETE_CURRENT: WPAD_CACHE_DELETE = 0;
pub type pfnInternetDeInitializeAutoProxyDll = Option<unsafe extern "system" fn(lpszmime: windows_sys::core::PCSTR, dwreserved: u32) -> windows_sys::core::BOOL>;
pub type pfnInternetGetProxyInfo = Option<unsafe extern "system" fn(lpszurl: windows_sys::core::PCSTR, dwurllength: u32, lpszurlhostname: windows_sys::core::PCSTR, dwurlhostnamelength: u32, lplpszproxyhostname: *mut windows_sys::core::PSTR, lpdwproxyhostnamelength: *mut u32) -> windows_sys::core::BOOL>;
pub type pfnInternetInitializeAutoProxyDll = Option<unsafe extern "system" fn(dwversion: u32, lpszdownloadedtempfile: windows_sys::core::PCSTR, lpszmime: windows_sys::core::PCSTR, lpautoproxycallbacks: *mut AutoProxyHelperFunctions, lpautoproxyscriptbuffer: *mut AUTO_PROXY_SCRIPT_BUFFER) -> windows_sys::core::BOOL>;
