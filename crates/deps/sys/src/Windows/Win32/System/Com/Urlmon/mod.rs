#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetClassObjectFromURL();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCombineIUri();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrl();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrlEx();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCompareUrl();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCreateSecurityManager();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCreateZoneManager();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetProtocolFlags();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetSecurityUrl();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetGetSecurityUrlEx();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetGetSession();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetIsFeatureEnabled();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetIsFeatureEnabledForIUri();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureEnabledForUrl();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureZoneElevationEnabled();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseIUri();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseUrl();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetQueryInfo();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetSetFeatureEnabled();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CompareSecurityIds();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CompatFlagsFromClsid();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyBindInfo();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyStgMedium();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateAsyncBindCtx();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateAsyncBindCtxEx();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateFormatEnumerator();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMoniker();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMonikerEx();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateURLMonikerEx2();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaultInIEFeature();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaType();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaTypeClass();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMimeFromData();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFileOrMime();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassURL();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComponentIDFromCLSSPEC();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSoftwareUpdateInfo();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkGoBack();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkGoForward();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkNavigateMoniker();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkNavigateString();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToMoniker();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToString();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IEGetUserPrivateNamespaceName();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn IEInstallScope();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn IsAsyncMoniker();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledA();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledW();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidURL();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayNameEx();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObtainUserAgentString();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RegisterBindStatusCallback();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RegisterFormatEnumerator();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypeClass();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypes();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseBindInfo();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RevokeBindStatusCallback();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RevokeFormatEnumerator();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAccessForIEAppContainer();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSoftwareUpdateAdvertisementState();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileA();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileW();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileA();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileW();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamA();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamW();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamA();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamW();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamA();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamW();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn UrlMkGetSessionOption();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn UrlMkSetSessionOption();
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteHitLogging();
}
