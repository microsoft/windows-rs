#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetClassObjectFromURL(rclassid: *const ::windows_sys::core::GUID, szcode: super::super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, sztype: super::super::super::Foundation::PWSTR, pbindctx: super::IBindCtx, dwclscontext: super::CLSCTX, pvreserved: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetCombineIUri(pbaseuri: super::IUri, prelativeuri: super::IUri, dwcombineflags: u32, ppcombineduri: *mut super::IUri, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrl(pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrlEx(pbaseuri: super::IUri, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, ppcombineduri: *mut super::IUri, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCompareUrl(pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetCreateSecurityManager(psp: super::IServiceProvider, ppsm: *mut IInternetSecurityManager, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetCreateZoneManager(psp: super::IServiceProvider, ppzm: *mut IInternetZoneManager, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetProtocolFlags(pwzurl: super::super::super::Foundation::PWSTR, pdwflags: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetSecurityUrl(pwszurl: super::super::super::Foundation::PWSTR, ppwszsecurl: *mut super::super::super::Foundation::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetGetSecurityUrlEx(puri: super::IUri, ppsecuri: *mut super::IUri, psuaction: PSUACTION, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut IInternetSession, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> ::windows_sys::core::HRESULT;
    pub fn CoInternetIsFeatureEnabledForIUri(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: super::IUri, psecmgr: IInternetSecurityManagerEx2) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureEnabledForUrl(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: super::super::super::Foundation::PWSTR, psecmgr: IInternetSecurityManager) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureZoneElevationEnabled(szfromurl: super::super::super::Foundation::PWSTR, sztourl: super::super::super::Foundation::PWSTR, psecmgr: IInternetSecurityManager, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseIUri(piuri: super::IUri, parseaction: PARSEACTION, dwflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseUrl(pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetQueryInfo(pwzurl: super::super::super::Foundation::PWSTR, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuffer: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetSetFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn CompareSecurityIds(pbsecurityid1: *const u8, dwlen1: u32, pbsecurityid2: *const u8, dwlen2: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn CompatFlagsFromClsid(pclsid: *const ::windows_sys::core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyBindInfo(pcbisrc: *const super::BINDINFO, pbidest: *mut super::BINDINFO) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyStgMedium(pcstgmedsrc: *const super::STGMEDIUM, pstgmeddest: *mut super::STGMEDIUM) -> ::windows_sys::core::HRESULT;
    pub fn CreateAsyncBindCtx(reserved: u32, pbscb: super::IBindStatusCallback, pefetc: super::IEnumFORMATETC, ppbc: *mut super::IBindCtx) -> ::windows_sys::core::HRESULT;
    pub fn CreateAsyncBindCtxEx(pbc: super::IBindCtx, dwoptions: u32, pbscb: super::IBindStatusCallback, penum: super::IEnumFORMATETC, ppbc: *mut super::IBindCtx, reserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn CreateFormatEnumerator(cfmtetc: u32, rgfmtetc: *const super::FORMATETC, ppenumfmtetc: *mut super::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMoniker(pmkctx: super::IMoniker, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMonikerEx(pmkctx: super::IMoniker, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut super::IMoniker, dwflags: u32) -> ::windows_sys::core::HRESULT;
    pub fn CreateURLMonikerEx2(pmkctx: super::IMoniker, puri: super::IUri, ppmk: *mut super::IMoniker, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaultInIEFeature(hwnd: super::super::super::Foundation::HWND, pclassspec: *const super::uCLSSPEC, pquery: *mut super::QUERYCONTEXT, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaType(rgsztypes: super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaTypeClass(pbc: super::IBindCtx, sztype: super::super::super::Foundation::PSTR, pclsid: *mut ::windows_sys::core::GUID, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMimeFromData(pbc: super::IBindCtx, pwzurl: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, pwzmimeproposed: super::super::super::Foundation::PWSTR, dwmimeflags: u32, ppwzmimeout: *mut super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFileOrMime(pbc: super::IBindCtx, szfilename: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, szmime: super::super::super::Foundation::PWSTR, dwreserved: u32, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassURL(szurl: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC, ppszcomponentid: *mut super::super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSoftwareUpdateInfo(szdistunit: super::super::super::Foundation::PWSTR, psdi: *mut SOFTDISTINFO) -> ::windows_sys::core::HRESULT;
    pub fn HlinkGoBack(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn HlinkGoForward(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn HlinkNavigateMoniker(punk: ::windows_sys::core::IUnknown, pmktarget: super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkNavigateString(punk: ::windows_sys::core::IUnknown, sztarget: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToMoniker(pmktarget: super::IMoniker, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows_sys::core::IUnknown, pbc: super::IBindCtx, param5: super::IBindStatusCallback, grfhlnf: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToString(sztarget: super::super::super::Foundation::PWSTR, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows_sys::core::IUnknown, pbc: super::IBindCtx, param5: super::IBindStatusCallback, grfhlnf: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IEGetUserPrivateNamespaceName() -> super::super::super::Foundation::PWSTR;
    pub fn IEInstallScope(pdwscope: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn IsAsyncMoniker(pmk: super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledA(pszurl: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledW(pwszurl: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidURL(pbc: super::IBindCtx, szurl: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayNameEx(pbc: super::IBindCtx, szdisplayname: super::super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObtainUserAgentString(dwoption: u32, pszuaout: super::super::super::Foundation::PSTR, cbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn RegisterBindStatusCallback(pbc: super::IBindCtx, pbscb: super::IBindStatusCallback, ppbscbprev: *mut super::IBindStatusCallback, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn RegisterFormatEnumerator(pbc: super::IBindCtx, pefetc: super::IEnumFORMATETC, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypeClass(pbc: super::IBindCtx, ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgclsid: *const ::windows_sys::core::GUID, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseBindInfo(pbindinfo: *mut super::BINDINFO);
    pub fn RevokeBindStatusCallback(pbc: super::IBindCtx, pbscb: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    pub fn RevokeFormatEnumerator(pbc: super::IBindCtx, pefetc: super::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAccessForIEAppContainer(hobject: super::super::super::Foundation::HANDLE, ieobjecttype: IEObjectType, dwaccessmask: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSoftwareUpdateAdvertisementState(szdistunit: super::super::super::Foundation::PWSTR, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, cchfilename: u32, param4: u32, param5: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, cchfilename: u32, param4: u32, param5: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: *mut super::IStream, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: *mut super::IStream, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    pub fn UrlMkGetSessionOption(dwoption: u32, pbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    pub fn UrlMkSetSessionOption(dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL;
}
pub const AUTHENTICATEF_PROXY: i32 = 1i32;
pub const AUTHENTICATEF_BASIC: i32 = 2i32;
pub const AUTHENTICATEF_HTTP: i32 = 4i32;
pub const BINDF_ASYNCHRONOUS: i32 = 1i32;
pub const BINDF_ASYNCSTORAGE: i32 = 2i32;
pub const BINDF_NOPROGRESSIVERENDERING: i32 = 4i32;
pub const BINDF_OFFLINEOPERATION: i32 = 8i32;
pub const BINDF_GETNEWESTVERSION: i32 = 16i32;
pub const BINDF_NOWRITECACHE: i32 = 32i32;
pub const BINDF_NEEDFILE: i32 = 64i32;
pub const BINDF_PULLDATA: i32 = 128i32;
pub const BINDF_IGNORESECURITYPROBLEM: i32 = 256i32;
pub const BINDF_RESYNCHRONIZE: i32 = 512i32;
pub const BINDF_HYPERLINK: i32 = 1024i32;
pub const BINDF_NO_UI: i32 = 2048i32;
pub const BINDF_SILENTOPERATION: i32 = 4096i32;
pub const BINDF_PRAGMA_NO_CACHE: i32 = 8192i32;
pub const BINDF_GETCLASSOBJECT: i32 = 16384i32;
pub const BINDF_RESERVED_1: i32 = 32768i32;
pub const BINDF_FREE_THREADED: i32 = 65536i32;
pub const BINDF_DIRECT_READ: i32 = 131072i32;
pub const BINDF_FORMS_SUBMIT: i32 = 262144i32;
pub const BINDF_GETFROMCACHE_IF_NET_FAIL: i32 = 524288i32;
pub const BINDF_FROMURLMON: i32 = 1048576i32;
pub const BINDF_FWD_BACK: i32 = 2097152i32;
pub const BINDF_PREFERDEFAULTHANDLER: i32 = 4194304i32;
pub const BINDF_ENFORCERESTRICTED: i32 = 8388608i32;
pub const BINDF_RESERVED_2: i32 = -2147483648i32;
pub const BINDF_RESERVED_3: i32 = 16777216i32;
pub const BINDF_RESERVED_4: i32 = 33554432i32;
pub const BINDF_RESERVED_5: i32 = 67108864i32;
pub const BINDF_RESERVED_6: i32 = 134217728i32;
pub const BINDF_RESERVED_7: i32 = 1073741824i32;
pub const BINDF_RESERVED_8: i32 = 536870912i32;
pub const BINDF2_DISABLEBASICOVERHTTP: i32 = 1i32;
pub const BINDF2_DISABLEAUTOCOOKIEHANDLING: i32 = 2i32;
pub const BINDF2_READ_DATA_GREATER_THAN_4GB: i32 = 4i32;
pub const BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID: i32 = 8i32;
pub const BINDF2_SETDOWNLOADMODE: i32 = 32i32;
pub const BINDF2_DISABLE_HTTP_REDIRECT_CACHING: i32 = 64i32;
pub const BINDF2_KEEP_CALLBACK_MODULE_LOADED: i32 = 128i32;
pub const BINDF2_ALLOW_PROXY_CRED_PROMPT: i32 = 256i32;
pub const BINDF2_RESERVED_17: i32 = 512i32;
pub const BINDF2_RESERVED_16: i32 = 1024i32;
pub const BINDF2_RESERVED_15: i32 = 2048i32;
pub const BINDF2_RESERVED_14: i32 = 4096i32;
pub const BINDF2_RESERVED_13: i32 = 8192i32;
pub const BINDF2_RESERVED_12: i32 = 16384i32;
pub const BINDF2_RESERVED_11: i32 = 32768i32;
pub const BINDF2_RESERVED_10: i32 = 65536i32;
pub const BINDF2_RESERVED_F: i32 = 131072i32;
pub const BINDF2_RESERVED_E: i32 = 262144i32;
pub const BINDF2_RESERVED_D: i32 = 524288i32;
pub const BINDF2_RESERVED_C: i32 = 1048576i32;
pub const BINDF2_RESERVED_B: i32 = 2097152i32;
pub const BINDF2_RESERVED_A: i32 = 4194304i32;
pub const BINDF2_RESERVED_9: i32 = 8388608i32;
pub const BINDF2_RESERVED_8: i32 = 16777216i32;
pub const BINDF2_RESERVED_7: i32 = 33554432i32;
pub const BINDF2_RESERVED_6: i32 = 67108864i32;
pub const BINDF2_RESERVED_5: i32 = 134217728i32;
pub const BINDF2_RESERVED_4: i32 = 268435456i32;
pub const BINDF2_RESERVED_3: i32 = 536870912i32;
pub const BINDF2_RESERVED_2: i32 = 1073741824i32;
pub const BINDF2_RESERVED_1: i32 = -2147483648i32;
pub const BINDHANDLETYPES_APPCACHE: i32 = 0i32;
pub const BINDHANDLETYPES_DEPENDENCY: i32 = 1i32;
pub const BINDHANDLETYPES_COUNT: i32 = 2i32;
pub const BINDINFO_OPTIONS_WININETFLAG: i32 = 65536i32;
pub const BINDINFO_OPTIONS_ENABLE_UTF8: i32 = 131072i32;
pub const BINDINFO_OPTIONS_DISABLE_UTF8: i32 = 262144i32;
pub const BINDINFO_OPTIONS_USE_IE_ENCODING: i32 = 524288i32;
pub const BINDINFO_OPTIONS_BINDTOOBJECT: i32 = 1048576i32;
pub const BINDINFO_OPTIONS_SECURITYOPTOUT: i32 = 2097152i32;
pub const BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN: i32 = 4194304i32;
pub const BINDINFO_OPTIONS_USEBINDSTRINGCREDS: i32 = 8388608i32;
pub const BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS: i32 = 16777216i32;
pub const BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE: i32 = 33554432i32;
pub const BINDINFO_WPC_DOWNLOADBLOCKED: i32 = 134217728i32;
pub const BINDINFO_WPC_LOGGING_ENABLED: i32 = 268435456i32;
pub const BINDINFO_OPTIONS_ALLOWCONNECTDATA: i32 = 536870912i32;
pub const BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS: i32 = 1073741824i32;
pub const BINDINFO_OPTIONS_SHDOCVW_NAVIGATE: i32 = -2147483648i32;
pub const BINDSTATUS_FINDINGRESOURCE: i32 = 1i32;
pub const BINDSTATUS_CONNECTING: i32 = 2i32;
pub const BINDSTATUS_REDIRECTING: i32 = 3i32;
pub const BINDSTATUS_BEGINDOWNLOADDATA: i32 = 4i32;
pub const BINDSTATUS_DOWNLOADINGDATA: i32 = 5i32;
pub const BINDSTATUS_ENDDOWNLOADDATA: i32 = 6i32;
pub const BINDSTATUS_BEGINDOWNLOADCOMPONENTS: i32 = 7i32;
pub const BINDSTATUS_INSTALLINGCOMPONENTS: i32 = 8i32;
pub const BINDSTATUS_ENDDOWNLOADCOMPONENTS: i32 = 9i32;
pub const BINDSTATUS_USINGCACHEDCOPY: i32 = 10i32;
pub const BINDSTATUS_SENDINGREQUEST: i32 = 11i32;
pub const BINDSTATUS_CLASSIDAVAILABLE: i32 = 12i32;
pub const BINDSTATUS_MIMETYPEAVAILABLE: i32 = 13i32;
pub const BINDSTATUS_CACHEFILENAMEAVAILABLE: i32 = 14i32;
pub const BINDSTATUS_BEGINSYNCOPERATION: i32 = 15i32;
pub const BINDSTATUS_ENDSYNCOPERATION: i32 = 16i32;
pub const BINDSTATUS_BEGINUPLOADDATA: i32 = 17i32;
pub const BINDSTATUS_UPLOADINGDATA: i32 = 18i32;
pub const BINDSTATUS_ENDUPLOADDATA: i32 = 19i32;
pub const BINDSTATUS_PROTOCOLCLASSID: i32 = 20i32;
pub const BINDSTATUS_ENCODING: i32 = 21i32;
pub const BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE: i32 = 22i32;
pub const BINDSTATUS_CLASSINSTALLLOCATION: i32 = 23i32;
pub const BINDSTATUS_DECODING: i32 = 24i32;
pub const BINDSTATUS_LOADINGMIMEHANDLER: i32 = 25i32;
pub const BINDSTATUS_CONTENTDISPOSITIONATTACH: i32 = 26i32;
pub const BINDSTATUS_FILTERREPORTMIMETYPE: i32 = 27i32;
pub const BINDSTATUS_CLSIDCANINSTANTIATE: i32 = 28i32;
pub const BINDSTATUS_IUNKNOWNAVAILABLE: i32 = 29i32;
pub const BINDSTATUS_DIRECTBIND: i32 = 30i32;
pub const BINDSTATUS_RAWMIMETYPE: i32 = 31i32;
pub const BINDSTATUS_PROXYDETECTING: i32 = 32i32;
pub const BINDSTATUS_ACCEPTRANGES: i32 = 33i32;
pub const BINDSTATUS_COOKIE_SENT: i32 = 34i32;
pub const BINDSTATUS_COMPACT_POLICY_RECEIVED: i32 = 35i32;
pub const BINDSTATUS_COOKIE_SUPPRESSED: i32 = 36i32;
pub const BINDSTATUS_COOKIE_STATE_UNKNOWN: i32 = 37i32;
pub const BINDSTATUS_COOKIE_STATE_ACCEPT: i32 = 38i32;
pub const BINDSTATUS_COOKIE_STATE_REJECT: i32 = 39i32;
pub const BINDSTATUS_COOKIE_STATE_PROMPT: i32 = 40i32;
pub const BINDSTATUS_COOKIE_STATE_LEASH: i32 = 41i32;
pub const BINDSTATUS_COOKIE_STATE_DOWNGRADE: i32 = 42i32;
pub const BINDSTATUS_POLICY_HREF: i32 = 43i32;
pub const BINDSTATUS_P3P_HEADER: i32 = 44i32;
pub const BINDSTATUS_SESSION_COOKIE_RECEIVED: i32 = 45i32;
pub const BINDSTATUS_PERSISTENT_COOKIE_RECEIVED: i32 = 46i32;
pub const BINDSTATUS_SESSION_COOKIES_ALLOWED: i32 = 47i32;
pub const BINDSTATUS_CACHECONTROL: i32 = 48i32;
pub const BINDSTATUS_CONTENTDISPOSITIONFILENAME: i32 = 49i32;
pub const BINDSTATUS_MIMETEXTPLAINMISMATCH: i32 = 50i32;
pub const BINDSTATUS_PUBLISHERAVAILABLE: i32 = 51i32;
pub const BINDSTATUS_DISPLAYNAMEAVAILABLE: i32 = 52i32;
pub const BINDSTATUS_SSLUX_NAVBLOCKED: i32 = 53i32;
pub const BINDSTATUS_SERVER_MIMETYPEAVAILABLE: i32 = 54i32;
pub const BINDSTATUS_SNIFFED_CLASSIDAVAILABLE: i32 = 55i32;
pub const BINDSTATUS_64BIT_PROGRESS: i32 = 56i32;
pub const BINDSTATUS_LAST: i32 = 56i32;
pub const BINDSTATUS_RESERVED_0: i32 = 57i32;
pub const BINDSTATUS_RESERVED_1: i32 = 58i32;
pub const BINDSTATUS_RESERVED_2: i32 = 59i32;
pub const BINDSTATUS_RESERVED_3: i32 = 60i32;
pub const BINDSTATUS_RESERVED_4: i32 = 61i32;
pub const BINDSTATUS_RESERVED_5: i32 = 62i32;
pub const BINDSTATUS_RESERVED_6: i32 = 63i32;
pub const BINDSTATUS_RESERVED_7: i32 = 64i32;
pub const BINDSTATUS_RESERVED_8: i32 = 65i32;
pub const BINDSTATUS_RESERVED_9: i32 = 66i32;
pub const BINDSTATUS_RESERVED_A: i32 = 67i32;
pub const BINDSTATUS_RESERVED_B: i32 = 68i32;
pub const BINDSTATUS_RESERVED_C: i32 = 69i32;
pub const BINDSTATUS_RESERVED_D: i32 = 70i32;
pub const BINDSTATUS_RESERVED_E: i32 = 71i32;
pub const BINDSTATUS_RESERVED_F: i32 = 72i32;
pub const BINDSTATUS_RESERVED_10: i32 = 73i32;
pub const BINDSTATUS_RESERVED_11: i32 = 74i32;
pub const BINDSTATUS_RESERVED_12: i32 = 75i32;
pub const BINDSTATUS_RESERVED_13: i32 = 76i32;
pub const BINDSTATUS_RESERVED_14: i32 = 77i32;
pub const BINDSTATUS_LAST_PRIVATE: i32 = 77i32;
pub const BINDSTRING_HEADERS: i32 = 1i32;
pub const BINDSTRING_ACCEPT_MIMES: i32 = 2i32;
pub const BINDSTRING_EXTRA_URL: i32 = 3i32;
pub const BINDSTRING_LANGUAGE: i32 = 4i32;
pub const BINDSTRING_USERNAME: i32 = 5i32;
pub const BINDSTRING_PASSWORD: i32 = 6i32;
pub const BINDSTRING_UA_PIXELS: i32 = 7i32;
pub const BINDSTRING_UA_COLOR: i32 = 8i32;
pub const BINDSTRING_OS: i32 = 9i32;
pub const BINDSTRING_USER_AGENT: i32 = 10i32;
pub const BINDSTRING_ACCEPT_ENCODINGS: i32 = 11i32;
pub const BINDSTRING_POST_COOKIE: i32 = 12i32;
pub const BINDSTRING_POST_DATA_MIME: i32 = 13i32;
pub const BINDSTRING_URL: i32 = 14i32;
pub const BINDSTRING_IID: i32 = 15i32;
pub const BINDSTRING_FLAG_BIND_TO_OBJECT: i32 = 16i32;
pub const BINDSTRING_PTR_BIND_CONTEXT: i32 = 17i32;
pub const BINDSTRING_XDR_ORIGIN: i32 = 18i32;
pub const BINDSTRING_DOWNLOADPATH: i32 = 19i32;
pub const BINDSTRING_ROOTDOC_URL: i32 = 20i32;
pub const BINDSTRING_INITIAL_FILENAME: i32 = 21i32;
pub const BINDSTRING_PROXY_USERNAME: i32 = 22i32;
pub const BINDSTRING_PROXY_PASSWORD: i32 = 23i32;
pub const BINDSTRING_ENTERPRISE_ID: i32 = 24i32;
pub const BINDSTRING_DOC_URL: i32 = 25i32;
pub const BINDSTRING_SAMESITE_COOKIE_LEVEL: i32 = 26i32;
pub const BINDVERB_GET: i32 = 0i32;
pub const BINDVERB_POST: i32 = 1i32;
pub const BINDVERB_PUT: i32 = 2i32;
pub const BINDVERB_CUSTOM: i32 = 3i32;
pub const BINDVERB_RESERVED1: i32 = 4i32;
pub const BSCF_FIRSTDATANOTIFICATION: i32 = 1i32;
pub const BSCF_INTERMEDIATEDATANOTIFICATION: i32 = 2i32;
pub const BSCF_LASTDATANOTIFICATION: i32 = 4i32;
pub const BSCF_DATAFULLYAVAILABLE: i32 = 8i32;
pub const BSCF_AVAILABLEDATASIZEUNKNOWN: i32 = 16i32;
pub const BSCF_SKIPDRAINDATAFORFILEURLS: i32 = 32i32;
pub const BSCF_64BITLENGTHDOWNLOAD: i32 = 64i32;
pub const CF_NULL: u32 = 0u32;
pub const CIP_DISK_FULL: i32 = 0i32;
pub const CIP_ACCESS_DENIED: i32 = 1i32;
pub const CIP_NEWER_VERSION_EXISTS: i32 = 2i32;
pub const CIP_OLDER_VERSION_EXISTS: i32 = 3i32;
pub const CIP_NAME_CONFLICT: i32 = 4i32;
pub const CIP_TRUST_VERIFICATION_COMPONENT_MISSING: i32 = 5i32;
pub const CIP_EXE_SELF_REGISTERATION_TIMEOUT: i32 = 6i32;
pub const CIP_UNSAFE_TO_ABORT: i32 = 7i32;
pub const CIP_NEED_REBOOT: i32 = 8i32;
pub const CIP_NEED_REBOOT_UI_PERMISSION: i32 = 9i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CODEBASEHOLD {
    pub cbSize: u32,
    pub szDistUnit: super::super::super::Foundation::PWSTR,
    pub szCodeBase: super::super::super::Foundation::PWSTR,
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
    pub dwStyle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CODEBASEHOLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CODEBASEHOLD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CONFIRMSAFETY {
    pub clsid: ::windows_sys::core::GUID,
    pub pUnk: ::windows_sys::core::IUnknown,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CONFIRMSAFETY {}
impl ::core::clone::Clone for CONFIRMSAFETY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1u32;
#[repr(C)]
pub struct DATAINFO {
    pub ulTotalSize: u32,
    pub ulavrPacketSize: u32,
    pub ulConnectSpeed: u32,
    pub ulProcessorSpeed: u32,
}
impl ::core::marker::Copy for DATAINFO {}
impl ::core::clone::Clone for DATAINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const E_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147483638i32 as _);
pub const FIEF_FLAG_FORCE_JITUI: u32 = 1u32;
pub const FIEF_FLAG_PEEK: u32 = 2u32;
pub const FIEF_FLAG_RESERVED_0: u32 = 8u32;
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: u32 = 4u32;
pub const FMFD_DEFAULT: u32 = 0u32;
pub const FMFD_ENABLEMIMESNIFFING: u32 = 2u32;
pub const FMFD_IGNOREMIMETEXTPLAIN: u32 = 4u32;
pub const FMFD_RESERVED_1: u32 = 64u32;
pub const FMFD_RESERVED_2: u32 = 128u32;
pub const FMFD_RESPECTTEXTPLAIN: u32 = 16u32;
pub const FMFD_RETURNUPDATEDIMGMIMES: u32 = 32u32;
pub const FMFD_SERVERMIME: u32 = 8u32;
pub const FMFD_URLASFILENAME: u32 = 1u32;
pub const GET_FEATURE_FROM_PROCESS: u32 = 2u32;
pub const GET_FEATURE_FROM_REGISTRY: u32 = 4u32;
pub const GET_FEATURE_FROM_THREAD: u32 = 1u32;
pub const GET_FEATURE_FROM_THREAD_INTERNET: u32 = 64u32;
pub const GET_FEATURE_FROM_THREAD_INTRANET: u32 = 16u32;
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: u32 = 8u32;
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: u32 = 128u32;
pub const GET_FEATURE_FROM_THREAD_TRUSTED: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HIT_LOGGING_INFO {
    pub dwStructSize: u32,
    pub lpszLoggedUrlName: super::super::super::Foundation::PSTR,
    pub StartTime: super::super::super::Foundation::SYSTEMTIME,
    pub EndTime: super::super::super::Foundation::SYSTEMTIME,
    pub lpszExtendedInfo: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIT_LOGGING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIT_LOGGING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindCallbackRedirect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindCallbackRedirect {}
impl ::core::clone::Clone for IBindCallbackRedirect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindHttpSecurity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindHttpSecurity {}
impl ::core::clone::Clone for IBindHttpSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindProtocol(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindProtocol {}
impl ::core::clone::Clone for IBindProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICatalogFileInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICatalogFileInfo {}
impl ::core::clone::Clone for ICatalogFileInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICodeInstall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICodeInstall {}
impl ::core::clone::Clone for ICodeInstall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataFilter {}
impl ::core::clone::Clone for IDataFilter {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IE_EPM_OBJECT_EVENT: i32 = 0i32;
pub const IE_EPM_OBJECT_MUTEX: i32 = 1i32;
pub const IE_EPM_OBJECT_SEMAPHORE: i32 = 2i32;
pub const IE_EPM_OBJECT_SHARED_MEMORY: i32 = 3i32;
pub const IE_EPM_OBJECT_WAITABLE_TIMER: i32 = 4i32;
pub const IE_EPM_OBJECT_FILE: i32 = 5i32;
pub const IE_EPM_OBJECT_NAMED_PIPE: i32 = 6i32;
pub const IE_EPM_OBJECT_REGISTRY: i32 = 7i32;
#[repr(transparent)]
pub struct IEncodingFilterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEncodingFilterFactory {}
impl ::core::clone::Clone for IEncodingFilterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetBindHandle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetBindHandle {}
impl ::core::clone::Clone for IGetBindHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpNegotiate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpNegotiate {}
impl ::core::clone::Clone for IHttpNegotiate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpNegotiate2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpNegotiate2 {}
impl ::core::clone::Clone for IHttpNegotiate2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpNegotiate3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpNegotiate3 {}
impl ::core::clone::Clone for IHttpNegotiate3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpSecurity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpSecurity {}
impl ::core::clone::Clone for IHttpSecurity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternet {}
impl ::core::clone::Clone for IInternet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetBindInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetBindInfo {}
impl ::core::clone::Clone for IInternetBindInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetBindInfoEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetBindInfoEx {}
impl ::core::clone::Clone for IInternetBindInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetHostSecurityManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetHostSecurityManager {}
impl ::core::clone::Clone for IInternetHostSecurityManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetPriority(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetPriority {}
impl ::core::clone::Clone for IInternetPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetProtocol(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetProtocol {}
impl ::core::clone::Clone for IInternetProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetProtocolEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetProtocolEx {}
impl ::core::clone::Clone for IInternetProtocolEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetProtocolInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetProtocolInfo {}
impl ::core::clone::Clone for IInternetProtocolInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetProtocolRoot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetProtocolRoot {}
impl ::core::clone::Clone for IInternetProtocolRoot {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetProtocolSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetProtocolSink {}
impl ::core::clone::Clone for IInternetProtocolSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetProtocolSinkStackable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetProtocolSinkStackable {}
impl ::core::clone::Clone for IInternetProtocolSinkStackable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetSecurityManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetSecurityManager {}
impl ::core::clone::Clone for IInternetSecurityManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetSecurityManagerEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetSecurityManagerEx {}
impl ::core::clone::Clone for IInternetSecurityManagerEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetSecurityManagerEx2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetSecurityManagerEx2 {}
impl ::core::clone::Clone for IInternetSecurityManagerEx2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetSecurityMgrSite(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetSecurityMgrSite {}
impl ::core::clone::Clone for IInternetSecurityMgrSite {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetSession {}
impl ::core::clone::Clone for IInternetSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetThreadSwitch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetThreadSwitch {}
impl ::core::clone::Clone for IInternetThreadSwitch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetZoneManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetZoneManager {}
impl ::core::clone::Clone for IInternetZoneManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetZoneManagerEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetZoneManagerEx {}
impl ::core::clone::Clone for IInternetZoneManagerEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInternetZoneManagerEx2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInternetZoneManagerEx2 {}
impl ::core::clone::Clone for IInternetZoneManagerEx2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMonikerProp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMonikerProp {}
impl ::core::clone::Clone for IMonikerProp {
    fn clone(&self) -> Self {
        *self
    }
}
pub const INET_E_AUTHENTICATION_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697207i32 as _);
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695930i32 as _);
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695931i32 as _);
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697189i32 as _);
pub const INET_E_CANNOT_CONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697212i32 as _);
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697200i32 as _);
pub const INET_E_CANNOT_LOAD_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697201i32 as _);
pub const INET_E_CANNOT_LOCK_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697194i32 as _);
pub const INET_E_CANNOT_REPLACE_SFP_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696448i32 as _);
pub const INET_E_CODE_DOWNLOAD_DECLINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696960i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695932i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695929i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695936i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695934i32 as _);
pub const INET_E_CODE_INSTALL_SUPPRESSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696192i32 as _);
pub const INET_E_CONNECTION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697205i32 as _);
pub const INET_E_DATA_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697209i32 as _);
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199i32;
pub const INET_E_DOMINJECTIONVALIDATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697188i32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695928i32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695935i32 as _);
pub const INET_E_DOWNLOAD_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697208i32 as _);
pub const INET_E_ERROR_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697214i32 as _);
pub const INET_E_ERROR_LAST: i32 = -2146695928i32;
pub const INET_E_FORBIDFRAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695933i32 as _);
pub const INET_E_HSTS_CERTIFICATE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697186i32 as _);
pub const INET_E_INVALID_CERTIFICATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697191i32 as _);
pub const INET_E_INVALID_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697204i32 as _);
pub const INET_E_INVALID_URL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697214i32 as _);
pub const INET_E_NO_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697213i32 as _);
pub const INET_E_NO_VALID_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697206i32 as _);
pub const INET_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697210i32 as _);
pub const INET_E_QUERYOPTION_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697197i32 as _);
pub const INET_E_REDIRECTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697196i32 as _);
pub const INET_E_REDIRECT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697196i32 as _);
pub const INET_E_REDIRECT_TO_DIR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697195i32 as _);
pub const INET_E_RESERVED_1: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697190i32 as _);
pub const INET_E_RESERVED_2: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697185i32 as _);
pub const INET_E_RESERVED_3: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697184i32 as _);
pub const INET_E_RESERVED_4: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697183i32 as _);
pub const INET_E_RESERVED_5: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697182i32 as _);
pub const INET_E_RESOURCE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697211i32 as _);
pub const INET_E_RESULT_DISPATCHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696704i32 as _);
pub const INET_E_SECURITY_PROBLEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697202i32 as _);
pub const INET_E_TERMINATED_BIND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697192i32 as _);
pub const INET_E_UNKNOWN_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697203i32 as _);
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697199i32 as _);
pub const INET_E_USE_DEFAULT_SETTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697198i32 as _);
pub const INET_E_USE_EXTEND_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697193i32 as _);
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697187i32 as _);
pub const MAX_ZONE_PATH: i32 = 260i32;
pub const MAX_ZONE_DESCRIPTION: i32 = 200i32;
pub const FEATURE_OBJECT_CACHING: i32 = 0i32;
pub const FEATURE_ZONE_ELEVATION: i32 = 1i32;
pub const FEATURE_MIME_HANDLING: i32 = 2i32;
pub const FEATURE_MIME_SNIFFING: i32 = 3i32;
pub const FEATURE_WINDOW_RESTRICTIONS: i32 = 4i32;
pub const FEATURE_WEBOC_POPUPMANAGEMENT: i32 = 5i32;
pub const FEATURE_BEHAVIORS: i32 = 6i32;
pub const FEATURE_DISABLE_MK_PROTOCOL: i32 = 7i32;
pub const FEATURE_LOCALMACHINE_LOCKDOWN: i32 = 8i32;
pub const FEATURE_SECURITYBAND: i32 = 9i32;
pub const FEATURE_RESTRICT_ACTIVEXINSTALL: i32 = 10i32;
pub const FEATURE_VALIDATE_NAVIGATE_URL: i32 = 11i32;
pub const FEATURE_RESTRICT_FILEDOWNLOAD: i32 = 12i32;
pub const FEATURE_ADDON_MANAGEMENT: i32 = 13i32;
pub const FEATURE_PROTOCOL_LOCKDOWN: i32 = 14i32;
pub const FEATURE_HTTP_USERNAME_PASSWORD_DISABLE: i32 = 15i32;
pub const FEATURE_SAFE_BINDTOOBJECT: i32 = 16i32;
pub const FEATURE_UNC_SAVEDFILECHECK: i32 = 17i32;
pub const FEATURE_GET_URL_DOM_FILEPATH_UNENCODED: i32 = 18i32;
pub const FEATURE_TABBED_BROWSING: i32 = 19i32;
pub const FEATURE_SSLUX: i32 = 20i32;
pub const FEATURE_DISABLE_NAVIGATION_SOUNDS: i32 = 21i32;
pub const FEATURE_DISABLE_LEGACY_COMPRESSION: i32 = 22i32;
pub const FEATURE_FORCE_ADDR_AND_STATUS: i32 = 23i32;
pub const FEATURE_XMLHTTP: i32 = 24i32;
pub const FEATURE_DISABLE_TELNET_PROTOCOL: i32 = 25i32;
pub const FEATURE_FEEDS: i32 = 26i32;
pub const FEATURE_BLOCK_INPUT_PROMPTS: i32 = 27i32;
pub const FEATURE_ENTRY_COUNT: i32 = 28i32;
#[repr(transparent)]
pub struct IPersistMoniker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistMoniker {}
impl ::core::clone::Clone for IPersistMoniker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISoftDistExt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISoftDistExt {}
impl ::core::clone::Clone for ISoftDistExt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriBuilderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriBuilderFactory {}
impl ::core::clone::Clone for IUriBuilderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUriContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUriContainer {}
impl ::core::clone::Clone for IUriContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinInetCacheHints(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinInetCacheHints {}
impl ::core::clone::Clone for IWinInetCacheHints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinInetCacheHints2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinInetCacheHints2 {}
impl ::core::clone::Clone for IWinInetCacheHints2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinInetFileStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinInetFileStream {}
impl ::core::clone::Clone for IWinInetFileStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinInetHttpInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinInetHttpInfo {}
impl ::core::clone::Clone for IWinInetHttpInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinInetHttpTimeouts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinInetHttpTimeouts {}
impl ::core::clone::Clone for IWinInetHttpTimeouts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinInetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinInetInfo {}
impl ::core::clone::Clone for IWinInetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowForBindingUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowForBindingUI {}
impl ::core::clone::Clone for IWindowForBindingUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWrappedProtocol(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWrappedProtocol {}
impl ::core::clone::Clone for IWrappedProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IZoneIdentifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IZoneIdentifier {}
impl ::core::clone::Clone for IZoneIdentifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IZoneIdentifier2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IZoneIdentifier2 {}
impl ::core::clone::Clone for IZoneIdentifier2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAX_SIZE_SECURITY_ID: u32 = 512u32;
pub const MKSYS_URLMONIKER: u32 = 6u32;
pub const MK_S_ASYNCHRONOUS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262632i32 as _);
pub const MIMETYPEPROP: i32 = 0i32;
pub const USE_SRC_URL: i32 = 1i32;
pub const CLASSIDPROP: i32 = 2i32;
pub const TRUSTEDDOWNLOADPROP: i32 = 3i32;
pub const POPUPLEVELPROP: i32 = 4i32;
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: u32 = 128u32;
pub const MUTZ_DONT_UNESCAPE: u32 = 2048u32;
pub const MUTZ_DONT_USE_CACHE: u32 = 4096u32;
pub const MUTZ_ENFORCERESTRICTED: u32 = 256u32;
pub const MUTZ_FORCE_INTRANET_FLAGS: u32 = 8192u32;
pub const MUTZ_IGNORE_ZONE_MAPPINGS: u32 = 16384u32;
pub const MUTZ_ISFILE: u32 = 2u32;
pub const MUTZ_NOSAVEDFILECHECK: u32 = 1u32;
pub const MUTZ_REQUIRESAVEDFILECHECK: u32 = 1024u32;
pub const MUTZ_RESERVED: u32 = 512u32;
pub const OIBDG_APARTMENTTHREADED: i32 = 256i32;
pub const OIBDG_DATAONLY: i32 = 4096i32;
pub const PARSE_CANONICALIZE: i32 = 1i32;
pub const PARSE_FRIENDLY: i32 = 2i32;
pub const PARSE_SECURITY_URL: i32 = 3i32;
pub const PARSE_ROOTDOCUMENT: i32 = 4i32;
pub const PARSE_DOCUMENT: i32 = 5i32;
pub const PARSE_ANCHOR: i32 = 6i32;
pub const PARSE_ENCODE_IS_UNESCAPE: i32 = 7i32;
pub const PARSE_DECODE_IS_ESCAPE: i32 = 8i32;
pub const PARSE_PATH_FROM_URL: i32 = 9i32;
pub const PARSE_URL_FROM_PATH: i32 = 10i32;
pub const PARSE_MIME: i32 = 11i32;
pub const PARSE_SERVER: i32 = 12i32;
pub const PARSE_SCHEMA: i32 = 13i32;
pub const PARSE_SITE: i32 = 14i32;
pub const PARSE_DOMAIN: i32 = 15i32;
pub const PARSE_LOCATION: i32 = 16i32;
pub const PARSE_SECURITY_DOMAIN: i32 = 17i32;
pub const PARSE_ESCAPE: i32 = 18i32;
pub const PARSE_UNESCAPE: i32 = 19i32;
pub const PI_PARSE_URL: i32 = 1i32;
pub const PI_FILTER_MODE: i32 = 2i32;
pub const PI_FORCE_ASYNC: i32 = 4i32;
pub const PI_USE_WORKERTHREAD: i32 = 8i32;
pub const PI_MIMEVERIFICATION: i32 = 16i32;
pub const PI_CLSIDLOOKUP: i32 = 32i32;
pub const PI_DATAPROGRESS: i32 = 64i32;
pub const PI_SYNCHRONOUS: i32 = 128i32;
pub const PI_APARTMENTTHREADED: i32 = 256i32;
pub const PI_CLASSINSTALL: i32 = 512i32;
pub const PI_PASSONBINDCTX: i32 = 8192i32;
pub const PI_NOMIMEHANDLER: i32 = 32768i32;
pub const PI_LOADAPPDIRECT: i32 = 16384i32;
pub const PD_FORCE_SWITCH: i32 = 65536i32;
pub const PI_PREFERDEFAULTHANDLER: i32 = 131072i32;
#[repr(C)]
pub struct PROTOCOLDATA {
    pub grfFlags: u32,
    pub dwState: u32,
    pub pData: *mut ::core::ffi::c_void,
    pub cbData: u32,
}
impl ::core::marker::Copy for PROTOCOLDATA {}
impl ::core::clone::Clone for PROTOCOLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROTOCOLFILTERDATA {
    pub cbSize: u32,
    pub pProtocolSink: IInternetProtocolSink,
    pub pProtocol: IInternetProtocol,
    pub pUnk: ::windows_sys::core::IUnknown,
    pub dwFilterFlags: u32,
}
impl ::core::marker::Copy for PROTOCOLFILTERDATA {}
impl ::core::clone::Clone for PROTOCOLFILTERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROTOCOL_ARGUMENT {
    pub szMethod: super::super::super::Foundation::PWSTR,
    pub szTargetUrl: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROTOCOL_ARGUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROTOCOL_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSU_DEFAULT: i32 = 1i32;
pub const PSU_SECURITY_URL_ONLY: i32 = 2i32;
pub const PUAF_DEFAULT: i32 = 0i32;
pub const PUAF_NOUI: i32 = 1i32;
pub const PUAF_ISFILE: i32 = 2i32;
pub const PUAF_WARN_IF_DENIED: i32 = 4i32;
pub const PUAF_FORCEUI_FOREGROUND: i32 = 8i32;
pub const PUAF_CHECK_TIFS: i32 = 16i32;
pub const PUAF_DONTCHECKBOXINDIALOG: i32 = 32i32;
pub const PUAF_TRUSTED: i32 = 64i32;
pub const PUAF_ACCEPT_WILDCARD_SCHEME: i32 = 128i32;
pub const PUAF_ENFORCERESTRICTED: i32 = 256i32;
pub const PUAF_NOSAVEDFILECHECK: i32 = 512i32;
pub const PUAF_REQUIRESAVEDFILECHECK: i32 = 1024i32;
pub const PUAF_DONT_USE_CACHE: i32 = 4096i32;
pub const PUAF_RESERVED1: i32 = 8192i32;
pub const PUAF_RESERVED2: i32 = 16384i32;
pub const PUAF_LMZ_UNLOCKED: i32 = 65536i32;
pub const PUAF_LMZ_LOCKED: i32 = 131072i32;
pub const PUAF_DEFAULTZONEPOL: i32 = 262144i32;
pub const PUAF_NPL_USE_LOCKED_IF_RESTRICTED: i32 = 524288i32;
pub const PUAF_NOUIIFLOCKED: i32 = 1048576i32;
pub const PUAF_DRAGPROTOCOLCHECK: i32 = 2097152i32;
pub const PUAFOUT_DEFAULT: i32 = 0i32;
pub const PUAFOUT_ISLOCKZONEPOLICY: i32 = 1i32;
pub const QUERY_EXPIRATION_DATE: i32 = 1i32;
pub const QUERY_TIME_OF_LAST_CHANGE: i32 = 2i32;
pub const QUERY_CONTENT_ENCODING: i32 = 3i32;
pub const QUERY_CONTENT_TYPE: i32 = 4i32;
pub const QUERY_REFRESH: i32 = 5i32;
pub const QUERY_RECOMBINE: i32 = 6i32;
pub const QUERY_CAN_NAVIGATE: i32 = 7i32;
pub const QUERY_USES_NETWORK: i32 = 8i32;
pub const QUERY_IS_CACHED: i32 = 9i32;
pub const QUERY_IS_INSTALLEDENTRY: i32 = 10i32;
pub const QUERY_IS_CACHED_OR_MAPPED: i32 = 11i32;
pub const QUERY_USES_CACHE: i32 = 12i32;
pub const QUERY_IS_SECURE: i32 = 13i32;
pub const QUERY_IS_SAFE: i32 = 14i32;
pub const QUERY_USES_HISTORYFOLDER: i32 = 15i32;
pub const QUERY_IS_CACHED_AND_USABLE_OFFLINE: i32 = 16i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REMSECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: u32,
    pub bInheritHandle: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REMSECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REMSECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RemBINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: super::super::super::Foundation::PWSTR,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: super::super::super::Foundation::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: REMSECURITY_ATTRIBUTES,
    pub iid: ::windows_sys::core::GUID,
    pub pUnk: ::windows_sys::core::IUnknown,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RemBINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RemBINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RemFORMATETC {
    pub cfFormat: u32,
    pub ptd: u32,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for RemFORMATETC {}
impl ::core::clone::Clone for RemFORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SECURITY_IE_STATE_GREEN: u32 = 0u32;
pub const SECURITY_IE_STATE_RED: u32 = 1u32;
pub const SET_FEATURE_IN_REGISTRY: u32 = 4u32;
pub const SET_FEATURE_ON_PROCESS: u32 = 2u32;
pub const SET_FEATURE_ON_THREAD: u32 = 1u32;
pub const SET_FEATURE_ON_THREAD_INTERNET: u32 = 64u32;
pub const SET_FEATURE_ON_THREAD_INTRANET: u32 = 16u32;
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: u32 = 8u32;
pub const SET_FEATURE_ON_THREAD_RESTRICTED: u32 = 128u32;
pub const SET_FEATURE_ON_THREAD_TRUSTED: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SOFTDISTINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwAdState: u32,
    pub szTitle: super::super::super::Foundation::PWSTR,
    pub szAbstract: super::super::super::Foundation::PWSTR,
    pub szHREF: super::super::super::Foundation::PWSTR,
    pub dwInstalledVersionMS: u32,
    pub dwInstalledVersionLS: u32,
    pub dwUpdateVersionMS: u32,
    pub dwUpdateVersionLS: u32,
    pub dwAdvertisedVersionMS: u32,
    pub dwAdvertisedVersionLS: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOFTDISTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOFTDISTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SOFTDIST_ADSTATE_AVAILABLE: u32 = 1u32;
pub const SOFTDIST_ADSTATE_DOWNLOADED: u32 = 2u32;
pub const SOFTDIST_ADSTATE_INSTALLED: u32 = 3u32;
pub const SOFTDIST_ADSTATE_NONE: u32 = 0u32;
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: u32 = 8u32;
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: u32 = 4u32;
pub const SOFTDIST_FLAG_USAGE_EMAIL: u32 = 1u32;
pub const SOFTDIST_FLAG_USAGE_PRECACHE: u32 = 2u32;
pub const SZM_CREATE: i32 = 0i32;
pub const SZM_DELETE: i32 = 1i32;
pub const S_ASYNCHRONOUS: i32 = 262632i32;
#[repr(C)]
pub struct StartParam {
    pub iid: ::windows_sys::core::GUID,
    pub pIBindCtx: super::IBindCtx,
    pub pItf: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for StartParam {}
impl ::core::clone::Clone for StartParam {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UAS_EXACTLEGACY: u32 = 4096u32;
pub const URLACTION_ACTIVEX_ALLOW_TDC: u32 = 4620u32;
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: u32 = 4612u32;
pub const URLACTION_ACTIVEX_CURR_MAX: u32 = 4620u32;
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: u32 = 4618u32;
pub const URLACTION_ACTIVEX_MAX: u32 = 5119u32;
pub const URLACTION_ACTIVEX_MIN: u32 = 4608u32;
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: u32 = 4614u32;
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: u32 = 4610u32;
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: u32 = 4619u32;
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: u32 = 4609u32;
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: u32 = 4616u32;
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: u32 = 4615u32;
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: u32 = 4611u32;
pub const URLACTION_ACTIVEX_RUN: u32 = 4608u32;
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: u32 = 4617u32;
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: u32 = 4613u32;
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: u32 = 9986u32;
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: u32 = 9996u32;
pub const URLACTION_ALLOW_APEVALUATION: u32 = 8961u32;
pub const URLACTION_ALLOW_AUDIO_VIDEO: u32 = 9985u32;
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: u32 = 9988u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: u32 = 9994u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: u32 = 9993u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: u32 = 9992u32;
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: u32 = 9997u32;
pub const URLACTION_ALLOW_JSCRIPT_IE: u32 = 5133u32;
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: u32 = 9995u32;
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: u32 = 8960u32;
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: u32 = 9987u32;
pub const URLACTION_ALLOW_VBSCRIPT_IE: u32 = 5132u32;
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: u32 = 5128u32;
pub const URLACTION_ALLOW_XHR_EVALUATION: u32 = 8962u32;
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: u32 = 9990u32;
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: u32 = 9989u32;
pub const URLACTION_AUTHENTICATE_CLIENT: u32 = 6657u32;
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: u32 = 8705u32;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: u32 = 8704u32;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: u32 = 8704u32;
pub const URLACTION_BEHAVIOR_MIN: u32 = 8192u32;
pub const URLACTION_BEHAVIOR_RUN: u32 = 8192u32;
pub const URLACTION_CHANNEL_SOFTDIST_MAX: u32 = 7935u32;
pub const URLACTION_CHANNEL_SOFTDIST_MIN: u32 = 7680u32;
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: u32 = 7685u32;
pub const URLACTION_CLIENT_CERT_PROMPT: u32 = 6660u32;
pub const URLACTION_COOKIES: u32 = 6658u32;
pub const URLACTION_COOKIES_ENABLED: u32 = 6672u32;
pub const URLACTION_COOKIES_SESSION: u32 = 6659u32;
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: u32 = 6662u32;
pub const URLACTION_COOKIES_THIRD_PARTY: u32 = 6661u32;
pub const URLACTION_CREDENTIALS_USE: u32 = 6656u32;
pub const URLACTION_CROSS_DOMAIN_DATA: u32 = 5126u32;
pub const URLACTION_DOTNET_USERCONTROLS: u32 = 8197u32;
pub const URLACTION_DOWNLOAD_CURR_MAX: u32 = 4100u32;
pub const URLACTION_DOWNLOAD_MAX: u32 = 4607u32;
pub const URLACTION_DOWNLOAD_MIN: u32 = 4096u32;
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: u32 = 4097u32;
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: u32 = 4100u32;
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: u32 = 8453u32;
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: u32 = 8455u32;
pub const URLACTION_FEATURE_DATA_BINDING: u32 = 8454u32;
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: u32 = 8452u32;
pub const URLACTION_FEATURE_MIME_SNIFFING: u32 = 8448u32;
pub const URLACTION_FEATURE_MIN: u32 = 8448u32;
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: u32 = 8451u32;
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: u32 = 8450u32;
pub const URLACTION_FEATURE_ZONE_ELEVATION: u32 = 8449u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: u32 = 5645u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: u32 = 5648u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: u32 = 5647u32;
pub const URLACTION_HTML_ALLOW_INDEXEDDB: u32 = 5649u32;
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: u32 = 5643u32;
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: u32 = 5646u32;
pub const URLACTION_HTML_FONT_DOWNLOAD: u32 = 5636u32;
pub const URLACTION_HTML_INCLUDE_FILE_PATH: u32 = 5642u32;
pub const URLACTION_HTML_JAVA_RUN: u32 = 5637u32;
pub const URLACTION_HTML_MAX: u32 = 6143u32;
pub const URLACTION_HTML_META_REFRESH: u32 = 5640u32;
pub const URLACTION_HTML_MIN: u32 = 5632u32;
pub const URLACTION_HTML_MIXED_CONTENT: u32 = 5641u32;
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: u32 = 5644u32;
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: u32 = 5639u32;
pub const URLACTION_HTML_SUBMIT_FORMS: u32 = 5633u32;
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: u32 = 5634u32;
pub const URLACTION_HTML_SUBMIT_FORMS_TO: u32 = 5635u32;
pub const URLACTION_HTML_USERDATA_SAVE: u32 = 5638u32;
pub const URLACTION_INFODELIVERY_CURR_MAX: u32 = 7430u32;
pub const URLACTION_INFODELIVERY_MAX: u32 = 7679u32;
pub const URLACTION_INFODELIVERY_MIN: u32 = 7424u32;
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: u32 = 7424u32;
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: u32 = 7427u32;
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: u32 = 7430u32;
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: u32 = 7425u32;
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: u32 = 7428u32;
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: u32 = 7426u32;
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: u32 = 7429u32;
pub const URLACTION_INPRIVATE_BLOCKING: u32 = 9984u32;
pub const URLACTION_JAVA_CURR_MAX: u32 = 7168u32;
pub const URLACTION_JAVA_MAX: u32 = 7423u32;
pub const URLACTION_JAVA_MIN: u32 = 7168u32;
pub const URLACTION_JAVA_PERMISSIONS: u32 = 7168u32;
pub const URLACTION_LOOSE_XAML: u32 = 9218u32;
pub const URLACTION_LOWRIGHTS: u32 = 9472u32;
pub const URLACTION_MIN: u32 = 4096u32;
pub const URLACTION_NETWORK_CURR_MAX: u32 = 6672u32;
pub const URLACTION_NETWORK_MAX: u32 = 7167u32;
pub const URLACTION_NETWORK_MIN: u32 = 6656u32;
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: u32 = 5131u32;
pub const URLACTION_SCRIPT_CURR_MAX: u32 = 5133u32;
pub const URLACTION_SCRIPT_JAVA_USE: u32 = 5122u32;
pub const URLACTION_SCRIPT_MAX: u32 = 5631u32;
pub const URLACTION_SCRIPT_MIN: u32 = 5120u32;
pub const URLACTION_SCRIPT_NAVIGATE: u32 = 5130u32;
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: u32 = 5121u32;
pub const URLACTION_SCRIPT_PASTE: u32 = 5127u32;
pub const URLACTION_SCRIPT_RUN: u32 = 5120u32;
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: u32 = 5125u32;
pub const URLACTION_SCRIPT_XSSFILTER: u32 = 5129u32;
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: u32 = 6161u32;
pub const URLACTION_SHELL_CURR_MAX: u32 = 6162u32;
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: u32 = 6155u32;
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: u32 = 6150u32;
pub const URLACTION_SHELL_EXECUTE_LOWRISK: u32 = 6152u32;
pub const URLACTION_SHELL_EXECUTE_MODRISK: u32 = 6151u32;
pub const URLACTION_SHELL_EXTENSIONSECURITY: u32 = 6156u32;
pub const URLACTION_SHELL_FILE_DOWNLOAD: u32 = 6147u32;
pub const URLACTION_SHELL_INSTALL_DTITEMS: u32 = 6144u32;
pub const URLACTION_SHELL_MAX: u32 = 6655u32;
pub const URLACTION_SHELL_MIN: u32 = 6144u32;
pub const URLACTION_SHELL_MOVE_OR_COPY: u32 = 6146u32;
pub const URLACTION_SHELL_POPUPMGR: u32 = 6153u32;
pub const URLACTION_SHELL_PREVIEW: u32 = 6159u32;
pub const URLACTION_SHELL_REMOTEQUERY: u32 = 6158u32;
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: u32 = 6154u32;
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: u32 = 6157u32;
pub const URLACTION_SHELL_SHARE: u32 = 6160u32;
pub const URLACTION_SHELL_SHELLEXECUTE: u32 = 6150u32;
pub const URLACTION_SHELL_TOCTOU_RISK: u32 = 6162u32;
pub const URLACTION_SHELL_VERB: u32 = 6148u32;
pub const URLACTION_SHELL_WEBVIEW_VERB: u32 = 6149u32;
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: u32 = 9216u32;
pub const URLACTION_WINFX_SETUP: u32 = 9728u32;
pub const URLACTION_XPS_DOCUMENTS: u32 = 9217u32;
pub const URLMON_OPTION_URL_ENCODING: u32 = 268435460u32;
pub const URLMON_OPTION_USERAGENT: u32 = 268435457u32;
pub const URLMON_OPTION_USERAGENT_REFRESH: u32 = 268435458u32;
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: u32 = 268435464u32;
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: u32 = 268435472u32;
pub const URLOSTRM_GETNEWESTVERSION: u32 = 3u32;
pub const URLOSTRM_USECACHEDCOPY: u32 = 2u32;
pub const URLOSTRM_USECACHEDCOPY_ONLY: u32 = 1u32;
pub const URLPOLICY_ACTIVEX_CHECK_LIST: u32 = 65536u32;
pub const URLPOLICY_ALLOW: u32 = 0u32;
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: u32 = 65536u32;
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: u32 = 0u32;
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: u32 = 196608u32;
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: u32 = 65536u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: u32 = 196608u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: u32 = 131072u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: u32 = 65536u32;
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: u32 = 196608u32;
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: u32 = 131072u32;
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: u32 = 65536u32;
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: u32 = 0u32;
pub const URLPOLICY_DISALLOW: u32 = 3u32;
pub const URLPOLICY_DONTCHECKDLGBOX: u32 = 256u32;
pub const URLPOLICY_JAVA_CUSTOM: u32 = 8388608u32;
pub const URLPOLICY_JAVA_HIGH: u32 = 65536u32;
pub const URLPOLICY_JAVA_LOW: u32 = 196608u32;
pub const URLPOLICY_JAVA_MEDIUM: u32 = 131072u32;
pub const URLPOLICY_JAVA_PROHIBIT: u32 = 0u32;
pub const URLPOLICY_LOG_ON_ALLOW: u32 = 64u32;
pub const URLPOLICY_LOG_ON_DISALLOW: u32 = 128u32;
pub const URLPOLICY_MASK_PERMISSIONS: u32 = 15u32;
pub const URLPOLICY_NOTIFY_ON_ALLOW: u32 = 16u32;
pub const URLPOLICY_NOTIFY_ON_DISALLOW: u32 = 32u32;
pub const URLPOLICY_QUERY: u32 = 1u32;
pub const URLTEMPLATE_CUSTOM: i32 = 0i32;
pub const URLTEMPLATE_PREDEFINED_MIN: i32 = 65536i32;
pub const URLTEMPLATE_LOW: i32 = 65536i32;
pub const URLTEMPLATE_MEDLOW: i32 = 66816i32;
pub const URLTEMPLATE_MEDIUM: i32 = 69632i32;
pub const URLTEMPLATE_MEDHIGH: i32 = 70912i32;
pub const URLTEMPLATE_HIGH: i32 = 73728i32;
pub const URLTEMPLATE_PREDEFINED_MAX: i32 = 131072i32;
pub const URLZONE_INVALID: i32 = -1i32;
pub const URLZONE_PREDEFINED_MIN: i32 = 0i32;
pub const URLZONE_LOCAL_MACHINE: i32 = 0i32;
pub const URLZONE_INTRANET: i32 = 1i32;
pub const URLZONE_TRUSTED: i32 = 2i32;
pub const URLZONE_INTERNET: i32 = 3i32;
pub const URLZONE_UNTRUSTED: i32 = 4i32;
pub const URLZONE_PREDEFINED_MAX: i32 = 999i32;
pub const URLZONE_USER_MIN: i32 = 1000i32;
pub const URLZONE_USER_MAX: i32 = 10000i32;
pub const URLZONEREG_DEFAULT: i32 = 0i32;
pub const URLZONEREG_HKLM: i32 = 1i32;
pub const URLZONEREG_HKCU: i32 = 2i32;
pub const URLZONE_ESC_FLAG: u32 = 256u32;
pub const URL_ENCODING_NONE: i32 = 0i32;
pub const URL_ENCODING_ENABLE_UTF8: i32 = 268435456i32;
pub const URL_ENCODING_DISABLE_UTF8: i32 = 536870912i32;
pub const URL_MK_LEGACY: u32 = 0u32;
pub const URL_MK_NO_CANONICALIZE: u32 = 2u32;
pub const URL_MK_UNIFORM: u32 = 1u32;
pub const UriBuilder_USE_ORIGINAL_FLAGS: u32 = 1u32;
pub const Uri_DISPLAY_IDN_HOST: u32 = 4u32;
pub const Uri_DISPLAY_NO_FRAGMENT: u32 = 1u32;
pub const Uri_DISPLAY_NO_PUNYCODE: u32 = 8u32;
pub const Uri_ENCODING_HOST_IS_IDN: u32 = 4u32;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: u32 = 16u32;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: u32 = 8u32;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: u32 = 64u32;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: u32 = 32u32;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: u32 = 2u32;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: u32 = 1u32;
pub const Uri_HOST_UNKNOWN: i32 = 0i32;
pub const Uri_HOST_DNS: i32 = 1i32;
pub const Uri_HOST_IPV4: i32 = 2i32;
pub const Uri_HOST_IPV6: i32 = 3i32;
pub const Uri_HOST_IDN: i32 = 4i32;
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2u32;
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534u32;
pub const ZAFLAGS_CUSTOM_EDIT: i32 = 1i32;
pub const ZAFLAGS_ADD_SITES: i32 = 2i32;
pub const ZAFLAGS_REQUIRE_VERIFICATION: i32 = 4i32;
pub const ZAFLAGS_INCLUDE_PROXY_OVERRIDE: i32 = 8i32;
pub const ZAFLAGS_INCLUDE_INTRANET_SITES: i32 = 16i32;
pub const ZAFLAGS_NO_UI: i32 = 32i32;
pub const ZAFLAGS_SUPPORTS_VERIFICATION: i32 = 64i32;
pub const ZAFLAGS_UNC_AS_INTRANET: i32 = 128i32;
pub const ZAFLAGS_DETECT_INTRANET: i32 = 256i32;
pub const ZAFLAGS_USE_LOCKED_ZONES: i32 = 65536i32;
pub const ZAFLAGS_VERIFY_TEMPLATE_SETTINGS: i32 = 131072i32;
pub const ZAFLAGS_NO_CACHE: i32 = 262144i32;
#[repr(C)]
pub struct ZONEATTRIBUTES {
    pub cbSize: u32,
    pub szDisplayName: [u16; 260],
    pub szDescription: [u16; 200],
    pub szIconPath: [u16; 260],
    pub dwTemplateMinLevel: u32,
    pub dwTemplateRecommended: u32,
    pub dwTemplateCurrentLevel: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for ZONEATTRIBUTES {}
impl ::core::clone::Clone for ZONEATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
