#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetClassObjectFromURL(rclassid: *const ::windows_sys::core::GUID, szcode: super::super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, sztype: super::super::super::Foundation::PWSTR, pbindctx: super::IBindCtx, dwclscontext: super::CLSCTX, pvreserved: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCombineIUri(pbaseuri: super::IUri, prelativeuri: super::IUri, dwcombineflags: u32, ppcombineduri: *mut super::IUri, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrl(pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrlEx(pbaseuri: super::IUri, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, ppcombineduri: *mut super::IUri, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCompareUrl(pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCreateSecurityManager(psp: super::IServiceProvider, ppsm: *mut IInternetSecurityManager, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCreateZoneManager(psp: super::IServiceProvider, ppzm: *mut IInternetZoneManager, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetProtocolFlags(pwzurl: super::super::super::Foundation::PWSTR, pdwflags: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetSecurityUrl(pwszurl: super::super::super::Foundation::PWSTR, ppwszsecurl: *mut super::super::super::Foundation::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetGetSecurityUrlEx(puri: super::IUri, ppsecuri: *mut super::IUri, psuaction: PSUACTION, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut IInternetSession, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetIsFeatureEnabledForIUri(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: super::IUri, psecmgr: IInternetSecurityManagerEx2) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureEnabledForUrl(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: super::super::super::Foundation::PWSTR, psecmgr: IInternetSecurityManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureZoneElevationEnabled(szfromurl: super::super::super::Foundation::PWSTR, sztourl: super::super::super::Foundation::PWSTR, psecmgr: IInternetSecurityManager, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseIUri(piuri: super::IUri, parseaction: PARSEACTION, dwflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseUrl(pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetQueryInfo(pwzurl: super::super::super::Foundation::PWSTR, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuffer: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetSetFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CompareSecurityIds(pbsecurityid1: *const u8, dwlen1: u32, pbsecurityid2: *const u8, dwlen2: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CompatFlagsFromClsid(pclsid: *const ::windows_sys::core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyBindInfo(pcbisrc: *const super::BINDINFO, pbidest: *mut super::BINDINFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyStgMedium(pcstgmedsrc: *const super::STGMEDIUM, pstgmeddest: *mut super::STGMEDIUM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateAsyncBindCtx(reserved: u32, pbscb: super::IBindStatusCallback, pefetc: super::IEnumFORMATETC, ppbc: *mut super::IBindCtx) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateAsyncBindCtxEx(pbc: super::IBindCtx, dwoptions: u32, pbscb: super::IBindStatusCallback, penum: super::IEnumFORMATETC, ppbc: *mut super::IBindCtx, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateFormatEnumerator(cfmtetc: u32, rgfmtetc: *const super::FORMATETC, ppenumfmtetc: *mut super::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMoniker(pmkctx: super::IMoniker, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMonikerEx(pmkctx: super::IMoniker, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut super::IMoniker, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateURLMonikerEx2(pmkctx: super::IMoniker, puri: super::IUri, ppmk: *mut super::IMoniker, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaultInIEFeature(hwnd: super::super::super::Foundation::HWND, pclassspec: *const super::uCLSSPEC, pquery: *mut super::QUERYCONTEXT, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaType(rgsztypes: super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaTypeClass(pbc: super::IBindCtx, sztype: super::super::super::Foundation::PSTR, pclsid: *mut ::windows_sys::core::GUID, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMimeFromData(pbc: super::IBindCtx, pwzurl: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, pwzmimeproposed: super::super::super::Foundation::PWSTR, dwmimeflags: u32, ppwzmimeout: *mut super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFileOrMime(pbc: super::IBindCtx, szfilename: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, szmime: super::super::super::Foundation::PWSTR, dwreserved: u32, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassURL(szurl: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC, ppszcomponentid: *mut super::super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSoftwareUpdateInfo(szdistunit: super::super::super::Foundation::PWSTR, psdi: *mut SOFTDISTINFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkGoBack(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkGoForward(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkNavigateMoniker(punk: ::windows_sys::core::IUnknown, pmktarget: super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkNavigateString(punk: ::windows_sys::core::IUnknown, sztarget: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToMoniker(pmktarget: super::IMoniker, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows_sys::core::IUnknown, pbc: super::IBindCtx, param5: super::IBindStatusCallback, grfhlnf: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToString(sztarget: super::super::super::Foundation::PWSTR, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows_sys::core::IUnknown, pbc: super::IBindCtx, param5: super::IBindStatusCallback, grfhlnf: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IEGetUserPrivateNamespaceName() -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn IEInstallScope(pdwscope: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn IsAsyncMoniker(pmk: super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledA(pszurl: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledW(pwszurl: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidURL(pbc: super::IBindCtx, szurl: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayNameEx(pbc: super::IBindCtx, szdisplayname: super::super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut super::IMoniker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObtainUserAgentString(dwoption: u32, pszuaout: super::super::super::Foundation::PSTR, cbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RegisterBindStatusCallback(pbc: super::IBindCtx, pbscb: super::IBindStatusCallback, ppbscbprev: *mut super::IBindStatusCallback, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RegisterFormatEnumerator(pbc: super::IBindCtx, pefetc: super::IEnumFORMATETC, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypeClass(pbc: super::IBindCtx, ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgclsid: *const ::windows_sys::core::GUID, reserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseBindInfo(pbindinfo: *mut super::BINDINFO);
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RevokeBindStatusCallback(pbc: super::IBindCtx, pbscb: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RevokeFormatEnumerator(pbc: super::IBindCtx, pefetc: super::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAccessForIEAppContainer(hobject: super::super::super::Foundation::HANDLE, ieobjecttype: IEObjectType, dwaccessmask: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSoftwareUpdateAdvertisementState(szdistunit: super::super::super::Foundation::PWSTR, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, cchfilename: u32, param4: u32, param5: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, cchfilename: u32, param4: u32, param5: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: *mut super::IStream, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: *mut super::IStream, param3: u32, param4: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamA(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamW(param0: ::windows_sys::core::IUnknown, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: super::IBindStatusCallback) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn UrlMkGetSessionOption(dwoption: u32, pbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn UrlMkSetSessionOption(dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL;
}
pub struct AUTHENTICATEF(i32);
pub struct BINDF(i32);
pub struct BINDF2(i32);
pub struct BINDHANDLETYPES(i32);
pub struct BINDINFO_OPTIONS(i32);
pub struct BINDSTATUS(i32);
pub struct BINDSTRING(i32);
pub struct BINDVERB(i32);
pub struct BSCF(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const CF_NULL: u32 = 0u32;
pub struct CIP_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CODEBASEHOLD(i32);
pub struct CONFIRMSAFETY(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1u32;
pub struct DATAINFO(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const E_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147483638i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FIEF_FLAG_FORCE_JITUI: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FIEF_FLAG_PEEK: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FIEF_FLAG_RESERVED_0: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_ENABLEMIMESNIFFING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_IGNOREMIMETEXTPLAIN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_RESERVED_1: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_RESERVED_2: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_RESPECTTEXTPLAIN: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_RETURNUPDATEDIMGMIMES: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_SERVERMIME: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const FMFD_URLASFILENAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_PROCESS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_THREAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_THREAD_INTERNET: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_THREAD_INTRANET: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const GET_FEATURE_FROM_THREAD_TRUSTED: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct HIT_LOGGING_INFO(i32);
pub struct IBindCallbackRedirect(pub *mut ::core::ffi::c_void);
pub struct IBindHttpSecurity(pub *mut ::core::ffi::c_void);
pub struct IBindProtocol(pub *mut ::core::ffi::c_void);
pub struct ICatalogFileInfo(pub *mut ::core::ffi::c_void);
pub struct ICodeInstall(pub *mut ::core::ffi::c_void);
pub struct IDataFilter(pub *mut ::core::ffi::c_void);
pub struct IEObjectType(i32);
pub struct IEncodingFilterFactory(pub *mut ::core::ffi::c_void);
pub struct IGetBindHandle(pub *mut ::core::ffi::c_void);
pub struct IHttpNegotiate(pub *mut ::core::ffi::c_void);
pub struct IHttpNegotiate2(pub *mut ::core::ffi::c_void);
pub struct IHttpNegotiate3(pub *mut ::core::ffi::c_void);
pub struct IHttpSecurity(pub *mut ::core::ffi::c_void);
pub struct IInternet(pub *mut ::core::ffi::c_void);
pub struct IInternetBindInfo(pub *mut ::core::ffi::c_void);
pub struct IInternetBindInfoEx(pub *mut ::core::ffi::c_void);
pub struct IInternetHostSecurityManager(pub *mut ::core::ffi::c_void);
pub struct IInternetPriority(pub *mut ::core::ffi::c_void);
pub struct IInternetProtocol(pub *mut ::core::ffi::c_void);
pub struct IInternetProtocolEx(pub *mut ::core::ffi::c_void);
pub struct IInternetProtocolInfo(pub *mut ::core::ffi::c_void);
pub struct IInternetProtocolRoot(pub *mut ::core::ffi::c_void);
pub struct IInternetProtocolSink(pub *mut ::core::ffi::c_void);
pub struct IInternetProtocolSinkStackable(pub *mut ::core::ffi::c_void);
pub struct IInternetSecurityManager(pub *mut ::core::ffi::c_void);
pub struct IInternetSecurityManagerEx(pub *mut ::core::ffi::c_void);
pub struct IInternetSecurityManagerEx2(pub *mut ::core::ffi::c_void);
pub struct IInternetSecurityMgrSite(pub *mut ::core::ffi::c_void);
pub struct IInternetSession(pub *mut ::core::ffi::c_void);
pub struct IInternetThreadSwitch(pub *mut ::core::ffi::c_void);
pub struct IInternetZoneManager(pub *mut ::core::ffi::c_void);
pub struct IInternetZoneManagerEx(pub *mut ::core::ffi::c_void);
pub struct IInternetZoneManagerEx2(pub *mut ::core::ffi::c_void);
pub struct IMonikerProp(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_AUTHENTICATION_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697207i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695930i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695931i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697189i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CANNOT_CONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697212i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697200i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CANNOT_LOAD_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697201i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CANNOT_LOCK_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697194i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CANNOT_REPLACE_SFP_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696448i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CODE_DOWNLOAD_DECLINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696960i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695932i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695929i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695936i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695934i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CODE_INSTALL_SUPPRESSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696192i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_CONNECTION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697205i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_DATA_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697209i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199i32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_DOMINJECTIONVALIDATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697188i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695928i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695935i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_DOWNLOAD_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697208i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_ERROR_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697214i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_ERROR_LAST: i32 = -2146695928i32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_FORBIDFRAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146695933i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_HSTS_CERTIFICATE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697186i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_INVALID_CERTIFICATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697191i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_INVALID_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697204i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_INVALID_URL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697214i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_NO_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697213i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_NO_VALID_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697206i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697210i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_QUERYOPTION_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697197i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_REDIRECTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697196i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_REDIRECT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697196i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_REDIRECT_TO_DIR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697195i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESERVED_1: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697190i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESERVED_2: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697185i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESERVED_3: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697184i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESERVED_4: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697183i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESERVED_5: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697182i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESOURCE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697211i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_RESULT_DISPATCHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146696704i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_SECURITY_PROBLEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697202i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_TERMINATED_BIND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697192i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_UNKNOWN_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697203i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697199i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_USE_DEFAULT_SETTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697198i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_USE_EXTEND_BINDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697193i32 as _);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146697187i32 as _);
pub struct INET_ZONE_MANAGER_CONSTANTS(i32);
pub struct INTERNETFEATURELIST(i32);
pub struct IPersistMoniker(pub *mut ::core::ffi::c_void);
pub struct ISoftDistExt(pub *mut ::core::ffi::c_void);
pub struct IUriBuilderFactory(pub *mut ::core::ffi::c_void);
pub struct IUriContainer(pub *mut ::core::ffi::c_void);
pub struct IWinInetCacheHints(pub *mut ::core::ffi::c_void);
pub struct IWinInetCacheHints2(pub *mut ::core::ffi::c_void);
pub struct IWinInetFileStream(pub *mut ::core::ffi::c_void);
pub struct IWinInetHttpInfo(pub *mut ::core::ffi::c_void);
pub struct IWinInetHttpTimeouts(pub *mut ::core::ffi::c_void);
pub struct IWinInetInfo(pub *mut ::core::ffi::c_void);
pub struct IWindowForBindingUI(pub *mut ::core::ffi::c_void);
pub struct IWrappedProtocol(pub *mut ::core::ffi::c_void);
pub struct IZoneIdentifier(pub *mut ::core::ffi::c_void);
pub struct IZoneIdentifier2(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MAX_SIZE_SECURITY_ID: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MKSYS_URLMONIKER: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MK_S_ASYNCHRONOUS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262632i32 as _);
pub struct MONIKERPROPERTY(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_DONT_UNESCAPE: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_DONT_USE_CACHE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_ENFORCERESTRICTED: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_FORCE_INTRANET_FLAGS: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_IGNORE_ZONE_MAPPINGS: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_ISFILE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_NOSAVEDFILECHECK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_REQUIRESAVEDFILECHECK: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const MUTZ_RESERVED: u32 = 512u32;
pub struct OIBDG_FLAGS(i32);
pub struct PARSEACTION(i32);
pub struct PI_FLAGS(i32);
pub struct PROTOCOLDATA(i32);
pub struct PROTOCOLFILTERDATA(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PROTOCOL_ARGUMENT(i32);
pub struct PSUACTION(i32);
pub struct PUAF(i32);
pub struct PUAFOUT(i32);
pub struct QUERYOPTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct REMSECURITY_ATTRIBUTES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RemBINDINFO(i32);
pub struct RemFORMATETC(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SECURITY_IE_STATE_GREEN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SECURITY_IE_STATE_RED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_IN_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_PROCESS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_THREAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_THREAD_INTERNET: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_THREAD_INTRANET: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_THREAD_RESTRICTED: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SET_FEATURE_ON_THREAD_TRUSTED: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SOFTDISTINFO(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_ADSTATE_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_ADSTATE_DOWNLOADED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_ADSTATE_INSTALLED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_ADSTATE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_FLAG_USAGE_EMAIL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const SOFTDIST_FLAG_USAGE_PRECACHE: u32 = 2u32;
pub struct SZM_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const S_ASYNCHRONOUS: i32 = 262632i32;
pub struct StartParam(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const UAS_EXACTLEGACY: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_ALLOW_TDC: u32 = 4620u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: u32 = 4612u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_CURR_MAX: u32 = 4620u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: u32 = 4618u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_MAX: u32 = 5119u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_MIN: u32 = 4608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: u32 = 4614u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: u32 = 4610u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: u32 = 4619u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: u32 = 4609u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: u32 = 4616u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: u32 = 4615u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: u32 = 4611u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_RUN: u32 = 4608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: u32 = 4617u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: u32 = 4613u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: u32 = 9986u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: u32 = 9996u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_APEVALUATION: u32 = 8961u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_AUDIO_VIDEO: u32 = 9985u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: u32 = 9988u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: u32 = 9994u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: u32 = 9993u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: u32 = 9992u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: u32 = 9997u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_JSCRIPT_IE: u32 = 5133u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: u32 = 9995u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: u32 = 8960u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: u32 = 9987u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_VBSCRIPT_IE: u32 = 5132u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: u32 = 5128u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_XHR_EVALUATION: u32 = 8962u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: u32 = 9990u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: u32 = 9989u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_AUTHENTICATE_CLIENT: u32 = 6657u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: u32 = 8705u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: u32 = 8704u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: u32 = 8704u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_BEHAVIOR_MIN: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_BEHAVIOR_RUN: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_CHANNEL_SOFTDIST_MAX: u32 = 7935u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_CHANNEL_SOFTDIST_MIN: u32 = 7680u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: u32 = 7685u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_CLIENT_CERT_PROMPT: u32 = 6660u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_COOKIES: u32 = 6658u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_COOKIES_ENABLED: u32 = 6672u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_COOKIES_SESSION: u32 = 6659u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: u32 = 6662u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_COOKIES_THIRD_PARTY: u32 = 6661u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_CREDENTIALS_USE: u32 = 6656u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_CROSS_DOMAIN_DATA: u32 = 5126u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_DOTNET_USERCONTROLS: u32 = 8197u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_DOWNLOAD_CURR_MAX: u32 = 4100u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_DOWNLOAD_MAX: u32 = 4607u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_DOWNLOAD_MIN: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: u32 = 4097u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: u32 = 4100u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: u32 = 8453u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: u32 = 8455u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_DATA_BINDING: u32 = 8454u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: u32 = 8452u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_MIME_SNIFFING: u32 = 8448u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_MIN: u32 = 8448u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: u32 = 8451u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: u32 = 8450u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_FEATURE_ZONE_ELEVATION: u32 = 8449u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: u32 = 5645u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: u32 = 5648u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: u32 = 5647u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_ALLOW_INDEXEDDB: u32 = 5649u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: u32 = 5643u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: u32 = 5646u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_FONT_DOWNLOAD: u32 = 5636u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_INCLUDE_FILE_PATH: u32 = 5642u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_JAVA_RUN: u32 = 5637u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_MAX: u32 = 6143u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_META_REFRESH: u32 = 5640u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_MIN: u32 = 5632u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_MIXED_CONTENT: u32 = 5641u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: u32 = 5644u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: u32 = 5639u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_SUBMIT_FORMS: u32 = 5633u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: u32 = 5634u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_SUBMIT_FORMS_TO: u32 = 5635u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_HTML_USERDATA_SAVE: u32 = 5638u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_CURR_MAX: u32 = 7430u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_MAX: u32 = 7679u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_MIN: u32 = 7424u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: u32 = 7424u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: u32 = 7427u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: u32 = 7430u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: u32 = 7425u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: u32 = 7428u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: u32 = 7426u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: u32 = 7429u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_INPRIVATE_BLOCKING: u32 = 9984u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_JAVA_CURR_MAX: u32 = 7168u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_JAVA_MAX: u32 = 7423u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_JAVA_MIN: u32 = 7168u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_JAVA_PERMISSIONS: u32 = 7168u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_LOOSE_XAML: u32 = 9218u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_LOWRIGHTS: u32 = 9472u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_MIN: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_NETWORK_CURR_MAX: u32 = 6672u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_NETWORK_MAX: u32 = 7167u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_NETWORK_MIN: u32 = 6656u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: u32 = 5131u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_CURR_MAX: u32 = 5133u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_JAVA_USE: u32 = 5122u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_MAX: u32 = 5631u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_MIN: u32 = 5120u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_NAVIGATE: u32 = 5130u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: u32 = 5121u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_PASTE: u32 = 5127u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_RUN: u32 = 5120u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: u32 = 5125u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SCRIPT_XSSFILTER: u32 = 5129u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: u32 = 6161u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_CURR_MAX: u32 = 6162u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: u32 = 6155u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: u32 = 6150u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_EXECUTE_LOWRISK: u32 = 6152u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_EXECUTE_MODRISK: u32 = 6151u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_EXTENSIONSECURITY: u32 = 6156u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_FILE_DOWNLOAD: u32 = 6147u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_INSTALL_DTITEMS: u32 = 6144u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_MAX: u32 = 6655u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_MIN: u32 = 6144u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_MOVE_OR_COPY: u32 = 6146u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_POPUPMGR: u32 = 6153u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_PREVIEW: u32 = 6159u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_REMOTEQUERY: u32 = 6158u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: u32 = 6154u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: u32 = 6157u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_SHARE: u32 = 6160u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_SHELLEXECUTE: u32 = 6150u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_TOCTOU_RISK: u32 = 6162u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_VERB: u32 = 6148u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_SHELL_WEBVIEW_VERB: u32 = 6149u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: u32 = 9216u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_WINFX_SETUP: u32 = 9728u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLACTION_XPS_DOCUMENTS: u32 = 9217u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLMON_OPTION_URL_ENCODING: u32 = 268435460u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLMON_OPTION_USERAGENT: u32 = 268435457u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLMON_OPTION_USERAGENT_REFRESH: u32 = 268435458u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: u32 = 268435464u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: u32 = 268435472u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLOSTRM_GETNEWESTVERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLOSTRM_USECACHEDCOPY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLOSTRM_USECACHEDCOPY_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_ACTIVEX_CHECK_LIST: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_ALLOW: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: u32 = 196608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: u32 = 196608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: u32 = 196608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_DISALLOW: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_DONTCHECKDLGBOX: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_JAVA_CUSTOM: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_JAVA_HIGH: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_JAVA_LOW: u32 = 196608u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_JAVA_MEDIUM: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_JAVA_PROHIBIT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_LOG_ON_ALLOW: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_LOG_ON_DISALLOW: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_MASK_PERMISSIONS: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_NOTIFY_ON_ALLOW: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_NOTIFY_ON_DISALLOW: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLPOLICY_QUERY: u32 = 1u32;
pub struct URLTEMPLATE(i32);
pub struct URLZONE(i32);
pub struct URLZONEREG(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URLZONE_ESC_FLAG: u32 = 256u32;
pub struct URL_ENCODING(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URL_MK_LEGACY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URL_MK_NO_CANONICALIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const URL_MK_UNIFORM: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const UriBuilder_USE_ORIGINAL_FLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_DISPLAY_IDN_HOST: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_DISPLAY_NO_FRAGMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_DISPLAY_NO_PUNYCODE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_HOST_IS_IDN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: u32 = 1u32;
pub struct Uri_HOST_TYPE(i32);
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534u32;
pub struct ZAFLAGS(i32);
pub struct ZONEATTRIBUTES(i32);
