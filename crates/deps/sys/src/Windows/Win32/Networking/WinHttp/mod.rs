#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpAddRequestHeaders();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpAddRequestHeadersEx();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCheckPlatform();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCloseHandle();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpConnect();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCrackUrl();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpCreateProxyResolver();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpCreateUrl();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpDetectAutoProxyConfigUrl();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeProxyResult();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeProxyResultEx();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeProxySettings();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpFreeQueryConnectionGroupResult();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetDefaultProxyConfiguration();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetIEProxyConfigForCurrentUser();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyForUrl();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyForUrlEx();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyForUrlEx2();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyResult();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpGetProxyResultEx();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpGetProxySettingsVersion();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpOpen();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpOpenRequest();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryAuthSchemes();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryConnectionGroup();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryDataAvailable();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryHeaders();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryHeadersEx();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpQueryOption();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpReadData();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpReadDataEx();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpReadProxySettings();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpReceiveResponse();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpResetAutoProxy();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSendRequest();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetCredentials();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetDefaultProxyConfiguration();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetOption();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetProxySettingsPerUser();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpSetStatusCallback();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpSetTimeouts();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpTimeFromSystemTime();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpTimeToSystemTime();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketClose();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketCompleteUpgrade();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketQueryCloseStatus();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketReceive();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketSend();
    #[doc = "*Required features: `Win32_Networking_WinHttp`*"]
    pub fn WinHttpWebSocketShutdown();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpWriteData();
    #[doc = "*Required features: `Win32_Networking_WinHttp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHttpWriteProxySettings();
}
