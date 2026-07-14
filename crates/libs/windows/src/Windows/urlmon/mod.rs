#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CoGetClassObjectFromURL<P1, P4, P5, T>(rclassid: *const windows_core::GUID, szcode: P1, dwfileversionms: u32, dwfileversionls: u32, sztype: P4, pbindctx: P5, dwclscontext: u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<super::objidl::IBindCtx>,
    T: windows_core::Interface,
{
    windows_core::link!("urlmon.dll" "system" fn CoGetClassObjectFromURL(rclassid : *const windows_core::GUID, szcode : windows_core::PCWSTR, dwfileversionms : u32, dwfileversionls : u32, sztype : windows_core::PCWSTR, pbindctx : *mut core::ffi::c_void, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetClassObjectFromURL(rclassid, szcode.param().abi(), dwfileversionms, dwfileversionls, sztype.param().abi(), pbindctx.param().abi(), dwclscontext, pvreserved.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoInternetCombineIUri<P0, P1>(pbaseuri: P0, prelativeuri: P1, dwcombineflags: u32, ppcombineduri: *mut Option<IUri>, dwreserved: Option<usize>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IUri>,
    P1: windows_core::Param<IUri>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetCombineIUri(pbaseuri : *mut core::ffi::c_void, prelativeuri : *mut core::ffi::c_void, dwcombineflags : u32, ppcombineduri : *mut *mut core::ffi::c_void, dwreserved : usize) -> windows_core::HRESULT);
    unsafe { CoInternetCombineIUri(pbaseuri.param().abi(), prelativeuri.param().abi(), dwcombineflags, core::mem::transmute(ppcombineduri), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInternetCombineUrl<P0, P1>(pwzbaseurl: P0, pwzrelativeurl: P1, dwcombineflags: u32, pszresult: &mut [u16], pcchresult: Option<*mut u32>, dwreserved: Option<u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetCombineUrl(pwzbaseurl : windows_core::PCWSTR, pwzrelativeurl : windows_core::PCWSTR, dwcombineflags : u32, pszresult : windows_core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetCombineUrl(pwzbaseurl.param().abi(), pwzrelativeurl.param().abi(), dwcombineflags, core::mem::transmute(pszresult.as_mut_ptr()), pszresult.len().try_into().unwrap(), pcchresult.unwrap_or(core::mem::zeroed()) as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInternetCombineUrlEx<P0, P1>(pbaseuri: P0, pwzrelativeurl: P1, dwcombineflags: u32, ppcombineduri: *mut Option<IUri>, dwreserved: Option<usize>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IUri>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetCombineUrlEx(pbaseuri : *mut core::ffi::c_void, pwzrelativeurl : windows_core::PCWSTR, dwcombineflags : u32, ppcombineduri : *mut *mut core::ffi::c_void, dwreserved : usize) -> windows_core::HRESULT);
    unsafe { CoInternetCombineUrlEx(pbaseuri.param().abi(), pwzrelativeurl.param().abi(), dwcombineflags, core::mem::transmute(ppcombineduri), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInternetCompareUrl<P0, P1>(pwzurl1: P0, pwzurl2: P1, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetCompareUrl(pwzurl1 : windows_core::PCWSTR, pwzurl2 : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { CoInternetCompareUrl(pwzurl1.param().abi(), pwzurl2.param().abi(), dwflags) }
}
#[cfg(feature = "servprov")]
#[inline]
pub unsafe fn CoInternetCreateSecurityManager<P0>(psp: P0, ppsm: *mut Option<IInternetSecurityManager>, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::servprov::IServiceProvider>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetCreateSecurityManager(psp : *mut core::ffi::c_void, ppsm : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetCreateSecurityManager(psp.param().abi(), core::mem::transmute(ppsm), dwreserved) }
}
#[cfg(feature = "servprov")]
#[inline]
pub unsafe fn CoInternetCreateZoneManager<P0>(psp: P0, ppzm: *mut Option<IInternetZoneManager>, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::servprov::IServiceProvider>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetCreateZoneManager(psp : *mut core::ffi::c_void, ppzm : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetCreateZoneManager(psp.param().abi(), core::mem::transmute(ppzm), dwreserved) }
}
#[inline]
pub unsafe fn CoInternetGetProtocolFlags<P0>(pwzurl: P0, pdwflags: *mut u32, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetGetProtocolFlags(pwzurl : windows_core::PCWSTR, pdwflags : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetGetProtocolFlags(pwzurl.param().abi(), pdwflags as _, dwreserved) }
}
#[inline]
pub unsafe fn CoInternetGetSecurityUrl<P0>(pwszurl: P0, ppwszsecurl: *mut windows_core::PWSTR, psuaction: PSUACTION, dwreserved: Option<u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetGetSecurityUrl(pwszurl : windows_core::PCWSTR, ppwszsecurl : *mut windows_core::PWSTR, psuaction : PSUACTION, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetGetSecurityUrl(pwszurl.param().abi(), ppwszsecurl as _, psuaction, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInternetGetSecurityUrlEx<P0>(puri: P0, ppsecuri: *mut Option<IUri>, psuaction: PSUACTION, dwreserved: Option<usize>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IUri>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetGetSecurityUrlEx(puri : *mut core::ffi::c_void, ppsecuri : *mut *mut core::ffi::c_void, psuaction : PSUACTION, dwreserved : usize) -> windows_core::HRESULT);
    unsafe { CoInternetGetSecurityUrlEx(puri.param().abi(), core::mem::transmute(ppsecuri), psuaction, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut Option<IInternetSession>, dwreserved: u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn CoInternetGetSession(dwsessionmode : u32, ppiinternetsession : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetGetSession(dwsessionmode, core::mem::transmute(ppiinternetsession), dwreserved) }
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabled(featureentry : INTERNETFEATURELIST, dwflags : u32) -> windows_core::HRESULT);
    unsafe { CoInternetIsFeatureEnabled(featureentry, dwflags) }
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForIUri<P2, P3>(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: P2, psecmgr: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<IUri>,
    P3: windows_core::Param<IInternetSecurityManagerEx2>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabledForIUri(featureentry : INTERNETFEATURELIST, dwflags : u32, piuri : *mut core::ffi::c_void, psecmgr : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoInternetIsFeatureEnabledForIUri(featureentry, dwflags, piuri.param().abi(), psecmgr.param().abi()) }
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForUrl<P2, P3>(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: P2, psecmgr: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IInternetSecurityManager>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabledForUrl(featureentry : INTERNETFEATURELIST, dwflags : u32, szurl : windows_core::PCWSTR, psecmgr : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoInternetIsFeatureEnabledForUrl(featureentry, dwflags, szurl.param().abi(), psecmgr.param().abi()) }
}
#[inline]
pub unsafe fn CoInternetIsFeatureZoneElevationEnabled<P0, P1, P2>(szfromurl: P0, sztourl: P1, psecmgr: P2, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IInternetSecurityManager>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetIsFeatureZoneElevationEnabled(szfromurl : windows_core::PCWSTR, sztourl : windows_core::PCWSTR, psecmgr : *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { CoInternetIsFeatureZoneElevationEnabled(szfromurl.param().abi(), sztourl.param().abi(), psecmgr.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CoInternetParseIUri<P0>(piuri: P0, parseaction: PARSEACTION, dwflags: u32, pwzresult: &mut [u16], pcchresult: *mut u32, dwreserved: Option<usize>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IUri>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetParseIUri(piuri : *mut core::ffi::c_void, parseaction : PARSEACTION, dwflags : u32, pwzresult : windows_core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : usize) -> windows_core::HRESULT);
    unsafe { CoInternetParseIUri(piuri.param().abi(), parseaction, dwflags, core::mem::transmute(pwzresult.as_mut_ptr()), pwzresult.len().try_into().unwrap(), pcchresult as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInternetParseUrl<P0>(pwzurl: P0, parseaction: PARSEACTION, dwflags: u32, pszresult: &mut [u16], pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetParseUrl(pwzurl : windows_core::PCWSTR, parseaction : PARSEACTION, dwflags : u32, pszresult : windows_core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetParseUrl(pwzurl.param().abi(), parseaction, dwflags, core::mem::transmute(pszresult.as_mut_ptr()), pszresult.len().try_into().unwrap(), pcchresult as _, dwreserved) }
}
#[inline]
pub unsafe fn CoInternetQueryInfo<P0>(pwzurl: P0, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuffer: Option<*mut u32>, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CoInternetQueryInfo(pwzurl : windows_core::PCWSTR, queryoptions : QUERYOPTION, dwqueryflags : u32, pvbuffer : *mut core::ffi::c_void, cbbuffer : u32, pcbbuffer : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoInternetQueryInfo(pwzurl.param().abi(), queryoptions, dwqueryflags, pvbuffer as _, cbbuffer, pcbbuffer.unwrap_or(core::mem::zeroed()) as _, dwreserved) }
}
#[inline]
pub unsafe fn CoInternetSetFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: bool) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn CoInternetSetFeatureEnabled(featureentry : INTERNETFEATURELIST, dwflags : u32, fenable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CoInternetSetFeatureEnabled(featureentry, dwflags, fenable.into()) }
}
#[inline]
pub unsafe fn CompareSecurityIds(pbsecurityid1: &[u8], pbsecurityid2: &[u8], dwreserved: u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn CompareSecurityIds(pbsecurityid1 : *const u8, dwlen1 : u32, pbsecurityid2 : *const u8, dwlen2 : u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CompareSecurityIds(pbsecurityid1.as_ptr(), pbsecurityid1.len().try_into().unwrap(), pbsecurityid2.as_ptr(), pbsecurityid2.len().try_into().unwrap(), dwreserved) }
}
#[inline]
pub unsafe fn CompatFlagsFromClsid(pclsid: *const windows_core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn CompatFlagsFromClsid(pclsid : *const windows_core::GUID, pdwcompatflags : *mut u32, pdwmiscstatusflags : *mut u32) -> windows_core::HRESULT);
    unsafe { CompatFlagsFromClsid(pclsid, pdwcompatflags as _, pdwmiscstatusflags as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn CopyBindInfo(pcbisrc: *const BINDINFO, pbidest: *mut BINDINFO) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn CopyBindInfo(pcbisrc : *const BINDINFO, pbidest : *mut BINDINFO) -> windows_core::HRESULT);
    unsafe { CopyBindInfo(core::mem::transmute(pcbisrc), core::mem::transmute(pbidest)) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn CopyStgMedium(pcstgmedsrc: *const super::objidl::STGMEDIUM) -> windows_core::Result<super::objidl::STGMEDIUM> {
    windows_core::link!("urlmon.dll" "system" fn CopyStgMedium(pcstgmedsrc : *const super::objidl::STGMEDIUM, pstgmeddest : *mut super::objidl::STGMEDIUM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CopyStgMedium(core::mem::transmute(pcstgmedsrc), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateAsyncBindCtx<P1, P2>(reserved: u32, pbscb: P1, pefetc: P2) -> windows_core::Result<super::objidl::IBindCtx>
where
    P1: windows_core::Param<IBindStatusCallback>,
    P2: windows_core::Param<super::objidl::IEnumFORMATETC>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateAsyncBindCtx(reserved : u32, pbscb : *mut core::ffi::c_void, pefetc : *mut core::ffi::c_void, ppbc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAsyncBindCtx(reserved, pbscb.param().abi(), pefetc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateAsyncBindCtxEx<P0, P2, P3>(pbc: P0, dwoptions: u32, pbscb: P2, penum: P3, ppbc: *mut Option<super::objidl::IBindCtx>, reserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P2: windows_core::Param<IBindStatusCallback>,
    P3: windows_core::Param<super::objidl::IEnumFORMATETC>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateAsyncBindCtxEx(pbc : *mut core::ffi::c_void, dwoptions : u32, pbscb : *mut core::ffi::c_void, penum : *mut core::ffi::c_void, ppbc : *mut *mut core::ffi::c_void, reserved : u32) -> windows_core::HRESULT);
    unsafe { CreateAsyncBindCtxEx(pbc.param().abi(), dwoptions, pbscb.param().abi(), penum.param().abi(), core::mem::transmute(ppbc), reserved) }
}
#[cfg(all(feature = "objidl", feature = "wtypes"))]
#[inline]
pub unsafe fn CreateFormatEnumerator(rgfmtetc: &[super::objidl::FORMATETC]) -> windows_core::Result<super::objidl::IEnumFORMATETC> {
    windows_core::link!("urlmon.dll" "system" fn CreateFormatEnumerator(cfmtetc : u32, rgfmtetc : *const super::objidl::FORMATETC, ppenumfmtetc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateFormatEnumerator(rgfmtetc.len().try_into().unwrap(), rgfmtetc.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateIUriBuilder<P0>(piuri: P0, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder>
where
    P0: windows_core::Param<IUri>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateIUriBuilder(piuri : *mut core::ffi::c_void, dwflags : u32, dwreserved : usize, ppiuribuilder : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateIUriBuilder(piuri.param().abi(), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateURLMoniker<P0, P1>(pmkctx: P0, szurl: P1) -> windows_core::Result<super::objidl::IMoniker>
where
    P0: windows_core::Param<super::objidl::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateURLMoniker(pmkctx : *mut core::ffi::c_void, szurl : windows_core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateURLMoniker(pmkctx.param().abi(), szurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateURLMonikerEx<P0, P1>(pmkctx: P0, szurl: P1, ppmk: *mut Option<super::objidl::IMoniker>, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateURLMonikerEx(pmkctx : *mut core::ffi::c_void, szurl : windows_core::PCWSTR, ppmk : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { CreateURLMonikerEx(pmkctx.param().abi(), szurl.param().abi(), core::mem::transmute(ppmk), dwflags) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateURLMonikerEx2<P0, P1>(pmkctx: P0, puri: P1, ppmk: *mut Option<super::objidl::IMoniker>, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IMoniker>,
    P1: windows_core::Param<IUri>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateURLMonikerEx2(pmkctx : *mut core::ffi::c_void, puri : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { CreateURLMonikerEx2(pmkctx.param().abi(), puri.param().abi(), core::mem::transmute(ppmk), dwflags) }
}
#[inline]
pub unsafe fn CreateUri<P0>(pwzuri: P0, dwflags: u32, dwreserved: Option<usize>) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateUri(pwzuri : windows_core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateUri(pwzuri.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateUriFromMultiByteString<P0>(pszansiinputuri: P0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: Option<usize>) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateUriFromMultiByteString(pszansiinputuri : windows_core::PCSTR, dwencodingflags : u32, dwcodepage : u32, dwcreateflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.param().abi(), dwencodingflags, dwcodepage, dwcreateflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateUriWithFragment<P0, P1>(pwzuri: P0, pwzfragment: P1, dwflags: u32, dwreserved: Option<usize>) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateUriWithFragment(pwzuri : windows_core::PCWSTR, pwzfragment : windows_core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateUriWithFragment(pwzuri.param().abi(), pwzfragment.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "windef", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn FaultInIEFeature(hwnd: super::windef::HWND, pclassspec: *const super::wtypes::uCLSSPEC, pquery: Option<*mut super::wtypes::QUERYCONTEXT>, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn FaultInIEFeature(hwnd : super::windef::HWND, pclassspec : *const super::wtypes::uCLSSPEC, pquery : *mut super::wtypes::QUERYCONTEXT, dwflags : u32) -> windows_core::HRESULT);
    unsafe { FaultInIEFeature(hwnd, pclassspec, pquery.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn FindMediaType<P0>(rgsztypes: P0) -> windows_core::Result<super::wtypes::CLIPFORMAT>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn FindMediaType(rgsztypes : windows_core::PCSTR, rgcftypes : *mut super::wtypes::CLIPFORMAT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        FindMediaType(rgsztypes.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn FindMediaTypeClass<P0, P1>(pbc: P0, sztype: P1, pclsid: *mut windows_core::GUID, reserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn FindMediaTypeClass(pbc : *mut core::ffi::c_void, sztype : windows_core::PCSTR, pclsid : *mut windows_core::GUID, reserved : u32) -> windows_core::HRESULT);
    unsafe { FindMediaTypeClass(pbc.param().abi(), sztype.param().abi(), pclsid as _, reserved) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn FindMimeFromData<P0, P1, P4>(pbc: P0, pwzurl: P1, pbuffer: Option<*const core::ffi::c_void>, cbsize: u32, pwzmimeproposed: P4, dwmimeflags: u32, ppwzmimeout: *mut windows_core::PWSTR, dwreserved: Option<u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn FindMimeFromData(pbc : *mut core::ffi::c_void, pwzurl : windows_core::PCWSTR, pbuffer : *const core::ffi::c_void, cbsize : u32, pwzmimeproposed : windows_core::PCWSTR, dwmimeflags : u32, ppwzmimeout : *mut windows_core::PWSTR, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { FindMimeFromData(pbc.param().abi(), pwzurl.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, cbsize, pwzmimeproposed.param().abi(), dwmimeflags, ppwzmimeout as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn GetClassFileOrMime<P0, P1, P4>(pbc: P0, szfilename: P1, pbuffer: Option<*const core::ffi::c_void>, cbsize: u32, szmime: P4, dwreserved: u32) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn GetClassFileOrMime(pbc : *mut core::ffi::c_void, szfilename : windows_core::PCWSTR, pbuffer : *const core::ffi::c_void, cbsize : u32, szmime : windows_core::PCWSTR, dwreserved : u32, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetClassFileOrMime(pbc.param().abi(), szfilename.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, cbsize, szmime.param().abi(), dwreserved, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetClassURL<P0>(szurl: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn GetClassURL(szurl : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetClassURL(szurl.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn GetComponentIDFromCLSSPEC(pclassspec: *const super::wtypes::uCLSSPEC) -> windows_core::Result<windows_core::PSTR> {
    windows_core::link!("urlmon.dll" "system" fn GetComponentIDFromCLSSPEC(pclassspec : *const super::wtypes::uCLSSPEC, ppszcomponentid : *mut windows_core::PSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetComponentIDFromCLSSPEC(pclassspec, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetSoftwareUpdateInfo<P0>(szdistunit: P0, psdi: *mut SOFTDISTINFO) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn GetSoftwareUpdateInfo(szdistunit : windows_core::PCWSTR, psdi : *mut SOFTDISTINFO) -> windows_core::HRESULT);
    unsafe { GetSoftwareUpdateInfo(szdistunit.param().abi(), psdi as _) }
}
#[inline]
pub unsafe fn HlinkGoBack<P0>(punk: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("urlmon.dll" "system" fn HlinkGoBack(punk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkGoBack(punk.param().abi()) }
}
#[inline]
pub unsafe fn HlinkGoForward<P0>(punk: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("urlmon.dll" "system" fn HlinkGoForward(punk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkGoForward(punk.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn HlinkNavigateMoniker<P0, P1>(punk: P0, pmktarget: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::objidl::IMoniker>,
{
    windows_core::link!("urlmon.dll" "system" fn HlinkNavigateMoniker(punk : *mut core::ffi::c_void, pmktarget : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkNavigateMoniker(punk.param().abi(), pmktarget.param().abi()) }
}
#[inline]
pub unsafe fn HlinkNavigateString<P0, P1>(punk: P0, sztarget: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn HlinkNavigateString(punk : *mut core::ffi::c_void, sztarget : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HlinkNavigateString(punk.param().abi(), sztarget.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn HlinkSimpleNavigateToMoniker<P0, P1, P2, P3, P4, P5>(pmktarget: P0, szlocation: P1, sztargetframename: P2, punk: P3, pbc: P4, param5: P5, grfhlnf: u32, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<super::objidl::IBindCtx>,
    P5: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn HlinkSimpleNavigateToMoniker(pmktarget : *mut core::ffi::c_void, szlocation : windows_core::PCWSTR, sztargetframename : windows_core::PCWSTR, punk : *mut core::ffi::c_void, pbc : *mut core::ffi::c_void, param5 : *mut core::ffi::c_void, grfhlnf : u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { HlinkSimpleNavigateToMoniker(pmktarget.param().abi(), szlocation.param().abi(), sztargetframename.param().abi(), punk.param().abi(), pbc.param().abi(), param5.param().abi(), grfhlnf, dwreserved) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn HlinkSimpleNavigateToString<P0, P1, P2, P3, P4, P5>(sztarget: P0, szlocation: P1, sztargetframename: P2, punk: P3, pbc: P4, param5: P5, grfhlnf: u32, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<super::objidl::IBindCtx>,
    P5: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn HlinkSimpleNavigateToString(sztarget : windows_core::PCWSTR, szlocation : windows_core::PCWSTR, sztargetframename : windows_core::PCWSTR, punk : *mut core::ffi::c_void, pbc : *mut core::ffi::c_void, param5 : *mut core::ffi::c_void, grfhlnf : u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { HlinkSimpleNavigateToString(sztarget.param().abi(), szlocation.param().abi(), sztargetframename.param().abi(), punk.param().abi(), pbc.param().abi(), param5.param().abi(), grfhlnf, dwreserved) }
}
#[inline]
pub unsafe fn IEGetUserPrivateNamespaceName() -> windows_core::PWSTR {
    windows_core::link!("urlmon.dll" "system" fn IEGetUserPrivateNamespaceName() -> windows_core::PWSTR);
    unsafe { IEGetUserPrivateNamespaceName() }
}
#[inline]
pub unsafe fn IEInstallScope() -> windows_core::Result<u32> {
    windows_core::link!("urlmon.dll" "system" fn IEInstallScope(pdwscope : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IEInstallScope(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn IsAsyncMoniker<P0>(pmk: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IMoniker>,
{
    windows_core::link!("urlmon.dll" "system" fn IsAsyncMoniker(pmk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { IsAsyncMoniker(pmk.param().abi()) }
}
#[inline]
pub unsafe fn IsLoggingEnabledA<P0>(pszurl: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn IsLoggingEnabledA(pszurl : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { IsLoggingEnabledA(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn IsLoggingEnabledW<P0>(pwszurl: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn IsLoggingEnabledW(pwszurl : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IsLoggingEnabledW(pwszurl.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn IsValidURL<P0, P1>(pbc: P0, szurl: P1, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn IsValidURL(pbc : *mut core::ffi::c_void, szurl : windows_core::PCWSTR, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { IsValidURL(pbc.param().abi(), szurl.param().abi(), dwreserved) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn MkParseDisplayNameEx<P0, P1>(pbc: P0, szdisplayname: P1, pcheaten: *mut u32, ppmk: *mut Option<super::objidl::IMoniker>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn MkParseDisplayNameEx(pbc : *mut core::ffi::c_void, szdisplayname : windows_core::PCWSTR, pcheaten : *mut u32, ppmk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MkParseDisplayNameEx(pbc.param().abi(), szdisplayname.param().abi(), pcheaten as _, core::mem::transmute(ppmk)) }
}
#[inline]
pub unsafe fn ObtainUserAgentString(dwoption: u32, pszuaout: windows_core::PSTR, cbsize: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn ObtainUserAgentString(dwoption : u32, pszuaout : windows_core::PSTR, cbsize : *mut u32) -> windows_core::HRESULT);
    unsafe { ObtainUserAgentString(dwoption, core::mem::transmute(pszuaout), cbsize as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn RegisterBindStatusCallback<P0, P1>(pbc: P0, pbscb: P1, ppbscbprev: *mut Option<IBindStatusCallback>, dwreserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn RegisterBindStatusCallback(pbc : *mut core::ffi::c_void, pbscb : *mut core::ffi::c_void, ppbscbprev : *mut *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { RegisterBindStatusCallback(pbc.param().abi(), pbscb.param().abi(), core::mem::transmute(ppbscbprev), dwreserved) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn RegisterFormatEnumerator<P0, P1>(pbc: P0, pefetc: P1, reserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<super::objidl::IEnumFORMATETC>,
{
    windows_core::link!("urlmon.dll" "system" fn RegisterFormatEnumerator(pbc : *mut core::ffi::c_void, pefetc : *mut core::ffi::c_void, reserved : u32) -> windows_core::HRESULT);
    unsafe { RegisterFormatEnumerator(pbc.param().abi(), pefetc.param().abi(), reserved) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn RegisterMediaTypeClass<P0>(pbc: P0, ctypes: u32, rgsztypes: *const windows_core::PCSTR, rgclsid: *const windows_core::GUID, reserved: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
{
    windows_core::link!("urlmon.dll" "system" fn RegisterMediaTypeClass(pbc : *mut core::ffi::c_void, ctypes : u32, rgsztypes : *const windows_core::PCSTR, rgclsid : *const windows_core::GUID, reserved : u32) -> windows_core::HRESULT);
    unsafe { RegisterMediaTypeClass(pbc.param().abi(), ctypes, rgsztypes, rgclsid, reserved) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const windows_core::PCSTR, rgcftypes: *mut super::wtypes::CLIPFORMAT) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn RegisterMediaTypes(ctypes : u32, rgsztypes : *const windows_core::PCSTR, rgcftypes : *mut super::wtypes::CLIPFORMAT) -> windows_core::HRESULT);
    unsafe { RegisterMediaTypes(ctypes, rgsztypes, rgcftypes as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn ReleaseBindInfo(pbindinfo: *mut BINDINFO) {
    windows_core::link!("urlmon.dll" "system" fn ReleaseBindInfo(pbindinfo : *mut BINDINFO));
    unsafe { ReleaseBindInfo(core::mem::transmute(pbindinfo)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn RevokeBindStatusCallback<P0, P1>(pbc: P0, pbscb: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn RevokeBindStatusCallback(pbc : *mut core::ffi::c_void, pbscb : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RevokeBindStatusCallback(pbc.param().abi(), pbscb.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn RevokeFormatEnumerator<P0, P1>(pbc: P0, pefetc: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
    P1: windows_core::Param<super::objidl::IEnumFORMATETC>,
{
    windows_core::link!("urlmon.dll" "system" fn RevokeFormatEnumerator(pbc : *mut core::ffi::c_void, pefetc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RevokeFormatEnumerator(pbc.param().abi(), pefetc.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetAccessForIEAppContainer(hobject: super::winnt::HANDLE, ieobjecttype: IEObjectType, dwaccessmask: u32) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn SetAccessForIEAppContainer(hobject : super::winnt::HANDLE, ieobjecttype : IEObjectType, dwaccessmask : u32) -> windows_core::HRESULT);
    unsafe { SetAccessForIEAppContainer(hobject, ieobjecttype, dwaccessmask) }
}
#[inline]
pub unsafe fn SetSoftwareUpdateAdvertisementState<P0>(szdistunit: P0, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn SetSoftwareUpdateAdvertisementState(szdistunit : windows_core::PCWSTR, dwadstate : u32, dwadvertisedversionms : u32, dwadvertisedversionls : u32) -> windows_core::HRESULT);
    unsafe { SetSoftwareUpdateAdvertisementState(szdistunit.param().abi(), dwadstate, dwadvertisedversionms, dwadvertisedversionls) }
}
#[inline]
pub unsafe fn URLDownloadToCacheFileA<P0, P1, P5>(param0: P0, param1: P1, param2: &mut [u8], param4: u32, param5: P5) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLDownloadToCacheFileA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : windows_core::PSTR, cchfilename : u32, param4 : u32, param5 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLDownloadToCacheFileA(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.as_mut_ptr()), param2.len().try_into().unwrap(), param4, param5.param().abi()) }
}
#[inline]
pub unsafe fn URLDownloadToCacheFileW<P0, P1, P5>(param0: P0, param1: P1, param2: &mut [u16], param4: u32, param5: P5) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLDownloadToCacheFileW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : windows_core::PWSTR, cchfilename : u32, param4 : u32, param5 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLDownloadToCacheFileW(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.as_mut_ptr()), param2.len().try_into().unwrap(), param4, param5.param().abi()) }
}
#[inline]
pub unsafe fn URLDownloadToFileA<P0, P1, P2, P4>(param0: P0, param1: P1, param2: P2, param3: u32, param4: P4) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLDownloadToFileA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLDownloadToFileA(param0.param().abi(), param1.param().abi(), param2.param().abi(), param3, param4.param().abi()) }
}
#[inline]
pub unsafe fn URLDownloadToFileW<P0, P1, P2, P4>(param0: P0, param1: P1, param2: P2, param3: u32, param4: P4) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLDownloadToFileW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLDownloadToFileW(param0.param().abi(), param1.param().abi(), param2.param().abi(), param3, param4.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn URLOpenBlockingStreamA<P0, P1, P4>(param0: P0, param1: P1, param2: *mut Option<super::objidlbase::IStream>, param3: u32, param4: P4) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLOpenBlockingStreamA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : *mut *mut core::ffi::c_void, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLOpenBlockingStreamA(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2), param3, param4.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn URLOpenBlockingStreamW<P0, P1, P4>(param0: P0, param1: P1, param2: *mut Option<super::objidlbase::IStream>, param3: u32, param4: P4) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLOpenBlockingStreamW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : *mut *mut core::ffi::c_void, param3 : u32, param4 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLOpenBlockingStreamW(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2), param3, param4.param().abi()) }
}
#[inline]
pub unsafe fn URLOpenPullStreamA<P0, P1, P3>(param0: P0, param1: P1, param2: u32, param3: P3) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLOpenPullStreamA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLOpenPullStreamA(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()) }
}
#[inline]
pub unsafe fn URLOpenPullStreamW<P0, P1, P3>(param0: P0, param1: P1, param2: u32, param3: P3) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLOpenPullStreamW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLOpenPullStreamW(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()) }
}
#[inline]
pub unsafe fn URLOpenStreamA<P0, P1, P3>(param0: P0, param1: P1, param2: u32, param3: P3) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLOpenStreamA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLOpenStreamA(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()) }
}
#[inline]
pub unsafe fn URLOpenStreamW<P0, P1, P3>(param0: P0, param1: P1, param2: u32, param3: P3) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("urlmon.dll" "system" fn URLOpenStreamW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { URLOpenStreamW(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()) }
}
#[inline]
pub unsafe fn UrlMkGetSessionOption(dwoption: u32, pbuffer: Option<*mut core::ffi::c_void>, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: Option<u32>) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn UrlMkGetSessionOption(dwoption : u32, pbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbufferlengthout : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { UrlMkGetSessionOption(dwoption, pbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength, pdwbufferlengthout as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UrlMkSetSessionOption(dwoption: u32, pbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32, dwreserved: Option<u32>) -> windows_core::HRESULT {
    windows_core::link!("urlmon.dll" "system" fn UrlMkSetSessionOption(dwoption : u32, pbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { UrlMkSetSessionOption(dwoption, pbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> windows_core::BOOL {
    windows_core::link!("urlmon.dll" "system" fn WriteHitLogging(lplogginginfo : *const HIT_LOGGING_INFO) -> windows_core::BOOL);
    unsafe { WriteHitLogging(lplogginginfo) }
}
pub type AUTHENTICATEF = i32;
pub const AUTHENTICATEF_BASIC: AUTHENTICATEF = 2;
pub const AUTHENTICATEF_HTTP: AUTHENTICATEF = 4;
pub const AUTHENTICATEF_PROXY: AUTHENTICATEF = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_core::PWSTR,
    pub stgmedData: super::objidl::STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::minwinbase::SECURITY_ATTRIBUTES,
    pub iid: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Clone for BINDINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODEBASEHOLD {
    pub cbSize: u32,
    pub szDistUnit: windows_core::PWSTR,
    pub szCodeBase: windows_core::PWSTR,
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
    pub dwStyle: u32,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CONFIRMSAFETY {
    pub clsid: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwFlags: u32,
}
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIT_LOGGING_INFO {
    pub dwStructSize: u32,
    pub lpszLoggedUrlName: windows_core::PSTR,
    pub StartTime: super::minwinbase::SYSTEMTIME,
    pub EndTime: super::minwinbase::SYSTEMTIME,
    pub lpszExtendedInfo: windows_core::PSTR,
}
windows_core::imp::define_interface!(IAuthenticate, IAuthenticate_Vtbl, 0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IAuthenticate, windows_core::IUnknown);
impl IAuthenticate {
    #[cfg(feature = "windef")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::windef::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), phwnd as _, pszusername as _, pszpassword as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Authenticate: usize,
}
#[cfg(feature = "windef")]
pub trait IAuthenticate_Impl: windows_core::IUnknownImpl {
    fn Authenticate(&self, phwnd: *mut super::windef::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAuthenticate_Vtbl {
    pub const fn new<Identity: IAuthenticate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Authenticate<Identity: IAuthenticate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAuthenticate_Impl::Authenticate(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pszusername), core::mem::transmute_copy(&pszpassword)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Authenticate: Authenticate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAuthenticate {}
windows_core::imp::define_interface!(IAuthenticateEx, IAuthenticateEx_Vtbl, 0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
impl core::ops::Deref for IAuthenticateEx {
    type Target = IAuthenticate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAuthenticateEx, windows_core::IUnknown, IAuthenticate);
impl IAuthenticateEx {
    #[cfg(feature = "windef")]
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut super::windef::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AuthenticateEx)(windows_core::Interface::as_raw(self), phwnd as _, pszusername as _, pszpassword as _, pauthinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateEx_Vtbl {
    pub base__: IAuthenticate_Vtbl,
    #[cfg(feature = "windef")]
    pub AuthenticateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *const AUTHENTICATEINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AuthenticateEx: usize,
}
#[cfg(feature = "windef")]
pub trait IAuthenticateEx_Impl: IAuthenticate_Impl {
    fn AuthenticateEx(&self, phwnd: *mut super::windef::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAuthenticateEx_Vtbl {
    pub const fn new<Identity: IAuthenticateEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AuthenticateEx<Identity: IAuthenticateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAuthenticateEx_Impl::AuthenticateEx(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pszusername), core::mem::transmute_copy(&pszpassword), core::mem::transmute_copy(&pauthinfo)).into()
            }
        }
        Self { base__: IAuthenticate_Vtbl::new::<Identity, OFFSET>(), AuthenticateEx: AuthenticateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticateEx as windows_core::Interface>::IID || iid == &<IAuthenticate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAuthenticateEx {}
windows_core::imp::define_interface!(IBindCallbackRedirect, IBindCallbackRedirect_Vtbl, 0x11c81bc2_121e_4ed5_b9c4_b430bd54f2c0);
windows_core::imp::interface_hierarchy!(IBindCallbackRedirect, windows_core::IUnknown);
impl IBindCallbackRedirect {
    #[cfg(feature = "wtypes")]
    pub unsafe fn Redirect<P0>(&self, lpcurl: P0) -> windows_core::Result<super::wtypes::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Redirect)(windows_core::Interface::as_raw(self), lpcurl.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCallbackRedirect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub Redirect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Redirect: usize,
}
#[cfg(feature = "wtypes")]
pub trait IBindCallbackRedirect_Impl: windows_core::IUnknownImpl {
    fn Redirect(&self, lpcurl: &windows_core::PCWSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(feature = "wtypes")]
impl IBindCallbackRedirect_Vtbl {
    pub const fn new<Identity: IBindCallbackRedirect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Redirect<Identity: IBindCallbackRedirect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcurl: windows_core::PCWSTR, vbcancel: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCallbackRedirect_Impl::Redirect(this, core::mem::transmute(&lpcurl)) {
                    Ok(ok__) => {
                        vbcancel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Redirect: Redirect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindCallbackRedirect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IBindCallbackRedirect {}
windows_core::imp::define_interface!(IBindHost, IBindHost_Vtbl, 0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
windows_core::imp::interface_hierarchy!(IBindHost, windows_core::IUnknown);
impl IBindHost {
    #[cfg(feature = "objidl")]
    pub unsafe fn CreateMoniker<P0, P1>(&self, szname: P0, pbc: P1, ppmk: *mut Option<super::objidl::IMoniker>, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMoniker)(windows_core::Interface::as_raw(self), szname.param().abi(), pbc.param().abi(), core::mem::transmute(ppmk), dwreserved) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn MonikerBindToStorage<P0, P1, P2, T>(&self, pmk: P0, pbc: P1, pbsc: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::objidl::IMoniker>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
        P2: windows_core::Param<IBindStatusCallback>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).MonikerBindToStorage)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), pbsc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn MonikerBindToObject<P0, P1, P2, T>(&self, pmk: P0, pbc: P1, pbsc: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::objidl::IMoniker>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
        P2: windows_core::Param<IBindStatusCallback>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).MonikerBindToObject)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), pbsc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidl")]
    pub CreateMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    CreateMoniker: usize,
    #[cfg(feature = "objidl")]
    pub MonikerBindToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    MonikerBindToStorage: usize,
    #[cfg(feature = "objidl")]
    pub MonikerBindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    MonikerBindToObject: usize,
}
#[cfg(feature = "objidl")]
pub trait IBindHost_Impl: windows_core::IUnknownImpl {
    fn CreateMoniker(&self, szname: &windows_core::PCWSTR, pbc: windows_core::Ref<super::objidl::IBindCtx>, ppmk: windows_core::OutRef<super::objidl::IMoniker>, dwreserved: u32) -> windows_core::Result<()>;
    fn MonikerBindToStorage(&self, pmk: windows_core::Ref<super::objidl::IMoniker>, pbc: windows_core::Ref<super::objidl::IBindCtx>, pbsc: windows_core::Ref<IBindStatusCallback>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn MonikerBindToObject(&self, pmk: windows_core::Ref<super::objidl::IMoniker>, pbc: windows_core::Ref<super::objidl::IBindCtx>, pbsc: windows_core::Ref<IBindStatusCallback>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IBindHost_Vtbl {
    pub const fn new<Identity: IBindHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMoniker<Identity: IBindHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindHost_Impl::CreateMoniker(this, core::mem::transmute(&szname), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&ppmk), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn MonikerBindToStorage<Identity: IBindHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pbsc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindHost_Impl::MonikerBindToStorage(this, core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pbsc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn MonikerBindToObject<Identity: IBindHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pbsc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindHost_Impl::MonikerBindToObject(this, core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pbsc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMoniker: CreateMoniker::<Identity, OFFSET>,
            MonikerBindToStorage: MonikerBindToStorage::<Identity, OFFSET>,
            MonikerBindToObject: MonikerBindToObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindHost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IBindHost {}
windows_core::imp::define_interface!(IBindHttpSecurity, IBindHttpSecurity_Vtbl, 0xa9eda967_f50e_4a33_b358_206f6ef3086d);
windows_core::imp::interface_hierarchy!(IBindHttpSecurity, windows_core::IUnknown);
impl IBindHttpSecurity {
    pub unsafe fn GetIgnoreCertMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIgnoreCertMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHttpSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIgnoreCertMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBindHttpSecurity_Impl: windows_core::IUnknownImpl {
    fn GetIgnoreCertMask(&self) -> windows_core::Result<u32>;
}
impl IBindHttpSecurity_Vtbl {
    pub const fn new<Identity: IBindHttpSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIgnoreCertMask<Identity: IBindHttpSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwignorecertmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindHttpSecurity_Impl::GetIgnoreCertMask(this) {
                    Ok(ok__) => {
                        pdwignorecertmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIgnoreCertMask: GetIgnoreCertMask::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindHttpSecurity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBindHttpSecurity {}
windows_core::imp::define_interface!(IBindProtocol, IBindProtocol_Vtbl, 0x79eac9cd_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IBindProtocol, windows_core::IUnknown);
impl IBindProtocol {
    #[cfg(feature = "objidl")]
    pub unsafe fn CreateBinding<P0, P1>(&self, szurl: P0, pbc: P1) -> windows_core::Result<IBinding>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBinding)(windows_core::Interface::as_raw(self), szurl.param().abi(), pbc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindProtocol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidl")]
    pub CreateBinding: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    CreateBinding: usize,
}
#[cfg(feature = "objidl")]
pub trait IBindProtocol_Impl: windows_core::IUnknownImpl {
    fn CreateBinding(&self, szurl: &windows_core::PCWSTR, pbc: windows_core::Ref<super::objidl::IBindCtx>) -> windows_core::Result<IBinding>;
}
#[cfg(feature = "objidl")]
impl IBindProtocol_Vtbl {
    pub const fn new<Identity: IBindProtocol_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBinding<Identity: IBindProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, ppb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindProtocol_Impl::CreateBinding(this, core::mem::transmute(&szurl), core::mem::transmute_copy(&pbc)) {
                    Ok(ok__) => {
                        ppb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateBinding: CreateBinding::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindProtocol as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IBindProtocol {}
windows_core::imp::define_interface!(IBindStatusCallback, IBindStatusCallback_Vtbl, 0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IBindStatusCallback, windows_core::IUnknown);
impl IBindStatusCallback {
    pub unsafe fn OnStartBinding<P1>(&self, dwreserved: u32, pib: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IBinding>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStartBinding)(windows_core::Interface::as_raw(self), dwreserved, pib.param().abi()) }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLowResource)(windows_core::Interface::as_raw(self), reserved) }
    }
    pub unsafe fn OnProgress<P3>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), ulprogress, ulprogressmax, ulstatuscode, szstatustext.param().abi()) }
    }
    pub unsafe fn OnStopBinding<P1>(&self, hresult: windows_core::HRESULT, szerror: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStopBinding)(windows_core::Interface::as_raw(self), hresult, szerror.param().abi()) }
    }
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindInfo)(windows_core::Interface::as_raw(self), grfbindf as _, core::mem::transmute(pbindinfo)) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const super::objidl::FORMATETC, pstgmed: *const super::objidl::STGMEDIUM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDataAvailable)(windows_core::Interface::as_raw(self), grfbscf, dwsize, pformatetc, core::mem::transmute(pstgmed)) }
    }
    pub unsafe fn OnObjectAvailable<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnObjectAvailable)(windows_core::Interface::as_raw(self), riid, punk.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartBinding: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OnLowResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnStopBinding: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub GetBindInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    GetBindInfo: usize,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub OnDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::objidl::FORMATETC, *const super::objidl::STGMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    OnDataAvailable: usize,
    pub OnObjectAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IBindStatusCallback_Impl: windows_core::IUnknownImpl {
    fn OnStartBinding(&self, dwreserved: u32, pib: windows_core::Ref<IBinding>) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn OnLowResource(&self, reserved: u32) -> windows_core::Result<()>;
    fn OnProgress(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnStopBinding(&self, hresult: windows_core::HRESULT, szerror: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::Result<()>;
    fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const super::objidl::FORMATETC, pstgmed: *const super::objidl::STGMEDIUM) -> windows_core::Result<()>;
    fn OnObjectAvailable(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IBindStatusCallback_Vtbl {
    pub const fn new<Identity: IBindStatusCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartBinding<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, pib: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnStartBinding(this, core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pib)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindStatusCallback_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pnpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnLowResource<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnLowResource(this, core::mem::transmute_copy(&reserved)).into()
            }
        }
        unsafe extern "system" fn OnProgress<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnProgress(this, core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax), core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szstatustext)).into()
            }
        }
        unsafe extern "system" fn OnStopBinding<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, szerror: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnStopBinding(this, core::mem::transmute_copy(&hresult), core::mem::transmute(&szerror)).into()
            }
        }
        unsafe extern "system" fn GetBindInfo<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::GetBindInfo(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo)).into()
            }
        }
        unsafe extern "system" fn OnDataAvailable<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const super::objidl::FORMATETC, pstgmed: *const super::objidl::STGMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnDataAvailable(this, core::mem::transmute_copy(&grfbscf), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed)).into()
            }
        }
        unsafe extern "system" fn OnObjectAvailable<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnObjectAvailable(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartBinding: OnStartBinding::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            OnLowResource: OnLowResource::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnStopBinding: OnStopBinding::<Identity, OFFSET>,
            GetBindInfo: GetBindInfo::<Identity, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, OFFSET>,
            OnObjectAvailable: OnObjectAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IBindStatusCallback {}
windows_core::imp::define_interface!(IBindStatusCallbackEx, IBindStatusCallbackEx_Vtbl, 0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
impl core::ops::Deref for IBindStatusCallbackEx {
    type Target = IBindStatusCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindStatusCallbackEx, windows_core::IUnknown, IBindStatusCallback);
impl IBindStatusCallbackEx {
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindInfoEx)(windows_core::Interface::as_raw(self), grfbindf as _, core::mem::transmute(pbindinfo), grfbindf2 as _, pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_Vtbl {
    pub base__: IBindStatusCallback_Vtbl,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub GetBindInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    GetBindInfoEx: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IBindStatusCallbackEx_Impl: IBindStatusCallback_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IBindStatusCallbackEx_Vtbl {
    pub const fn new<Identity: IBindStatusCallbackEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBindInfoEx<Identity: IBindStatusCallbackEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallbackEx_Impl::GetBindInfoEx(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&grfbindf2), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IBindStatusCallback_Vtbl::new::<Identity, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindStatusCallbackEx as windows_core::Interface>::IID || iid == &<IBindStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IBindStatusCallbackEx {}
windows_core::imp::define_interface!(IBinding, IBinding_Vtbl, 0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IBinding, windows_core::IUnknown);
impl IBinding {
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Suspend(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPriority(&self, npriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), npriority) }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindResult)(windows_core::Interface::as_raw(self), pclsidprotocol as _, pdwresult as _, pszresult as _, pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetBindResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IBinding_Impl: windows_core::IUnknownImpl {
    fn Abort(&self) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetPriority(&self, npriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn GetBindResult(&self, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl IBinding_Vtbl {
    pub const fn new<Identity: IBinding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Abort<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn Suspend<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::Suspend(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::SetPriority(this, core::mem::transmute_copy(&npriority)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBinding_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pnpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBindResult<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::GetBindResult(this, core::mem::transmute_copy(&pclsidprotocol), core::mem::transmute_copy(&pdwresult), core::mem::transmute_copy(&pszresult), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Abort: Abort::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            GetBindResult: GetBindResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBinding as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBinding {}
windows_core::imp::define_interface!(ICatalogFileInfo, ICatalogFileInfo_Vtbl, 0x711c7600_6b48_11d1_b403_00aa00b92af1);
windows_core::imp::interface_hierarchy!(ICatalogFileInfo, windows_core::IUnknown);
impl ICatalogFileInfo {
    pub unsafe fn GetCatalogFile(&self) -> windows_core::Result<windows_core::PSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCatalogFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetJavaTrust(&self, ppjavatrust: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJavaTrust)(windows_core::Interface::as_raw(self), ppjavatrust as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogFileInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCatalogFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PSTR) -> windows_core::HRESULT,
    pub GetJavaTrust: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICatalogFileInfo_Impl: windows_core::IUnknownImpl {
    fn GetCatalogFile(&self) -> windows_core::Result<windows_core::PSTR>;
    fn GetJavaTrust(&self, ppjavatrust: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICatalogFileInfo_Vtbl {
    pub const fn new<Identity: ICatalogFileInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCatalogFile<Identity: ICatalogFileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcatalogfile: *mut windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogFileInfo_Impl::GetCatalogFile(this) {
                    Ok(ok__) => {
                        ppszcatalogfile.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJavaTrust<Identity: ICatalogFileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppjavatrust: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogFileInfo_Impl::GetJavaTrust(this, core::mem::transmute_copy(&ppjavatrust)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCatalogFile: GetCatalogFile::<Identity, OFFSET>,
            GetJavaTrust: GetJavaTrust::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogFileInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICatalogFileInfo {}
windows_core::imp::define_interface!(ICodeInstall, ICodeInstall_Vtbl, 0x79eac9d1_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for ICodeInstall {
    type Target = IWindowForBindingUI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICodeInstall, windows_core::IUnknown, IWindowForBindingUI);
impl ICodeInstall {
    pub unsafe fn OnCodeInstallProblem<P1, P2>(&self, ulstatuscode: u32, szdestination: P1, szsource: P2, dwreserved: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCodeInstallProblem)(windows_core::Interface::as_raw(self), ulstatuscode, szdestination.param().abi(), szsource.param().abi(), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodeInstall_Vtbl {
    pub base__: IWindowForBindingUI_Vtbl,
    pub OnCodeInstallProblem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ICodeInstall_Impl: IWindowForBindingUI_Impl {
    fn OnCodeInstallProblem(&self, ulstatuscode: u32, szdestination: &windows_core::PCWSTR, szsource: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl ICodeInstall_Vtbl {
    pub const fn new<Identity: ICodeInstall_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCodeInstallProblem<Identity: ICodeInstall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstatuscode: u32, szdestination: windows_core::PCWSTR, szsource: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICodeInstall_Impl::OnCodeInstallProblem(this, core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szdestination), core::mem::transmute(&szsource), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self { base__: IWindowForBindingUI_Vtbl::new::<Identity, OFFSET>(), OnCodeInstallProblem: OnCodeInstallProblem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICodeInstall as windows_core::Interface>::IID || iid == &<IWindowForBindingUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ICodeInstall {}
windows_core::imp::define_interface!(IDataFilter, IDataFilter_Vtbl, 0x69d14c80_c18e_11d0_a9ce_006097942311);
windows_core::imp::interface_hierarchy!(IDataFilter, windows_core::IUnknown);
impl IDataFilter {
    pub unsafe fn DoEncode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoEncode)(windows_core::Interface::as_raw(self), dwflags, linbuffersize, pbinbuffer, loutbuffersize, pboutbuffer as _, linbytesavailable, plinbytesread as _, ploutbyteswritten as _, dwreserved) }
    }
    pub unsafe fn DoDecode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoDecode)(windows_core::Interface::as_raw(self), dwflags, linbuffersize, pbinbuffer, loutbuffersize, pboutbuffer as _, linbytesavailable, plinbytesread as _, ploutbyteswritten as _, dwreserved) }
    }
    pub unsafe fn SetEncodingLevel(&self, dwenclevel: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEncodingLevel)(windows_core::Interface::as_raw(self), dwenclevel) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoEncode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const u8, i32, *mut u8, i32, *mut i32, *mut i32, u32) -> windows_core::HRESULT,
    pub DoDecode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const u8, i32, *mut u8, i32, *mut i32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetEncodingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDataFilter_Impl: windows_core::IUnknownImpl {
    fn DoEncode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::Result<()>;
    fn DoDecode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::Result<()>;
    fn SetEncodingLevel(&self, dwenclevel: u32) -> windows_core::Result<()>;
}
impl IDataFilter_Vtbl {
    pub const fn new<Identity: IDataFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoEncode<Identity: IDataFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataFilter_Impl::DoEncode(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&linbuffersize), core::mem::transmute_copy(&pbinbuffer), core::mem::transmute_copy(&loutbuffersize), core::mem::transmute_copy(&pboutbuffer), core::mem::transmute_copy(&linbytesavailable), core::mem::transmute_copy(&plinbytesread), core::mem::transmute_copy(&ploutbyteswritten), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn DoDecode<Identity: IDataFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataFilter_Impl::DoDecode(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&linbuffersize), core::mem::transmute_copy(&pbinbuffer), core::mem::transmute_copy(&loutbuffersize), core::mem::transmute_copy(&pboutbuffer), core::mem::transmute_copy(&linbytesavailable), core::mem::transmute_copy(&plinbytesread), core::mem::transmute_copy(&ploutbyteswritten), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SetEncodingLevel<Identity: IDataFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenclevel: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataFilter_Impl::SetEncodingLevel(this, core::mem::transmute_copy(&dwenclevel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoEncode: DoEncode::<Identity, OFFSET>,
            DoDecode: DoDecode::<Identity, OFFSET>,
            SetEncodingLevel: SetEncodingLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDataFilter {}
pub type IEObjectType = i32;
pub const IE_EPM_OBJECT_EVENT: IEObjectType = 0;
pub const IE_EPM_OBJECT_FILE: IEObjectType = 5;
pub const IE_EPM_OBJECT_MUTEX: IEObjectType = 1;
pub const IE_EPM_OBJECT_NAMED_PIPE: IEObjectType = 6;
pub const IE_EPM_OBJECT_REGISTRY: IEObjectType = 7;
pub const IE_EPM_OBJECT_SEMAPHORE: IEObjectType = 2;
pub const IE_EPM_OBJECT_SHARED_MEMORY: IEObjectType = 3;
pub const IE_EPM_OBJECT_WAITABLE_TIMER: IEObjectType = 4;
windows_core::imp::define_interface!(IEncodingFilterFactory, IEncodingFilterFactory_Vtbl, 0x70bdde00_c18e_11d0_a9ce_006097942311);
windows_core::imp::interface_hierarchy!(IEncodingFilterFactory, windows_core::IUnknown);
impl IEncodingFilterFactory {
    pub unsafe fn FindBestFilter<P0, P1>(&self, pwzcodein: P0, pwzcodeout: P1, info: DATAINFO) -> windows_core::Result<IDataFilter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindBestFilter)(windows_core::Interface::as_raw(self), pwzcodein.param().abi(), pwzcodeout.param().abi(), core::mem::transmute(info), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDefaultFilter<P0, P1>(&self, pwzcodein: P0, pwzcodeout: P1) -> windows_core::Result<IDataFilter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultFilter)(windows_core::Interface::as_raw(self), pwzcodein.param().abi(), pwzcodeout.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEncodingFilterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindBestFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, DATAINFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEncodingFilterFactory_Impl: windows_core::IUnknownImpl {
    fn FindBestFilter(&self, pwzcodein: &windows_core::PCWSTR, pwzcodeout: &windows_core::PCWSTR, info: &DATAINFO) -> windows_core::Result<IDataFilter>;
    fn GetDefaultFilter(&self, pwzcodein: &windows_core::PCWSTR, pwzcodeout: &windows_core::PCWSTR) -> windows_core::Result<IDataFilter>;
}
impl IEncodingFilterFactory_Vtbl {
    pub const fn new<Identity: IEncodingFilterFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindBestFilter<Identity: IEncodingFilterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzcodein: windows_core::PCWSTR, pwzcodeout: windows_core::PCWSTR, info: DATAINFO, ppdf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEncodingFilterFactory_Impl::FindBestFilter(this, core::mem::transmute(&pwzcodein), core::mem::transmute(&pwzcodeout), core::mem::transmute(&info)) {
                    Ok(ok__) => {
                        ppdf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: IEncodingFilterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzcodein: windows_core::PCWSTR, pwzcodeout: windows_core::PCWSTR, ppdf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEncodingFilterFactory_Impl::GetDefaultFilter(this, core::mem::transmute(&pwzcodein), core::mem::transmute(&pwzcodeout)) {
                    Ok(ok__) => {
                        ppdf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindBestFilter: FindBestFilter::<Identity, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEncodingFilterFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEncodingFilterFactory {}
windows_core::imp::define_interface!(IGetBindHandle, IGetBindHandle_Vtbl, 0xaf0ff408_129d_4b20_91f0_02bd23d88352);
windows_core::imp::interface_hierarchy!(IGetBindHandle, windows_core::IUnknown);
impl IGetBindHandle {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBindHandle)(windows_core::Interface::as_raw(self), enumrequestedhandle, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetBindHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetBindHandle: unsafe extern "system" fn(*mut core::ffi::c_void, BINDHANDLETYPES, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetBindHandle: usize,
}
#[cfg(feature = "winnt")]
pub trait IGetBindHandle_Impl: windows_core::IUnknownImpl {
    fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(feature = "winnt")]
impl IGetBindHandle_Vtbl {
    pub const fn new<Identity: IGetBindHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBindHandle<Identity: IGetBindHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGetBindHandle_Impl::GetBindHandle(this, core::mem::transmute_copy(&enumrequestedhandle)) {
                    Ok(ok__) => {
                        prethandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBindHandle: GetBindHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetBindHandle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IGetBindHandle {}
windows_core::imp::define_interface!(IHttpNegotiate, IHttpNegotiate_Vtbl, 0x79eac9d2_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IHttpNegotiate, windows_core::IUnknown);
impl IHttpNegotiate {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginningTransaction)(windows_core::Interface::as_raw(self), szurl.param().abi(), szheaders.param().abi(), dwreserved, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnResponse<P1, P2>(&self, dwresponsecode: u32, szresponseheaders: P1, szrequestheaders: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnResponse)(windows_core::Interface::as_raw(self), dwresponsecode, szresponseheaders.param().abi(), szrequestheaders.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginningTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub OnResponse: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IHttpNegotiate_Impl: windows_core::IUnknownImpl {
    fn BeginningTransaction(&self, szurl: &windows_core::PCWSTR, szheaders: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn OnResponse(&self, dwresponsecode: u32, szresponseheaders: &windows_core::PCWSTR, szrequestheaders: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl IHttpNegotiate_Vtbl {
    pub const fn new<Identity: IHttpNegotiate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginningTransaction<Identity: IHttpNegotiate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, szheaders: windows_core::PCWSTR, dwreserved: u32, pszadditionalheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHttpNegotiate_Impl::BeginningTransaction(this, core::mem::transmute(&szurl), core::mem::transmute(&szheaders), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        pszadditionalheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnResponse<Identity: IHttpNegotiate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwresponsecode: u32, szresponseheaders: windows_core::PCWSTR, szrequestheaders: windows_core::PCWSTR, pszadditionalrequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHttpNegotiate_Impl::OnResponse(this, core::mem::transmute_copy(&dwresponsecode), core::mem::transmute(&szresponseheaders), core::mem::transmute(&szrequestheaders)) {
                    Ok(ok__) => {
                        pszadditionalrequestheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginningTransaction: BeginningTransaction::<Identity, OFFSET>,
            OnResponse: OnResponse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpNegotiate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHttpNegotiate {}
windows_core::imp::define_interface!(IHttpNegotiate2, IHttpNegotiate2_Vtbl, 0x4f9f9fcb_e0f4_48eb_b7ab_fa2ea9365cb4);
impl core::ops::Deref for IHttpNegotiate2 {
    type Target = IHttpNegotiate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpNegotiate2, windows_core::IUnknown, IHttpNegotiate);
impl IHttpNegotiate2 {
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRootSecurityId)(windows_core::Interface::as_raw(self), pbsecurityid as _, pcbsecurityid as _, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate2_Vtbl {
    pub base__: IHttpNegotiate_Vtbl,
    pub GetRootSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
}
pub trait IHttpNegotiate2_Impl: IHttpNegotiate_Impl {
    fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
}
impl IHttpNegotiate2_Vtbl {
    pub const fn new<Identity: IHttpNegotiate2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRootSecurityId<Identity: IHttpNegotiate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHttpNegotiate2_Impl::GetRootSecurityId(this, core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self { base__: IHttpNegotiate_Vtbl::new::<Identity, OFFSET>(), GetRootSecurityId: GetRootSecurityId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpNegotiate2 as windows_core::Interface>::IID || iid == &<IHttpNegotiate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHttpNegotiate2 {}
windows_core::imp::define_interface!(IHttpNegotiate3, IHttpNegotiate3_Vtbl, 0x57b6c80a_34c2_4602_bc26_66a02fc57153);
impl core::ops::Deref for IHttpNegotiate3 {
    type Target = IHttpNegotiate2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpNegotiate3, windows_core::IUnknown, IHttpNegotiate, IHttpNegotiate2);
impl IHttpNegotiate3 {
    pub unsafe fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSerializedClientCertContext)(windows_core::Interface::as_raw(self), ppbcert as _, pcbcert as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate3_Vtbl {
    pub base__: IHttpNegotiate2_Vtbl,
    pub GetSerializedClientCertContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IHttpNegotiate3_Impl: IHttpNegotiate2_Impl {
    fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::Result<()>;
}
impl IHttpNegotiate3_Vtbl {
    pub const fn new<Identity: IHttpNegotiate3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSerializedClientCertContext<Identity: IHttpNegotiate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHttpNegotiate3_Impl::GetSerializedClientCertContext(this, core::mem::transmute_copy(&ppbcert), core::mem::transmute_copy(&pcbcert)).into()
            }
        }
        Self { base__: IHttpNegotiate2_Vtbl::new::<Identity, OFFSET>(), GetSerializedClientCertContext: GetSerializedClientCertContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpNegotiate3 as windows_core::Interface>::IID || iid == &<IHttpNegotiate as windows_core::Interface>::IID || iid == &<IHttpNegotiate2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHttpNegotiate3 {}
windows_core::imp::define_interface!(IHttpSecurity, IHttpSecurity_Vtbl, 0x79eac9d7_bafa_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IHttpSecurity {
    type Target = IWindowForBindingUI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpSecurity, windows_core::IUnknown, IWindowForBindingUI);
impl IHttpSecurity {
    pub unsafe fn OnSecurityProblem(&self, dwproblem: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSecurityProblem)(windows_core::Interface::as_raw(self), dwproblem) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpSecurity_Vtbl {
    pub base__: IWindowForBindingUI_Vtbl,
    pub OnSecurityProblem: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IHttpSecurity_Impl: IWindowForBindingUI_Impl {
    fn OnSecurityProblem(&self, dwproblem: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IHttpSecurity_Vtbl {
    pub const fn new<Identity: IHttpSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSecurityProblem<Identity: IHttpSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproblem: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHttpSecurity_Impl::OnSecurityProblem(this, core::mem::transmute_copy(&dwproblem)).into()
            }
        }
        Self { base__: IWindowForBindingUI_Vtbl::new::<Identity, OFFSET>(), OnSecurityProblem: OnSecurityProblem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpSecurity as windows_core::Interface>::IID || iid == &<IWindowForBindingUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IHttpSecurity {}
windows_core::imp::define_interface!(IInternet, IInternet_Vtbl, 0x79eac9e0_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternet, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IInternet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IInternet_Impl: windows_core::IUnknownImpl {}
impl IInternet_Vtbl {
    pub const fn new<Identity: IInternet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternet {}
windows_core::imp::define_interface!(IInternetBindInfo, IInternetBindInfo_Vtbl, 0x79eac9e1_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetBindInfo, windows_core::IUnknown);
impl IInternetBindInfo {
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindInfo)(windows_core::Interface::as_raw(self), grfbindf as _, core::mem::transmute(pbindinfo)) }
    }
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindString)(windows_core::Interface::as_raw(self), ulstringtype, ppwzstr as _, cel, pcelfetched as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetBindInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub GetBindInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    GetBindInfo: usize,
    pub GetBindString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IInternetBindInfo_Impl: windows_core::IUnknownImpl {
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::Result<()>;
    fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IInternetBindInfo_Vtbl {
    pub const fn new<Identity: IInternetBindInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBindInfo<Identity: IInternetBindInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetBindInfo_Impl::GetBindInfo(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo)).into()
            }
        }
        unsafe extern "system" fn GetBindString<Identity: IInternetBindInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetBindInfo_Impl::GetBindString(this, core::mem::transmute_copy(&ulstringtype), core::mem::transmute_copy(&ppwzstr), core::mem::transmute_copy(&cel), core::mem::transmute_copy(&pcelfetched)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBindInfo: GetBindInfo::<Identity, OFFSET>,
            GetBindString: GetBindString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetBindInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IInternetBindInfo {}
windows_core::imp::define_interface!(IInternetBindInfoEx, IInternetBindInfoEx_Vtbl, 0xa3e015b7_a82c_4dcd_a150_569aeeed36ab);
impl core::ops::Deref for IInternetBindInfoEx {
    type Target = IInternetBindInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetBindInfoEx, windows_core::IUnknown, IInternetBindInfo);
impl IInternetBindInfoEx {
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindInfoEx)(windows_core::Interface::as_raw(self), grfbindf as _, core::mem::transmute(pbindinfo), grfbindf2 as _, pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetBindInfoEx_Vtbl {
    pub base__: IInternetBindInfo_Vtbl,
    #[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub GetBindInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    GetBindInfoEx: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IInternetBindInfoEx_Impl: IInternetBindInfo_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IInternetBindInfoEx_Vtbl {
    pub const fn new<Identity: IInternetBindInfoEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBindInfoEx<Identity: IInternetBindInfoEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetBindInfoEx_Impl::GetBindInfoEx(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&grfbindf2), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IInternetBindInfo_Vtbl::new::<Identity, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetBindInfoEx as windows_core::Interface>::IID || iid == &<IInternetBindInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IInternetBindInfoEx {}
windows_core::imp::define_interface!(IInternetHostSecurityManager, IInternetHostSecurityManager_Vtbl, 0x3af280b6_cb3f_11d0_891e_00c04fb6bfc4);
windows_core::imp::interface_hierarchy!(IInternetHostSecurityManager, windows_core::IUnknown);
impl IInternetHostSecurityManager {
    pub unsafe fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSecurityId)(windows_core::Interface::as_raw(self), pbsecurityid as _, pcbsecurityid as _, dwreserved) }
    }
    pub unsafe fn ProcessUrlAction(&self, dwaction: u32, ppolicy: &mut [u8], pcontext: Option<&[u8]>, dwflags: u32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessUrlAction)(windows_core::Interface::as_raw(self), dwaction, ppolicy.as_mut_ptr(), ppolicy.len().try_into().unwrap(), pcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), pcontext.map_or(0, |slice| slice.len().try_into().unwrap()), dwflags, dwreserved) }
    }
    pub unsafe fn QueryCustomPolicy(&self, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: &[u8], dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryCustomPolicy)(windows_core::Interface::as_raw(self), guidkey, pppolicy as _, pcbpolicy as _, pcontext.as_ptr(), pcontext.len().try_into().unwrap(), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetHostSecurityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
    pub ProcessUrlAction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, u32, *const u8, u32, u32, u32) -> windows_core::HRESULT,
    pub QueryCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut u8, *mut u32, *const u8, u32, u32) -> windows_core::HRESULT,
}
pub trait IInternetHostSecurityManager_Impl: windows_core::IUnknownImpl {
    fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
    fn ProcessUrlAction(&self, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn QueryCustomPolicy(&self, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl IInternetHostSecurityManager_Vtbl {
    pub const fn new<Identity: IInternetHostSecurityManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecurityId<Identity: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetHostSecurityManager_Impl::GetSecurityId(this, core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetHostSecurityManager_Impl::ProcessUrlAction(this, core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetHostSecurityManager_Impl::QueryCustomPolicy(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSecurityId: GetSecurityId::<Identity, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetHostSecurityManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetHostSecurityManager {}
windows_core::imp::define_interface!(IInternetPriority, IInternetPriority_Vtbl, 0x79eac9eb_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetPriority, windows_core::IUnknown);
impl IInternetPriority {
    pub unsafe fn SetPriority(&self, npriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), npriority) }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetPriority_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IInternetPriority_Impl: windows_core::IUnknownImpl {
    fn SetPriority(&self, npriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
}
impl IInternetPriority_Vtbl {
    pub const fn new<Identity: IInternetPriority_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPriority<Identity: IInternetPriority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetPriority_Impl::SetPriority(this, core::mem::transmute_copy(&npriority)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IInternetPriority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInternetPriority_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pnpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetPriority as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetPriority {}
windows_core::imp::define_interface!(IInternetProtocol, IInternetProtocol_Vtbl, 0x79eac9e4_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetProtocol {
    type Target = IInternetProtocolRoot;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocol, windows_core::IUnknown, IInternetProtocolRoot);
impl IInternetProtocol {
    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv as _, cb, pcbread as _) }
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRequest)(windows_core::Interface::as_raw(self), dwoptions) }
    }
    pub unsafe fn UnlockRequest(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRequest)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocol_Vtbl {
    pub base__: IInternetProtocolRoot_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, *mut u64) -> windows_core::HRESULT,
    pub LockRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnlockRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "basetsd")]
pub trait IInternetProtocol_Impl: IInternetProtocolRoot_Impl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn Seek(&self, dlibmove: i64, dworigin: u32) -> windows_core::Result<u64>;
    fn LockRequest(&self, dwoptions: u32) -> windows_core::Result<()>;
    fn UnlockRequest(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "basetsd")]
impl IInternetProtocol_Vtbl {
    pub const fn new<Identity: IInternetProtocol_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocol_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInternetProtocol_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin)) {
                    Ok(ok__) => {
                        plibnewposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockRequest<Identity: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocol_Impl::LockRequest(this, core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn UnlockRequest<Identity: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocol_Impl::UnlockRequest(this).into()
            }
        }
        Self {
            base__: IInternetProtocolRoot_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            LockRequest: LockRequest::<Identity, OFFSET>,
            UnlockRequest: UnlockRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocol as windows_core::Interface>::IID || iid == &<IInternetProtocolRoot as windows_core::Interface>::IID
    }
}
#[cfg(feature = "basetsd")]
impl windows_core::RuntimeName for IInternetProtocol {}
windows_core::imp::define_interface!(IInternetProtocolEx, IInternetProtocolEx_Vtbl, 0xc7a98e66_1010_492c_a1c8_c809e1f75905);
impl core::ops::Deref for IInternetProtocolEx {
    type Target = IInternetProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocolEx, windows_core::IUnknown, IInternetProtocolRoot, IInternetProtocol);
impl IInternetProtocolEx {
    #[cfg(feature = "basetsd")]
    pub unsafe fn StartEx<P0, P1, P2>(&self, puri: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: super::basetsd::HANDLE_PTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUri>,
        P1: windows_core::Param<IInternetProtocolSink>,
        P2: windows_core::Param<IInternetBindInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartEx)(windows_core::Interface::as_raw(self), puri.param().abi(), poiprotsink.param().abi(), poibindinfo.param().abi(), grfpi, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolEx_Vtbl {
    pub base__: IInternetProtocol_Vtbl,
    #[cfg(feature = "basetsd")]
    pub StartEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::basetsd::HANDLE_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    StartEx: usize,
}
#[cfg(feature = "basetsd")]
pub trait IInternetProtocolEx_Impl: IInternetProtocol_Impl {
    fn StartEx(&self, puri: windows_core::Ref<IUri>, poiprotsink: windows_core::Ref<IInternetProtocolSink>, poibindinfo: windows_core::Ref<IInternetBindInfo>, grfpi: u32, dwreserved: super::basetsd::HANDLE_PTR) -> windows_core::Result<()>;
}
#[cfg(feature = "basetsd")]
impl IInternetProtocolEx_Vtbl {
    pub const fn new<Identity: IInternetProtocolEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartEx<Identity: IInternetProtocolEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, poiprotsink: *mut core::ffi::c_void, poibindinfo: *mut core::ffi::c_void, grfpi: u32, dwreserved: super::basetsd::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolEx_Impl::StartEx(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&poiprotsink), core::mem::transmute_copy(&poibindinfo), core::mem::transmute_copy(&grfpi), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self { base__: IInternetProtocol_Vtbl::new::<Identity, OFFSET>(), StartEx: StartEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolEx as windows_core::Interface>::IID || iid == &<IInternetProtocolRoot as windows_core::Interface>::IID || iid == &<IInternetProtocol as windows_core::Interface>::IID
    }
}
#[cfg(feature = "basetsd")]
impl windows_core::RuntimeName for IInternetProtocolEx {}
windows_core::imp::define_interface!(IInternetProtocolInfo, IInternetProtocolInfo_Vtbl, 0x79eac9ec_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetProtocolInfo, windows_core::IUnknown);
impl IInternetProtocolInfo {
    pub unsafe fn ParseUrl<P0>(&self, pwzurl: P0, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseUrl)(windows_core::Interface::as_raw(self), pwzurl.param().abi(), parseaction, dwparseflags, core::mem::transmute(pwzresult), cchresult, pcchresult as _, dwreserved) }
    }
    pub unsafe fn CombineUrl<P0, P1>(&self, pwzbaseurl: P0, pwzrelativeurl: P1, dwcombineflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CombineUrl)(windows_core::Interface::as_raw(self), pwzbaseurl.param().abi(), pwzrelativeurl.param().abi(), dwcombineflags, core::mem::transmute(pwzresult), cchresult, pcchresult as _, dwreserved) }
    }
    pub unsafe fn CompareUrl<P0, P1>(&self, pwzurl1: P0, pwzurl2: P1, dwcompareflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompareUrl)(windows_core::Interface::as_raw(self), pwzurl1.param().abi(), pwzurl2.param().abi(), dwcompareflags) }
    }
    pub unsafe fn QueryInfo<P0>(&self, pwzurl: P0, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryInfo)(windows_core::Interface::as_raw(self), pwzurl.param().abi(), oueryoption, dwqueryflags, pbuffer as _, cbbuffer, pcbbuf as _, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParseUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PARSEACTION, u32, windows_core::PWSTR, u32, *mut u32, u32) -> windows_core::HRESULT,
    pub CombineUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, windows_core::PWSTR, u32, *mut u32, u32) -> windows_core::HRESULT,
    pub CompareUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub QueryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, QUERYOPTION, u32, *mut core::ffi::c_void, u32, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IInternetProtocolInfo_Impl: windows_core::IUnknownImpl {
    fn ParseUrl(&self, pwzurl: &windows_core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
    fn CombineUrl(&self, pwzbaseurl: &windows_core::PCWSTR, pwzrelativeurl: &windows_core::PCWSTR, dwcombineflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
    fn CompareUrl(&self, pwzurl1: &windows_core::PCWSTR, pwzurl2: &windows_core::PCWSTR, dwcompareflags: u32) -> windows_core::Result<()>;
    fn QueryInfo(&self, pwzurl: &windows_core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl IInternetProtocolInfo_Vtbl {
    pub const fn new<Identity: IInternetProtocolInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseUrl<Identity: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzurl: windows_core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolInfo_Impl::ParseUrl(this, core::mem::transmute(&pwzurl), core::mem::transmute_copy(&parseaction), core::mem::transmute_copy(&dwparseflags), core::mem::transmute_copy(&pwzresult), core::mem::transmute_copy(&cchresult), core::mem::transmute_copy(&pcchresult), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn CombineUrl<Identity: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbaseurl: windows_core::PCWSTR, pwzrelativeurl: windows_core::PCWSTR, dwcombineflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolInfo_Impl::CombineUrl(this, core::mem::transmute(&pwzbaseurl), core::mem::transmute(&pwzrelativeurl), core::mem::transmute_copy(&dwcombineflags), core::mem::transmute_copy(&pwzresult), core::mem::transmute_copy(&cchresult), core::mem::transmute_copy(&pcchresult), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn CompareUrl<Identity: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzurl1: windows_core::PCWSTR, pwzurl2: windows_core::PCWSTR, dwcompareflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolInfo_Impl::CompareUrl(this, core::mem::transmute(&pwzurl1), core::mem::transmute(&pwzurl2), core::mem::transmute_copy(&dwcompareflags)).into()
            }
        }
        unsafe extern "system" fn QueryInfo<Identity: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzurl: windows_core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolInfo_Impl::QueryInfo(this, core::mem::transmute(&pwzurl), core::mem::transmute_copy(&oueryoption), core::mem::transmute_copy(&dwqueryflags), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcbbuf), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseUrl: ParseUrl::<Identity, OFFSET>,
            CombineUrl: CombineUrl::<Identity, OFFSET>,
            CompareUrl: CompareUrl::<Identity, OFFSET>,
            QueryInfo: QueryInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetProtocolInfo {}
windows_core::imp::define_interface!(IInternetProtocolRoot, IInternetProtocolRoot_Vtbl, 0x79eac9e3_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetProtocolRoot, windows_core::IUnknown);
impl IInternetProtocolRoot {
    #[cfg(feature = "basetsd")]
    pub unsafe fn Start<P0, P1, P2>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: super::basetsd::HANDLE_PTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IInternetProtocolSink>,
        P2: windows_core::Param<IInternetBindInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), szurl.param().abi(), poiprotsink.param().abi(), poibindinfo.param().abi(), grfpi, dwreserved) }
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self), pprotocoldata) }
    }
    pub unsafe fn Abort(&self, hrreason: windows_core::HRESULT, dwoptions: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), hrreason, dwoptions) }
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), dwoptions) }
    }
    pub unsafe fn Suspend(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolRoot_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "basetsd")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::basetsd::HANDLE_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    Start: usize,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROTOCOLDATA) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "basetsd")]
pub trait IInternetProtocolRoot_Impl: windows_core::IUnknownImpl {
    fn Start(&self, szurl: &windows_core::PCWSTR, poiprotsink: windows_core::Ref<IInternetProtocolSink>, poibindinfo: windows_core::Ref<IInternetBindInfo>, grfpi: u32, dwreserved: super::basetsd::HANDLE_PTR) -> windows_core::Result<()>;
    fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::Result<()>;
    fn Abort(&self, hrreason: windows_core::HRESULT, dwoptions: u32) -> windows_core::Result<()>;
    fn Terminate(&self, dwoptions: u32) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "basetsd")]
impl IInternetProtocolRoot_Vtbl {
    pub const fn new<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, poiprotsink: *mut core::ffi::c_void, poibindinfo: *mut core::ffi::c_void, grfpi: u32, dwreserved: super::basetsd::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolRoot_Impl::Start(this, core::mem::transmute(&szurl), core::mem::transmute_copy(&poiprotsink), core::mem::transmute_copy(&poibindinfo), core::mem::transmute_copy(&grfpi), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn Continue<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolRoot_Impl::Continue(this, core::mem::transmute_copy(&pprotocoldata)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT, dwoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolRoot_Impl::Abort(this, core::mem::transmute_copy(&hrreason), core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolRoot_Impl::Terminate(this, core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn Suspend<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolRoot_Impl::Suspend(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolRoot_Impl::Resume(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Continue: Continue::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolRoot as windows_core::Interface>::IID
    }
}
#[cfg(feature = "basetsd")]
impl windows_core::RuntimeName for IInternetProtocolRoot {}
windows_core::imp::define_interface!(IInternetProtocolSink, IInternetProtocolSink_Vtbl, 0x79eac9e5_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetProtocolSink, windows_core::IUnknown);
impl IInternetProtocolSink {
    pub unsafe fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Switch)(windows_core::Interface::as_raw(self), pprotocoldata) }
    }
    pub unsafe fn ReportProgress<P1>(&self, ulstatuscode: u32, szstatustext: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), ulstatuscode, szstatustext.param().abi()) }
    }
    pub unsafe fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReportData)(windows_core::Interface::as_raw(self), grfbscf, ulprogress, ulprogressmax) }
    }
    pub unsafe fn ReportResult<P2>(&self, hrresult: windows_core::HRESULT, dwerror: u32, szresult: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportResult)(windows_core::Interface::as_raw(self), hrresult, dwerror, szresult.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Switch: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROTOCOLDATA) -> windows_core::HRESULT,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReportData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub ReportResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IInternetProtocolSink_Impl: windows_core::IUnknownImpl {
    fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::Result<()>;
    fn ReportProgress(&self, ulstatuscode: u32, szstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()>;
    fn ReportResult(&self, hrresult: windows_core::HRESULT, dwerror: u32, szresult: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IInternetProtocolSink_Vtbl {
    pub const fn new<Identity: IInternetProtocolSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Switch<Identity: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSink_Impl::Switch(this, core::mem::transmute_copy(&pprotocoldata)).into()
            }
        }
        unsafe extern "system" fn ReportProgress<Identity: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstatuscode: u32, szstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSink_Impl::ReportProgress(this, core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szstatustext)).into()
            }
        }
        unsafe extern "system" fn ReportData<Identity: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSink_Impl::ReportData(this, core::mem::transmute_copy(&grfbscf), core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax)).into()
            }
        }
        unsafe extern "system" fn ReportResult<Identity: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT, dwerror: u32, szresult: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSink_Impl::ReportResult(this, core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&dwerror), core::mem::transmute(&szresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Switch: Switch::<Identity, OFFSET>,
            ReportProgress: ReportProgress::<Identity, OFFSET>,
            ReportData: ReportData::<Identity, OFFSET>,
            ReportResult: ReportResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetProtocolSink {}
windows_core::imp::define_interface!(IInternetProtocolSinkStackable, IInternetProtocolSinkStackable_Vtbl, 0x79eac9f0_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetProtocolSinkStackable, windows_core::IUnknown);
impl IInternetProtocolSinkStackable {
    pub unsafe fn SwitchSink<P0>(&self, poiprotsink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IInternetProtocolSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SwitchSink)(windows_core::Interface::as_raw(self), poiprotsink.param().abi()) }
    }
    pub unsafe fn CommitSwitch(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitSwitch)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RollbackSwitch(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RollbackSwitch)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolSinkStackable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SwitchSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitSwitch: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RollbackSwitch: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternetProtocolSinkStackable_Impl: windows_core::IUnknownImpl {
    fn SwitchSink(&self, poiprotsink: windows_core::Ref<IInternetProtocolSink>) -> windows_core::Result<()>;
    fn CommitSwitch(&self) -> windows_core::Result<()>;
    fn RollbackSwitch(&self) -> windows_core::Result<()>;
}
impl IInternetProtocolSinkStackable_Vtbl {
    pub const fn new<Identity: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SwitchSink<Identity: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poiprotsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSinkStackable_Impl::SwitchSink(this, core::mem::transmute_copy(&poiprotsink)).into()
            }
        }
        unsafe extern "system" fn CommitSwitch<Identity: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSinkStackable_Impl::CommitSwitch(this).into()
            }
        }
        unsafe extern "system" fn RollbackSwitch<Identity: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetProtocolSinkStackable_Impl::RollbackSwitch(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SwitchSink: SwitchSink::<Identity, OFFSET>,
            CommitSwitch: CommitSwitch::<Identity, OFFSET>,
            RollbackSwitch: RollbackSwitch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolSinkStackable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetProtocolSinkStackable {}
windows_core::imp::define_interface!(IInternetSecurityManager, IInternetSecurityManager_Vtbl, 0x79eac9ee_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetSecurityManager, windows_core::IUnknown);
impl IInternetSecurityManager {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IInternetSecurityMgrSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecuritySite)(windows_core::Interface::as_raw(self), psite.param().abi()) }
    }
    pub unsafe fn GetSecuritySite(&self) -> windows_core::Result<IInternetSecurityMgrSite> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecuritySite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MapUrlToZone)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pdwzone as _, dwflags) }
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSecurityId)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pbsecurityid as _, pcbsecurityid as _, dwreserved) }
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessUrlAction)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwaction, ppolicy as _, cbpolicy, pcontext, cbcontext, dwflags, dwreserved) }
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryCustomPolicy)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), guidkey, pppolicy as _, pcbpolicy as _, pcontext, cbcontext, dwreserved) }
    }
    pub unsafe fn SetZoneMapping<P1>(&self, dwzone: u32, lpszpattern: P1, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetZoneMapping)(windows_core::Interface::as_raw(self), dwzone, lpszpattern.param().abi(), dwflags) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut Option<super::objidlbase::IEnumString>, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneMappings)(windows_core::Interface::as_raw(self), dwzone, core::mem::transmute(ppenumstring), dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSecuritySite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecuritySite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MapUrlToZone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, u32) -> windows_core::HRESULT,
    pub GetSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
    pub ProcessUrlAction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u8, u32, *const u8, u32, u32, u32) -> windows_core::HRESULT,
    pub QueryCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut u8, *mut u32, *const u8, u32, u32) -> windows_core::HRESULT,
    pub SetZoneMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub GetZoneMappings: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    GetZoneMappings: usize,
}
#[cfg(feature = "objidlbase")]
pub trait IInternetSecurityManager_Impl: windows_core::IUnknownImpl {
    fn SetSecuritySite(&self, psite: windows_core::Ref<IInternetSecurityMgrSite>) -> windows_core::Result<()>;
    fn GetSecuritySite(&self) -> windows_core::Result<IInternetSecurityMgrSite>;
    fn MapUrlToZone(&self, pwszurl: &windows_core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetSecurityId(&self, pwszurl: &windows_core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
    fn ProcessUrlAction(&self, pwszurl: &windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn QueryCustomPolicy(&self, pwszurl: &windows_core::PCWSTR, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn SetZoneMapping(&self, dwzone: u32, lpszpattern: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetZoneMappings(&self, dwzone: u32, ppenumstring: windows_core::OutRef<super::objidlbase::IEnumString>, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IInternetSecurityManager_Vtbl {
    pub const fn new<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSecuritySite<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::SetSecuritySite(this, core::mem::transmute_copy(&psite)).into()
            }
        }
        unsafe extern "system" fn GetSecuritySite<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInternetSecurityManager_Impl::GetSecuritySite(this) {
                    Ok(ok__) => {
                        ppsite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MapUrlToZone<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::MapUrlToZone(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pdwzone), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetSecurityId<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::GetSecurityId(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::ProcessUrlAction(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::QueryCustomPolicy(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SetZoneMapping<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, lpszpattern: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::SetZoneMapping(this, core::mem::transmute_copy(&dwzone), core::mem::transmute(&lpszpattern), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetZoneMappings<Identity: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, ppenumstring: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManager_Impl::GetZoneMappings(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&ppenumstring), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSecuritySite: SetSecuritySite::<Identity, OFFSET>,
            GetSecuritySite: GetSecuritySite::<Identity, OFFSET>,
            MapUrlToZone: MapUrlToZone::<Identity, OFFSET>,
            GetSecurityId: GetSecurityId::<Identity, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, OFFSET>,
            SetZoneMapping: SetZoneMapping::<Identity, OFFSET>,
            GetZoneMappings: GetZoneMappings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IInternetSecurityManager {}
windows_core::imp::define_interface!(IInternetSecurityManagerEx, IInternetSecurityManagerEx_Vtbl, 0xf164edf1_cc7c_4f0d_9a94_34222625c393);
impl core::ops::Deref for IInternetSecurityManagerEx {
    type Target = IInternetSecurityManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSecurityManagerEx, windows_core::IUnknown, IInternetSecurityManager);
impl IInternetSecurityManagerEx {
    pub unsafe fn ProcessUrlActionEx<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessUrlActionEx)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwaction, ppolicy as _, cbpolicy, pcontext, cbcontext, dwflags, dwreserved, pdwoutflags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManagerEx_Vtbl {
    pub base__: IInternetSecurityManager_Vtbl,
    pub ProcessUrlActionEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u8, u32, *const u8, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IInternetSecurityManagerEx_Impl: IInternetSecurityManager_Impl {
    fn ProcessUrlActionEx(&self, pwszurl: &windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IInternetSecurityManagerEx_Vtbl {
    pub const fn new<Identity: IInternetSecurityManagerEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessUrlActionEx<Identity: IInternetSecurityManagerEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManagerEx_Impl::ProcessUrlActionEx(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pdwoutflags)).into()
            }
        }
        Self { base__: IInternetSecurityManager_Vtbl::new::<Identity, OFFSET>(), ProcessUrlActionEx: ProcessUrlActionEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx as windows_core::Interface>::IID || iid == &<IInternetSecurityManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IInternetSecurityManagerEx {}
windows_core::imp::define_interface!(IInternetSecurityManagerEx2, IInternetSecurityManagerEx2_Vtbl, 0xf1e50292_a795_4117_8e09_2b560a72ac60);
impl core::ops::Deref for IInternetSecurityManagerEx2 {
    type Target = IInternetSecurityManagerEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSecurityManagerEx2, windows_core::IUnknown, IInternetSecurityManager, IInternetSecurityManagerEx);
impl IInternetSecurityManagerEx2 {
    pub unsafe fn MapUrlToZoneEx2<P0>(&self, puri: P0, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut windows_core::PWSTR, pdwoutflags: Option<*mut u32>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).MapUrlToZoneEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), pdwzone as _, dwflags, ppwszmappedurl as _, pdwoutflags.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ProcessUrlActionEx2<P0>(&self, puri: P0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessUrlActionEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), dwaction, ppolicy as _, cbpolicy, pcontext, cbcontext, dwflags, dwreserved, pdwoutflags as _) }
    }
    pub unsafe fn GetSecurityIdEx2<P0>(&self, puri: P0, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSecurityIdEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), pbsecurityid as _, pcbsecurityid as _, dwreserved) }
    }
    pub unsafe fn QueryCustomPolicyEx2<P0>(&self, puri: P0, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryCustomPolicyEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), guidkey, pppolicy as _, pcbpolicy as _, pcontext, cbcontext, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManagerEx2_Vtbl {
    pub base__: IInternetSecurityManagerEx_Vtbl,
    pub MapUrlToZoneEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ProcessUrlActionEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u8, u32, *const u8, u32, u32, usize, *mut u32) -> windows_core::HRESULT,
    pub GetSecurityIdEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
    pub QueryCustomPolicyEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut u8, *mut u32, *const u8, u32, usize) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IInternetSecurityManagerEx2_Impl: IInternetSecurityManagerEx_Impl {
    fn MapUrlToZoneEx2(&self, puri: windows_core::Ref<IUri>, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut windows_core::PWSTR, pdwoutflags: *mut u32) -> windows_core::Result<()>;
    fn ProcessUrlActionEx2(&self, puri: windows_core::Ref<IUri>, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> windows_core::Result<()>;
    fn GetSecurityIdEx2(&self, puri: windows_core::Ref<IUri>, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
    fn QueryCustomPolicyEx2(&self, puri: windows_core::Ref<IUri>, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IInternetSecurityManagerEx2_Vtbl {
    pub const fn new<Identity: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MapUrlToZoneEx2<Identity: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut windows_core::PWSTR, pdwoutflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManagerEx2_Impl::MapUrlToZoneEx2(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&pdwzone), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppwszmappedurl), core::mem::transmute_copy(&pdwoutflags)).into()
            }
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Identity: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManagerEx2_Impl::ProcessUrlActionEx2(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pdwoutflags)).into()
            }
        }
        unsafe extern "system" fn GetSecurityIdEx2<Identity: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManagerEx2_Impl::GetSecurityIdEx2(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Identity: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityManagerEx2_Impl::QueryCustomPolicyEx2(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: IInternetSecurityManagerEx_Vtbl::new::<Identity, OFFSET>(),
            MapUrlToZoneEx2: MapUrlToZoneEx2::<Identity, OFFSET>,
            ProcessUrlActionEx2: ProcessUrlActionEx2::<Identity, OFFSET>,
            GetSecurityIdEx2: GetSecurityIdEx2::<Identity, OFFSET>,
            QueryCustomPolicyEx2: QueryCustomPolicyEx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx2 as windows_core::Interface>::IID || iid == &<IInternetSecurityManager as windows_core::Interface>::IID || iid == &<IInternetSecurityManagerEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IInternetSecurityManagerEx2 {}
windows_core::imp::define_interface!(IInternetSecurityMgrSite, IInternetSecurityMgrSite_Vtbl, 0x79eac9ed_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetSecurityMgrSite, windows_core::IUnknown);
impl IInternetSecurityMgrSite {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnableModeless(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityMgrSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindow: usize,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IInternetSecurityMgrSite_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::windef::HWND>;
    fn EnableModeless(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IInternetSecurityMgrSite_Vtbl {
    pub const fn new<Identity: IInternetSecurityMgrSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IInternetSecurityMgrSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInternetSecurityMgrSite_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: IInternetSecurityMgrSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSecurityMgrSite_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityMgrSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IInternetSecurityMgrSite {}
windows_core::imp::define_interface!(IInternetSession, IInternetSession_Vtbl, 0x79eac9e7_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetSession, windows_core::IUnknown);
impl IInternetSession {
    #[cfg(feature = "unknwnbase")]
    pub unsafe fn RegisterNameSpace<P0, P2>(&self, pcf: P0, rclsid: *const windows_core::GUID, pwzprotocol: P2, cpatterns: u32, ppwzpatterns: *const windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::unknwnbase::IClassFactory>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterNameSpace)(windows_core::Interface::as_raw(self), pcf.param().abi(), rclsid, pwzprotocol.param().abi(), cpatterns, ppwzpatterns, dwreserved) }
    }
    #[cfg(feature = "unknwnbase")]
    pub unsafe fn UnregisterNameSpace<P0, P1>(&self, pcf: P0, pszprotocol: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::unknwnbase::IClassFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterNameSpace)(windows_core::Interface::as_raw(self), pcf.param().abi(), pszprotocol.param().abi()) }
    }
    #[cfg(feature = "unknwnbase")]
    pub unsafe fn RegisterMimeFilter<P0, P2>(&self, pcf: P0, rclsid: *const windows_core::GUID, pwztype: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::unknwnbase::IClassFactory>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterMimeFilter)(windows_core::Interface::as_raw(self), pcf.param().abi(), rclsid, pwztype.param().abi()) }
    }
    #[cfg(feature = "unknwnbase")]
    pub unsafe fn UnregisterMimeFilter<P0, P1>(&self, pcf: P0, pwztype: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::unknwnbase::IClassFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterMimeFilter)(windows_core::Interface::as_raw(self), pcf.param().abi(), pwztype.param().abi()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn CreateBinding<P0, P1, P2>(&self, pbc: P0, szurl: P1, punkouter: P2, ppunk: *mut Option<windows_core::IUnknown>, ppoinetprot: *mut Option<IInternetProtocol>, dwoption: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IBindCtx>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateBinding)(windows_core::Interface::as_raw(self), pbc.param().abi(), szurl.param().abi(), punkouter.param().abi(), core::mem::transmute(ppunk), core::mem::transmute(ppoinetprot), dwoption) }
    }
    pub unsafe fn SetSessionOption(&self, dwoption: u32, pbuffer: *const core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSessionOption)(windows_core::Interface::as_raw(self), dwoption, pbuffer, dwbufferlength, dwreserved) }
    }
    pub unsafe fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSessionOption)(windows_core::Interface::as_raw(self), dwoption, pbuffer as _, pdwbufferlength as _, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "unknwnbase")]
    pub RegisterNameSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "unknwnbase"))]
    RegisterNameSpace: usize,
    #[cfg(feature = "unknwnbase")]
    pub UnregisterNameSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "unknwnbase"))]
    UnregisterNameSpace: usize,
    #[cfg(feature = "unknwnbase")]
    pub RegisterMimeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "unknwnbase"))]
    RegisterMimeFilter: usize,
    #[cfg(feature = "unknwnbase")]
    pub UnregisterMimeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "unknwnbase"))]
    UnregisterMimeFilter: usize,
    #[cfg(feature = "objidl")]
    pub CreateBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    CreateBinding: usize,
    pub SetSessionOption: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSessionOption: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
pub trait IInternetSession_Impl: windows_core::IUnknownImpl {
    fn RegisterNameSpace(&self, pcf: windows_core::Ref<super::unknwnbase::IClassFactory>, rclsid: *const windows_core::GUID, pwzprotocol: &windows_core::PCWSTR, cpatterns: u32, ppwzpatterns: *const windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn UnregisterNameSpace(&self, pcf: windows_core::Ref<super::unknwnbase::IClassFactory>, pszprotocol: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RegisterMimeFilter(&self, pcf: windows_core::Ref<super::unknwnbase::IClassFactory>, rclsid: *const windows_core::GUID, pwztype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterMimeFilter(&self, pcf: windows_core::Ref<super::unknwnbase::IClassFactory>, pwztype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateBinding(&self, pbc: windows_core::Ref<super::objidl::IBindCtx>, szurl: &windows_core::PCWSTR, punkouter: windows_core::Ref<windows_core::IUnknown>, ppunk: windows_core::OutRef<windows_core::IUnknown>, ppoinetprot: windows_core::OutRef<IInternetProtocol>, dwoption: u32) -> windows_core::Result<()>;
    fn SetSessionOption(&self, dwoption: u32, pbuffer: *const core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
impl IInternetSession_Vtbl {
    pub const fn new<Identity: IInternetSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterNameSpace<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pwzprotocol: windows_core::PCWSTR, cpatterns: u32, ppwzpatterns: *const windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::RegisterNameSpace(this, core::mem::transmute_copy(&pcf), core::mem::transmute_copy(&rclsid), core::mem::transmute(&pwzprotocol), core::mem::transmute_copy(&cpatterns), core::mem::transmute_copy(&ppwzpatterns), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn UnregisterNameSpace<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, pszprotocol: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::UnregisterNameSpace(this, core::mem::transmute_copy(&pcf), core::mem::transmute(&pszprotocol)).into()
            }
        }
        unsafe extern "system" fn RegisterMimeFilter<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pwztype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::RegisterMimeFilter(this, core::mem::transmute_copy(&pcf), core::mem::transmute_copy(&rclsid), core::mem::transmute(&pwztype)).into()
            }
        }
        unsafe extern "system" fn UnregisterMimeFilter<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, pwztype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::UnregisterMimeFilter(this, core::mem::transmute_copy(&pcf), core::mem::transmute(&pwztype)).into()
            }
        }
        unsafe extern "system" fn CreateBinding<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, punkouter: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void, ppoinetprot: *mut *mut core::ffi::c_void, dwoption: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::CreateBinding(this, core::mem::transmute_copy(&pbc), core::mem::transmute(&szurl), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&ppunk), core::mem::transmute_copy(&ppoinetprot), core::mem::transmute_copy(&dwoption)).into()
            }
        }
        unsafe extern "system" fn SetSessionOption<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *const core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::SetSessionOption(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbufferlength), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetSessionOption<Identity: IInternetSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *mut core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetSession_Impl::GetSessionOption(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pdwbufferlength), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterNameSpace: RegisterNameSpace::<Identity, OFFSET>,
            UnregisterNameSpace: UnregisterNameSpace::<Identity, OFFSET>,
            RegisterMimeFilter: RegisterMimeFilter::<Identity, OFFSET>,
            UnregisterMimeFilter: UnregisterMimeFilter::<Identity, OFFSET>,
            CreateBinding: CreateBinding::<Identity, OFFSET>,
            SetSessionOption: SetSessionOption::<Identity, OFFSET>,
            GetSessionOption: GetSessionOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
impl windows_core::RuntimeName for IInternetSession {}
windows_core::imp::define_interface!(IInternetThreadSwitch, IInternetThreadSwitch_Vtbl, 0x79eac9e8_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetThreadSwitch, windows_core::IUnknown);
impl IInternetThreadSwitch {
    pub unsafe fn Prepare(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Continue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetThreadSwitch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternetThreadSwitch_Impl: windows_core::IUnknownImpl {
    fn Prepare(&self) -> windows_core::Result<()>;
    fn Continue(&self) -> windows_core::Result<()>;
}
impl IInternetThreadSwitch_Vtbl {
    pub const fn new<Identity: IInternetThreadSwitch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Prepare<Identity: IInternetThreadSwitch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetThreadSwitch_Impl::Prepare(this).into()
            }
        }
        unsafe extern "system" fn Continue<Identity: IInternetThreadSwitch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetThreadSwitch_Impl::Continue(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Prepare: Prepare::<Identity, OFFSET>, Continue: Continue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetThreadSwitch as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetThreadSwitch {}
windows_core::imp::define_interface!(IInternetZoneManager, IInternetZoneManager_Vtbl, 0x79eac9ef_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IInternetZoneManager, windows_core::IUnknown);
impl IInternetZoneManager {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneAttributes)(windows_core::Interface::as_raw(self), dwzone, pzoneattributes as _) }
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetZoneAttributes)(windows_core::Interface::as_raw(self), dwzone, pzoneattributes) }
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneCustomPolicy)(windows_core::Interface::as_raw(self), dwzone, guidkey, pppolicy as _, pcbpolicy as _, urlzonereg) }
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetZoneCustomPolicy)(windows_core::Interface::as_raw(self), dwzone, guidkey, ppolicy.as_ptr(), ppolicy.len().try_into().unwrap(), urlzonereg) }
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneActionPolicy)(windows_core::Interface::as_raw(self), dwzone, dwaction, ppolicy.as_mut_ptr(), ppolicy.len().try_into().unwrap(), urlzonereg) }
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetZoneActionPolicy)(windows_core::Interface::as_raw(self), dwzone, dwaction, ppolicy.as_ptr(), ppolicy.len().try_into().unwrap(), urlzonereg) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn PromptAction<P2, P3>(&self, dwaction: u32, hwndparent: super::windef::HWND, pwszurl: P2, pwsztext: P3, dwpromptflags: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PromptAction)(windows_core::Interface::as_raw(self), dwaction, hwndparent, pwszurl.param().abi(), pwsztext.param().abi(), dwpromptflags) }
    }
    pub unsafe fn LogAction<P1, P2>(&self, dwaction: u32, pwszurl: P1, pwsztext: P2, dwlogflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LogAction)(windows_core::Interface::as_raw(self), dwaction, pwszurl.param().abi(), pwsztext.param().abi(), dwlogflags) }
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateZoneEnumerator)(windows_core::Interface::as_raw(self), pdwenum as _, pdwcount as _, dwflags) }
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetZoneAt)(windows_core::Interface::as_raw(self), dwenum, dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DestroyZoneEnumerator)(windows_core::Interface::as_raw(self), dwenum) }
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyTemplatePoliciesToZone)(windows_core::Interface::as_raw(self), dwtemplate, dwzone, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetZoneAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ZONEATTRIBUTES) -> windows_core::HRESULT,
    pub SetZoneAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ZONEATTRIBUTES) -> windows_core::HRESULT,
    pub GetZoneCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut u8, *mut u32, URLZONEREG) -> windows_core::HRESULT,
    pub SetZoneCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const u8, u32, URLZONEREG) -> windows_core::HRESULT,
    pub GetZoneActionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, u32, URLZONEREG) -> windows_core::HRESULT,
    pub SetZoneActionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, URLZONEREG) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub PromptAction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::windef::HWND, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    PromptAction: usize,
    pub LogAction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub CreateZoneEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, u32) -> windows_core::HRESULT,
    pub GetZoneAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub DestroyZoneEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CopyTemplatePoliciesToZone: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IInternetZoneManager_Impl: windows_core::IUnknownImpl {
    fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> windows_core::Result<()>;
    fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> windows_core::Result<()>;
    fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn PromptAction(&self, dwaction: u32, hwndparent: super::windef::HWND, pwszurl: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR, dwpromptflags: u32) -> windows_core::Result<()>;
    fn LogAction(&self, dwaction: u32, pwszurl: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR, dwlogflags: u32) -> windows_core::Result<()>;
    fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> windows_core::Result<u32>;
    fn DestroyZoneEnumerator(&self, dwenum: u32) -> windows_core::Result<()>;
    fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IInternetZoneManager_Vtbl {
    pub const fn new<Identity: IInternetZoneManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetZoneAttributes<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::GetZoneAttributes(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&pzoneattributes)).into()
            }
        }
        unsafe extern "system" fn SetZoneAttributes<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::SetZoneAttributes(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&pzoneattributes)).into()
            }
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::GetZoneCustomPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
            }
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, guidkey: *const windows_core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::SetZoneCustomPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
            }
        }
        unsafe extern "system" fn GetZoneActionPolicy<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::GetZoneActionPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
            }
        }
        unsafe extern "system" fn SetZoneActionPolicy<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::SetZoneActionPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
            }
        }
        unsafe extern "system" fn PromptAction<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaction: u32, hwndparent: super::windef::HWND, pwszurl: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR, dwpromptflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::PromptAction(this, core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&pwszurl), core::mem::transmute(&pwsztext), core::mem::transmute_copy(&dwpromptflags)).into()
            }
        }
        unsafe extern "system" fn LogAction<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaction: u32, pwszurl: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR, dwlogflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::LogAction(this, core::mem::transmute_copy(&dwaction), core::mem::transmute(&pwszurl), core::mem::transmute(&pwsztext), core::mem::transmute_copy(&dwlogflags)).into()
            }
        }
        unsafe extern "system" fn CreateZoneEnumerator<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::CreateZoneEnumerator(this, core::mem::transmute_copy(&pdwenum), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetZoneAt<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInternetZoneManager_Impl::GetZoneAt(this, core::mem::transmute_copy(&dwenum), core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pdwzone.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenum: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::DestroyZoneEnumerator(this, core::mem::transmute_copy(&dwenum)).into()
            }
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Identity: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManager_Impl::CopyTemplatePoliciesToZone(this, core::mem::transmute_copy(&dwtemplate), core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetZoneAttributes: GetZoneAttributes::<Identity, OFFSET>,
            SetZoneAttributes: SetZoneAttributes::<Identity, OFFSET>,
            GetZoneCustomPolicy: GetZoneCustomPolicy::<Identity, OFFSET>,
            SetZoneCustomPolicy: SetZoneCustomPolicy::<Identity, OFFSET>,
            GetZoneActionPolicy: GetZoneActionPolicy::<Identity, OFFSET>,
            SetZoneActionPolicy: SetZoneActionPolicy::<Identity, OFFSET>,
            PromptAction: PromptAction::<Identity, OFFSET>,
            LogAction: LogAction::<Identity, OFFSET>,
            CreateZoneEnumerator: CreateZoneEnumerator::<Identity, OFFSET>,
            GetZoneAt: GetZoneAt::<Identity, OFFSET>,
            DestroyZoneEnumerator: DestroyZoneEnumerator::<Identity, OFFSET>,
            CopyTemplatePoliciesToZone: CopyTemplatePoliciesToZone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetZoneManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IInternetZoneManager {}
windows_core::imp::define_interface!(IInternetZoneManagerEx, IInternetZoneManagerEx_Vtbl, 0xa4c23339_8e06_431e_9bf4_7e711c085648);
impl core::ops::Deref for IInternetZoneManagerEx {
    type Target = IInternetZoneManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetZoneManagerEx, windows_core::IUnknown, IInternetZoneManager);
impl IInternetZoneManagerEx {
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneActionPolicyEx)(windows_core::Interface::as_raw(self), dwzone, dwaction, ppolicy.as_mut_ptr(), ppolicy.len().try_into().unwrap(), urlzonereg, dwflags) }
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetZoneActionPolicyEx)(windows_core::Interface::as_raw(self), dwzone, dwaction, ppolicy.as_ptr(), ppolicy.len().try_into().unwrap(), urlzonereg, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManagerEx_Vtbl {
    pub base__: IInternetZoneManager_Vtbl,
    pub GetZoneActionPolicyEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, u32, URLZONEREG, u32) -> windows_core::HRESULT,
    pub SetZoneActionPolicyEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, URLZONEREG, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IInternetZoneManagerEx_Impl: IInternetZoneManager_Impl {
    fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::Result<()>;
    fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IInternetZoneManagerEx_Vtbl {
    pub const fn new<Identity: IInternetZoneManagerEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetZoneActionPolicyEx<Identity: IInternetZoneManagerEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManagerEx_Impl::GetZoneActionPolicyEx(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Identity: IInternetZoneManagerEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManagerEx_Impl::SetZoneActionPolicyEx(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: IInternetZoneManager_Vtbl::new::<Identity, OFFSET>(),
            GetZoneActionPolicyEx: GetZoneActionPolicyEx::<Identity, OFFSET>,
            SetZoneActionPolicyEx: SetZoneActionPolicyEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx as windows_core::Interface>::IID || iid == &<IInternetZoneManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IInternetZoneManagerEx {}
windows_core::imp::define_interface!(IInternetZoneManagerEx2, IInternetZoneManagerEx2_Vtbl, 0xedc17559_dd5d_4846_8eef_8becba5a4abf);
impl core::ops::Deref for IInternetZoneManagerEx2 {
    type Target = IInternetZoneManagerEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetZoneManagerEx2, windows_core::IUnknown, IInternetZoneManager, IInternetZoneManagerEx);
impl IInternetZoneManagerEx2 {
    pub unsafe fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneAttributesEx)(windows_core::Interface::as_raw(self), dwzone, pzoneattributes as _, dwflags) }
    }
    pub unsafe fn GetZoneSecurityState(&self, dwzoneindex: u32, frespectpolicy: bool, pdwstate: *mut u32, pfpolicyencountered: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetZoneSecurityState)(windows_core::Interface::as_raw(self), dwzoneindex, frespectpolicy.into(), pdwstate as _, pfpolicyencountered as _) }
    }
    pub unsafe fn GetIESecurityState(&self, frespectpolicy: bool, pdwstate: *mut u32, pfpolicyencountered: *mut windows_core::BOOL, fnocache: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIESecurityState)(windows_core::Interface::as_raw(self), frespectpolicy.into(), pdwstate as _, pfpolicyencountered as _, fnocache.into()) }
    }
    pub unsafe fn FixUnsecureSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FixUnsecureSettings)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManagerEx2_Vtbl {
    pub base__: IInternetZoneManagerEx_Vtbl,
    pub GetZoneAttributesEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ZONEATTRIBUTES, u32) -> windows_core::HRESULT,
    pub GetZoneSecurityState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetIESecurityState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut u32, *mut windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub FixUnsecureSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IInternetZoneManagerEx2_Impl: IInternetZoneManagerEx_Impl {
    fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> windows_core::Result<()>;
    fn GetZoneSecurityState(&self, dwzoneindex: u32, frespectpolicy: windows_core::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetIESecurityState(&self, frespectpolicy: windows_core::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut windows_core::BOOL, fnocache: windows_core::BOOL) -> windows_core::Result<()>;
    fn FixUnsecureSettings(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IInternetZoneManagerEx2_Vtbl {
    pub const fn new<Identity: IInternetZoneManagerEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetZoneAttributesEx<Identity: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManagerEx2_Impl::GetZoneAttributesEx(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&pzoneattributes), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetZoneSecurityState<Identity: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzoneindex: u32, frespectpolicy: windows_core::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManagerEx2_Impl::GetZoneSecurityState(this, core::mem::transmute_copy(&dwzoneindex), core::mem::transmute_copy(&frespectpolicy), core::mem::transmute_copy(&pdwstate), core::mem::transmute_copy(&pfpolicyencountered)).into()
            }
        }
        unsafe extern "system" fn GetIESecurityState<Identity: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frespectpolicy: windows_core::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut windows_core::BOOL, fnocache: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManagerEx2_Impl::GetIESecurityState(this, core::mem::transmute_copy(&frespectpolicy), core::mem::transmute_copy(&pdwstate), core::mem::transmute_copy(&pfpolicyencountered), core::mem::transmute_copy(&fnocache)).into()
            }
        }
        unsafe extern "system" fn FixUnsecureSettings<Identity: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetZoneManagerEx2_Impl::FixUnsecureSettings(this).into()
            }
        }
        Self {
            base__: IInternetZoneManagerEx_Vtbl::new::<Identity, OFFSET>(),
            GetZoneAttributesEx: GetZoneAttributesEx::<Identity, OFFSET>,
            GetZoneSecurityState: GetZoneSecurityState::<Identity, OFFSET>,
            GetIESecurityState: GetIESecurityState::<Identity, OFFSET>,
            FixUnsecureSettings: FixUnsecureSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx2 as windows_core::Interface>::IID || iid == &<IInternetZoneManager as windows_core::Interface>::IID || iid == &<IInternetZoneManagerEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IInternetZoneManagerEx2 {}
windows_core::imp::define_interface!(IMonikerProp, IMonikerProp_Vtbl, 0xa5ca5f7f_1847_4d87_9c5b_918509f7511d);
windows_core::imp::interface_hierarchy!(IMonikerProp, windows_core::IUnknown);
impl IMonikerProp {
    pub unsafe fn PutProperty<P1>(&self, mkp: MONIKERPROPERTY, val: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), mkp, val.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMonikerProp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, MONIKERPROPERTY, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IMonikerProp_Impl: windows_core::IUnknownImpl {
    fn PutProperty(&self, mkp: MONIKERPROPERTY, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IMonikerProp_Vtbl {
    pub const fn new<Identity: IMonikerProp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PutProperty<Identity: IMonikerProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mkp: MONIKERPROPERTY, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonikerProp_Impl::PutProperty(this, core::mem::transmute_copy(&mkp), core::mem::transmute(&val)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PutProperty: PutProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonikerProp as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMonikerProp {}
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: windows_core::HRESULT = windows_core::HRESULT(0x800C0506_u32 as _);
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: windows_core::HRESULT = windows_core::HRESULT(0x800C0505_u32 as _);
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: windows_core::HRESULT = windows_core::HRESULT(0x800C001B_u32 as _);
pub const INET_E_CANNOT_LOCK_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x800C0016_u32 as _);
pub const INET_E_CANNOT_REPLACE_SFP_FILE: windows_core::HRESULT = windows_core::HRESULT(0x800C0300_u32 as _);
pub const INET_E_CODE_DOWNLOAD_DECLINED: windows_core::HRESULT = windows_core::HRESULT(0x800C0100_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: windows_core::HRESULT = windows_core::HRESULT(0x800C0504_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: windows_core::HRESULT = windows_core::HRESULT(0x800C0507_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x800C0500_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: windows_core::HRESULT = windows_core::HRESULT(0x800C0502_u32 as _);
pub const INET_E_CODE_INSTALL_SUPPRESSED: windows_core::HRESULT = windows_core::HRESULT(0x800C0400_u32 as _);
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199;
pub const INET_E_DOMINJECTIONVALIDATION: windows_core::HRESULT = windows_core::HRESULT(0x800C001C_u32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: windows_core::HRESULT = windows_core::HRESULT(0x800C0508_u32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: windows_core::HRESULT = windows_core::HRESULT(0x800C0501_u32 as _);
pub const INET_E_ERROR_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x800C0002_u32 as _);
pub const INET_E_ERROR_LAST: i32 = -2146695928;
pub const INET_E_FORBIDFRAMING: windows_core::HRESULT = windows_core::HRESULT(0x800C0503_u32 as _);
pub const INET_E_HSTS_CERTIFICATE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x800C001E_u32 as _);
pub const INET_E_QUERYOPTION_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x800C0013_u32 as _);
pub const INET_E_REDIRECTING: windows_core::HRESULT = windows_core::HRESULT(0x800C0014_u32 as _);
pub const INET_E_RESERVED_1: windows_core::HRESULT = windows_core::HRESULT(0x800C001A_u32 as _);
pub const INET_E_RESERVED_2: windows_core::HRESULT = windows_core::HRESULT(0x800C001F_u32 as _);
pub const INET_E_RESERVED_3: windows_core::HRESULT = windows_core::HRESULT(0x800C0020_u32 as _);
pub const INET_E_RESERVED_4: windows_core::HRESULT = windows_core::HRESULT(0x800C0021_u32 as _);
pub const INET_E_RESERVED_5: windows_core::HRESULT = windows_core::HRESULT(0x800C0022_u32 as _);
pub const INET_E_RESULT_DISPATCHED: windows_core::HRESULT = windows_core::HRESULT(0x800C0200_u32 as _);
pub const INET_E_TERMINATED_BIND: windows_core::HRESULT = windows_core::HRESULT(0x800C0018_u32 as _);
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: windows_core::HRESULT = windows_core::HRESULT(0x800C0011_u32 as _);
pub const INET_E_USE_DEFAULT_SETTING: windows_core::HRESULT = windows_core::HRESULT(0x800C0012_u32 as _);
pub const INET_E_USE_EXTEND_BINDING: windows_core::HRESULT = windows_core::HRESULT(0x800C0017_u32 as _);
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: windows_core::HRESULT = windows_core::HRESULT(0x800C001D_u32 as _);
pub type INTERNETFEATURELIST = i32;
windows_core::imp::define_interface!(IPersistMoniker, IPersistMoniker_Vtbl, 0x79eac9c9_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IPersistMoniker, windows_core::IUnknown);
impl IPersistMoniker {
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn Load<P1, P2>(&self, ffullyavailable: bool, pimkname: P1, pibc: P2, grfmode: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IMoniker>,
        P2: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ffullyavailable.into(), pimkname.param().abi(), pibc.param().abi(), grfmode) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn Save<P0, P1>(&self, pimkname: P0, pbc: P1, fremember: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IMoniker>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pimkname.param().abi(), pbc.param().abi(), fremember.into()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SaveCompleted<P0, P1>(&self, pimkname: P0, pibc: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IMoniker>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pimkname.param().abi(), pibc.param().abi()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetCurMoniker(&self) -> windows_core::Result<super::objidl::IMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurMoniker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMoniker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    Load: usize,
    #[cfg(feature = "objidl")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    Save: usize,
    #[cfg(feature = "objidl")]
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SaveCompleted: usize,
    #[cfg(feature = "objidl")]
    pub GetCurMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetCurMoniker: usize,
}
#[cfg(feature = "objidl")]
pub trait IPersistMoniker_Impl: windows_core::IUnknownImpl {
    fn GetClassID(&self) -> windows_core::Result<windows_core::GUID>;
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, ffullyavailable: windows_core::BOOL, pimkname: windows_core::Ref<super::objidl::IMoniker>, pibc: windows_core::Ref<super::objidl::IBindCtx>, grfmode: u32) -> windows_core::Result<()>;
    fn Save(&self, pimkname: windows_core::Ref<super::objidl::IMoniker>, pbc: windows_core::Ref<super::objidl::IBindCtx>, fremember: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pimkname: windows_core::Ref<super::objidl::IMoniker>, pibc: windows_core::Ref<super::objidl::IBindCtx>) -> windows_core::Result<()>;
    fn GetCurMoniker(&self) -> windows_core::Result<super::objidl::IMoniker>;
}
#[cfg(feature = "objidl")]
impl IPersistMoniker_Vtbl {
    pub const fn new<Identity: IPersistMoniker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassID<Identity: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistMoniker_Impl::GetClassID(this) {
                    Ok(ok__) => {
                        pclassid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDirty<Identity: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMoniker_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffullyavailable: windows_core::BOOL, pimkname: *mut core::ffi::c_void, pibc: *mut core::ffi::c_void, grfmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMoniker_Impl::Load(this, core::mem::transmute_copy(&ffullyavailable), core::mem::transmute_copy(&pimkname), core::mem::transmute_copy(&pibc), core::mem::transmute_copy(&grfmode)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimkname: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, fremember: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMoniker_Impl::Save(this, core::mem::transmute_copy(&pimkname), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&fremember)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimkname: *mut core::ffi::c_void, pibc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMoniker_Impl::SaveCompleted(this, core::mem::transmute_copy(&pimkname), core::mem::transmute_copy(&pibc)).into()
            }
        }
        unsafe extern "system" fn GetCurMoniker<Identity: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimkname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistMoniker_Impl::GetCurMoniker(this) {
                    Ok(ok__) => {
                        ppimkname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClassID: GetClassID::<Identity, OFFSET>,
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            GetCurMoniker: GetCurMoniker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistMoniker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IPersistMoniker {}
windows_core::imp::define_interface!(ISoftDistExt, ISoftDistExt_Vtbl, 0xb15b8dc1_c7e1_11d0_8680_00aa00bdcb71);
windows_core::imp::interface_hierarchy!(ISoftDistExt, windows_core::IUnknown);
impl ISoftDistExt {
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub unsafe fn ProcessSoftDist<P0, P1>(&self, szcdfurl: P0, psoftdistelement: P1, lpsdi: *mut SOFTDISTINFO) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::msxml::IXMLElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessSoftDist)(windows_core::Interface::as_raw(self), szcdfurl.param().abi(), psoftdistelement.param().abi(), lpsdi as _) }
    }
    pub unsafe fn GetFirstCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFirstCodeBase)(windows_core::Interface::as_raw(self), szcodebase, dwmaxsize) }
    }
    pub unsafe fn GetNextCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextCodeBase)(windows_core::Interface::as_raw(self), szcodebase, dwmaxsize) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn AsyncInstallDistributionUnit<P0>(&self, pbc: P0, pvreserved: *const core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncInstallDistributionUnit)(windows_core::Interface::as_raw(self), pbc.param().abi(), pvreserved, flags, lpcbh) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftDistExt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub ProcessSoftDist: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut SOFTDISTINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "msxml", feature = "oaidl")))]
    ProcessSoftDist: usize,
    pub GetFirstCodeBase: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, *const u32) -> windows_core::HRESULT,
    pub GetNextCodeBase: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, *const u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub AsyncInstallDistributionUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *const CODEBASEHOLD) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    AsyncInstallDistributionUnit: usize,
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "objidl"))]
pub trait ISoftDistExt_Impl: windows_core::IUnknownImpl {
    fn ProcessSoftDist(&self, szcdfurl: &windows_core::PCWSTR, psoftdistelement: windows_core::Ref<super::msxml::IXMLElement>, lpsdi: *mut SOFTDISTINFO) -> windows_core::Result<()>;
    fn GetFirstCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::Result<()>;
    fn GetNextCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::Result<()>;
    fn AsyncInstallDistributionUnit(&self, pbc: windows_core::Ref<super::objidl::IBindCtx>, pvreserved: *const core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "objidl"))]
impl ISoftDistExt_Vtbl {
    pub const fn new<Identity: ISoftDistExt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessSoftDist<Identity: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcdfurl: windows_core::PCWSTR, psoftdistelement: *mut core::ffi::c_void, lpsdi: *mut SOFTDISTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftDistExt_Impl::ProcessSoftDist(this, core::mem::transmute(&szcdfurl), core::mem::transmute_copy(&psoftdistelement), core::mem::transmute_copy(&lpsdi)).into()
            }
        }
        unsafe extern "system" fn GetFirstCodeBase<Identity: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftDistExt_Impl::GetFirstCodeBase(this, core::mem::transmute_copy(&szcodebase), core::mem::transmute_copy(&dwmaxsize)).into()
            }
        }
        unsafe extern "system" fn GetNextCodeBase<Identity: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftDistExt_Impl::GetNextCodeBase(this, core::mem::transmute_copy(&szcodebase), core::mem::transmute_copy(&dwmaxsize)).into()
            }
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Identity: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pvreserved: *const core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftDistExt_Impl::AsyncInstallDistributionUnit(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pvreserved), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&lpcbh)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProcessSoftDist: ProcessSoftDist::<Identity, OFFSET>,
            GetFirstCodeBase: GetFirstCodeBase::<Identity, OFFSET>,
            GetNextCodeBase: GetNextCodeBase::<Identity, OFFSET>,
            AsyncInstallDistributionUnit: AsyncInstallDistributionUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISoftDistExt as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "objidl"))]
impl windows_core::RuntimeName for ISoftDistExt {}
windows_core::imp::define_interface!(IUri, IUri_Vtbl, 0xa39ee748_6a27_4817_a6f2_13914bef5890);
windows_core::imp::interface_hierarchy!(IUri, windows_core::IUnknown);
impl IUri {
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut windows_core::BSTR, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyBSTR)(windows_core::Interface::as_raw(self), uriprop, core::mem::transmute(pbstrproperty), dwflags) }
    }
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyLength)(windows_core::Interface::as_raw(self), uriprop, pcchproperty as _, dwflags) }
    }
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyDWORD)(windows_core::Interface::as_raw(self), uriprop, pdwproperty as _, dwflags) }
    }
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasProperty)(windows_core::Interface::as_raw(self), uriprop, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAbsoluteUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAbsoluteUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAuthority(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthority)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFragment(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFragment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHost(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPassword(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPathAndQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPathAndQuery)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRawUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRawUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSchemeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSchemeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUserInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHostType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPort(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScheme(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetZone(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetZone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), puri.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyBSTR: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPropertyLength: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut u32, u32) -> windows_core::HRESULT,
    pub GetPropertyDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut u32, u32) -> windows_core::HRESULT,
    pub HasProperty: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAbsoluteUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAuthority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPathAndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRawUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHostType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUri_Impl: windows_core::IUnknownImpl {
    fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut windows_core::BSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn HasProperty(&self, uriprop: Uri_PROPERTY) -> windows_core::Result<windows_core::BOOL>;
    fn GetAbsoluteUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthority(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFragment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHost(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPassword(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPathAndQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRawUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSchemeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHostType(&self) -> windows_core::Result<u32>;
    fn GetPort(&self) -> windows_core::Result<u32>;
    fn GetScheme(&self) -> windows_core::Result<u32>;
    fn GetZone(&self) -> windows_core::Result<u32>;
    fn GetProperties(&self) -> windows_core::Result<u32>;
    fn IsEqual(&self, puri: windows_core::Ref<IUri>) -> windows_core::Result<windows_core::BOOL>;
}
impl IUri_Vtbl {
    pub const fn new<Identity: IUri_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyBSTR<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUri_Impl::GetPropertyBSTR(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pbstrproperty), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetPropertyLength<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUri_Impl::GetPropertyLength(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pcchproperty), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetPropertyDWORD<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUri_Impl::GetPropertyDWORD(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pdwproperty), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn HasProperty<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::HasProperty(this, core::mem::transmute_copy(&uriprop)) {
                    Ok(ok__) => {
                        pfhasproperty.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAbsoluteUri<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrabsoluteuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetAbsoluteUri(this) {
                    Ok(ok__) => {
                        pbstrabsoluteuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAuthority<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthority: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetAuthority(this) {
                    Ok(ok__) => {
                        pbstrauthority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayUri<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplaystring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetDisplayUri(this) {
                    Ok(ok__) => {
                        pbstrdisplaystring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDomain<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetDomain(this) {
                    Ok(ok__) => {
                        pbstrdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExtension<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextension: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetExtension(this) {
                    Ok(ok__) => {
                        pbstrextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFragment<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfragment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetFragment(this) {
                    Ok(ok__) => {
                        pbstrfragment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHost<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhost: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetHost(this) {
                    Ok(ok__) => {
                        pbstrhost.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPassword<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpassword: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPassword(this) {
                    Ok(ok__) => {
                        pbstrpassword.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPath<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPath(this) {
                    Ok(ok__) => {
                        pbstrpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPathAndQuery<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathandquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPathAndQuery(this) {
                    Ok(ok__) => {
                        pbstrpathandquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQuery<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetQuery(this) {
                    Ok(ok__) => {
                        pbstrquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRawUri<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrawuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetRawUri(this) {
                    Ok(ok__) => {
                        pbstrrawuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSchemeName<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrschemename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetSchemeName(this) {
                    Ok(ok__) => {
                        pbstrschemename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserInfo<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruserinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetUserInfo(this) {
                    Ok(ok__) => {
                        pbstruserinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserName<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetUserName(this) {
                    Ok(ok__) => {
                        pbstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHostType<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhosttype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetHostType(this) {
                    Ok(ok__) => {
                        pdwhosttype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPort<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPort(this) {
                    Ok(ok__) => {
                        pdwport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScheme<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscheme: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetScheme(this) {
                    Ok(ok__) => {
                        pdwscheme.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetZone<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwzone: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetZone(this) {
                    Ok(ok__) => {
                        pdwzone.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::IsEqual(this, core::mem::transmute_copy(&puri)) {
                    Ok(ok__) => {
                        pfequal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyBSTR: GetPropertyBSTR::<Identity, OFFSET>,
            GetPropertyLength: GetPropertyLength::<Identity, OFFSET>,
            GetPropertyDWORD: GetPropertyDWORD::<Identity, OFFSET>,
            HasProperty: HasProperty::<Identity, OFFSET>,
            GetAbsoluteUri: GetAbsoluteUri::<Identity, OFFSET>,
            GetAuthority: GetAuthority::<Identity, OFFSET>,
            GetDisplayUri: GetDisplayUri::<Identity, OFFSET>,
            GetDomain: GetDomain::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
            GetFragment: GetFragment::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetPathAndQuery: GetPathAndQuery::<Identity, OFFSET>,
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetRawUri: GetRawUri::<Identity, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, OFFSET>,
            GetUserInfo: GetUserInfo::<Identity, OFFSET>,
            GetUserName: GetUserName::<Identity, OFFSET>,
            GetHostType: GetHostType::<Identity, OFFSET>,
            GetPort: GetPort::<Identity, OFFSET>,
            GetScheme: GetScheme::<Identity, OFFSET>,
            GetZone: GetZone::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUri as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUri {}
windows_core::imp::define_interface!(IUriBuilder, IUriBuilder_Vtbl, 0x4221b2e1_8955_46c0_bd5b_de9897565de7);
windows_core::imp::interface_hierarchy!(IUriBuilder, windows_core::IUnknown);
impl IUriBuilder {
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUriSimple)(windows_core::Interface::as_raw(self), dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUri)(windows_core::Interface::as_raw(self), dwcreateflags, dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUriWithFlags)(windows_core::Interface::as_raw(self), dwcreateflags, dwuribuilderflags, dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIUri(&self) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetIUri<P0>(&self, piuri: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIUri)(windows_core::Interface::as_raw(self), piuri.param().abi()) }
    }
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFragment)(windows_core::Interface::as_raw(self), pcchfragment as _, ppwzfragment as _) }
    }
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), pcchhost as _, ppwzhost as _) }
    }
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), pcchpassword as _, ppwzpassword as _) }
    }
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), pcchpath as _, ppwzpath as _) }
    }
    pub unsafe fn GetPort(&self, pfhasport: *mut windows_core::BOOL, pdwport: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), pfhasport as _, pdwport as _) }
    }
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), pcchquery as _, ppwzquery as _) }
    }
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSchemeName)(windows_core::Interface::as_raw(self), pcchschemename as _, ppwzschemename as _) }
    }
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUserName)(windows_core::Interface::as_raw(self), pcchusername as _, ppwzusername as _) }
    }
    pub unsafe fn SetFragment<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFragment)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn SetHost<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHost)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn SetPassword<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn SetPath<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn SetPort(&self, fhasport: bool, dwnewvalue: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), fhasport.into(), dwnewvalue) }
    }
    pub unsafe fn SetQuery<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQuery)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn SetSchemeName<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchemeName)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn SetUserName<P0>(&self, pwznewvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()) }
    }
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveProperties)(windows_core::Interface::as_raw(self), dwpropertymask) }
    }
    pub unsafe fn HasBeenModified(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasBeenModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateUriSimple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUri: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUriWithFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHost: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HasBeenModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUriBuilder_Impl: windows_core::IUnknownImpl {
    fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn GetIUri(&self) -> windows_core::Result<IUri>;
    fn SetIUri(&self, piuri: windows_core::Ref<IUri>) -> windows_core::Result<()>;
    fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPort(&self, pfhasport: *mut windows_core::BOOL, pdwport: *mut u32) -> windows_core::Result<()>;
    fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFragment(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHost(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPath(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPort(&self, fhasport: windows_core::BOOL, dwnewvalue: u32) -> windows_core::Result<()>;
    fn SetQuery(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSchemeName(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetUserName(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveProperties(&self, dwpropertymask: u32) -> windows_core::Result<()>;
    fn HasBeenModified(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUriBuilder_Vtbl {
    pub const fn new<Identity: IUriBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateUriSimple<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::CreateUriSimple(this, core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUri<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::CreateUri(this, core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUriWithFlags<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::CreateUriWithFlags(this, core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&dwuribuilderflags), core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIUri<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::GetIUri(this) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIUri<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetIUri(this, core::mem::transmute_copy(&piuri)).into()
            }
        }
        unsafe extern "system" fn GetFragment<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetFragment(this, core::mem::transmute_copy(&pcchfragment), core::mem::transmute_copy(&ppwzfragment)).into()
            }
        }
        unsafe extern "system" fn GetHost<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetHost(this, core::mem::transmute_copy(&pcchhost), core::mem::transmute_copy(&ppwzhost)).into()
            }
        }
        unsafe extern "system" fn GetPassword<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetPassword(this, core::mem::transmute_copy(&pcchpassword), core::mem::transmute_copy(&ppwzpassword)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetPath(this, core::mem::transmute_copy(&pcchpath), core::mem::transmute_copy(&ppwzpath)).into()
            }
        }
        unsafe extern "system" fn GetPort<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasport: *mut windows_core::BOOL, pdwport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetPort(this, core::mem::transmute_copy(&pfhasport), core::mem::transmute_copy(&pdwport)).into()
            }
        }
        unsafe extern "system" fn GetQuery<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetQuery(this, core::mem::transmute_copy(&pcchquery), core::mem::transmute_copy(&ppwzquery)).into()
            }
        }
        unsafe extern "system" fn GetSchemeName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetSchemeName(this, core::mem::transmute_copy(&pcchschemename), core::mem::transmute_copy(&ppwzschemename)).into()
            }
        }
        unsafe extern "system" fn GetUserName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetUserName(this, core::mem::transmute_copy(&pcchusername), core::mem::transmute_copy(&ppwzusername)).into()
            }
        }
        unsafe extern "system" fn SetFragment<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetFragment(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetHost<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetHost(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetPassword(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetPath<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetPath(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetPort<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fhasport: windows_core::BOOL, dwnewvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetPort(this, core::mem::transmute_copy(&fhasport), core::mem::transmute_copy(&dwnewvalue)).into()
            }
        }
        unsafe extern "system" fn SetQuery<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetQuery(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetSchemeName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetSchemeName(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetUserName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetUserName(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn RemoveProperties<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpropertymask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::RemoveProperties(this, core::mem::transmute_copy(&dwpropertymask)).into()
            }
        }
        unsafe extern "system" fn HasBeenModified<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmodified: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::HasBeenModified(this) {
                    Ok(ok__) => {
                        pfmodified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateUriSimple: CreateUriSimple::<Identity, OFFSET>,
            CreateUri: CreateUri::<Identity, OFFSET>,
            CreateUriWithFlags: CreateUriWithFlags::<Identity, OFFSET>,
            GetIUri: GetIUri::<Identity, OFFSET>,
            SetIUri: SetIUri::<Identity, OFFSET>,
            GetFragment: GetFragment::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetPort: GetPort::<Identity, OFFSET>,
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, OFFSET>,
            GetUserName: GetUserName::<Identity, OFFSET>,
            SetFragment: SetFragment::<Identity, OFFSET>,
            SetHost: SetHost::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            SetQuery: SetQuery::<Identity, OFFSET>,
            SetSchemeName: SetSchemeName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            RemoveProperties: RemoveProperties::<Identity, OFFSET>,
            HasBeenModified: HasBeenModified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriBuilder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUriBuilder {}
windows_core::imp::define_interface!(IUriBuilderFactory, IUriBuilderFactory_Vtbl, 0xe982ce48_0b96_440c_bc37_0c869b27a29e);
windows_core::imp::interface_hierarchy!(IUriBuilderFactory, windows_core::IUnknown);
impl IUriBuilderFactory {
    pub unsafe fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateIUriBuilder)(windows_core::Interface::as_raw(self), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInitializedIUriBuilder)(windows_core::Interface::as_raw(self), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilderFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateIUriBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInitializedIUriBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUriBuilderFactory_Impl: windows_core::IUnknownImpl {
    fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder>;
    fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder>;
}
impl IUriBuilderFactory_Vtbl {
    pub const fn new<Identity: IUriBuilderFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateIUriBuilder<Identity: IUriBuilderFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilderFactory_Impl::CreateIUriBuilder(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuribuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Identity: IUriBuilderFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilderFactory_Impl::CreateInitializedIUriBuilder(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuribuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIUriBuilder: CreateIUriBuilder::<Identity, OFFSET>,
            CreateInitializedIUriBuilder: CreateInitializedIUriBuilder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriBuilderFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUriBuilderFactory {}
windows_core::imp::define_interface!(IUriContainer, IUriContainer_Vtbl, 0xa158a630_ed6f_45fb_b987_f68676f57752);
windows_core::imp::interface_hierarchy!(IUriContainer, windows_core::IUnknown);
impl IUriContainer {
    pub unsafe fn GetIUri(&self) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUriContainer_Impl: windows_core::IUnknownImpl {
    fn GetIUri(&self) -> windows_core::Result<IUri>;
}
impl IUriContainer_Vtbl {
    pub const fn new<Identity: IUriContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIUri<Identity: IUriContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriContainer_Impl::GetIUri(this) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIUri: GetIUri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriContainer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUriContainer {}
windows_core::imp::define_interface!(IWinInetCacheHints, IWinInetCacheHints_Vtbl, 0xdd1ec3b3_8391_4fdb_a9e6_347c3caaa7dd);
windows_core::imp::interface_hierarchy!(IWinInetCacheHints, windows_core::IUnknown);
impl IWinInetCacheHints {
    pub unsafe fn SetCacheExtension<P0>(&self, pwzext: P0, pszcachefile: *mut core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCacheExtension)(windows_core::Interface::as_raw(self), pwzext.param().abi(), pszcachefile as _, pcbcachefile as _, pdwwinineterror as _, pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetCacheHints_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCacheExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IWinInetCacheHints_Impl: windows_core::IUnknownImpl {
    fn SetCacheExtension(&self, pwzext: &windows_core::PCWSTR, pszcachefile: *mut core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl IWinInetCacheHints_Vtbl {
    pub const fn new<Identity: IWinInetCacheHints_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCacheExtension<Identity: IWinInetCacheHints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzext: windows_core::PCWSTR, pszcachefile: *mut core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetCacheHints_Impl::SetCacheExtension(this, core::mem::transmute(&pwzext), core::mem::transmute_copy(&pszcachefile), core::mem::transmute_copy(&pcbcachefile), core::mem::transmute_copy(&pdwwinineterror), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetCacheExtension: SetCacheExtension::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetCacheHints as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinInetCacheHints {}
windows_core::imp::define_interface!(IWinInetCacheHints2, IWinInetCacheHints2_Vtbl, 0x7857aeac_d31f_49bf_884e_dd46df36780a);
impl core::ops::Deref for IWinInetCacheHints2 {
    type Target = IWinInetCacheHints;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetCacheHints2, windows_core::IUnknown, IWinInetCacheHints);
impl IWinInetCacheHints2 {
    pub unsafe fn SetCacheExtension2<P0>(&self, pwzext: P0, pwzcachefile: *mut u16, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCacheExtension2)(windows_core::Interface::as_raw(self), pwzext.param().abi(), pwzcachefile as _, pcchcachefile as _, pdwwinineterror as _, pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetCacheHints2_Vtbl {
    pub base__: IWinInetCacheHints_Vtbl,
    pub SetCacheExtension2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u16, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IWinInetCacheHints2_Impl: IWinInetCacheHints_Impl {
    fn SetCacheExtension2(&self, pwzext: &windows_core::PCWSTR, pwzcachefile: *mut u16, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl IWinInetCacheHints2_Vtbl {
    pub const fn new<Identity: IWinInetCacheHints2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCacheExtension2<Identity: IWinInetCacheHints2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzext: windows_core::PCWSTR, pwzcachefile: *mut u16, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetCacheHints2_Impl::SetCacheExtension2(this, core::mem::transmute(&pwzext), core::mem::transmute_copy(&pwzcachefile), core::mem::transmute_copy(&pcchcachefile), core::mem::transmute_copy(&pdwwinineterror), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IWinInetCacheHints_Vtbl::new::<Identity, OFFSET>(), SetCacheExtension2: SetCacheExtension2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetCacheHints2 as windows_core::Interface>::IID || iid == &<IWinInetCacheHints as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinInetCacheHints2 {}
windows_core::imp::define_interface!(IWinInetFileStream, IWinInetFileStream_Vtbl, 0xf134c4b7_b1f8_4e75_b886_74b90943becb);
windows_core::imp::interface_hierarchy!(IWinInetFileStream, windows_core::IUnknown);
impl IWinInetFileStream {
    pub unsafe fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHandleForUnlock)(windows_core::Interface::as_raw(self), hwininetlockhandle, dwreserved) }
    }
    pub unsafe fn SetDeleteFile(&self, dwreserved: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDeleteFile)(windows_core::Interface::as_raw(self), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetFileStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHandleForUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub SetDeleteFile: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait IWinInetFileStream_Impl: windows_core::IUnknownImpl {
    fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> windows_core::Result<()>;
    fn SetDeleteFile(&self, dwreserved: usize) -> windows_core::Result<()>;
}
impl IWinInetFileStream_Vtbl {
    pub const fn new<Identity: IWinInetFileStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHandleForUnlock<Identity: IWinInetFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetFileStream_Impl::SetHandleForUnlock(this, core::mem::transmute_copy(&hwininetlockhandle), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SetDeleteFile<Identity: IWinInetFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetFileStream_Impl::SetDeleteFile(this, core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHandleForUnlock: SetHandleForUnlock::<Identity, OFFSET>,
            SetDeleteFile: SetDeleteFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetFileStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinInetFileStream {}
windows_core::imp::define_interface!(IWinInetHttpInfo, IWinInetHttpInfo_Vtbl, 0x79eac9d8_bafa_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IWinInetHttpInfo {
    type Target = IWinInetInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetHttpInfo, windows_core::IUnknown, IWinInetInfo);
impl IWinInetHttpInfo {
    pub unsafe fn QueryInfo(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInfo)(windows_core::Interface::as_raw(self), dwoption, pbuffer as _, pcbbuf as _, pdwflags as _, pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetHttpInfo_Vtbl {
    pub base__: IWinInetInfo_Vtbl,
    pub QueryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IWinInetHttpInfo_Impl: IWinInetInfo_Impl {
    fn QueryInfo(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl IWinInetHttpInfo_Vtbl {
    pub const fn new<Identity: IWinInetHttpInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInfo<Identity: IWinInetHttpInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetHttpInfo_Impl::QueryInfo(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbbuf), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IWinInetInfo_Vtbl::new::<Identity, OFFSET>(), QueryInfo: QueryInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetHttpInfo as windows_core::Interface>::IID || iid == &<IWinInetInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinInetHttpInfo {}
windows_core::imp::define_interface!(IWinInetHttpTimeouts, IWinInetHttpTimeouts_Vtbl, 0xf286fa56_c1fd_4270_8e67_b3eb790a81e8);
windows_core::imp::interface_hierarchy!(IWinInetHttpTimeouts, windows_core::IUnknown);
impl IWinInetHttpTimeouts {
    pub unsafe fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRequestTimeouts)(windows_core::Interface::as_raw(self), pdwconnecttimeout as _, pdwsendtimeout as _, pdwreceivetimeout as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetHttpTimeouts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequestTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IWinInetHttpTimeouts_Impl: windows_core::IUnknownImpl {
    fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> windows_core::Result<()>;
}
impl IWinInetHttpTimeouts_Vtbl {
    pub const fn new<Identity: IWinInetHttpTimeouts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRequestTimeouts<Identity: IWinInetHttpTimeouts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetHttpTimeouts_Impl::GetRequestTimeouts(this, core::mem::transmute_copy(&pdwconnecttimeout), core::mem::transmute_copy(&pdwsendtimeout), core::mem::transmute_copy(&pdwreceivetimeout)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRequestTimeouts: GetRequestTimeouts::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetHttpTimeouts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinInetHttpTimeouts {}
windows_core::imp::define_interface!(IWinInetInfo, IWinInetInfo_Vtbl, 0x79eac9d6_bafa_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IWinInetInfo, windows_core::IUnknown);
impl IWinInetInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryOption)(windows_core::Interface::as_raw(self), dwoption, pbuffer as _, pcbbuf as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryOption: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWinInetInfo_Impl: windows_core::IUnknownImpl {
    fn QueryOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32) -> windows_core::Result<()>;
}
impl IWinInetInfo_Vtbl {
    pub const fn new<Identity: IWinInetInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryOption<Identity: IWinInetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinInetInfo_Impl::QueryOption(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbbuf)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryOption: QueryOption::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinInetInfo {}
windows_core::imp::define_interface!(IWindowForBindingUI, IWindowForBindingUI_Vtbl, 0x79eac9d5_bafa_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IWindowForBindingUI, windows_core::IUnknown);
impl IWindowForBindingUI {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindow(&self, rguidreason: *const windows_core::GUID) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), rguidreason, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowForBindingUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IWindowForBindingUI_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self, rguidreason: *const windows_core::GUID) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(feature = "windef")]
impl IWindowForBindingUI_Vtbl {
    pub const fn new<Identity: IWindowForBindingUI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IWindowForBindingUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidreason: *const windows_core::GUID, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowForBindingUI_Impl::GetWindow(this, core::mem::transmute_copy(&rguidreason)) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowForBindingUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IWindowForBindingUI {}
windows_core::imp::define_interface!(IWrappedProtocol, IWrappedProtocol_Vtbl, 0x53c84785_8425_4dc5_971b_e58d9c19f9b6);
windows_core::imp::interface_hierarchy!(IWrappedProtocol, windows_core::IUnknown);
impl IWrappedProtocol {
    pub unsafe fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWrapperCode)(windows_core::Interface::as_raw(self), pncode as _, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWrappedProtocol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWrapperCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, usize) -> windows_core::HRESULT,
}
pub trait IWrappedProtocol_Impl: windows_core::IUnknownImpl {
    fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> windows_core::Result<()>;
}
impl IWrappedProtocol_Vtbl {
    pub const fn new<Identity: IWrappedProtocol_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWrapperCode<Identity: IWrappedProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWrappedProtocol_Impl::GetWrapperCode(this, core::mem::transmute_copy(&pncode), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWrapperCode: GetWrapperCode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWrappedProtocol as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWrappedProtocol {}
windows_core::imp::define_interface!(IZoneIdentifier, IZoneIdentifier_Vtbl, 0xcd45f185_1b21_48e2_967b_ead743a8914e);
windows_core::imp::interface_hierarchy!(IZoneIdentifier, windows_core::IUnknown);
impl IZoneIdentifier {
    pub unsafe fn GetId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), dwzone) }
    }
    pub unsafe fn Remove(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoneIdentifier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IZoneIdentifier_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<u32>;
    fn SetId(&self, dwzone: u32) -> windows_core::Result<()>;
    fn Remove(&self) -> windows_core::Result<()>;
}
impl IZoneIdentifier_Vtbl {
    pub const fn new<Identity: IZoneIdentifier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwzone: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IZoneIdentifier_Impl::GetId(this) {
                    Ok(ok__) => {
                        pdwzone.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetId<Identity: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoneIdentifier_Impl::SetId(this, core::mem::transmute_copy(&dwzone)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoneIdentifier_Impl::Remove(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IZoneIdentifier as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IZoneIdentifier {}
windows_core::imp::define_interface!(IZoneIdentifier2, IZoneIdentifier2_Vtbl, 0xeb5e760c_09ef_45c0_b510_70830ce31e6a);
impl core::ops::Deref for IZoneIdentifier2 {
    type Target = IZoneIdentifier;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IZoneIdentifier2, windows_core::IUnknown, IZoneIdentifier);
impl IZoneIdentifier2 {
    pub unsafe fn GetLastWriterPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastWriterPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastWriterPackageFamilyName<P0>(&self, packagefamilyname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLastWriterPackageFamilyName)(windows_core::Interface::as_raw(self), packagefamilyname.param().abi()) }
    }
    pub unsafe fn RemoveLastWriterPackageFamilyName(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveLastWriterPackageFamilyName)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAppZoneId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAppZoneId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppZoneId(&self, zone: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAppZoneId)(windows_core::Interface::as_raw(self), zone) }
    }
    pub unsafe fn RemoveAppZoneId(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAppZoneId)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoneIdentifier2_Vtbl {
    pub base__: IZoneIdentifier_Vtbl,
    pub GetLastWriterPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLastWriterPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveLastWriterPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppZoneId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAppZoneId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAppZoneId: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IZoneIdentifier2_Impl: IZoneIdentifier_Impl {
    fn GetLastWriterPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLastWriterPackageFamilyName(&self, packagefamilyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveLastWriterPackageFamilyName(&self) -> windows_core::Result<()>;
    fn GetAppZoneId(&self) -> windows_core::Result<u32>;
    fn SetAppZoneId(&self, zone: u32) -> windows_core::Result<()>;
    fn RemoveAppZoneId(&self) -> windows_core::Result<()>;
}
impl IZoneIdentifier2_Vtbl {
    pub const fn new<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IZoneIdentifier2_Impl::GetLastWriterPackageFamilyName(this) {
                    Ok(ok__) => {
                        packagefamilyname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoneIdentifier2_Impl::SetLastWriterPackageFamilyName(this, core::mem::transmute(&packagefamilyname)).into()
            }
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoneIdentifier2_Impl::RemoveLastWriterPackageFamilyName(this).into()
            }
        }
        unsafe extern "system" fn GetAppZoneId<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zone: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IZoneIdentifier2_Impl::GetAppZoneId(this) {
                    Ok(ok__) => {
                        zone.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppZoneId<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zone: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoneIdentifier2_Impl::SetAppZoneId(this, core::mem::transmute_copy(&zone)).into()
            }
        }
        unsafe extern "system" fn RemoveAppZoneId<Identity: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoneIdentifier2_Impl::RemoveAppZoneId(this).into()
            }
        }
        Self {
            base__: IZoneIdentifier_Vtbl::new::<Identity, OFFSET>(),
            GetLastWriterPackageFamilyName: GetLastWriterPackageFamilyName::<Identity, OFFSET>,
            SetLastWriterPackageFamilyName: SetLastWriterPackageFamilyName::<Identity, OFFSET>,
            RemoveLastWriterPackageFamilyName: RemoveLastWriterPackageFamilyName::<Identity, OFFSET>,
            GetAppZoneId: GetAppZoneId::<Identity, OFFSET>,
            SetAppZoneId: SetAppZoneId::<Identity, OFFSET>,
            RemoveAppZoneId: RemoveAppZoneId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IZoneIdentifier2 as windows_core::Interface>::IID || iid == &<IZoneIdentifier as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IZoneIdentifier2 {}
pub type LPCODEBASEHOLD = *mut CODEBASEHOLD;
#[cfg(feature = "minwinbase")]
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
pub const MK_S_ASYNCHRONOUS: windows_core::HRESULT = windows_core::HRESULT(0x401E8_u32 as _);
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PROTOCOLFILTERDATA {
    pub cbSize: u32,
    pub pProtocolSink: core::mem::ManuallyDrop<Option<IInternetProtocolSink>>,
    pub pProtocol: core::mem::ManuallyDrop<Option<IInternetProtocol>>,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwFilterFlags: u32,
}
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROTOCOL_ARGUMENT {
    pub szMethod: windows_core::PCWSTR,
    pub szTargetUrl: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REMSECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: u32,
    pub bInheritHandle: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RemBINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_core::PWSTR,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: REMSECURITY_ATTRIBUTES,
    pub iid: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOFTDISTINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwAdState: u32,
    pub szTitle: windows_core::PWSTR,
    pub szAbstract: windows_core::PWSTR,
    pub szHREF: windows_core::PWSTR,
    pub dwInstalledVersionMS: u32,
    pub dwInstalledVersionLS: u32,
    pub dwUpdateVersionMS: u32,
    pub dwUpdateVersionLS: u32,
    pub dwAdvertisedVersionMS: u32,
    pub dwAdvertisedVersionLS: u32,
    pub dwReserved: u32,
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
#[cfg(feature = "objidl")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StartParam {
    pub iid: windows_core::GUID,
    pub pIBindCtx: core::mem::ManuallyDrop<Option<super::objidl::IBindCtx>>,
    pub pItf: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
