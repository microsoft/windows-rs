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
#[repr(transparent)]
pub struct AUTHENTICATEF(pub i32);
pub const AUTHENTICATEF_PROXY: AUTHENTICATEF = AUTHENTICATEF(1i32);
pub const AUTHENTICATEF_BASIC: AUTHENTICATEF = AUTHENTICATEF(2i32);
pub const AUTHENTICATEF_HTTP: AUTHENTICATEF = AUTHENTICATEF(4i32);
impl ::core::marker::Copy for AUTHENTICATEF {}
impl ::core::clone::Clone for AUTHENTICATEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDF(pub i32);
pub const BINDF_ASYNCHRONOUS: BINDF = BINDF(1i32);
pub const BINDF_ASYNCSTORAGE: BINDF = BINDF(2i32);
pub const BINDF_NOPROGRESSIVERENDERING: BINDF = BINDF(4i32);
pub const BINDF_OFFLINEOPERATION: BINDF = BINDF(8i32);
pub const BINDF_GETNEWESTVERSION: BINDF = BINDF(16i32);
pub const BINDF_NOWRITECACHE: BINDF = BINDF(32i32);
pub const BINDF_NEEDFILE: BINDF = BINDF(64i32);
pub const BINDF_PULLDATA: BINDF = BINDF(128i32);
pub const BINDF_IGNORESECURITYPROBLEM: BINDF = BINDF(256i32);
pub const BINDF_RESYNCHRONIZE: BINDF = BINDF(512i32);
pub const BINDF_HYPERLINK: BINDF = BINDF(1024i32);
pub const BINDF_NO_UI: BINDF = BINDF(2048i32);
pub const BINDF_SILENTOPERATION: BINDF = BINDF(4096i32);
pub const BINDF_PRAGMA_NO_CACHE: BINDF = BINDF(8192i32);
pub const BINDF_GETCLASSOBJECT: BINDF = BINDF(16384i32);
pub const BINDF_RESERVED_1: BINDF = BINDF(32768i32);
pub const BINDF_FREE_THREADED: BINDF = BINDF(65536i32);
pub const BINDF_DIRECT_READ: BINDF = BINDF(131072i32);
pub const BINDF_FORMS_SUBMIT: BINDF = BINDF(262144i32);
pub const BINDF_GETFROMCACHE_IF_NET_FAIL: BINDF = BINDF(524288i32);
pub const BINDF_FROMURLMON: BINDF = BINDF(1048576i32);
pub const BINDF_FWD_BACK: BINDF = BINDF(2097152i32);
pub const BINDF_PREFERDEFAULTHANDLER: BINDF = BINDF(4194304i32);
pub const BINDF_ENFORCERESTRICTED: BINDF = BINDF(8388608i32);
pub const BINDF_RESERVED_2: BINDF = BINDF(-2147483648i32);
pub const BINDF_RESERVED_3: BINDF = BINDF(16777216i32);
pub const BINDF_RESERVED_4: BINDF = BINDF(33554432i32);
pub const BINDF_RESERVED_5: BINDF = BINDF(67108864i32);
pub const BINDF_RESERVED_6: BINDF = BINDF(134217728i32);
pub const BINDF_RESERVED_7: BINDF = BINDF(1073741824i32);
pub const BINDF_RESERVED_8: BINDF = BINDF(536870912i32);
impl ::core::marker::Copy for BINDF {}
impl ::core::clone::Clone for BINDF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDF2(pub i32);
pub const BINDF2_DISABLEBASICOVERHTTP: BINDF2 = BINDF2(1i32);
pub const BINDF2_DISABLEAUTOCOOKIEHANDLING: BINDF2 = BINDF2(2i32);
pub const BINDF2_READ_DATA_GREATER_THAN_4GB: BINDF2 = BINDF2(4i32);
pub const BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID: BINDF2 = BINDF2(8i32);
pub const BINDF2_SETDOWNLOADMODE: BINDF2 = BINDF2(32i32);
pub const BINDF2_DISABLE_HTTP_REDIRECT_CACHING: BINDF2 = BINDF2(64i32);
pub const BINDF2_KEEP_CALLBACK_MODULE_LOADED: BINDF2 = BINDF2(128i32);
pub const BINDF2_ALLOW_PROXY_CRED_PROMPT: BINDF2 = BINDF2(256i32);
pub const BINDF2_RESERVED_17: BINDF2 = BINDF2(512i32);
pub const BINDF2_RESERVED_16: BINDF2 = BINDF2(1024i32);
pub const BINDF2_RESERVED_15: BINDF2 = BINDF2(2048i32);
pub const BINDF2_RESERVED_14: BINDF2 = BINDF2(4096i32);
pub const BINDF2_RESERVED_13: BINDF2 = BINDF2(8192i32);
pub const BINDF2_RESERVED_12: BINDF2 = BINDF2(16384i32);
pub const BINDF2_RESERVED_11: BINDF2 = BINDF2(32768i32);
pub const BINDF2_RESERVED_10: BINDF2 = BINDF2(65536i32);
pub const BINDF2_RESERVED_F: BINDF2 = BINDF2(131072i32);
pub const BINDF2_RESERVED_E: BINDF2 = BINDF2(262144i32);
pub const BINDF2_RESERVED_D: BINDF2 = BINDF2(524288i32);
pub const BINDF2_RESERVED_C: BINDF2 = BINDF2(1048576i32);
pub const BINDF2_RESERVED_B: BINDF2 = BINDF2(2097152i32);
pub const BINDF2_RESERVED_A: BINDF2 = BINDF2(4194304i32);
pub const BINDF2_RESERVED_9: BINDF2 = BINDF2(8388608i32);
pub const BINDF2_RESERVED_8: BINDF2 = BINDF2(16777216i32);
pub const BINDF2_RESERVED_7: BINDF2 = BINDF2(33554432i32);
pub const BINDF2_RESERVED_6: BINDF2 = BINDF2(67108864i32);
pub const BINDF2_RESERVED_5: BINDF2 = BINDF2(134217728i32);
pub const BINDF2_RESERVED_4: BINDF2 = BINDF2(268435456i32);
pub const BINDF2_RESERVED_3: BINDF2 = BINDF2(536870912i32);
pub const BINDF2_RESERVED_2: BINDF2 = BINDF2(1073741824i32);
pub const BINDF2_RESERVED_1: BINDF2 = BINDF2(-2147483648i32);
impl ::core::marker::Copy for BINDF2 {}
impl ::core::clone::Clone for BINDF2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDHANDLETYPES(pub i32);
pub const BINDHANDLETYPES_APPCACHE: BINDHANDLETYPES = BINDHANDLETYPES(0i32);
pub const BINDHANDLETYPES_DEPENDENCY: BINDHANDLETYPES = BINDHANDLETYPES(1i32);
pub const BINDHANDLETYPES_COUNT: BINDHANDLETYPES = BINDHANDLETYPES(2i32);
impl ::core::marker::Copy for BINDHANDLETYPES {}
impl ::core::clone::Clone for BINDHANDLETYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDINFO_OPTIONS(pub i32);
pub const BINDINFO_OPTIONS_WININETFLAG: BINDINFO_OPTIONS = BINDINFO_OPTIONS(65536i32);
pub const BINDINFO_OPTIONS_ENABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(131072i32);
pub const BINDINFO_OPTIONS_DISABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(262144i32);
pub const BINDINFO_OPTIONS_USE_IE_ENCODING: BINDINFO_OPTIONS = BINDINFO_OPTIONS(524288i32);
pub const BINDINFO_OPTIONS_BINDTOOBJECT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1048576i32);
pub const BINDINFO_OPTIONS_SECURITYOPTOUT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(2097152i32);
pub const BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN: BINDINFO_OPTIONS = BINDINFO_OPTIONS(4194304i32);
pub const BINDINFO_OPTIONS_USEBINDSTRINGCREDS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(8388608i32);
pub const BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(16777216i32);
pub const BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(33554432i32);
pub const BINDINFO_WPC_DOWNLOADBLOCKED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(134217728i32);
pub const BINDINFO_WPC_LOGGING_ENABLED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(268435456i32);
pub const BINDINFO_OPTIONS_ALLOWCONNECTDATA: BINDINFO_OPTIONS = BINDINFO_OPTIONS(536870912i32);
pub const BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1073741824i32);
pub const BINDINFO_OPTIONS_SHDOCVW_NAVIGATE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(-2147483648i32);
impl ::core::marker::Copy for BINDINFO_OPTIONS {}
impl ::core::clone::Clone for BINDINFO_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDSTATUS(pub i32);
pub const BINDSTATUS_FINDINGRESOURCE: BINDSTATUS = BINDSTATUS(1i32);
pub const BINDSTATUS_CONNECTING: BINDSTATUS = BINDSTATUS(2i32);
pub const BINDSTATUS_REDIRECTING: BINDSTATUS = BINDSTATUS(3i32);
pub const BINDSTATUS_BEGINDOWNLOADDATA: BINDSTATUS = BINDSTATUS(4i32);
pub const BINDSTATUS_DOWNLOADINGDATA: BINDSTATUS = BINDSTATUS(5i32);
pub const BINDSTATUS_ENDDOWNLOADDATA: BINDSTATUS = BINDSTATUS(6i32);
pub const BINDSTATUS_BEGINDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(7i32);
pub const BINDSTATUS_INSTALLINGCOMPONENTS: BINDSTATUS = BINDSTATUS(8i32);
pub const BINDSTATUS_ENDDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(9i32);
pub const BINDSTATUS_USINGCACHEDCOPY: BINDSTATUS = BINDSTATUS(10i32);
pub const BINDSTATUS_SENDINGREQUEST: BINDSTATUS = BINDSTATUS(11i32);
pub const BINDSTATUS_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(12i32);
pub const BINDSTATUS_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(13i32);
pub const BINDSTATUS_CACHEFILENAMEAVAILABLE: BINDSTATUS = BINDSTATUS(14i32);
pub const BINDSTATUS_BEGINSYNCOPERATION: BINDSTATUS = BINDSTATUS(15i32);
pub const BINDSTATUS_ENDSYNCOPERATION: BINDSTATUS = BINDSTATUS(16i32);
pub const BINDSTATUS_BEGINUPLOADDATA: BINDSTATUS = BINDSTATUS(17i32);
pub const BINDSTATUS_UPLOADINGDATA: BINDSTATUS = BINDSTATUS(18i32);
pub const BINDSTATUS_ENDUPLOADDATA: BINDSTATUS = BINDSTATUS(19i32);
pub const BINDSTATUS_PROTOCOLCLASSID: BINDSTATUS = BINDSTATUS(20i32);
pub const BINDSTATUS_ENCODING: BINDSTATUS = BINDSTATUS(21i32);
pub const BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(22i32);
pub const BINDSTATUS_CLASSINSTALLLOCATION: BINDSTATUS = BINDSTATUS(23i32);
pub const BINDSTATUS_DECODING: BINDSTATUS = BINDSTATUS(24i32);
pub const BINDSTATUS_LOADINGMIMEHANDLER: BINDSTATUS = BINDSTATUS(25i32);
pub const BINDSTATUS_CONTENTDISPOSITIONATTACH: BINDSTATUS = BINDSTATUS(26i32);
pub const BINDSTATUS_FILTERREPORTMIMETYPE: BINDSTATUS = BINDSTATUS(27i32);
pub const BINDSTATUS_CLSIDCANINSTANTIATE: BINDSTATUS = BINDSTATUS(28i32);
pub const BINDSTATUS_IUNKNOWNAVAILABLE: BINDSTATUS = BINDSTATUS(29i32);
pub const BINDSTATUS_DIRECTBIND: BINDSTATUS = BINDSTATUS(30i32);
pub const BINDSTATUS_RAWMIMETYPE: BINDSTATUS = BINDSTATUS(31i32);
pub const BINDSTATUS_PROXYDETECTING: BINDSTATUS = BINDSTATUS(32i32);
pub const BINDSTATUS_ACCEPTRANGES: BINDSTATUS = BINDSTATUS(33i32);
pub const BINDSTATUS_COOKIE_SENT: BINDSTATUS = BINDSTATUS(34i32);
pub const BINDSTATUS_COMPACT_POLICY_RECEIVED: BINDSTATUS = BINDSTATUS(35i32);
pub const BINDSTATUS_COOKIE_SUPPRESSED: BINDSTATUS = BINDSTATUS(36i32);
pub const BINDSTATUS_COOKIE_STATE_UNKNOWN: BINDSTATUS = BINDSTATUS(37i32);
pub const BINDSTATUS_COOKIE_STATE_ACCEPT: BINDSTATUS = BINDSTATUS(38i32);
pub const BINDSTATUS_COOKIE_STATE_REJECT: BINDSTATUS = BINDSTATUS(39i32);
pub const BINDSTATUS_COOKIE_STATE_PROMPT: BINDSTATUS = BINDSTATUS(40i32);
pub const BINDSTATUS_COOKIE_STATE_LEASH: BINDSTATUS = BINDSTATUS(41i32);
pub const BINDSTATUS_COOKIE_STATE_DOWNGRADE: BINDSTATUS = BINDSTATUS(42i32);
pub const BINDSTATUS_POLICY_HREF: BINDSTATUS = BINDSTATUS(43i32);
pub const BINDSTATUS_P3P_HEADER: BINDSTATUS = BINDSTATUS(44i32);
pub const BINDSTATUS_SESSION_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(45i32);
pub const BINDSTATUS_PERSISTENT_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(46i32);
pub const BINDSTATUS_SESSION_COOKIES_ALLOWED: BINDSTATUS = BINDSTATUS(47i32);
pub const BINDSTATUS_CACHECONTROL: BINDSTATUS = BINDSTATUS(48i32);
pub const BINDSTATUS_CONTENTDISPOSITIONFILENAME: BINDSTATUS = BINDSTATUS(49i32);
pub const BINDSTATUS_MIMETEXTPLAINMISMATCH: BINDSTATUS = BINDSTATUS(50i32);
pub const BINDSTATUS_PUBLISHERAVAILABLE: BINDSTATUS = BINDSTATUS(51i32);
pub const BINDSTATUS_DISPLAYNAMEAVAILABLE: BINDSTATUS = BINDSTATUS(52i32);
pub const BINDSTATUS_SSLUX_NAVBLOCKED: BINDSTATUS = BINDSTATUS(53i32);
pub const BINDSTATUS_SERVER_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(54i32);
pub const BINDSTATUS_SNIFFED_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(55i32);
pub const BINDSTATUS_64BIT_PROGRESS: BINDSTATUS = BINDSTATUS(56i32);
pub const BINDSTATUS_LAST: BINDSTATUS = BINDSTATUS(56i32);
pub const BINDSTATUS_RESERVED_0: BINDSTATUS = BINDSTATUS(57i32);
pub const BINDSTATUS_RESERVED_1: BINDSTATUS = BINDSTATUS(58i32);
pub const BINDSTATUS_RESERVED_2: BINDSTATUS = BINDSTATUS(59i32);
pub const BINDSTATUS_RESERVED_3: BINDSTATUS = BINDSTATUS(60i32);
pub const BINDSTATUS_RESERVED_4: BINDSTATUS = BINDSTATUS(61i32);
pub const BINDSTATUS_RESERVED_5: BINDSTATUS = BINDSTATUS(62i32);
pub const BINDSTATUS_RESERVED_6: BINDSTATUS = BINDSTATUS(63i32);
pub const BINDSTATUS_RESERVED_7: BINDSTATUS = BINDSTATUS(64i32);
pub const BINDSTATUS_RESERVED_8: BINDSTATUS = BINDSTATUS(65i32);
pub const BINDSTATUS_RESERVED_9: BINDSTATUS = BINDSTATUS(66i32);
pub const BINDSTATUS_RESERVED_A: BINDSTATUS = BINDSTATUS(67i32);
pub const BINDSTATUS_RESERVED_B: BINDSTATUS = BINDSTATUS(68i32);
pub const BINDSTATUS_RESERVED_C: BINDSTATUS = BINDSTATUS(69i32);
pub const BINDSTATUS_RESERVED_D: BINDSTATUS = BINDSTATUS(70i32);
pub const BINDSTATUS_RESERVED_E: BINDSTATUS = BINDSTATUS(71i32);
pub const BINDSTATUS_RESERVED_F: BINDSTATUS = BINDSTATUS(72i32);
pub const BINDSTATUS_RESERVED_10: BINDSTATUS = BINDSTATUS(73i32);
pub const BINDSTATUS_RESERVED_11: BINDSTATUS = BINDSTATUS(74i32);
pub const BINDSTATUS_RESERVED_12: BINDSTATUS = BINDSTATUS(75i32);
pub const BINDSTATUS_RESERVED_13: BINDSTATUS = BINDSTATUS(76i32);
pub const BINDSTATUS_RESERVED_14: BINDSTATUS = BINDSTATUS(77i32);
pub const BINDSTATUS_LAST_PRIVATE: BINDSTATUS = BINDSTATUS(77i32);
impl ::core::marker::Copy for BINDSTATUS {}
impl ::core::clone::Clone for BINDSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDSTRING(pub i32);
pub const BINDSTRING_HEADERS: BINDSTRING = BINDSTRING(1i32);
pub const BINDSTRING_ACCEPT_MIMES: BINDSTRING = BINDSTRING(2i32);
pub const BINDSTRING_EXTRA_URL: BINDSTRING = BINDSTRING(3i32);
pub const BINDSTRING_LANGUAGE: BINDSTRING = BINDSTRING(4i32);
pub const BINDSTRING_USERNAME: BINDSTRING = BINDSTRING(5i32);
pub const BINDSTRING_PASSWORD: BINDSTRING = BINDSTRING(6i32);
pub const BINDSTRING_UA_PIXELS: BINDSTRING = BINDSTRING(7i32);
pub const BINDSTRING_UA_COLOR: BINDSTRING = BINDSTRING(8i32);
pub const BINDSTRING_OS: BINDSTRING = BINDSTRING(9i32);
pub const BINDSTRING_USER_AGENT: BINDSTRING = BINDSTRING(10i32);
pub const BINDSTRING_ACCEPT_ENCODINGS: BINDSTRING = BINDSTRING(11i32);
pub const BINDSTRING_POST_COOKIE: BINDSTRING = BINDSTRING(12i32);
pub const BINDSTRING_POST_DATA_MIME: BINDSTRING = BINDSTRING(13i32);
pub const BINDSTRING_URL: BINDSTRING = BINDSTRING(14i32);
pub const BINDSTRING_IID: BINDSTRING = BINDSTRING(15i32);
pub const BINDSTRING_FLAG_BIND_TO_OBJECT: BINDSTRING = BINDSTRING(16i32);
pub const BINDSTRING_PTR_BIND_CONTEXT: BINDSTRING = BINDSTRING(17i32);
pub const BINDSTRING_XDR_ORIGIN: BINDSTRING = BINDSTRING(18i32);
pub const BINDSTRING_DOWNLOADPATH: BINDSTRING = BINDSTRING(19i32);
pub const BINDSTRING_ROOTDOC_URL: BINDSTRING = BINDSTRING(20i32);
pub const BINDSTRING_INITIAL_FILENAME: BINDSTRING = BINDSTRING(21i32);
pub const BINDSTRING_PROXY_USERNAME: BINDSTRING = BINDSTRING(22i32);
pub const BINDSTRING_PROXY_PASSWORD: BINDSTRING = BINDSTRING(23i32);
pub const BINDSTRING_ENTERPRISE_ID: BINDSTRING = BINDSTRING(24i32);
pub const BINDSTRING_DOC_URL: BINDSTRING = BINDSTRING(25i32);
pub const BINDSTRING_SAMESITE_COOKIE_LEVEL: BINDSTRING = BINDSTRING(26i32);
impl ::core::marker::Copy for BINDSTRING {}
impl ::core::clone::Clone for BINDSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BINDVERB(pub i32);
pub const BINDVERB_GET: BINDVERB = BINDVERB(0i32);
pub const BINDVERB_POST: BINDVERB = BINDVERB(1i32);
pub const BINDVERB_PUT: BINDVERB = BINDVERB(2i32);
pub const BINDVERB_CUSTOM: BINDVERB = BINDVERB(3i32);
pub const BINDVERB_RESERVED1: BINDVERB = BINDVERB(4i32);
impl ::core::marker::Copy for BINDVERB {}
impl ::core::clone::Clone for BINDVERB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BSCF(pub i32);
pub const BSCF_FIRSTDATANOTIFICATION: BSCF = BSCF(1i32);
pub const BSCF_INTERMEDIATEDATANOTIFICATION: BSCF = BSCF(2i32);
pub const BSCF_LASTDATANOTIFICATION: BSCF = BSCF(4i32);
pub const BSCF_DATAFULLYAVAILABLE: BSCF = BSCF(8i32);
pub const BSCF_AVAILABLEDATASIZEUNKNOWN: BSCF = BSCF(16i32);
pub const BSCF_SKIPDRAINDATAFORFILEURLS: BSCF = BSCF(32i32);
pub const BSCF_64BITLENGTHDOWNLOAD: BSCF = BSCF(64i32);
impl ::core::marker::Copy for BSCF {}
impl ::core::clone::Clone for BSCF {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_NULL: u32 = 0u32;
#[repr(transparent)]
pub struct CIP_STATUS(pub i32);
pub const CIP_DISK_FULL: CIP_STATUS = CIP_STATUS(0i32);
pub const CIP_ACCESS_DENIED: CIP_STATUS = CIP_STATUS(1i32);
pub const CIP_NEWER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(2i32);
pub const CIP_OLDER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(3i32);
pub const CIP_NAME_CONFLICT: CIP_STATUS = CIP_STATUS(4i32);
pub const CIP_TRUST_VERIFICATION_COMPONENT_MISSING: CIP_STATUS = CIP_STATUS(5i32);
pub const CIP_EXE_SELF_REGISTERATION_TIMEOUT: CIP_STATUS = CIP_STATUS(6i32);
pub const CIP_UNSAFE_TO_ABORT: CIP_STATUS = CIP_STATUS(7i32);
pub const CIP_NEED_REBOOT: CIP_STATUS = CIP_STATUS(8i32);
pub const CIP_NEED_REBOOT_UI_PERMISSION: CIP_STATUS = CIP_STATUS(9i32);
impl ::core::marker::Copy for CIP_STATUS {}
impl ::core::clone::Clone for CIP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct IEObjectType(pub i32);
pub const IE_EPM_OBJECT_EVENT: IEObjectType = IEObjectType(0i32);
pub const IE_EPM_OBJECT_MUTEX: IEObjectType = IEObjectType(1i32);
pub const IE_EPM_OBJECT_SEMAPHORE: IEObjectType = IEObjectType(2i32);
pub const IE_EPM_OBJECT_SHARED_MEMORY: IEObjectType = IEObjectType(3i32);
pub const IE_EPM_OBJECT_WAITABLE_TIMER: IEObjectType = IEObjectType(4i32);
pub const IE_EPM_OBJECT_FILE: IEObjectType = IEObjectType(5i32);
pub const IE_EPM_OBJECT_NAMED_PIPE: IEObjectType = IEObjectType(6i32);
pub const IE_EPM_OBJECT_REGISTRY: IEObjectType = IEObjectType(7i32);
impl ::core::marker::Copy for IEObjectType {}
impl ::core::clone::Clone for IEObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct INET_ZONE_MANAGER_CONSTANTS(pub i32);
pub const MAX_ZONE_PATH: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(260i32);
pub const MAX_ZONE_DESCRIPTION: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(200i32);
impl ::core::marker::Copy for INET_ZONE_MANAGER_CONSTANTS {}
impl ::core::clone::Clone for INET_ZONE_MANAGER_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INTERNETFEATURELIST(pub i32);
pub const FEATURE_OBJECT_CACHING: INTERNETFEATURELIST = INTERNETFEATURELIST(0i32);
pub const FEATURE_ZONE_ELEVATION: INTERNETFEATURELIST = INTERNETFEATURELIST(1i32);
pub const FEATURE_MIME_HANDLING: INTERNETFEATURELIST = INTERNETFEATURELIST(2i32);
pub const FEATURE_MIME_SNIFFING: INTERNETFEATURELIST = INTERNETFEATURELIST(3i32);
pub const FEATURE_WINDOW_RESTRICTIONS: INTERNETFEATURELIST = INTERNETFEATURELIST(4i32);
pub const FEATURE_WEBOC_POPUPMANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(5i32);
pub const FEATURE_BEHAVIORS: INTERNETFEATURELIST = INTERNETFEATURELIST(6i32);
pub const FEATURE_DISABLE_MK_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(7i32);
pub const FEATURE_LOCALMACHINE_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(8i32);
pub const FEATURE_SECURITYBAND: INTERNETFEATURELIST = INTERNETFEATURELIST(9i32);
pub const FEATURE_RESTRICT_ACTIVEXINSTALL: INTERNETFEATURELIST = INTERNETFEATURELIST(10i32);
pub const FEATURE_VALIDATE_NAVIGATE_URL: INTERNETFEATURELIST = INTERNETFEATURELIST(11i32);
pub const FEATURE_RESTRICT_FILEDOWNLOAD: INTERNETFEATURELIST = INTERNETFEATURELIST(12i32);
pub const FEATURE_ADDON_MANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(13i32);
pub const FEATURE_PROTOCOL_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(14i32);
pub const FEATURE_HTTP_USERNAME_PASSWORD_DISABLE: INTERNETFEATURELIST = INTERNETFEATURELIST(15i32);
pub const FEATURE_SAFE_BINDTOOBJECT: INTERNETFEATURELIST = INTERNETFEATURELIST(16i32);
pub const FEATURE_UNC_SAVEDFILECHECK: INTERNETFEATURELIST = INTERNETFEATURELIST(17i32);
pub const FEATURE_GET_URL_DOM_FILEPATH_UNENCODED: INTERNETFEATURELIST = INTERNETFEATURELIST(18i32);
pub const FEATURE_TABBED_BROWSING: INTERNETFEATURELIST = INTERNETFEATURELIST(19i32);
pub const FEATURE_SSLUX: INTERNETFEATURELIST = INTERNETFEATURELIST(20i32);
pub const FEATURE_DISABLE_NAVIGATION_SOUNDS: INTERNETFEATURELIST = INTERNETFEATURELIST(21i32);
pub const FEATURE_DISABLE_LEGACY_COMPRESSION: INTERNETFEATURELIST = INTERNETFEATURELIST(22i32);
pub const FEATURE_FORCE_ADDR_AND_STATUS: INTERNETFEATURELIST = INTERNETFEATURELIST(23i32);
pub const FEATURE_XMLHTTP: INTERNETFEATURELIST = INTERNETFEATURELIST(24i32);
pub const FEATURE_DISABLE_TELNET_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(25i32);
pub const FEATURE_FEEDS: INTERNETFEATURELIST = INTERNETFEATURELIST(26i32);
pub const FEATURE_BLOCK_INPUT_PROMPTS: INTERNETFEATURELIST = INTERNETFEATURELIST(27i32);
pub const FEATURE_ENTRY_COUNT: INTERNETFEATURELIST = INTERNETFEATURELIST(28i32);
impl ::core::marker::Copy for INTERNETFEATURELIST {}
impl ::core::clone::Clone for INTERNETFEATURELIST {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct MONIKERPROPERTY(pub i32);
pub const MIMETYPEPROP: MONIKERPROPERTY = MONIKERPROPERTY(0i32);
pub const USE_SRC_URL: MONIKERPROPERTY = MONIKERPROPERTY(1i32);
pub const CLASSIDPROP: MONIKERPROPERTY = MONIKERPROPERTY(2i32);
pub const TRUSTEDDOWNLOADPROP: MONIKERPROPERTY = MONIKERPROPERTY(3i32);
pub const POPUPLEVELPROP: MONIKERPROPERTY = MONIKERPROPERTY(4i32);
impl ::core::marker::Copy for MONIKERPROPERTY {}
impl ::core::clone::Clone for MONIKERPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct OIBDG_FLAGS(pub i32);
pub const OIBDG_APARTMENTTHREADED: OIBDG_FLAGS = OIBDG_FLAGS(256i32);
pub const OIBDG_DATAONLY: OIBDG_FLAGS = OIBDG_FLAGS(4096i32);
impl ::core::marker::Copy for OIBDG_FLAGS {}
impl ::core::clone::Clone for OIBDG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PARSEACTION(pub i32);
pub const PARSE_CANONICALIZE: PARSEACTION = PARSEACTION(1i32);
pub const PARSE_FRIENDLY: PARSEACTION = PARSEACTION(2i32);
pub const PARSE_SECURITY_URL: PARSEACTION = PARSEACTION(3i32);
pub const PARSE_ROOTDOCUMENT: PARSEACTION = PARSEACTION(4i32);
pub const PARSE_DOCUMENT: PARSEACTION = PARSEACTION(5i32);
pub const PARSE_ANCHOR: PARSEACTION = PARSEACTION(6i32);
pub const PARSE_ENCODE_IS_UNESCAPE: PARSEACTION = PARSEACTION(7i32);
pub const PARSE_DECODE_IS_ESCAPE: PARSEACTION = PARSEACTION(8i32);
pub const PARSE_PATH_FROM_URL: PARSEACTION = PARSEACTION(9i32);
pub const PARSE_URL_FROM_PATH: PARSEACTION = PARSEACTION(10i32);
pub const PARSE_MIME: PARSEACTION = PARSEACTION(11i32);
pub const PARSE_SERVER: PARSEACTION = PARSEACTION(12i32);
pub const PARSE_SCHEMA: PARSEACTION = PARSEACTION(13i32);
pub const PARSE_SITE: PARSEACTION = PARSEACTION(14i32);
pub const PARSE_DOMAIN: PARSEACTION = PARSEACTION(15i32);
pub const PARSE_LOCATION: PARSEACTION = PARSEACTION(16i32);
pub const PARSE_SECURITY_DOMAIN: PARSEACTION = PARSEACTION(17i32);
pub const PARSE_ESCAPE: PARSEACTION = PARSEACTION(18i32);
pub const PARSE_UNESCAPE: PARSEACTION = PARSEACTION(19i32);
impl ::core::marker::Copy for PARSEACTION {}
impl ::core::clone::Clone for PARSEACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PI_FLAGS(pub i32);
pub const PI_PARSE_URL: PI_FLAGS = PI_FLAGS(1i32);
pub const PI_FILTER_MODE: PI_FLAGS = PI_FLAGS(2i32);
pub const PI_FORCE_ASYNC: PI_FLAGS = PI_FLAGS(4i32);
pub const PI_USE_WORKERTHREAD: PI_FLAGS = PI_FLAGS(8i32);
pub const PI_MIMEVERIFICATION: PI_FLAGS = PI_FLAGS(16i32);
pub const PI_CLSIDLOOKUP: PI_FLAGS = PI_FLAGS(32i32);
pub const PI_DATAPROGRESS: PI_FLAGS = PI_FLAGS(64i32);
pub const PI_SYNCHRONOUS: PI_FLAGS = PI_FLAGS(128i32);
pub const PI_APARTMENTTHREADED: PI_FLAGS = PI_FLAGS(256i32);
pub const PI_CLASSINSTALL: PI_FLAGS = PI_FLAGS(512i32);
pub const PI_PASSONBINDCTX: PI_FLAGS = PI_FLAGS(8192i32);
pub const PI_NOMIMEHANDLER: PI_FLAGS = PI_FLAGS(32768i32);
pub const PI_LOADAPPDIRECT: PI_FLAGS = PI_FLAGS(16384i32);
pub const PD_FORCE_SWITCH: PI_FLAGS = PI_FLAGS(65536i32);
pub const PI_PREFERDEFAULTHANDLER: PI_FLAGS = PI_FLAGS(131072i32);
impl ::core::marker::Copy for PI_FLAGS {}
impl ::core::clone::Clone for PI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct PSUACTION(pub i32);
pub const PSU_DEFAULT: PSUACTION = PSUACTION(1i32);
pub const PSU_SECURITY_URL_ONLY: PSUACTION = PSUACTION(2i32);
impl ::core::marker::Copy for PSUACTION {}
impl ::core::clone::Clone for PSUACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PUAF(pub i32);
pub const PUAF_DEFAULT: PUAF = PUAF(0i32);
pub const PUAF_NOUI: PUAF = PUAF(1i32);
pub const PUAF_ISFILE: PUAF = PUAF(2i32);
pub const PUAF_WARN_IF_DENIED: PUAF = PUAF(4i32);
pub const PUAF_FORCEUI_FOREGROUND: PUAF = PUAF(8i32);
pub const PUAF_CHECK_TIFS: PUAF = PUAF(16i32);
pub const PUAF_DONTCHECKBOXINDIALOG: PUAF = PUAF(32i32);
pub const PUAF_TRUSTED: PUAF = PUAF(64i32);
pub const PUAF_ACCEPT_WILDCARD_SCHEME: PUAF = PUAF(128i32);
pub const PUAF_ENFORCERESTRICTED: PUAF = PUAF(256i32);
pub const PUAF_NOSAVEDFILECHECK: PUAF = PUAF(512i32);
pub const PUAF_REQUIRESAVEDFILECHECK: PUAF = PUAF(1024i32);
pub const PUAF_DONT_USE_CACHE: PUAF = PUAF(4096i32);
pub const PUAF_RESERVED1: PUAF = PUAF(8192i32);
pub const PUAF_RESERVED2: PUAF = PUAF(16384i32);
pub const PUAF_LMZ_UNLOCKED: PUAF = PUAF(65536i32);
pub const PUAF_LMZ_LOCKED: PUAF = PUAF(131072i32);
pub const PUAF_DEFAULTZONEPOL: PUAF = PUAF(262144i32);
pub const PUAF_NPL_USE_LOCKED_IF_RESTRICTED: PUAF = PUAF(524288i32);
pub const PUAF_NOUIIFLOCKED: PUAF = PUAF(1048576i32);
pub const PUAF_DRAGPROTOCOLCHECK: PUAF = PUAF(2097152i32);
impl ::core::marker::Copy for PUAF {}
impl ::core::clone::Clone for PUAF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PUAFOUT(pub i32);
pub const PUAFOUT_DEFAULT: PUAFOUT = PUAFOUT(0i32);
pub const PUAFOUT_ISLOCKZONEPOLICY: PUAFOUT = PUAFOUT(1i32);
impl ::core::marker::Copy for PUAFOUT {}
impl ::core::clone::Clone for PUAFOUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QUERYOPTION(pub i32);
pub const QUERY_EXPIRATION_DATE: QUERYOPTION = QUERYOPTION(1i32);
pub const QUERY_TIME_OF_LAST_CHANGE: QUERYOPTION = QUERYOPTION(2i32);
pub const QUERY_CONTENT_ENCODING: QUERYOPTION = QUERYOPTION(3i32);
pub const QUERY_CONTENT_TYPE: QUERYOPTION = QUERYOPTION(4i32);
pub const QUERY_REFRESH: QUERYOPTION = QUERYOPTION(5i32);
pub const QUERY_RECOMBINE: QUERYOPTION = QUERYOPTION(6i32);
pub const QUERY_CAN_NAVIGATE: QUERYOPTION = QUERYOPTION(7i32);
pub const QUERY_USES_NETWORK: QUERYOPTION = QUERYOPTION(8i32);
pub const QUERY_IS_CACHED: QUERYOPTION = QUERYOPTION(9i32);
pub const QUERY_IS_INSTALLEDENTRY: QUERYOPTION = QUERYOPTION(10i32);
pub const QUERY_IS_CACHED_OR_MAPPED: QUERYOPTION = QUERYOPTION(11i32);
pub const QUERY_USES_CACHE: QUERYOPTION = QUERYOPTION(12i32);
pub const QUERY_IS_SECURE: QUERYOPTION = QUERYOPTION(13i32);
pub const QUERY_IS_SAFE: QUERYOPTION = QUERYOPTION(14i32);
pub const QUERY_USES_HISTORYFOLDER: QUERYOPTION = QUERYOPTION(15i32);
pub const QUERY_IS_CACHED_AND_USABLE_OFFLINE: QUERYOPTION = QUERYOPTION(16i32);
impl ::core::marker::Copy for QUERYOPTION {}
impl ::core::clone::Clone for QUERYOPTION {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SZM_FLAGS(pub i32);
pub const SZM_CREATE: SZM_FLAGS = SZM_FLAGS(0i32);
pub const SZM_DELETE: SZM_FLAGS = SZM_FLAGS(1i32);
impl ::core::marker::Copy for SZM_FLAGS {}
impl ::core::clone::Clone for SZM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct URLTEMPLATE(pub i32);
pub const URLTEMPLATE_CUSTOM: URLTEMPLATE = URLTEMPLATE(0i32);
pub const URLTEMPLATE_PREDEFINED_MIN: URLTEMPLATE = URLTEMPLATE(65536i32);
pub const URLTEMPLATE_LOW: URLTEMPLATE = URLTEMPLATE(65536i32);
pub const URLTEMPLATE_MEDLOW: URLTEMPLATE = URLTEMPLATE(66816i32);
pub const URLTEMPLATE_MEDIUM: URLTEMPLATE = URLTEMPLATE(69632i32);
pub const URLTEMPLATE_MEDHIGH: URLTEMPLATE = URLTEMPLATE(70912i32);
pub const URLTEMPLATE_HIGH: URLTEMPLATE = URLTEMPLATE(73728i32);
pub const URLTEMPLATE_PREDEFINED_MAX: URLTEMPLATE = URLTEMPLATE(131072i32);
impl ::core::marker::Copy for URLTEMPLATE {}
impl ::core::clone::Clone for URLTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct URLZONE(pub i32);
pub const URLZONE_INVALID: URLZONE = URLZONE(-1i32);
pub const URLZONE_PREDEFINED_MIN: URLZONE = URLZONE(0i32);
pub const URLZONE_LOCAL_MACHINE: URLZONE = URLZONE(0i32);
pub const URLZONE_INTRANET: URLZONE = URLZONE(1i32);
pub const URLZONE_TRUSTED: URLZONE = URLZONE(2i32);
pub const URLZONE_INTERNET: URLZONE = URLZONE(3i32);
pub const URLZONE_UNTRUSTED: URLZONE = URLZONE(4i32);
pub const URLZONE_PREDEFINED_MAX: URLZONE = URLZONE(999i32);
pub const URLZONE_USER_MIN: URLZONE = URLZONE(1000i32);
pub const URLZONE_USER_MAX: URLZONE = URLZONE(10000i32);
impl ::core::marker::Copy for URLZONE {}
impl ::core::clone::Clone for URLZONE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct URLZONEREG(pub i32);
pub const URLZONEREG_DEFAULT: URLZONEREG = URLZONEREG(0i32);
pub const URLZONEREG_HKLM: URLZONEREG = URLZONEREG(1i32);
pub const URLZONEREG_HKCU: URLZONEREG = URLZONEREG(2i32);
impl ::core::marker::Copy for URLZONEREG {}
impl ::core::clone::Clone for URLZONEREG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const URLZONE_ESC_FLAG: u32 = 256u32;
#[repr(transparent)]
pub struct URL_ENCODING(pub i32);
pub const URL_ENCODING_NONE: URL_ENCODING = URL_ENCODING(0i32);
pub const URL_ENCODING_ENABLE_UTF8: URL_ENCODING = URL_ENCODING(268435456i32);
pub const URL_ENCODING_DISABLE_UTF8: URL_ENCODING = URL_ENCODING(536870912i32);
impl ::core::marker::Copy for URL_ENCODING {}
impl ::core::clone::Clone for URL_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct Uri_HOST_TYPE(pub i32);
pub const Uri_HOST_UNKNOWN: Uri_HOST_TYPE = Uri_HOST_TYPE(0i32);
pub const Uri_HOST_DNS: Uri_HOST_TYPE = Uri_HOST_TYPE(1i32);
pub const Uri_HOST_IPV4: Uri_HOST_TYPE = Uri_HOST_TYPE(2i32);
pub const Uri_HOST_IPV6: Uri_HOST_TYPE = Uri_HOST_TYPE(3i32);
pub const Uri_HOST_IDN: Uri_HOST_TYPE = Uri_HOST_TYPE(4i32);
impl ::core::marker::Copy for Uri_HOST_TYPE {}
impl ::core::clone::Clone for Uri_HOST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2u32;
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534u32;
#[repr(transparent)]
pub struct ZAFLAGS(pub i32);
pub const ZAFLAGS_CUSTOM_EDIT: ZAFLAGS = ZAFLAGS(1i32);
pub const ZAFLAGS_ADD_SITES: ZAFLAGS = ZAFLAGS(2i32);
pub const ZAFLAGS_REQUIRE_VERIFICATION: ZAFLAGS = ZAFLAGS(4i32);
pub const ZAFLAGS_INCLUDE_PROXY_OVERRIDE: ZAFLAGS = ZAFLAGS(8i32);
pub const ZAFLAGS_INCLUDE_INTRANET_SITES: ZAFLAGS = ZAFLAGS(16i32);
pub const ZAFLAGS_NO_UI: ZAFLAGS = ZAFLAGS(32i32);
pub const ZAFLAGS_SUPPORTS_VERIFICATION: ZAFLAGS = ZAFLAGS(64i32);
pub const ZAFLAGS_UNC_AS_INTRANET: ZAFLAGS = ZAFLAGS(128i32);
pub const ZAFLAGS_DETECT_INTRANET: ZAFLAGS = ZAFLAGS(256i32);
pub const ZAFLAGS_USE_LOCKED_ZONES: ZAFLAGS = ZAFLAGS(65536i32);
pub const ZAFLAGS_VERIFY_TEMPLATE_SETTINGS: ZAFLAGS = ZAFLAGS(131072i32);
pub const ZAFLAGS_NO_CACHE: ZAFLAGS = ZAFLAGS(262144i32);
impl ::core::marker::Copy for ZAFLAGS {}
impl ::core::clone::Clone for ZAFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
