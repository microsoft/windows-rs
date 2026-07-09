#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn CoGetClassObjectFromURL(rclassid : *const windows_sys::core::GUID, szcode : windows_sys::core::PCWSTR, dwfileversionms : u32, dwfileversionls : u32, sztype : windows_sys::core::PCWSTR, pbindctx : *mut core::ffi::c_void, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetCombineIUri(pbaseuri : *mut core::ffi::c_void, prelativeuri : *mut core::ffi::c_void, dwcombineflags : u32, ppcombineduri : *mut *mut core::ffi::c_void, dwreserved : usize) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetCombineUrl(pwzbaseurl : windows_sys::core::PCWSTR, pwzrelativeurl : windows_sys::core::PCWSTR, dwcombineflags : u32, pszresult : windows_sys::core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetCombineUrlEx(pbaseuri : *mut core::ffi::c_void, pwzrelativeurl : windows_sys::core::PCWSTR, dwcombineflags : u32, ppcombineduri : *mut *mut core::ffi::c_void, dwreserved : usize) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetCompareUrl(pwzurl1 : windows_sys::core::PCWSTR, pwzurl2 : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_servprov")]
windows_link::link!("urlmon.dll" "system" fn CoInternetCreateSecurityManager(psp : *mut core::ffi::c_void, ppsm : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_servprov")]
windows_link::link!("urlmon.dll" "system" fn CoInternetCreateZoneManager(psp : *mut core::ffi::c_void, ppzm : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetGetProtocolFlags(pwzurl : windows_sys::core::PCWSTR, pdwflags : *mut u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetGetSecurityUrl(pwszurl : windows_sys::core::PCWSTR, ppwszsecurl : *mut windows_sys::core::PWSTR, psuaction : PSUACTION, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetGetSecurityUrlEx(puri : *mut core::ffi::c_void, ppsecuri : *mut *mut core::ffi::c_void, psuaction : PSUACTION, dwreserved : usize) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetGetSession(dwsessionmode : u32, ppiinternetsession : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabled(featureentry : INTERNETFEATURELIST, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabledForIUri(featureentry : INTERNETFEATURELIST, dwflags : u32, piuri : *mut core::ffi::c_void, psecmgr : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabledForUrl(featureentry : INTERNETFEATURELIST, dwflags : u32, szurl : windows_sys::core::PCWSTR, psecmgr : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetIsFeatureZoneElevationEnabled(szfromurl : windows_sys::core::PCWSTR, sztourl : windows_sys::core::PCWSTR, psecmgr : *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetParseIUri(piuri : *mut core::ffi::c_void, parseaction : PARSEACTION, dwflags : u32, pwzresult : windows_sys::core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : usize) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetParseUrl(pwzurl : windows_sys::core::PCWSTR, parseaction : PARSEACTION, dwflags : u32, pszresult : windows_sys::core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetQueryInfo(pwzurl : windows_sys::core::PCWSTR, queryoptions : QUERYOPTION, dwqueryflags : u32, pvbuffer : *mut core::ffi::c_void, cbbuffer : u32, pcbbuffer : *mut u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CoInternetSetFeatureEnabled(featureentry : INTERNETFEATURELIST, dwflags : u32, fenable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CompareSecurityIds(pbsecurityid1 : *const u8, dwlen1 : u32, pbsecurityid2 : *const u8, dwlen2 : u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CompatFlagsFromClsid(pclsid : *const windows_sys::core::GUID, pdwcompatflags : *mut u32, pdwmiscstatusflags : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("urlmon.dll" "system" fn CopyBindInfo(pcbisrc : *const BINDINFO, pbidest : *mut BINDINFO) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("urlmon.dll" "system" fn CopyStgMedium(pcstgmedsrc : *const super::objidl::STGMEDIUM, pstgmeddest : *mut super::objidl::STGMEDIUM) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn CreateAsyncBindCtx(reserved : u32, pbscb : *mut core::ffi::c_void, pefetc : *mut core::ffi::c_void, ppbc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn CreateAsyncBindCtxEx(pbc : *mut core::ffi::c_void, dwoptions : u32, pbscb : *mut core::ffi::c_void, penum : *mut core::ffi::c_void, ppbc : *mut *mut core::ffi::c_void, reserved : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
windows_link::link!("urlmon.dll" "system" fn CreateFormatEnumerator(cfmtetc : u32, rgfmtetc : *const super::objidl::FORMATETC, ppenumfmtetc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateIUriBuilder(piuri : *mut core::ffi::c_void, dwflags : u32, dwreserved : usize, ppiuribuilder : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn CreateURLMoniker(pmkctx : *mut core::ffi::c_void, szurl : windows_sys::core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn CreateURLMonikerEx(pmkctx : *mut core::ffi::c_void, szurl : windows_sys::core::PCWSTR, ppmk : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn CreateURLMonikerEx2(pmkctx : *mut core::ffi::c_void, puri : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateUri(pwzuri : windows_sys::core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateUriFromMultiByteString(pszansiinputuri : windows_sys::core::PCSTR, dwencodingflags : u32, dwcodepage : u32, dwcreateflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn CreateUriWithFragment(pwzuri : windows_sys::core::PCWSTR, pwzfragment : windows_sys::core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("urlmon.dll" "system" fn FaultInIEFeature(hwnd : super::windef::HWND, pclassspec : *const super::wtypes::uCLSSPEC, pquery : *mut super::wtypes::QUERYCONTEXT, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("urlmon.dll" "system" fn FindMediaType(rgsztypes : windows_sys::core::PCSTR, rgcftypes : *mut super::wtypes::CLIPFORMAT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn FindMediaTypeClass(pbc : *mut core::ffi::c_void, sztype : windows_sys::core::PCSTR, pclsid : *mut windows_sys::core::GUID, reserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn FindMimeFromData(pbc : *mut core::ffi::c_void, pwzurl : windows_sys::core::PCWSTR, pbuffer : *const core::ffi::c_void, cbsize : u32, pwzmimeproposed : windows_sys::core::PCWSTR, dwmimeflags : u32, ppwzmimeout : *mut windows_sys::core::PWSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn GetClassFileOrMime(pbc : *mut core::ffi::c_void, szfilename : windows_sys::core::PCWSTR, pbuffer : *const core::ffi::c_void, cbsize : u32, szmime : windows_sys::core::PCWSTR, dwreserved : u32, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn GetClassURL(szurl : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("urlmon.dll" "system" fn GetComponentIDFromCLSSPEC(pclassspec : *const super::wtypes::uCLSSPEC, ppszcomponentid : *mut windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn GetSoftwareUpdateInfo(szdistunit : windows_sys::core::PCWSTR, psdi : *mut SOFTDISTINFO) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn HlinkGoBack(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn HlinkGoForward(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn HlinkNavigateMoniker(punk : *mut core::ffi::c_void, pmktarget : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn HlinkNavigateString(punk : *mut core::ffi::c_void, sztarget : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn HlinkSimpleNavigateToMoniker(pmktarget : *mut core::ffi::c_void, szlocation : windows_sys::core::PCWSTR, sztargetframename : windows_sys::core::PCWSTR, punk : *mut core::ffi::c_void, pbc : *mut core::ffi::c_void, param5 : *mut core::ffi::c_void, grfhlnf : u32, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn HlinkSimpleNavigateToString(sztarget : windows_sys::core::PCWSTR, szlocation : windows_sys::core::PCWSTR, sztargetframename : windows_sys::core::PCWSTR, punk : *mut core::ffi::c_void, pbc : *mut core::ffi::c_void, param5 : *mut core::ffi::c_void, grfhlnf : u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn IEGetUserPrivateNamespaceName() -> windows_sys::core::PWSTR);
windows_link::link!("urlmon.dll" "system" fn IEInstallScope(pdwscope : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn IsAsyncMoniker(pmk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn IsLoggingEnabledA(pszurl : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("urlmon.dll" "system" fn IsLoggingEnabledW(pwszurl : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn IsValidURL(pbc : *mut core::ffi::c_void, szurl : windows_sys::core::PCWSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn MkParseDisplayNameEx(pbc : *mut core::ffi::c_void, szdisplayname : windows_sys::core::PCWSTR, pcheaten : *mut u32, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn ObtainUserAgentString(dwoption : u32, pszuaout : windows_sys::core::PSTR, cbsize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn RegisterBindStatusCallback(pbc : *mut core::ffi::c_void, pbscb : *mut core::ffi::c_void, ppbscbprev : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn RegisterFormatEnumerator(pbc : *mut core::ffi::c_void, pefetc : *mut core::ffi::c_void, reserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn RegisterMediaTypeClass(pbc : *mut core::ffi::c_void, ctypes : u32, rgsztypes : *const windows_sys::core::PCSTR, rgclsid : *const windows_sys::core::GUID, reserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("urlmon.dll" "system" fn RegisterMediaTypes(ctypes : u32, rgsztypes : *const windows_sys::core::PCSTR, rgcftypes : *mut super::wtypes::CLIPFORMAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("urlmon.dll" "system" fn ReleaseBindInfo(pbindinfo : *mut BINDINFO));
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn RevokeBindStatusCallback(pbc : *mut core::ffi::c_void, pbscb : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("urlmon.dll" "system" fn RevokeFormatEnumerator(pbc : *mut core::ffi::c_void, pefetc : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("urlmon.dll" "system" fn SetAccessForIEAppContainer(hobject : super::winnt::HANDLE, ieobjecttype : IEObjectType, dwaccessmask : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn SetSoftwareUpdateAdvertisementState(szdistunit : windows_sys::core::PCWSTR, dwadstate : u32, dwadvertisedversionms : u32, dwadvertisedversionls : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLDownloadToCacheFileA(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCSTR, param2 : windows_sys::core::PSTR, cchfilename : u32, param4 : u32, param5 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLDownloadToCacheFileW(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCWSTR, param2 : windows_sys::core::PWSTR, cchfilename : u32, param4 : u32, param5 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLDownloadToFileA(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCSTR, param2 : windows_sys::core::PCSTR, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLDownloadToFileW(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCWSTR, param2 : windows_sys::core::PCWSTR, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("urlmon.dll" "system" fn URLOpenBlockingStreamA(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCSTR, param2 : *mut *mut core::ffi::c_void, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("urlmon.dll" "system" fn URLOpenBlockingStreamW(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCWSTR, param2 : *mut *mut core::ffi::c_void, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLOpenPullStreamA(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLOpenPullStreamW(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCWSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLOpenStreamA(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn URLOpenStreamW(param0 : *mut core::ffi::c_void, param1 : windows_sys::core::PCWSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn UrlMkGetSessionOption(dwoption : u32, pbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbufferlengthout : *mut u32, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("urlmon.dll" "system" fn UrlMkSetSessionOption(dwoption : u32, pbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("urlmon.dll" "system" fn WriteHitLogging(lplogginginfo : *const HIT_LOGGING_INFO) -> windows_sys::core::BOOL);
pub type AUTHENTICATEF = i32;
pub const AUTHENTICATEF_BASIC: AUTHENTICATEF = 2;
pub const AUTHENTICATEF_HTTP: AUTHENTICATEF = 4;
pub const AUTHENTICATEF_PROXY: AUTHENTICATEF = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
pub type BINDF = i32;
pub type BINDF2 = i32;
pub const BINDF2_ALLOW_PROXY_CRED_PROMPT: BINDF2 = 256;
pub const BINDF2_DISABLEAUTOCOOKIEHANDLING: BINDF2 = 2;
pub const BINDF2_DISABLEBASICOVERHTTP: BINDF2 = 1;
pub const BINDF2_DISABLE_HTTP_REDIRECT_CACHING: BINDF2 = 64;
pub const BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID: BINDF2 = 8;
pub const BINDF2_KEEP_CALLBACK_MODULE_LOADED: BINDF2 = 128;
pub const BINDF2_READ_DATA_GREATER_THAN_4GB: BINDF2 = 4;
pub const BINDF2_RESERVED_1: BINDF2 = -2147483648;
pub const BINDF2_RESERVED_10: BINDF2 = 65536;
pub const BINDF2_RESERVED_11: BINDF2 = 32768;
pub const BINDF2_RESERVED_12: BINDF2 = 16384;
pub const BINDF2_RESERVED_13: BINDF2 = 8192;
pub const BINDF2_RESERVED_14: BINDF2 = 4096;
pub const BINDF2_RESERVED_15: BINDF2 = 2048;
pub const BINDF2_RESERVED_16: BINDF2 = 1024;
pub const BINDF2_RESERVED_17: BINDF2 = 512;
pub const BINDF2_RESERVED_2: BINDF2 = 1073741824;
pub const BINDF2_RESERVED_3: BINDF2 = 536870912;
pub const BINDF2_RESERVED_4: BINDF2 = 268435456;
pub const BINDF2_RESERVED_5: BINDF2 = 134217728;
pub const BINDF2_RESERVED_6: BINDF2 = 67108864;
pub const BINDF2_RESERVED_7: BINDF2 = 33554432;
pub const BINDF2_RESERVED_8: BINDF2 = 16777216;
pub const BINDF2_RESERVED_9: BINDF2 = 8388608;
pub const BINDF2_RESERVED_A: BINDF2 = 4194304;
pub const BINDF2_RESERVED_B: BINDF2 = 2097152;
pub const BINDF2_RESERVED_C: BINDF2 = 1048576;
pub const BINDF2_RESERVED_D: BINDF2 = 524288;
pub const BINDF2_RESERVED_E: BINDF2 = 262144;
pub const BINDF2_RESERVED_F: BINDF2 = 131072;
pub const BINDF2_SETDOWNLOADMODE: BINDF2 = 32;
pub const BINDF_ASYNCHRONOUS: BINDF = 1;
pub const BINDF_ASYNCSTORAGE: BINDF = 2;
pub const BINDF_DIRECT_READ: BINDF = 131072;
pub const BINDF_DONTPUTINCACHE: u32 = 32;
pub const BINDF_DONTUSECACHE: u32 = 16;
pub const BINDF_ENFORCERESTRICTED: BINDF = 8388608;
pub const BINDF_FORMS_SUBMIT: BINDF = 262144;
pub const BINDF_FREE_THREADED: BINDF = 65536;
pub const BINDF_FROMURLMON: BINDF = 1048576;
pub const BINDF_FWD_BACK: BINDF = 2097152;
pub const BINDF_GETCLASSOBJECT: BINDF = 16384;
pub const BINDF_GETFROMCACHE_IF_NET_FAIL: BINDF = 524288;
pub const BINDF_GETNEWESTVERSION: BINDF = 16;
pub const BINDF_HYPERLINK: BINDF = 1024;
pub const BINDF_IGNORESECURITYPROBLEM: BINDF = 256;
pub const BINDF_NEEDFILE: BINDF = 64;
pub const BINDF_NOCOPYDATA: u32 = 128;
pub const BINDF_NOPROGRESSIVERENDERING: BINDF = 4;
pub const BINDF_NOWRITECACHE: BINDF = 32;
pub const BINDF_NO_UI: BINDF = 2048;
pub const BINDF_OFFLINEOPERATION: BINDF = 8;
pub const BINDF_PRAGMA_NO_CACHE: BINDF = 8192;
pub const BINDF_PREFERDEFAULTHANDLER: BINDF = 4194304;
pub const BINDF_PULLDATA: BINDF = 128;
pub const BINDF_RESERVED_1: BINDF = 32768;
pub const BINDF_RESERVED_2: BINDF = -2147483648;
pub const BINDF_RESERVED_3: BINDF = 16777216;
pub const BINDF_RESERVED_4: BINDF = 33554432;
pub const BINDF_RESERVED_5: BINDF = 67108864;
pub const BINDF_RESERVED_6: BINDF = 134217728;
pub const BINDF_RESERVED_7: BINDF = 1073741824;
pub const BINDF_RESERVED_8: BINDF = 536870912;
pub const BINDF_RESYNCHRONIZE: BINDF = 512;
pub const BINDF_SILENTOPERATION: BINDF = 4096;
pub type BINDHANDLETYPES = i32;
pub const BINDHANDLETYPES_APPCACHE: BINDHANDLETYPES = 0;
pub const BINDHANDLETYPES_COUNT: BINDHANDLETYPES = 2;
pub const BINDHANDLETYPES_DEPENDENCY: BINDHANDLETYPES = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_sys::core::PWSTR,
    pub stgmedData: super::objidl::STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_sys::core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::minwinbase::SECURITY_ATTRIBUTES,
    pub iid: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl Default for BINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BINDINFOF = i32;
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = 2;
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = 1;
pub type BINDINFO_OPTIONS = i32;
pub const BINDINFO_OPTIONS_ALLOWCONNECTDATA: BINDINFO_OPTIONS = 536870912;
pub const BINDINFO_OPTIONS_BINDTOOBJECT: BINDINFO_OPTIONS = 1048576;
pub const BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS: BINDINFO_OPTIONS = 1073741824;
pub const BINDINFO_OPTIONS_DISABLE_UTF8: BINDINFO_OPTIONS = 262144;
pub const BINDINFO_OPTIONS_ENABLE_UTF8: BINDINFO_OPTIONS = 131072;
pub const BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS: BINDINFO_OPTIONS = 16777216;
pub const BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN: BINDINFO_OPTIONS = 4194304;
pub const BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE: BINDINFO_OPTIONS = 33554432;
pub const BINDINFO_OPTIONS_SECURITYOPTOUT: BINDINFO_OPTIONS = 2097152;
pub const BINDINFO_OPTIONS_SHDOCVW_NAVIGATE: BINDINFO_OPTIONS = -2147483648;
pub const BINDINFO_OPTIONS_USEBINDSTRINGCREDS: BINDINFO_OPTIONS = 8388608;
pub const BINDINFO_OPTIONS_USE_IE_ENCODING: BINDINFO_OPTIONS = 524288;
pub const BINDINFO_OPTIONS_WININETFLAG: BINDINFO_OPTIONS = 65536;
pub const BINDINFO_WPC_DOWNLOADBLOCKED: BINDINFO_OPTIONS = 134217728;
pub const BINDINFO_WPC_LOGGING_ENABLED: BINDINFO_OPTIONS = 268435456;
pub type BINDSTATUS = i32;
pub const BINDSTATUS_64BIT_PROGRESS: BINDSTATUS = 56;
pub const BINDSTATUS_ACCEPTRANGES: BINDSTATUS = 33;
pub const BINDSTATUS_BEGINDOWNLOADCOMPONENTS: BINDSTATUS = 7;
pub const BINDSTATUS_BEGINDOWNLOADDATA: BINDSTATUS = 4;
pub const BINDSTATUS_BEGINSYNCOPERATION: BINDSTATUS = 15;
pub const BINDSTATUS_BEGINUPLOADDATA: BINDSTATUS = 17;
pub const BINDSTATUS_CACHECONTROL: BINDSTATUS = 48;
pub const BINDSTATUS_CACHEFILENAMEAVAILABLE: BINDSTATUS = 14;
pub const BINDSTATUS_CLASSIDAVAILABLE: BINDSTATUS = 12;
pub const BINDSTATUS_CLASSINSTALLLOCATION: BINDSTATUS = 23;
pub const BINDSTATUS_CLSIDCANINSTANTIATE: BINDSTATUS = 28;
pub const BINDSTATUS_COMPACT_POLICY_RECEIVED: BINDSTATUS = 35;
pub const BINDSTATUS_CONNECTING: BINDSTATUS = 2;
pub const BINDSTATUS_CONTENTDISPOSITIONATTACH: BINDSTATUS = 26;
pub const BINDSTATUS_CONTENTDISPOSITIONFILENAME: BINDSTATUS = 49;
pub const BINDSTATUS_COOKIE_SENT: BINDSTATUS = 34;
pub const BINDSTATUS_COOKIE_STATE_ACCEPT: BINDSTATUS = 38;
pub const BINDSTATUS_COOKIE_STATE_DOWNGRADE: BINDSTATUS = 42;
pub const BINDSTATUS_COOKIE_STATE_LEASH: BINDSTATUS = 41;
pub const BINDSTATUS_COOKIE_STATE_PROMPT: BINDSTATUS = 40;
pub const BINDSTATUS_COOKIE_STATE_REJECT: BINDSTATUS = 39;
pub const BINDSTATUS_COOKIE_STATE_UNKNOWN: BINDSTATUS = 37;
pub const BINDSTATUS_COOKIE_SUPPRESSED: BINDSTATUS = 36;
pub const BINDSTATUS_DECODING: BINDSTATUS = 24;
pub const BINDSTATUS_DIRECTBIND: BINDSTATUS = 30;
pub const BINDSTATUS_DISPLAYNAMEAVAILABLE: BINDSTATUS = 52;
pub const BINDSTATUS_DOWNLOADINGDATA: BINDSTATUS = 5;
pub const BINDSTATUS_ENCODING: BINDSTATUS = 21;
pub const BINDSTATUS_ENDDOWNLOADCOMPONENTS: BINDSTATUS = 9;
pub const BINDSTATUS_ENDDOWNLOADDATA: BINDSTATUS = 6;
pub const BINDSTATUS_ENDSYNCOPERATION: BINDSTATUS = 16;
pub const BINDSTATUS_ENDUPLOADDATA: BINDSTATUS = 19;
pub const BINDSTATUS_FILTERREPORTMIMETYPE: BINDSTATUS = 27;
pub const BINDSTATUS_FINDINGRESOURCE: BINDSTATUS = 1;
pub const BINDSTATUS_INSTALLINGCOMPONENTS: BINDSTATUS = 8;
pub const BINDSTATUS_IUNKNOWNAVAILABLE: BINDSTATUS = 29;
pub const BINDSTATUS_LAST: BINDSTATUS = 56;
pub const BINDSTATUS_LAST_PRIVATE: BINDSTATUS = 77;
pub const BINDSTATUS_LOADINGMIMEHANDLER: BINDSTATUS = 25;
pub const BINDSTATUS_MIMETEXTPLAINMISMATCH: BINDSTATUS = 50;
pub const BINDSTATUS_MIMETYPEAVAILABLE: BINDSTATUS = 13;
pub const BINDSTATUS_P3P_HEADER: BINDSTATUS = 44;
pub const BINDSTATUS_PERSISTENT_COOKIE_RECEIVED: BINDSTATUS = 46;
pub const BINDSTATUS_POLICY_HREF: BINDSTATUS = 43;
pub const BINDSTATUS_PROTOCOLCLASSID: BINDSTATUS = 20;
pub const BINDSTATUS_PROXYDETECTING: BINDSTATUS = 32;
pub const BINDSTATUS_PUBLISHERAVAILABLE: BINDSTATUS = 51;
pub const BINDSTATUS_RAWMIMETYPE: BINDSTATUS = 31;
pub const BINDSTATUS_REDIRECTING: BINDSTATUS = 3;
pub const BINDSTATUS_RESERVED_0: BINDSTATUS = 57;
pub const BINDSTATUS_RESERVED_1: BINDSTATUS = 58;
pub const BINDSTATUS_RESERVED_10: BINDSTATUS = 73;
pub const BINDSTATUS_RESERVED_11: BINDSTATUS = 74;
pub const BINDSTATUS_RESERVED_12: BINDSTATUS = 75;
pub const BINDSTATUS_RESERVED_13: BINDSTATUS = 76;
pub const BINDSTATUS_RESERVED_14: BINDSTATUS = 77;
pub const BINDSTATUS_RESERVED_2: BINDSTATUS = 59;
pub const BINDSTATUS_RESERVED_3: BINDSTATUS = 60;
pub const BINDSTATUS_RESERVED_4: BINDSTATUS = 61;
pub const BINDSTATUS_RESERVED_5: BINDSTATUS = 62;
pub const BINDSTATUS_RESERVED_6: BINDSTATUS = 63;
pub const BINDSTATUS_RESERVED_7: BINDSTATUS = 64;
pub const BINDSTATUS_RESERVED_8: BINDSTATUS = 65;
pub const BINDSTATUS_RESERVED_9: BINDSTATUS = 66;
pub const BINDSTATUS_RESERVED_A: BINDSTATUS = 67;
pub const BINDSTATUS_RESERVED_B: BINDSTATUS = 68;
pub const BINDSTATUS_RESERVED_C: BINDSTATUS = 69;
pub const BINDSTATUS_RESERVED_D: BINDSTATUS = 70;
pub const BINDSTATUS_RESERVED_E: BINDSTATUS = 71;
pub const BINDSTATUS_RESERVED_F: BINDSTATUS = 72;
pub const BINDSTATUS_SENDINGREQUEST: BINDSTATUS = 11;
pub const BINDSTATUS_SERVER_MIMETYPEAVAILABLE: BINDSTATUS = 54;
pub const BINDSTATUS_SESSION_COOKIES_ALLOWED: BINDSTATUS = 47;
pub const BINDSTATUS_SESSION_COOKIE_RECEIVED: BINDSTATUS = 45;
pub const BINDSTATUS_SNIFFED_CLASSIDAVAILABLE: BINDSTATUS = 55;
pub const BINDSTATUS_SSLUX_NAVBLOCKED: BINDSTATUS = 53;
pub const BINDSTATUS_UPLOADINGDATA: BINDSTATUS = 18;
pub const BINDSTATUS_USINGCACHEDCOPY: BINDSTATUS = 10;
pub const BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE: BINDSTATUS = 22;
pub type BINDSTRING = i32;
pub const BINDSTRING_ACCEPT_ENCODINGS: BINDSTRING = 11;
pub const BINDSTRING_ACCEPT_MIMES: BINDSTRING = 2;
pub const BINDSTRING_DOC_URL: BINDSTRING = 25;
pub const BINDSTRING_DOWNLOADPATH: BINDSTRING = 19;
pub const BINDSTRING_ENTERPRISE_ID: BINDSTRING = 24;
pub const BINDSTRING_EXTRA_URL: BINDSTRING = 3;
pub const BINDSTRING_FLAG_BIND_TO_OBJECT: BINDSTRING = 16;
pub const BINDSTRING_HEADERS: BINDSTRING = 1;
pub const BINDSTRING_IID: BINDSTRING = 15;
pub const BINDSTRING_INITIAL_FILENAME: BINDSTRING = 21;
pub const BINDSTRING_LANGUAGE: BINDSTRING = 4;
pub const BINDSTRING_OS: BINDSTRING = 9;
pub const BINDSTRING_PASSWORD: BINDSTRING = 6;
pub const BINDSTRING_POST_COOKIE: BINDSTRING = 12;
pub const BINDSTRING_POST_DATA_MIME: BINDSTRING = 13;
pub const BINDSTRING_PROXY_PASSWORD: BINDSTRING = 23;
pub const BINDSTRING_PROXY_USERNAME: BINDSTRING = 22;
pub const BINDSTRING_PTR_BIND_CONTEXT: BINDSTRING = 17;
pub const BINDSTRING_ROOTDOC_URL: BINDSTRING = 20;
pub const BINDSTRING_SAMESITE_COOKIE_LEVEL: BINDSTRING = 26;
pub const BINDSTRING_UA_COLOR: BINDSTRING = 8;
pub const BINDSTRING_UA_PIXELS: BINDSTRING = 7;
pub const BINDSTRING_URL: BINDSTRING = 14;
pub const BINDSTRING_USERNAME: BINDSTRING = 5;
pub const BINDSTRING_USER_AGENT: BINDSTRING = 10;
pub const BINDSTRING_XDR_ORIGIN: BINDSTRING = 18;
pub type BINDVERB = i32;
pub const BINDVERB_CUSTOM: BINDVERB = 3;
pub const BINDVERB_GET: BINDVERB = 0;
pub const BINDVERB_POST: BINDVERB = 1;
pub const BINDVERB_PUT: BINDVERB = 2;
pub const BINDVERB_RESERVED1: BINDVERB = 4;
pub type BSCF = i32;
pub const BSCF_64BITLENGTHDOWNLOAD: BSCF = 64;
pub const BSCF_AVAILABLEDATASIZEUNKNOWN: BSCF = 16;
pub const BSCF_DATAFULLYAVAILABLE: BSCF = 8;
pub const BSCF_FIRSTDATANOTIFICATION: BSCF = 1;
pub const BSCF_INTERMEDIATEDATANOTIFICATION: BSCF = 2;
pub const BSCF_LASTDATANOTIFICATION: BSCF = 4;
pub const BSCF_SKIPDRAINDATAFORFILEURLS: BSCF = 32;
pub const CFSTR_MIME_NULL: u32 = 0;
pub const CF_NULL: u32 = 0;
pub const CIP_ACCESS_DENIED: CIP_STATUS = 1;
pub const CIP_DISK_FULL: CIP_STATUS = 0;
pub const CIP_EXE_SELF_REGISTERATION_TIMEOUT: CIP_STATUS = 6;
pub const CIP_NAME_CONFLICT: CIP_STATUS = 4;
pub const CIP_NEED_REBOOT: CIP_STATUS = 8;
pub const CIP_NEED_REBOOT_UI_PERMISSION: CIP_STATUS = 9;
pub const CIP_NEWER_VERSION_EXISTS: CIP_STATUS = 2;
pub const CIP_OLDER_VERSION_EXISTS: CIP_STATUS = 3;
pub type CIP_STATUS = i32;
pub const CIP_TRUST_VERIFICATION_COMPONENT_MISSING: CIP_STATUS = 5;
pub const CIP_UNSAFE_TO_ABORT: CIP_STATUS = 7;
pub const CLASSIDPROP: MONIKERPROPERTY = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CODEBASEHOLD {
    pub cbSize: u32,
    pub szDistUnit: windows_sys::core::PWSTR,
    pub szCodeBase: windows_sys::core::PWSTR,
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
    pub dwStyle: u32,
}
impl Default for CODEBASEHOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONFIRMSAFETY {
    pub clsid: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub dwFlags: u32,
}
impl Default for CONFIRMSAFETY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DATAINFO {
    pub ulTotalSize: u32,
    pub ulavrPacketSize: u32,
    pub ulConnectSpeed: u32,
    pub ulProcessorSpeed: u32,
}
pub const FEATURE_ADDON_MANAGEMENT: INTERNETFEATURELIST = 13;
pub const FEATURE_BEHAVIORS: INTERNETFEATURELIST = 6;
pub const FEATURE_BLOCK_INPUT_PROMPTS: INTERNETFEATURELIST = 27;
pub const FEATURE_DISABLE_LEGACY_COMPRESSION: INTERNETFEATURELIST = 22;
pub const FEATURE_DISABLE_MK_PROTOCOL: INTERNETFEATURELIST = 7;
pub const FEATURE_DISABLE_NAVIGATION_SOUNDS: INTERNETFEATURELIST = 21;
pub const FEATURE_DISABLE_TELNET_PROTOCOL: INTERNETFEATURELIST = 25;
pub const FEATURE_ENTRY_COUNT: INTERNETFEATURELIST = 28;
pub const FEATURE_FEEDS: INTERNETFEATURELIST = 26;
pub const FEATURE_FORCE_ADDR_AND_STATUS: INTERNETFEATURELIST = 23;
pub const FEATURE_GET_URL_DOM_FILEPATH_UNENCODED: INTERNETFEATURELIST = 18;
pub const FEATURE_HTTP_USERNAME_PASSWORD_DISABLE: INTERNETFEATURELIST = 15;
pub const FEATURE_LOCALMACHINE_LOCKDOWN: INTERNETFEATURELIST = 8;
pub const FEATURE_MIME_HANDLING: INTERNETFEATURELIST = 2;
pub const FEATURE_MIME_SNIFFING: INTERNETFEATURELIST = 3;
pub const FEATURE_OBJECT_CACHING: INTERNETFEATURELIST = 0;
pub const FEATURE_PROTOCOL_LOCKDOWN: INTERNETFEATURELIST = 14;
pub const FEATURE_RESTRICT_ACTIVEXINSTALL: INTERNETFEATURELIST = 10;
pub const FEATURE_RESTRICT_FILEDOWNLOAD: INTERNETFEATURELIST = 12;
pub const FEATURE_SAFE_BINDTOOBJECT: INTERNETFEATURELIST = 16;
pub const FEATURE_SECURITYBAND: INTERNETFEATURELIST = 9;
pub const FEATURE_SSLUX: INTERNETFEATURELIST = 20;
pub const FEATURE_TABBED_BROWSING: INTERNETFEATURELIST = 19;
pub const FEATURE_UNC_SAVEDFILECHECK: INTERNETFEATURELIST = 17;
pub const FEATURE_VALIDATE_NAVIGATE_URL: INTERNETFEATURELIST = 11;
pub const FEATURE_WEBOC_POPUPMANAGEMENT: INTERNETFEATURELIST = 5;
pub const FEATURE_WINDOW_RESTRICTIONS: INTERNETFEATURELIST = 4;
pub const FEATURE_XMLHTTP: INTERNETFEATURELIST = 24;
pub const FEATURE_ZONE_ELEVATION: INTERNETFEATURELIST = 1;
pub const FIEF_FLAG_FORCE_JITUI: u32 = 1;
pub const FIEF_FLAG_PEEK: u32 = 2;
pub const FIEF_FLAG_RESERVED_0: u32 = 8;
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: u32 = 4;
pub const FMFD_DEFAULT: u32 = 0;
pub const FMFD_ENABLEMIMESNIFFING: u32 = 2;
pub const FMFD_IGNOREMIMETEXTPLAIN: u32 = 4;
pub const FMFD_RESERVED_1: u32 = 64;
pub const FMFD_RESERVED_2: u32 = 128;
pub const FMFD_RESPECTTEXTPLAIN: u32 = 16;
pub const FMFD_RETURNUPDATEDIMGMIMES: u32 = 32;
pub const FMFD_SERVERMIME: u32 = 8;
pub const FMFD_URLASFILENAME: u32 = 1;
pub const GET_FEATURE_FROM_PROCESS: u32 = 2;
pub const GET_FEATURE_FROM_REGISTRY: u32 = 4;
pub const GET_FEATURE_FROM_THREAD: u32 = 1;
pub const GET_FEATURE_FROM_THREAD_INTERNET: u32 = 64;
pub const GET_FEATURE_FROM_THREAD_INTRANET: u32 = 16;
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: u32 = 8;
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: u32 = 128;
pub const GET_FEATURE_FROM_THREAD_TRUSTED: u32 = 32;
#[repr(C)]
#[cfg(feature = "Win32_minwinbase")]
#[derive(Clone, Copy)]
pub struct HIT_LOGGING_INFO {
    pub dwStructSize: u32,
    pub lpszLoggedUrlName: windows_sys::core::PSTR,
    pub StartTime: super::minwinbase::SYSTEMTIME,
    pub EndTime: super::minwinbase::SYSTEMTIME,
    pub lpszExtendedInfo: windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for HIT_LOGGING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IEObjectType = i32;
pub const IE_EPM_OBJECT_EVENT: IEObjectType = 0;
pub const IE_EPM_OBJECT_FILE: IEObjectType = 5;
pub const IE_EPM_OBJECT_MUTEX: IEObjectType = 1;
pub const IE_EPM_OBJECT_NAMED_PIPE: IEObjectType = 6;
pub const IE_EPM_OBJECT_REGISTRY: IEObjectType = 7;
pub const IE_EPM_OBJECT_SEMAPHORE: IEObjectType = 2;
pub const IE_EPM_OBJECT_SHARED_MEMORY: IEObjectType = 3;
pub const IE_EPM_OBJECT_WAITABLE_TIMER: IEObjectType = 4;
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: windows_sys::core::HRESULT = 0x800C0506_u32 as _;
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: windows_sys::core::HRESULT = 0x800C0505_u32 as _;
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: windows_sys::core::HRESULT = 0x800C001B_u32 as _;
pub const INET_E_CANNOT_LOCK_REQUEST: windows_sys::core::HRESULT = 0x800C0016_u32 as _;
pub const INET_E_CANNOT_REPLACE_SFP_FILE: windows_sys::core::HRESULT = 0x800C0300_u32 as _;
pub const INET_E_CODE_DOWNLOAD_DECLINED: windows_sys::core::HRESULT = 0x800C0100_u32 as _;
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: windows_sys::core::HRESULT = 0x800C0504_u32 as _;
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: windows_sys::core::HRESULT = 0x800C0507_u32 as _;
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: windows_sys::core::HRESULT = 0x800C0500_u32 as _;
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: windows_sys::core::HRESULT = 0x800C0502_u32 as _;
pub const INET_E_CODE_INSTALL_SUPPRESSED: windows_sys::core::HRESULT = 0x800C0400_u32 as _;
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199;
pub const INET_E_DOMINJECTIONVALIDATION: windows_sys::core::HRESULT = 0x800C001C_u32 as _;
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: windows_sys::core::HRESULT = 0x800C0508_u32 as _;
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: windows_sys::core::HRESULT = 0x800C0501_u32 as _;
pub const INET_E_ERROR_FIRST: windows_sys::core::HRESULT = 0x800C0002_u32 as _;
pub const INET_E_ERROR_LAST: i32 = -2146695928;
pub const INET_E_FORBIDFRAMING: windows_sys::core::HRESULT = 0x800C0503_u32 as _;
pub const INET_E_HSTS_CERTIFICATE_ERROR: windows_sys::core::HRESULT = 0x800C001E_u32 as _;
pub const INET_E_QUERYOPTION_UNKNOWN: windows_sys::core::HRESULT = 0x800C0013_u32 as _;
pub const INET_E_REDIRECTING: windows_sys::core::HRESULT = 0x800C0014_u32 as _;
pub const INET_E_RESERVED_1: windows_sys::core::HRESULT = 0x800C001A_u32 as _;
pub const INET_E_RESERVED_2: windows_sys::core::HRESULT = 0x800C001F_u32 as _;
pub const INET_E_RESERVED_3: windows_sys::core::HRESULT = 0x800C0020_u32 as _;
pub const INET_E_RESERVED_4: windows_sys::core::HRESULT = 0x800C0021_u32 as _;
pub const INET_E_RESERVED_5: windows_sys::core::HRESULT = 0x800C0022_u32 as _;
pub const INET_E_RESULT_DISPATCHED: windows_sys::core::HRESULT = 0x800C0200_u32 as _;
pub const INET_E_TERMINATED_BIND: windows_sys::core::HRESULT = 0x800C0018_u32 as _;
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: windows_sys::core::HRESULT = 0x800C0011_u32 as _;
pub const INET_E_USE_DEFAULT_SETTING: windows_sys::core::HRESULT = 0x800C0012_u32 as _;
pub const INET_E_USE_EXTEND_BINDING: windows_sys::core::HRESULT = 0x800C0017_u32 as _;
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: windows_sys::core::HRESULT = 0x800C001D_u32 as _;
pub type INTERNETFEATURELIST = i32;
pub type LPCODEBASEHOLD = *mut CODEBASEHOLD;
#[cfg(feature = "Win32_minwinbase")]
pub type LPHIT_LOGGING_INFO = *mut HIT_LOGGING_INFO;
pub type LPPROTOCOL_ARGUMENT = *mut PROTOCOL_ARGUMENT;
pub type LPREMFORMATETC = *mut RemFORMATETC;
pub type LPREMSECURITY_ATTRIBUTES = *mut REMSECURITY_ATTRIBUTES;
pub type LPSOFTDISTINFO = *mut SOFTDISTINFO;
pub type LPZONEATTRIBUTES = *mut ZONEATTRIBUTES;
pub const MAX_SIZE_SECURITY_ID: u32 = 512;
pub const MAX_ZONE_DESCRIPTION: i32 = 200;
pub const MAX_ZONE_PATH: i32 = 260;
pub const MIMETYPEPROP: MONIKERPROPERTY = 0;
pub const MKSYS_URLMONIKER: u32 = 6;
pub const MK_S_ASYNCHRONOUS: windows_sys::core::HRESULT = 0x401E8_u32 as _;
pub type MONIKERPROPERTY = i32;
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: u32 = 128;
pub const MUTZ_DONT_UNESCAPE: u32 = 2048;
pub const MUTZ_DONT_USE_CACHE: u32 = 4096;
pub const MUTZ_ENFORCERESTRICTED: u32 = 256;
pub const MUTZ_FORCE_INTRANET_FLAGS: u32 = 8192;
pub const MUTZ_IGNORE_ZONE_MAPPINGS: u32 = 16384;
pub const MUTZ_ISFILE: u32 = 2;
pub const MUTZ_NOSAVEDFILECHECK: u32 = 1;
pub const MUTZ_REQUIRESAVEDFILECHECK: u32 = 1024;
pub const MUTZ_RESERVED: u32 = 512;
pub const OIBDG_APARTMENTTHREADED: OIBDG_FLAGS = 256;
pub const OIBDG_DATAONLY: OIBDG_FLAGS = 4096;
pub type OIBDG_FLAGS = i32;
pub type PARSEACTION = i32;
pub const PARSE_ANCHOR: PARSEACTION = 6;
pub const PARSE_CANONICALIZE: PARSEACTION = 1;
pub const PARSE_DECODE: u32 = 8;
pub const PARSE_DECODE_IS_ESCAPE: PARSEACTION = 8;
pub const PARSE_DOCUMENT: PARSEACTION = 5;
pub const PARSE_DOMAIN: PARSEACTION = 15;
pub const PARSE_ENCODE: u32 = 7;
pub const PARSE_ENCODE_IS_UNESCAPE: PARSEACTION = 7;
pub const PARSE_ESCAPE: PARSEACTION = 18;
pub const PARSE_FRIENDLY: PARSEACTION = 2;
pub const PARSE_LOCATION: PARSEACTION = 16;
pub const PARSE_MIME: PARSEACTION = 11;
pub const PARSE_PATH_FROM_URL: PARSEACTION = 9;
pub const PARSE_ROOTDOCUMENT: PARSEACTION = 4;
pub const PARSE_SCHEMA: PARSEACTION = 13;
pub const PARSE_SECURITY_DOMAIN: PARSEACTION = 17;
pub const PARSE_SECURITY_URL: PARSEACTION = 3;
pub const PARSE_SERVER: PARSEACTION = 12;
pub const PARSE_SITE: PARSEACTION = 14;
pub const PARSE_UNESCAPE: PARSEACTION = 19;
pub const PARSE_URL_FROM_PATH: PARSEACTION = 10;
pub const PD_FORCE_SWITCH: PI_FLAGS = 65536;
pub const PI_APARTMENTTHREADED: PI_FLAGS = 256;
pub const PI_CLASSINSTALL: PI_FLAGS = 512;
pub const PI_CLSIDLOOKUP: PI_FLAGS = 32;
pub const PI_DATAPROGRESS: PI_FLAGS = 64;
pub const PI_DOCFILECLSIDLOOKUP: u32 = 32;
pub const PI_FILTER_MODE: PI_FLAGS = 2;
pub type PI_FLAGS = i32;
pub const PI_FORCE_ASYNC: PI_FLAGS = 4;
pub const PI_LOADAPPDIRECT: PI_FLAGS = 16384;
pub const PI_MIMEVERIFICATION: PI_FLAGS = 16;
pub const PI_NOMIMEHANDLER: PI_FLAGS = 32768;
pub const PI_PARSE_URL: PI_FLAGS = 1;
pub const PI_PASSONBINDCTX: PI_FLAGS = 8192;
pub const PI_PREFERDEFAULTHANDLER: PI_FLAGS = 131072;
pub const PI_SYNCHRONOUS: PI_FLAGS = 128;
pub const PI_USE_WORKERTHREAD: PI_FLAGS = 8;
pub const POPUPLEVELPROP: MONIKERPROPERTY = 4;
pub type PREMSECURITY_ATTRIBUTES = *mut REMSECURITY_ATTRIBUTES;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROTOCOLDATA {
    pub grfFlags: u32,
    pub dwState: u32,
    pub pData: *mut core::ffi::c_void,
    pub cbData: u32,
}
impl Default for PROTOCOLDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROTOCOLFILTERDATA {
    pub cbSize: u32,
    pub pProtocolSink: *mut core::ffi::c_void,
    pub pProtocol: *mut core::ffi::c_void,
    pub pUnk: *mut core::ffi::c_void,
    pub dwFilterFlags: u32,
}
impl Default for PROTOCOLFILTERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROTOCOL_ARGUMENT {
    pub szMethod: windows_sys::core::PCWSTR,
    pub szTargetUrl: windows_sys::core::PCWSTR,
}
impl Default for PROTOCOL_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PSUACTION = i32;
pub const PSU_DEFAULT: PSUACTION = 1;
pub const PSU_SECURITY_URL_ONLY: PSUACTION = 2;
pub type PUAF = i32;
pub type PUAFOUT = i32;
pub const PUAFOUT_DEFAULT: PUAFOUT = 0;
pub const PUAFOUT_ISLOCKZONEPOLICY: PUAFOUT = 1;
pub const PUAF_ACCEPT_WILDCARD_SCHEME: PUAF = 128;
pub const PUAF_CHECK_TIFS: PUAF = 16;
pub const PUAF_DEFAULT: PUAF = 0;
pub const PUAF_DEFAULTZONEPOL: PUAF = 262144;
pub const PUAF_DONTCHECKBOXINDIALOG: PUAF = 32;
pub const PUAF_DONT_USE_CACHE: PUAF = 4096;
pub const PUAF_DRAGPROTOCOLCHECK: PUAF = 2097152;
pub const PUAF_ENFORCERESTRICTED: PUAF = 256;
pub const PUAF_FORCEUI_FOREGROUND: PUAF = 8;
pub const PUAF_ISFILE: PUAF = 2;
pub const PUAF_LMZ_LOCKED: PUAF = 131072;
pub const PUAF_LMZ_UNLOCKED: PUAF = 65536;
pub const PUAF_NOSAVEDFILECHECK: PUAF = 512;
pub const PUAF_NOUI: PUAF = 1;
pub const PUAF_NOUIIFLOCKED: PUAF = 1048576;
pub const PUAF_NPL_USE_LOCKED_IF_RESTRICTED: PUAF = 524288;
pub const PUAF_REQUIRESAVEDFILECHECK: PUAF = 1024;
pub const PUAF_RESERVED1: PUAF = 8192;
pub const PUAF_RESERVED2: PUAF = 16384;
pub const PUAF_TRUSTED: PUAF = 64;
pub const PUAF_WARN_IF_DENIED: PUAF = 4;
pub type QUERYOPTION = i32;
pub const QUERY_CAN_NAVIGATE: QUERYOPTION = 7;
pub const QUERY_CONTENT_ENCODING: QUERYOPTION = 3;
pub const QUERY_CONTENT_TYPE: QUERYOPTION = 4;
pub const QUERY_EXPIRATION_DATE: QUERYOPTION = 1;
pub const QUERY_IS_CACHED: QUERYOPTION = 9;
pub const QUERY_IS_CACHED_AND_USABLE_OFFLINE: QUERYOPTION = 16;
pub const QUERY_IS_CACHED_OR_MAPPED: QUERYOPTION = 11;
pub const QUERY_IS_INSTALLEDENTRY: QUERYOPTION = 10;
pub const QUERY_IS_SAFE: QUERYOPTION = 14;
pub const QUERY_IS_SECURE: QUERYOPTION = 13;
pub const QUERY_RECOMBINE: QUERYOPTION = 6;
pub const QUERY_REFRESH: QUERYOPTION = 5;
pub const QUERY_TIME_OF_LAST_CHANGE: QUERYOPTION = 2;
pub const QUERY_USES_CACHE: QUERYOPTION = 12;
pub const QUERY_USES_HISTORYFOLDER: QUERYOPTION = 15;
pub const QUERY_USES_NETWORK: QUERYOPTION = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REMSECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: u32,
    pub bInheritHandle: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RemBINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_sys::core::PWSTR,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_sys::core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: REMSECURITY_ATTRIBUTES,
    pub iid: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub dwReserved: u32,
}
impl Default for RemBINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RemFORMATETC {
    pub cfFormat: u32,
    pub ptd: u32,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
pub const SECURITY_IE_STATE_GREEN: u32 = 0;
pub const SECURITY_IE_STATE_RED: u32 = 1;
pub const SET_FEATURE_IN_REGISTRY: u32 = 4;
pub const SET_FEATURE_ON_PROCESS: u32 = 2;
pub const SET_FEATURE_ON_THREAD: u32 = 1;
pub const SET_FEATURE_ON_THREAD_INTERNET: u32 = 64;
pub const SET_FEATURE_ON_THREAD_INTRANET: u32 = 16;
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: u32 = 8;
pub const SET_FEATURE_ON_THREAD_RESTRICTED: u32 = 128;
pub const SET_FEATURE_ON_THREAD_TRUSTED: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOFTDISTINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwAdState: u32,
    pub szTitle: windows_sys::core::PWSTR,
    pub szAbstract: windows_sys::core::PWSTR,
    pub szHREF: windows_sys::core::PWSTR,
    pub dwInstalledVersionMS: u32,
    pub dwInstalledVersionLS: u32,
    pub dwUpdateVersionMS: u32,
    pub dwUpdateVersionLS: u32,
    pub dwAdvertisedVersionMS: u32,
    pub dwAdvertisedVersionLS: u32,
    pub dwReserved: u32,
}
impl Default for SOFTDISTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SOFTDIST_ADSTATE_AVAILABLE: u32 = 1;
pub const SOFTDIST_ADSTATE_DOWNLOADED: u32 = 2;
pub const SOFTDIST_ADSTATE_INSTALLED: u32 = 3;
pub const SOFTDIST_ADSTATE_NONE: u32 = 0;
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: u32 = 8;
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: u32 = 4;
pub const SOFTDIST_FLAG_USAGE_EMAIL: u32 = 1;
pub const SOFTDIST_FLAG_USAGE_PRECACHE: u32 = 2;
pub const SZM_CREATE: SZM_FLAGS = 0;
pub const SZM_DELETE: SZM_FLAGS = 1;
pub type SZM_FLAGS = i32;
pub const S_ASYNCHRONOUS: u32 = 262632;
#[repr(C)]
#[cfg(feature = "Win32_objidl")]
#[derive(Clone, Copy)]
pub struct StartParam {
    pub iid: windows_sys::core::GUID,
    pub pIBindCtx: *mut core::ffi::c_void,
    pub pItf: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_objidl")]
impl Default for StartParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TRUSTEDDOWNLOADPROP: MONIKERPROPERTY = 3;
pub const UAS_EXACTLEGACY: u32 = 4096;
pub const URLACTION_ACTIVEX_ALLOW_TDC: u32 = 4620;
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: u32 = 4612;
pub const URLACTION_ACTIVEX_CURR_MAX: u32 = 4620;
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: u32 = 4618;
pub const URLACTION_ACTIVEX_MAX: u32 = 5119;
pub const URLACTION_ACTIVEX_MIN: u32 = 4608;
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: u32 = 4614;
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: u32 = 4610;
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: u32 = 4619;
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: u32 = 4609;
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: u32 = 4616;
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: u32 = 4615;
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: u32 = 4611;
pub const URLACTION_ACTIVEX_RUN: u32 = 4608;
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: u32 = 4617;
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: u32 = 4613;
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: u32 = 9986;
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: u32 = 9996;
pub const URLACTION_ALLOW_APEVALUATION: u32 = 8961;
pub const URLACTION_ALLOW_AUDIO_VIDEO: u32 = 9985;
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: u32 = 9988;
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: u32 = 9994;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: u32 = 9993;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: u32 = 9992;
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: u32 = 9997;
pub const URLACTION_ALLOW_JSCRIPT_IE: u32 = 5133;
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: u32 = 9995;
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: u32 = 8960;
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: u32 = 9987;
pub const URLACTION_ALLOW_VBSCRIPT_IE: u32 = 5132;
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: u32 = 5128;
pub const URLACTION_ALLOW_XHR_EVALUATION: u32 = 8962;
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: u32 = 9990;
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: u32 = 9989;
pub const URLACTION_AUTHENTICATE_CLIENT: u32 = 6657;
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: u32 = 8705;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: u32 = 8704;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: u32 = 8704;
pub const URLACTION_BEHAVIOR_MIN: u32 = 8192;
pub const URLACTION_BEHAVIOR_RUN: u32 = 8192;
pub const URLACTION_CHANNEL_SOFTDIST_MAX: u32 = 7935;
pub const URLACTION_CHANNEL_SOFTDIST_MIN: u32 = 7680;
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: u32 = 7685;
pub const URLACTION_CLIENT_CERT_PROMPT: u32 = 6660;
pub const URLACTION_COOKIES: u32 = 6658;
pub const URLACTION_COOKIES_ENABLED: u32 = 6672;
pub const URLACTION_COOKIES_SESSION: u32 = 6659;
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: u32 = 6662;
pub const URLACTION_COOKIES_THIRD_PARTY: u32 = 6661;
pub const URLACTION_CREDENTIALS_USE: u32 = 6656;
pub const URLACTION_CROSS_DOMAIN_DATA: u32 = 5126;
pub const URLACTION_DOTNET_USERCONTROLS: u32 = 8197;
pub const URLACTION_DOWNLOAD_CURR_MAX: u32 = 4100;
pub const URLACTION_DOWNLOAD_MAX: u32 = 4607;
pub const URLACTION_DOWNLOAD_MIN: u32 = 4096;
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: u32 = 4097;
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: u32 = 4100;
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: u32 = 8453;
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: u32 = 8455;
pub const URLACTION_FEATURE_DATA_BINDING: u32 = 8454;
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: u32 = 8452;
pub const URLACTION_FEATURE_MIME_SNIFFING: u32 = 8448;
pub const URLACTION_FEATURE_MIN: u32 = 8448;
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: u32 = 8451;
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: u32 = 8450;
pub const URLACTION_FEATURE_ZONE_ELEVATION: u32 = 8449;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: u32 = 5645;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: u32 = 5648;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: u32 = 5647;
pub const URLACTION_HTML_ALLOW_INDEXEDDB: u32 = 5649;
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: u32 = 5643;
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: u32 = 5646;
pub const URLACTION_HTML_FONT_DOWNLOAD: u32 = 5636;
pub const URLACTION_HTML_INCLUDE_FILE_PATH: u32 = 5642;
pub const URLACTION_HTML_JAVA_RUN: u32 = 5637;
pub const URLACTION_HTML_MAX: u32 = 6143;
pub const URLACTION_HTML_META_REFRESH: u32 = 5640;
pub const URLACTION_HTML_MIN: u32 = 5632;
pub const URLACTION_HTML_MIXED_CONTENT: u32 = 5641;
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: u32 = 5644;
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: u32 = 5639;
pub const URLACTION_HTML_SUBMIT_FORMS: u32 = 5633;
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: u32 = 5634;
pub const URLACTION_HTML_SUBMIT_FORMS_TO: u32 = 5635;
pub const URLACTION_HTML_USERDATA_SAVE: u32 = 5638;
pub const URLACTION_INFODELIVERY_CURR_MAX: u32 = 7430;
pub const URLACTION_INFODELIVERY_MAX: u32 = 7679;
pub const URLACTION_INFODELIVERY_MIN: u32 = 7424;
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: u32 = 7424;
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: u32 = 7427;
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: u32 = 7430;
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: u32 = 7425;
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: u32 = 7428;
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: u32 = 7426;
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: u32 = 7429;
pub const URLACTION_INPRIVATE_BLOCKING: u32 = 9984;
pub const URLACTION_JAVA_CURR_MAX: u32 = 7168;
pub const URLACTION_JAVA_MAX: u32 = 7423;
pub const URLACTION_JAVA_MIN: u32 = 7168;
pub const URLACTION_JAVA_PERMISSIONS: u32 = 7168;
pub const URLACTION_LOOSE_XAML: u32 = 9218;
pub const URLACTION_LOWRIGHTS: u32 = 9472;
pub const URLACTION_MIN: u32 = 4096;
pub const URLACTION_NETWORK_CURR_MAX: u32 = 6672;
pub const URLACTION_NETWORK_MAX: u32 = 7167;
pub const URLACTION_NETWORK_MIN: u32 = 6656;
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: u32 = 5131;
pub const URLACTION_SCRIPT_CURR_MAX: u32 = 5133;
pub const URLACTION_SCRIPT_JAVA_USE: u32 = 5122;
pub const URLACTION_SCRIPT_MAX: u32 = 5631;
pub const URLACTION_SCRIPT_MIN: u32 = 5120;
pub const URLACTION_SCRIPT_NAVIGATE: u32 = 5130;
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: u32 = 5121;
pub const URLACTION_SCRIPT_PASTE: u32 = 5127;
pub const URLACTION_SCRIPT_RUN: u32 = 5120;
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: u32 = 5125;
pub const URLACTION_SCRIPT_XSSFILTER: u32 = 5129;
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: u32 = 6161;
pub const URLACTION_SHELL_CURR_MAX: u32 = 6162;
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: u32 = 6155;
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: u32 = 6150;
pub const URLACTION_SHELL_EXECUTE_LOWRISK: u32 = 6152;
pub const URLACTION_SHELL_EXECUTE_MODRISK: u32 = 6151;
pub const URLACTION_SHELL_EXTENSIONSECURITY: u32 = 6156;
pub const URLACTION_SHELL_FILE_DOWNLOAD: u32 = 6147;
pub const URLACTION_SHELL_INSTALL_DTITEMS: u32 = 6144;
pub const URLACTION_SHELL_MAX: u32 = 6655;
pub const URLACTION_SHELL_MIN: u32 = 6144;
pub const URLACTION_SHELL_MOVE_OR_COPY: u32 = 6146;
pub const URLACTION_SHELL_POPUPMGR: u32 = 6153;
pub const URLACTION_SHELL_PREVIEW: u32 = 6159;
pub const URLACTION_SHELL_REMOTEQUERY: u32 = 6158;
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: u32 = 6154;
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: u32 = 6157;
pub const URLACTION_SHELL_SHARE: u32 = 6160;
pub const URLACTION_SHELL_SHELLEXECUTE: u32 = 6150;
pub const URLACTION_SHELL_TOCTOU_RISK: u32 = 6162;
pub const URLACTION_SHELL_VERB: u32 = 6148;
pub const URLACTION_SHELL_WEBVIEW_VERB: u32 = 6149;
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: u32 = 9216;
pub const URLACTION_WINFX_SETUP: u32 = 9728;
pub const URLACTION_XPS_DOCUMENTS: u32 = 9217;
pub const URLMON_OPTION_URL_ENCODING: u32 = 268435460;
pub const URLMON_OPTION_USERAGENT: u32 = 268435457;
pub const URLMON_OPTION_USERAGENT_REFRESH: u32 = 268435458;
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: u32 = 268435464;
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: u32 = 268435472;
pub const URLOSTRM_GETNEWESTVERSION: u32 = 3;
pub const URLOSTRM_USECACHEDCOPY: u32 = 2;
pub const URLOSTRM_USECACHEDCOPY_ONLY: u32 = 1;
pub const URLPOLICY_ACTIVEX_CHECK_LIST: u32 = 65536;
pub const URLPOLICY_ALLOW: u32 = 0;
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: u32 = 65536;
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: u32 = 0;
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: u32 = 196608;
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: u32 = 65536;
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: u32 = 196608;
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: u32 = 131072;
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: u32 = 65536;
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: u32 = 196608;
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: u32 = 131072;
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: u32 = 65536;
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: u32 = 0;
pub const URLPOLICY_DISALLOW: u32 = 3;
pub const URLPOLICY_DONTCHECKDLGBOX: u32 = 256;
pub const URLPOLICY_JAVA_CUSTOM: u32 = 8388608;
pub const URLPOLICY_JAVA_HIGH: u32 = 65536;
pub const URLPOLICY_JAVA_LOW: u32 = 196608;
pub const URLPOLICY_JAVA_MEDIUM: u32 = 131072;
pub const URLPOLICY_JAVA_PROHIBIT: u32 = 0;
pub const URLPOLICY_LOG_ON_ALLOW: u32 = 64;
pub const URLPOLICY_LOG_ON_DISALLOW: u32 = 128;
pub const URLPOLICY_MASK_PERMISSIONS: u32 = 15;
pub const URLPOLICY_NOTIFY_ON_ALLOW: u32 = 16;
pub const URLPOLICY_NOTIFY_ON_DISALLOW: u32 = 32;
pub const URLPOLICY_QUERY: u32 = 1;
pub type URLTEMPLATE = i32;
pub const URLTEMPLATE_CUSTOM: URLTEMPLATE = 0;
pub const URLTEMPLATE_HIGH: URLTEMPLATE = 73728;
pub const URLTEMPLATE_LOW: URLTEMPLATE = 65536;
pub const URLTEMPLATE_MEDHIGH: URLTEMPLATE = 70912;
pub const URLTEMPLATE_MEDIUM: URLTEMPLATE = 69632;
pub const URLTEMPLATE_MEDLOW: URLTEMPLATE = 66816;
pub const URLTEMPLATE_PREDEFINED_MAX: URLTEMPLATE = 131072;
pub const URLTEMPLATE_PREDEFINED_MIN: URLTEMPLATE = 65536;
pub type URLZONE = i32;
pub type URLZONEREG = i32;
pub const URLZONEREG_DEFAULT: URLZONEREG = 0;
pub const URLZONEREG_HKCU: URLZONEREG = 2;
pub const URLZONEREG_HKLM: URLZONEREG = 1;
pub const URLZONE_ESC_FLAG: u32 = 256;
pub const URLZONE_INTERNET: URLZONE = 3;
pub const URLZONE_INTRANET: URLZONE = 1;
pub const URLZONE_INVALID: URLZONE = -1;
pub const URLZONE_LOCAL_MACHINE: URLZONE = 0;
pub const URLZONE_PREDEFINED_MAX: URLZONE = 999;
pub const URLZONE_PREDEFINED_MIN: URLZONE = 0;
pub const URLZONE_TRUSTED: URLZONE = 2;
pub const URLZONE_UNTRUSTED: URLZONE = 4;
pub const URLZONE_USER_MAX: URLZONE = 10000;
pub const URLZONE_USER_MIN: URLZONE = 1000;
pub type URL_ENCODING = i32;
pub const URL_ENCODING_DISABLE_UTF8: URL_ENCODING = 536870912;
pub const URL_ENCODING_ENABLE_UTF8: URL_ENCODING = 268435456;
pub const URL_ENCODING_NONE: URL_ENCODING = 0;
pub const URL_MK_LEGACY: u32 = 0;
pub const URL_MK_NO_CANONICALIZE: u32 = 2;
pub const URL_MK_UNIFORM: u32 = 1;
pub const USE_SRC_URL: MONIKERPROPERTY = 1;
pub const UriBuilder_USE_ORIGINAL_FLAGS: u32 = 1;
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: u32 = 4;
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: u32 = 2;
pub const Uri_CREATE_ALLOW_RELATIVE: u32 = 1;
pub const Uri_CREATE_CANONICALIZE: u32 = 256;
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: u32 = 131072;
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: u32 = 512;
pub const Uri_CREATE_DECODE_EXTRA_INFO: u32 = 64;
pub const Uri_CREATE_FILE_USE_DOS_PATH: u32 = 32;
pub const Uri_CREATE_IE_SETTINGS: u32 = 8192;
pub const Uri_CREATE_NOFRAG: u32 = 8;
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: u32 = 65536;
pub const Uri_CREATE_NO_CANONICALIZE: u32 = 16;
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: u32 = 1024;
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: u32 = 128;
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: u32 = 32768;
pub const Uri_CREATE_NO_IE_SETTINGS: u32 = 16384;
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: u32 = 4096;
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: u32 = 2048;
pub const Uri_DISPLAY_IDN_HOST: u32 = 4;
pub const Uri_DISPLAY_NO_FRAGMENT: u32 = 1;
pub const Uri_DISPLAY_NO_PUNYCODE: u32 = 8;
pub const Uri_ENCODING_HOST_IS_IDN: u32 = 4;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: u32 = 16;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: u32 = 8;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: u32 = 64;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: u32 = 32;
pub const Uri_ENCODING_RFC: u32 = 41;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: u32 = 2;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: u32 = 1;
pub const Uri_HAS_ABSOLUTE_URI: u32 = 1;
pub const Uri_HAS_AUTHORITY: u32 = 2;
pub const Uri_HAS_DISPLAY_URI: u32 = 4;
pub const Uri_HAS_DOMAIN: u32 = 8;
pub const Uri_HAS_EXTENSION: u32 = 16;
pub const Uri_HAS_FRAGMENT: u32 = 32;
pub const Uri_HAS_HOST: u32 = 64;
pub const Uri_HAS_HOST_TYPE: u32 = 32768;
pub const Uri_HAS_PASSWORD: u32 = 128;
pub const Uri_HAS_PATH: u32 = 256;
pub const Uri_HAS_PATH_AND_QUERY: u32 = 512;
pub const Uri_HAS_PORT: u32 = 65536;
pub const Uri_HAS_QUERY: u32 = 1024;
pub const Uri_HAS_RAW_URI: u32 = 2048;
pub const Uri_HAS_SCHEME: u32 = 131072;
pub const Uri_HAS_SCHEME_NAME: u32 = 4096;
pub const Uri_HAS_USER_INFO: u32 = 8192;
pub const Uri_HAS_USER_NAME: u32 = 16384;
pub const Uri_HAS_ZONE: u32 = 262144;
pub const Uri_HOST_DNS: Uri_HOST_TYPE = 1;
pub const Uri_HOST_IDN: Uri_HOST_TYPE = 4;
pub const Uri_HOST_IPV4: Uri_HOST_TYPE = 2;
pub const Uri_HOST_IPV6: Uri_HOST_TYPE = 3;
pub type Uri_HOST_TYPE = i32;
pub const Uri_HOST_UNKNOWN: Uri_HOST_TYPE = 0;
pub type Uri_PROPERTY = i32;
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = 0;
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = 1;
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = 2;
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = 3;
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = 18;
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = 15;
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = 4;
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = 5;
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = 6;
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = 15;
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = 7;
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = 8;
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = 9;
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = 16;
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = 10;
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = 11;
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = 17;
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = 12;
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = 14;
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = 0;
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = 13;
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = 14;
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = 18;
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2;
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534;
pub type ZAFLAGS = i32;
pub const ZAFLAGS_ADD_SITES: ZAFLAGS = 2;
pub const ZAFLAGS_CUSTOM_EDIT: ZAFLAGS = 1;
pub const ZAFLAGS_DETECT_INTRANET: ZAFLAGS = 256;
pub const ZAFLAGS_INCLUDE_INTRANET_SITES: ZAFLAGS = 16;
pub const ZAFLAGS_INCLUDE_PROXY_OVERRIDE: ZAFLAGS = 8;
pub const ZAFLAGS_NO_CACHE: ZAFLAGS = 262144;
pub const ZAFLAGS_NO_UI: ZAFLAGS = 32;
pub const ZAFLAGS_REQUIRE_VERIFICATION: ZAFLAGS = 4;
pub const ZAFLAGS_SUPPORTS_VERIFICATION: ZAFLAGS = 64;
pub const ZAFLAGS_UNC_AS_INTRANET: ZAFLAGS = 128;
pub const ZAFLAGS_USE_LOCKED_ZONES: ZAFLAGS = 65536;
pub const ZAFLAGS_VERIFY_TEMPLATE_SETTINGS: ZAFLAGS = 131072;
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for ZONEATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
