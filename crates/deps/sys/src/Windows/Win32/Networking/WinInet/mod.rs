#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheCheckManifest();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn AppCacheCloseHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheCreateAndCommitFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheDeleteGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheDeleteIEGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn AppCacheDuplicateHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn AppCacheFinalize();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeDownloadList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeGroupList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeIESpace();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheFreeSpace();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetDownloadList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetFallbackUrl();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetGroupList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetIEGroupList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetInfo();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheGetManifestUrl();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppCacheLookup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitUrlCacheEntryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitUrlCacheEntryBinaryBlob();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitUrlCacheEntryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMD5SSOHash();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheContainerA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheContainerW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheEntryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheEntryExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUrlCacheEntryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn CreateUrlCacheGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteIE3Cache();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheContainerA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheContainerW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheEntry();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheEntryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheEntryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUrlCacheGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteWpadCacheForNetworks();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetectAutoProxyUrl();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoConnectoidsExist();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExportCookieFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExportCookieFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCloseUrlCache();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheContainerA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheContainerW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheEntryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstUrlCacheGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheContainerA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheContainerW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheEntryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextUrlCacheGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindP3PPolicySymbol();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUrlCacheSpaceA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUrlCacheSpaceW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCommandA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCommandW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCreateDirectoryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpCreateDirectoryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpDeleteFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpDeleteFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn FtpFindFirstFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn FtpFindFirstFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetCurrentDirectoryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetCurrentDirectoryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetFileEx();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn FtpGetFileSize();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpGetFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpOpenFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpOpenFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpPutFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpPutFileEx();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpPutFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRemoveDirectoryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRemoveDirectoryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRenameFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpRenameFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpSetCurrentDirectoryA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtpSetCurrentDirectoryW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheConfigInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheConfigInfoW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryBinaryBlob();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheEntryInfoW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheGroupAttributeA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheGroupAttributeW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUrlCacheHeaderData();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherCreateLocatorA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherCreateLocatorW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherFindFirstFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherFindFirstFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetAttributeA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetAttributeW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetLocatorTypeA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherGetLocatorTypeW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherOpenFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GopherOpenFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddRequestHeadersA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddRequestHeadersW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCheckDavComplianceA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCheckDavComplianceW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpCloseDependencyHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpDuplicateDependencyHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpEndRequestA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpEndRequestW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpGetServerCredentials();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpIndicatePageLoadComplete();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpIsHostHstsEnabled();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpOpenDependencyHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpOpenRequestA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpOpenRequestW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpPushClose();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpPushEnable();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpPushWait();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryInfoW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSendRequestW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketClose();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn HttpWebSocketCompleteUpgrade();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketQueryCloseStatus();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketReceive();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketSend();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpWebSocketShutdown();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportCookieFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportCookieFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IncrementUrlCacheHeaderData();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternalInternetGetCookie();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAlgIdToStringA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAlgIdToStringW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetAttemptConnect();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAutodial();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetAutodialHangup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCanonicalizeUrlA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCanonicalizeUrlW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCheckConnectionA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCheckConnectionW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetClearAllPerSiteCookieDecisions();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCloseHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCombineUrlA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCombineUrlW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConfirmZoneCrossing();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConfirmZoneCrossingA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConfirmZoneCrossingW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConnectA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConnectW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetConvertUrlFromWireToWideChar();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Networking_WinHttp`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
    pub fn InternetCrackUrlA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Networking_WinHttp`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
    pub fn InternetCrackUrlW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCreateUrlA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetCreateUrlW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetDial();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetDialA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetDialW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetEnumPerSiteCookieDecisionA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetEnumPerSiteCookieDecisionW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetErrorDlg();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFindNextFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFindNextFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFortezzaCommand();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFreeCookies();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetFreeProxyInfoList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedState();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedStateEx();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedStateExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetConnectedStateExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieEx2();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetCookieW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetLastResponseInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetLastResponseInfoW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetPerSiteCookieDecisionA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetPerSiteCookieDecisionW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGetProxyForUrl();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn InternetGetSecurityInfoByURL();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn InternetGetSecurityInfoByURLA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn InternetGetSecurityInfoByURLW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGoOnline();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGoOnlineA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetGoOnlineW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetHangUp();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetInitializeAutoProxyDll();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetLockRequestFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenUrlA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenUrlW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetOpenW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryDataAvailable();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryFortezzaStatus();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryOptionA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetQueryOptionW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetReadFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetReadFileExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetReadFileExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSecurityProtocolToStringA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSecurityProtocolToStringW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieEx2();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetCookieW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetDialState();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetDialStateA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetDialStateW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetFilePointer();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetOptionW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetPerSiteCookieDecisionA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetSetPerSiteCookieDecisionW();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetStatusCallback();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetStatusCallbackA();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn InternetSetStatusCallbackW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetShowSecurityInfoByURL();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetShowSecurityInfoByURLA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetShowSecurityInfoByURLW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeFromSystemTime();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeFromSystemTimeA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeFromSystemTimeW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeToSystemTime();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeToSystemTimeA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetTimeToSystemTimeW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetUnlockRequestFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetWriteFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetWriteFileExA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternetWriteFileExW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDomainLegalCookieDomainA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDomainLegalCookieDomainW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsHostInProxyBypassList();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProfilesEnabled();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUrlCacheEntryExpiredA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUrlCacheEntryExpiredW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUrlCacheContent();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseX509EncodedCertificateForListBoxEntry();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerformOperationOverUrlCacheA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivacyGetZonePreferenceW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivacySetZonePreferenceW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadGuidsForConnectedNetworks();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadUrlCacheEntryStream();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadUrlCacheEntryStreamEx();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterUrlCacheNotification();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResumeSuspendedDownload();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryStreamA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RetrieveUrlCacheEntryStreamW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunOnceUrlCache();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheConfigInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheConfigInfoW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryGroup();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryGroupA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryGroupW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryInfoA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheEntryInfoW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheGroupAttributeA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheGroupAttributeW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUrlCacheHeaderData();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowClientAuthCerts();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
    pub fn ShowSecurityInfo();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowX509EncodedCertificate();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryFileA();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryFileW();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockUrlCacheEntryStream();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateUrlCacheContentPath();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheCheckEntriesExist();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheCloseEntryHandle();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheContainerSetEntryMaximumAge();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheCreateContainer();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheFindFirstEntry();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheFindNextEntry();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheFreeEntryInfo();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheFreeGlobalSpace();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheGetContentPaths();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheGetEntryInfo();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheGetGlobalCacheSize();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheGetGlobalLimit();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheReadEntryStream();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheReloadSettings();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheRetrieveEntryFile();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheRetrieveEntryStream();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheServer();
    #[doc = "*Required features: `Win32_Networking_WinInet`*"]
    pub fn UrlCacheSetGlobalLimit();
    #[doc = "*Required features: `Win32_Networking_WinInet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCacheUpdateEntryExtraData();
}
