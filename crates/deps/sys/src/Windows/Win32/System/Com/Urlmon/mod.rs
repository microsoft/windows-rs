#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetClassObjectFromURL(rclassid: *const ::windows::runtime::GUID, szcode: super::super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, sztype: super::super::super::Foundation::PWSTR, pbindctx: ::windows::runtime::RawPtr, dwclscontext: super::CLSCTX, pvreserved: *mut ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCombineIUri(pbaseuri: ::windows::runtime::RawPtr, prelativeuri: ::windows::runtime::RawPtr, dwcombineflags: u32, ppcombineduri: *mut ::windows::runtime::RawPtr, dwreserved: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrl(pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCombineUrlEx(pbaseuri: ::windows::runtime::RawPtr, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, ppcombineduri: *mut ::windows::runtime::RawPtr, dwreserved: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetCompareUrl(pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCreateSecurityManager(psp: ::windows::runtime::RawPtr, ppsm: *mut ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetCreateZoneManager(psp: ::windows::runtime::RawPtr, ppzm: *mut ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetProtocolFlags(pwzurl: super::super::super::Foundation::PWSTR, pdwflags: *mut u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetGetSecurityUrl(pwszurl: super::super::super::Foundation::PWSTR, ppwszsecurl: *mut super::super::super::Foundation::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetGetSecurityUrlEx(puri: ::windows::runtime::RawPtr, ppsecuri: *mut ::windows::runtime::RawPtr, psuaction: PSUACTION, dwreserved: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CoInternetIsFeatureEnabledForIUri(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: ::windows::runtime::RawPtr, psecmgr: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureEnabledForUrl(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: super::super::super::Foundation::PWSTR, psecmgr: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetIsFeatureZoneElevationEnabled(szfromurl: super::super::super::Foundation::PWSTR, sztourl: super::super::super::Foundation::PWSTR, psecmgr: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseIUri(piuri: ::windows::runtime::RawPtr, parseaction: PARSEACTION, dwflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetParseUrl(pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetQueryInfo(pwzurl: super::super::super::Foundation::PWSTR, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuffer: *mut u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoInternetSetFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CompareSecurityIds(pbsecurityid1: *const u8, dwlen1: u32, pbsecurityid2: *const u8, dwlen2: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CompatFlagsFromClsid(pclsid: *const ::windows::runtime::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyBindInfo(pcbisrc: *const ::core::mem::ManuallyDrop<super::BINDINFO>, pbidest: *mut ::core::mem::ManuallyDrop<super::BINDINFO>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn CopyStgMedium(pcstgmedsrc: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>, pstgmeddest: *mut ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateAsyncBindCtx(reserved: u32, pbscb: ::windows::runtime::RawPtr, pefetc: ::windows::runtime::RawPtr, ppbc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateAsyncBindCtxEx(pbc: ::windows::runtime::RawPtr, dwoptions: u32, pbscb: ::windows::runtime::RawPtr, penum: ::windows::runtime::RawPtr, ppbc: *mut ::windows::runtime::RawPtr, reserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateFormatEnumerator(cfmtetc: u32, rgfmtetc: *const super::FORMATETC, ppenumfmtetc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMoniker(pmkctx: ::windows::runtime::RawPtr, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateURLMonikerEx(pmkctx: ::windows::runtime::RawPtr, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn CreateURLMonikerEx2(pmkctx: ::windows::runtime::RawPtr, puri: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaultInIEFeature(hwnd: super::super::super::Foundation::HWND, pclassspec: *const super::uCLSSPEC, pquery: *mut super::QUERYCONTEXT, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaType(rgsztypes: super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMediaTypeClass(pbc: ::windows::runtime::RawPtr, sztype: super::super::super::Foundation::PSTR, pclsid: *mut ::windows::runtime::GUID, reserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindMimeFromData(pbc: ::windows::runtime::RawPtr, pwzurl: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, pwzmimeproposed: super::super::super::Foundation::PWSTR, dwmimeflags: u32, ppwzmimeout: *mut super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassFileOrMime(pbc: ::windows::runtime::RawPtr, szfilename: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, szmime: super::super::super::Foundation::PWSTR, dwreserved: u32, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassURL(szurl: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC, ppszcomponentid: *mut super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSoftwareUpdateInfo(szdistunit: super::super::super::Foundation::PWSTR, psdi: *mut SOFTDISTINFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkGoBack(punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkGoForward(punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn HlinkNavigateMoniker(punk: ::windows::runtime::RawPtr, pmktarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkNavigateString(punk: ::windows::runtime::RawPtr, sztarget: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToMoniker(pmktarget: ::windows::runtime::RawPtr, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, param5: ::windows::runtime::RawPtr, grfhlnf: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSimpleNavigateToString(sztarget: super::super::super::Foundation::PWSTR, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, param5: ::windows::runtime::RawPtr, grfhlnf: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IEGetUserPrivateNamespaceName() -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn IEInstallScope(pdwscope: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn IsAsyncMoniker(pmk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledA(pszurl: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLoggingEnabledW(pwszurl: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidURL(pbc: ::windows::runtime::RawPtr, szurl: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MkParseDisplayNameEx(pbc: ::windows::runtime::RawPtr, szdisplayname: super::super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObtainUserAgentString(dwoption: u32, pszuaout: super::super::super::Foundation::PSTR, cbsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RegisterBindStatusCallback(pbc: ::windows::runtime::RawPtr, pbscb: ::windows::runtime::RawPtr, ppbscbprev: *mut ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RegisterFormatEnumerator(pbc: ::windows::runtime::RawPtr, pefetc: ::windows::runtime::RawPtr, reserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypeClass(pbc: ::windows::runtime::RawPtr, ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgclsid: *const ::windows::runtime::GUID, reserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseBindInfo(pbindinfo: *mut ::core::mem::ManuallyDrop<super::BINDINFO>);
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RevokeBindStatusCallback(pbc: ::windows::runtime::RawPtr, pbscb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn RevokeFormatEnumerator(pbc: ::windows::runtime::RawPtr, pefetc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAccessForIEAppContainer(hobject: super::super::super::Foundation::HANDLE, ieobjecttype: IEObjectType, dwaccessmask: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSoftwareUpdateAdvertisementState(szdistunit: super::super::super::Foundation::PWSTR, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileA(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, cchfilename: u32, param4: u32, param5: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToCacheFileW(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, cchfilename: u32, param4: u32, param5: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileA(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: u32, param4: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLDownloadToFileW(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: u32, param4: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamA(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PSTR, param2: *mut ::windows::runtime::RawPtr, param3: u32, param4: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenBlockingStreamW(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: *mut ::windows::runtime::RawPtr, param3: u32, param4: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamA(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PSTR, param2: u32, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenPullStreamW(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamA(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PSTR, param2: u32, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn URLOpenStreamW(param0: ::windows::runtime::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn UrlMkGetSessionOption(dwoption: u32, pbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`*"]
    pub fn UrlMkSetSessionOption(dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_Urlmon`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL;
}
