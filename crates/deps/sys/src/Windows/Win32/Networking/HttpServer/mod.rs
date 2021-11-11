#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpAddFragmentToCache();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddUrl();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpAddUrlToUrlGroup();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpCancelHttpRequest();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCloseRequestQueue();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCloseServerSession();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCloseUrlGroup();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpCreateHttpHandle();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HttpCreateRequestQueue();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCreateServerSession();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpCreateUrlGroup();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpDeclarePush();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpDelegateRequestEx();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpDeleteServiceConfiguration();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpFindUrlGroupId();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpFlushResponseCache();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpGetExtension();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpInitialize();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpIsFeatureSupported();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpPrepareUrl();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpQueryRequestQueueProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpQueryServerSessionProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpQueryServiceConfiguration();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpQueryUrlGroupProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReadFragmentFromCache();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReceiveClientCertificate();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
    pub fn HttpReceiveHttpRequest();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpReceiveRequestEntityBody();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpRemoveUrl();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpRemoveUrlFromUrlGroup();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSendHttpResponse();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSendResponseEntityBody();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSetRequestProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpSetRequestQueueProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpSetServerSessionProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpSetServiceConfiguration();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpSetUrlGroupProperty();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpShutdownRequestQueue();
    #[doc = "*Required features: `Win32_Networking_HttpServer`*"]
    pub fn HttpTerminate();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpUpdateServiceConfiguration();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDemandStart();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDisconnect();
    #[doc = "*Required features: `Win32_Networking_HttpServer`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn HttpWaitForDisconnectEx();
}
