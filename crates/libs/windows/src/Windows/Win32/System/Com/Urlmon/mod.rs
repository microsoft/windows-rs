#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoGetClassObjectFromURL<P0, P1, P2>(rclassid: *const ::windows::core::GUID, szcode: P0, dwfileversionms: u32, dwfileversionls: u32, sztype: P1, pbindctx: P2, dwclscontext: super::CLSCTX, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::IBindCtx>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoGetClassObjectFromURL ( rclassid : *const ::windows::core::GUID , szcode : ::windows::core::PCWSTR , dwfileversionms : u32 , dwfileversionls : u32 , sztype : ::windows::core::PCWSTR , pbindctx : * mut::core::ffi::c_void , dwclscontext : super:: CLSCTX , pvreserved : *const ::core::ffi::c_void , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CoGetClassObjectFromURL(rclassid, szcode.into_param().abi(), dwfileversionms, dwfileversionls, sztype.into_param().abi(), pbindctx.into_param().abi(), dwclscontext, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetCombineIUri<P0, P1>(pbaseuri: P0, prelativeuri: P1, dwcombineflags: u32, ppcombineduri: *mut ::core::option::Option<super::IUri>, dwreserved: usize) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IUri>,
    P1: ::windows::core::IntoParam<super::IUri>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetCombineIUri ( pbaseuri : * mut::core::ffi::c_void , prelativeuri : * mut::core::ffi::c_void , dwcombineflags : u32 , ppcombineduri : *mut * mut::core::ffi::c_void , dwreserved : usize ) -> ::windows::core::HRESULT );
    CoInternetCombineIUri(pbaseuri.into_param().abi(), prelativeuri.into_param().abi(), dwcombineflags, ::core::mem::transmute(ppcombineduri), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetCombineUrl<P0, P1>(pwzbaseurl: P0, pwzrelativeurl: P1, dwcombineflags: u32, pszresult: &mut [u16], pcchresult: ::core::option::Option<*mut u32>, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetCombineUrl ( pwzbaseurl : ::windows::core::PCWSTR , pwzrelativeurl : ::windows::core::PCWSTR , dwcombineflags : u32 , pszresult : ::windows::core::PWSTR , cchresult : u32 , pcchresult : *mut u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetCombineUrl(pwzbaseurl.into_param().abi(), pwzrelativeurl.into_param().abi(), dwcombineflags, ::core::mem::transmute(pszresult.as_ptr()), pszresult.len() as _, ::core::mem::transmute(pcchresult.unwrap_or(::std::ptr::null_mut())), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetCombineUrlEx<P0, P1>(pbaseuri: P0, pwzrelativeurl: P1, dwcombineflags: u32, ppcombineduri: *mut ::core::option::Option<super::IUri>, dwreserved: usize) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IUri>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetCombineUrlEx ( pbaseuri : * mut::core::ffi::c_void , pwzrelativeurl : ::windows::core::PCWSTR , dwcombineflags : u32 , ppcombineduri : *mut * mut::core::ffi::c_void , dwreserved : usize ) -> ::windows::core::HRESULT );
    CoInternetCombineUrlEx(pbaseuri.into_param().abi(), pwzrelativeurl.into_param().abi(), dwcombineflags, ::core::mem::transmute(ppcombineduri), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetCompareUrl<P0, P1>(pwzurl1: P0, pwzurl2: P1, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetCompareUrl ( pwzurl1 : ::windows::core::PCWSTR , pwzurl2 : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    CoInternetCompareUrl(pwzurl1.into_param().abi(), pwzurl2.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetCreateSecurityManager<P0>(psp: P0, ppsm: *mut ::core::option::Option<IInternetSecurityManager>, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IServiceProvider>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetCreateSecurityManager ( psp : * mut::core::ffi::c_void , ppsm : *mut * mut::core::ffi::c_void , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetCreateSecurityManager(psp.into_param().abi(), ::core::mem::transmute(ppsm), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetCreateZoneManager<P0>(psp: P0, ppzm: *mut ::core::option::Option<IInternetZoneManager>, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IServiceProvider>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetCreateZoneManager ( psp : * mut::core::ffi::c_void , ppzm : *mut * mut::core::ffi::c_void , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetCreateZoneManager(psp.into_param().abi(), ::core::mem::transmute(ppzm), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetGetProtocolFlags<P0>(pwzurl: P0, pdwflags: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetGetProtocolFlags ( pwzurl : ::windows::core::PCWSTR , pdwflags : *mut u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetGetProtocolFlags(pwzurl.into_param().abi(), pdwflags, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetGetSecurityUrl<P0>(pwszurl: P0, ppwszsecurl: *mut ::windows::core::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetGetSecurityUrl ( pwszurl : ::windows::core::PCWSTR , ppwszsecurl : *mut ::windows::core::PWSTR , psuaction : PSUACTION , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetGetSecurityUrl(pwszurl.into_param().abi(), ppwszsecurl, psuaction, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetGetSecurityUrlEx<P0>(puri: P0, ppsecuri: *mut ::core::option::Option<super::IUri>, psuaction: PSUACTION, dwreserved: usize) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IUri>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetGetSecurityUrlEx ( puri : * mut::core::ffi::c_void , ppsecuri : *mut * mut::core::ffi::c_void , psuaction : PSUACTION , dwreserved : usize ) -> ::windows::core::HRESULT );
    CoInternetGetSecurityUrlEx(puri.into_param().abi(), ::core::mem::transmute(ppsecuri), psuaction, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut ::core::option::Option<IInternetSession>, dwreserved: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetGetSession ( dwsessionmode : u32 , ppiinternetsession : *mut * mut::core::ffi::c_void , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetGetSession(dwsessionmode, ::core::mem::transmute(ppiinternetsession), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetIsFeatureEnabled ( featureentry : INTERNETFEATURELIST , dwflags : u32 ) -> ::windows::core::HRESULT );
    CoInternetIsFeatureEnabled(featureentry, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForIUri<P0, P1>(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: P0, psecmgr: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IUri>,
    P1: ::windows::core::IntoParam<IInternetSecurityManagerEx2>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetIsFeatureEnabledForIUri ( featureentry : INTERNETFEATURELIST , dwflags : u32 , piuri : * mut::core::ffi::c_void , psecmgr : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CoInternetIsFeatureEnabledForIUri(featureentry, dwflags, piuri.into_param().abi(), psecmgr.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForUrl<P0, P1>(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: P0, psecmgr: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<IInternetSecurityManager>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetIsFeatureEnabledForUrl ( featureentry : INTERNETFEATURELIST , dwflags : u32 , szurl : ::windows::core::PCWSTR , psecmgr : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CoInternetIsFeatureEnabledForUrl(featureentry, dwflags, szurl.into_param().abi(), psecmgr.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetIsFeatureZoneElevationEnabled<P0, P1, P2>(szfromurl: P0, sztourl: P1, psecmgr: P2, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<IInternetSecurityManager>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetIsFeatureZoneElevationEnabled ( szfromurl : ::windows::core::PCWSTR , sztourl : ::windows::core::PCWSTR , psecmgr : * mut::core::ffi::c_void , dwflags : u32 ) -> ::windows::core::HRESULT );
    CoInternetIsFeatureZoneElevationEnabled(szfromurl.into_param().abi(), sztourl.into_param().abi(), psecmgr.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetParseIUri<P0>(piuri: P0, parseaction: PARSEACTION, dwflags: u32, pwzresult: &mut [u16], pcchresult: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IUri>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetParseIUri ( piuri : * mut::core::ffi::c_void , parseaction : PARSEACTION , dwflags : u32 , pwzresult : ::windows::core::PWSTR , cchresult : u32 , pcchresult : *mut u32 , dwreserved : usize ) -> ::windows::core::HRESULT );
    CoInternetParseIUri(piuri.into_param().abi(), parseaction, dwflags, ::core::mem::transmute(pwzresult.as_ptr()), pwzresult.len() as _, pcchresult, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetParseUrl<P0>(pwzurl: P0, parseaction: PARSEACTION, dwflags: u32, pszresult: &mut [u16], pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetParseUrl ( pwzurl : ::windows::core::PCWSTR , parseaction : PARSEACTION , dwflags : u32 , pszresult : ::windows::core::PWSTR , cchresult : u32 , pcchresult : *mut u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetParseUrl(pwzurl.into_param().abi(), parseaction, dwflags, ::core::mem::transmute(pszresult.as_ptr()), pszresult.len() as _, pcchresult, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CoInternetQueryInfo<P0>(pwzurl: P0, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuffer: ::core::option::Option<*mut u32>, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetQueryInfo ( pwzurl : ::windows::core::PCWSTR , queryoptions : QUERYOPTION , dwqueryflags : u32 , pvbuffer : *mut ::core::ffi::c_void , cbbuffer : u32 , pcbbuffer : *mut u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CoInternetQueryInfo(pwzurl.into_param().abi(), queryoptions, dwqueryflags, pvbuffer, cbbuffer, ::core::mem::transmute(pcbbuffer.unwrap_or(::std::ptr::null_mut())), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetSetFeatureEnabled<P0>(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CoInternetSetFeatureEnabled ( featureentry : INTERNETFEATURELIST , dwflags : u32 , fenable : super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    CoInternetSetFeatureEnabled(featureentry, dwflags, fenable.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CompareSecurityIds(pbsecurityid1: &[u8], pbsecurityid2: &[u8], dwreserved: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CompareSecurityIds ( pbsecurityid1 : *const u8 , dwlen1 : u32 , pbsecurityid2 : *const u8 , dwlen2 : u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    CompareSecurityIds(::core::mem::transmute(pbsecurityid1.as_ptr()), pbsecurityid1.len() as _, ::core::mem::transmute(pbsecurityid2.as_ptr()), pbsecurityid2.len() as _, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CompatFlagsFromClsid(pclsid: *const ::windows::core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CompatFlagsFromClsid ( pclsid : *const ::windows::core::GUID , pdwcompatflags : *mut u32 , pdwmiscstatusflags : *mut u32 ) -> ::windows::core::HRESULT );
    CompatFlagsFromClsid(pclsid, pdwcompatflags, pdwmiscstatusflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn CopyBindInfo(pcbisrc: *const super::BINDINFO, pbidest: *mut super::BINDINFO) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CopyBindInfo ( pcbisrc : *const super:: BINDINFO , pbidest : *mut super:: BINDINFO ) -> ::windows::core::HRESULT );
    CopyBindInfo(pcbisrc, pbidest).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn CopyStgMedium(pcstgmedsrc: *const super::STGMEDIUM) -> ::windows::core::Result<super::STGMEDIUM> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CopyStgMedium ( pcstgmedsrc : *const super:: STGMEDIUM , pstgmeddest : *mut super:: STGMEDIUM ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::STGMEDIUM>();
    CopyStgMedium(pcstgmedsrc, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CreateAsyncBindCtx<P0, P1>(reserved: u32, pbscb: P0, pefetc: P1) -> ::windows::core::Result<super::IBindCtx>
where
    P0: ::windows::core::IntoParam<super::IBindStatusCallback>,
    P1: ::windows::core::IntoParam<super::IEnumFORMATETC>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CreateAsyncBindCtx ( reserved : u32 , pbscb : * mut::core::ffi::c_void , pefetc : * mut::core::ffi::c_void , ppbc : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::IBindCtx>();
    CreateAsyncBindCtx(reserved, pbscb.into_param().abi(), pefetc.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CreateAsyncBindCtxEx<P0, P1, P2>(pbc: P0, dwoptions: u32, pbscb: P1, penum: P2, ppbc: *mut ::core::option::Option<super::IBindCtx>, reserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<super::IBindStatusCallback>,
    P2: ::windows::core::IntoParam<super::IEnumFORMATETC>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CreateAsyncBindCtxEx ( pbc : * mut::core::ffi::c_void , dwoptions : u32 , pbscb : * mut::core::ffi::c_void , penum : * mut::core::ffi::c_void , ppbc : *mut * mut::core::ffi::c_void , reserved : u32 ) -> ::windows::core::HRESULT );
    CreateAsyncBindCtxEx(pbc.into_param().abi(), dwoptions, pbscb.into_param().abi(), penum.into_param().abi(), ::core::mem::transmute(ppbc), reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CreateFormatEnumerator(rgfmtetc: &[super::FORMATETC]) -> ::windows::core::Result<super::IEnumFORMATETC> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn CreateFormatEnumerator ( cfmtetc : u32 , rgfmtetc : *const super:: FORMATETC , ppenumfmtetc : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::IEnumFORMATETC>();
    CreateFormatEnumerator(rgfmtetc.len() as _, ::core::mem::transmute(rgfmtetc.as_ptr()), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CreateURLMoniker<P0, P1>(pmkctx: P0, szurl: P1) -> ::windows::core::Result<super::IMoniker>
where
    P0: ::windows::core::IntoParam<super::IMoniker>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CreateURLMoniker ( pmkctx : * mut::core::ffi::c_void , szurl : ::windows::core::PCWSTR , ppmk : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::IMoniker>();
    CreateURLMoniker(pmkctx.into_param().abi(), szurl.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CreateURLMonikerEx<P0, P1>(pmkctx: P0, szurl: P1, ppmk: *mut ::core::option::Option<super::IMoniker>, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IMoniker>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CreateURLMonikerEx ( pmkctx : * mut::core::ffi::c_void , szurl : ::windows::core::PCWSTR , ppmk : *mut * mut::core::ffi::c_void , dwflags : u32 ) -> ::windows::core::HRESULT );
    CreateURLMonikerEx(pmkctx.into_param().abi(), szurl.into_param().abi(), ::core::mem::transmute(ppmk), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn CreateURLMonikerEx2<P0, P1>(pmkctx: P0, puri: P1, ppmk: *mut ::core::option::Option<super::IMoniker>, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IMoniker>,
    P1: ::windows::core::IntoParam<super::IUri>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn CreateURLMonikerEx2 ( pmkctx : * mut::core::ffi::c_void , puri : * mut::core::ffi::c_void , ppmk : *mut * mut::core::ffi::c_void , dwflags : u32 ) -> ::windows::core::HRESULT );
    CreateURLMonikerEx2(pmkctx.into_param().abi(), puri.into_param().abi(), ::core::mem::transmute(ppmk), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaultInIEFeature<P0>(hwnd: P0, pclassspec: *const super::uCLSSPEC, pquery: ::core::option::Option<*mut super::QUERYCONTEXT>, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn FaultInIEFeature ( hwnd : super::super::super::Foundation:: HWND , pclassspec : *const super:: uCLSSPEC , pquery : *mut super:: QUERYCONTEXT , dwflags : u32 ) -> ::windows::core::HRESULT );
    FaultInIEFeature(hwnd.into_param().abi(), pclassspec, ::core::mem::transmute(pquery.unwrap_or(::std::ptr::null_mut())), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn FindMediaType<P0>(rgsztypes: P0) -> ::windows::core::Result<u16>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn FindMediaType ( rgsztypes : ::windows::core::PCSTR , rgcftypes : *mut u16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u16>();
    FindMediaType(rgsztypes.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn FindMediaTypeClass<P0, P1>(pbc: P0, sztype: P1, pclsid: *mut ::windows::core::GUID, reserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn FindMediaTypeClass ( pbc : * mut::core::ffi::c_void , sztype : ::windows::core::PCSTR , pclsid : *mut ::windows::core::GUID , reserved : u32 ) -> ::windows::core::HRESULT );
    FindMediaTypeClass(pbc.into_param().abi(), sztype.into_param().abi(), pclsid, reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn FindMimeFromData<P0, P1, P2>(pbc: P0, pwzurl: P1, pbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbsize: u32, pwzmimeproposed: P2, dwmimeflags: u32, ppwzmimeout: *mut ::windows::core::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn FindMimeFromData ( pbc : * mut::core::ffi::c_void , pwzurl : ::windows::core::PCWSTR , pbuffer : *const ::core::ffi::c_void , cbsize : u32 , pwzmimeproposed : ::windows::core::PCWSTR , dwmimeflags : u32 , ppwzmimeout : *mut ::windows::core::PWSTR , dwreserved : u32 ) -> ::windows::core::HRESULT );
    FindMimeFromData(pbc.into_param().abi(), pwzurl.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null())), cbsize, pwzmimeproposed.into_param().abi(), dwmimeflags, ppwzmimeout, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn GetClassFileOrMime<P0, P1, P2>(pbc: P0, szfilename: P1, pbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbsize: u32, szmime: P2, dwreserved: u32) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn GetClassFileOrMime ( pbc : * mut::core::ffi::c_void , szfilename : ::windows::core::PCWSTR , pbuffer : *const ::core::ffi::c_void , cbsize : u32 , szmime : ::windows::core::PCWSTR , dwreserved : u32 , pclsid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    GetClassFileOrMime(pbc.into_param().abi(), szfilename.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null())), cbsize, szmime.into_param().abi(), dwreserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn GetClassURL<P0>(szurl: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn GetClassURL ( szurl : ::windows::core::PCWSTR , pclsid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    GetClassURL(szurl.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC) -> ::windows::core::Result<::windows::core::PSTR> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn GetComponentIDFromCLSSPEC ( pclassspec : *const super:: uCLSSPEC , ppszcomponentid : *mut ::windows::core::PSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PSTR>();
    GetComponentIDFromCLSSPEC(pclassspec, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn GetSoftwareUpdateInfo<P0>(szdistunit: P0, psdi: *mut SOFTDISTINFO) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn GetSoftwareUpdateInfo ( szdistunit : ::windows::core::PCWSTR , psdi : *mut SOFTDISTINFO ) -> ::windows::core::HRESULT );
    GetSoftwareUpdateInfo(szdistunit.into_param().abi(), psdi).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn HlinkGoBack<P0>(punk: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn HlinkGoBack ( punk : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    HlinkGoBack(punk.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn HlinkGoForward<P0>(punk: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn HlinkGoForward ( punk : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    HlinkGoForward(punk.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn HlinkNavigateMoniker<P0, P1>(punk: P0, pmktarget: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<super::IMoniker>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn HlinkNavigateMoniker ( punk : * mut::core::ffi::c_void , pmktarget : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    HlinkNavigateMoniker(punk.into_param().abi(), pmktarget.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn HlinkNavigateString<P0, P1>(punk: P0, sztarget: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn HlinkNavigateString ( punk : * mut::core::ffi::c_void , sztarget : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    HlinkNavigateString(punk.into_param().abi(), sztarget.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn HlinkSimpleNavigateToMoniker<P0, P1, P2, P3, P4, P5>(pmktarget: P0, szlocation: P1, sztargetframename: P2, punk: P3, pbc: P4, param5: P5, grfhlnf: u32, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IMoniker>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P4: ::windows::core::IntoParam<super::IBindCtx>,
    P5: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn HlinkSimpleNavigateToMoniker ( pmktarget : * mut::core::ffi::c_void , szlocation : ::windows::core::PCWSTR , sztargetframename : ::windows::core::PCWSTR , punk : * mut::core::ffi::c_void , pbc : * mut::core::ffi::c_void , param5 : * mut::core::ffi::c_void , grfhlnf : u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    HlinkSimpleNavigateToMoniker(pmktarget.into_param().abi(), szlocation.into_param().abi(), sztargetframename.into_param().abi(), punk.into_param().abi(), pbc.into_param().abi(), param5.into_param().abi(), grfhlnf, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn HlinkSimpleNavigateToString<P0, P1, P2, P3, P4, P5>(sztarget: P0, szlocation: P1, sztargetframename: P2, punk: P3, pbc: P4, param5: P5, grfhlnf: u32, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P4: ::windows::core::IntoParam<super::IBindCtx>,
    P5: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn HlinkSimpleNavigateToString ( sztarget : ::windows::core::PCWSTR , szlocation : ::windows::core::PCWSTR , sztargetframename : ::windows::core::PCWSTR , punk : * mut::core::ffi::c_void , pbc : * mut::core::ffi::c_void , param5 : * mut::core::ffi::c_void , grfhlnf : u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    HlinkSimpleNavigateToString(sztarget.into_param().abi(), szlocation.into_param().abi(), sztargetframename.into_param().abi(), punk.into_param().abi(), pbc.into_param().abi(), param5.into_param().abi(), grfhlnf, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn IEGetUserPrivateNamespaceName() -> ::windows::core::PWSTR {
    ::windows_targets::link ! ( "urlmon.dll""system" fn IEGetUserPrivateNamespaceName ( ) -> ::windows::core::PWSTR );
    IEGetUserPrivateNamespaceName()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn IEInstallScope() -> ::windows::core::Result<u32> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn IEInstallScope ( pdwscope : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    IEInstallScope(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn IsAsyncMoniker<P0>(pmk: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IMoniker>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn IsAsyncMoniker ( pmk : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    IsAsyncMoniker(pmk.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsLoggingEnabledA<P0>(pszurl: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn IsLoggingEnabledA ( pszurl : ::windows::core::PCSTR ) -> super::super::super::Foundation:: BOOL );
    IsLoggingEnabledA(pszurl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsLoggingEnabledW<P0>(pwszurl: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn IsLoggingEnabledW ( pwszurl : ::windows::core::PCWSTR ) -> super::super::super::Foundation:: BOOL );
    IsLoggingEnabledW(pwszurl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn IsValidURL<P0, P1>(pbc: P0, szurl: P1, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn IsValidURL ( pbc : * mut::core::ffi::c_void , szurl : ::windows::core::PCWSTR , dwreserved : u32 ) -> ::windows::core::HRESULT );
    IsValidURL(pbc.into_param().abi(), szurl.into_param().abi(), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn MkParseDisplayNameEx<P0, P1>(pbc: P0, szdisplayname: P1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<super::IMoniker>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn MkParseDisplayNameEx ( pbc : * mut::core::ffi::c_void , szdisplayname : ::windows::core::PCWSTR , pcheaten : *mut u32 , ppmk : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    MkParseDisplayNameEx(pbc.into_param().abi(), szdisplayname.into_param().abi(), pcheaten, ::core::mem::transmute(ppmk)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn ObtainUserAgentString(dwoption: u32, pszuaout: ::windows::core::PSTR, cbsize: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn ObtainUserAgentString ( dwoption : u32 , pszuaout : ::windows::core::PSTR , cbsize : *mut u32 ) -> ::windows::core::HRESULT );
    ObtainUserAgentString(dwoption, ::core::mem::transmute(pszuaout), cbsize).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn RegisterBindStatusCallback<P0, P1>(pbc: P0, pbscb: P1, ppbscbprev: *mut ::core::option::Option<super::IBindStatusCallback>, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn RegisterBindStatusCallback ( pbc : * mut::core::ffi::c_void , pbscb : * mut::core::ffi::c_void , ppbscbprev : *mut * mut::core::ffi::c_void , dwreserved : u32 ) -> ::windows::core::HRESULT );
    RegisterBindStatusCallback(pbc.into_param().abi(), pbscb.into_param().abi(), ::core::mem::transmute(ppbscbprev), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn RegisterFormatEnumerator<P0, P1>(pbc: P0, pefetc: P1, reserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<super::IEnumFORMATETC>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn RegisterFormatEnumerator ( pbc : * mut::core::ffi::c_void , pefetc : * mut::core::ffi::c_void , reserved : u32 ) -> ::windows::core::HRESULT );
    RegisterFormatEnumerator(pbc.into_param().abi(), pefetc.into_param().abi(), reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn RegisterMediaTypeClass<P0>(pbc: P0, ctypes: u32, rgsztypes: *const ::windows::core::PCSTR, rgclsid: *const ::windows::core::GUID, reserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn RegisterMediaTypeClass ( pbc : * mut::core::ffi::c_void , ctypes : u32 , rgsztypes : *const ::windows::core::PCSTR , rgclsid : *const ::windows::core::GUID , reserved : u32 ) -> ::windows::core::HRESULT );
    RegisterMediaTypeClass(pbc.into_param().abi(), ctypes, rgsztypes, rgclsid, reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const ::windows::core::PCSTR, rgcftypes: *mut u16) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn RegisterMediaTypes ( ctypes : u32 , rgsztypes : *const ::windows::core::PCSTR , rgcftypes : *mut u16 ) -> ::windows::core::HRESULT );
    RegisterMediaTypes(ctypes, rgsztypes, rgcftypes).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseBindInfo(pbindinfo: *mut super::BINDINFO) {
    ::windows_targets::link ! ( "urlmon.dll""system" fn ReleaseBindInfo ( pbindinfo : *mut super:: BINDINFO ) -> ( ) );
    ReleaseBindInfo(pbindinfo)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn RevokeBindStatusCallback<P0, P1>(pbc: P0, pbscb: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn RevokeBindStatusCallback ( pbc : * mut::core::ffi::c_void , pbscb : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RevokeBindStatusCallback(pbc.into_param().abi(), pbscb.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn RevokeFormatEnumerator<P0, P1>(pbc: P0, pefetc: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IBindCtx>,
    P1: ::windows::core::IntoParam<super::IEnumFORMATETC>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn RevokeFormatEnumerator ( pbc : * mut::core::ffi::c_void , pefetc : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RevokeFormatEnumerator(pbc.into_param().abi(), pefetc.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetAccessForIEAppContainer<P0>(hobject: P0, ieobjecttype: IEObjectType, dwaccessmask: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn SetAccessForIEAppContainer ( hobject : super::super::super::Foundation:: HANDLE , ieobjecttype : IEObjectType , dwaccessmask : u32 ) -> ::windows::core::HRESULT );
    SetAccessForIEAppContainer(hobject.into_param().abi(), ieobjecttype, dwaccessmask).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn SetSoftwareUpdateAdvertisementState<P0>(szdistunit: P0, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn SetSoftwareUpdateAdvertisementState ( szdistunit : ::windows::core::PCWSTR , dwadstate : u32 , dwadvertisedversionms : u32 , dwadvertisedversionls : u32 ) -> ::windows::core::HRESULT );
    SetSoftwareUpdateAdvertisementState(szdistunit.into_param().abi(), dwadstate, dwadvertisedversionms, dwadvertisedversionls).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLDownloadToCacheFileA<P0, P1, P2>(param0: P0, param1: P1, param2: &mut [u8], param4: u32, param5: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLDownloadToCacheFileA ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCSTR , param2 : ::windows::core::PSTR , cchfilename : u32 , param4 : u32 , param5 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLDownloadToCacheFileA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2.as_ptr()), param2.len() as _, param4, param5.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLDownloadToCacheFileW<P0, P1, P2>(param0: P0, param1: P1, param2: &mut [u16], param4: u32, param5: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLDownloadToCacheFileW ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCWSTR , param2 : ::windows::core::PWSTR , cchfilename : u32 , param4 : u32 , param5 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLDownloadToCacheFileW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2.as_ptr()), param2.len() as _, param4, param5.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLDownloadToFileA<P0, P1, P2, P3>(param0: P0, param1: P1, param2: P2, param3: u32, param4: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLDownloadToFileA ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCSTR , param2 : ::windows::core::PCSTR , param3 : u32 , param4 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLDownloadToFileA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi(), param3, param4.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLDownloadToFileW<P0, P1, P2, P3>(param0: P0, param1: P1, param2: P2, param3: u32, param4: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLDownloadToFileW ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCWSTR , param2 : ::windows::core::PCWSTR , param3 : u32 , param4 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLDownloadToFileW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi(), param3, param4.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLOpenBlockingStreamA<P0, P1, P2>(param0: P0, param1: P1, param2: *mut ::core::option::Option<super::IStream>, param3: u32, param4: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLOpenBlockingStreamA ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCSTR , param2 : *mut * mut::core::ffi::c_void , param3 : u32 , param4 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLOpenBlockingStreamA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), param3, param4.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLOpenBlockingStreamW<P0, P1, P2>(param0: P0, param1: P1, param2: *mut ::core::option::Option<super::IStream>, param3: u32, param4: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLOpenBlockingStreamW ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCWSTR , param2 : *mut * mut::core::ffi::c_void , param3 : u32 , param4 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLOpenBlockingStreamW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), param3, param4.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLOpenPullStreamA<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLOpenPullStreamA ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCSTR , param2 : u32 , param3 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLOpenPullStreamA(param0.into_param().abi(), param1.into_param().abi(), param2, param3.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLOpenPullStreamW<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLOpenPullStreamW ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCWSTR , param2 : u32 , param3 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLOpenPullStreamW(param0.into_param().abi(), param1.into_param().abi(), param2, param3.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLOpenStreamA<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLOpenStreamA ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCSTR , param2 : u32 , param3 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLOpenStreamA(param0.into_param().abi(), param1.into_param().abi(), param2, param3.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn URLOpenStreamW<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::IBindStatusCallback>,
{
    ::windows_targets::link ! ( "urlmon.dll""system" fn URLOpenStreamW ( param0 : * mut::core::ffi::c_void , param1 : ::windows::core::PCWSTR , param2 : u32 , param3 : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    URLOpenStreamW(param0.into_param().abi(), param1.into_param().abi(), param2, param3.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn UrlMkGetSessionOption(dwoption: u32, pbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn UrlMkGetSessionOption ( dwoption : u32 , pbuffer : *mut ::core::ffi::c_void , dwbufferlength : u32 , pdwbufferlengthout : *mut u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    UrlMkGetSessionOption(dwoption, ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), dwbufferlength, pdwbufferlengthout, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[inline]
pub unsafe fn UrlMkSetSessionOption(dwoption: u32, pbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "urlmon.dll""system" fn UrlMkSetSessionOption ( dwoption : u32 , pbuffer : *const ::core::ffi::c_void , dwbufferlength : u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    UrlMkSetSessionOption(dwoption, ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null())), dwbufferlength, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "urlmon.dll""system" fn WriteHitLogging ( lplogginginfo : *const HIT_LOGGING_INFO ) -> super::super::super::Foundation:: BOOL );
    WriteHitLogging(lplogginginfo)
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IBindCallbackRedirect(::windows::core::IUnknown);
impl IBindCallbackRedirect {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Redirect<P0>(&self, lpcurl: P0) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).Redirect)(::windows::core::Interface::as_raw(self), lpcurl.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IBindCallbackRedirect, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IBindCallbackRedirect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindCallbackRedirect {}
impl ::core::fmt::Debug for IBindCallbackRedirect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCallbackRedirect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindCallbackRedirect {
    type Vtable = IBindCallbackRedirect_Vtbl;
}
impl ::core::clone::Clone for IBindCallbackRedirect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBindCallbackRedirect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c81bc2_121e_4ed5_b9c4_b430bd54f2c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCallbackRedirect_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Redirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcurl: ::windows::core::PCWSTR, vbcancel: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Redirect: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IBindHttpSecurity(::windows::core::IUnknown);
impl IBindHttpSecurity {
    pub unsafe fn GetIgnoreCertMask(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetIgnoreCertMask)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IBindHttpSecurity, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IBindHttpSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindHttpSecurity {}
impl ::core::fmt::Debug for IBindHttpSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHttpSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindHttpSecurity {
    type Vtable = IBindHttpSecurity_Vtbl;
}
impl ::core::clone::Clone for IBindHttpSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBindHttpSecurity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9eda967_f50e_4a33_b358_206f6ef3086d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHttpSecurity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIgnoreCertMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwignorecertmask: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IBindProtocol(::windows::core::IUnknown);
impl IBindProtocol {
    pub unsafe fn CreateBinding<P0, P1>(&self, szurl: P0, pbc: P1) -> ::windows::core::Result<super::IBinding>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::IBindCtx>,
    {
        let mut result__ = ::windows::core::zeroed::<super::IBinding>();
        (::windows::core::Interface::vtable(self).CreateBinding)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), pbc.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IBindProtocol, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IBindProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindProtocol {}
impl ::core::fmt::Debug for IBindProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindProtocol {
    type Vtable = IBindProtocol_Vtbl;
}
impl ::core::clone::Clone for IBindProtocol {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBindProtocol {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9cd_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindProtocol_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, ppb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct ICatalogFileInfo(::windows::core::IUnknown);
impl ICatalogFileInfo {
    pub unsafe fn GetCatalogFile(&self) -> ::windows::core::Result<::windows::core::PSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PSTR>();
        (::windows::core::Interface::vtable(self).GetCatalogFile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetJavaTrust(&self, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetJavaTrust)(::windows::core::Interface::as_raw(self), ppjavatrust).ok()
    }
}
::windows::imp::interface_hierarchy!(ICatalogFileInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICatalogFileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatalogFileInfo {}
impl ::core::fmt::Debug for ICatalogFileInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogFileInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICatalogFileInfo {
    type Vtable = ICatalogFileInfo_Vtbl;
}
impl ::core::clone::Clone for ICatalogFileInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICatalogFileInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x711c7600_6b48_11d1_b403_00aa00b92af1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogFileInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCatalogFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcatalogfile: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT,
    pub GetJavaTrust: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct ICodeInstall(::windows::core::IUnknown);
impl ICodeInstall {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.GetWindow)(::windows::core::Interface::as_raw(self), rguidreason, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnCodeInstallProblem<P0, P1>(&self, ulstatuscode: u32, szdestination: P0, szsource: P1, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnCodeInstallProblem)(::windows::core::Interface::as_raw(self), ulstatuscode, szdestination.into_param().abi(), szsource.into_param().abi(), dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(ICodeInstall, ::windows::core::IUnknown, IWindowForBindingUI);
impl ::core::cmp::PartialEq for ICodeInstall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICodeInstall {}
impl ::core::fmt::Debug for ICodeInstall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICodeInstall").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICodeInstall {
    type Vtable = ICodeInstall_Vtbl;
}
impl ::core::clone::Clone for ICodeInstall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICodeInstall {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d1_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodeInstall_Vtbl {
    pub base__: IWindowForBindingUI_Vtbl,
    pub OnCodeInstallProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szdestination: ::windows::core::PCWSTR, szsource: ::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IDataFilter(::windows::core::IUnknown);
impl IDataFilter {
    pub unsafe fn DoEncode(&self, dwflags: u32, pbinbuffer: &[u8], pboutbuffer: &mut [u8], linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoEncode)(::windows::core::Interface::as_raw(self), dwflags, pbinbuffer.len() as _, ::core::mem::transmute(pbinbuffer.as_ptr()), pboutbuffer.len() as _, ::core::mem::transmute(pboutbuffer.as_ptr()), linbytesavailable, plinbytesread, ploutbyteswritten, dwreserved).ok()
    }
    pub unsafe fn DoDecode(&self, dwflags: u32, pbinbuffer: &[u8], pboutbuffer: &mut [u8], linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoDecode)(::windows::core::Interface::as_raw(self), dwflags, pbinbuffer.len() as _, ::core::mem::transmute(pbinbuffer.as_ptr()), pboutbuffer.len() as _, ::core::mem::transmute(pboutbuffer.as_ptr()), linbytesavailable, plinbytesread, ploutbyteswritten, dwreserved).ok()
    }
    pub unsafe fn SetEncodingLevel(&self, dwenclevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEncodingLevel)(::windows::core::Interface::as_raw(self), dwenclevel).ok()
    }
}
::windows::imp::interface_hierarchy!(IDataFilter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDataFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataFilter {}
impl ::core::fmt::Debug for IDataFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataFilter {
    type Vtable = IDataFilter_Vtbl;
}
impl ::core::clone::Clone for IDataFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69d14c80_c18e_11d0_a9ce_006097942311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DoEncode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub DoDecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub SetEncodingLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenclevel: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IEncodingFilterFactory(::windows::core::IUnknown);
impl IEncodingFilterFactory {
    pub unsafe fn FindBestFilter<P0, P1>(&self, pwzcodein: P0, pwzcodeout: P1, info: DATAINFO) -> ::windows::core::Result<IDataFilter>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IDataFilter>();
        (::windows::core::Interface::vtable(self).FindBestFilter)(::windows::core::Interface::as_raw(self), pwzcodein.into_param().abi(), pwzcodeout.into_param().abi(), ::core::mem::transmute(info), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultFilter<P0, P1>(&self, pwzcodein: P0, pwzcodeout: P1) -> ::windows::core::Result<IDataFilter>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IDataFilter>();
        (::windows::core::Interface::vtable(self).GetDefaultFilter)(::windows::core::Interface::as_raw(self), pwzcodein.into_param().abi(), pwzcodeout.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEncodingFilterFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEncodingFilterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEncodingFilterFactory {}
impl ::core::fmt::Debug for IEncodingFilterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEncodingFilterFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEncodingFilterFactory {
    type Vtable = IEncodingFilterFactory_Vtbl;
}
impl ::core::clone::Clone for IEncodingFilterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEncodingFilterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70bdde00_c18e_11d0_a9ce_006097942311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEncodingFilterFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindBestFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzcodein: ::windows::core::PCWSTR, pwzcodeout: ::windows::core::PCWSTR, info: DATAINFO, ppdf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzcodein: ::windows::core::PCWSTR, pwzcodeout: ::windows::core::PCWSTR, ppdf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IGetBindHandle(::windows::core::IUnknown);
impl IGetBindHandle {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HANDLE>();
        (::windows::core::Interface::vtable(self).GetBindHandle)(::windows::core::Interface::as_raw(self), enumrequestedhandle, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IGetBindHandle, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IGetBindHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetBindHandle {}
impl ::core::fmt::Debug for IGetBindHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetBindHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGetBindHandle {
    type Vtable = IGetBindHandle_Vtbl;
}
impl ::core::clone::Clone for IGetBindHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGetBindHandle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf0ff408_129d_4b20_91f0_02bd23d88352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetBindHandle_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBindHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBindHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IHttpNegotiate(::windows::core::IUnknown);
impl IHttpNegotiate {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).BeginningTransaction)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), szheaders.into_param().abi(), dwreserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnResponse<P0, P1>(&self, dwresponsecode: u32, szresponseheaders: P0, szrequestheaders: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).OnResponse)(::windows::core::Interface::as_raw(self), dwresponsecode, szresponseheaders.into_param().abi(), szrequestheaders.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IHttpNegotiate, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHttpNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpNegotiate {}
impl ::core::fmt::Debug for IHttpNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpNegotiate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHttpNegotiate {
    type Vtable = IHttpNegotiate_Vtbl;
}
impl ::core::clone::Clone for IHttpNegotiate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpNegotiate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d2_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginningTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, szheaders: ::windows::core::PCWSTR, dwreserved: u32, pszadditionalheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub OnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwresponsecode: u32, szresponseheaders: ::windows::core::PCWSTR, szrequestheaders: ::windows::core::PCWSTR, pszadditionalrequestheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IHttpNegotiate2(::windows::core::IUnknown);
impl IHttpNegotiate2 {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.BeginningTransaction)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), szheaders.into_param().abi(), dwreserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnResponse<P0, P1>(&self, dwresponsecode: u32, szresponseheaders: P0, szrequestheaders: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.OnResponse)(::windows::core::Interface::as_raw(self), dwresponsecode, szresponseheaders.into_param().abi(), szrequestheaders.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRootSecurityId)(::windows::core::Interface::as_raw(self), pbsecurityid, pcbsecurityid, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IHttpNegotiate2, ::windows::core::IUnknown, IHttpNegotiate);
impl ::core::cmp::PartialEq for IHttpNegotiate2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpNegotiate2 {}
impl ::core::fmt::Debug for IHttpNegotiate2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpNegotiate2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHttpNegotiate2 {
    type Vtable = IHttpNegotiate2_Vtbl;
}
impl ::core::clone::Clone for IHttpNegotiate2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpNegotiate2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f9f9fcb_e0f4_48eb_b7ab_fa2ea9365cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate2_Vtbl {
    pub base__: IHttpNegotiate_Vtbl,
    pub GetRootSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IHttpNegotiate3(::windows::core::IUnknown);
impl IHttpNegotiate3 {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.BeginningTransaction)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), szheaders.into_param().abi(), dwreserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnResponse<P0, P1>(&self, dwresponsecode: u32, szresponseheaders: P0, szrequestheaders: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.OnResponse)(::windows::core::Interface::as_raw(self), dwresponsecode, szresponseheaders.into_param().abi(), szrequestheaders.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRootSecurityId)(::windows::core::Interface::as_raw(self), pbsecurityid, pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerializedClientCertContext)(::windows::core::Interface::as_raw(self), ppbcert, pcbcert).ok()
    }
}
::windows::imp::interface_hierarchy!(IHttpNegotiate3, ::windows::core::IUnknown, IHttpNegotiate, IHttpNegotiate2);
impl ::core::cmp::PartialEq for IHttpNegotiate3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpNegotiate3 {}
impl ::core::fmt::Debug for IHttpNegotiate3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpNegotiate3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHttpNegotiate3 {
    type Vtable = IHttpNegotiate3_Vtbl;
}
impl ::core::clone::Clone for IHttpNegotiate3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpNegotiate3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57b6c80a_34c2_4602_bc26_66a02fc57153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate3_Vtbl {
    pub base__: IHttpNegotiate2_Vtbl,
    pub GetSerializedClientCertContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IHttpSecurity(::windows::core::IUnknown);
impl IHttpSecurity {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.GetWindow)(::windows::core::Interface::as_raw(self), rguidreason, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnSecurityProblem(&self, dwproblem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSecurityProblem)(::windows::core::Interface::as_raw(self), dwproblem).ok()
    }
}
::windows::imp::interface_hierarchy!(IHttpSecurity, ::windows::core::IUnknown, IWindowForBindingUI);
impl ::core::cmp::PartialEq for IHttpSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpSecurity {}
impl ::core::fmt::Debug for IHttpSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHttpSecurity {
    type Vtable = IHttpSecurity_Vtbl;
}
impl ::core::clone::Clone for IHttpSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpSecurity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d7_bafa_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpSecurity_Vtbl {
    pub base__: IWindowForBindingUI_Vtbl,
    pub OnSecurityProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproblem: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternet(::windows::core::IUnknown);
impl IInternet {}
::windows::imp::interface_hierarchy!(IInternet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternet {}
impl ::core::fmt::Debug for IInternet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternet {
    type Vtable = IInternet_Vtbl;
}
impl ::core::clone::Clone for IInternet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetBindInfo(::windows::core::IUnknown);
impl IInternetBindInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindInfo)(::windows::core::Interface::as_raw(self), grfbindf, pbindinfo).ok()
    }
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut ::windows::core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindString)(::windows::core::Interface::as_raw(self), ulstringtype, ppwzstr, cel, pcelfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetBindInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetBindInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetBindInfo {}
impl ::core::fmt::Debug for IInternetBindInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetBindInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetBindInfo {
    type Vtable = IInternetBindInfo_Vtbl;
}
impl ::core::clone::Clone for IInternetBindInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetBindInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e1_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetBindInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfo: usize,
    pub GetBindString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut ::windows::core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetBindInfoEx(::windows::core::IUnknown);
impl IInternetBindInfoEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBindInfo)(::windows::core::Interface::as_raw(self), grfbindf, pbindinfo).ok()
    }
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut ::windows::core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBindString)(::windows::core::Interface::as_raw(self), ulstringtype, ppwzstr, cel, pcelfetched).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindInfoEx)(::windows::core::Interface::as_raw(self), grfbindf, pbindinfo, grfbindf2, pdwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetBindInfoEx, ::windows::core::IUnknown, IInternetBindInfo);
impl ::core::cmp::PartialEq for IInternetBindInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetBindInfoEx {}
impl ::core::fmt::Debug for IInternetBindInfoEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetBindInfoEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetBindInfoEx {
    type Vtable = IInternetBindInfoEx_Vtbl;
}
impl ::core::clone::Clone for IInternetBindInfoEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetBindInfoEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3e015b7_a82c_4dcd_a150_569aeeed36ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetBindInfoEx_Vtbl {
    pub base__: IInternetBindInfo_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetHostSecurityManager(::windows::core::IUnknown);
impl IInternetHostSecurityManager {
    pub unsafe fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSecurityId)(::windows::core::Interface::as_raw(self), pbsecurityid, pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction(&self, dwaction: u32, ppolicy: &mut [u8], pcontext: ::core::option::Option<&[u8]>, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessUrlAction)(::windows::core::Interface::as_raw(self), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, ::core::mem::transmute(pcontext.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcontext.as_deref().map_or(0, |slice| slice.len() as _), dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy(&self, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: &[u8], dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryCustomPolicy)(::windows::core::Interface::as_raw(self), guidkey, pppolicy, pcbpolicy, ::core::mem::transmute(pcontext.as_ptr()), pcontext.len() as _, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetHostSecurityManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetHostSecurityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetHostSecurityManager {}
impl ::core::fmt::Debug for IInternetHostSecurityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetHostSecurityManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetHostSecurityManager {
    type Vtable = IInternetHostSecurityManager_Vtbl;
}
impl ::core::clone::Clone for IInternetHostSecurityManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetHostSecurityManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af280b6_cb3f_11d0_891e_00c04fb6bfc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetHostSecurityManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    pub ProcessUrlAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub QueryCustomPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetPriority(::windows::core::IUnknown);
impl IInternetPriority {
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), npriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetPriority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IInternetPriority, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetPriority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetPriority {}
impl ::core::fmt::Debug for IInternetPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetPriority {
    type Vtable = IInternetPriority_Vtbl;
}
impl ::core::clone::Clone for IInternetPriority {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetPriority {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9eb_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetPriority_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetProtocol(::windows::core::IUnknown);
impl IInternetProtocol {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<P0, P1, P2, P3>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IInternetProtocolSink>,
        P2: ::windows::core::IntoParam<IInternetBindInfo>,
        P3: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE_PTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), grfpi, dwreserved.into_param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Continue)(::windows::core::Interface::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Abort)(::windows::core::Interface::as_raw(self), hrreason, dwoptions).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Terminate)(::windows::core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Read(&self, pv: &mut [u8], pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv.as_ptr()), pv.len() as _, pcbread).ok()
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, &mut result__).from_abi(result__)
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRequest)(::windows::core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn UnlockRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRequest)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetProtocol, ::windows::core::IUnknown, IInternetProtocolRoot);
impl ::core::cmp::PartialEq for IInternetProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocol {}
impl ::core::fmt::Debug for IInternetProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocol {
    type Vtable = IInternetProtocol_Vtbl;
}
impl ::core::clone::Clone for IInternetProtocol {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetProtocol {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e4_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocol_Vtbl {
    pub base__: IInternetProtocolRoot_Vtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub LockRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT,
    pub UnlockRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetProtocolEx(::windows::core::IUnknown);
impl IInternetProtocolEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<P0, P1, P2, P3>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IInternetProtocolSink>,
        P2: ::windows::core::IntoParam<IInternetBindInfo>,
        P3: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE_PTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Start)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), grfpi, dwreserved.into_param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Continue)(::windows::core::Interface::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Abort)(::windows::core::Interface::as_raw(self), hrreason, dwoptions).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Terminate)(::windows::core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Read(&self, pv: &mut [u8], pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv.as_ptr()), pv.len() as _, pcbread).ok()
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, &mut result__).from_abi(result__)
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRequest)(::windows::core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn UnlockRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRequest)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartEx<P0, P1, P2, P3>(&self, puri: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IUri>,
        P1: ::windows::core::IntoParam<IInternetProtocolSink>,
        P2: ::windows::core::IntoParam<IInternetBindInfo>,
        P3: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE_PTR>,
    {
        (::windows::core::Interface::vtable(self).StartEx)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), grfpi, dwreserved.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetProtocolEx, ::windows::core::IUnknown, IInternetProtocolRoot, IInternetProtocol);
impl ::core::cmp::PartialEq for IInternetProtocolEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolEx {}
impl ::core::fmt::Debug for IInternetProtocolEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolEx {
    type Vtable = IInternetProtocolEx_Vtbl;
}
impl ::core::clone::Clone for IInternetProtocolEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetProtocolEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7a98e66_1010_492c_a1c8_c809e1f75905);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolEx_Vtbl {
    pub base__: IInternetProtocol_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub StartEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, poiprotsink: *mut ::core::ffi::c_void, poibindinfo: *mut ::core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartEx: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetProtocolInfo(::windows::core::IUnknown);
impl IInternetProtocolInfo {
    pub unsafe fn ParseUrl<P0>(&self, pwzurl: P0, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: ::windows::core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ParseUrl)(::windows::core::Interface::as_raw(self), pwzurl.into_param().abi(), parseaction, dwparseflags, ::core::mem::transmute(pwzresult), cchresult, pcchresult, dwreserved).ok()
    }
    pub unsafe fn CombineUrl<P0, P1, P2>(&self, pwzbaseurl: P0, pwzrelativeurl: P1, dwcombineflags: u32, pwzresult: P2, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CombineUrl)(::windows::core::Interface::as_raw(self), pwzbaseurl.into_param().abi(), pwzrelativeurl.into_param().abi(), dwcombineflags, pwzresult.into_param().abi(), cchresult, pcchresult, dwreserved).ok()
    }
    pub unsafe fn CompareUrl<P0, P1>(&self, pwzurl1: P0, pwzurl2: P1, dwcompareflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CompareUrl)(::windows::core::Interface::as_raw(self), pwzurl1.into_param().abi(), pwzurl2.into_param().abi(), dwcompareflags).ok()
    }
    pub unsafe fn QueryInfo<P0>(&self, pwzurl: P0, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).QueryInfo)(::windows::core::Interface::as_raw(self), pwzurl.into_param().abi(), oueryoption, dwqueryflags, pbuffer, cbbuffer, pcbbuf, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetProtocolInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetProtocolInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolInfo {}
impl ::core::fmt::Debug for IInternetProtocolInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolInfo {
    type Vtable = IInternetProtocolInfo_Vtbl;
}
impl ::core::clone::Clone for IInternetProtocolInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetProtocolInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ec_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ParseUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzurl: ::windows::core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: ::windows::core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub CombineUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzbaseurl: ::windows::core::PCWSTR, pwzrelativeurl: ::windows::core::PCWSTR, dwcombineflags: u32, pwzresult: ::windows::core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub CompareUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzurl1: ::windows::core::PCWSTR, pwzurl2: ::windows::core::PCWSTR, dwcompareflags: u32) -> ::windows::core::HRESULT,
    pub QueryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzurl: ::windows::core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetProtocolRoot(::windows::core::IUnknown);
impl IInternetProtocolRoot {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<P0, P1, P2, P3>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IInternetProtocolSink>,
        P2: ::windows::core::IntoParam<IInternetBindInfo>,
        P3: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE_PTR>,
    {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), szurl.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), grfpi, dwreserved.into_param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Continue)(::windows::core::Interface::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self), hrreason, dwoptions).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetProtocolRoot, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetProtocolRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolRoot {}
impl ::core::fmt::Debug for IInternetProtocolRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolRoot").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolRoot {
    type Vtable = IInternetProtocolRoot_Vtbl;
}
impl ::core::clone::Clone for IInternetProtocolRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetProtocolRoot {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e3_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolRoot_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, poiprotsink: *mut ::core::ffi::c_void, poibindinfo: *mut ::core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Start: usize,
    pub Continue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetProtocolSink(::windows::core::IUnknown);
impl IInternetProtocolSink {
    pub unsafe fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Switch)(::windows::core::Interface::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn ReportProgress<P0>(&self, ulstatuscode: u32, szstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ReportProgress)(::windows::core::Interface::as_raw(self), ulstatuscode, szstatustext.into_param().abi()).ok()
    }
    pub unsafe fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportData)(::windows::core::Interface::as_raw(self), grfbscf, ulprogress, ulprogressmax).ok()
    }
    pub unsafe fn ReportResult<P0>(&self, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ReportResult)(::windows::core::Interface::as_raw(self), hrresult, dwerror, szresult.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetProtocolSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetProtocolSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolSink {}
impl ::core::fmt::Debug for IInternetProtocolSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolSink {
    type Vtable = IInternetProtocolSink_Vtbl;
}
impl ::core::clone::Clone for IInternetProtocolSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetProtocolSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e5_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Switch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT,
    pub ReportProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ReportData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT,
    pub ReportResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetProtocolSinkStackable(::windows::core::IUnknown);
impl IInternetProtocolSinkStackable {
    pub unsafe fn SwitchSink<P0>(&self, poiprotsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IInternetProtocolSink>,
    {
        (::windows::core::Interface::vtable(self).SwitchSink)(::windows::core::Interface::as_raw(self), poiprotsink.into_param().abi()).ok()
    }
    pub unsafe fn CommitSwitch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitSwitch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RollbackSwitch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RollbackSwitch)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetProtocolSinkStackable, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetProtocolSinkStackable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolSinkStackable {}
impl ::core::fmt::Debug for IInternetProtocolSinkStackable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolSinkStackable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolSinkStackable {
    type Vtable = IInternetProtocolSinkStackable_Vtbl;
}
impl ::core::clone::Clone for IInternetProtocolSinkStackable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetProtocolSinkStackable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9f0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolSinkStackable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SwitchSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poiprotsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CommitSwitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RollbackSwitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetSecurityManager(::windows::core::IUnknown);
impl IInternetSecurityManager {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IInternetSecurityMgrSite>,
    {
        (::windows::core::Interface::vtable(self).SetSecuritySite)(::windows::core::Interface::as_raw(self), psite.into_param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__ = ::windows::core::zeroed::<IInternetSecurityMgrSite>();
        (::windows::core::Interface::vtable(self).GetSecuritySite)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MapUrlToZone)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pdwzone, dwflags).ok()
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetSecurityId)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), ::core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ProcessUrlAction)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).QueryCustomPolicy)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
    pub unsafe fn SetZoneMapping<P0>(&self, dwzone: u32, lpszpattern: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetZoneMapping)(::windows::core::Interface::as_raw(self), dwzone, lpszpattern.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetZoneMappings)(::windows::core::Interface::as_raw(self), dwzone, ::core::mem::transmute(ppenumstring), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetSecurityManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetSecurityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityManager {}
impl ::core::fmt::Debug for IInternetSecurityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityManager {
    type Vtable = IInternetSecurityManager_Vtbl;
}
impl ::core::clone::Clone for IInternetSecurityManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetSecurityManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ee_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSecuritySite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSecuritySite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MapUrlToZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    pub ProcessUrlAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub QueryCustomPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub SetZoneMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, lpszpattern: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetZoneMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, ppenumstring: *mut *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetSecurityManagerEx(::windows::core::IUnknown);
impl IInternetSecurityManagerEx {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IInternetSecurityMgrSite>,
    {
        (::windows::core::Interface::vtable(self).base__.SetSecuritySite)(::windows::core::Interface::as_raw(self), psite.into_param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__ = ::windows::core::zeroed::<IInternetSecurityMgrSite>();
        (::windows::core::Interface::vtable(self).base__.GetSecuritySite)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.MapUrlToZone)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pdwzone, dwflags).ok()
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetSecurityId)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), ::core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.ProcessUrlAction)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.QueryCustomPolicy)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
    pub unsafe fn SetZoneMapping<P0>(&self, dwzone: u32, lpszpattern: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetZoneMapping)(::windows::core::Interface::as_raw(self), dwzone, lpszpattern.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetZoneMappings)(::windows::core::Interface::as_raw(self), dwzone, ::core::mem::transmute(ppenumstring), dwflags).ok()
    }
    pub unsafe fn ProcessUrlActionEx<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ProcessUrlActionEx)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved, pdwoutflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetSecurityManagerEx, ::windows::core::IUnknown, IInternetSecurityManager);
impl ::core::cmp::PartialEq for IInternetSecurityManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityManagerEx {}
impl ::core::fmt::Debug for IInternetSecurityManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityManagerEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityManagerEx {
    type Vtable = IInternetSecurityManagerEx_Vtbl;
}
impl ::core::clone::Clone for IInternetSecurityManagerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetSecurityManagerEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf164edf1_cc7c_4f0d_9a94_34222625c393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManagerEx_Vtbl {
    pub base__: IInternetSecurityManager_Vtbl,
    pub ProcessUrlActionEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetSecurityManagerEx2(::windows::core::IUnknown);
impl IInternetSecurityManagerEx2 {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IInternetSecurityMgrSite>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetSecuritySite)(::windows::core::Interface::as_raw(self), psite.into_param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__ = ::windows::core::zeroed::<IInternetSecurityMgrSite>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSecuritySite)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.MapUrlToZone)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), pdwzone, dwflags).ok()
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetSecurityId)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), ::core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ProcessUrlAction)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.QueryCustomPolicy)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
    pub unsafe fn SetZoneMapping<P0>(&self, dwzone: u32, lpszpattern: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetZoneMapping)(::windows::core::Interface::as_raw(self), dwzone, lpszpattern.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetZoneMappings)(::windows::core::Interface::as_raw(self), dwzone, ::core::mem::transmute(ppenumstring), dwflags).ok()
    }
    pub unsafe fn ProcessUrlActionEx<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.ProcessUrlActionEx)(::windows::core::Interface::as_raw(self), pwszurl.into_param().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved, pdwoutflags).ok()
    }
    pub unsafe fn MapUrlToZoneEx2<P0>(&self, puri: P0, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: ::core::option::Option<*mut ::windows::core::PWSTR>, pdwoutflags: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IUri>,
    {
        (::windows::core::Interface::vtable(self).MapUrlToZoneEx2)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), pdwzone, dwflags, ::core::mem::transmute(ppwszmappedurl.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwoutflags.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ProcessUrlActionEx2<P0>(&self, puri: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IUri>,
    {
        (::windows::core::Interface::vtable(self).ProcessUrlActionEx2)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved, pdwoutflags).ok()
    }
    pub unsafe fn GetSecurityIdEx2<P0>(&self, puri: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IUri>,
    {
        (::windows::core::Interface::vtable(self).GetSecurityIdEx2)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), ::core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicyEx2<P0>(&self, puri: P0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IUri>,
    {
        (::windows::core::Interface::vtable(self).QueryCustomPolicyEx2)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetSecurityManagerEx2, ::windows::core::IUnknown, IInternetSecurityManager, IInternetSecurityManagerEx);
impl ::core::cmp::PartialEq for IInternetSecurityManagerEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityManagerEx2 {}
impl ::core::fmt::Debug for IInternetSecurityManagerEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityManagerEx2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityManagerEx2 {
    type Vtable = IInternetSecurityManagerEx2_Vtbl;
}
impl ::core::clone::Clone for IInternetSecurityManagerEx2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetSecurityManagerEx2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1e50292_a795_4117_8e09_2b560a72ac60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManagerEx2_Vtbl {
    pub base__: IInternetSecurityManagerEx_Vtbl,
    pub MapUrlToZoneEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut ::windows::core::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    pub ProcessUrlActionEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetSecurityIdEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    pub QueryCustomPolicyEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetSecurityMgrSite(::windows::core::IUnknown);
impl IInternetSecurityMgrSite {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableModeless)(::windows::core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetSecurityMgrSite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetSecurityMgrSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityMgrSite {}
impl ::core::fmt::Debug for IInternetSecurityMgrSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityMgrSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityMgrSite {
    type Vtable = IInternetSecurityMgrSite_Vtbl;
}
impl ::core::clone::Clone for IInternetSecurityMgrSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetSecurityMgrSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ed_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityMgrSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableModeless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableModeless: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetSession(::windows::core::IUnknown);
impl IInternetSession {
    pub unsafe fn RegisterNameSpace<P0, P1>(&self, pcf: P0, rclsid: *const ::windows::core::GUID, pwzprotocol: P1, cpatterns: u32, ppwzpatterns: *const ::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IClassFactory>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RegisterNameSpace)(::windows::core::Interface::as_raw(self), pcf.into_param().abi(), rclsid, pwzprotocol.into_param().abi(), cpatterns, ppwzpatterns, dwreserved).ok()
    }
    pub unsafe fn UnregisterNameSpace<P0, P1>(&self, pcf: P0, pszprotocol: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IClassFactory>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UnregisterNameSpace)(::windows::core::Interface::as_raw(self), pcf.into_param().abi(), pszprotocol.into_param().abi()).ok()
    }
    pub unsafe fn RegisterMimeFilter<P0, P1>(&self, pcf: P0, rclsid: *const ::windows::core::GUID, pwztype: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IClassFactory>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RegisterMimeFilter)(::windows::core::Interface::as_raw(self), pcf.into_param().abi(), rclsid, pwztype.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterMimeFilter<P0, P1>(&self, pcf: P0, pwztype: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IClassFactory>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UnregisterMimeFilter)(::windows::core::Interface::as_raw(self), pcf.into_param().abi(), pwztype.into_param().abi()).ok()
    }
    pub unsafe fn CreateBinding<P0, P1, P2>(&self, pbc: P0, szurl: P1, punkouter: P2, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>, ppoinetprot: *mut ::core::option::Option<IInternetProtocol>, dwoption: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IBindCtx>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateBinding)(::windows::core::Interface::as_raw(self), pbc.into_param().abi(), szurl.into_param().abi(), punkouter.into_param().abi(), ::core::mem::transmute(ppunk), ::core::mem::transmute(ppoinetprot), dwoption).ok()
    }
    pub unsafe fn SetSessionOption(&self, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSessionOption)(::windows::core::Interface::as_raw(self), dwoption, pbuffer, dwbufferlength, dwreserved).ok()
    }
    pub unsafe fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSessionOption)(::windows::core::Interface::as_raw(self), dwoption, pbuffer, pdwbufferlength, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetSession, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSession {}
impl ::core::fmt::Debug for IInternetSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetSession {
    type Vtable = IInternetSession_Vtbl;
}
impl ::core::clone::Clone for IInternetSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e7_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterNameSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pwzprotocol: ::windows::core::PCWSTR, cpatterns: u32, ppwzpatterns: *const ::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::HRESULT,
    pub UnregisterNameSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, pszprotocol: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RegisterMimeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pwztype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub UnregisterMimeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, pwztype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub CreateBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, punkouter: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void, ppoinetprot: *mut *mut ::core::ffi::c_void, dwoption: u32) -> ::windows::core::HRESULT,
    pub SetSessionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub GetSessionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetThreadSwitch(::windows::core::IUnknown);
impl IInternetThreadSwitch {
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Prepare)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Continue)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetThreadSwitch, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetThreadSwitch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetThreadSwitch {}
impl ::core::fmt::Debug for IInternetThreadSwitch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetThreadSwitch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetThreadSwitch {
    type Vtable = IInternetThreadSwitch_Vtbl;
}
impl ::core::clone::Clone for IInternetThreadSwitch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetThreadSwitch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e8_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetThreadSwitch_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Prepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Continue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetZoneManager(::windows::core::IUnknown);
impl IInternetZoneManager {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetZoneAttributes)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetZoneAttributes)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetZoneCustomPolicy)(::windows::core::Interface::as_raw(self), dwzone, guidkey, pppolicy, pcbpolicy, urlzonereg).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetZoneCustomPolicy)(::windows::core::Interface::as_raw(self), dwzone, guidkey, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetZoneActionPolicy)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetZoneActionPolicy)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<P0, P1, P2>(&self, dwaction: u32, hwndparent: P0, pwszurl: P1, pwsztext: P2, dwpromptflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).PromptAction)(::windows::core::Interface::as_raw(self), dwaction, hwndparent.into_param().abi(), pwszurl.into_param().abi(), pwsztext.into_param().abi(), dwpromptflags).ok()
    }
    pub unsafe fn LogAction<P0, P1>(&self, dwaction: u32, pwszurl: P0, pwsztext: P1, dwlogflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LogAction)(::windows::core::Interface::as_raw(self), dwaction, pwszurl.into_param().abi(), pwsztext.into_param().abi(), dwlogflags).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateZoneEnumerator)(::windows::core::Interface::as_raw(self), pdwenum, pdwcount, dwflags).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetZoneAt)(::windows::core::Interface::as_raw(self), dwenum, dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DestroyZoneEnumerator)(::windows::core::Interface::as_raw(self), dwenum).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTemplatePoliciesToZone)(::windows::core::Interface::as_raw(self), dwtemplate, dwzone, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetZoneManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetZoneManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetZoneManager {}
impl ::core::fmt::Debug for IInternetZoneManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetZoneManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetZoneManager {
    type Vtable = IInternetZoneManager_Vtbl;
}
impl ::core::clone::Clone for IInternetZoneManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetZoneManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ef_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetZoneAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub SetZoneAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub GetZoneCustomPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub SetZoneCustomPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub GetZoneActionPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub SetZoneActionPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptAction: usize,
    pub LogAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaction: u32, pwszurl: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR, dwlogflags: u32) -> ::windows::core::HRESULT,
    pub CreateZoneEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetZoneAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub DestroyZoneEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenum: u32) -> ::windows::core::HRESULT,
    pub CopyTemplatePoliciesToZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetZoneManagerEx(::windows::core::IUnknown);
impl IInternetZoneManagerEx {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetZoneAttributes)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetZoneAttributes)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetZoneCustomPolicy)(::windows::core::Interface::as_raw(self), dwzone, guidkey, pppolicy, pcbpolicy, urlzonereg).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetZoneCustomPolicy)(::windows::core::Interface::as_raw(self), dwzone, guidkey, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetZoneActionPolicy)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetZoneActionPolicy)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<P0, P1, P2>(&self, dwaction: u32, hwndparent: P0, pwszurl: P1, pwsztext: P2, dwpromptflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.PromptAction)(::windows::core::Interface::as_raw(self), dwaction, hwndparent.into_param().abi(), pwszurl.into_param().abi(), pwsztext.into_param().abi(), dwpromptflags).ok()
    }
    pub unsafe fn LogAction<P0, P1>(&self, dwaction: u32, pwszurl: P0, pwsztext: P1, dwlogflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.LogAction)(::windows::core::Interface::as_raw(self), dwaction, pwszurl.into_param().abi(), pwsztext.into_param().abi(), dwlogflags).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CreateZoneEnumerator)(::windows::core::Interface::as_raw(self), pdwenum, pdwcount, dwflags).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetZoneAt)(::windows::core::Interface::as_raw(self), dwenum, dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DestroyZoneEnumerator)(::windows::core::Interface::as_raw(self), dwenum).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyTemplatePoliciesToZone)(::windows::core::Interface::as_raw(self), dwtemplate, dwzone, dwreserved).ok()
    }
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetZoneActionPolicyEx)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg, dwflags).ok()
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetZoneActionPolicyEx)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg, dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetZoneManagerEx, ::windows::core::IUnknown, IInternetZoneManager);
impl ::core::cmp::PartialEq for IInternetZoneManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetZoneManagerEx {}
impl ::core::fmt::Debug for IInternetZoneManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetZoneManagerEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetZoneManagerEx {
    type Vtable = IInternetZoneManagerEx_Vtbl;
}
impl ::core::clone::Clone for IInternetZoneManagerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetZoneManagerEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c23339_8e06_431e_9bf4_7e711c085648);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManagerEx_Vtbl {
    pub base__: IInternetZoneManager_Vtbl,
    pub GetZoneActionPolicyEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT,
    pub SetZoneActionPolicyEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IInternetZoneManagerEx2(::windows::core::IUnknown);
impl IInternetZoneManagerEx2 {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetZoneAttributes)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetZoneAttributes)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetZoneCustomPolicy)(::windows::core::Interface::as_raw(self), dwzone, guidkey, pppolicy, pcbpolicy, urlzonereg).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetZoneCustomPolicy)(::windows::core::Interface::as_raw(self), dwzone, guidkey, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetZoneActionPolicy)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetZoneActionPolicy)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<P0, P1, P2>(&self, dwaction: u32, hwndparent: P0, pwszurl: P1, pwsztext: P2, dwpromptflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.PromptAction)(::windows::core::Interface::as_raw(self), dwaction, hwndparent.into_param().abi(), pwszurl.into_param().abi(), pwsztext.into_param().abi(), dwpromptflags).ok()
    }
    pub unsafe fn LogAction<P0, P1>(&self, dwaction: u32, pwszurl: P0, pwsztext: P1, dwlogflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.LogAction)(::windows::core::Interface::as_raw(self), dwaction, pwszurl.into_param().abi(), pwsztext.into_param().abi(), dwlogflags).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CreateZoneEnumerator)(::windows::core::Interface::as_raw(self), pdwenum, pdwcount, dwflags).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetZoneAt)(::windows::core::Interface::as_raw(self), dwenum, dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DestroyZoneEnumerator)(::windows::core::Interface::as_raw(self), dwenum).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTemplatePoliciesToZone)(::windows::core::Interface::as_raw(self), dwtemplate, dwzone, dwreserved).ok()
    }
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetZoneActionPolicyEx)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg, dwflags).ok()
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetZoneActionPolicyEx)(::windows::core::Interface::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg, dwflags).ok()
    }
    pub unsafe fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetZoneAttributesEx)(::windows::core::Interface::as_raw(self), dwzone, pzoneattributes, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetZoneSecurityState<P0>(&self, dwzoneindex: u32, frespectpolicy: P0, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetZoneSecurityState)(::windows::core::Interface::as_raw(self), dwzoneindex, frespectpolicy.into_param().abi(), pdwstate, pfpolicyencountered).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIESecurityState<P0, P1>(&self, frespectpolicy: P0, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetIESecurityState)(::windows::core::Interface::as_raw(self), frespectpolicy.into_param().abi(), pdwstate, pfpolicyencountered, fnocache.into_param().abi()).ok()
    }
    pub unsafe fn FixUnsecureSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FixUnsecureSettings)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternetZoneManagerEx2, ::windows::core::IUnknown, IInternetZoneManager, IInternetZoneManagerEx);
impl ::core::cmp::PartialEq for IInternetZoneManagerEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetZoneManagerEx2 {}
impl ::core::fmt::Debug for IInternetZoneManagerEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetZoneManagerEx2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetZoneManagerEx2 {
    type Vtable = IInternetZoneManagerEx2_Vtbl;
}
impl ::core::clone::Clone for IInternetZoneManagerEx2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetZoneManagerEx2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedc17559_dd5d_4846_8eef_8becba5a4abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManagerEx2_Vtbl {
    pub base__: IInternetZoneManagerEx_Vtbl,
    pub GetZoneAttributesEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetZoneSecurityState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetZoneSecurityState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIESecurityState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIESecurityState: usize,
    pub FixUnsecureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IMonikerProp(::windows::core::IUnknown);
impl IMonikerProp {
    pub unsafe fn PutProperty<P0>(&self, mkp: MONIKERPROPERTY, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).PutProperty)(::windows::core::Interface::as_raw(self), mkp, val.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMonikerProp, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMonikerProp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMonikerProp {}
impl ::core::fmt::Debug for IMonikerProp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMonikerProp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMonikerProp {
    type Vtable = IMonikerProp_Vtbl;
}
impl ::core::clone::Clone for IMonikerProp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMonikerProp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ca5f7f_1847_4d87_9c5b_918509f7511d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMonikerProp_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PutProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mkp: MONIKERPROPERTY, val: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IPersistMoniker(::windows::core::IUnknown);
impl IPersistMoniker {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetClassID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).IsDirty)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<P0, P1, P2>(&self, ffullyavailable: P0, pimkname: P1, pibc: P2, grfmode: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::IMoniker>,
        P2: ::windows::core::IntoParam<super::IBindCtx>,
    {
        (::windows::core::Interface::vtable(self).Load)(::windows::core::Interface::as_raw(self), ffullyavailable.into_param().abi(), pimkname.into_param().abi(), pibc.into_param().abi(), grfmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1, P2>(&self, pimkname: P0, pbc: P1, fremember: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IMoniker>,
        P1: ::windows::core::IntoParam<super::IBindCtx>,
        P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Save)(::windows::core::Interface::as_raw(self), pimkname.into_param().abi(), pbc.into_param().abi(), fremember.into_param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<P0, P1>(&self, pimkname: P0, pibc: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IMoniker>,
        P1: ::windows::core::IntoParam<super::IBindCtx>,
    {
        (::windows::core::Interface::vtable(self).SaveCompleted)(::windows::core::Interface::as_raw(self), pimkname.into_param().abi(), pibc.into_param().abi()).ok()
    }
    pub unsafe fn GetCurMoniker(&self) -> ::windows::core::Result<super::IMoniker> {
        let mut result__ = ::windows::core::zeroed::<super::IMoniker>();
        (::windows::core::Interface::vtable(self).GetCurMoniker)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPersistMoniker, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPersistMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMoniker {}
impl ::core::fmt::Debug for IPersistMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistMoniker {
    type Vtable = IPersistMoniker_Vtbl;
}
impl ::core::clone::Clone for IPersistMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPersistMoniker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c9_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMoniker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: *mut ::core::ffi::c_void, pibc: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Load: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimkname: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimkname: *mut ::core::ffi::c_void, pibc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimkname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct ISoftDistExt(::windows::core::IUnknown);
impl ISoftDistExt {
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`*"]
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn ProcessSoftDist<P0, P1>(&self, szcdfurl: P0, psoftdistelement: P1, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Data::Xml::MsXml::IXMLElement>,
    {
        (::windows::core::Interface::vtable(self).ProcessSoftDist)(::windows::core::Interface::as_raw(self), szcdfurl.into_param().abi(), psoftdistelement.into_param().abi(), lpsdi).ok()
    }
    pub unsafe fn GetFirstCodeBase(&self, szcodebase: *const ::windows::core::PCWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFirstCodeBase)(::windows::core::Interface::as_raw(self), szcodebase, dwmaxsize).ok()
    }
    pub unsafe fn GetNextCodeBase(&self, szcodebase: *const ::windows::core::PCWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextCodeBase)(::windows::core::Interface::as_raw(self), szcodebase, dwmaxsize).ok()
    }
    pub unsafe fn AsyncInstallDistributionUnit<P0>(&self, pbc: P0, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IBindCtx>,
    {
        (::windows::core::Interface::vtable(self).AsyncInstallDistributionUnit)(::windows::core::Interface::as_raw(self), pbc.into_param().abi(), pvreserved, flags, lpcbh).ok()
    }
}
::windows::imp::interface_hierarchy!(ISoftDistExt, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISoftDistExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISoftDistExt {}
impl ::core::fmt::Debug for ISoftDistExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISoftDistExt").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISoftDistExt {
    type Vtable = ISoftDistExt_Vtbl;
}
impl ::core::clone::Clone for ISoftDistExt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISoftDistExt {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15b8dc1_c7e1_11d0_8680_00aa00bdcb71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftDistExt_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub ProcessSoftDist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szcdfurl: ::windows::core::PCWSTR, psoftdistelement: *mut ::core::ffi::c_void, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))]
    ProcessSoftDist: usize,
    pub GetFirstCodeBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szcodebase: *const ::windows::core::PCWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT,
    pub GetNextCodeBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szcodebase: *const ::windows::core::PCWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT,
    pub AsyncInstallDistributionUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IUriBuilderFactory(::windows::core::IUnknown);
impl IUriBuilderFactory {
    pub unsafe fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder> {
        let mut result__ = ::windows::core::zeroed::<super::IUriBuilder>();
        (::windows::core::Interface::vtable(self).CreateIUriBuilder)(::windows::core::Interface::as_raw(self), dwflags, dwreserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder> {
        let mut result__ = ::windows::core::zeroed::<super::IUriBuilder>();
        (::windows::core::Interface::vtable(self).CreateInitializedIUriBuilder)(::windows::core::Interface::as_raw(self), dwflags, dwreserved, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IUriBuilderFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IUriBuilderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilderFactory {}
impl ::core::fmt::Debug for IUriBuilderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilderFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUriBuilderFactory {
    type Vtable = IUriBuilderFactory_Vtbl;
}
impl ::core::clone::Clone for IUriBuilderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUriBuilderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe982ce48_0b96_440c_bc37_0c869b27a29e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilderFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateIUriBuilder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInitializedIUriBuilder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IUriContainer(::windows::core::IUnknown);
impl IUriContainer {
    pub unsafe fn GetIUri(&self) -> ::windows::core::Result<super::IUri> {
        let mut result__ = ::windows::core::zeroed::<super::IUri>();
        (::windows::core::Interface::vtable(self).GetIUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IUriContainer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IUriContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriContainer {}
impl ::core::fmt::Debug for IUriContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUriContainer {
    type Vtable = IUriContainer_Vtbl;
}
impl ::core::clone::Clone for IUriContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUriContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa158a630_ed6f_45fb_b987_f68676f57752);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWinInetCacheHints(::windows::core::IUnknown);
impl IWinInetCacheHints {
    pub unsafe fn SetCacheExtension<P0>(&self, pwzext: P0, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCacheExtension)(::windows::core::Interface::as_raw(self), pwzext.into_param().abi(), pszcachefile, pcbcachefile, pdwwinineterror, pdwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinInetCacheHints, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWinInetCacheHints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetCacheHints {}
impl ::core::fmt::Debug for IWinInetCacheHints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetCacheHints").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinInetCacheHints {
    type Vtable = IWinInetCacheHints_Vtbl;
}
impl ::core::clone::Clone for IWinInetCacheHints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinInetCacheHints {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd1ec3b3_8391_4fdb_a9e6_347c3caaa7dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetCacheHints_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetCacheExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzext: ::windows::core::PCWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWinInetCacheHints2(::windows::core::IUnknown);
impl IWinInetCacheHints2 {
    pub unsafe fn SetCacheExtension<P0>(&self, pwzext: P0, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetCacheExtension)(::windows::core::Interface::as_raw(self), pwzext.into_param().abi(), pszcachefile, pcbcachefile, pdwwinineterror, pdwreserved).ok()
    }
    pub unsafe fn SetCacheExtension2<P0>(&self, pwzext: P0, pwzcachefile: ::windows::core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCacheExtension2)(::windows::core::Interface::as_raw(self), pwzext.into_param().abi(), ::core::mem::transmute(pwzcachefile), pcchcachefile, pdwwinineterror, pdwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinInetCacheHints2, ::windows::core::IUnknown, IWinInetCacheHints);
impl ::core::cmp::PartialEq for IWinInetCacheHints2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetCacheHints2 {}
impl ::core::fmt::Debug for IWinInetCacheHints2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetCacheHints2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinInetCacheHints2 {
    type Vtable = IWinInetCacheHints2_Vtbl;
}
impl ::core::clone::Clone for IWinInetCacheHints2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinInetCacheHints2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7857aeac_d31f_49bf_884e_dd46df36780a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetCacheHints2_Vtbl {
    pub base__: IWinInetCacheHints_Vtbl,
    pub SetCacheExtension2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzext: ::windows::core::PCWSTR, pwzcachefile: ::windows::core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWinInetFileStream(::windows::core::IUnknown);
impl IWinInetFileStream {
    pub unsafe fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHandleForUnlock)(::windows::core::Interface::as_raw(self), hwininetlockhandle, dwreserved).ok()
    }
    pub unsafe fn SetDeleteFile(&self, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeleteFile)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinInetFileStream, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWinInetFileStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetFileStream {}
impl ::core::fmt::Debug for IWinInetFileStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetFileStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinInetFileStream {
    type Vtable = IWinInetFileStream_Vtbl;
}
impl ::core::clone::Clone for IWinInetFileStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinInetFileStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf134c4b7_b1f8_4e75_b886_74b90943becb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetFileStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetHandleForUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::HRESULT,
    pub SetDeleteFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWinInetHttpInfo(::windows::core::IUnknown);
impl IWinInetHttpInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryOption)(::windows::core::Interface::as_raw(self), dwoption, pbuffer, pcbbuf).ok()
    }
    pub unsafe fn QueryInfo(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryInfo)(::windows::core::Interface::as_raw(self), dwoption, pbuffer, pcbbuf, pdwflags, pdwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinInetHttpInfo, ::windows::core::IUnknown, IWinInetInfo);
impl ::core::cmp::PartialEq for IWinInetHttpInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetHttpInfo {}
impl ::core::fmt::Debug for IWinInetHttpInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetHttpInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinInetHttpInfo {
    type Vtable = IWinInetHttpInfo_Vtbl;
}
impl ::core::clone::Clone for IWinInetHttpInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinInetHttpInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d8_bafa_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetHttpInfo_Vtbl {
    pub base__: IWinInetInfo_Vtbl,
    pub QueryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWinInetHttpTimeouts(::windows::core::IUnknown);
impl IWinInetHttpTimeouts {
    pub unsafe fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRequestTimeouts)(::windows::core::Interface::as_raw(self), pdwconnecttimeout, pdwsendtimeout, pdwreceivetimeout).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinInetHttpTimeouts, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWinInetHttpTimeouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetHttpTimeouts {}
impl ::core::fmt::Debug for IWinInetHttpTimeouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetHttpTimeouts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinInetHttpTimeouts {
    type Vtable = IWinInetHttpTimeouts_Vtbl;
}
impl ::core::clone::Clone for IWinInetHttpTimeouts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinInetHttpTimeouts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf286fa56_c1fd_4270_8e67_b3eb790a81e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetHttpTimeouts_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRequestTimeouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWinInetInfo(::windows::core::IUnknown);
impl IWinInetInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryOption)(::windows::core::Interface::as_raw(self), dwoption, pbuffer, pcbbuf).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinInetInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWinInetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetInfo {}
impl ::core::fmt::Debug for IWinInetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinInetInfo {
    type Vtable = IWinInetInfo_Vtbl;
}
impl ::core::clone::Clone for IWinInetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinInetInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d6_bafa_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWindowForBindingUI(::windows::core::IUnknown);
impl IWindowForBindingUI {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWindow)(::windows::core::Interface::as_raw(self), rguidreason, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWindowForBindingUI, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWindowForBindingUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowForBindingUI {}
impl ::core::fmt::Debug for IWindowForBindingUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowForBindingUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowForBindingUI {
    type Vtable = IWindowForBindingUI_Vtbl;
}
impl ::core::clone::Clone for IWindowForBindingUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowForBindingUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d5_bafa_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowForBindingUI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IWrappedProtocol(::windows::core::IUnknown);
impl IWrappedProtocol {
    pub unsafe fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWrapperCode)(::windows::core::Interface::as_raw(self), pncode, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IWrappedProtocol, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWrappedProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWrappedProtocol {}
impl ::core::fmt::Debug for IWrappedProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWrappedProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWrappedProtocol {
    type Vtable = IWrappedProtocol_Vtbl;
}
impl ::core::clone::Clone for IWrappedProtocol {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWrappedProtocol {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53c84785_8425_4dc5_971b_e58d9c19f9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWrappedProtocol_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetWrapperCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IZoneIdentifier(::windows::core::IUnknown);
impl IZoneIdentifier {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), dwzone).ok()
    }
    pub unsafe fn Remove(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IZoneIdentifier, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IZoneIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IZoneIdentifier {}
impl ::core::fmt::Debug for IZoneIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IZoneIdentifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IZoneIdentifier {
    type Vtable = IZoneIdentifier_Vtbl;
}
impl ::core::clone::Clone for IZoneIdentifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IZoneIdentifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd45f185_1b21_48e2_967b_ead743a8914e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoneIdentifier_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwzone: u32) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
pub struct IZoneIdentifier2(::windows::core::IUnknown);
impl IZoneIdentifier2 {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), dwzone).ok()
    }
    pub unsafe fn Remove(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Remove)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLastWriterPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetLastWriterPackageFamilyName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastWriterPackageFamilyName<P0>(&self, packagefamilyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLastWriterPackageFamilyName)(::windows::core::Interface::as_raw(self), packagefamilyname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveLastWriterPackageFamilyName(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveLastWriterPackageFamilyName)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAppZoneId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAppZoneId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAppZoneId(&self, zone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppZoneId)(::windows::core::Interface::as_raw(self), zone).ok()
    }
    pub unsafe fn RemoveAppZoneId(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAppZoneId)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IZoneIdentifier2, ::windows::core::IUnknown, IZoneIdentifier);
impl ::core::cmp::PartialEq for IZoneIdentifier2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IZoneIdentifier2 {}
impl ::core::fmt::Debug for IZoneIdentifier2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IZoneIdentifier2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IZoneIdentifier2 {
    type Vtable = IZoneIdentifier2_Vtbl;
}
impl ::core::clone::Clone for IZoneIdentifier2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IZoneIdentifier2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb5e760c_09ef_45c0_b510_70830ce31e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoneIdentifier2_Vtbl {
    pub base__: IZoneIdentifier_Vtbl,
    pub GetLastWriterPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLastWriterPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveLastWriterPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAppZoneId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zone: *mut u32) -> ::windows::core::HRESULT,
    pub SetAppZoneId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zone: u32) -> ::windows::core::HRESULT,
    pub RemoveAppZoneId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CF_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const E_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147483638i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FIEF_FLAG_FORCE_JITUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FIEF_FLAG_PEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FIEF_FLAG_RESERVED_0: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_ENABLEMIMESNIFFING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_IGNOREMIMETEXTPLAIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_RESERVED_1: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_RESERVED_2: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_RESPECTTEXTPLAIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_RETURNUPDATEDIMGMIMES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_SERVERMIME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FMFD_URLASFILENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_PROCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_THREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_THREAD_INTERNET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_THREAD_INTRANET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const GET_FEATURE_FROM_THREAD_TRUSTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_AUTHENTICATION_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697207i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695930i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695931i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697189i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CANNOT_CONNECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697212i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697200i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CANNOT_LOAD_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697201i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CANNOT_LOCK_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697194i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CANNOT_REPLACE_SFP_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696448i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CODE_DOWNLOAD_DECLINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696960i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695932i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695929i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695936i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695934i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CODE_INSTALL_SUPPRESSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696192i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_CONNECTION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697205i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_DATA_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697209i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199i32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_DOMINJECTIONVALIDATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697188i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695928i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695935i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_DOWNLOAD_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697208i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_ERROR_FIRST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697214i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_ERROR_LAST: i32 = -2146695928i32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_FORBIDFRAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695933i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_HSTS_CERTIFICATE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697186i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_INVALID_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697191i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_INVALID_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697204i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_INVALID_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697214i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_NO_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697213i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_NO_VALID_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697206i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697210i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_QUERYOPTION_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697197i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_REDIRECTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697196i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_REDIRECT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697196i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_REDIRECT_TO_DIR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697195i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESERVED_1: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697190i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESERVED_2: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697185i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESERVED_3: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697184i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESERVED_4: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697183i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESERVED_5: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697182i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESOURCE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697211i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_RESULT_DISPATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696704i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_SECURITY_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697202i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_TERMINATED_BIND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697192i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_UNKNOWN_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697203i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697199i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_USE_DEFAULT_SETTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697198i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_USE_EXTEND_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697193i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697187i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MAX_SIZE_SECURITY_ID: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MKSYS_URLMONIKER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MK_S_ASYNCHRONOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(262632i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_DONT_UNESCAPE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_DONT_USE_CACHE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_ENFORCERESTRICTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_FORCE_INTRANET_FLAGS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_IGNORE_ZONE_MAPPINGS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_ISFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_NOSAVEDFILECHECK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_REQUIRESAVEDFILECHECK: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MUTZ_RESERVED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SECURITY_IE_STATE_GREEN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SECURITY_IE_STATE_RED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_IN_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_PROCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_THREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_THREAD_INTERNET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_THREAD_INTRANET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_THREAD_RESTRICTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SET_FEATURE_ON_THREAD_TRUSTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_ADSTATE_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_ADSTATE_DOWNLOADED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_ADSTATE_INSTALLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_ADSTATE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_FLAG_USAGE_EMAIL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SOFTDIST_FLAG_USAGE_PRECACHE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const S_ASYNCHRONOUS: i32 = 262632i32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const UAS_EXACTLEGACY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_ALLOW_TDC: u32 = 4620u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: u32 = 4612u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_CURR_MAX: u32 = 4620u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: u32 = 4618u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_MAX: u32 = 5119u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_MIN: u32 = 4608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: u32 = 4614u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: u32 = 4610u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: u32 = 4619u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: u32 = 4609u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: u32 = 4616u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: u32 = 4615u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: u32 = 4611u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_RUN: u32 = 4608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: u32 = 4617u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: u32 = 4613u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: u32 = 9986u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: u32 = 9996u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_APEVALUATION: u32 = 8961u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_AUDIO_VIDEO: u32 = 9985u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: u32 = 9988u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: u32 = 9994u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: u32 = 9993u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: u32 = 9992u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: u32 = 9997u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_JSCRIPT_IE: u32 = 5133u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: u32 = 9995u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: u32 = 8960u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: u32 = 9987u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_VBSCRIPT_IE: u32 = 5132u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: u32 = 5128u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_XHR_EVALUATION: u32 = 8962u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: u32 = 9990u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: u32 = 9989u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_AUTHENTICATE_CLIENT: u32 = 6657u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: u32 = 8705u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: u32 = 8704u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: u32 = 8704u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_BEHAVIOR_MIN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_BEHAVIOR_RUN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_CHANNEL_SOFTDIST_MAX: u32 = 7935u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_CHANNEL_SOFTDIST_MIN: u32 = 7680u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: u32 = 7685u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_CLIENT_CERT_PROMPT: u32 = 6660u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_COOKIES: u32 = 6658u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_COOKIES_ENABLED: u32 = 6672u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_COOKIES_SESSION: u32 = 6659u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: u32 = 6662u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_COOKIES_THIRD_PARTY: u32 = 6661u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_CREDENTIALS_USE: u32 = 6656u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_CROSS_DOMAIN_DATA: u32 = 5126u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_DOTNET_USERCONTROLS: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_DOWNLOAD_CURR_MAX: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_DOWNLOAD_MAX: u32 = 4607u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_DOWNLOAD_MIN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: u32 = 8453u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: u32 = 8455u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_DATA_BINDING: u32 = 8454u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: u32 = 8452u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_MIME_SNIFFING: u32 = 8448u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_MIN: u32 = 8448u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: u32 = 8451u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: u32 = 8450u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_FEATURE_ZONE_ELEVATION: u32 = 8449u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: u32 = 5645u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: u32 = 5648u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: u32 = 5647u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_ALLOW_INDEXEDDB: u32 = 5649u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: u32 = 5643u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: u32 = 5646u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_FONT_DOWNLOAD: u32 = 5636u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_INCLUDE_FILE_PATH: u32 = 5642u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_JAVA_RUN: u32 = 5637u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_MAX: u32 = 6143u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_META_REFRESH: u32 = 5640u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_MIN: u32 = 5632u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_MIXED_CONTENT: u32 = 5641u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: u32 = 5644u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: u32 = 5639u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_SUBMIT_FORMS: u32 = 5633u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: u32 = 5634u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_SUBMIT_FORMS_TO: u32 = 5635u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_HTML_USERDATA_SAVE: u32 = 5638u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_CURR_MAX: u32 = 7430u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_MAX: u32 = 7679u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_MIN: u32 = 7424u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: u32 = 7424u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: u32 = 7427u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: u32 = 7430u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: u32 = 7425u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: u32 = 7428u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: u32 = 7426u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: u32 = 7429u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_INPRIVATE_BLOCKING: u32 = 9984u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_JAVA_CURR_MAX: u32 = 7168u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_JAVA_MAX: u32 = 7423u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_JAVA_MIN: u32 = 7168u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_JAVA_PERMISSIONS: u32 = 7168u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_LOOSE_XAML: u32 = 9218u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_LOWRIGHTS: u32 = 9472u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_MIN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_NETWORK_CURR_MAX: u32 = 6672u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_NETWORK_MAX: u32 = 7167u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_NETWORK_MIN: u32 = 6656u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: u32 = 5131u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_CURR_MAX: u32 = 5133u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_JAVA_USE: u32 = 5122u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_MAX: u32 = 5631u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_MIN: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_NAVIGATE: u32 = 5130u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: u32 = 5121u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_PASTE: u32 = 5127u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_RUN: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: u32 = 5125u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SCRIPT_XSSFILTER: u32 = 5129u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: u32 = 6161u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_CURR_MAX: u32 = 6162u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: u32 = 6155u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: u32 = 6150u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_EXECUTE_LOWRISK: u32 = 6152u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_EXECUTE_MODRISK: u32 = 6151u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_EXTENSIONSECURITY: u32 = 6156u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_FILE_DOWNLOAD: u32 = 6147u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_INSTALL_DTITEMS: u32 = 6144u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_MAX: u32 = 6655u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_MIN: u32 = 6144u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_MOVE_OR_COPY: u32 = 6146u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_POPUPMGR: u32 = 6153u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_PREVIEW: u32 = 6159u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_REMOTEQUERY: u32 = 6158u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: u32 = 6154u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: u32 = 6157u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_SHARE: u32 = 6160u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_SHELLEXECUTE: u32 = 6150u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_TOCTOU_RISK: u32 = 6162u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_VERB: u32 = 6148u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_SHELL_WEBVIEW_VERB: u32 = 6149u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: u32 = 9216u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_WINFX_SETUP: u32 = 9728u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLACTION_XPS_DOCUMENTS: u32 = 9217u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLMON_OPTION_URL_ENCODING: u32 = 268435460u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLMON_OPTION_USERAGENT: u32 = 268435457u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLMON_OPTION_USERAGENT_REFRESH: u32 = 268435458u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: u32 = 268435464u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: u32 = 268435472u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLOSTRM_GETNEWESTVERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLOSTRM_USECACHEDCOPY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLOSTRM_USECACHEDCOPY_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_ACTIVEX_CHECK_LIST: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_ALLOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_DISALLOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_DONTCHECKDLGBOX: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_JAVA_CUSTOM: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_JAVA_HIGH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_JAVA_LOW: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_JAVA_MEDIUM: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_JAVA_PROHIBIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_LOG_ON_ALLOW: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_LOG_ON_DISALLOW: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_MASK_PERMISSIONS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_NOTIFY_ON_ALLOW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_NOTIFY_ON_DISALLOW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLPOLICY_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_ESC_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URL_MK_LEGACY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URL_MK_NO_CANONICALIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URL_MK_UNIFORM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const UriBuilder_USE_ORIGINAL_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_DISPLAY_IDN_HOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_DISPLAY_NO_FRAGMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_DISPLAY_NO_PUNYCODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_HOST_IS_IDN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534u32;
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHENTICATEF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const AUTHENTICATEF_PROXY: AUTHENTICATEF = AUTHENTICATEF(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const AUTHENTICATEF_BASIC: AUTHENTICATEF = AUTHENTICATEF(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const AUTHENTICATEF_HTTP: AUTHENTICATEF = AUTHENTICATEF(4i32);
impl ::core::marker::Copy for AUTHENTICATEF {}
impl ::core::clone::Clone for AUTHENTICATEF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHENTICATEF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHENTICATEF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHENTICATEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATEF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_ASYNCHRONOUS: BINDF = BINDF(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_ASYNCSTORAGE: BINDF = BINDF(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_NOPROGRESSIVERENDERING: BINDF = BINDF(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_OFFLINEOPERATION: BINDF = BINDF(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_GETNEWESTVERSION: BINDF = BINDF(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_NOWRITECACHE: BINDF = BINDF(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_NEEDFILE: BINDF = BINDF(64i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_PULLDATA: BINDF = BINDF(128i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_IGNORESECURITYPROBLEM: BINDF = BINDF(256i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESYNCHRONIZE: BINDF = BINDF(512i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_HYPERLINK: BINDF = BINDF(1024i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_NO_UI: BINDF = BINDF(2048i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_SILENTOPERATION: BINDF = BINDF(4096i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_PRAGMA_NO_CACHE: BINDF = BINDF(8192i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_GETCLASSOBJECT: BINDF = BINDF(16384i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_1: BINDF = BINDF(32768i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_FREE_THREADED: BINDF = BINDF(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_DIRECT_READ: BINDF = BINDF(131072i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_FORMS_SUBMIT: BINDF = BINDF(262144i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_GETFROMCACHE_IF_NET_FAIL: BINDF = BINDF(524288i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_FROMURLMON: BINDF = BINDF(1048576i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_FWD_BACK: BINDF = BINDF(2097152i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_PREFERDEFAULTHANDLER: BINDF = BINDF(4194304i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_ENFORCERESTRICTED: BINDF = BINDF(8388608i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_2: BINDF = BINDF(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_3: BINDF = BINDF(16777216i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_4: BINDF = BINDF(33554432i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_5: BINDF = BINDF(67108864i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_6: BINDF = BINDF(134217728i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_7: BINDF = BINDF(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF_RESERVED_8: BINDF = BINDF(536870912i32);
impl ::core::marker::Copy for BINDF {}
impl ::core::clone::Clone for BINDF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDF2(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_DISABLEBASICOVERHTTP: BINDF2 = BINDF2(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_DISABLEAUTOCOOKIEHANDLING: BINDF2 = BINDF2(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_READ_DATA_GREATER_THAN_4GB: BINDF2 = BINDF2(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID: BINDF2 = BINDF2(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_SETDOWNLOADMODE: BINDF2 = BINDF2(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_DISABLE_HTTP_REDIRECT_CACHING: BINDF2 = BINDF2(64i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_KEEP_CALLBACK_MODULE_LOADED: BINDF2 = BINDF2(128i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_ALLOW_PROXY_CRED_PROMPT: BINDF2 = BINDF2(256i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_17: BINDF2 = BINDF2(512i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_16: BINDF2 = BINDF2(1024i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_15: BINDF2 = BINDF2(2048i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_14: BINDF2 = BINDF2(4096i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_13: BINDF2 = BINDF2(8192i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_12: BINDF2 = BINDF2(16384i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_11: BINDF2 = BINDF2(32768i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_10: BINDF2 = BINDF2(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_F: BINDF2 = BINDF2(131072i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_E: BINDF2 = BINDF2(262144i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_D: BINDF2 = BINDF2(524288i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_C: BINDF2 = BINDF2(1048576i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_B: BINDF2 = BINDF2(2097152i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_A: BINDF2 = BINDF2(4194304i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_9: BINDF2 = BINDF2(8388608i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_8: BINDF2 = BINDF2(16777216i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_7: BINDF2 = BINDF2(33554432i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_6: BINDF2 = BINDF2(67108864i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_5: BINDF2 = BINDF2(134217728i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_4: BINDF2 = BINDF2(268435456i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_3: BINDF2 = BINDF2(536870912i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_2: BINDF2 = BINDF2(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDF2_RESERVED_1: BINDF2 = BINDF2(-2147483648i32);
impl ::core::marker::Copy for BINDF2 {}
impl ::core::clone::Clone for BINDF2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDF2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDF2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDF2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDF2").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDHANDLETYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDHANDLETYPES_APPCACHE: BINDHANDLETYPES = BINDHANDLETYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDHANDLETYPES_DEPENDENCY: BINDHANDLETYPES = BINDHANDLETYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDHANDLETYPES_COUNT: BINDHANDLETYPES = BINDHANDLETYPES(2i32);
impl ::core::marker::Copy for BINDHANDLETYPES {}
impl ::core::clone::Clone for BINDHANDLETYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDHANDLETYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDHANDLETYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDHANDLETYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDHANDLETYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDINFO_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_WININETFLAG: BINDINFO_OPTIONS = BINDINFO_OPTIONS(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_ENABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(131072i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_DISABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(262144i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_USE_IE_ENCODING: BINDINFO_OPTIONS = BINDINFO_OPTIONS(524288i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_BINDTOOBJECT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1048576i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_SECURITYOPTOUT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(2097152i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN: BINDINFO_OPTIONS = BINDINFO_OPTIONS(4194304i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_USEBINDSTRINGCREDS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(8388608i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(16777216i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(33554432i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_WPC_DOWNLOADBLOCKED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(134217728i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_WPC_LOGGING_ENABLED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(268435456i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_ALLOWCONNECTDATA: BINDINFO_OPTIONS = BINDINFO_OPTIONS(536870912i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDINFO_OPTIONS_SHDOCVW_NAVIGATE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(-2147483648i32);
impl ::core::marker::Copy for BINDINFO_OPTIONS {}
impl ::core::clone::Clone for BINDINFO_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDINFO_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDINFO_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDINFO_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDINFO_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_FINDINGRESOURCE: BINDSTATUS = BINDSTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CONNECTING: BINDSTATUS = BINDSTATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_REDIRECTING: BINDSTATUS = BINDSTATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_BEGINDOWNLOADDATA: BINDSTATUS = BINDSTATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_DOWNLOADINGDATA: BINDSTATUS = BINDSTATUS(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_ENDDOWNLOADDATA: BINDSTATUS = BINDSTATUS(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_BEGINDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(7i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_INSTALLINGCOMPONENTS: BINDSTATUS = BINDSTATUS(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_ENDDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(9i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_USINGCACHEDCOPY: BINDSTATUS = BINDSTATUS(10i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_SENDINGREQUEST: BINDSTATUS = BINDSTATUS(11i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(12i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(13i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CACHEFILENAMEAVAILABLE: BINDSTATUS = BINDSTATUS(14i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_BEGINSYNCOPERATION: BINDSTATUS = BINDSTATUS(15i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_ENDSYNCOPERATION: BINDSTATUS = BINDSTATUS(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_BEGINUPLOADDATA: BINDSTATUS = BINDSTATUS(17i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_UPLOADINGDATA: BINDSTATUS = BINDSTATUS(18i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_ENDUPLOADDATA: BINDSTATUS = BINDSTATUS(19i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_PROTOCOLCLASSID: BINDSTATUS = BINDSTATUS(20i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_ENCODING: BINDSTATUS = BINDSTATUS(21i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(22i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CLASSINSTALLLOCATION: BINDSTATUS = BINDSTATUS(23i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_DECODING: BINDSTATUS = BINDSTATUS(24i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_LOADINGMIMEHANDLER: BINDSTATUS = BINDSTATUS(25i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CONTENTDISPOSITIONATTACH: BINDSTATUS = BINDSTATUS(26i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_FILTERREPORTMIMETYPE: BINDSTATUS = BINDSTATUS(27i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CLSIDCANINSTANTIATE: BINDSTATUS = BINDSTATUS(28i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_IUNKNOWNAVAILABLE: BINDSTATUS = BINDSTATUS(29i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_DIRECTBIND: BINDSTATUS = BINDSTATUS(30i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RAWMIMETYPE: BINDSTATUS = BINDSTATUS(31i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_PROXYDETECTING: BINDSTATUS = BINDSTATUS(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_ACCEPTRANGES: BINDSTATUS = BINDSTATUS(33i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_SENT: BINDSTATUS = BINDSTATUS(34i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COMPACT_POLICY_RECEIVED: BINDSTATUS = BINDSTATUS(35i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_SUPPRESSED: BINDSTATUS = BINDSTATUS(36i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_STATE_UNKNOWN: BINDSTATUS = BINDSTATUS(37i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_STATE_ACCEPT: BINDSTATUS = BINDSTATUS(38i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_STATE_REJECT: BINDSTATUS = BINDSTATUS(39i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_STATE_PROMPT: BINDSTATUS = BINDSTATUS(40i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_STATE_LEASH: BINDSTATUS = BINDSTATUS(41i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_COOKIE_STATE_DOWNGRADE: BINDSTATUS = BINDSTATUS(42i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_POLICY_HREF: BINDSTATUS = BINDSTATUS(43i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_P3P_HEADER: BINDSTATUS = BINDSTATUS(44i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_SESSION_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(45i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_PERSISTENT_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(46i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_SESSION_COOKIES_ALLOWED: BINDSTATUS = BINDSTATUS(47i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CACHECONTROL: BINDSTATUS = BINDSTATUS(48i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_CONTENTDISPOSITIONFILENAME: BINDSTATUS = BINDSTATUS(49i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_MIMETEXTPLAINMISMATCH: BINDSTATUS = BINDSTATUS(50i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_PUBLISHERAVAILABLE: BINDSTATUS = BINDSTATUS(51i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_DISPLAYNAMEAVAILABLE: BINDSTATUS = BINDSTATUS(52i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_SSLUX_NAVBLOCKED: BINDSTATUS = BINDSTATUS(53i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_SERVER_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(54i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_SNIFFED_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(55i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_64BIT_PROGRESS: BINDSTATUS = BINDSTATUS(56i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_LAST: BINDSTATUS = BINDSTATUS(56i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_0: BINDSTATUS = BINDSTATUS(57i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_1: BINDSTATUS = BINDSTATUS(58i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_2: BINDSTATUS = BINDSTATUS(59i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_3: BINDSTATUS = BINDSTATUS(60i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_4: BINDSTATUS = BINDSTATUS(61i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_5: BINDSTATUS = BINDSTATUS(62i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_6: BINDSTATUS = BINDSTATUS(63i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_7: BINDSTATUS = BINDSTATUS(64i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_8: BINDSTATUS = BINDSTATUS(65i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_9: BINDSTATUS = BINDSTATUS(66i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_A: BINDSTATUS = BINDSTATUS(67i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_B: BINDSTATUS = BINDSTATUS(68i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_C: BINDSTATUS = BINDSTATUS(69i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_D: BINDSTATUS = BINDSTATUS(70i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_E: BINDSTATUS = BINDSTATUS(71i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_F: BINDSTATUS = BINDSTATUS(72i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_10: BINDSTATUS = BINDSTATUS(73i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_11: BINDSTATUS = BINDSTATUS(74i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_12: BINDSTATUS = BINDSTATUS(75i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_13: BINDSTATUS = BINDSTATUS(76i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_RESERVED_14: BINDSTATUS = BINDSTATUS(77i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTATUS_LAST_PRIVATE: BINDSTATUS = BINDSTATUS(77i32);
impl ::core::marker::Copy for BINDSTATUS {}
impl ::core::clone::Clone for BINDSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDSTRING(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_HEADERS: BINDSTRING = BINDSTRING(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_ACCEPT_MIMES: BINDSTRING = BINDSTRING(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_EXTRA_URL: BINDSTRING = BINDSTRING(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_LANGUAGE: BINDSTRING = BINDSTRING(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_USERNAME: BINDSTRING = BINDSTRING(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_PASSWORD: BINDSTRING = BINDSTRING(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_UA_PIXELS: BINDSTRING = BINDSTRING(7i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_UA_COLOR: BINDSTRING = BINDSTRING(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_OS: BINDSTRING = BINDSTRING(9i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_USER_AGENT: BINDSTRING = BINDSTRING(10i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_ACCEPT_ENCODINGS: BINDSTRING = BINDSTRING(11i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_POST_COOKIE: BINDSTRING = BINDSTRING(12i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_POST_DATA_MIME: BINDSTRING = BINDSTRING(13i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_URL: BINDSTRING = BINDSTRING(14i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_IID: BINDSTRING = BINDSTRING(15i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_FLAG_BIND_TO_OBJECT: BINDSTRING = BINDSTRING(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_PTR_BIND_CONTEXT: BINDSTRING = BINDSTRING(17i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_XDR_ORIGIN: BINDSTRING = BINDSTRING(18i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_DOWNLOADPATH: BINDSTRING = BINDSTRING(19i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_ROOTDOC_URL: BINDSTRING = BINDSTRING(20i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_INITIAL_FILENAME: BINDSTRING = BINDSTRING(21i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_PROXY_USERNAME: BINDSTRING = BINDSTRING(22i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_PROXY_PASSWORD: BINDSTRING = BINDSTRING(23i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_ENTERPRISE_ID: BINDSTRING = BINDSTRING(24i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_DOC_URL: BINDSTRING = BINDSTRING(25i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDSTRING_SAMESITE_COOKIE_LEVEL: BINDSTRING = BINDSTRING(26i32);
impl ::core::marker::Copy for BINDSTRING {}
impl ::core::clone::Clone for BINDSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDSTRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDSTRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDSTRING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDVERB(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDVERB_GET: BINDVERB = BINDVERB(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDVERB_POST: BINDVERB = BINDVERB(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDVERB_PUT: BINDVERB = BINDVERB(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDVERB_CUSTOM: BINDVERB = BINDVERB(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BINDVERB_RESERVED1: BINDVERB = BINDVERB(4i32);
impl ::core::marker::Copy for BINDVERB {}
impl ::core::clone::Clone for BINDVERB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDVERB {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BINDVERB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BINDVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDVERB").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BSCF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_FIRSTDATANOTIFICATION: BSCF = BSCF(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_INTERMEDIATEDATANOTIFICATION: BSCF = BSCF(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_LASTDATANOTIFICATION: BSCF = BSCF(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_DATAFULLYAVAILABLE: BSCF = BSCF(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_AVAILABLEDATASIZEUNKNOWN: BSCF = BSCF(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_SKIPDRAINDATAFORFILEURLS: BSCF = BSCF(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const BSCF_64BITLENGTHDOWNLOAD: BSCF = BSCF(64i32);
impl ::core::marker::Copy for BSCF {}
impl ::core::clone::Clone for BSCF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BSCF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BSCF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BSCF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BSCF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CIP_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_DISK_FULL: CIP_STATUS = CIP_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_ACCESS_DENIED: CIP_STATUS = CIP_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_NEWER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_OLDER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_NAME_CONFLICT: CIP_STATUS = CIP_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_TRUST_VERIFICATION_COMPONENT_MISSING: CIP_STATUS = CIP_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_EXE_SELF_REGISTERATION_TIMEOUT: CIP_STATUS = CIP_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_UNSAFE_TO_ABORT: CIP_STATUS = CIP_STATUS(7i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_NEED_REBOOT: CIP_STATUS = CIP_STATUS(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CIP_NEED_REBOOT_UI_PERMISSION: CIP_STATUS = CIP_STATUS(9i32);
impl ::core::marker::Copy for CIP_STATUS {}
impl ::core::clone::Clone for CIP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CIP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CIP_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CIP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CIP_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IEObjectType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_EVENT: IEObjectType = IEObjectType(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_MUTEX: IEObjectType = IEObjectType(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_SEMAPHORE: IEObjectType = IEObjectType(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_SHARED_MEMORY: IEObjectType = IEObjectType(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_WAITABLE_TIMER: IEObjectType = IEObjectType(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_FILE: IEObjectType = IEObjectType(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_NAMED_PIPE: IEObjectType = IEObjectType(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const IE_EPM_OBJECT_REGISTRY: IEObjectType = IEObjectType(7i32);
impl ::core::marker::Copy for IEObjectType {}
impl ::core::clone::Clone for IEObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IEObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IEObjectType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IEObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEObjectType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INET_ZONE_MANAGER_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MAX_ZONE_PATH: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(260i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MAX_ZONE_DESCRIPTION: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(200i32);
impl ::core::marker::Copy for INET_ZONE_MANAGER_CONSTANTS {}
impl ::core::clone::Clone for INET_ZONE_MANAGER_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INET_ZONE_MANAGER_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INET_ZONE_MANAGER_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INET_ZONE_MANAGER_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INET_ZONE_MANAGER_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNETFEATURELIST(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_OBJECT_CACHING: INTERNETFEATURELIST = INTERNETFEATURELIST(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_ZONE_ELEVATION: INTERNETFEATURELIST = INTERNETFEATURELIST(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_MIME_HANDLING: INTERNETFEATURELIST = INTERNETFEATURELIST(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_MIME_SNIFFING: INTERNETFEATURELIST = INTERNETFEATURELIST(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_WINDOW_RESTRICTIONS: INTERNETFEATURELIST = INTERNETFEATURELIST(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_WEBOC_POPUPMANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_BEHAVIORS: INTERNETFEATURELIST = INTERNETFEATURELIST(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_DISABLE_MK_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(7i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_LOCALMACHINE_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_SECURITYBAND: INTERNETFEATURELIST = INTERNETFEATURELIST(9i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_RESTRICT_ACTIVEXINSTALL: INTERNETFEATURELIST = INTERNETFEATURELIST(10i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_VALIDATE_NAVIGATE_URL: INTERNETFEATURELIST = INTERNETFEATURELIST(11i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_RESTRICT_FILEDOWNLOAD: INTERNETFEATURELIST = INTERNETFEATURELIST(12i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_ADDON_MANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(13i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_PROTOCOL_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(14i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_HTTP_USERNAME_PASSWORD_DISABLE: INTERNETFEATURELIST = INTERNETFEATURELIST(15i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_SAFE_BINDTOOBJECT: INTERNETFEATURELIST = INTERNETFEATURELIST(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_UNC_SAVEDFILECHECK: INTERNETFEATURELIST = INTERNETFEATURELIST(17i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_GET_URL_DOM_FILEPATH_UNENCODED: INTERNETFEATURELIST = INTERNETFEATURELIST(18i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_TABBED_BROWSING: INTERNETFEATURELIST = INTERNETFEATURELIST(19i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_SSLUX: INTERNETFEATURELIST = INTERNETFEATURELIST(20i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_DISABLE_NAVIGATION_SOUNDS: INTERNETFEATURELIST = INTERNETFEATURELIST(21i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_DISABLE_LEGACY_COMPRESSION: INTERNETFEATURELIST = INTERNETFEATURELIST(22i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_FORCE_ADDR_AND_STATUS: INTERNETFEATURELIST = INTERNETFEATURELIST(23i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_XMLHTTP: INTERNETFEATURELIST = INTERNETFEATURELIST(24i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_DISABLE_TELNET_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(25i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_FEEDS: INTERNETFEATURELIST = INTERNETFEATURELIST(26i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_BLOCK_INPUT_PROMPTS: INTERNETFEATURELIST = INTERNETFEATURELIST(27i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const FEATURE_ENTRY_COUNT: INTERNETFEATURELIST = INTERNETFEATURELIST(28i32);
impl ::core::marker::Copy for INTERNETFEATURELIST {}
impl ::core::clone::Clone for INTERNETFEATURELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNETFEATURELIST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNETFEATURELIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNETFEATURELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNETFEATURELIST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MONIKERPROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const MIMETYPEPROP: MONIKERPROPERTY = MONIKERPROPERTY(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const USE_SRC_URL: MONIKERPROPERTY = MONIKERPROPERTY(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const CLASSIDPROP: MONIKERPROPERTY = MONIKERPROPERTY(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const TRUSTEDDOWNLOADPROP: MONIKERPROPERTY = MONIKERPROPERTY(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const POPUPLEVELPROP: MONIKERPROPERTY = MONIKERPROPERTY(4i32);
impl ::core::marker::Copy for MONIKERPROPERTY {}
impl ::core::clone::Clone for MONIKERPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MONIKERPROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MONIKERPROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MONIKERPROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONIKERPROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OIBDG_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const OIBDG_APARTMENTTHREADED: OIBDG_FLAGS = OIBDG_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const OIBDG_DATAONLY: OIBDG_FLAGS = OIBDG_FLAGS(4096i32);
impl ::core::marker::Copy for OIBDG_FLAGS {}
impl ::core::clone::Clone for OIBDG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OIBDG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OIBDG_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OIBDG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OIBDG_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARSEACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_CANONICALIZE: PARSEACTION = PARSEACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_FRIENDLY: PARSEACTION = PARSEACTION(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_SECURITY_URL: PARSEACTION = PARSEACTION(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_ROOTDOCUMENT: PARSEACTION = PARSEACTION(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_DOCUMENT: PARSEACTION = PARSEACTION(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_ANCHOR: PARSEACTION = PARSEACTION(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_ENCODE_IS_UNESCAPE: PARSEACTION = PARSEACTION(7i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_DECODE_IS_ESCAPE: PARSEACTION = PARSEACTION(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_PATH_FROM_URL: PARSEACTION = PARSEACTION(9i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_URL_FROM_PATH: PARSEACTION = PARSEACTION(10i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_MIME: PARSEACTION = PARSEACTION(11i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_SERVER: PARSEACTION = PARSEACTION(12i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_SCHEMA: PARSEACTION = PARSEACTION(13i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_SITE: PARSEACTION = PARSEACTION(14i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_DOMAIN: PARSEACTION = PARSEACTION(15i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_LOCATION: PARSEACTION = PARSEACTION(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_SECURITY_DOMAIN: PARSEACTION = PARSEACTION(17i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_ESCAPE: PARSEACTION = PARSEACTION(18i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PARSE_UNESCAPE: PARSEACTION = PARSEACTION(19i32);
impl ::core::marker::Copy for PARSEACTION {}
impl ::core::clone::Clone for PARSEACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARSEACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PARSEACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PARSEACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARSEACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PI_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_PARSE_URL: PI_FLAGS = PI_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_FILTER_MODE: PI_FLAGS = PI_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_FORCE_ASYNC: PI_FLAGS = PI_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_USE_WORKERTHREAD: PI_FLAGS = PI_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_MIMEVERIFICATION: PI_FLAGS = PI_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_CLSIDLOOKUP: PI_FLAGS = PI_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_DATAPROGRESS: PI_FLAGS = PI_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_SYNCHRONOUS: PI_FLAGS = PI_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_APARTMENTTHREADED: PI_FLAGS = PI_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_CLASSINSTALL: PI_FLAGS = PI_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_PASSONBINDCTX: PI_FLAGS = PI_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_NOMIMEHANDLER: PI_FLAGS = PI_FLAGS(32768i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_LOADAPPDIRECT: PI_FLAGS = PI_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PD_FORCE_SWITCH: PI_FLAGS = PI_FLAGS(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PI_PREFERDEFAULTHANDLER: PI_FLAGS = PI_FLAGS(131072i32);
impl ::core::marker::Copy for PI_FLAGS {}
impl ::core::clone::Clone for PI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PI_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PI_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSUACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PSU_DEFAULT: PSUACTION = PSUACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PSU_SECURITY_URL_ONLY: PSUACTION = PSUACTION(2i32);
impl ::core::marker::Copy for PSUACTION {}
impl ::core::clone::Clone for PSUACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSUACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PSUACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PSUACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSUACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PUAF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_DEFAULT: PUAF = PUAF(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_NOUI: PUAF = PUAF(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_ISFILE: PUAF = PUAF(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_WARN_IF_DENIED: PUAF = PUAF(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_FORCEUI_FOREGROUND: PUAF = PUAF(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_CHECK_TIFS: PUAF = PUAF(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_DONTCHECKBOXINDIALOG: PUAF = PUAF(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_TRUSTED: PUAF = PUAF(64i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_ACCEPT_WILDCARD_SCHEME: PUAF = PUAF(128i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_ENFORCERESTRICTED: PUAF = PUAF(256i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_NOSAVEDFILECHECK: PUAF = PUAF(512i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_REQUIRESAVEDFILECHECK: PUAF = PUAF(1024i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_DONT_USE_CACHE: PUAF = PUAF(4096i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_RESERVED1: PUAF = PUAF(8192i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_RESERVED2: PUAF = PUAF(16384i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_LMZ_UNLOCKED: PUAF = PUAF(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_LMZ_LOCKED: PUAF = PUAF(131072i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_DEFAULTZONEPOL: PUAF = PUAF(262144i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_NPL_USE_LOCKED_IF_RESTRICTED: PUAF = PUAF(524288i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_NOUIIFLOCKED: PUAF = PUAF(1048576i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAF_DRAGPROTOCOLCHECK: PUAF = PUAF(2097152i32);
impl ::core::marker::Copy for PUAF {}
impl ::core::clone::Clone for PUAF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PUAF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PUAF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PUAF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUAF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PUAFOUT(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAFOUT_DEFAULT: PUAFOUT = PUAFOUT(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const PUAFOUT_ISLOCKZONEPOLICY: PUAFOUT = PUAFOUT(1i32);
impl ::core::marker::Copy for PUAFOUT {}
impl ::core::clone::Clone for PUAFOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PUAFOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PUAFOUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PUAFOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUAFOUT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUERYOPTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_EXPIRATION_DATE: QUERYOPTION = QUERYOPTION(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_TIME_OF_LAST_CHANGE: QUERYOPTION = QUERYOPTION(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_CONTENT_ENCODING: QUERYOPTION = QUERYOPTION(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_CONTENT_TYPE: QUERYOPTION = QUERYOPTION(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_REFRESH: QUERYOPTION = QUERYOPTION(5i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_RECOMBINE: QUERYOPTION = QUERYOPTION(6i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_CAN_NAVIGATE: QUERYOPTION = QUERYOPTION(7i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_USES_NETWORK: QUERYOPTION = QUERYOPTION(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_IS_CACHED: QUERYOPTION = QUERYOPTION(9i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_IS_INSTALLEDENTRY: QUERYOPTION = QUERYOPTION(10i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_IS_CACHED_OR_MAPPED: QUERYOPTION = QUERYOPTION(11i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_USES_CACHE: QUERYOPTION = QUERYOPTION(12i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_IS_SECURE: QUERYOPTION = QUERYOPTION(13i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_IS_SAFE: QUERYOPTION = QUERYOPTION(14i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_USES_HISTORYFOLDER: QUERYOPTION = QUERYOPTION(15i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const QUERY_IS_CACHED_AND_USABLE_OFFLINE: QUERYOPTION = QUERYOPTION(16i32);
impl ::core::marker::Copy for QUERYOPTION {}
impl ::core::clone::Clone for QUERYOPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUERYOPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for QUERYOPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for QUERYOPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERYOPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SZM_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SZM_CREATE: SZM_FLAGS = SZM_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const SZM_DELETE: SZM_FLAGS = SZM_FLAGS(1i32);
impl ::core::marker::Copy for SZM_FLAGS {}
impl ::core::clone::Clone for SZM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SZM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SZM_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SZM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SZM_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct URLTEMPLATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_CUSTOM: URLTEMPLATE = URLTEMPLATE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_PREDEFINED_MIN: URLTEMPLATE = URLTEMPLATE(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_LOW: URLTEMPLATE = URLTEMPLATE(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_MEDLOW: URLTEMPLATE = URLTEMPLATE(66816i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_MEDIUM: URLTEMPLATE = URLTEMPLATE(69632i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_MEDHIGH: URLTEMPLATE = URLTEMPLATE(70912i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_HIGH: URLTEMPLATE = URLTEMPLATE(73728i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLTEMPLATE_PREDEFINED_MAX: URLTEMPLATE = URLTEMPLATE(131072i32);
impl ::core::marker::Copy for URLTEMPLATE {}
impl ::core::clone::Clone for URLTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URLTEMPLATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for URLTEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for URLTEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLTEMPLATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct URLZONE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_INVALID: URLZONE = URLZONE(-1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_PREDEFINED_MIN: URLZONE = URLZONE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_LOCAL_MACHINE: URLZONE = URLZONE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_INTRANET: URLZONE = URLZONE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_TRUSTED: URLZONE = URLZONE(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_INTERNET: URLZONE = URLZONE(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_UNTRUSTED: URLZONE = URLZONE(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_PREDEFINED_MAX: URLZONE = URLZONE(999i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_USER_MIN: URLZONE = URLZONE(1000i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONE_USER_MAX: URLZONE = URLZONE(10000i32);
impl ::core::marker::Copy for URLZONE {}
impl ::core::clone::Clone for URLZONE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URLZONE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for URLZONE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for URLZONE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLZONE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct URLZONEREG(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONEREG_DEFAULT: URLZONEREG = URLZONEREG(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONEREG_HKLM: URLZONEREG = URLZONEREG(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URLZONEREG_HKCU: URLZONEREG = URLZONEREG(2i32);
impl ::core::marker::Copy for URLZONEREG {}
impl ::core::clone::Clone for URLZONEREG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URLZONEREG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for URLZONEREG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for URLZONEREG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLZONEREG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct URL_ENCODING(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URL_ENCODING_NONE: URL_ENCODING = URL_ENCODING(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URL_ENCODING_ENABLE_UTF8: URL_ENCODING = URL_ENCODING(268435456i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const URL_ENCODING_DISABLE_UTF8: URL_ENCODING = URL_ENCODING(536870912i32);
impl ::core::marker::Copy for URL_ENCODING {}
impl ::core::clone::Clone for URL_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URL_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for URL_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for URL_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URL_ENCODING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Uri_HOST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_HOST_UNKNOWN: Uri_HOST_TYPE = Uri_HOST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_HOST_DNS: Uri_HOST_TYPE = Uri_HOST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_HOST_IPV4: Uri_HOST_TYPE = Uri_HOST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_HOST_IPV6: Uri_HOST_TYPE = Uri_HOST_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const Uri_HOST_IDN: Uri_HOST_TYPE = Uri_HOST_TYPE(4i32);
impl ::core::marker::Copy for Uri_HOST_TYPE {}
impl ::core::clone::Clone for Uri_HOST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Uri_HOST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Uri_HOST_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Uri_HOST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri_HOST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ZAFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_CUSTOM_EDIT: ZAFLAGS = ZAFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_ADD_SITES: ZAFLAGS = ZAFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_REQUIRE_VERIFICATION: ZAFLAGS = ZAFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_INCLUDE_PROXY_OVERRIDE: ZAFLAGS = ZAFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_INCLUDE_INTRANET_SITES: ZAFLAGS = ZAFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_NO_UI: ZAFLAGS = ZAFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_SUPPORTS_VERIFICATION: ZAFLAGS = ZAFLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_UNC_AS_INTRANET: ZAFLAGS = ZAFLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_DETECT_INTRANET: ZAFLAGS = ZAFLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_USE_LOCKED_ZONES: ZAFLAGS = ZAFLAGS(65536i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_VERIFY_TEMPLATE_SETTINGS: ZAFLAGS = ZAFLAGS(131072i32);
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub const ZAFLAGS_NO_CACHE: ZAFLAGS = ZAFLAGS(262144i32);
impl ::core::marker::Copy for ZAFLAGS {}
impl ::core::clone::Clone for ZAFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ZAFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ZAFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ZAFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZAFLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub struct CODEBASEHOLD {
    pub cbSize: u32,
    pub szDistUnit: ::windows::core::PWSTR,
    pub szCodeBase: ::windows::core::PWSTR,
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
    pub dwStyle: u32,
}
impl ::core::marker::Copy for CODEBASEHOLD {}
impl ::core::clone::Clone for CODEBASEHOLD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CODEBASEHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CODEBASEHOLD").field("cbSize", &self.cbSize).field("szDistUnit", &self.szDistUnit).field("szCodeBase", &self.szCodeBase).field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).field("dwStyle", &self.dwStyle).finish()
    }
}
impl ::windows::core::TypeKind for CODEBASEHOLD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CODEBASEHOLD {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szDistUnit == other.szDistUnit && self.szCodeBase == other.szCodeBase && self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS && self.dwStyle == other.dwStyle
    }
}
impl ::core::cmp::Eq for CODEBASEHOLD {}
impl ::core::default::Default for CODEBASEHOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub struct CONFIRMSAFETY {
    pub clsid: ::windows::core::GUID,
    pub pUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub dwFlags: u32,
}
impl ::core::clone::Clone for CONFIRMSAFETY {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for CONFIRMSAFETY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIRMSAFETY").field("clsid", &self.clsid).field("pUnk", &self.pUnk).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for CONFIRMSAFETY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONFIRMSAFETY {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.pUnk == other.pUnk && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CONFIRMSAFETY {}
impl ::core::default::Default for CONFIRMSAFETY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
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
impl ::core::fmt::Debug for DATAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATAINFO").field("ulTotalSize", &self.ulTotalSize).field("ulavrPacketSize", &self.ulavrPacketSize).field("ulConnectSpeed", &self.ulConnectSpeed).field("ulProcessorSpeed", &self.ulProcessorSpeed).finish()
    }
}
impl ::windows::core::TypeKind for DATAINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulTotalSize == other.ulTotalSize && self.ulavrPacketSize == other.ulavrPacketSize && self.ulConnectSpeed == other.ulConnectSpeed && self.ulProcessorSpeed == other.ulProcessorSpeed
    }
}
impl ::core::cmp::Eq for DATAINFO {}
impl ::core::default::Default for DATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIT_LOGGING_INFO {
    pub dwStructSize: u32,
    pub lpszLoggedUrlName: ::windows::core::PSTR,
    pub StartTime: super::super::super::Foundation::SYSTEMTIME,
    pub EndTime: super::super::super::Foundation::SYSTEMTIME,
    pub lpszExtendedInfo: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIT_LOGGING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIT_LOGGING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIT_LOGGING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIT_LOGGING_INFO").field("dwStructSize", &self.dwStructSize).field("lpszLoggedUrlName", &self.lpszLoggedUrlName).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("lpszExtendedInfo", &self.lpszExtendedInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HIT_LOGGING_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIT_LOGGING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszLoggedUrlName == other.lpszLoggedUrlName && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.lpszExtendedInfo == other.lpszExtendedInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIT_LOGGING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIT_LOGGING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
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
impl ::core::fmt::Debug for PROTOCOLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOLDATA").field("grfFlags", &self.grfFlags).field("dwState", &self.dwState).field("pData", &self.pData).field("cbData", &self.cbData).finish()
    }
}
impl ::windows::core::TypeKind for PROTOCOLDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROTOCOLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.grfFlags == other.grfFlags && self.dwState == other.dwState && self.pData == other.pData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for PROTOCOLDATA {}
impl ::core::default::Default for PROTOCOLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub struct PROTOCOLFILTERDATA {
    pub cbSize: u32,
    pub pProtocolSink: ::std::mem::ManuallyDrop<::core::option::Option<IInternetProtocolSink>>,
    pub pProtocol: ::std::mem::ManuallyDrop<::core::option::Option<IInternetProtocol>>,
    pub pUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub dwFilterFlags: u32,
}
impl ::core::clone::Clone for PROTOCOLFILTERDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PROTOCOLFILTERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOLFILTERDATA").field("cbSize", &self.cbSize).field("pProtocolSink", &self.pProtocolSink).field("pProtocol", &self.pProtocol).field("pUnk", &self.pUnk).field("dwFilterFlags", &self.dwFilterFlags).finish()
    }
}
impl ::windows::core::TypeKind for PROTOCOLFILTERDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROTOCOLFILTERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pProtocolSink == other.pProtocolSink && self.pProtocol == other.pProtocol && self.pUnk == other.pUnk && self.dwFilterFlags == other.dwFilterFlags
    }
}
impl ::core::cmp::Eq for PROTOCOLFILTERDATA {}
impl ::core::default::Default for PROTOCOLFILTERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub struct PROTOCOL_ARGUMENT {
    pub szMethod: ::windows::core::PCWSTR,
    pub szTargetUrl: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for PROTOCOL_ARGUMENT {}
impl ::core::clone::Clone for PROTOCOL_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROTOCOL_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOL_ARGUMENT").field("szMethod", &self.szMethod).field("szTargetUrl", &self.szTargetUrl).finish()
    }
}
impl ::windows::core::TypeKind for PROTOCOL_ARGUMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROTOCOL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.szMethod == other.szMethod && self.szTargetUrl == other.szTargetUrl
    }
}
impl ::core::cmp::Eq for PROTOCOL_ARGUMENT {}
impl ::core::default::Default for PROTOCOL_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REMSECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMSECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for REMSECURITY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REMSECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength && self.lpSecurityDescriptor == other.lpSecurityDescriptor && self.bInheritHandle == other.bInheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REMSECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REMSECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RemBINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: ::windows::core::PWSTR,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: ::windows::core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: REMSECURITY_ATTRIBUTES,
    pub iid: ::windows::core::GUID,
    pub pUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RemBINDINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RemBINDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemBINDINFO")
            .field("cbSize", &self.cbSize)
            .field("szExtraInfo", &self.szExtraInfo)
            .field("grfBindInfoF", &self.grfBindInfoF)
            .field("dwBindVerb", &self.dwBindVerb)
            .field("szCustomVerb", &self.szCustomVerb)
            .field("cbstgmedData", &self.cbstgmedData)
            .field("dwOptions", &self.dwOptions)
            .field("dwOptionsFlags", &self.dwOptionsFlags)
            .field("dwCodePage", &self.dwCodePage)
            .field("securityAttributes", &self.securityAttributes)
            .field("iid", &self.iid)
            .field("pUnk", &self.pUnk)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RemBINDINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RemBINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szExtraInfo == other.szExtraInfo && self.grfBindInfoF == other.grfBindInfoF && self.dwBindVerb == other.dwBindVerb && self.szCustomVerb == other.szCustomVerb && self.cbstgmedData == other.cbstgmedData && self.dwOptions == other.dwOptions && self.dwOptionsFlags == other.dwOptionsFlags && self.dwCodePage == other.dwCodePage && self.securityAttributes == other.securityAttributes && self.iid == other.iid && self.pUnk == other.pUnk && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RemBINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RemBINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
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
impl ::core::fmt::Debug for RemFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemFORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
impl ::windows::core::TypeKind for RemFORMATETC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RemFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat && self.ptd == other.ptd && self.dwAspect == other.dwAspect && self.lindex == other.lindex && self.tymed == other.tymed
    }
}
impl ::core::cmp::Eq for RemFORMATETC {}
impl ::core::default::Default for RemFORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub struct SOFTDISTINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwAdState: u32,
    pub szTitle: ::windows::core::PWSTR,
    pub szAbstract: ::windows::core::PWSTR,
    pub szHREF: ::windows::core::PWSTR,
    pub dwInstalledVersionMS: u32,
    pub dwInstalledVersionLS: u32,
    pub dwUpdateVersionMS: u32,
    pub dwUpdateVersionLS: u32,
    pub dwAdvertisedVersionMS: u32,
    pub dwAdvertisedVersionLS: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for SOFTDISTINFO {}
impl ::core::clone::Clone for SOFTDISTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOFTDISTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOFTDISTINFO")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwAdState", &self.dwAdState)
            .field("szTitle", &self.szTitle)
            .field("szAbstract", &self.szAbstract)
            .field("szHREF", &self.szHREF)
            .field("dwInstalledVersionMS", &self.dwInstalledVersionMS)
            .field("dwInstalledVersionLS", &self.dwInstalledVersionLS)
            .field("dwUpdateVersionMS", &self.dwUpdateVersionMS)
            .field("dwUpdateVersionLS", &self.dwUpdateVersionLS)
            .field("dwAdvertisedVersionMS", &self.dwAdvertisedVersionMS)
            .field("dwAdvertisedVersionLS", &self.dwAdvertisedVersionLS)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::windows::core::TypeKind for SOFTDISTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SOFTDISTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.dwAdState == other.dwAdState && self.szTitle == other.szTitle && self.szAbstract == other.szAbstract && self.szHREF == other.szHREF && self.dwInstalledVersionMS == other.dwInstalledVersionMS && self.dwInstalledVersionLS == other.dwInstalledVersionLS && self.dwUpdateVersionMS == other.dwUpdateVersionMS && self.dwUpdateVersionLS == other.dwUpdateVersionLS && self.dwAdvertisedVersionMS == other.dwAdvertisedVersionMS && self.dwAdvertisedVersionLS == other.dwAdvertisedVersionLS && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for SOFTDISTINFO {}
impl ::core::default::Default for SOFTDISTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
pub struct StartParam {
    pub iid: ::windows::core::GUID,
    pub pIBindCtx: ::std::mem::ManuallyDrop<::core::option::Option<super::IBindCtx>>,
    pub pItf: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for StartParam {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for StartParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StartParam").field("iid", &self.iid).field("pIBindCtx", &self.pIBindCtx).field("pItf", &self.pItf).finish()
    }
}
impl ::windows::core::TypeKind for StartParam {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for StartParam {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid && self.pIBindCtx == other.pIBindCtx && self.pItf == other.pItf
    }
}
impl ::core::cmp::Eq for StartParam {}
impl ::core::default::Default for StartParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
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
impl ::core::fmt::Debug for ZONEATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ZONEATTRIBUTES").field("cbSize", &self.cbSize).field("szDisplayName", &self.szDisplayName).field("szDescription", &self.szDescription).field("szIconPath", &self.szIconPath).field("dwTemplateMinLevel", &self.dwTemplateMinLevel).field("dwTemplateRecommended", &self.dwTemplateRecommended).field("dwTemplateCurrentLevel", &self.dwTemplateCurrentLevel).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for ZONEATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ZONEATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szDisplayName == other.szDisplayName && self.szDescription == other.szDescription && self.szIconPath == other.szIconPath && self.dwTemplateMinLevel == other.dwTemplateMinLevel && self.dwTemplateRecommended == other.dwTemplateRecommended && self.dwTemplateCurrentLevel == other.dwTemplateCurrentLevel && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for ZONEATTRIBUTES {}
impl ::core::default::Default for ZONEATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
