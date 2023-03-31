#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ComputeInvCMAP(prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD, ncolors: u32, pinvtable: *mut u8, cbtable: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "imgutil.dll""system" fn ComputeInvCMAP ( prgbcolors : *const super::super::Graphics::Gdi:: RGBQUAD , ncolors : u32 , pinvtable : *mut u8 , cbtable : u32 ) -> ::windows::core::HRESULT );
    ComputeInvCMAP(prgbcolors, ncolors, pinvtable, cbtable).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateDDrawSurfaceOnDIB<P0>(hbmdib: P0) -> ::windows::core::Result<super::super::Graphics::DirectDraw::IDirectDrawSurface>
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HBITMAP>,
{
    ::windows_targets::link ! ( "imgutil.dll""system" fn CreateDDrawSurfaceOnDIB ( hbmdib : super::super::Graphics::Gdi:: HBITMAP , ppsurface : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Graphics::DirectDraw::IDirectDrawSurface>();
    CreateDDrawSurfaceOnDIB(hbmdib.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn CreateMIMEMap() -> ::windows::core::Result<IMapMIMEToCLSID> {
    ::windows_targets::link ! ( "imgutil.dll""system" fn CreateMIMEMap ( ppmap : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IMapMIMEToCLSID>();
    CreateMIMEMap(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DecodeImage<P0, P1, P2>(pstream: P0, pmap: P1, peventsink: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    P1: ::windows::core::IntoParam<IMapMIMEToCLSID>,
    P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "imgutil.dll""system" fn DecodeImage ( pstream : * mut::core::ffi::c_void , pmap : * mut::core::ffi::c_void , peventsink : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DecodeImage(pstream.into_param().abi(), pmap.into_param().abi(), peventsink.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DecodeImageEx<P0, P1, P2, P3>(pstream: P0, pmap: P1, peventsink: P2, pszmimetypeparam: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    P1: ::windows::core::IntoParam<IMapMIMEToCLSID>,
    P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "imgutil.dll""system" fn DecodeImageEx ( pstream : * mut::core::ffi::c_void , pmap : * mut::core::ffi::c_void , peventsink : * mut::core::ffi::c_void , pszmimetypeparam : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    DecodeImageEx(pstream.into_param().abi(), pmap.into_param().abi(), peventsink.into_param().abi(), pszmimetypeparam.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DitherTo8(pdestbits: *mut u8, ndestpitch: i32, psrcbits: *mut u8, nsrcpitch: i32, bfidsrc: *const ::windows::core::GUID, prgbdestcolors: *mut super::super::Graphics::Gdi::RGBQUAD, prgbsrccolors: *mut super::super::Graphics::Gdi::RGBQUAD, pbdestinvmap: *mut u8, x: i32, y: i32, cx: i32, cy: i32, ldesttrans: i32, lsrctrans: i32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "imgutil.dll""system" fn DitherTo8 ( pdestbits : *mut u8 , ndestpitch : i32 , psrcbits : *mut u8 , nsrcpitch : i32 , bfidsrc : *const ::windows::core::GUID , prgbdestcolors : *mut super::super::Graphics::Gdi:: RGBQUAD , prgbsrccolors : *mut super::super::Graphics::Gdi:: RGBQUAD , pbdestinvmap : *mut u8 , x : i32 , y : i32 , cx : i32 , cy : i32 , ldesttrans : i32 , lsrctrans : i32 ) -> ::windows::core::HRESULT );
    DitherTo8(pdestbits, ndestpitch, psrcbits, nsrcpitch, bfidsrc, prgbdestcolors, prgbsrccolors, pbdestinvmap, x, y, cx, cy, ldesttrans, lsrctrans).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn GetMaxMIMEIDBytes(pnmaxbytes: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "imgutil.dll""system" fn GetMaxMIMEIDBytes ( pnmaxbytes : *mut u32 ) -> ::windows::core::HRESULT );
    GetMaxMIMEIDBytes(pnmaxbytes).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IEAssociateThreadWithTab(dwtabthreadid: u32, dwassociatedthreadid: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEAssociateThreadWithTab ( dwtabthreadid : u32 , dwassociatedthreadid : u32 ) -> ::windows::core::HRESULT );
    IEAssociateThreadWithTab(dwtabthreadid, dwassociatedthreadid).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IECancelSaveFile<P0>(hstate: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IECancelSaveFile ( hstate : super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    IECancelSaveFile(hstate.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IECreateDirectory<P0>(lppathname: P0, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IECreateDirectory ( lppathname : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    IECreateDirectory(lppathname.into_param().abi(), lpsecurityattributes)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IECreateFile<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IECreateFile ( lpfilename : ::windows::core::PCWSTR , dwdesiredaccess : u32 , dwsharemode : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwcreationdisposition : u32 , dwflagsandattributes : u32 , htemplatefile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: HANDLE );
    IECreateFile(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes, dwcreationdisposition, dwflagsandattributes, htemplatefile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEDeleteFile<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEDeleteFile ( lpfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    IEDeleteFile(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IEDisassociateThreadWithTab(dwtabthreadid: u32, dwassociatedthreadid: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEDisassociateThreadWithTab ( dwtabthreadid : u32 , dwassociatedthreadid : u32 ) -> ::windows::core::HRESULT );
    IEDisassociateThreadWithTab(dwtabthreadid, dwassociatedthreadid).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn IEFindFirstFile<P0>(lpfilename: P0, lpfindfiledata: *const super::super::Storage::FileSystem::WIN32_FIND_DATAA) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEFindFirstFile ( lpfilename : ::windows::core::PCWSTR , lpfindfiledata : *const super::super::Storage::FileSystem:: WIN32_FIND_DATAA ) -> super::super::Foundation:: HANDLE );
    IEFindFirstFile(lpfilename.into_param().abi(), lpfindfiledata)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn IEGetFileAttributesEx<P0>(lpfilename: P0, finfolevelid: super::super::Storage::FileSystem::GET_FILEEX_INFO_LEVELS, lpfileinformation: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEGetFileAttributesEx ( lpfilename : ::windows::core::PCWSTR , finfolevelid : super::super::Storage::FileSystem:: GET_FILEEX_INFO_LEVELS , lpfileinformation : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    IEGetFileAttributesEx(lpfilename.into_param().abi(), finfolevelid, lpfileinformation)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IEGetProtectedModeCookie<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: ::windows::core::PWSTR, pcchcookiedata: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEGetProtectedModeCookie ( lpszurl : ::windows::core::PCWSTR , lpszcookiename : ::windows::core::PCWSTR , lpszcookiedata : ::windows::core::PWSTR , pcchcookiedata : *mut u32 , dwflags : u32 ) -> ::windows::core::HRESULT );
    IEGetProtectedModeCookie(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), ::core::mem::transmute(lpszcookiedata), pcchcookiedata, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IEGetWriteableFolderPath(clsidfolderid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEGetWriteableFolderPath ( clsidfolderid : *const ::windows::core::GUID , lppwstrpath : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    IEGetWriteableFolderPath(clsidfolderid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn IEGetWriteableLowHKCU() -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEGetWriteableLowHKCU ( phkey : *mut super::super::System::Registry:: HKEY ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::System::Registry::HKEY>();
    IEGetWriteableLowHKCU(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEInPrivateFilteringEnabled() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEInPrivateFilteringEnabled ( ) -> super::super::Foundation:: BOOL );
    IEInPrivateFilteringEnabled()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEIsInPrivateBrowsing() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEIsInPrivateBrowsing ( ) -> super::super::Foundation:: BOOL );
    IEIsInPrivateBrowsing()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEIsProtectedModeProcess() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEIsProtectedModeProcess ( pbresult : *mut super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
    IEIsProtectedModeProcess(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IEIsProtectedModeURL<P0>(lpwstrurl: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEIsProtectedModeURL ( lpwstrurl : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    IEIsProtectedModeURL(lpwstrurl.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn IELaunchURL<P0>(lpwstrurl: P0, lpprocinfo: *mut super::super::System::Threading::PROCESS_INFORMATION, lpinfo: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IELaunchURL ( lpwstrurl : ::windows::core::PCWSTR , lpprocinfo : *mut super::super::System::Threading:: PROCESS_INFORMATION , lpinfo : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    IELaunchURL(lpwstrurl.into_param().abi(), lpprocinfo, ::core::mem::transmute(lpinfo.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEMoveFileEx<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEMoveFileEx ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    IEMoveFileEx(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IERefreshElevationPolicy() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IERefreshElevationPolicy ( ) -> ::windows::core::HRESULT );
    IERefreshElevationPolicy().ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn IERegCreateKeyEx<P0, P1>(lpsubkey: P0, reserved: u32, lpclass: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IERegCreateKeyEx ( lpsubkey : ::windows::core::PCWSTR , reserved : u32 , lpclass : ::windows::core::PCWSTR , dwoptions : u32 , samdesired : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut super::super::System::Registry:: HKEY , lpdwdisposition : *mut u32 ) -> ::windows::core::HRESULT );
    IERegCreateKeyEx(lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, lpdwdisposition).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IERegSetValueEx<P0, P1>(lpsubkey: P0, lpvaluename: P1, reserved: u32, dwtype: u32, lpdata: &[u8]) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IERegSetValueEx ( lpsubkey : ::windows::core::PCWSTR , lpvaluename : ::windows::core::PCWSTR , reserved : u32 , dwtype : u32 , lpdata : *const u8 , cbdata : u32 ) -> ::windows::core::HRESULT );
    IERegSetValueEx(lpsubkey.into_param().abi(), lpvaluename.into_param().abi(), reserved, dwtype, ::core::mem::transmute(lpdata.as_ptr()), lpdata.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IERegisterWritableRegistryKey<P0, P1>(guid: ::windows::core::GUID, lpsubkey: P0, fsubkeyallowed: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IERegisterWritableRegistryKey ( guid : ::windows::core::GUID , lpsubkey : ::windows::core::PCWSTR , fsubkeyallowed : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    IERegisterWritableRegistryKey(::core::mem::transmute(guid), lpsubkey.into_param().abi(), fsubkeyallowed.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IERegisterWritableRegistryValue<P0, P1>(guid: ::windows::core::GUID, lppath: P0, lpvaluename: P1, dwtype: u32, lpdata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IERegisterWritableRegistryValue ( guid : ::windows::core::GUID , lppath : ::windows::core::PCWSTR , lpvaluename : ::windows::core::PCWSTR , dwtype : u32 , lpdata : *const u8 , cbmaxdata : u32 ) -> ::windows::core::HRESULT );
    IERegisterWritableRegistryValue(::core::mem::transmute(guid), lppath.into_param().abi(), lpvaluename.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IERemoveDirectory<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IERemoveDirectory ( lppathname : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    IERemoveDirectory(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IESaveFile<P0, P1>(hstate: P0, lpwstrsourcefile: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IESaveFile ( hstate : super::super::Foundation:: HANDLE , lpwstrsourcefile : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    IESaveFile(hstate.into_param().abi(), lpwstrsourcefile.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IESetProtectedModeCookie<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IESetProtectedModeCookie ( lpszurl : ::windows::core::PCWSTR , lpszcookiename : ::windows::core::PCWSTR , lpszcookiedata : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    IESetProtectedModeCookie(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEShowOpenFileDialog<P0, P1, P2, P3>(hwnd: P0, lpwstrfilename: &mut [u16], lpwstrinitialdir: P1, lpwstrfilter: P2, lpwstrdefext: P3, dwfilterindex: u32, dwflags: u32, phfile: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEShowOpenFileDialog ( hwnd : super::super::Foundation:: HWND , lpwstrfilename : ::windows::core::PWSTR , cchmaxfilename : u32 , lpwstrinitialdir : ::windows::core::PCWSTR , lpwstrfilter : ::windows::core::PCWSTR , lpwstrdefext : ::windows::core::PCWSTR , dwfilterindex : u32 , dwflags : u32 , phfile : *mut super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    IEShowOpenFileDialog(hwnd.into_param().abi(), ::core::mem::transmute(lpwstrfilename.as_ptr()), lpwstrfilename.len() as _, lpwstrinitialdir.into_param().abi(), lpwstrfilter.into_param().abi(), lpwstrdefext.into_param().abi(), dwfilterindex, dwflags, phfile).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEShowSaveFileDialog<P0, P1, P2, P3, P4>(hwnd: P0, lpwstrinitialfilename: P1, lpwstrinitialdir: P2, lpwstrfilter: P3, lpwstrdefext: P4, dwfilterindex: u32, dwflags: u32, lppwstrdestinationfilepath: *mut ::windows::core::PWSTR, phstate: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEShowSaveFileDialog ( hwnd : super::super::Foundation:: HWND , lpwstrinitialfilename : ::windows::core::PCWSTR , lpwstrinitialdir : ::windows::core::PCWSTR , lpwstrfilter : ::windows::core::PCWSTR , lpwstrdefext : ::windows::core::PCWSTR , dwfilterindex : u32 , dwflags : u32 , lppwstrdestinationfilepath : *mut ::windows::core::PWSTR , phstate : *mut super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    IEShowSaveFileDialog(hwnd.into_param().abi(), lpwstrinitialfilename.into_param().abi(), lpwstrinitialdir.into_param().abi(), lpwstrfilter.into_param().abi(), lpwstrdefext.into_param().abi(), dwfilterindex, dwflags, lppwstrdestinationfilepath, phstate).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IETrackingProtectionEnabled() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IETrackingProtectionEnabled ( ) -> super::super::Foundation:: BOOL );
    IETrackingProtectionEnabled()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IEUnregisterWritableRegistry(guid: ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ieframe.dll""system" fn IEUnregisterWritableRegistry ( guid : ::windows::core::GUID ) -> ::windows::core::HRESULT );
    IEUnregisterWritableRegistry(::core::mem::transmute(guid)).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn IdentifyMIMEType(pbbytes: *const u8, nbytes: u32, pnformat: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "imgutil.dll""system" fn IdentifyMIMEType ( pbbytes : *const u8 , nbytes : u32 , pnformat : *mut u32 ) -> ::windows::core::HRESULT );
    IdentifyMIMEType(pbbytes, nbytes, pnformat).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingAccessDeniedDialog<P0, P1, P2>(hdlg: P0, pszusername: P1, pszcontentdescription: P2, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingAccessDeniedDialog ( hdlg : super::super::Foundation:: HWND , pszusername : ::windows::core::PCSTR , pszcontentdescription : ::windows::core::PCSTR , pratingdetails : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingAccessDeniedDialog(hdlg.into_param().abi(), pszusername.into_param().abi(), pszcontentdescription.into_param().abi(), pratingdetails).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingAccessDeniedDialog2<P0, P1>(hdlg: P0, pszusername: P1, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingAccessDeniedDialog2 ( hdlg : super::super::Foundation:: HWND , pszusername : ::windows::core::PCSTR , pratingdetails : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingAccessDeniedDialog2(hdlg.into_param().abi(), pszusername.into_param().abi(), pratingdetails).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingAccessDeniedDialog2W<P0, P1>(hdlg: P0, pszusername: P1, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingAccessDeniedDialog2W ( hdlg : super::super::Foundation:: HWND , pszusername : ::windows::core::PCWSTR , pratingdetails : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingAccessDeniedDialog2W(hdlg.into_param().abi(), pszusername.into_param().abi(), pratingdetails).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingAccessDeniedDialogW<P0, P1, P2>(hdlg: P0, pszusername: P1, pszcontentdescription: P2, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingAccessDeniedDialogW ( hdlg : super::super::Foundation:: HWND , pszusername : ::windows::core::PCWSTR , pszcontentdescription : ::windows::core::PCWSTR , pratingdetails : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingAccessDeniedDialogW(hdlg.into_param().abi(), pszusername.into_param().abi(), pszcontentdescription.into_param().abi(), pratingdetails).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingAddToApprovedSites<P0, P1, P2, P3, P4>(hdlg: P0, pbpasswordblob: &mut [u8], lpszurl: P1, falwaysnever: P2, fsitepage: P3, fapprovedsitesenforced: P4) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P4: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingAddToApprovedSites ( hdlg : super::super::Foundation:: HWND , cbpasswordblob : u32 , pbpasswordblob : *mut u8 , lpszurl : ::windows::core::PCWSTR , falwaysnever : super::super::Foundation:: BOOL , fsitepage : super::super::Foundation:: BOOL , fapprovedsitesenforced : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    RatingAddToApprovedSites(hdlg.into_param().abi(), pbpasswordblob.len() as _, ::core::mem::transmute(pbpasswordblob.as_ptr()), lpszurl.into_param().abi(), falwaysnever.into_param().abi(), fsitepage.into_param().abi(), fapprovedsitesenforced.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn RatingCheckUserAccess<P0, P1, P2>(pszusername: P0, pszurl: P1, pszratinginfo: P2, pdata: ::core::option::Option<&[u8]>, ppratingdetails: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingCheckUserAccess ( pszusername : ::windows::core::PCSTR , pszurl : ::windows::core::PCSTR , pszratinginfo : ::windows::core::PCSTR , pdata : *const u8 , cbdata : u32 , ppratingdetails : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingCheckUserAccess(pszusername.into_param().abi(), pszurl.into_param().abi(), pszratinginfo.into_param().abi(), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppratingdetails.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn RatingCheckUserAccessW<P0, P1, P2>(pszusername: P0, pszurl: P1, pszratinginfo: P2, pdata: ::core::option::Option<&[u8]>, ppratingdetails: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingCheckUserAccessW ( pszusername : ::windows::core::PCWSTR , pszurl : ::windows::core::PCWSTR , pszratinginfo : ::windows::core::PCWSTR , pdata : *const u8 , cbdata : u32 , ppratingdetails : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingCheckUserAccessW(pszusername.into_param().abi(), pszurl.into_param().abi(), pszratinginfo.into_param().abi(), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppratingdetails.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingClickedOnPRFInternal<P0, P1, P2>(hwndowner: P0, param1: P1, lpszfilename: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingClickedOnPRFInternal ( hwndowner : super::super::Foundation:: HWND , param1 : super::super::Foundation:: HMODULE , lpszfilename : ::windows::core::PCSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    RatingClickedOnPRFInternal(hwndowner.into_param().abi(), param1.into_param().abi(), lpszfilename.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingClickedOnRATInternal<P0, P1, P2>(hwndowner: P0, param1: P1, lpszfilename: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingClickedOnRATInternal ( hwndowner : super::super::Foundation:: HWND , param1 : super::super::Foundation:: HMODULE , lpszfilename : ::windows::core::PCSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    RatingClickedOnRATInternal(hwndowner.into_param().abi(), param1.into_param().abi(), lpszfilename.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingEnable<P0, P1, P2>(hwndparent: P0, pszusername: P1, fenable: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingEnable ( hwndparent : super::super::Foundation:: HWND , pszusername : ::windows::core::PCSTR , fenable : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    RatingEnable(hwndparent.into_param().abi(), pszusername.into_param().abi(), fenable.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingEnableW<P0, P1, P2>(hwndparent: P0, pszusername: P1, fenable: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingEnableW ( hwndparent : super::super::Foundation:: HWND , pszusername : ::windows::core::PCWSTR , fenable : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    RatingEnableW(hwndparent.into_param().abi(), pszusername.into_param().abi(), fenable.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn RatingEnabledQuery() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingEnabledQuery ( ) -> ::windows::core::HRESULT );
    RatingEnabledQuery().ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn RatingFreeDetails(pratingdetails: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingFreeDetails ( pratingdetails : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RatingFreeDetails(::core::mem::transmute(pratingdetails.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[inline]
pub unsafe fn RatingInit() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingInit ( ) -> ::windows::core::HRESULT );
    RatingInit().ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingObtainCancel<P0>(hratingobtainquery: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingObtainCancel ( hratingobtainquery : super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    RatingObtainCancel(hratingobtainquery.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingObtainQuery<P0>(psztargeturl: P0, dwuserdata: u32, fcallback: isize, phratingobtainquery: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingObtainQuery ( psztargeturl : ::windows::core::PCSTR , dwuserdata : u32 , fcallback : isize , phratingobtainquery : *mut super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    RatingObtainQuery(psztargeturl.into_param().abi(), dwuserdata, fcallback, ::core::mem::transmute(phratingobtainquery.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingObtainQueryW<P0>(psztargeturl: P0, dwuserdata: u32, fcallback: isize, phratingobtainquery: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingObtainQueryW ( psztargeturl : ::windows::core::PCWSTR , dwuserdata : u32 , fcallback : isize , phratingobtainquery : *mut super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    RatingObtainQueryW(psztargeturl.into_param().abi(), dwuserdata, fcallback, ::core::mem::transmute(phratingobtainquery.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingSetupUI<P0, P1>(hdlg: P0, pszusername: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingSetupUI ( hdlg : super::super::Foundation:: HWND , pszusername : ::windows::core::PCSTR ) -> ::windows::core::HRESULT );
    RatingSetupUI(hdlg.into_param().abi(), pszusername.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RatingSetupUIW<P0, P1>(hdlg: P0, pszusername: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "msrating.dll""system" fn RatingSetupUIW ( hdlg : super::super::Foundation:: HWND , pszusername : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    RatingSetupUIW(hdlg.into_param().abi(), pszusername.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SniffStream<P0>(pinstream: P0, pnformat: *mut u32, ppoutstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
{
    ::windows_targets::link ! ( "imgutil.dll""system" fn SniffStream ( pinstream : * mut::core::ffi::c_void , pnformat : *mut u32 , ppoutstream : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    SniffStream(pinstream.into_param().abi(), pnformat, ::core::mem::transmute(ppoutstream)).ok()
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IActiveXUIHandlerSite(::windows::core::IUnknown);
impl IActiveXUIHandlerSite {
    pub unsafe fn CreateScrollableContextMenu(&self) -> ::windows::core::Result<IScrollableContextMenu> {
        let mut result__ = ::windows::core::zeroed::<IScrollableContextMenu>();
        (::windows::core::Interface::vtable(self).CreateScrollableContextMenu)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PickFileAndGetResult<P0, P1>(&self, filepicker: P0, allowmultipleselections: P1) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).PickFileAndGetResult)(::windows::core::Interface::as_raw(self), filepicker.into_param().abi(), allowmultipleselections.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveXUIHandlerSite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveXUIHandlerSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveXUIHandlerSite {}
impl ::core::fmt::Debug for IActiveXUIHandlerSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveXUIHandlerSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveXUIHandlerSite {
    type Vtable = IActiveXUIHandlerSite_Vtbl;
}
impl ::core::clone::Clone for IActiveXUIHandlerSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveXUIHandlerSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510853_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveXUIHandlerSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateScrollableContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollablecontextmenu: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PickFileAndGetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepicker: *mut ::core::ffi::c_void, allowmultipleselections: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PickFileAndGetResult: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IActiveXUIHandlerSite2(::windows::core::IUnknown);
impl IActiveXUIHandlerSite2 {
    pub unsafe fn AddSuspensionExemption(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).AddSuspensionExemption)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveSuspensionExemption(&self, ullcookie: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveSuspensionExemption)(::windows::core::Interface::as_raw(self), ullcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveXUIHandlerSite2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveXUIHandlerSite2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveXUIHandlerSite2 {}
impl ::core::fmt::Debug for IActiveXUIHandlerSite2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveXUIHandlerSite2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveXUIHandlerSite2 {
    type Vtable = IActiveXUIHandlerSite2_Vtbl;
}
impl ::core::clone::Clone for IActiveXUIHandlerSite2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveXUIHandlerSite2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e3707b2_d087_4542_ac1f_a0d2fcd080fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveXUIHandlerSite2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddSuspensionExemption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullcookie: *mut u64) -> ::windows::core::HRESULT,
    pub RemoveSuspensionExemption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullcookie: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IActiveXUIHandlerSite3(::windows::core::IUnknown);
impl IActiveXUIHandlerSite3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBoxW<P0, P1, P2>(&self, hwnd: P0, text: P1, caption: P2, r#type: u32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MessageBoxW)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), text.into_param().abi(), caption.into_param().abi(), r#type, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveXUIHandlerSite3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveXUIHandlerSite3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveXUIHandlerSite3 {}
impl ::core::fmt::Debug for IActiveXUIHandlerSite3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveXUIHandlerSite3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveXUIHandlerSite3 {
    type Vtable = IActiveXUIHandlerSite3_Vtbl;
}
impl ::core::clone::Clone for IActiveXUIHandlerSite3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveXUIHandlerSite3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7904009a_1238_47f4_901c_871375c34608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveXUIHandlerSite3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageBoxW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, text: ::windows::core::PCWSTR, caption: ::windows::core::PCWSTR, r#type: u32, result: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageBoxW: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAnchorClick(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAnchorClick {
    pub unsafe fn ProcOnClick(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcOnClick)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAnchorClick, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAnchorClick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAnchorClick {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAnchorClick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAnchorClick").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAnchorClick {
    type Vtable = IAnchorClick_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAnchorClick {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAnchorClick {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d5413b_33b9_11d2_95a7_00c04f8ecb02);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAnchorClick_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProcOnClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IAudioSessionSite(::windows::core::IUnknown);
impl IAudioSessionSite {
    pub unsafe fn GetAudioSessionGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetAudioSessionGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnAudioStreamCreated<P0>(&self, endpointid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnAudioStreamCreated)(::windows::core::Interface::as_raw(self), endpointid.into_param().abi()).ok()
    }
    pub unsafe fn OnAudioStreamDestroyed<P0>(&self, endpointid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnAudioStreamDestroyed)(::windows::core::Interface::as_raw(self), endpointid.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionSite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSessionSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionSite {}
impl ::core::fmt::Debug for IAudioSessionSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionSite {
    type Vtable = IAudioSessionSite_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d8b684_d02d_4517_b6b7_19e3dfe29c45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAudioSessionGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosessionguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnAudioStreamCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnAudioStreamDestroyed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ICaretPositionProvider(::windows::core::IUnknown);
impl ICaretPositionProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCaretPosition(&self, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaretPosition)(::windows::core::Interface::as_raw(self), pptcaret, pflheight).ok()
    }
}
::windows::imp::interface_hierarchy!(ICaretPositionProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICaretPositionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICaretPositionProvider {}
impl ::core::fmt::Debug for ICaretPositionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICaretPositionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICaretPositionProvider {
    type Vtable = ICaretPositionProvider_Vtbl;
}
impl ::core::clone::Clone for ICaretPositionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICaretPositionProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58da43a2_108e_4d5b_9f75_e5f74f93fff5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICaretPositionProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCaretPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCaretPosition: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDeviceRect(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDeviceRect {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IDeviceRect, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDeviceRect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDeviceRect {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDeviceRect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceRect").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDeviceRect {
    type Vtable = IDeviceRect_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDeviceRect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IDeviceRect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f6d5_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRect_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IDithererImpl(::windows::core::IUnknown);
impl IDithererImpl {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetDestColorTable(&self, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDestColorTable)(::windows::core::Interface::as_raw(self), ncolors, prgbcolors).ok()
    }
    pub unsafe fn SetEventSink<P0>(&self, peventsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IImageDecodeEventSink>,
    {
        (::windows::core::Interface::vtable(self).SetEventSink)(::windows::core::Interface::as_raw(self), peventsink.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDithererImpl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDithererImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDithererImpl {}
impl ::core::fmt::Debug for IDithererImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDithererImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDithererImpl {
    type Vtable = IDithererImpl_Vtbl;
}
impl ::core::clone::Clone for IDithererImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDithererImpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c48e840_3910_11d0_86fc_00a0c913f750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDithererImpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetDestColorTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetDestColorTable: usize,
    pub SetEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IDocObjectService(::windows::core::IUnknown);
impl IDocObjectService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn FireBeforeNavigate2<P0, P1, P2, P3, P4>(&self, pdispatch: P0, lpszurl: P1, dwflags: u32, lpszframename: P2, ppostdata: *const u8, cbpostdata: u32, lpszheaders: P3, fplaynavsound: P4) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).FireBeforeNavigate2)(::windows::core::Interface::as_raw(self), pdispatch.into_param().abi(), lpszurl.into_param().abi(), dwflags, lpszframename.into_param().abi(), ppostdata, cbpostdata, lpszheaders.into_param().abi(), fplaynavsound.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn FireNavigateComplete2<P0>(&self, phtmlwindow2: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::MsHtml::IHTMLWindow2>,
    {
        (::windows::core::Interface::vtable(self).FireNavigateComplete2)(::windows::core::Interface::as_raw(self), phtmlwindow2.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn FireDownloadBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireDownloadBegin)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDownloadComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireDownloadComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn FireDocumentComplete<P0>(&self, phtmlwindow: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::MsHtml::IHTMLWindow2>,
    {
        (::windows::core::Interface::vtable(self).FireDocumentComplete)(::windows::core::Interface::as_raw(self), phtmlwindow.into_param().abi(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn UpdateDesktopComponent<P0>(&self, phtmlwindow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::MsHtml::IHTMLWindow2>,
    {
        (::windows::core::Interface::vtable(self).UpdateDesktopComponent)(::windows::core::Interface::as_raw(self), phtmlwindow.into_param().abi()).ok()
    }
    pub unsafe fn GetPendingUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetPendingUrl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn ActiveElementChanged<P0>(&self, phtmlelement: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::MsHtml::IHTMLElement>,
    {
        (::windows::core::Interface::vtable(self).ActiveElementChanged)(::windows::core::Interface::as_raw(self), phtmlelement.into_param().abi()).ok()
    }
    pub unsafe fn GetUrlSearchComponent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetUrlSearchComponent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsErrorUrl<P0>(&self, lpszurl: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsErrorUrl)(::windows::core::Interface::as_raw(self), lpszurl.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDocObjectService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDocObjectService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDocObjectService {}
impl ::core::fmt::Debug for IDocObjectService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDocObjectService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDocObjectService {
    type Vtable = IDocObjectService_Vtbl;
}
impl ::core::clone::Clone for IDocObjectService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDocObjectService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f801_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocObjectService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub FireBeforeNavigate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdispatch: *mut ::core::ffi::c_void, lpszurl: ::windows::core::PCWSTR, dwflags: u32, lpszframename: ::windows::core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: ::windows::core::PCWSTR, fplaynavsound: super::super::Foundation::BOOL, pfcancel: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    FireBeforeNavigate2: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub FireNavigateComplete2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtmlwindow2: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    FireNavigateComplete2: usize,
    pub FireDownloadBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FireDownloadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub FireDocumentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtmlwindow: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    FireDocumentComplete: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub UpdateDesktopComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtmlwindow: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    UpdateDesktopComponent: usize,
    pub GetPendingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpendingurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub ActiveElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtmlelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    ActiveElementChanged: usize,
    pub GetUrlSearchComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsearch: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsErrorUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszurl: ::windows::core::PCWSTR, pfiserror: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsErrorUrl: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDownloadBehavior(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadBehavior {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn startDownload<P0, P1>(&self, bstrurl: P0, pdispcallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).startDownload)(::windows::core::Interface::as_raw(self), bstrurl.into_param().abi(), pdispcallback.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IDownloadBehavior, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDownloadBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDownloadBehavior {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDownloadBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadBehavior").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDownloadBehavior {
    type Vtable = IDownloadBehavior_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDownloadBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IDownloadBehavior {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f5bd_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadBehavior_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub startDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, pdispcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    startDownload: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IDownloadManager(::windows::core::IUnknown);
impl IDownloadManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Download<P0, P1, P2, P3>(&self, pmk: P0, pbc: P1, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: P2, pszredir: P3, uicp: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IMoniker>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IBindCtx>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Download)(::windows::core::Interface::as_raw(self), pmk.into_param().abi(), pbc.into_param().abi(), dwbindverb, grfbindf, pbindinfo, pszheaders.into_param().abi(), pszredir.into_param().abi(), uicp).ok()
    }
}
::windows::imp::interface_hierarchy!(IDownloadManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDownloadManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDownloadManager {}
impl ::core::fmt::Debug for IDownloadManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDownloadManager {
    type Vtable = IDownloadManager_Vtbl;
}
impl ::core::clone::Clone for IDownloadManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDownloadManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x988934a4_064b_11d3_bb80_00104b35e7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: ::windows::core::PCWSTR, pszredir: ::windows::core::PCWSTR, uicp: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    Download: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IEnumManagerFrames(::windows::core::IUnknown);
impl IEnumManagerFrames {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, ppwindows: &mut [*mut super::super::Foundation::HWND], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppwindows.len() as _, ::core::mem::transmute(ppwindows.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumManagerFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumManagerFrames>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumManagerFrames, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumManagerFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumManagerFrames {}
impl ::core::fmt::Debug for IEnumManagerFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumManagerFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumManagerFrames {
    type Vtable = IEnumManagerFrames_Vtbl;
}
impl ::core::clone::Clone for IEnumManagerFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumManagerFrames {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3caa826a_9b1f_4a79_bc81_f0430ded1648);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumManagerFrames_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IEnumOpenServiceActivity(::windows::core::IUnknown);
impl IEnumOpenServiceActivity {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IOpenServiceActivity>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumOpenServiceActivity> {
        let mut result__ = ::windows::core::zeroed::<IEnumOpenServiceActivity>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumOpenServiceActivity, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumOpenServiceActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOpenServiceActivity {}
impl ::core::fmt::Debug for IEnumOpenServiceActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOpenServiceActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumOpenServiceActivity {
    type Vtable = IEnumOpenServiceActivity_Vtbl;
}
impl ::core::clone::Clone for IEnumOpenServiceActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumOpenServiceActivity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa436d7d2_17c3_4ef4_a1e8_5c86faff26c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOpenServiceActivity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IEnumOpenServiceActivityCategory(::windows::core::IUnknown);
impl IEnumOpenServiceActivityCategory {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IOpenServiceActivityCategory>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumOpenServiceActivityCategory> {
        let mut result__ = ::windows::core::zeroed::<IEnumOpenServiceActivityCategory>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumOpenServiceActivityCategory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumOpenServiceActivityCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOpenServiceActivityCategory {}
impl ::core::fmt::Debug for IEnumOpenServiceActivityCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOpenServiceActivityCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumOpenServiceActivityCategory {
    type Vtable = IEnumOpenServiceActivityCategory_Vtbl;
}
impl ::core::clone::Clone for IEnumOpenServiceActivityCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumOpenServiceActivityCategory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33627a56_8c9a_4430_8fd1_b5f5c771afb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOpenServiceActivityCategory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IEnumSTATURL(::windows::core::IUnknown);
impl IEnumSTATURL {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, rgelt, pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATURL> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATURL>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFilter<P0>(&self, poszfilter: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFilter)(::windows::core::Interface::as_raw(self), poszfilter.into_param().abi(), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumSTATURL, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumSTATURL {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATURL {}
impl ::core::fmt::Debug for IEnumSTATURL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATURL").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATURL {
    type Vtable = IEnumSTATURL_Vtbl;
}
impl ::core::clone::Clone for IEnumSTATURL {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumSTATURL {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c374a42_bae4_11cf_bf7d_00aa006946ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATURL_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poszfilter: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IExtensionValidation(::windows::core::IUnknown);
impl IExtensionValidation {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn Validate<P0, P1, P2, P3>(&self, extensionguid: *const ::windows::core::GUID, extensionmodulepath: P0, extensionfileversionms: u32, extensionfileversionls: u32, htmldocumenttop: P1, htmldocumentsubframe: P2, htmlelement: P3, contexts: ExtensionValidationContexts) -> ::windows::core::Result<ExtensionValidationResults>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::MsHtml::IHTMLDocument2>,
        P2: ::windows::core::IntoParam<super::MsHtml::IHTMLDocument2>,
        P3: ::windows::core::IntoParam<super::MsHtml::IHTMLElement>,
    {
        let mut result__ = ::windows::core::zeroed::<ExtensionValidationResults>();
        (::windows::core::Interface::vtable(self).Validate)(::windows::core::Interface::as_raw(self), extensionguid, extensionmodulepath.into_param().abi(), extensionfileversionms, extensionfileversionls, htmldocumenttop.into_param().abi(), htmldocumentsubframe.into_param().abi(), htmlelement.into_param().abi(), contexts, &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IExtensionValidation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IExtensionValidation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtensionValidation {}
impl ::core::fmt::Debug for IExtensionValidation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtensionValidation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtensionValidation {
    type Vtable = IExtensionValidation_Vtbl;
}
impl ::core::clone::Clone for IExtensionValidation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtensionValidation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d33f73d_8525_4e0f_87db_830288baff44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtensionValidation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensionguid: *const ::windows::core::GUID, extensionmodulepath: ::windows::core::PCWSTR, extensionfileversionms: u32, extensionfileversionls: u32, htmldocumenttop: *mut ::core::ffi::c_void, htmldocumentsubframe: *mut ::core::ffi::c_void, htmlelement: *mut ::core::ffi::c_void, contexts: ExtensionValidationContexts, results: *mut ExtensionValidationResults) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    Validate: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IHTMLPersistData(::windows::core::IUnknown);
impl IHTMLPersistData {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn save<P0>(&self, punk: P0, ltype: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).save)(::windows::core::Interface::as_raw(self), punk.into_param().abi(), ltype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn load<P0>(&self, punk: P0, ltype: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).load)(::windows::core::Interface::as_raw(self), punk.into_param().abi(), ltype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn queryType(&self, ltype: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).queryType)(::windows::core::Interface::as_raw(self), ltype, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IHTMLPersistData, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHTMLPersistData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHTMLPersistData {}
impl ::core::fmt::Debug for IHTMLPersistData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHTMLPersistData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHTMLPersistData {
    type Vtable = IHTMLPersistData_Vtbl;
}
impl ::core::clone::Clone for IHTMLPersistData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHTMLPersistData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f4c5_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHTMLPersistData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, ltype: i32, fcontinuebroacast: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    save: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, ltype: i32, fdodefault: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    load: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub queryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltype: i32, pfsupportstype: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    queryType: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IHTMLPersistDataOM(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IHTMLPersistDataOM {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn XMLDocument(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).XMLDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getAttribute<P0>(&self, name: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setAttribute<P0>(&self, name: P0, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn removeAttribute<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).removeAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IHTMLPersistDataOM, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IHTMLPersistDataOM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IHTMLPersistDataOM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IHTMLPersistDataOM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHTMLPersistDataOM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IHTMLPersistDataOM {
    type Vtable = IHTMLPersistDataOM_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IHTMLPersistDataOM {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IHTMLPersistDataOM {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f4c0_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHTMLPersistDataOM_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub XMLDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    XMLDocument: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IHTMLUserDataOM(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IHTMLUserDataOM {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn XMLDocument(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).XMLDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn save<P0>(&self, strname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).save)(::windows::core::Interface::as_raw(self), strname.into_param().abi()).ok()
    }
    pub unsafe fn load<P0>(&self, strname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).load)(::windows::core::Interface::as_raw(self), strname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn getAttribute<P0>(&self, name: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).getAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn setAttribute<P0>(&self, name: P0, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn removeAttribute<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).removeAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Setexpires<P0>(&self, bstr: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setexpires)(::windows::core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn expires(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).expires)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IHTMLUserDataOM, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IHTMLUserDataOM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IHTMLUserDataOM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IHTMLUserDataOM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHTMLUserDataOM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IHTMLUserDataOM {
    type Vtable = IHTMLUserDataOM_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IHTMLUserDataOM {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IHTMLUserDataOM {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f48f_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHTMLUserDataOM_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub XMLDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    XMLDocument: usize,
    pub save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub getAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    getAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub setAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    setAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setexpires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub expires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IHeaderFooter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IHeaderFooter {
    pub unsafe fn htmlHead(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).htmlHead)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn htmlFoot(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).htmlFoot)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettextHead<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SettextHead)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn textHead(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).textHead)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettextFoot<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SettextFoot)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn textFoot(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).textFoot)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setpage(&self, v: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Setpage)(::windows::core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn page(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).page)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetpageTotal(&self, v: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetpageTotal)(::windows::core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn pageTotal(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).pageTotal)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetURL<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetURL)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn URL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).URL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Settitle<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Settitle)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).title)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetdateShort<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetdateShort)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn dateShort(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).dateShort)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetdateLong<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetdateLong)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn dateLong(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).dateLong)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettimeShort<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SettimeShort)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn timeShort(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).timeShort)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettimeLong<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SettimeLong)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn timeLong(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).timeLong)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IHeaderFooter, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IHeaderFooter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IHeaderFooter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IHeaderFooter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHeaderFooter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IHeaderFooter {
    type Vtable = IHeaderFooter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IHeaderFooter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IHeaderFooter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f6ce_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderFooter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub htmlHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub htmlFoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SettextHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub textHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SettextFoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub textFoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Setpage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: u32) -> ::windows::core::HRESULT,
    pub page: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut u32) -> ::windows::core::HRESULT,
    pub SetpageTotal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: u32) -> ::windows::core::HRESULT,
    pub pageTotal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut u32) -> ::windows::core::HRESULT,
    pub SetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub URL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Settitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetdateShort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub dateShort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetdateLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub dateLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SettimeShort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub timeShort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SettimeLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub timeLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IHeaderFooter2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IHeaderFooter2 {
    pub unsafe fn htmlHead(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.htmlHead)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn htmlFoot(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.htmlFoot)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettextHead<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SettextHead)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn textHead(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.textHead)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettextFoot<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SettextFoot)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn textFoot(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.textFoot)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setpage(&self, v: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Setpage)(::windows::core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn page(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.page)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetpageTotal(&self, v: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetpageTotal)(::windows::core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn pageTotal(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.pageTotal)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetURL<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetURL)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn URL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.URL)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Settitle<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Settitle)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.title)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetdateShort<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdateShort)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn dateShort(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.dateShort)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetdateLong<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetdateLong)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn dateLong(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.dateLong)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettimeShort<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SettimeShort)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn timeShort(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.timeShort)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettimeLong<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SettimeLong)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn timeLong(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.timeLong)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Setfont<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Setfont)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    pub unsafe fn font(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).font)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IHeaderFooter2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IHeaderFooter);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IHeaderFooter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IHeaderFooter2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IHeaderFooter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHeaderFooter2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IHeaderFooter2 {
    type Vtable = IHeaderFooter2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IHeaderFooter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IHeaderFooter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x305104a5_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderFooter2_Vtbl {
    pub base__: IHeaderFooter_Vtbl,
    pub Setfont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IHomePage(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IHomePage {
    pub unsafe fn navigateHomePage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).navigateHomePage)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn setHomePage<P0>(&self, bstrurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).setHomePage)(::windows::core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn isHomePage<P0>(&self, bstrurl: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).isHomePage)(::windows::core::Interface::as_raw(self), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IHomePage, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IHomePage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IHomePage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IHomePage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHomePage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IHomePage {
    type Vtable = IHomePage_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IHomePage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IHomePage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x766bf2af_d650_11d1_9811_00c04fc31d2e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHomePage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub navigateHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub setHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub isHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    isHomePage: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IHomePageSetting(::windows::core::IUnknown);
impl IHomePageSetting {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHomePage<P0, P1, P2>(&self, hwnd: P0, homepageuri: P1, brandingmessage: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetHomePage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), homepageuri.into_param().abi(), brandingmessage.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsHomePage<P0>(&self, uri: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsHomePage)(::windows::core::Interface::as_raw(self), uri.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHomePageToBrowserDefault(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHomePageToBrowserDefault)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IHomePageSetting, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHomePageSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHomePageSetting {}
impl ::core::fmt::Debug for IHomePageSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHomePageSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHomePageSetting {
    type Vtable = IHomePageSetting_Vtbl;
}
impl ::core::clone::Clone for IHomePageSetting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHomePageSetting {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdfc244f_18fa_4ff2_b08e_1d618f3ffbe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHomePageSetting_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, homepageuri: ::windows::core::PCWSTR, brandingmessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHomePage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, isdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsHomePage: usize,
    pub SetHomePageToBrowserDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IIEWebDriverManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIEWebDriverManager {
    pub unsafe fn ExecuteCommand<P0>(&self, command: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).ExecuteCommand)(::windows::core::Interface::as_raw(self), command.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IIEWebDriverManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IIEWebDriverManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IIEWebDriverManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IIEWebDriverManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIEWebDriverManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IIEWebDriverManager {
    type Vtable = IIEWebDriverManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IIEWebDriverManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IIEWebDriverManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1dc630_6590_4ca2_a293_6bc72b2438d8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIEWebDriverManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExecuteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows::core::PCWSTR, response: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IIEWebDriverSite(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIEWebDriverSite {
    pub unsafe fn WindowOperation(&self, operationcode: u32, hwnd: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WindowOperation)(::windows::core::Interface::as_raw(self), operationcode, hwnd).ok()
    }
    pub unsafe fn DetachWebdriver<P0>(&self, punkwd: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).DetachWebdriver)(::windows::core::Interface::as_raw(self), punkwd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCapabilityValue<P0, P1>(&self, punkwd: P0, capname: P1) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetCapabilityValue)(::windows::core::Interface::as_raw(self), punkwd.into_param().abi(), capname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IIEWebDriverSite, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IIEWebDriverSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IIEWebDriverSite {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IIEWebDriverSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIEWebDriverSite").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IIEWebDriverSite {
    type Vtable = IIEWebDriverSite_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IIEWebDriverSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IIEWebDriverSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffb84444_453d_4fbc_9f9d_8db5c471ec75);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIEWebDriverSite_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WindowOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationcode: u32, hwnd: u32) -> ::windows::core::HRESULT,
    pub DetachWebdriver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkwd: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCapabilityValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkwd: *mut ::core::ffi::c_void, capname: ::windows::core::PCWSTR, capvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCapabilityValue: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IImageDecodeEventSink(::windows::core::IUnknown);
impl IImageDecodeEventSink {
    pub unsafe fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const ::windows::core::GUID, npasses: u32, dwhints: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetSurface)(::windows::core::Interface::as_raw(self), nwidth, nheight, bfid, npasses, dwhints, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBeginDecode)(::windows::core::Interface::as_raw(self), pdwevents, pnformats, ppformats).ok()
    }
    pub unsafe fn OnBitsComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBitsComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDecodeComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDecodeComplete)(::windows::core::Interface::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn OnPalette(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPalette)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<P0>(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnProgress)(::windows::core::Interface::as_raw(self), pbounds, bcomplete.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IImageDecodeEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IImageDecodeEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageDecodeEventSink {}
impl ::core::fmt::Debug for IImageDecodeEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageDecodeEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImageDecodeEventSink {
    type Vtable = IImageDecodeEventSink_Vtbl;
}
impl ::core::clone::Clone for IImageDecodeEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageDecodeEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbaa342a0_2ded_11d0_86f4_00a0c913f750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDecodeEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nwidth: i32, nheight: i32, bfid: *const ::windows::core::GUID, npasses: u32, dwhints: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnBeginDecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnBitsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnDecodeComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnProgress: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IImageDecodeEventSink2(::windows::core::IUnknown);
impl IImageDecodeEventSink2 {
    pub unsafe fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const ::windows::core::GUID, npasses: u32, dwhints: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.GetSurface)(::windows::core::Interface::as_raw(self), nwidth, nheight, bfid, npasses, dwhints, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnBeginDecode)(::windows::core::Interface::as_raw(self), pdwevents, pnformats, ppformats).ok()
    }
    pub unsafe fn OnBitsComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnBitsComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDecodeComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnDecodeComplete)(::windows::core::Interface::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn OnPalette(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnPalette)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<P0>(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.OnProgress)(::windows::core::Interface::as_raw(self), pbounds, bcomplete.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAlphaPremultRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsAlphaPremultRequired)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IImageDecodeEventSink2, ::windows::core::IUnknown, IImageDecodeEventSink);
impl ::core::cmp::PartialEq for IImageDecodeEventSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageDecodeEventSink2 {}
impl ::core::fmt::Debug for IImageDecodeEventSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageDecodeEventSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImageDecodeEventSink2 {
    type Vtable = IImageDecodeEventSink2_Vtbl;
}
impl ::core::clone::Clone for IImageDecodeEventSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageDecodeEventSink2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ebd8a57_8a96_48c9_84a6_962e2db9c931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDecodeEventSink2_Vtbl {
    pub base__: IImageDecodeEventSink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAlphaPremultRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfpremultalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAlphaPremultRequired: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IImageDecodeFilter(::windows::core::IUnknown);
impl IImageDecodeFilter {
    pub unsafe fn Initialize<P0>(&self, peventsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IImageDecodeEventSink>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), peventsink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Process<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Process)(::windows::core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self), hrstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IImageDecodeFilter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IImageDecodeFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageDecodeFilter {}
impl ::core::fmt::Debug for IImageDecodeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageDecodeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImageDecodeFilter {
    type Vtable = IImageDecodeFilter_Vtbl;
}
impl ::core::clone::Clone for IImageDecodeFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageDecodeFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3ccedf3_2de2_11d0_86f4_00a0c913f750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDecodeFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Process: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IIntelliForms(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIntelliForms {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).enabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Setenabled<P0>(&self, bval: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Setenabled)(::windows::core::Interface::as_raw(self), bval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IIntelliForms, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IIntelliForms {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IIntelliForms {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IIntelliForms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIntelliForms").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IIntelliForms {
    type Vtable = IIntelliForms_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IIntelliForms {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IIntelliForms {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9f68e6_1aaa_11d2_bca5_00c04fd929db);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIntelliForms_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Setenabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Setenabled: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IInternetExplorerManager(::windows::core::IUnknown);
impl IInternetExplorerManager {
    pub unsafe fn CreateObject<P0, T>(&self, dwconfig: u32, pszurl: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateObject)(::windows::core::Interface::as_raw(self), dwconfig, pszurl.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IInternetExplorerManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetExplorerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetExplorerManager {}
impl ::core::fmt::Debug for IInternetExplorerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetExplorerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetExplorerManager {
    type Vtable = IInternetExplorerManager_Vtbl;
}
impl ::core::clone::Clone for IInternetExplorerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetExplorerManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacc84351_04ff_44f9_b23f_655ed168c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetExplorerManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconfig: u32, pszurl: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IInternetExplorerManager2(::windows::core::IUnknown);
impl IInternetExplorerManager2 {
    pub unsafe fn EnumFrameWindows(&self) -> ::windows::core::Result<IEnumManagerFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumManagerFrames>();
        (::windows::core::Interface::vtable(self).EnumFrameWindows)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IInternetExplorerManager2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternetExplorerManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetExplorerManager2 {}
impl ::core::fmt::Debug for IInternetExplorerManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetExplorerManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternetExplorerManager2 {
    type Vtable = IInternetExplorerManager2_Vtbl;
}
impl ::core::clone::Clone for IInternetExplorerManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternetExplorerManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfbb5136_9259_4895_b4a7_c1934429919a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetExplorerManager2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumFrameWindows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ILayoutRect(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ILayoutRect {
    pub unsafe fn SetnextRect<P0>(&self, bstrelementid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetnextRect)(::windows::core::Interface::as_raw(self), bstrelementid.into_param().abi()).ok()
    }
    pub unsafe fn nextRect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).nextRect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetcontentSrc(&self, varcontentsrc: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetcontentSrc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varcontentsrc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn contentSrc(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).contentSrc)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SethonorPageBreaks<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SethonorPageBreaks)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn honorPageBreaks(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).honorPageBreaks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SethonorPageRules<P0>(&self, v: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SethonorPageRules)(::windows::core::Interface::as_raw(self), v.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn honorPageRules(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).honorPageRules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetnextRectElement<P0>(&self, pelem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).SetnextRectElement)(::windows::core::Interface::as_raw(self), pelem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextRectElement(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).nextRectElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn contentDocument(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).contentDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ILayoutRect, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ILayoutRect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ILayoutRect {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ILayoutRect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILayoutRect").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ILayoutRect {
    type Vtable = ILayoutRect_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ILayoutRect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ILayoutRect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f665_98b5_11cf_bb82_00aa00bdce0b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutRect_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetnextRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrelementid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub nextRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrelementid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetcontentSrc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varcontentsrc: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetcontentSrc: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub contentSrc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcontentsrc: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    contentSrc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SethonorPageBreaks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SethonorPageBreaks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub honorPageBreaks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    honorPageBreaks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SethonorPageRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SethonorPageRules: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub honorPageRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    honorPageRules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetnextRectElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetnextRectElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub nextRectElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    nextRectElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub contentDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    contentDocument: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IMapMIMEToCLSID(::windows::core::IUnknown);
impl IMapMIMEToCLSID {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableDefaultMappings<P0>(&self, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableDefaultMappings)(::windows::core::Interface::as_raw(self), benable.into_param().abi()).ok()
    }
    pub unsafe fn MapMIMEToCLSID<P0>(&self, pszmimetype: P0, pclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MapMIMEToCLSID)(::windows::core::Interface::as_raw(self), pszmimetype.into_param().abi(), pclsid).ok()
    }
    pub unsafe fn SetMapping<P0>(&self, pszmimetype: P0, dwmapmode: u32, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetMapping)(::windows::core::Interface::as_raw(self), pszmimetype.into_param().abi(), dwmapmode, clsid).ok()
    }
}
::windows::imp::interface_hierarchy!(IMapMIMEToCLSID, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMapMIMEToCLSID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMapMIMEToCLSID {}
impl ::core::fmt::Debug for IMapMIMEToCLSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMapMIMEToCLSID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMapMIMEToCLSID {
    type Vtable = IMapMIMEToCLSID_Vtbl;
}
impl ::core::clone::Clone for IMapMIMEToCLSID {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMapMIMEToCLSID {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9e89500_30fa_11d0_b724_00aa006c1a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapMIMEToCLSID_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableDefaultMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableDefaultMappings: usize,
    pub MapMIMEToCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmimetype: ::windows::core::PCWSTR, pclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmimetype: ::windows::core::PCWSTR, dwmapmode: u32, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IMediaActivityNotifySite(::windows::core::IUnknown);
impl IMediaActivityNotifySite {
    pub unsafe fn OnMediaActivityStarted(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMediaActivityStarted)(::windows::core::Interface::as_raw(self), mediaactivitytype).ok()
    }
    pub unsafe fn OnMediaActivityStopped(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMediaActivityStopped)(::windows::core::Interface::as_raw(self), mediaactivitytype).ok()
    }
}
::windows::imp::interface_hierarchy!(IMediaActivityNotifySite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMediaActivityNotifySite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaActivityNotifySite {}
impl ::core::fmt::Debug for IMediaActivityNotifySite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaActivityNotifySite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMediaActivityNotifySite {
    type Vtable = IMediaActivityNotifySite_Vtbl;
}
impl ::core::clone::Clone for IMediaActivityNotifySite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaActivityNotifySite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8165cfef_179d_46c2_bc71_3fa726dc1f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaActivityNotifySite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnMediaActivityStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows::core::HRESULT,
    pub OnMediaActivityStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenService(::windows::core::IUnknown);
impl IOpenService {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsDefault)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefault<P0, P1>(&self, fdefault: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetDefault)(::windows::core::Interface::as_raw(self), fdefault.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpenService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpenService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenService {}
impl ::core::fmt::Debug for IOpenService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenService {
    type Vtable = IOpenService_Vtbl;
}
impl ::core::clone::Clone for IOpenService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2952ed1_6a89_4606_925f_1ed8b4be0630);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefault: usize,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenServiceActivity(::windows::core::IUnknown);
impl IOpenServiceActivity {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsDefault)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefault<P0, P1>(&self, fdefault: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDefault)(::windows::core::Interface::as_raw(self), fdefault.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Execute<P0, P1>(&self, pinput: P0, poutput: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivityInput>,
        P1: ::windows::core::IntoParam<IOpenServiceActivityOutputContext>,
    {
        (::windows::core::Interface::vtable(self).Execute)(::windows::core::Interface::as_raw(self), pinput.into_param().abi(), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanExecute<P0, P1>(&self, pinput: P0, poutput: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivityInput>,
        P1: ::windows::core::IntoParam<IOpenServiceActivityOutputContext>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanExecute)(::windows::core::Interface::as_raw(self), pinput.into_param().abi(), poutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanExecuteType(&self, r#type: OpenServiceActivityContentType) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanExecuteType)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn Preview<P0, P1>(&self, pinput: P0, poutput: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivityInput>,
        P1: ::windows::core::IntoParam<IOpenServiceActivityOutputContext>,
    {
        (::windows::core::Interface::vtable(self).Preview)(::windows::core::Interface::as_raw(self), pinput.into_param().abi(), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanPreview<P0, P1>(&self, pinput: P0, poutput: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivityInput>,
        P1: ::windows::core::IntoParam<IOpenServiceActivityOutputContext>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanPreview)(::windows::core::Interface::as_raw(self), pinput.into_param().abi(), poutput.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanPreviewType(&self, r#type: OpenServiceActivityContentType) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanPreviewType)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatusText<P0>(&self, pinput: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivityInput>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetStatusText)(::windows::core::Interface::as_raw(self), pinput.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetHomepageUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetHomepageUrl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCategoryName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetCategoryName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetIconPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetIcon<P0>(&self, fsmallicon: P0) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HICON>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::UI::WindowsAndMessaging::HICON>();
        (::windows::core::Interface::vtable(self).GetIcon)(::windows::core::Interface::as_raw(self), fsmallicon.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescriptionFilePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescriptionFilePath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDownloadUrl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInstallUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetInstallUrl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IOpenServiceActivity, ::windows::core::IUnknown, IOpenService);
impl ::core::cmp::PartialEq for IOpenServiceActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenServiceActivity {}
impl ::core::fmt::Debug for IOpenServiceActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenServiceActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenServiceActivity {
    type Vtable = IOpenServiceActivity_Vtbl;
}
impl ::core::clone::Clone for IOpenServiceActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenServiceActivity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13645c88_221a_4905_8ed1_4f5112cfc108);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivity_Vtbl {
    pub base__: IOpenService_Vtbl,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, pfcanexecute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanExecute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanExecuteType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanexecute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanExecuteType: usize,
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, pfcanpreview: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanPreview: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanPreviewType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanpreview: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanPreviewType: usize,
    pub GetStatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetHomepageUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhomepageurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetCategoryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcategoryname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstriconpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsmallicon: super::super::Foundation::BOOL, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetIcon: usize,
    pub GetDescriptionFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxmlpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxmluri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetInstallUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinstalluri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenServiceActivityCategory(::windows::core::IUnknown);
impl IOpenServiceActivityCategory {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasDefaultActivity(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).HasDefaultActivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultActivity(&self) -> ::windows::core::Result<IOpenServiceActivity> {
        let mut result__ = ::windows::core::zeroed::<IOpenServiceActivity>();
        (::windows::core::Interface::vtable(self).GetDefaultActivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultActivity<P0, P1>(&self, pactivity: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivity>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetDefaultActivity)(::windows::core::Interface::as_raw(self), pactivity.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActivityEnumerator<P0, P1>(&self, pinput: P0, poutput: P1) -> ::windows::core::Result<IEnumOpenServiceActivity>
    where
        P0: ::windows::core::IntoParam<IOpenServiceActivityInput>,
        P1: ::windows::core::IntoParam<IOpenServiceActivityOutputContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumOpenServiceActivity>();
        (::windows::core::Interface::vtable(self).GetActivityEnumerator)(::windows::core::Interface::as_raw(self), pinput.into_param().abi(), poutput.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpenServiceActivityCategory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpenServiceActivityCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenServiceActivityCategory {}
impl ::core::fmt::Debug for IOpenServiceActivityCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenServiceActivityCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenServiceActivityCategory {
    type Vtable = IOpenServiceActivityCategory_Vtbl;
}
impl ::core::clone::Clone for IOpenServiceActivityCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenServiceActivityCategory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x850af9d6_7309_40b5_bdb8_786c106b2153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityCategory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HasDefaultActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasdefaultactivity: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasDefaultActivity: usize,
    pub GetDefaultActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdefaultactivity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivity: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultActivity: usize,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetActivityEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, ppenumactivity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenServiceActivityInput(::windows::core::IUnknown);
impl IOpenServiceActivityInput {
    pub unsafe fn GetVariable<P0, P1>(&self, pwzvariablename: P0, pwzvariabletype: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetVariable)(::windows::core::Interface::as_raw(self), pwzvariablename.into_param().abi(), pwzvariabletype.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVariable<P0, P1>(&self, pwzvariablename: P0, pwzvariabletype: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).HasVariable)(::windows::core::Interface::as_raw(self), pwzvariablename.into_param().abi(), pwzvariabletype.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<OpenServiceActivityContentType> {
        let mut result__ = ::windows::core::zeroed::<OpenServiceActivityContentType>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpenServiceActivityInput, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpenServiceActivityInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenServiceActivityInput {}
impl ::core::fmt::Debug for IOpenServiceActivityInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenServiceActivityInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenServiceActivityInput {
    type Vtable = IOpenServiceActivityInput_Vtbl;
}
impl ::core::clone::Clone for IOpenServiceActivityInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenServiceActivityInput {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cb4db9_6da0_4da3_83ce_422b6a433346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityInput_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzvariablename: ::windows::core::PCWSTR, pwzvariabletype: ::windows::core::PCWSTR, pbstrvariablecontent: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzvariablename: ::windows::core::PCWSTR, pwzvariabletype: ::windows::core::PCWSTR, pfhasvariable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVariable: usize,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut OpenServiceActivityContentType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenServiceActivityManager(::windows::core::IUnknown);
impl IOpenServiceActivityManager {
    pub unsafe fn GetCategoryEnumerator(&self, etype: OpenServiceActivityContentType) -> ::windows::core::Result<IEnumOpenServiceActivityCategory> {
        let mut result__ = ::windows::core::zeroed::<IEnumOpenServiceActivityCategory>();
        (::windows::core::Interface::vtable(self).GetCategoryEnumerator)(::windows::core::Interface::as_raw(self), etype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActivityByID<P0>(&self, pwzactivityid: P0) -> ::windows::core::Result<IOpenServiceActivity>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpenServiceActivity>();
        (::windows::core::Interface::vtable(self).GetActivityByID)(::windows::core::Interface::as_raw(self), pwzactivityid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActivityByHomepageAndCategory<P0, P1>(&self, pwzhomepage: P0, pwzcategory: P1) -> ::windows::core::Result<IOpenServiceActivity>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpenServiceActivity>();
        (::windows::core::Interface::vtable(self).GetActivityByHomepageAndCategory)(::windows::core::Interface::as_raw(self), pwzhomepage.into_param().abi(), pwzcategory.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersionCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetVersionCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpenServiceActivityManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpenServiceActivityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenServiceActivityManager {}
impl ::core::fmt::Debug for IOpenServiceActivityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenServiceActivityManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenServiceActivityManager {
    type Vtable = IOpenServiceActivityManager_Vtbl;
}
impl ::core::clone::Clone for IOpenServiceActivityManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenServiceActivityManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a2d0a9d_e920_4bdc_a291_d30f650bc4f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCategoryEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, etype: OpenServiceActivityContentType, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActivityByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzactivityid: ::windows::core::PCWSTR, ppactivity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActivityByHomepageAndCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzhomepage: ::windows::core::PCWSTR, pwzcategory: ::windows::core::PCWSTR, ppactivity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVersionCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversioncookie: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenServiceActivityOutputContext(::windows::core::IUnknown);
impl IOpenServiceActivityOutputContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Navigate<P0, P1, P2, P3>(&self, pwzuri: P0, pwzmethod: P1, pwzheaders: P2, ppostdata: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Navigate)(::windows::core::Interface::as_raw(self), pwzuri.into_param().abi(), pwzmethod.into_param().abi(), pwzheaders.into_param().abi(), ppostdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CanNavigate<P0, P1, P2, P3>(&self, pwzuri: P0, pwzmethod: P1, pwzheaders: P2, ppostdata: P3) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanNavigate)(::windows::core::Interface::as_raw(self), pwzuri.into_param().abi(), pwzmethod.into_param().abi(), pwzheaders.into_param().abi(), ppostdata.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpenServiceActivityOutputContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpenServiceActivityOutputContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenServiceActivityOutputContext {}
impl ::core::fmt::Debug for IOpenServiceActivityOutputContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenServiceActivityOutputContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenServiceActivityOutputContext {
    type Vtable = IOpenServiceActivityOutputContext_Vtbl;
}
impl ::core::clone::Clone for IOpenServiceActivityOutputContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenServiceActivityOutputContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe289deab_f709_49a9_b99e_282364074571);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityOutputContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzuri: ::windows::core::PCWSTR, pwzmethod: ::windows::core::PCWSTR, pwzheaders: ::windows::core::PCWSTR, ppostdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Navigate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CanNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzuri: ::windows::core::PCWSTR, pwzmethod: ::windows::core::PCWSTR, pwzheaders: ::windows::core::PCWSTR, ppostdata: *mut ::core::ffi::c_void, pfcannavigate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CanNavigate: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IOpenServiceManager(::windows::core::IUnknown);
impl IOpenServiceManager {
    pub unsafe fn InstallService<P0>(&self, pwzserviceurl: P0) -> ::windows::core::Result<IOpenService>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpenService>();
        (::windows::core::Interface::vtable(self).InstallService)(::windows::core::Interface::as_raw(self), pwzserviceurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UninstallService<P0>(&self, pservice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpenService>,
    {
        (::windows::core::Interface::vtable(self).UninstallService)(::windows::core::Interface::as_raw(self), pservice.into_param().abi()).ok()
    }
    pub unsafe fn GetServiceByID<P0>(&self, pwzid: P0) -> ::windows::core::Result<IOpenService>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpenService>();
        (::windows::core::Interface::vtable(self).GetServiceByID)(::windows::core::Interface::as_raw(self), pwzid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpenServiceManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpenServiceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenServiceManager {}
impl ::core::fmt::Debug for IOpenServiceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenServiceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpenServiceManager {
    type Vtable = IOpenServiceManager_Vtbl;
}
impl ::core::clone::Clone for IOpenServiceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpenServiceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5664125f_4e10_4e90_98e4_e4513d955a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InstallService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzserviceurl: ::windows::core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UninstallService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetServiceByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzid: ::windows::core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IPeerFactory(::windows::core::IUnknown);
impl IPeerFactory {}
::windows::imp::interface_hierarchy!(IPeerFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPeerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPeerFactory {}
impl ::core::fmt::Debug for IPeerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPeerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPeerFactory {
    type Vtable = IPeerFactory_Vtbl;
}
impl ::core::clone::Clone for IPeerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPeerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6663f9d3_b482_11d1_89c6_00c04fb6bfc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPersistHistory(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPersistHistory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetClassID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadHistory<P0, P1>(&self, pstream: P0, pbc: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IBindCtx>,
    {
        (::windows::core::Interface::vtable(self).LoadHistory)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), pbc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveHistory<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).SaveHistory)(::windows::core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
    pub unsafe fn SetPositionCookie(&self, dwpositioncookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPositionCookie)(::windows::core::Interface::as_raw(self), dwpositioncookie).ok()
    }
    pub unsafe fn GetPositionCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPositionCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IPersistHistory, ::windows::core::IUnknown, super::super::System::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistHistory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistHistory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPersistHistory {
    type Vtable = IPersistHistory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPersistHistory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IPersistHistory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a565c1_e38f_11d0_94bf_00a0c9055cbf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistHistory_Vtbl {
    pub base__: super::super::System::Com::IPersist_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadHistory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveHistory: usize,
    pub SetPositionCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpositioncookie: u32) -> ::windows::core::HRESULT,
    pub GetPositionCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpositioncookie: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IPrintTaskRequestFactory(::windows::core::IUnknown);
impl IPrintTaskRequestFactory {
    pub unsafe fn CreatePrintTaskRequest<P0>(&self, pprinttaskrequesthandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPrintTaskRequestHandler>,
    {
        (::windows::core::Interface::vtable(self).CreatePrintTaskRequest)(::windows::core::Interface::as_raw(self), pprinttaskrequesthandler.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPrintTaskRequestFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPrintTaskRequestFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskRequestFactory {}
impl ::core::fmt::Debug for IPrintTaskRequestFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskRequestFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintTaskRequestFactory {
    type Vtable = IPrintTaskRequestFactory_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskRequestFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskRequestFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb516745_8c34_4f8b_9605_684dcb144be5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreatePrintTaskRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprinttaskrequesthandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IPrintTaskRequestHandler(::windows::core::IUnknown);
impl IPrintTaskRequestHandler {
    pub unsafe fn HandlePrintTaskRequest<P0>(&self, pprinttaskrequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        (::windows::core::Interface::vtable(self).HandlePrintTaskRequest)(::windows::core::Interface::as_raw(self), pprinttaskrequest.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPrintTaskRequestHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPrintTaskRequestHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskRequestHandler {}
impl ::core::fmt::Debug for IPrintTaskRequestHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskRequestHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintTaskRequestHandler {
    type Vtable = IPrintTaskRequestHandler_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskRequestHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskRequestHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x191cd340_cf36_44ff_bd53_d1b701799d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandlePrintTaskRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprinttaskrequest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IScrollableContextMenu(::windows::core::IUnknown);
impl IScrollableContextMenu {
    pub unsafe fn AddItem<P0>(&self, itemtext: P0, cmdid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddItem)(::windows::core::Interface::as_raw(self), itemtext.into_param().abi(), cmdid).ok()
    }
    pub unsafe fn ShowModal(&self, x: i32, y: i32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).ShowModal)(::windows::core::Interface::as_raw(self), x, y, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IScrollableContextMenu, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IScrollableContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollableContextMenu {}
impl ::core::fmt::Debug for IScrollableContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollableContextMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScrollableContextMenu {
    type Vtable = IScrollableContextMenu_Vtbl;
}
impl ::core::clone::Clone for IScrollableContextMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IScrollableContextMenu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510854_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollableContextMenu_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemtext: ::windows::core::PCWSTR, cmdid: u32) -> ::windows::core::HRESULT,
    pub ShowModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, cmdid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IScrollableContextMenu2(::windows::core::IUnknown);
impl IScrollableContextMenu2 {
    pub unsafe fn AddItem<P0>(&self, itemtext: P0, cmdid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddItem)(::windows::core::Interface::as_raw(self), itemtext.into_param().abi(), cmdid).ok()
    }
    pub unsafe fn ShowModal(&self, x: i32, y: i32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.ShowModal)(::windows::core::Interface::as_raw(self), x, y, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddSeparator(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSeparator)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPlacement(&self, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlacement)(::windows::core::Interface::as_raw(self), scmp).ok()
    }
}
::windows::imp::interface_hierarchy!(IScrollableContextMenu2, ::windows::core::IUnknown, IScrollableContextMenu);
impl ::core::cmp::PartialEq for IScrollableContextMenu2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollableContextMenu2 {}
impl ::core::fmt::Debug for IScrollableContextMenu2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollableContextMenu2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScrollableContextMenu2 {
    type Vtable = IScrollableContextMenu2_Vtbl;
}
impl ::core::clone::Clone for IScrollableContextMenu2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IScrollableContextMenu2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf77e9056_8674_4936_924c_0e4a06fa634a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollableContextMenu2_Vtbl {
    pub base__: IScrollableContextMenu_Vtbl,
    pub AddSeparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ISniffStream(::windows::core::IUnknown);
impl ISniffStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Init<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
    pub unsafe fn Peek(&self, pbuffer: *mut ::core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Peek)(::windows::core::Interface::as_raw(self), pbuffer, nbytes, pnbytesread).ok()
    }
}
::windows::imp::interface_hierarchy!(ISniffStream, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISniffStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISniffStream {}
impl ::core::fmt::Debug for ISniffStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISniffStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISniffStream {
    type Vtable = ISniffStream_Vtbl;
}
impl ::core::clone::Clone for ISniffStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISniffStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef17940_30e0_11d0_b724_00aa006c1a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISniffStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Init: usize,
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ISurfacePresenterFlip(::windows::core::IUnknown);
impl ISurfacePresenterFlip {
    pub unsafe fn Present(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Present)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBuffer(&self, backbufferindex: u32, riid: *const ::windows::core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), backbufferindex, riid, ppbuffer).ok()
    }
}
::windows::imp::interface_hierarchy!(ISurfacePresenterFlip, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISurfacePresenterFlip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfacePresenterFlip {}
impl ::core::fmt::Debug for ISurfacePresenterFlip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfacePresenterFlip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurfacePresenterFlip {
    type Vtable = ISurfacePresenterFlip_Vtbl;
}
impl ::core::clone::Clone for ISurfacePresenterFlip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISurfacePresenterFlip {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510848_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfacePresenterFlip_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backbufferindex: u32, riid: *const ::windows::core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ISurfacePresenterFlip2(::windows::core::IUnknown);
impl ISurfacePresenterFlip2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRotation)(::windows::core::Interface::as_raw(self), dxgirotation).ok()
    }
}
::windows::imp::interface_hierarchy!(ISurfacePresenterFlip2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISurfacePresenterFlip2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfacePresenterFlip2 {}
impl ::core::fmt::Debug for ISurfacePresenterFlip2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfacePresenterFlip2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurfacePresenterFlip2 {
    type Vtable = ISurfacePresenterFlip2_Vtbl;
}
impl ::core::clone::Clone for ISurfacePresenterFlip2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISurfacePresenterFlip2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510865_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfacePresenterFlip2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ISurfacePresenterFlipBuffer(::windows::core::IUnknown);
impl ISurfacePresenterFlipBuffer {
    pub unsafe fn BeginDraw(&self, riid: *const ::windows::core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginDraw)(::windows::core::Interface::as_raw(self), riid, ppbuffer).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISurfacePresenterFlipBuffer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISurfacePresenterFlipBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfacePresenterFlipBuffer {}
impl ::core::fmt::Debug for ISurfacePresenterFlipBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfacePresenterFlipBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurfacePresenterFlipBuffer {
    type Vtable = ISurfacePresenterFlipBuffer_Vtbl;
}
impl ::core::clone::Clone for ISurfacePresenterFlipBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISurfacePresenterFlipBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe43f4a08_8bbc_4665_ac92_c55ce61fd7e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfacePresenterFlipBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetContainer(::windows::core::IUnknown);
impl ITargetContainer {
    pub unsafe fn GetFrameUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFrameUrl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> ::windows::core::Result<super::super::System::Ole::IOleContainer> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Ole::IOleContainer>();
        (::windows::core::Interface::vtable(self).GetFramesContainer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITargetContainer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITargetContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetContainer {}
impl ::core::fmt::Debug for ITargetContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetContainer {
    type Vtable = ITargetContainer_Vtbl;
}
impl ::core::clone::Clone for ITargetContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7847ec01_2bec_11d0_82b4_00a0c90c29c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFrameUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetEmbedding(::windows::core::IUnknown);
impl ITargetEmbedding {
    pub unsafe fn GetTargetFrame(&self) -> ::windows::core::Result<ITargetFrame> {
        let mut result__ = ::windows::core::zeroed::<ITargetFrame>();
        (::windows::core::Interface::vtable(self).GetTargetFrame)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITargetEmbedding, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITargetEmbedding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetEmbedding {}
impl ::core::fmt::Debug for ITargetEmbedding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetEmbedding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetEmbedding {
    type Vtable = ITargetEmbedding_Vtbl;
}
impl ::core::clone::Clone for ITargetEmbedding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetEmbedding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x548793c0_9e74_11cf_9655_00a0c9034923);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetEmbedding_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTargetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptargetframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetFrame(::windows::core::IUnknown);
impl ITargetFrame {
    pub unsafe fn SetFrameName<P0>(&self, pszframename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFrameName)(::windows::core::Interface::as_raw(self), pszframename.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFrameName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentFrame(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetParentFrame)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFrame<P0, P1>(&self, psztargetname: P0, ppunkcontextframe: P1, dwflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).FindFrame)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), ppunkcontextframe.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFrameSrc<P0>(&self, pszframesrc: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFrameSrc)(::windows::core::Interface::as_raw(self), pszframesrc.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameSrc(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFrameSrc)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> ::windows::core::Result<super::super::System::Ole::IOleContainer> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Ole::IOleContainer>();
        (::windows::core::Interface::vtable(self).GetFramesContainer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFrameOptions(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFrameOptions)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFrameOptions(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetFrameOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFrameMargins)(::windows::core::Interface::as_raw(self), dwwidth, dwheight).ok()
    }
    pub unsafe fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameMargins)(::windows::core::Interface::as_raw(self), pdwwidth, pdwheight).ok()
    }
    pub unsafe fn RemoteNavigate(&self, puldata: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoteNavigate)(::windows::core::Interface::as_raw(self), puldata.len() as _, ::core::mem::transmute(puldata.as_ptr())).ok()
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnChildFrameActivate)(::windows::core::Interface::as_raw(self), punkchildframe.into_param().abi()).ok()
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnChildFrameDeactivate)(::windows::core::Interface::as_raw(self), punkchildframe.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITargetFrame, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITargetFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetFrame {}
impl ::core::fmt::Debug for ITargetFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetFrame {
    type Vtable = ITargetFrame_Vtbl;
}
impl ::core::clone::Clone for ITargetFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f78c80_5252_11cf_90fa_00aa0042106e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFrame_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFrameName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszframename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetFrameName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszframename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetParentFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, ppunkcontextframe: *mut ::core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFrameSrc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszframesrc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetFrameSrc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
    pub SetFrameOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetFrameOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetFrameMargins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwidth: u32, dwheight: u32) -> ::windows::core::HRESULT,
    pub GetFrameMargins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT,
    pub RemoteNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clength: u32, puldata: *const u32) -> ::windows::core::HRESULT,
    pub OnChildFrameActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnChildFrameDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetFrame2(::windows::core::IUnknown);
impl ITargetFrame2 {
    pub unsafe fn SetFrameName<P0>(&self, pszframename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFrameName)(::windows::core::Interface::as_raw(self), pszframename.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFrameName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentFrame(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetParentFrame)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFrameSrc<P0>(&self, pszframesrc: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFrameSrc)(::windows::core::Interface::as_raw(self), pszframesrc.into_param().abi()).ok()
    }
    pub unsafe fn GetFrameSrc(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFrameSrc)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> ::windows::core::Result<super::super::System::Ole::IOleContainer> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Ole::IOleContainer>();
        (::windows::core::Interface::vtable(self).GetFramesContainer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFrameOptions(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFrameOptions)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFrameOptions(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetFrameOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFrameMargins)(::windows::core::Interface::as_raw(self), dwwidth, dwheight).ok()
    }
    pub unsafe fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameMargins)(::windows::core::Interface::as_raw(self), pdwwidth, pdwheight).ok()
    }
    pub unsafe fn FindFrame<P0>(&self, psztargetname: P0, dwflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).FindFrame)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetAlias<P0>(&self, psztargetname: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetTargetAlias)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITargetFrame2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITargetFrame2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetFrame2 {}
impl ::core::fmt::Debug for ITargetFrame2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetFrame2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetFrame2 {
    type Vtable = ITargetFrame2_Vtbl;
}
impl ::core::clone::Clone for ITargetFrame2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetFrame2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86d52e11_94a8_11d0_82af_00c04fd5ae38);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFrame2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFrameName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszframename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetFrameName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszframename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetParentFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFrameSrc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszframesrc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetFrameSrc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
    pub SetFrameOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetFrameOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetFrameMargins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwidth: u32, dwheight: u32) -> ::windows::core::HRESULT,
    pub GetFrameMargins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT,
    pub FindFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTargetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, ppsztargetalias: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetFramePriv(::windows::core::IUnknown);
impl ITargetFramePriv {
    pub unsafe fn FindFrameDownwards<P0>(&self, psztargetname: P0, dwflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).FindFrameDownwards)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFrameInContext<P0, P1>(&self, psztargetname: P0, punkcontextframe: P1, dwflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).FindFrameInContext)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), punkcontextframe.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnChildFrameActivate)(::windows::core::Interface::as_raw(self), punkchildframe.into_param().abi()).ok()
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnChildFrameDeactivate)(::windows::core::Interface::as_raw(self), punkchildframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NavigateHack<P0, P1, P2, P3, P4>(&self, grfhlnf: u32, pbc: P0, pibsc: P1, psztargetname: P2, pszurl: P3, pszlocation: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IBindCtx>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IBindStatusCallback>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).NavigateHack)(::windows::core::Interface::as_raw(self), grfhlnf, pbc.into_param().abi(), pibsc.into_param().abi(), psztargetname.into_param().abi(), pszurl.into_param().abi(), pszlocation.into_param().abi()).ok()
    }
    pub unsafe fn FindBrowserByIndex(&self, dwid: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).FindBrowserByIndex)(::windows::core::Interface::as_raw(self), dwid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITargetFramePriv, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITargetFramePriv {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetFramePriv {}
impl ::core::fmt::Debug for ITargetFramePriv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetFramePriv").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetFramePriv {
    type Vtable = ITargetFramePriv_Vtbl;
}
impl ::core::clone::Clone for ITargetFramePriv {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetFramePriv {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9216e421_2bf5_11d0_82b4_00a0c90c29c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFramePriv_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindFrameDownwards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFrameInContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, punkcontextframe: *mut ::core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnChildFrameActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnChildFrameDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NavigateHack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfhlnf: u32, pbc: *mut ::core::ffi::c_void, pibsc: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, pszurl: ::windows::core::PCWSTR, pszlocation: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NavigateHack: usize,
    pub FindBrowserByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32, ppunkbrowser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetFramePriv2(::windows::core::IUnknown);
impl ITargetFramePriv2 {
    pub unsafe fn FindFrameDownwards<P0>(&self, psztargetname: P0, dwflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.FindFrameDownwards)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindFrameInContext<P0, P1>(&self, psztargetname: P0, punkcontextframe: P1, dwflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.FindFrameInContext)(::windows::core::Interface::as_raw(self), psztargetname.into_param().abi(), punkcontextframe.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.OnChildFrameActivate)(::windows::core::Interface::as_raw(self), punkchildframe.into_param().abi()).ok()
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.OnChildFrameDeactivate)(::windows::core::Interface::as_raw(self), punkchildframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NavigateHack<P0, P1, P2, P3, P4>(&self, grfhlnf: u32, pbc: P0, pibsc: P1, psztargetname: P2, pszurl: P3, pszlocation: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IBindCtx>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IBindStatusCallback>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.NavigateHack)(::windows::core::Interface::as_raw(self), grfhlnf, pbc.into_param().abi(), pibsc.into_param().abi(), psztargetname.into_param().abi(), pszurl.into_param().abi(), pszlocation.into_param().abi()).ok()
    }
    pub unsafe fn FindBrowserByIndex(&self, dwid: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.FindBrowserByIndex)(::windows::core::Interface::as_raw(self), dwid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AggregatedNavigation2<P0, P1, P2, P3, P4>(&self, grfhlnf: u32, pbc: P0, pibsc: P1, psztargetname: P2, puri: P3, pszlocation: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IBindCtx>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IBindStatusCallback>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::System::Com::IUri>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AggregatedNavigation2)(::windows::core::Interface::as_raw(self), grfhlnf, pbc.into_param().abi(), pibsc.into_param().abi(), psztargetname.into_param().abi(), puri.into_param().abi(), pszlocation.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITargetFramePriv2, ::windows::core::IUnknown, ITargetFramePriv);
impl ::core::cmp::PartialEq for ITargetFramePriv2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetFramePriv2 {}
impl ::core::fmt::Debug for ITargetFramePriv2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetFramePriv2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetFramePriv2 {
    type Vtable = ITargetFramePriv2_Vtbl;
}
impl ::core::clone::Clone for ITargetFramePriv2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetFramePriv2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2c867e6_69d6_46f2_a611_ded9a4bd7fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFramePriv2_Vtbl {
    pub base__: ITargetFramePriv_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AggregatedNavigation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfhlnf: u32, pbc: *mut ::core::ffi::c_void, pibsc: *mut ::core::ffi::c_void, psztargetname: ::windows::core::PCWSTR, puri: *mut ::core::ffi::c_void, pszlocation: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AggregatedNavigation2: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetNotify(::windows::core::IUnknown);
impl ITargetNotify {
    pub unsafe fn OnCreate<P0>(&self, punkdestination: P0, cbcookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnCreate)(::windows::core::Interface::as_raw(self), punkdestination.into_param().abi(), cbcookie).ok()
    }
    pub unsafe fn OnReuse<P0>(&self, punkdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnReuse)(::windows::core::Interface::as_raw(self), punkdestination.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITargetNotify, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITargetNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetNotify {}
impl ::core::fmt::Debug for ITargetNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetNotify {
    type Vtable = ITargetNotify_Vtbl;
}
impl ::core::clone::Clone for ITargetNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x863a99a0_21bc_11d0_82b4_00a0c90c29c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdestination: *mut ::core::ffi::c_void, cbcookie: u32) -> ::windows::core::HRESULT,
    pub OnReuse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITargetNotify2(::windows::core::IUnknown);
impl ITargetNotify2 {
    pub unsafe fn OnCreate<P0>(&self, punkdestination: P0, cbcookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.OnCreate)(::windows::core::Interface::as_raw(self), punkdestination.into_param().abi(), cbcookie).ok()
    }
    pub unsafe fn OnReuse<P0>(&self, punkdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.OnReuse)(::windows::core::Interface::as_raw(self), punkdestination.into_param().abi()).ok()
    }
    pub unsafe fn GetOptionString(&self, pbstroptions: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOptionString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstroptions)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITargetNotify2, ::windows::core::IUnknown, ITargetNotify);
impl ::core::cmp::PartialEq for ITargetNotify2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetNotify2 {}
impl ::core::fmt::Debug for ITargetNotify2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetNotify2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITargetNotify2 {
    type Vtable = ITargetNotify2_Vtbl;
}
impl ::core::clone::Clone for ITargetNotify2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetNotify2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f6b1_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetNotify2_Vtbl {
    pub base__: ITargetNotify_Vtbl,
    pub GetOptionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstroptions: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITimer(::windows::core::IUnknown);
impl ITimer {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Advise<P0>(&self, vtimemin: super::super::System::Com::VARIANT, vtimemax: super::super::System::Com::VARIANT, vtimeinterval: super::super::System::Com::VARIANT, dwflags: u32, ptimersink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<ITimerSink>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vtimemin), ::core::mem::transmute(vtimemax), ::core::mem::transmute(vtimeinterval), dwflags, ptimersink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Freeze<P0>(&self, ffreeze: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Freeze)(::windows::core::Interface::as_raw(self), ffreeze.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITimer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimer {}
impl ::core::fmt::Debug for ITimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITimer {
    type Vtable = ITimer_Vtbl;
}
impl ::core::clone::Clone for ITimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f360_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtimemin: super::super::System::Com::VARIANT, vtimemax: super::super::System::Com::VARIANT, vtimeinterval: super::super::System::Com::VARIANT, dwflags: u32, ptimersink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Freeze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffreeze: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Freeze: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetTime: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITimerEx(::windows::core::IUnknown);
impl ITimerEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Advise<P0>(&self, vtimemin: super::super::System::Com::VARIANT, vtimemax: super::super::System::Com::VARIANT, vtimeinterval: super::super::System::Com::VARIANT, dwflags: u32, ptimersink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<ITimerSink>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.Advise)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vtimemin), ::core::mem::transmute(vtimemax), ::core::mem::transmute(vtimeinterval), dwflags, ptimersink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Unadvise)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Freeze<P0>(&self, ffreeze: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Freeze)(::windows::core::Interface::as_raw(self), ffreeze.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMode(&self, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::windows::core::Interface::as_raw(self), dwmode).ok()
    }
}
::windows::imp::interface_hierarchy!(ITimerEx, ::windows::core::IUnknown, ITimer);
impl ::core::cmp::PartialEq for ITimerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimerEx {}
impl ::core::fmt::Debug for ITimerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimerEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITimerEx {
    type Vtable = ITimerEx_Vtbl;
}
impl ::core::clone::Clone for ITimerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimerEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510414_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimerEx_Vtbl {
    pub base__: ITimer_Vtbl,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITimerService(::windows::core::IUnknown);
impl ITimerService {
    pub unsafe fn CreateTimer<P0>(&self, preferencetimer: P0) -> ::windows::core::Result<ITimer>
    where
        P0: ::windows::core::IntoParam<ITimer>,
    {
        let mut result__ = ::windows::core::zeroed::<ITimer>();
        (::windows::core::Interface::vtable(self).CreateTimer)(::windows::core::Interface::as_raw(self), preferencetimer.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNamedTimer(&self, rguidname: *const ::windows::core::GUID) -> ::windows::core::Result<ITimer> {
        let mut result__ = ::windows::core::zeroed::<ITimer>();
        (::windows::core::Interface::vtable(self).GetNamedTimer)(::windows::core::Interface::as_raw(self), rguidname, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNamedTimerReference<P0>(&self, rguidname: *const ::windows::core::GUID, preferencetimer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITimer>,
    {
        (::windows::core::Interface::vtable(self).SetNamedTimerReference)(::windows::core::Interface::as_raw(self), rguidname, preferencetimer.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITimerService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITimerService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimerService {}
impl ::core::fmt::Debug for ITimerService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimerService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITimerService {
    type Vtable = ITimerService_Vtbl;
}
impl ::core::clone::Clone for ITimerService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimerService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f35f_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimerService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferencetimer: *mut ::core::ffi::c_void, ppnewtimer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidname: *const ::windows::core::GUID, pptimer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedTimerReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidname: *const ::windows::core::GUID, preferencetimer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITimerSink(::windows::core::IUnknown);
impl ITimerSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnTimer(&self, vtimeadvise: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTimer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vtimeadvise)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITimerSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITimerSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimerSink {}
impl ::core::fmt::Debug for ITimerSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimerSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITimerSink {
    type Vtable = ITimerSink_Vtbl;
}
impl ::core::clone::Clone for ITimerSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimerSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f361_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimerSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtimeadvise: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnTimer: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITridentTouchInput(::windows::core::IUnknown);
impl ITridentTouchInput {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPointerMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnPointerMessage)(::windows::core::Interface::as_raw(self), msg, wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITridentTouchInput, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITridentTouchInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITridentTouchInput {}
impl ::core::fmt::Debug for ITridentTouchInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITridentTouchInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITridentTouchInput {
    type Vtable = ITridentTouchInput_Vtbl;
}
impl ::core::clone::Clone for ITridentTouchInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITridentTouchInput {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510850_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITridentTouchInput_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPointerMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfallowmanipulations: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPointerMessage: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct ITridentTouchInputSite(::windows::core::IUnknown);
impl ITridentTouchInputSite {
    #[doc = "*Required features: `\"Win32_Web_MsHtml\"`*"]
    #[cfg(feature = "Win32_Web_MsHtml")]
    pub unsafe fn SetManipulationMode(&self, mstouchaction: super::MsHtml::styleMsTouchAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManipulationMode)(::windows::core::Interface::as_raw(self), mstouchaction).ok()
    }
    pub unsafe fn ZoomToPoint(&self, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ZoomToPoint)(::windows::core::Interface::as_raw(self), x, y).ok()
    }
}
::windows::imp::interface_hierarchy!(ITridentTouchInputSite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITridentTouchInputSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITridentTouchInputSite {}
impl ::core::fmt::Debug for ITridentTouchInputSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITridentTouchInputSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITridentTouchInputSite {
    type Vtable = ITridentTouchInputSite_Vtbl;
}
impl ::core::clone::Clone for ITridentTouchInputSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITridentTouchInputSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510849_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITridentTouchInputSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Web_MsHtml")]
    pub SetManipulationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mstouchaction: super::MsHtml::styleMsTouchAction) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Web_MsHtml"))]
    SetManipulationMode: usize,
    pub ZoomToPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
#[repr(transparent)]
pub struct IUrlHistoryNotify(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Ole")]
impl IUrlHistoryNotify {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn QueryStatus(&self, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut super::super::System::Ole::OLECMD, pcmdtext: *mut super::super::System::Ole::OLECMDTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryStatus)(::windows::core::Interface::as_raw(self), pguidcmdgroup, ccmds, prgcmds, pcmdtext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Exec(&self, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::super::System::Com::VARIANT, pvaout: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Exec)(::windows::core::Interface::as_raw(self), pguidcmdgroup, ncmdid, ncmdexecopt, pvain, pvaout).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
::windows::imp::interface_hierarchy!(IUrlHistoryNotify, ::windows::core::IUnknown, super::super::System::Ole::IOleCommandTarget);
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IUrlHistoryNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IUrlHistoryNotify {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IUrlHistoryNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlHistoryNotify").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Interface for IUrlHistoryNotify {
    type Vtable = IUrlHistoryNotify_Vtbl;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for IUrlHistoryNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::ComInterface for IUrlHistoryNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc40bec1_c493_11d0_831b_00c04fd5ae38);
}
#[cfg(feature = "Win32_System_Ole")]
#[repr(C)]
#[doc(hidden)]
pub struct IUrlHistoryNotify_Vtbl {
    pub base__: super::super::System::Ole::IOleCommandTarget_Vtbl,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IUrlHistoryStg(::windows::core::IUnknown);
impl IUrlHistoryStg {
    pub unsafe fn AddUrl<P0, P1>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddUrl)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), pocstitle.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteUrl<P0>(&self, pocsurl: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteUrl)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryUrl<P0>(&self, pocsurl: P0, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).QueryUrl)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), dwflags, lpstaturl).ok()
    }
    pub unsafe fn BindToObject<P0, T>(&self, pocsurl: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).BindToObject)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumUrls(&self) -> ::windows::core::Result<IEnumSTATURL> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATURL>();
        (::windows::core::Interface::vtable(self).EnumUrls)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IUrlHistoryStg, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IUrlHistoryStg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlHistoryStg {}
impl ::core::fmt::Debug for IUrlHistoryStg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlHistoryStg").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUrlHistoryStg {
    type Vtable = IUrlHistoryStg_Vtbl;
}
impl ::core::clone::Clone for IUrlHistoryStg {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUrlHistoryStg {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c374a41_bae4_11cf_bf7d_00aa006946ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlHistoryStg_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pocsurl: ::windows::core::PCWSTR, pocstitle: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pocsurl: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pocsurl: ::windows::core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryUrl: usize,
    pub BindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pocsurl: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumUrls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IUrlHistoryStg2(::windows::core::IUnknown);
impl IUrlHistoryStg2 {
    pub unsafe fn AddUrl<P0, P1>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddUrl)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), pocstitle.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteUrl<P0>(&self, pocsurl: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteUrl)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryUrl<P0>(&self, pocsurl: P0, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.QueryUrl)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), dwflags, lpstaturl).ok()
    }
    pub unsafe fn BindToObject<P0, T>(&self, pocsurl: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.BindToObject)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumUrls(&self) -> ::windows::core::Result<IEnumSTATURL> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATURL>();
        (::windows::core::Interface::vtable(self).base__.EnumUrls)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn AddUrlAndNotify<P0, P1, P2, P3, P4>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32, fwritehistory: P2, poctnotify: P3, punkisfolder: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::System::Ole::IOleCommandTarget>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).AddUrlAndNotify)(::windows::core::Interface::as_raw(self), pocsurl.into_param().abi(), pocstitle.into_param().abi(), dwflags, fwritehistory.into_param().abi(), poctnotify.into_param().abi(), punkisfolder.into_param().abi()).ok()
    }
    pub unsafe fn ClearHistory(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearHistory)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IUrlHistoryStg2, ::windows::core::IUnknown, IUrlHistoryStg);
impl ::core::cmp::PartialEq for IUrlHistoryStg2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlHistoryStg2 {}
impl ::core::fmt::Debug for IUrlHistoryStg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlHistoryStg2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUrlHistoryStg2 {
    type Vtable = IUrlHistoryStg2_Vtbl;
}
impl ::core::clone::Clone for IUrlHistoryStg2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUrlHistoryStg2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafa0dc11_c313_11d0_831a_00c04fd5ae38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlHistoryStg2_Vtbl {
    pub base__: IUrlHistoryStg_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub AddUrlAndNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pocsurl: ::windows::core::PCWSTR, pocstitle: ::windows::core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: *mut ::core::ffi::c_void, punkisfolder: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    AddUrlAndNotify: usize,
    pub ClearHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IViewObjectPresentFlip(::windows::core::IUnknown);
impl IViewObjectPresentFlip {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifyRender<P0>(&self, frecreatepresenter: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).NotifyRender)(::windows::core::Interface::as_raw(self), frecreatepresenter.into_param().abi()).ok()
    }
    pub unsafe fn RenderObjectToBitmap<P0>(&self, pbitmap: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).RenderObjectToBitmap)(::windows::core::Interface::as_raw(self), pbitmap.into_param().abi()).ok()
    }
    pub unsafe fn RenderObjectToSharedBuffer<P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISurfacePresenterFlipBuffer>,
    {
        (::windows::core::Interface::vtable(self).RenderObjectToSharedBuffer)(::windows::core::Interface::as_raw(self), pbuffer.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IViewObjectPresentFlip, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IViewObjectPresentFlip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObjectPresentFlip {}
impl ::core::fmt::Debug for IViewObjectPresentFlip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObjectPresentFlip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IViewObjectPresentFlip {
    type Vtable = IViewObjectPresentFlip_Vtbl;
}
impl ::core::clone::Clone for IViewObjectPresentFlip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IViewObjectPresentFlip {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510847_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlip_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyRender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frecreatepresenter: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyRender: usize,
    pub RenderObjectToBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RenderObjectToSharedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IViewObjectPresentFlip2(::windows::core::IUnknown);
impl IViewObjectPresentFlip2 {
    pub unsafe fn NotifyLeavingView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyLeavingView)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IViewObjectPresentFlip2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IViewObjectPresentFlip2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObjectPresentFlip2 {}
impl ::core::fmt::Debug for IViewObjectPresentFlip2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObjectPresentFlip2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IViewObjectPresentFlip2 {
    type Vtable = IViewObjectPresentFlip2_Vtbl;
}
impl ::core::clone::Clone for IViewObjectPresentFlip2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IViewObjectPresentFlip2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510856_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlip2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NotifyLeavingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IViewObjectPresentFlipSite(::windows::core::IUnknown);
impl IViewObjectPresentFlipSite {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn CreateSurfacePresenterFlip<P0>(&self, pdevice: P0, width: u32, height: u32, backbuffercount: u32, format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT, mode: super::MsHtml::VIEW_OBJECT_ALPHA_MODE) -> ::windows::core::Result<ISurfacePresenterFlip>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<ISurfacePresenterFlip>();
        (::windows::core::Interface::vtable(self).CreateSurfacePresenterFlip)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), width, height, backbuffercount, format, mode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceLuid(&self) -> ::windows::core::Result<super::super::Foundation::LUID> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LUID>();
        (::windows::core::Interface::vtable(self).GetDeviceLuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnterFullScreen(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnterFullScreen)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExitFullScreen(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExitFullScreen)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullScreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsFullScreen)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoundingRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetBoundingRect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics(&self, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMetrics)(::windows::core::Interface::as_raw(self), ppos, psize, pscalex, pscaley).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullScreenSize(&self) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::SIZE>();
        (::windows::core::Interface::vtable(self).GetFullScreenSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IViewObjectPresentFlipSite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IViewObjectPresentFlipSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObjectPresentFlipSite {}
impl ::core::fmt::Debug for IViewObjectPresentFlipSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObjectPresentFlipSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IViewObjectPresentFlipSite {
    type Vtable = IViewObjectPresentFlipSite_Vtbl;
}
impl ::core::clone::Clone for IViewObjectPresentFlipSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IViewObjectPresentFlipSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30510846_98b5_11cf_bb82_00aa00bdce0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlipSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
    pub CreateSurfacePresenterFlip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, width: u32, height: u32, backbuffercount: u32, format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT, mode: super::MsHtml::VIEW_OBJECT_ALPHA_MODE, ppspflip: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml")))]
    CreateSurfacePresenterFlip: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceLuid: usize,
    pub EnterFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExitFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullScreen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBoundingRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullScreenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullScreenSize: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IViewObjectPresentFlipSite2(::windows::core::IUnknown);
impl IViewObjectPresentFlipSite2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotationForCurrentOutput(&self) -> ::windows::core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION>();
        (::windows::core::Interface::vtable(self).GetRotationForCurrentOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IViewObjectPresentFlipSite2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IViewObjectPresentFlipSite2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObjectPresentFlipSite2 {}
impl ::core::fmt::Debug for IViewObjectPresentFlipSite2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObjectPresentFlipSite2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IViewObjectPresentFlipSite2 {
    type Vtable = IViewObjectPresentFlipSite2_Vtbl;
}
impl ::core::clone::Clone for IViewObjectPresentFlipSite2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IViewObjectPresentFlipSite2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaad0cbf1_e7fd_4f12_8902_c78132a8e01d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlipSite2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotationForCurrentOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdxgirotation: *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotationForCurrentOutput: usize,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IWebBrowserEventsService(::windows::core::IUnknown);
impl IWebBrowserEventsService {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireBeforeNavigate2Event(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).FireBeforeNavigate2Event)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FireNavigateComplete2Event(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireNavigateComplete2Event)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDownloadBeginEvent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireDownloadBeginEvent)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDownloadCompleteEvent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireDownloadCompleteEvent)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDocumentCompleteEvent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireDocumentCompleteEvent)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWebBrowserEventsService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWebBrowserEventsService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebBrowserEventsService {}
impl ::core::fmt::Debug for IWebBrowserEventsService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebBrowserEventsService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebBrowserEventsService {
    type Vtable = IWebBrowserEventsService_Vtbl;
}
impl ::core::clone::Clone for IWebBrowserEventsService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebBrowserEventsService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54a8f188_9ebd_4795_ad16_9b4945119636);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowserEventsService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub FireBeforeNavigate2Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcancel: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireBeforeNavigate2Event: usize,
    pub FireNavigateComplete2Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FireDownloadBeginEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FireDownloadCompleteEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FireDocumentCompleteEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
pub struct IWebBrowserEventsUrlService(::windows::core::IUnknown);
impl IWebBrowserEventsUrlService {
    pub unsafe fn GetUrlForEvents(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetUrlForEvents)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWebBrowserEventsUrlService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWebBrowserEventsUrlService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebBrowserEventsUrlService {}
impl ::core::fmt::Debug for IWebBrowserEventsUrlService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebBrowserEventsUrlService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebBrowserEventsUrlService {
    type Vtable = IWebBrowserEventsUrlService_Vtbl;
}
impl ::core::clone::Clone for IWebBrowserEventsUrlService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebBrowserEventsUrlService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87cc5d04_eafa_4833_9820_8f986530cc00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowserEventsUrlService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUrlForEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Iwfolders(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Iwfolders {
    pub unsafe fn navigate<P0>(&self, bstrurl: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).navigate)(::windows::core::Interface::as_raw(self), bstrurl.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn navigateFrame<P0, P1>(&self, bstrurl: P0, bstrtargetframe: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).navigateFrame)(::windows::core::Interface::as_raw(self), bstrurl.into_param().abi(), bstrtargetframe.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn navigateNoSite<P0, P1, P2>(&self, bstrurl: P0, bstrtargetframe: P1, dwhwnd: u32, pwb: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).navigateNoSite)(::windows::core::Interface::as_raw(self), bstrurl.into_param().abi(), bstrtargetframe.into_param().abi(), dwhwnd, pwb.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Iwfolders, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Iwfolders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Iwfolders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Iwfolders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Iwfolders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Iwfolders {
    type Vtable = Iwfolders_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Iwfolders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Iwfolders {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbae31f98_1b81_11d2_a97a_00c04f8ecb02);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Iwfolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrretval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub navigateFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtargetframe: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrretval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub navigateNoSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtargetframe: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwhwnd: u32, pwb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ADDRESSBAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const AnchorClick: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d5413c_33b9_11d2_95a7_00c04f8ecb02);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CATID_MSOfficeAntiVirus: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ffcc30_d398_11d0_b2ae_00a0c908fa49);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CDeviceRect: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f6d4_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CDownloadBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f5be_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CHeaderFooter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f6cd_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CLayoutRect: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f664_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const COLOR_NO_TRANSPARENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CPersistDataPeer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f487_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CPersistHistory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f4c8_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CPersistShortcut: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f4c6_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CPersistSnapshot: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f4c9_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CPersistUserData: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f48e_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CoDitherToRGB8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa860ce50_3910_11d0_86fc_00a0c913f750);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CoMapMIMEToCLSID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30c3b080_30fb_11d0_b724_00aa006c1a01);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const CoSniffStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a01fda0_30df_11d0_b724_00aa006c1a01);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ACTIVEXFILTERINGENABLED: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDCHANNEL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDDESKTOPCOMPONENT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDFAVORITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDSEARCHPROVIDER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDSERVICE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDSITEMODE: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDTHUMBNAILBUTTONS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDTOFAVORITESBAR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADDTRACKINGPROTECTIONLIST: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ADVANCEERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_AMBIENT_OFFLINEIFNOTCONNECTED: i32 = -5501i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_AMBIENT_SILENT: i32 = -5502i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_AUTOCOMPLETEATTACH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_AUTOCOMPLETESAVEFORM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_AUTOSCAN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_BEFORENAVIGATE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_BEFORENAVIGATE2: u32 = 250u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_BEFORESCRIPTEXECUTE: u32 = 290u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_BRANDIMAGEURI: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_BUILDNEWTABPAGE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CANADVANCEERROR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CANRETREATERROR: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CHANGEDEFAULTBROWSER: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CLEARNOTIFICATION: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CLEARSITEMODEICONOVERLAY: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CLIENTTOHOSTWINDOW: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_COMMANDSTATECHANGE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CONTENTDISCOVERYRESET: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_COUNTVIEWTYPES: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CREATESUBSCRIPTION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CUSTOMIZECLEARTYPE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_CUSTOMIZESETTINGS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DEFAULTSEARCHPROVIDER: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DELETESUBSCRIPTION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DEPTH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DIAGNOSECONNECTION: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DIAGNOSECONNECTIONUILESS: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DOCUMENTCOMPLETE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DOUBLECLICK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DOWNLOADBEGIN: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_DOWNLOADCOMPLETE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUE: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUELARGE: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUESQUARE: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUEWIDE: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ENABLESUGGESTEDSITES: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ENUMOPTIONS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_EXPAND: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_EXPORT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_FAVSELECTIONCHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_FILEDOWNLOAD: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_FLAGS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_FRAMEBEFORENAVIGATE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_FRAMENAVIGATECOMPLETE: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_FRAMENEWWINDOW: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETALWAYSSHOWLOCKSTATE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETCVLISTDATA: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETCVLISTLOCALDATA: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETDETAILSSTATE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETEMIELISTDATA: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETEMIELISTLOCALDATA: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETERRORCHAR: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETERRORCODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETERRORLINE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETERRORMSG: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETERRORURL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETEXPERIMENTALFLAG: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETEXPERIMENTALVALUE: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETNEEDHVSIAUTOLAUNCHFLAG: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETNEEDIEAUTOLAUNCHFLAG: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETOSSKU: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_GETPERERRSTATE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_HASNEEDHVSIAUTOLAUNCHFLAG: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_HASNEEDIEAUTOLAUNCHFLAG: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_IMPORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_IMPORTEXPORTFAVORITES: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_INITIALIZED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_INPRIVATEFILTERINGENABLED: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_INVOKECONTEXTMENU: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISMETAREFERRERAVAILABLE: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISSEARCHMIGRATED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISSEARCHPROVIDERINSTALLED: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISSERVICEINSTALLED: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISSITEMODE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISSITEMODEFIRSTRUN: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ISSUBSCRIBED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_LAUNCHIE: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_LAUNCHINHVSI: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_LAUNCHINTERNETOPTIONS: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_LAUNCHNETWORKCLIENTHELP: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_MODE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_MOVESELECTIONDOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_MOVESELECTIONTO: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_MOVESELECTIONUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NAVIGATEANDFIND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NAVIGATECOMPLETE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NAVIGATECOMPLETE2: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NAVIGATEERROR: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NAVIGATETOSUGGESTEDSITES: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NEWFOLDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NEWPROCESS: u32 = 284u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NEWWINDOW: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NEWWINDOW2: u32 = 251u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NEWWINDOW3: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_NSCOLUMNS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONADDRESSBAR: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONFULLSCREEN: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONMENUBAR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONQUIT: u32 = 253u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONSTATUSBAR: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONTHEATERMODE: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONTOOLBAR: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ONVISIBLE: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_OPENFAVORITESPANE: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_OPENFAVORITESSETTINGS: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PHISHINGENABLED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PINNEDSITESTATE: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PRINTTEMPLATEINSTANTIATION: u32 = 225u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PRINTTEMPLATETEARDOWN: u32 = 226u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PRIVACYIMPACTEDSTATECHANGE: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PROGRESSCHANGE: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PROPERTYCHANGE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_PROVISIONNETWORKS: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_QUIT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_REDIRECTXDOMAINBLOCKED: u32 = 286u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_REFRESHOFFLINEDESKTOP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_REMOVESCHEDULEDTILENOTIFICATION: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_REPORTSAFEURL: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RESETEXPERIMENTALFLAGS: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RESETFIRSTBOOTMODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RESETSAFEMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RESETSORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RETREATERROR: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_ROOT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RUNONCEHASSHOWN: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RUNONCEREQUIREDSETTINGSCOMPLETE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_RUNONCESHOWN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SCHEDULEDTILENOTIFICATION: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SEARCHGUIDEURL: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SELECTEDITEM: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SELECTEDITEMS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SELECTIONCHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETACTIVITIESVISIBLE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETDETAILSSTATE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETEXPERIMENTALFLAG: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETEXPERIMENTALVALUE: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETMSDEFAULTS: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETNEEDHVSIAUTOLAUNCHFLAG: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETNEEDIEAUTOLAUNCHFLAG: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETPERERRSTATE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETPHISHINGFILTERSTATUS: u32 = 282u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETRECENTLYCLOSEDVISIBLE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETROOT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETSECURELOCKICON: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETSITEMODEICONOVERLAY: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETSITEMODEPROPERTIES: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETTHUMBNAILBUTTONS: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SETVIEWTYPE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SHELLUIHELPERLAST: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SHOWBROWSERUI: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SHOWINPRIVATEHELP: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SHOWTABSHELP: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODEACTIVATE: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODEADDBUTTONSTYLE: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODEADDJUMPLISTITEM: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODECLEARBADGE: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODECLEARJUMPLIST: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODECREATEJUMPLIST: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODEREFRESHBADGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODESHOWBUTTONSTYLE: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SITEMODESHOWJUMPLIST: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SKIPRUNONCE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SKIPTABSWELCOME: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SQMENABLED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_STARTBADGEUPDATE: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_STARTPERIODICUPDATE: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_STARTPERIODICUPDATEBATCH: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_STATUSTEXTCHANGE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_STOPBADGEUPDATE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_STOPPERIODICUPDATE: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SUBSCRIPTIONSENABLED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SUGGESTEDSITESENABLED: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_SYNCHRONIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_THIRDPARTYURLBLOCKED: u32 = 285u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_TITLECHANGE: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_TITLEICONCHANGE: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_TRACKINGPROTECTIONENABLED: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_TVFLAGS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_UNSELECTALL: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_UPDATEPAGESTATUS: u32 = 227u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_UPDATETHUMBNAILBUTTON: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_VIEWUPDATE: u32 = 281u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WEBWORKERFINISHED: u32 = 289u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WEBWORKERSTARTED: u32 = 288u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWACTIVATE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWCLOSING: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWMOVE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWREGISTERED: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWRESIZE: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWREVOKED: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWSETHEIGHT: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWSETLEFT: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWSETRESIZABLE: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWSETTOP: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWSETWIDTH: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const DISPID_WINDOWSTATECHANGED: u32 = 283u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const E_SURFACE_DISCARDED: i32 = -2147434493i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const E_SURFACE_NODC: i32 = -2147434492i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const E_SURFACE_NOSURFACE: i32 = -2147434496i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const E_SURFACE_NOTMYDC: i32 = -2147434491i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const E_SURFACE_NOTMYPOINTER: i32 = -2147434494i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const E_SURFACE_UNKNOWN_FORMAT: i32 = -2147434495i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const HomePage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x766bf2ae_d650_11d1_9811_00c04fc31d2e);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const HomePageSetting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x374cede0_873a_4c4f_bc86_bcc8cf5116a3);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_ARG_CLEAR_FORMS_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_ARG_CLEAR_FORMS_ALL_BUT_PASSWORDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_ARG_CLEAR_FORMS_PASSWORDS_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_BEFORENAVIGATE_DOEXTERNALBROWSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_BEFORENAVIGATE_GETIDLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_BEFORENAVIGATE_GETSHELLBROWSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_CLEAR_AUTOCOMPLETE_FOR_FORMS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_GET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_SETID_AUTOCOMPLETE_FOR_FORMS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IECMDID_SET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IEGetProcessModule_PROC_NAME: ::windows::core::PCSTR = ::windows::core::s!("IEGetProcessModule");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IEGetTabWindowExports_PROC_NAME: ::windows::core::PCSTR = ::windows::core::s!("IEGetTabWindowExports");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IEPROCESS_MODULE_NAME: ::windows::core::PCWSTR = ::windows::core::w!("IERtUtil.dll");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IEWebDriverManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90314af2_5250_47b3_89d8_6295fc23bc22);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_MAIL_HKEY: i32 = -2147483647i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_MAIL_KEY: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft\\Internet Explorer\\Mail");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_MAIL_VALUE: ::windows::core::PCWSTR = ::windows::core::w!("Use Outlook Express");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_NEWS_HKEY: i32 = -2147483647i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_NEWS_KEY: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft\\Internet Explorer\\News");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_NEWS_VALUE: ::windows::core::PCWSTR = ::windows::core::w!("Use Outlook Express");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_PRESENT_HKEY: i32 = -2147483646i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IE_USE_OE_PRESENT_KEY: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\app.paths\\msimn.exe");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_EVENT_BEGINBITS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_EVENT_BITSCOMPLETE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_EVENT_PALETTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_EVENT_PROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_EVENT_USEDDRAW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_HINT_BOTTOMUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_HINT_FULLWIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IMGDECODE_HINT_TOPDOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IntelliForms: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x613ab92e_16bf_11d2_bca5_00c04fd929db);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const InternetExplorerManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf4fcc34_067a_4e0a_8352_4a1a5095346e);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const LINKSBAND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MAPMIME_CLSID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MAPMIME_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MAPMIME_DEFAULT_ALWAYS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MAPMIME_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MAX_SEARCH_FORMAT_STRING: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const OpenServiceActivityManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5efd803_50f8_43cd_9ab8_aafc1394c9e0);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const OpenServiceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x098870b6_39ea_480b_b8b5_dd0167c4db59);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const PeerFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f4cf_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTRA_VAL_STARTPAGE: ::windows::core::PCSTR = ::windows::core::s!("Start Page");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_PATH_CURRENT: ::windows::core::PCWSTR = ::windows::core::w!("current");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_PATH_DEFAULT: ::windows::core::PCWSTR = ::windows::core::w!("default");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_PATH_INETCPL_RESTRICTIONS: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Policies\\Microsoft\\Internet Explorer\\Control Panel");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_PATH_MIME_DATABASE: ::windows::core::PCWSTR = ::windows::core::w!("MIME\\Database");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_PATH_REMOTEACCESS: ::windows::core::PCWSTR = ::windows::core::w!("RemoteAccess");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_PATH_REMOTEACESS: ::windows::core::PCWSTR = ::windows::core::w!("RemoteAccess");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_SHIFTQUICKSUFFIX: ::windows::core::PCWSTR = ::windows::core::w!("ShiftQuickCompleteSuffix");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ACCEPT_LANGUAGE: ::windows::core::PCWSTR = ::windows::core::w!("AcceptLanguage");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ACCESSMEDIUM: ::windows::core::PCWSTR = ::windows::core::w!("AccessMedium");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ACCESSTYPE: ::windows::core::PCWSTR = ::windows::core::w!("AccessType");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ALIASTO: ::windows::core::PCWSTR = ::windows::core::w!("AliasForCharset");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ANCHORCOLOR: ::windows::core::PCWSTR = ::windows::core::w!("Anchor Color");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ANCHORCOLORHOVER: ::windows::core::PCWSTR = ::windows::core::w!("Anchor Color Hover");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ANCHORCOLORVISITED: ::windows::core::PCWSTR = ::windows::core::w!("Anchor Color Visited");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ANCHORUNDERLINE: ::windows::core::PCWSTR = ::windows::core::w!("Anchor Underline");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTODETECT: ::windows::core::PCWSTR = ::windows::core::w!("AutoDetect");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTODIALDLLNAME: ::windows::core::PCWSTR = ::windows::core::w!("AutodialDllName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTODIALFCNNAME: ::windows::core::PCWSTR = ::windows::core::w!("AutodialFcnName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTODIAL_MONITORCLASSNAME: ::windows::core::PCWSTR = ::windows::core::w!("MS_AutodialMonitor");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTODIAL_TRYONLYONCE: ::windows::core::PCWSTR = ::windows::core::w!("TryAutodialOnce");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTONAVIGATE: ::windows::core::PCWSTR = ::windows::core::w!("SearchForExtensions");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_AUTOSEARCH: ::windows::core::PCWSTR = ::windows::core::w!("Do404Search");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_BACKBITMAP: ::windows::core::PCWSTR = ::windows::core::w!("BackBitmap");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_BACKGROUNDCOLOR: ::windows::core::PCWSTR = ::windows::core::w!("Background Color");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_BODYCHARSET: ::windows::core::PCWSTR = ::windows::core::w!("BodyCharset");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_BYPASSAUTOCONFIG: ::windows::core::PCWSTR = ::windows::core::w!("BypassAutoconfig");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_CACHEPREFIX: ::windows::core::PCWSTR = ::windows::core::w!("CachePrefix");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_CHECKASSOC: ::windows::core::PCWSTR = ::windows::core::w!("Check_Associations");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_CODEDOWNLOAD: ::windows::core::PCWSTR = ::windows::core::w!("Code Download");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_CODEDOWNLOAD_DEF: ::windows::core::PCWSTR = ::windows::core::w!("yes");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_CODEPAGE: ::windows::core::PCWSTR = ::windows::core::w!("CodePage");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_COVEREXCLUDE: ::windows::core::PCWSTR = ::windows::core::w!("CoverExclude");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DAYSTOKEEP: ::windows::core::PCWSTR = ::windows::core::w!("DaysToKeep");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DEFAULT_CODEPAGE: ::windows::core::PCWSTR = ::windows::core::w!("Default_CodePage");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DEFAULT_SCRIPT: ::windows::core::PCWSTR = ::windows::core::w!("Default_Script");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DEF_ENCODING: ::windows::core::PCWSTR = ::windows::core::w!("Default_Encoding");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DEF_INETENCODING: ::windows::core::PCWSTR = ::windows::core::w!("Default_InternetEncoding");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DESCRIPTION: ::windows::core::PCWSTR = ::windows::core::w!("Description");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DIRECTORY: ::windows::core::PCWSTR = ::windows::core::w!("Directory");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_DISCONNECTIDLETIME: ::windows::core::PCWSTR = ::windows::core::w!("DisconnectIdleTime");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENABLEAUTODIAL: ::windows::core::PCWSTR = ::windows::core::w!("EnableAutodial");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENABLEAUTODIALDISCONNECT: ::windows::core::PCWSTR = ::windows::core::w!("EnableAutodisconnect");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENABLEAUTODISCONNECT: ::windows::core::PCWSTR = ::windows::core::w!("EnableAutodisconnect");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENABLEEXITDISCONNECT: ::windows::core::PCWSTR = ::windows::core::w!("EnableExitDisconnect");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENABLESECURITYCHECK: ::windows::core::PCWSTR = ::windows::core::w!("EnableSecurityCheck");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENABLEUNATTENDED: ::windows::core::PCWSTR = ::windows::core::w!("EnableUnattended");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_ENCODENAME: ::windows::core::PCWSTR = ::windows::core::w!("EncodingName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FAMILY: ::windows::core::PCWSTR = ::windows::core::w!("Family");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FIXEDWIDTHFONT: ::windows::core::PCWSTR = ::windows::core::w!("FixedWidthFont");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FIXED_FONT: ::windows::core::PCWSTR = ::windows::core::w!("IEFixedFontName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FONT_SCRIPT: ::windows::core::PCWSTR = ::windows::core::w!("Script");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FONT_SCRIPTS: ::windows::core::PCWSTR = ::windows::core::w!("Scripts");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FONT_SCRIPT_NAME: ::windows::core::PCWSTR = ::windows::core::w!("Script");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FONT_SIZE: ::windows::core::PCWSTR = ::windows::core::w!("IEFontSize");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_FONT_SIZE_DEF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_HEADERCHARSET: ::windows::core::PCWSTR = ::windows::core::w!("HeaderCharset");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_HTTP_ERRORS: ::windows::core::PCWSTR = ::windows::core::w!("Friendly http errors");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_IE_CUSTOMCOLORS: ::windows::core::PCWSTR = ::windows::core::w!("Custom Colors");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_ADVANCEDTAB: ::windows::core::PCWSTR = ::windows::core::w!("AdvancedTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_CONNECTIONSTAB: ::windows::core::PCWSTR = ::windows::core::w!("ConnectionsTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_CONTENTTAB: ::windows::core::PCWSTR = ::windows::core::w!("ContentTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_GENERALTAB: ::windows::core::PCWSTR = ::windows::core::w!("GeneralTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_IEAK: ::windows::core::PCWSTR = ::windows::core::w!("IEAKContext");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_PRIVACYTAB: ::windows::core::PCWSTR = ::windows::core::w!("PrivacyTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_PROGRAMSTAB: ::windows::core::PCWSTR = ::windows::core::w!("ProgramsTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETCPL_SECURITYTAB: ::windows::core::PCWSTR = ::windows::core::w!("SecurityTab");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INETENCODING: ::windows::core::PCWSTR = ::windows::core::w!("InternetEncoding");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INTERNETENTRY: ::windows::core::PCWSTR = ::windows::core::w!("InternetProfile");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INTERNETENTRYBKUP: ::windows::core::PCWSTR = ::windows::core::w!("BackupInternetProfile");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_INTERNETPROFILE: ::windows::core::PCWSTR = ::windows::core::w!("InternetProfile");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_JAVAJIT: ::windows::core::PCWSTR = ::windows::core::w!("EnableJIT");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_JAVAJIT_DEF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_JAVALOGGING: ::windows::core::PCWSTR = ::windows::core::w!("EnableLogging");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_JAVALOGGING_DEF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_LEVEL: ::windows::core::PCWSTR = ::windows::core::w!("Level");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_LOADIMAGES: ::windows::core::PCWSTR = ::windows::core::w!("Display Inline Images");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_LOCALPAGE: ::windows::core::PCWSTR = ::windows::core::w!("Local Page");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_MOSDISCONNECT: ::windows::core::PCWSTR = ::windows::core::w!("DisconnectTimeout");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_NEWDIRECTORY: ::windows::core::PCWSTR = ::windows::core::w!("NewDirectory");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_NONETAUTODIAL: ::windows::core::PCWSTR = ::windows::core::w!("NoNetAutodial");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PLAYSOUNDS: ::windows::core::PCWSTR = ::windows::core::w!("Play_Background_Sounds");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PLAYVIDEOS: ::windows::core::PCWSTR = ::windows::core::w!("Display Inline Videos");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PRIVCONVERTER: ::windows::core::PCWSTR = ::windows::core::w!("PrivConverter");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PROPORTIONALFONT: ::windows::core::PCWSTR = ::windows::core::w!("ProportionalFont");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PROP_FONT: ::windows::core::PCWSTR = ::windows::core::w!("IEPropFontName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PROXYENABLE: ::windows::core::PCWSTR = ::windows::core::w!("ProxyEnable");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PROXYOVERRIDE: ::windows::core::PCWSTR = ::windows::core::w!("ProxyOverride");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_PROXYSERVER: ::windows::core::PCWSTR = ::windows::core::w!("ProxyServer");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_REDIALATTEMPTS: ::windows::core::PCWSTR = ::windows::core::w!("RedialAttempts");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_REDIALINTERVAL: ::windows::core::PCWSTR = ::windows::core::w!("RedialWait");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_RNAINSTALLED: ::windows::core::PCWSTR = ::windows::core::w!("Installed");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SAFETYWARNINGLEVEL: ::windows::core::PCWSTR = ::windows::core::w!("Safety Warning Level");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SCHANNELENABLEPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("Enabled");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SCHANNELENABLEPROTOCOL_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SCRIPT_FIXED_FONT: ::windows::core::PCWSTR = ::windows::core::w!("IEFixedFontName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SCRIPT_PROP_FONT: ::windows::core::PCWSTR = ::windows::core::w!("IEPropFontName");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SEARCHPAGE: ::windows::core::PCWSTR = ::windows::core::w!("Search Page");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYACTICEXSCRIPTS: ::windows::core::PCWSTR = ::windows::core::w!("Security_RunScripts");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYACTICEXSCRIPTS_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYACTIVEX: ::windows::core::PCWSTR = ::windows::core::w!("Security_RunActiveXControls");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYACTIVEX_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYALLOWCOOKIES: ::windows::core::PCWSTR = ::windows::core::w!("AllowCookies");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYALLOWCOOKIES_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYDISABLECACHINGOFSSLPAGES: ::windows::core::PCWSTR = ::windows::core::w!("DisableCachingOfSSLPages");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYDISABLECACHINGOFSSLPAGES_DEF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYJAVA: ::windows::core::PCWSTR = ::windows::core::w!("Security_RunJavaApplets");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYJAVA_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONBADCERTSENDING: ::windows::core::PCWSTR = ::windows::core::w!("WarnOnBadCertSending");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONBADCERTSENDING_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONBADCERTVIEWING: ::windows::core::PCWSTR = ::windows::core::w!("WarnOnBadCertRecving");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONBADCERTVIEWING_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONSEND: ::windows::core::PCWSTR = ::windows::core::w!("WarnOnPost");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONSENDALWAYS: ::windows::core::PCWSTR = ::windows::core::w!("WarnAlwaysOnPost");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONSENDALWAYS_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONSEND_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONVIEW: ::windows::core::PCWSTR = ::windows::core::w!("WarnOnView");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONVIEW_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONZONECROSSING: ::windows::core::PCWSTR = ::windows::core::w!("WarnOnZoneCrossing");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SECURITYWARNONZONECROSSING_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SHOWADDRESSBAR: ::windows::core::PCWSTR = ::windows::core::w!("Show_URLToolBar");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SHOWFOCUS: ::windows::core::PCWSTR = ::windows::core::w!("Tabstop - MouseDown");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SHOWFOCUS_DEF: ::windows::core::PCWSTR = ::windows::core::w!("no");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SHOWFULLURLS: ::windows::core::PCWSTR = ::windows::core::w!("Show_FullURL");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SHOWTOOLBAR: ::windows::core::PCWSTR = ::windows::core::w!("Show_ToolBar");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SMOOTHSCROLL: ::windows::core::PCWSTR = ::windows::core::w!("SmoothScroll");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_SMOOTHSCROLL_DEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_STARTPAGE: ::windows::core::PCWSTR = ::windows::core::w!("Start Page");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_TEXTCOLOR: ::windows::core::PCWSTR = ::windows::core::w!("Text Color");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_TRUSTWARNINGLEVEL_HIGH: ::windows::core::PCWSTR = ::windows::core::w!("High");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_TRUSTWARNINGLEVEL_LOW: ::windows::core::PCWSTR = ::windows::core::w!("No Security");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_TRUSTWARNINGLEVEL_MED: ::windows::core::PCWSTR = ::windows::core::w!("Medium");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEAUTOAPPEND: ::windows::core::PCWSTR = ::windows::core::w!("Append Completion");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEAUTOCOMPLETE: ::windows::core::PCWSTR = ::windows::core::w!("Use AutoComplete");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEAUTOSUGGEST: ::windows::core::PCWSTR = ::windows::core::w!("AutoSuggest");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEDLGCOLORS: ::windows::core::PCWSTR = ::windows::core::w!("Use_DlgBox_Colors");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEHOVERCOLOR: ::windows::core::PCWSTR = ::windows::core::w!("Use Anchor Hover Color");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEIBAR: ::windows::core::PCWSTR = ::windows::core::w!("UseBar");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEICM: ::windows::core::PCWSTR = ::windows::core::w!("UseICM");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USEICM_DEF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USERAGENT: ::windows::core::PCWSTR = ::windows::core::w!("User Agent");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USESTYLESHEETS: ::windows::core::PCWSTR = ::windows::core::w!("Use Stylesheets");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_USESTYLESHEETS_DEF: ::windows::core::PCWSTR = ::windows::core::w!("yes");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_VISIBLEBANDS: ::windows::core::PCWSTR = ::windows::core::w!("VisibleBands");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_VISIBLEBANDS_DEF: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const REGSTR_VAL_WEBCHARSET: ::windows::core::PCWSTR = ::windows::core::w!("WebCharset");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const STATURLFLAG_ISCACHED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const STATURLFLAG_ISTOPLEVEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const STATURL_QUERYFLAG_ISCACHED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const STATURL_QUERYFLAG_NOTITLE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const STATURL_QUERYFLAG_NOURL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const STATURL_QUERYFLAG_TOPLEVEL: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SURFACE_LOCK_ALLOW_DISCARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SURFACE_LOCK_EXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SURFACE_LOCK_WAIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZBACKBITMAP: ::windows::core::PCSTR = ::windows::core::s!("BackBitmap");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZJAVAVMPATH: ::windows::core::PCSTR = ::windows::core::s!("\\Java VM");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZNOTEXT: ::windows::core::PCSTR = ::windows::core::s!("NoText");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZTOOLBAR: ::windows::core::PCSTR = ::windows::core::s!("\\Toolbar");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZTRUSTWARNLEVEL: ::windows::core::PCSTR = ::windows::core::s!("Trust Warning Level");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZVISIBLE: ::windows::core::PCSTR = ::windows::core::s!("VisibleBands");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_DEFAULT_HTML_EDITOR: ::windows::core::PCSTR = ::windows::core::s!("Default HTML Editor");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_IBAR: ::windows::core::PCSTR = ::windows::core::s!("Bar");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_IBAR_BANDS: ::windows::core::PCSTR = ::windows::core::s!("Bands");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_MAIN: ::windows::core::PCSTR = ::windows::core::s!("Main");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_SEARCHSTRINGS: ::windows::core::PCSTR = ::windows::core::s!("UrlTemplate");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_SECURITY: ::windows::core::PCSTR = ::windows::core::s!("Security");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_SETTINGS: ::windows::core::PCSTR = ::windows::core::s!("Settings");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SZ_IE_THRESHOLDS: ::windows::core::PCSTR = ::windows::core::s!("ErrorThresholds");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const S_SURFACE_DISCARDED: i32 = 49155i32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TARGET_NOTIFY_OBJECT_NAME: ::windows::core::PCWSTR = ::windows::core::w!("863a99a0-21bc-11d0-82b4-00a0c90c29c5");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TF_NAVIGATE: u32 = 2142153644u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TIMERMODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TIMERMODE_VISIBILITYAWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TOOLSBAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZCALENDARPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("unk");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZCALLTOPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("callto");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZINTERNETCLIENTSPATH: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft\\Internet Explorer\\Unix");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZLDAPPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("ldap");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZMAILTOPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("mailto");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZMICROSOFTPATH: ::windows::core::PCWSTR = ::windows::core::w!("Software\\Microsoft");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZNEWSPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("news");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZPROTOCOLSPATH: ::windows::core::PCWSTR = ::windows::core::w!("Protocols\\");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZSCHANNELPATH: ::windows::core::PCWSTR = ::windows::core::w!("SYSTEM\\CurrentControlSet\\Control\\SecurityProviders\\SCHANNEL");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const TSZVSOURCEPROTOCOL: ::windows::core::PCWSTR = ::windows::core::w!("view source");
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msodsvFailed: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msodsvLowSecurityLevel: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msodsvNoMacros: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msodsvPassedTrusted: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msodsvPassedTrustedCert: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msodsvUnsigned: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoedmDisable: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoedmDontOpen: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoedmEnable: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoslHigh: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoslMedium: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoslNone: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const msoslUndefined: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const wfolders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbae31f9a_1b81_11d2_a97a_00c04f8ecb02);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADDURL_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ADDURL_FIRST: ADDURL_FLAG = ADDURL_FLAG(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ADDURL_ADDTOHISTORYANDCACHE: ADDURL_FLAG = ADDURL_FLAG(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ADDURL_ADDTOCACHE: ADDURL_FLAG = ADDURL_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ADDURL_Max: ADDURL_FLAG = ADDURL_FLAG(2147483647i32);
impl ::core::marker::Copy for ADDURL_FLAG {}
impl ::core::clone::Clone for ADDURL_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADDURL_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ADDURL_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ADDURL_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDURL_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExtensionValidationContexts(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ExtensionValidationContextNone: ExtensionValidationContexts = ExtensionValidationContexts(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ExtensionValidationContextDynamic: ExtensionValidationContexts = ExtensionValidationContexts(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ExtensionValidationContextParsed: ExtensionValidationContexts = ExtensionValidationContexts(2i32);
impl ::core::marker::Copy for ExtensionValidationContexts {}
impl ::core::clone::Clone for ExtensionValidationContexts {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtensionValidationContexts {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ExtensionValidationContexts {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ExtensionValidationContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtensionValidationContexts").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExtensionValidationResults(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ExtensionValidationResultNone: ExtensionValidationResults = ExtensionValidationResults(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ExtensionValidationResultDoNotInstantiate: ExtensionValidationResults = ExtensionValidationResults(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ExtensionValidationResultArrestPageLoad: ExtensionValidationResults = ExtensionValidationResults(2i32);
impl ::core::marker::Copy for ExtensionValidationResults {}
impl ::core::clone::Clone for ExtensionValidationResults {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtensionValidationResults {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ExtensionValidationResults {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ExtensionValidationResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtensionValidationResults").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FINDFRAME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FINDFRAME_NONE: FINDFRAME_FLAGS = FINDFRAME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FINDFRAME_JUSTTESTEXISTENCE: FINDFRAME_FLAGS = FINDFRAME_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FINDFRAME_INTERNAL: FINDFRAME_FLAGS = FINDFRAME_FLAGS(-2147483648i32);
impl ::core::marker::Copy for FINDFRAME_FLAGS {}
impl ::core::clone::Clone for FINDFRAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FINDFRAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FINDFRAME_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FINDFRAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDFRAME_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMEOPTIONS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_SCROLL_YES: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_SCROLL_NO: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_SCROLL_AUTO: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_NORESIZE: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_NO3DBORDER: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_DESKTOP: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const FRAMEOPTIONS_BROWSERBAND: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(64i32);
impl ::core::marker::Copy for FRAMEOPTIONS_FLAGS {}
impl ::core::clone::Clone for FRAMEOPTIONS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FRAMEOPTIONS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FRAMEOPTIONS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FRAMEOPTIONS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMEOPTIONS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IELAUNCHOPTION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IELAUNCHOPTION_SCRIPTDEBUG: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IELAUNCHOPTION_FORCE_COMPAT: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IELAUNCHOPTION_FORCE_EDGE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const IELAUNCHOPTION_LOCK_ENGINE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(8i32);
impl ::core::marker::Copy for IELAUNCHOPTION_FLAGS {}
impl ::core::clone::Clone for IELAUNCHOPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IELAUNCHOPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IELAUNCHOPTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IELAUNCHOPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IELAUNCHOPTION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INTERNETEXPLORERCONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const INTERNETEXPLORERCONFIGURATION_HOST: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(2i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER_EDGE: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(4i32);
impl ::core::marker::Copy for INTERNETEXPLORERCONFIGURATION {}
impl ::core::clone::Clone for INTERNETEXPLORERCONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNETEXPLORERCONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INTERNETEXPLORERCONFIGURATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INTERNETEXPLORERCONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNETEXPLORERCONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEDIA_ACTIVITY_NOTIFY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MediaPlayback: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MediaRecording: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const MediaCasting: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(2i32);
impl ::core::marker::Copy for MEDIA_ACTIVITY_NOTIFY_TYPE {}
impl ::core::clone::Clone for MEDIA_ACTIVITY_NOTIFY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIA_ACTIVITY_NOTIFY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MEDIA_ACTIVITY_NOTIFY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MEDIA_ACTIVITY_NOTIFY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_ACTIVITY_NOTIFY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAVIGATEFRAME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_RECORD: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_POST: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_NO_DOC_CACHE: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_NO_IMAGE_CACHE: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_AUTH_FAIL_CACHE_OK: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_SENDING_FROM_FORM: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const NAVIGATEFRAME_FL_REALLY_SENDING_FROM_FORM: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(64i32);
impl ::core::marker::Copy for NAVIGATEFRAME_FLAGS {}
impl ::core::clone::Clone for NAVIGATEFRAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAVIGATEFRAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NAVIGATEFRAME_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NAVIGATEFRAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAVIGATEFRAME_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OpenServiceActivityContentType(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ActivityContentNone: OpenServiceActivityContentType = OpenServiceActivityContentType(-1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ActivityContentDocument: OpenServiceActivityContentType = OpenServiceActivityContentType(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ActivityContentSelection: OpenServiceActivityContentType = OpenServiceActivityContentType(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ActivityContentLink: OpenServiceActivityContentType = OpenServiceActivityContentType(2i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const ActivityContentCount: OpenServiceActivityContentType = OpenServiceActivityContentType(3i32);
impl ::core::marker::Copy for OpenServiceActivityContentType {}
impl ::core::clone::Clone for OpenServiceActivityContentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OpenServiceActivityContentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OpenServiceActivityContentType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OpenServiceActivityContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpenServiceActivityContentType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OpenServiceErrors(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const OS_E_NOTFOUND: OpenServiceErrors = OpenServiceErrors(-2147287038i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const OS_E_NOTSUPPORTED: OpenServiceErrors = OpenServiceErrors(-2147467231i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const OS_E_CANCELLED: OpenServiceErrors = OpenServiceErrors(-2147471631i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const OS_E_GPDISABLED: OpenServiceErrors = OpenServiceErrors(-1072886820i32);
impl ::core::marker::Copy for OpenServiceErrors {}
impl ::core::clone::Clone for OpenServiceErrors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OpenServiceErrors {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OpenServiceErrors {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OpenServiceErrors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpenServiceErrors").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCROLLABLECONTEXTMENU_PLACEMENT(pub i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SCMP_TOP: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(0i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SCMP_BOTTOM: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(1i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SCMP_LEFT: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(2i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SCMP_RIGHT: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(3i32);
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub const SCMP_FULL: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(4i32);
impl ::core::marker::Copy for SCROLLABLECONTEXTMENU_PLACEMENT {}
impl ::core::clone::Clone for SCROLLABLECONTEXTMENU_PLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCROLLABLECONTEXTMENU_PLACEMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCROLLABLECONTEXTMENU_PLACEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCROLLABLECONTEXTMENU_PLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLABLECONTEXTMENU_PLACEMENT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct HTMLPersistEvents(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub struct IELAUNCHURLINFO {
    pub cbSize: u32,
    pub dwCreationFlags: u32,
    pub dwLaunchOptionFlags: u32,
}
impl ::core::marker::Copy for IELAUNCHURLINFO {}
impl ::core::clone::Clone for IELAUNCHURLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IELAUNCHURLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IELAUNCHURLINFO").field("cbSize", &self.cbSize).field("dwCreationFlags", &self.dwCreationFlags).field("dwLaunchOptionFlags", &self.dwLaunchOptionFlags).finish()
    }
}
impl ::windows::core::TypeKind for IELAUNCHURLINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IELAUNCHURLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwCreationFlags == other.dwCreationFlags && self.dwLaunchOptionFlags == other.dwLaunchOptionFlags
    }
}
impl ::core::cmp::Eq for IELAUNCHURLINFO {}
impl ::core::default::Default for IELAUNCHURLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LayoutRectEvents(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`*"]
pub struct NAVIGATEDATA {
    pub ulTarget: u32,
    pub ulURL: u32,
    pub ulRefURL: u32,
    pub ulPostData: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NAVIGATEDATA {}
impl ::core::clone::Clone for NAVIGATEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAVIGATEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAVIGATEDATA").field("ulTarget", &self.ulTarget).field("ulURL", &self.ulURL).field("ulRefURL", &self.ulRefURL).field("ulPostData", &self.ulPostData).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for NAVIGATEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NAVIGATEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.ulTarget == other.ulTarget && self.ulURL == other.ulURL && self.ulRefURL == other.ulRefURL && self.ulPostData == other.ulPostData && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for NAVIGATEDATA {}
impl ::core::default::Default for NAVIGATEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Web_InternetExplorer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATURL {
    pub cbSize: u32,
    pub pwcsUrl: ::windows::core::PWSTR,
    pub pwcsTitle: ::windows::core::PWSTR,
    pub ftLastVisited: super::super::Foundation::FILETIME,
    pub ftLastUpdated: super::super::Foundation::FILETIME,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATURL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATURL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATURL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATURL").field("cbSize", &self.cbSize).field("pwcsUrl", &self.pwcsUrl).field("pwcsTitle", &self.pwcsTitle).field("ftLastVisited", &self.ftLastVisited).field("ftLastUpdated", &self.ftLastUpdated).field("ftExpires", &self.ftExpires).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STATURL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATURL {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pwcsUrl == other.pwcsUrl && self.pwcsTitle == other.pwcsTitle && self.ftLastVisited == other.ftLastVisited && self.ftLastUpdated == other.ftLastUpdated && self.ftExpires == other.ftExpires && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATURL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATURL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
