#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheCheckManifest(pwszmasterurl: super::super::Foundation::PWSTR, pwszmanifesturl: super::super::Foundation::PWSTR, pbmanifestdata: *const u8, dwmanifestdatasize: u32, pbmanifestresponseheaders: *const u8, dwmanifestresponseheaderssize: u32, pestate: *mut APP_CACHE_STATE, phnewappcache: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn AppCacheCloseHandle(happcache: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheCreateAndCommitFile(happcache: *const ::core::ffi::c_void, pwszsourcefilepath: super::super::Foundation::PWSTR, pwszurl: super::super::Foundation::PWSTR, pbresponseheaders: *const u8, dwresponseheaderssize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheDeleteGroup(pwszmanifesturl: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheDeleteIEGroup(pwszmanifesturl: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn AppCacheDuplicateHandle(happcache: *const ::core::ffi::c_void, phduplicatedappcache: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn AppCacheFinalize(happcache: *const ::core::ffi::c_void, pbmanifestdata: *const u8, dwmanifestdatasize: u32, pestate: *mut APP_CACHE_FINALIZE_STATE) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeDownloadList(pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST);
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST);
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeIESpace(ftcutoff: super::super::Foundation::FILETIME) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeSpace(ftcutoff: super::super::Foundation::FILETIME) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetDownloadList(happcache: *const ::core::ffi::c_void, pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetFallbackUrl(happcache: *const ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, ppwszfallbackurl: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetIEGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetInfo(happcache: *const ::core::ffi::c_void, pappcacheinfo: *mut APP_CACHE_GROUP_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetManifestUrl(happcache: *const ::core::ffi::c_void, ppwszmanifesturl: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheLookup(pwszurl: super::super::Foundation::PWSTR, dwflags: u32, phappcache: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitUrlCacheEntryA(lpszurlname: super::super::Foundation::PSTR, lpszlocalfilename: super::super::Foundation::PSTR, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpheaderinfo: *const u8, cchheaderinfo: u32, lpszfileextension: super::super::Foundation::PSTR, lpszoriginalurl: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitUrlCacheEntryBinaryBlob(pwszurlname: super::super::Foundation::PWSTR, dwtype: u32, ftexpiretime: super::super::Foundation::FILETIME, ftmodifiedtime: super::super::Foundation::FILETIME, pbblob: *const u8, cbblob: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitUrlCacheEntryW(lpszurlname: super::super::Foundation::PWSTR, lpszlocalfilename: super::super::Foundation::PWSTR, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpszheaderinfo: super::super::Foundation::PWSTR, cchheaderinfo: u32, lpszfileextension: super::super::Foundation::PWSTR, lpszoriginalurl: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMD5SSOHash(pszchallengeinfo: super::super::Foundation::PWSTR, pwszrealm: super::super::Foundation::PWSTR, pwsztarget: super::super::Foundation::PWSTR, pbhexhash: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheContainerA(name: super::super::Foundation::PSTR, lpcacheprefix: super::super::Foundation::PSTR, lpszcachepath: super::super::Foundation::PSTR, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheContainerW(name: super::super::Foundation::PWSTR, lpcacheprefix: super::super::Foundation::PWSTR, lpszcachepath: super::super::Foundation::PWSTR, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheEntryA(lpszurlname: super::super::Foundation::PSTR, dwexpectedfilesize: u32, lpszfileextension: super::super::Foundation::PSTR, lpszfilename: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheEntryExW(lpszurlname: super::super::Foundation::PWSTR, dwexpectedfilesize: u32, lpszfileextension: super::super::Foundation::PWSTR, lpszfilename: super::super::Foundation::PWSTR, dwreserved: u32, fpreserveincomingfilename: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheEntryW(lpszurlname: super::super::Foundation::PWSTR, dwexpectedfilesize: u32, lpszfileextension: super::super::Foundation::PWSTR, lpszfilename: super::super::Foundation::PWSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn CreateUrlCacheGroup(dwflags: u32, lpreserved: *mut ::core::ffi::c_void) -> i64;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteIE3Cache(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmd: super::super::Foundation::PSTR, ncmdshow: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheContainerA(name: super::super::Foundation::PSTR, dwoptions: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheContainerW(name: super::super::Foundation::PWSTR, dwoptions: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheEntry(lpszurlname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheEntryA(lpszurlname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheEntryW(lpszurlname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheGroup(groupid: i64, dwflags: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteWpadCacheForNetworks(param0: WPAD_CACHE_DELETE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetectAutoProxyUrl(pszautoproxyurl: super::super::Foundation::PSTR, cchautoproxyurl: u32, dwdetectflags: PROXY_AUTO_DETECT_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoConnectoidsExist() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExportCookieFileA(szfilename: super::super::Foundation::PSTR, fappend: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExportCookieFileW(szfilename: super::super::Foundation::PWSTR, fappend: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCloseUrlCache(henumhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheContainerA(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheContainerW(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryA(lpszurlsearchpattern: super::super::Foundation::PSTR, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryExA(lpszurlsearchpattern: super::super::Foundation::PSTR, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryExW(lpszurlsearchpattern: super::super::Foundation::PWSTR, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryW(lpszurlsearchpattern: super::super::Foundation::PWSTR, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheGroup(dwflags: u32, dwfilter: u32, lpsearchcondition: *mut ::core::ffi::c_void, dwsearchcondition: u32, lpgroupid: *mut i64, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheContainerA(henumhandle: super::super::Foundation::HANDLE, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheContainerW(henumhandle: super::super::Foundation::HANDLE, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryA(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryExA(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryExW(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryW(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheGroup(hfind: super::super::Foundation::HANDLE, lpgroupid: *mut i64, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindP3PPolicySymbol(pszsymbol: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUrlCacheSpaceA(lpszcachepath: super::super::Foundation::PSTR, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUrlCacheSpaceW(lpszcachepath: super::super::Foundation::PWSTR, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCommandA(hconnect: *const ::core::ffi::c_void, fexpectresponse: super::super::Foundation::BOOL, dwflags: FTP_FLAGS, lpszcommand: super::super::Foundation::PSTR, dwcontext: usize, phftpcommand: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCommandW(hconnect: *const ::core::ffi::c_void, fexpectresponse: super::super::Foundation::BOOL, dwflags: FTP_FLAGS, lpszcommand: super::super::Foundation::PWSTR, dwcontext: usize, phftpcommand: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCreateDirectoryA(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCreateDirectoryW(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpDeleteFileA(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpDeleteFileW(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn FtpFindFirstFileA(hconnect: *const ::core::ffi::c_void, lpszsearchfile: super::super::Foundation::PSTR, lpfindfiledata: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAA, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn FtpFindFirstFileW(hconnect: *const ::core::ffi::c_void, lpszsearchfile: super::super::Foundation::PWSTR, lpfindfiledata: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAW, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetCurrentDirectoryA(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: super::super::Foundation::PSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetCurrentDirectoryW(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: super::super::Foundation::PWSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetFileA(hconnect: *const ::core::ffi::c_void, lpszremotefile: super::super::Foundation::PSTR, lpsznewfile: super::super::Foundation::PSTR, ffailifexists: super::super::Foundation::BOOL, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetFileEx(hftpsession: *const ::core::ffi::c_void, lpszremotefile: super::super::Foundation::PSTR, lpsznewfile: super::super::Foundation::PWSTR, ffailifexists: super::super::Foundation::BOOL, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn FtpGetFileSize(hfile: *const ::core::ffi::c_void, lpdwfilesizehigh: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetFileW(hconnect: *const ::core::ffi::c_void, lpszremotefile: super::super::Foundation::PWSTR, lpsznewfile: super::super::Foundation::PWSTR, ffailifexists: super::super::Foundation::BOOL, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpOpenFileA(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PSTR, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpOpenFileW(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PWSTR, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpPutFileA(hconnect: *const ::core::ffi::c_void, lpszlocalfile: super::super::Foundation::PSTR, lpsznewremotefile: super::super::Foundation::PSTR, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpPutFileEx(hftpsession: *const ::core::ffi::c_void, lpszlocalfile: super::super::Foundation::PWSTR, lpsznewremotefile: super::super::Foundation::PSTR, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpPutFileW(hconnect: *const ::core::ffi::c_void, lpszlocalfile: super::super::Foundation::PWSTR, lpsznewremotefile: super::super::Foundation::PWSTR, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRemoveDirectoryA(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRemoveDirectoryW(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRenameFileA(hconnect: *const ::core::ffi::c_void, lpszexisting: super::super::Foundation::PSTR, lpsznew: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRenameFileW(hconnect: *const ::core::ffi::c_void, lpszexisting: super::super::Foundation::PWSTR, lpsznew: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpSetCurrentDirectoryA(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpSetCurrentDirectoryW(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskInfoA(pszpath: super::super::Foundation::PSTR, pdwclustersize: *mut u32, pdlavail: *mut u64, pdltotal: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheConfigInfoA(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOA, lpcbcacheconfiginfo: *mut u32, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheConfigInfoW(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOW, lpcbcacheconfiginfo: *mut u32, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryBinaryBlob(pwszurlname: super::super::Foundation::PWSTR, dwtype: *mut u32, pftexpiretime: *mut super::super::Foundation::FILETIME, pftaccesstime: *mut super::super::Foundation::FILETIME, pftmodifiedtime: *mut super::super::Foundation::FILETIME, ppbblob: *mut *mut u8, pcbblob: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoExA(lpszurl: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpszredirecturl: super::super::Foundation::PSTR, lpcbredirecturl: *mut u32, lpreserved: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoExW(lpszurl: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpszredirecturl: super::super::Foundation::PWSTR, lpcbredirecturl: *mut u32, lpreserved: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherCreateLocatorA(lpszhost: super::super::Foundation::PSTR, nserverport: u16, lpszdisplaystring: super::super::Foundation::PSTR, lpszselectorstring: super::super::Foundation::PSTR, dwgophertype: u32, lpszlocator: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherCreateLocatorW(lpszhost: super::super::Foundation::PWSTR, nserverport: u16, lpszdisplaystring: super::super::Foundation::PWSTR, lpszselectorstring: super::super::Foundation::PWSTR, dwgophertype: u32, lpszlocator: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherFindFirstFileA(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PSTR, lpszsearchstring: super::super::Foundation::PSTR, lpfinddata: *mut GOPHER_FIND_DATAA, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherFindFirstFileW(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PWSTR, lpszsearchstring: super::super::Foundation::PWSTR, lpfinddata: *mut GOPHER_FIND_DATAW, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetAttributeA(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PSTR, lpszattributename: super::super::Foundation::PSTR, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: ::windows::runtime::RawPtr, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetAttributeW(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PWSTR, lpszattributename: super::super::Foundation::PWSTR, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: ::windows::runtime::RawPtr, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetLocatorTypeA(lpszlocator: super::super::Foundation::PSTR, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetLocatorTypeW(lpszlocator: super::super::Foundation::PWSTR, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherOpenFileA(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PSTR, lpszview: super::super::Foundation::PSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherOpenFileW(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PWSTR, lpszview: super::super::Foundation::PWSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddRequestHeadersA(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PSTR, dwheaderslength: u32, dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddRequestHeadersW(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCheckDavComplianceA(lpszurl: super::super::Foundation::PSTR, lpszcompliancetoken: super::super::Foundation::PSTR, lpffound: *mut i32, hwnd: super::super::Foundation::HWND, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCheckDavComplianceW(lpszurl: super::super::Foundation::PWSTR, lpszcompliancetoken: super::super::Foundation::PWSTR, lpffound: *mut i32, hwnd: super::super::Foundation::HWND, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpCloseDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpDuplicateDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void, phduplicateddependencyhandle: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpEndRequestA(hrequest: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpEndRequestW(hrequest: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpGetServerCredentials(pwszurl: super::super::Foundation::PWSTR, ppwszusername: *mut super::super::Foundation::PWSTR, ppwszpassword: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpIndicatePageLoadComplete(hdependencyhandle: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpIsHostHstsEnabled(pcwszurl: super::super::Foundation::PWSTR, pfishsts: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpOpenDependencyHandle(hrequesthandle: *const ::core::ffi::c_void, fbackground: super::super::Foundation::BOOL, phdependencyhandle: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpOpenRequestA(hconnect: *const ::core::ffi::c_void, lpszverb: super::super::Foundation::PSTR, lpszobjectname: super::super::Foundation::PSTR, lpszversion: super::super::Foundation::PSTR, lpszreferrer: super::super::Foundation::PSTR, lplpszaccepttypes: *const super::super::Foundation::PSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpOpenRequestW(hconnect: *const ::core::ffi::c_void, lpszverb: super::super::Foundation::PWSTR, lpszobjectname: super::super::Foundation::PWSTR, lpszversion: super::super::Foundation::PWSTR, lpszreferrer: super::super::Foundation::PWSTR, lplpszaccepttypes: *const super::super::Foundation::PWSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpPushClose(hwait: HTTP_PUSH_WAIT_HANDLE);
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpPushEnable(hrequest: *const ::core::ffi::c_void, ptransportsetting: *const HTTP_PUSH_TRANSPORT_SETTING, phwait: *mut HTTP_PUSH_WAIT_HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpPushWait(hwait: HTTP_PUSH_WAIT_HANDLE, etype: HTTP_PUSH_WAIT_TYPE, pnotificationstatus: *mut HTTP_PUSH_NOTIFICATION_STATUS) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryInfoA(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryInfoW(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestA(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PSTR, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestExA(hrequest: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestExW(hrequest: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestW(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketClose(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpWebSocketCompleteUpgrade(hrequest: *const ::core::ffi::c_void, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketQueryCloseStatus(hwebsocket: *const ::core::ffi::c_void, pusstatus: *mut u16, pvreason: *mut ::core::ffi::c_void, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketReceive(hwebsocket: *const ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pbuffertype: *mut HTTP_WEB_SOCKET_BUFFER_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketSend(hwebsocket: *const ::core::ffi::c_void, buffertype: HTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketShutdown(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportCookieFileA(szfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportCookieFileW(szfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IncrementUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternalInternetGetCookie(lpszurl: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, lpdwdatasize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAlgIdToStringA(ai: u32, lpstr: super::super::Foundation::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAlgIdToStringW(ai: u32, lpstr: super::super::Foundation::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetAttemptConnect(dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAutodial(dwflags: INTERNET_AUTODIAL, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAutodialHangup(dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCanonicalizeUrlA(lpszurl: super::super::Foundation::PSTR, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCanonicalizeUrlW(lpszurl: super::super::Foundation::PWSTR, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCheckConnectionA(lpszurl: super::super::Foundation::PSTR, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCheckConnectionW(lpszurl: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetClearAllPerSiteCookieDecisions() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCloseHandle(hinternet: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCombineUrlA(lpszbaseurl: super::super::Foundation::PSTR, lpszrelativeurl: super::super::Foundation::PSTR, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCombineUrlW(lpszbaseurl: super::super::Foundation::PWSTR, lpszrelativeurl: super::super::Foundation::PWSTR, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConfirmZoneCrossing(hwnd: super::super::Foundation::HWND, szurlprev: super::super::Foundation::PSTR, szurlnew: super::super::Foundation::PSTR, bpost: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConfirmZoneCrossingA(hwnd: super::super::Foundation::HWND, szurlprev: super::super::Foundation::PSTR, szurlnew: super::super::Foundation::PSTR, bpost: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConfirmZoneCrossingW(hwnd: super::super::Foundation::HWND, szurlprev: super::super::Foundation::PWSTR, szurlnew: super::super::Foundation::PWSTR, bpost: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConnectA(hinternet: *const ::core::ffi::c_void, lpszservername: super::super::Foundation::PSTR, nserverport: u16, lpszusername: super::super::Foundation::PSTR, lpszpassword: super::super::Foundation::PSTR, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConnectW(hinternet: *const ::core::ffi::c_void, lpszservername: super::super::Foundation::PWSTR, nserverport: u16, lpszusername: super::super::Foundation::PWSTR, lpszpassword: super::super::Foundation::PWSTR, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConvertUrlFromWireToWideChar(pcszurl: super::super::Foundation::PSTR, cchurl: u32, pcwszbaseurl: super::super::Foundation::PWSTR, dwcodepagehost: u32, dwcodepagepath: u32, fencodepathextra: super::super::Foundation::BOOL, dwcodepageextra: u32, ppwszconvertedurl: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Networking_WinHttp`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
    pub fn InternetCrackUrlA(lpszurl: super::super::Foundation::PSTR, dwurllength: u32, dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Networking_WinHttp`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
    pub fn InternetCrackUrlW(lpszurl: super::super::Foundation::PWSTR, dwurllength: u32, dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCreateUrlA(lpurlcomponents: *const URL_COMPONENTSA, dwflags: u32, lpszurl: super::super::Foundation::PSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCreateUrlW(lpurlcomponents: *const URL_COMPONENTSW, dwflags: u32, lpszurl: super::super::Foundation::PWSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetDial(hwndparent: super::super::Foundation::HWND, lpszconnectoid: super::super::Foundation::PSTR, dwflags: u32, lpdwconnection: *mut u32, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetDialA(hwndparent: super::super::Foundation::HWND, lpszconnectoid: super::super::Foundation::PSTR, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetDialW(hwndparent: super::super::Foundation::HWND, lpszconnectoid: super::super::Foundation::PWSTR, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetEnumPerSiteCookieDecisionA(pszsitename: super::super::Foundation::PSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetEnumPerSiteCookieDecisionW(pszsitename: super::super::Foundation::PWSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetErrorDlg(hwnd: super::super::Foundation::HWND, hrequest: *mut ::core::ffi::c_void, dwerror: u32, dwflags: u32, lppvdata: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFindNextFileA(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFindNextFileW(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFortezzaCommand(dwcommand: u32, hwnd: super::super::Foundation::HWND, dwreserved: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFreeCookies(pcookies: *mut INTERNET_COOKIE2, dwcookiecount: u32);
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFreeProxyInfoList(pproxyinfolist: *mut WININET_PROXY_INFO_LIST);
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedState(lpdwflags: *mut INTERNET_CONNECTION, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedStateEx(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PSTR, dwnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedStateExA(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PSTR, cchnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedStateExW(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PWSTR, cchnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieEx2(pcwszurl: super::super::Foundation::PWSTR, pcwszcookiename: super::super::Foundation::PWSTR, dwflags: u32, ppcookies: *mut *mut INTERNET_COOKIE2, pdwcookiecount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieExA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieExW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetLastResponseInfoA(lpdwerror: *mut u32, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetLastResponseInfoW(lpdwerror: *mut u32, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetPerSiteCookieDecisionA(pchhostname: super::super::Foundation::PSTR, presult: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetPerSiteCookieDecisionW(pchhostname: super::super::Foundation::PWSTR, presult: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetProxyForUrl(hinternet: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pproxyinfolist: *mut WININET_PROXY_INFO_LIST) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn InternetGetSecurityInfoByURL(lpszurl: super::super::Foundation::PSTR, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn InternetGetSecurityInfoByURLA(lpszurl: super::super::Foundation::PSTR, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn InternetGetSecurityInfoByURLW(lpszurl: super::super::Foundation::PWSTR, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGoOnline(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGoOnlineA(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGoOnlineW(lpszurl: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetHangUp(dwconnection: usize, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetInitializeAutoProxyDll(dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetLockRequestFile(hinternet: *const ::core::ffi::c_void, lphlockrequestinfo: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenA(lpszagent: super::super::Foundation::PSTR, dwaccesstype: u32, lpszproxy: super::super::Foundation::PSTR, lpszproxybypass: super::super::Foundation::PSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenUrlA(hinternet: *const ::core::ffi::c_void, lpszurl: super::super::Foundation::PSTR, lpszheaders: super::super::Foundation::PSTR, dwheaderslength: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenUrlW(hinternet: *const ::core::ffi::c_void, lpszurl: super::super::Foundation::PWSTR, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenW(lpszagent: super::super::Foundation::PWSTR, dwaccesstype: u32, lpszproxy: super::super::Foundation::PWSTR, lpszproxybypass: super::super::Foundation::PWSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryDataAvailable(hfile: *const ::core::ffi::c_void, lpdwnumberofbytesavailable: *mut u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryFortezzaStatus(pdwstatus: *mut u32, dwreserved: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryOptionA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryOptionW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetReadFile(hfile: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetReadFileExA(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetReadFileExW(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSecurityProtocolToStringA(dwprotocol: u32, lpstr: super::super::Foundation::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSecurityProtocolToStringW(dwprotocol: u32, lpstr: super::super::Foundation::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieEx2(pcwszurl: super::super::Foundation::PWSTR, pcookie: *const INTERNET_COOKIE2, pcwszp3ppolicy: super::super::Foundation::PWSTR, dwflags: u32, pdwcookiestate: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieExA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, dwflags: u32, dwreserved: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieExW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetDialState(lpszconnectoid: super::super::Foundation::PSTR, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetDialStateA(lpszconnectoid: super::super::Foundation::PSTR, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetDialStateW(lpszconnectoid: super::super::Foundation::PWSTR, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetFilePointer(hfile: *const ::core::ffi::c_void, ldistancetomove: i32, lpdistancetomovehigh: *mut i32, dwmovemethod: u32, dwcontext: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionExA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionExW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetPerSiteCookieDecisionA(pchhostname: super::super::Foundation::PSTR, dwdecision: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetPerSiteCookieDecisionW(pchhostname: super::super::Foundation::PWSTR, dwdecision: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetStatusCallback(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::windows::runtime::RawPtr) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK>;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetStatusCallbackA(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::windows::runtime::RawPtr) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK>;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetStatusCallbackW(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::windows::runtime::RawPtr) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK>;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetShowSecurityInfoByURL(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetShowSecurityInfoByURLA(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetShowSecurityInfoByURLW(lpszurl: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PSTR, cbtime: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeFromSystemTimeA(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PSTR, cbtime: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeFromSystemTimeW(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PWSTR, cbtime: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeToSystemTime(lpsztime: super::super::Foundation::PSTR, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeToSystemTimeA(lpsztime: super::super::Foundation::PSTR, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeToSystemTimeW(lpsztime: super::super::Foundation::PWSTR, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetUnlockRequestFile(hlockrequestinfo: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetWriteFile(hfile: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetWriteFileExA(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetWriteFileExW(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDomainLegalCookieDomainA(pchdomain: super::super::Foundation::PSTR, pchfulldomain: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDomainLegalCookieDomainW(pchdomain: super::super::Foundation::PWSTR, pchfulldomain: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsHostInProxyBypassList(tscheme: INTERNET_SCHEME, lpszhost: super::super::Foundation::PSTR, cchhost: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProfilesEnabled() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUrlCacheEntryExpiredA(lpszurlname: super::super::Foundation::PSTR, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUrlCacheEntryExpiredW(lpszurlname: super::super::Foundation::PWSTR, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUrlCacheContent() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseX509EncodedCertificateForListBoxEntry(lpcert: *const u8, cbcert: u32, lpszlistboxentry: super::super::Foundation::PSTR, lpdwlistboxentry: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerformOperationOverUrlCacheA(pszurlsearchpattern: super::super::Foundation::PSTR, dwflags: u32, dwfilter: u32, groupid: i64, preserved1: *mut ::core::ffi::c_void, pdwreserved2: *mut u32, preserved3: *mut ::core::ffi::c_void, op: ::windows::runtime::RawPtr, poperatordata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivacyGetZonePreferenceW(dwzone: u32, dwtype: u32, pdwtemplate: *mut u32, pszbuffer: super::super::Foundation::PWSTR, pdwbufferlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivacySetZonePreferenceW(dwzone: u32, dwtype: u32, dwtemplate: u32, pszpreference: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadGuidsForConnectedNetworks(pcnetworks: *mut u32, pppwsznetworkguids: *mut *mut super::super::Foundation::PWSTR, pppbstrnetworknames: *mut *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppwszgwmacs: *mut *mut super::super::Foundation::PWSTR, pcgatewaymacs: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadUrlCacheEntryStream(hurlcachestream: super::super::Foundation::HANDLE, dwlocation: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32, reserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadUrlCacheEntryStreamEx(hurlcachestream: super::super::Foundation::HANDLE, qwlocation: u64, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterUrlCacheNotification(hwnd: super::super::Foundation::HWND, umsg: u32, gid: i64, dwopsfilter: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResumeSuspendedDownload(hrequest: *const ::core::ffi::c_void, dwresultcode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryFileA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryFileW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryStreamA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, frandomread: super::super::Foundation::BOOL, dwreserved: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryStreamW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, frandomread: super::super::Foundation::BOOL, dwreserved: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunOnceUrlCache(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmd: super::super::Foundation::PSTR, ncmdshow: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheConfigInfoA(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheConfigInfoW(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryGroup(lpszurlname: super::super::Foundation::PSTR, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryGroupA(lpszurlname: super::super::Foundation::PSTR, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryGroupW(lpszurlname: super::super::Foundation::PWSTR, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryInfoA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryInfoW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOA, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOW, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheHeaderData(nidx: u32, dwdata: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowClientAuthCerts(hwndparent: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
    pub fn ShowSecurityInfo(hwndparent: super::super::Foundation::HWND, psecurityinfo: *const INTERNET_SECURITY_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowX509EncodedCertificate(hwndparent: super::super::Foundation::HWND, lpcert: *const u8, cbcert: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryFile(lpszurlname: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryFileA(lpszurlname: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryFileW(lpszurlname: super::super::Foundation::PWSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryStream(hurlcachestream: super::super::Foundation::HANDLE, reserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateUrlCacheContentPath(sznewpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheCheckEntriesExist(rgpwszurls: *const super::super::Foundation::PWSTR, centries: u32, rgfexist: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheCloseEntryHandle(hentryfile: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheContainerSetEntryMaximumAge(pwszprefix: super::super::Foundation::PWSTR, dwentrymaxage: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheCreateContainer(pwszname: super::super::Foundation::PWSTR, pwszprefix: super::super::Foundation::PWSTR, pwszdirectory: super::super::Foundation::PWSTR, ulllimit: u64, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheFindFirstEntry(pwszprefix: super::super::Foundation::PWSTR, dwflags: u32, dwfilter: u32, groupid: i64, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phfind: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheFindNextEntry(hfind: super::super::Foundation::HANDLE, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheFreeEntryInfo(pcacheentryinfo: *mut URLCACHE_ENTRY_INFO);
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheFreeGlobalSpace(ulltargetsize: u64, dwfilter: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheGetContentPaths(pppwszdirectories: *mut *mut super::super::Foundation::PWSTR, pcdirectories: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheGetEntryInfo(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheGetGlobalCacheSize(dwfilter: u32, pullsize: *mut u64, pulllimit: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheGetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, pulllimit: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheReadEntryStream(hurlcachestream: *const ::core::ffi::c_void, ulllocation: u64, pbuffer: *mut ::core::ffi::c_void, dwbufferlen: u32, pdwbufferlen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheReloadSettings() -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheRetrieveEntryFile(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentryfile: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheRetrieveEntryStream(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, frandomread: super::super::Foundation::BOOL, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentrystream: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheServer() -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheSetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, ulllimit: u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheUpdateEntryExtraData(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pbextradata: *const u8, cbextradata: u32) -> u32;
}
