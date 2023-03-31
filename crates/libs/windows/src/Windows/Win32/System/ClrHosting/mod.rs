#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CLRCreateInstance(clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn CLRCreateInstance ( clsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppinterface : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CLRCreateInstance(clsid, riid, ppinterface).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CallFunctionShim<P0, P1, P2>(szdllname: P0, szfunctionname: P1, lpvargument1: *mut ::core::ffi::c_void, lpvargument2: *mut ::core::ffi::c_void, szversion: P2, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CallFunctionShim ( szdllname : ::windows::core::PCWSTR , szfunctionname : ::windows::core::PCSTR , lpvargument1 : *mut ::core::ffi::c_void , lpvargument2 : *mut ::core::ffi::c_void , szversion : ::windows::core::PCWSTR , pvreserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CallFunctionShim(szdllname.into_param().abi(), szfunctionname.into_param().abi(), lpvargument1, lpvargument2, szversion.into_param().abi(), pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn ClrCreateManagedInstance<P0>(ptypename: P0, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn ClrCreateManagedInstance ( ptypename : ::windows::core::PCWSTR , riid : *const ::windows::core::GUID , ppobject : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    ClrCreateManagedInstance(ptypename.into_param().abi(), riid, ppobject).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CorBindToCurrentRuntime<P0>(pwszfilename: P0, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorBindToCurrentRuntime ( pwszfilename : ::windows::core::PCWSTR , rclsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CorBindToCurrentRuntime(pwszfilename.into_param().abi(), rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CorBindToRuntime<P0, P1>(pwszversion: P0, pwszbuildflavor: P1, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorBindToRuntime ( pwszversion : ::windows::core::PCWSTR , pwszbuildflavor : ::windows::core::PCWSTR , rclsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CorBindToRuntime(pwszversion.into_param().abi(), pwszbuildflavor.into_param().abi(), rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CorBindToRuntimeByCfg<P0>(pcfgstream: P0, reserved: u32, startupflags: u32, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::Com::IStream>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorBindToRuntimeByCfg ( pcfgstream : * mut::core::ffi::c_void , reserved : u32 , startupflags : u32 , rclsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CorBindToRuntimeByCfg(pcfgstream.into_param().abi(), reserved, startupflags, rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CorBindToRuntimeEx<P0, P1>(pwszversion: P0, pwszbuildflavor: P1, startupflags: u32, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorBindToRuntimeEx ( pwszversion : ::windows::core::PCWSTR , pwszbuildflavor : ::windows::core::PCWSTR , startupflags : u32 , rclsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CorBindToRuntimeEx(pwszversion.into_param().abi(), pwszbuildflavor.into_param().abi(), startupflags, rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CorBindToRuntimeHost<P0, P1, P2>(pwszversion: P0, pwszbuildflavor: P1, pwszhostconfigfile: P2, preserved: *mut ::core::ffi::c_void, startupflags: u32, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorBindToRuntimeHost ( pwszversion : ::windows::core::PCWSTR , pwszbuildflavor : ::windows::core::PCWSTR , pwszhostconfigfile : ::windows::core::PCWSTR , preserved : *mut ::core::ffi::c_void , startupflags : u32 , rclsid : *const ::windows::core::GUID , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CorBindToRuntimeHost(pwszversion.into_param().abi(), pwszbuildflavor.into_param().abi(), pwszhostconfigfile.into_param().abi(), preserved, startupflags, rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CorExitProcess(exitcode: i32) {
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorExitProcess ( exitcode : i32 ) -> ( ) );
    CorExitProcess(exitcode)
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn CorLaunchApplication<P0>(dwclickoncehost: HOST_TYPE, pwzappfullname: P0, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows::core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows::core::PCWSTR, lpprocessinformation: *mut super::Threading::PROCESS_INFORMATION) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorLaunchApplication ( dwclickoncehost : HOST_TYPE , pwzappfullname : ::windows::core::PCWSTR , dwmanifestpaths : u32 , ppwzmanifestpaths : *const ::windows::core::PCWSTR , dwactivationdata : u32 , ppwzactivationdata : *const ::windows::core::PCWSTR , lpprocessinformation : *mut super::Threading:: PROCESS_INFORMATION ) -> ::windows::core::HRESULT );
    CorLaunchApplication(dwclickoncehost, pwzappfullname.into_param().abi(), dwmanifestpaths, ppwzmanifestpaths, dwactivationdata, ppwzactivationdata, lpprocessinformation).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CorMarkThreadInThreadPool() {
    ::windows_targets::link ! ( "mscoree.dll""system" fn CorMarkThreadInThreadPool ( ) -> ( ) );
    CorMarkThreadInThreadPool()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn CreateDebuggingInterfaceFromVersion<P0>(idebuggerversion: i32, szdebuggeeversion: P0) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn CreateDebuggingInterfaceFromVersion ( idebuggerversion : i32 , szdebuggeeversion : ::windows::core::PCWSTR , ppcordb : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
    CreateDebuggingInterfaceFromVersion(idebuggerversion, szdebuggeeversion.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetCLRIdentityManager(riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetCLRIdentityManager ( riid : *const ::windows::core::GUID , ppmanager : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
    GetCLRIdentityManager(riid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetCORRequiredVersion(pbuffer: &mut [u16], dwlength: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetCORRequiredVersion ( pbuffer : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetCORRequiredVersion(::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, dwlength).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetCORSystemDirectory(pbuffer: &mut [u16], dwlength: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetCORSystemDirectory ( pbuffer : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetCORSystemDirectory(::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, dwlength).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetCORVersion(pbbuffer: &mut [u16], dwlength: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetCORVersion ( pbbuffer : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetCORVersion(::core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len() as _, dwlength).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetFileVersion<P0>(szfilename: P0, szbuffer: ::core::option::Option<&mut [u16]>, dwlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetFileVersion ( szfilename : ::windows::core::PCWSTR , szbuffer : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetFileVersion(szfilename.into_param().abi(), ::core::mem::transmute(szbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), szbuffer.as_deref().map_or(0, |slice| slice.len() as _), dwlength).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetRealProcAddress<P0>(pwszprocname: P0, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetRealProcAddress ( pwszprocname : ::windows::core::PCSTR , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    GetRealProcAddress(pwszprocname.into_param().abi(), ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetRequestedRuntimeInfo<P0, P1, P2>(pexe: P0, pwszversion: P1, pconfigurationfile: P2, startupflags: u32, runtimeinfoflags: u32, pdirectory: ::core::option::Option<&mut [u16]>, dwdirectorylength: ::core::option::Option<*mut u32>, pversion: ::core::option::Option<&mut [u16]>, dwlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetRequestedRuntimeInfo ( pexe : ::windows::core::PCWSTR , pwszversion : ::windows::core::PCWSTR , pconfigurationfile : ::windows::core::PCWSTR , startupflags : u32 , runtimeinfoflags : u32 , pdirectory : ::windows::core::PWSTR , dwdirectory : u32 , dwdirectorylength : *mut u32 , pversion : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetRequestedRuntimeInfo(
        pexe.into_param().abi(),
        pwszversion.into_param().abi(),
        pconfigurationfile.into_param().abi(),
        startupflags,
        runtimeinfoflags,
        ::core::mem::transmute(pdirectory.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pdirectory.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(dwdirectorylength.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pversion.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pversion.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(dwlength.unwrap_or(::std::ptr::null_mut())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetRequestedRuntimeVersion<P0>(pexe: P0, pversion: &mut [u16], dwlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetRequestedRuntimeVersion ( pexe : ::windows::core::PCWSTR , pversion : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetRequestedRuntimeVersion(pexe.into_param().abi(), ::core::mem::transmute(pversion.as_ptr()), pversion.len() as _, dwlength).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn GetRequestedRuntimeVersionForCLSID(rclsid: *const ::windows::core::GUID, pversion: ::core::option::Option<&mut [u16]>, dwlength: ::core::option::Option<*mut u32>, dwresolutionflags: CLSID_RESOLUTION_FLAGS) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetRequestedRuntimeVersionForCLSID ( rclsid : *const ::windows::core::GUID , pversion : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 , dwresolutionflags : CLSID_RESOLUTION_FLAGS ) -> ::windows::core::HRESULT );
    GetRequestedRuntimeVersionForCLSID(rclsid, ::core::mem::transmute(pversion.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pversion.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(dwlength.unwrap_or(::std::ptr::null_mut())), dwresolutionflags).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromProcess<P0>(hprocess: P0, pversion: &mut [u16], dwlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn GetVersionFromProcess ( hprocess : super::super::Foundation:: HANDLE , pversion : ::windows::core::PWSTR , cchbuffer : u32 , dwlength : *mut u32 ) -> ::windows::core::HRESULT );
    GetVersionFromProcess(hprocess.into_param().abi(), ::core::mem::transmute(pversion.as_ptr()), pversion.len() as _, dwlength).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryShim<P0, P1>(szdllname: P0, szversion: P1, pvreserved: *mut ::core::ffi::c_void, phmoddll: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn LoadLibraryShim ( szdllname : ::windows::core::PCWSTR , szversion : ::windows::core::PCWSTR , pvreserved : *mut ::core::ffi::c_void , phmoddll : *mut super::super::Foundation:: HMODULE ) -> ::windows::core::HRESULT );
    LoadLibraryShim(szdllname.into_param().abi(), szversion.into_param().abi(), pvreserved, phmoddll).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn LoadStringRC(iresouceid: u32, szbuffer: &mut [u16], bquiet: i32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn LoadStringRC ( iresouceid : u32 , szbuffer : ::windows::core::PWSTR , imax : i32 , bquiet : i32 ) -> ::windows::core::HRESULT );
    LoadStringRC(iresouceid, ::core::mem::transmute(szbuffer.as_ptr()), szbuffer.len() as _, bquiet).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn LoadStringRCEx(lcid: u32, iresouceid: u32, szbuffer: &mut [u16], bquiet: i32, pcwchused: *mut i32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn LoadStringRCEx ( lcid : u32 , iresouceid : u32 , szbuffer : ::windows::core::PWSTR , imax : i32 , bquiet : i32 , pcwchused : *mut i32 ) -> ::windows::core::HRESULT );
    LoadStringRCEx(lcid, iresouceid, ::core::mem::transmute(szbuffer.as_ptr()), szbuffer.len() as _, bquiet, pcwchused).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[inline]
pub unsafe fn LockClrVersion(hostcallback: FLockClrVersionCallback, pbeginhostsetup: *mut FLockClrVersionCallback, pendhostsetup: *mut FLockClrVersionCallback) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "mscoree.dll""system" fn LockClrVersion ( hostcallback : FLockClrVersionCallback , pbeginhostsetup : *mut FLockClrVersionCallback , pendhostsetup : *mut FLockClrVersionCallback ) -> ::windows::core::HRESULT );
    LockClrVersion(hostcallback, pbeginhostsetup, pendhostsetup).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunDll32ShimW<P0, P1, P2>(hwnd: P0, hinst: P1, lpszcmdline: P2, ncmdshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "mscoree.dll""system" fn RunDll32ShimW ( hwnd : super::super::Foundation:: HWND , hinst : super::super::Foundation:: HMODULE , lpszcmdline : ::windows::core::PCWSTR , ncmdshow : i32 ) -> ::windows::core::HRESULT );
    RunDll32ShimW(hwnd.into_param().abi(), hinst.into_param().abi(), lpszcmdline.into_param().abi(), ncmdshow).ok()
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IActionOnCLREvent(::windows::core::IUnknown);
impl IActionOnCLREvent {
    pub unsafe fn OnEvent(&self, event: EClrEvent, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEvent)(::windows::core::Interface::as_raw(self), event, data).ok()
    }
}
::windows::imp::interface_hierarchy!(IActionOnCLREvent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActionOnCLREvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActionOnCLREvent {}
impl ::core::fmt::Debug for IActionOnCLREvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActionOnCLREvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActionOnCLREvent {
    type Vtable = IActionOnCLREvent_Vtbl;
}
impl ::core::clone::Clone for IActionOnCLREvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActionOnCLREvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x607be24b_d91b_4e28_a242_61871ce56e35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionOnCLREvent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: EClrEvent, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IApartmentCallback(::windows::core::IUnknown);
impl IApartmentCallback {
    pub unsafe fn DoCallback(&self, pfunc: usize, pdata: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoCallback)(::windows::core::Interface::as_raw(self), pfunc, pdata).ok()
    }
}
::windows::imp::interface_hierarchy!(IApartmentCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IApartmentCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApartmentCallback {}
impl ::core::fmt::Debug for IApartmentCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApartmentCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApartmentCallback {
    type Vtable = IApartmentCallback_Vtbl;
}
impl ::core::clone::Clone for IApartmentCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApartmentCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178e5337_1528_4591_b1c9_1c6e484686d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DoCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunc: usize, pdata: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IAppDomainBinding(::windows::core::IUnknown);
impl IAppDomainBinding {
    pub unsafe fn OnAppDomain<P0>(&self, pappdomain: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnAppDomain)(::windows::core::Interface::as_raw(self), pappdomain.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAppDomainBinding, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAppDomainBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppDomainBinding {}
impl ::core::fmt::Debug for IAppDomainBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppDomainBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAppDomainBinding {
    type Vtable = IAppDomainBinding_Vtbl;
}
impl ::core::clone::Clone for IAppDomainBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppDomainBinding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c2b07a7_1e98_11d3_872f_00c04f79ed0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDomainBinding_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnAppDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRAppDomainResourceMonitor(::windows::core::IUnknown);
impl ICLRAppDomainResourceMonitor {
    pub unsafe fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentAllocated)(::windows::core::Interface::as_raw(self), dwappdomainid, pbytesallocated).ok()
    }
    pub unsafe fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentSurvived)(::windows::core::Interface::as_raw(self), dwappdomainid, pappdomainbytessurvived, ptotalbytessurvived).ok()
    }
    pub unsafe fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentCpuTime)(::windows::core::Interface::as_raw(self), dwappdomainid, pmilliseconds).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRAppDomainResourceMonitor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRAppDomainResourceMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRAppDomainResourceMonitor {}
impl ::core::fmt::Debug for ICLRAppDomainResourceMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRAppDomainResourceMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRAppDomainResourceMonitor {
    type Vtable = ICLRAppDomainResourceMonitor_Vtbl;
}
impl ::core::clone::Clone for ICLRAppDomainResourceMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRAppDomainResourceMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc62de18c_2e23_4aea_8423_b40c1fc59eae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRAppDomainResourceMonitor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentAllocated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows::core::HRESULT,
    pub GetCurrentSurvived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows::core::HRESULT,
    pub GetCurrentCpuTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRAssemblyIdentityManager(::windows::core::IUnknown);
impl ICLRAssemblyIdentityManager {
    pub unsafe fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const ::windows::core::PCWSTR, dwnumofreferences: u32) -> ::windows::core::Result<ICLRAssemblyReferenceList> {
        let mut result__ = ::windows::core::zeroed::<ICLRAssemblyReferenceList>();
        (::windows::core::Interface::vtable(self).GetCLRAssemblyReferenceList)(::windows::core::Interface::as_raw(self), ppwzassemblyreferences, dwnumofreferences, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBindingIdentityFromFile<P0>(&self, pwzfilepath: P0, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetBindingIdentityFromFile)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), dwflags, ::core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBindingIdentityFromStream<P0>(&self, pstream: P0, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).GetBindingIdentityFromStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), dwflags, ::core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
    pub unsafe fn GetReferencedAssembliesFromFile<P0, P1>(&self, pwzfilepath: P0, dwflags: u32, pexcludeassemblieslist: P1) -> ::windows::core::Result<ICLRReferenceAssemblyEnum>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ICLRAssemblyReferenceList>,
    {
        let mut result__ = ::windows::core::zeroed::<ICLRReferenceAssemblyEnum>();
        (::windows::core::Interface::vtable(self).GetReferencedAssembliesFromFile)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), dwflags, pexcludeassemblieslist.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReferencedAssembliesFromStream<P0, P1>(&self, pstream: P0, dwflags: u32, pexcludeassemblieslist: P1) -> ::windows::core::Result<ICLRReferenceAssemblyEnum>
    where
        P0: ::windows::core::IntoParam<super::Com::IStream>,
        P1: ::windows::core::IntoParam<ICLRAssemblyReferenceList>,
    {
        let mut result__ = ::windows::core::zeroed::<ICLRReferenceAssemblyEnum>();
        (::windows::core::Interface::vtable(self).GetReferencedAssembliesFromStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), dwflags, pexcludeassemblieslist.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProbingAssembliesFromReference<P0>(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: P0) -> ::windows::core::Result<ICLRProbingAssemblyEnum>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ICLRProbingAssemblyEnum>();
        (::windows::core::Interface::vtable(self).GetProbingAssembliesFromReference)(::windows::core::Interface::as_raw(self), dwmachinetype, dwflags, pwzreferenceidentity.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStronglyNamed<P0>(&self, pwzassemblyidentity: P0, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsStronglyNamed)(::windows::core::Interface::as_raw(self), pwzassemblyidentity.into_param().abi(), pbisstronglynamed).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRAssemblyIdentityManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRAssemblyIdentityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRAssemblyIdentityManager {}
impl ::core::fmt::Debug for ICLRAssemblyIdentityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRAssemblyIdentityManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRAssemblyIdentityManager {
    type Vtable = ICLRAssemblyIdentityManager_Vtbl;
}
impl ::core::clone::Clone for ICLRAssemblyIdentityManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRAssemblyIdentityManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15f0a9da_3ff6_4393_9da9_fdfd284e6972);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRAssemblyIdentityManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCLRAssemblyReferenceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwzassemblyreferences: *const ::windows::core::PCWSTR, dwnumofreferences: u32, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBindingIdentityFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBindingIdentityFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBindingIdentityFromStream: usize,
    pub GetReferencedAssembliesFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetReferencedAssembliesFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetReferencedAssembliesFromStream: usize,
    pub GetProbingAssembliesFromReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: ::windows::core::PCWSTR, ppprobingassemblyenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStronglyNamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzassemblyidentity: ::windows::core::PCWSTR, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStronglyNamed: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRAssemblyReferenceList(::windows::core::IUnknown);
impl ICLRAssemblyReferenceList {
    pub unsafe fn IsStringAssemblyReferenceInList<P0>(&self, pwzassemblyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsStringAssemblyReferenceInList)(::windows::core::Interface::as_raw(self), pwzassemblyname.into_param().abi()).ok()
    }
    pub unsafe fn IsAssemblyReferenceInList<P0>(&self, pname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).IsAssemblyReferenceInList)(::windows::core::Interface::as_raw(self), pname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRAssemblyReferenceList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRAssemblyReferenceList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRAssemblyReferenceList {}
impl ::core::fmt::Debug for ICLRAssemblyReferenceList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRAssemblyReferenceList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRAssemblyReferenceList {
    type Vtable = ICLRAssemblyReferenceList_Vtbl;
}
impl ::core::clone::Clone for ICLRAssemblyReferenceList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRAssemblyReferenceList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b2c9750_2e66_4bda_8b44_0a642c5cd733);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRAssemblyReferenceList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsStringAssemblyReferenceInList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzassemblyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub IsAssemblyReferenceInList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRControl(::windows::core::IUnknown);
impl ICLRControl {
    pub unsafe fn GetCLRManager(&self, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCLRManager)(::windows::core::Interface::as_raw(self), riid, ppobject).ok()
    }
    pub unsafe fn SetAppDomainManagerType<P0, P1>(&self, pwzappdomainmanagerassembly: P0, pwzappdomainmanagertype: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAppDomainManagerType)(::windows::core::Interface::as_raw(self), pwzappdomainmanagerassembly.into_param().abi(), pwzappdomainmanagertype.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRControl {}
impl ::core::fmt::Debug for ICLRControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRControl {
    type Vtable = ICLRControl_Vtbl;
}
impl ::core::clone::Clone for ICLRControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9065597e_d1a1_4fb2_b6ba_7e1fce230f61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCLRManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAppDomainManagerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzappdomainmanagerassembly: ::windows::core::PCWSTR, pwzappdomainmanagertype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRDebugManager(::windows::core::IUnknown);
impl ICLRDebugManager {
    pub unsafe fn BeginConnection<P0>(&self, dwconnectionid: u32, szconnectionname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).BeginConnection)(::windows::core::Interface::as_raw(self), dwconnectionid, szconnectionname.into_param().abi()).ok()
    }
    pub unsafe fn SetConnectionTasks(&self, id: u32, dwcount: u32) -> ::windows::core::Result<ICLRTask> {
        let mut result__ = ::windows::core::zeroed::<ICLRTask>();
        (::windows::core::Interface::vtable(self).SetConnectionTasks)(::windows::core::Interface::as_raw(self), id, dwcount, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndConnection(&self, dwconnectionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndConnection)(::windows::core::Interface::as_raw(self), dwconnectionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn SetDacl(&self, pacl: *mut super::super::Security::ACL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDacl)(::windows::core::Interface::as_raw(self), pacl).ok()
    }
    #[doc = "*Required features: `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDacl(&self, pacl: *mut *mut super::super::Security::ACL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDacl)(::windows::core::Interface::as_raw(self), pacl).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDebuggerAttached(&self, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDebuggerAttached)(::windows::core::Interface::as_raw(self), pbattached).ok()
    }
    pub unsafe fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSymbolReadingPolicy)(::windows::core::Interface::as_raw(self), policy).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRDebugManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRDebugManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRDebugManager {}
impl ::core::fmt::Debug for ICLRDebugManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRDebugManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRDebugManager {
    type Vtable = ICLRDebugManager_Vtbl;
}
impl ::core::clone::Clone for ICLRDebugManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRDebugManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00dcaec6_2ac0_43a9_acf9_1e36c139b10d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDebugManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionid: u32, szconnectionname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetConnectionTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u32, dwcount: u32, ppclrtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub SetDacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacl: *mut super::super::Security::ACL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    SetDacl: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetDacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacl: *mut *mut super::super::Security::ACL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetDacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDebuggerAttached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDebuggerAttached: usize,
    pub SetSymbolReadingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: ESymbolReadingPolicy) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRDebugging(::windows::core::IUnknown);
impl ICLRDebugging {
    pub unsafe fn OpenVirtualProcess<P0, P1>(&self, modulebaseaddress: u64, pdatatarget: P0, plibraryprovider: P1, pmaxdebuggersupportedversion: *mut CLR_DEBUGGING_VERSION, riidprocess: *const ::windows::core::GUID, ppprocess: *mut ::core::option::Option<::windows::core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<ICLRDebuggingLibraryProvider>,
    {
        (::windows::core::Interface::vtable(self).OpenVirtualProcess)(::windows::core::Interface::as_raw(self), modulebaseaddress, pdatatarget.into_param().abi(), plibraryprovider.into_param().abi(), pmaxdebuggersupportedversion, riidprocess, ::core::mem::transmute(ppprocess), pversion, pdwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanUnloadNow<P0>(&self, hmodule: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    {
        (::windows::core::Interface::vtable(self).CanUnloadNow)(::windows::core::Interface::as_raw(self), hmodule.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRDebugging, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRDebugging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRDebugging {}
impl ::core::fmt::Debug for ICLRDebugging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRDebugging").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRDebugging {
    type Vtable = ICLRDebugging_Vtbl;
}
impl ::core::clone::Clone for ICLRDebugging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRDebugging {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd28f3c5a_9634_4206_a509_477552eefb10);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDebugging_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenVirtualProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modulebaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, plibraryprovider: *mut ::core::ffi::c_void, pmaxdebuggersupportedversion: *mut CLR_DEBUGGING_VERSION, riidprocess: *const ::windows::core::GUID, ppprocess: *mut *mut ::core::ffi::c_void, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanUnloadNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanUnloadNow: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRDebuggingLibraryProvider(::windows::core::IUnknown);
impl ICLRDebuggingLibraryProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProvideLibrary<P0>(&self, pwszfilename: P0, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ProvideLibrary)(::windows::core::Interface::as_raw(self), pwszfilename.into_param().abi(), dwtimestamp, dwsizeofimage, phmodule).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRDebuggingLibraryProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRDebuggingLibraryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRDebuggingLibraryProvider {}
impl ::core::fmt::Debug for ICLRDebuggingLibraryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRDebuggingLibraryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRDebuggingLibraryProvider {
    type Vtable = ICLRDebuggingLibraryProvider_Vtbl;
}
impl ::core::clone::Clone for ICLRDebuggingLibraryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRDebuggingLibraryProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3151c08d_4d09_4f9b_8838_2880bf18fe51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDebuggingLibraryProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ProvideLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProvideLibrary: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRDomainManager(::windows::core::IUnknown);
impl ICLRDomainManager {
    pub unsafe fn SetAppDomainManagerType<P0, P1>(&self, wszappdomainmanagerassembly: P0, wszappdomainmanagertype: P1, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAppDomainManagerType)(::windows::core::Interface::as_raw(self), wszappdomainmanagerassembly.into_param().abi(), wszappdomainmanagertype.into_param().abi(), dwinitializedomainflags).ok()
    }
    pub unsafe fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const ::windows::core::PCWSTR, pwszpropertyvalues: *const ::windows::core::PCWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertiesForDefaultAppDomain)(::windows::core::Interface::as_raw(self), nproperties, pwszpropertynames, pwszpropertyvalues).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRDomainManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRDomainManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRDomainManager {}
impl ::core::fmt::Debug for ICLRDomainManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRDomainManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRDomainManager {
    type Vtable = ICLRDomainManager_Vtbl;
}
impl ::core::clone::Clone for ICLRDomainManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRDomainManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x270d00a2_8e15_4d0b_adeb_37bc3e47df77);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDomainManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAppDomainManagerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszappdomainmanagerassembly: ::windows::core::PCWSTR, wszappdomainmanagertype: ::windows::core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows::core::HRESULT,
    pub SetPropertiesForDefaultAppDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperties: u32, pwszpropertynames: *const ::windows::core::PCWSTR, pwszpropertyvalues: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRErrorReportingManager(::windows::core::IUnknown);
impl ICLRErrorReportingManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBucketParametersForCurrentException)(::windows::core::Interface::as_raw(self), pparams).ok()
    }
    pub unsafe fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *mut CustomDumpItem, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginCustomDump)(::windows::core::Interface::as_raw(self), dwflavor, dwnumitems, items, dwreserved).ok()
    }
    pub unsafe fn EndCustomDump(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndCustomDump)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRErrorReportingManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRErrorReportingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRErrorReportingManager {}
impl ::core::fmt::Debug for ICLRErrorReportingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRErrorReportingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRErrorReportingManager {
    type Vtable = ICLRErrorReportingManager_Vtbl;
}
impl ::core::clone::Clone for ICLRErrorReportingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRErrorReportingManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x980d2f1a_bf79_4c08_812a_bb9778928f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRErrorReportingManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBucketParametersForCurrentException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *mut BucketParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBucketParametersForCurrentException: usize,
    pub BeginCustomDump: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *mut CustomDumpItem, dwreserved: u32) -> ::windows::core::HRESULT,
    pub EndCustomDump: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRGCManager(::windows::core::IUnknown);
impl ICLRGCManager {
    pub unsafe fn Collect(&self, generation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Collect)(::windows::core::Interface::as_raw(self), generation).ok()
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStats)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGCStartupLimits)(::windows::core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRGCManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRGCManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRGCManager {}
impl ::core::fmt::Debug for ICLRGCManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRGCManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRGCManager {
    type Vtable = ICLRGCManager_Vtbl;
}
impl ::core::clone::Clone for ICLRGCManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRGCManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54d9007e_a8e2_4885_b7bf_f998deee4f2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRGCManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Collect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows::core::HRESULT,
    pub GetStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows::core::HRESULT,
    pub SetGCStartupLimits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRGCManager2(::windows::core::IUnknown);
impl ICLRGCManager2 {
    pub unsafe fn Collect(&self, generation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Collect)(::windows::core::Interface::as_raw(self), generation).ok()
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStats)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetGCStartupLimits)(::windows::core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
    pub unsafe fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGCStartupLimitsEx)(::windows::core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRGCManager2, ::windows::core::IUnknown, ICLRGCManager);
impl ::core::cmp::PartialEq for ICLRGCManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRGCManager2 {}
impl ::core::fmt::Debug for ICLRGCManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRGCManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRGCManager2 {
    type Vtable = ICLRGCManager2_Vtbl;
}
impl ::core::clone::Clone for ICLRGCManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRGCManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0603b793_a97a_4712_9cb4_0cd1c74c0f7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRGCManager2_Vtbl {
    pub base__: ICLRGCManager_Vtbl,
    pub SetGCStartupLimitsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRHostBindingPolicyManager(::windows::core::IUnknown);
impl ICLRHostBindingPolicyManager {
    pub unsafe fn ModifyApplicationPolicy<P0, P1>(&self, pwzsourceassemblyidentity: P0, pwztargetassemblyidentity: P1, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ModifyApplicationPolicy)(::windows::core::Interface::as_raw(self), pwzsourceassemblyidentity.into_param().abi(), pwztargetassemblyidentity.into_param().abi(), pbapplicationpolicy, cbapppolicysize, dwpolicymodifyflags, pbnewapplicationpolicy, pcbnewapppolicysize).ok()
    }
    pub unsafe fn EvaluatePolicy<P0>(&self, pwzreferenceidentity: P0, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows::core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EvaluatePolicy)(::windows::core::Interface::as_raw(self), pwzreferenceidentity.into_param().abi(), pbapplicationpolicy, cbapppolicysize, ::core::mem::transmute(pwzpostpolicyreferenceidentity), pcchpostpolicyreferenceidentity, pdwpoliciesapplied).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRHostBindingPolicyManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRHostBindingPolicyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRHostBindingPolicyManager {}
impl ::core::fmt::Debug for ICLRHostBindingPolicyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRHostBindingPolicyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRHostBindingPolicyManager {
    type Vtable = ICLRHostBindingPolicyManager_Vtbl;
}
impl ::core::clone::Clone for ICLRHostBindingPolicyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRHostBindingPolicyManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b3545e7_1856_48c9_a8ba_24b21a753c09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRHostBindingPolicyManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ModifyApplicationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzsourceassemblyidentity: ::windows::core::PCWSTR, pwztargetassemblyidentity: ::windows::core::PCWSTR, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows::core::HRESULT,
    pub EvaluatePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzreferenceidentity: ::windows::core::PCWSTR, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows::core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRHostProtectionManager(::windows::core::IUnknown);
impl ICLRHostProtectionManager {
    pub unsafe fn SetProtectedCategories(&self, categories: EApiCategories) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProtectedCategories)(::windows::core::Interface::as_raw(self), categories).ok()
    }
    pub unsafe fn SetEagerSerializeGrantSets(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEagerSerializeGrantSets)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRHostProtectionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRHostProtectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRHostProtectionManager {}
impl ::core::fmt::Debug for ICLRHostProtectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRHostProtectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRHostProtectionManager {
    type Vtable = ICLRHostProtectionManager_Vtbl;
}
impl ::core::clone::Clone for ICLRHostProtectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRHostProtectionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89f25f5c_ceef_43e1_9cfa_a68ce863aaac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRHostProtectionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetProtectedCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categories: EApiCategories) -> ::windows::core::HRESULT,
    pub SetEagerSerializeGrantSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRIoCompletionManager(::windows::core::IUnknown);
impl ICLRIoCompletionManager {
    pub unsafe fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnComplete)(::windows::core::Interface::as_raw(self), dwerrorcode, numberofbytestransferred, pvoverlapped).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRIoCompletionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRIoCompletionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRIoCompletionManager {}
impl ::core::fmt::Debug for ICLRIoCompletionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRIoCompletionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRIoCompletionManager {
    type Vtable = ICLRIoCompletionManager_Vtbl;
}
impl ::core::clone::Clone for ICLRIoCompletionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRIoCompletionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d74ce86_b8d6_4c84_b3a7_9768933b3c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRIoCompletionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRMemoryNotificationCallback(::windows::core::IUnknown);
impl ICLRMemoryNotificationCallback {
    pub unsafe fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMemoryNotification)(::windows::core::Interface::as_raw(self), ememoryavailable).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRMemoryNotificationCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRMemoryNotificationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRMemoryNotificationCallback {}
impl ::core::fmt::Debug for ICLRMemoryNotificationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRMemoryNotificationCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRMemoryNotificationCallback {
    type Vtable = ICLRMemoryNotificationCallback_Vtbl;
}
impl ::core::clone::Clone for ICLRMemoryNotificationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRMemoryNotificationCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47eb8e57_0846_4546_af76_6f42fcfc2649);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRMemoryNotificationCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnMemoryNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ememoryavailable: EMemoryAvailable) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRMetaHost(::windows::core::IUnknown);
impl ICLRMetaHost {
    pub unsafe fn GetRuntime<P0>(&self, pwzversion: P0, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetRuntime)(::windows::core::Interface::as_raw(self), pwzversion.into_param().abi(), riid, ppruntime).ok()
    }
    pub unsafe fn GetVersionFromFile<P0>(&self, pwzfilepath: P0, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetVersionFromFile)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), ::core::mem::transmute(pwzbuffer), pcchbuffer).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateInstalledRuntimes(&self) -> ::windows::core::Result<super::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).EnumerateInstalledRuntimes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumerateLoadedRuntimes<P0>(&self, hndprocess: P0) -> ::windows::core::Result<super::Com::IEnumUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).EnumerateLoadedRuntimes)(::windows::core::Interface::as_raw(self), hndprocess.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestRuntimeLoadedNotification)(::windows::core::Interface::as_raw(self), pcallbackfunction).ok()
    }
    pub unsafe fn QueryLegacyV2RuntimeBinding(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryLegacyV2RuntimeBinding)(::windows::core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn ExitProcess(&self, iexitcode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExitProcess)(::windows::core::Interface::as_raw(self), iexitcode).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRMetaHost, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRMetaHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRMetaHost {}
impl ::core::fmt::Debug for ICLRMetaHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRMetaHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRMetaHost {
    type Vtable = ICLRMetaHost_Vtbl;
}
impl ::core::clone::Clone for ICLRMetaHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRMetaHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd332db9e_b9b3_4125_8207_a14884f53216);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRMetaHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzversion: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVersionFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateInstalledRuntimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateInstalledRuntimes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumerateLoadedRuntimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumerateLoadedRuntimes: usize,
    pub RequestRuntimeLoadedNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows::core::HRESULT,
    pub QueryLegacyV2RuntimeBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExitProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iexitcode: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRMetaHostPolicy(::windows::core::IUnknown);
impl ICLRMetaHostPolicy {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRequestedRuntime<P0, P1>(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: P0, pcfgstream: P1, pwzversion: ::windows::core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows::core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).GetRequestedRuntime)(::windows::core::Interface::as_raw(self), dwpolicyflags, pwzbinary.into_param().abi(), pcfgstream.into_param().abi(), ::core::mem::transmute(pwzversion), pcchversion, ::core::mem::transmute(pwzimageversion), pcchimageversion, pdwconfigflags, riid, ppruntime).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRMetaHostPolicy, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRMetaHostPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRMetaHostPolicy {}
impl ::core::fmt::Debug for ICLRMetaHostPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRMetaHostPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRMetaHostPolicy {
    type Vtable = ICLRMetaHostPolicy_Vtbl;
}
impl ::core::clone::Clone for ICLRMetaHostPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRMetaHostPolicy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2190695_77b2_492e_8e14_c4b3a7fdd593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRMetaHostPolicy_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRequestedRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: ::windows::core::PCWSTR, pcfgstream: *mut ::core::ffi::c_void, pwzversion: ::windows::core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows::core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRequestedRuntime: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLROnEventManager(::windows::core::IUnknown);
impl ICLROnEventManager {
    pub unsafe fn RegisterActionOnEvent<P0>(&self, event: EClrEvent, paction: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActionOnCLREvent>,
    {
        (::windows::core::Interface::vtable(self).RegisterActionOnEvent)(::windows::core::Interface::as_raw(self), event, paction.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterActionOnEvent<P0>(&self, event: EClrEvent, paction: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActionOnCLREvent>,
    {
        (::windows::core::Interface::vtable(self).UnregisterActionOnEvent)(::windows::core::Interface::as_raw(self), event, paction.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLROnEventManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLROnEventManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLROnEventManager {}
impl ::core::fmt::Debug for ICLROnEventManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLROnEventManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLROnEventManager {
    type Vtable = ICLROnEventManager_Vtbl;
}
impl ::core::clone::Clone for ICLROnEventManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLROnEventManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d0e0132_e64f_493d_9260_025c0e32c175);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLROnEventManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterActionOnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterActionOnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRPolicyManager(::windows::core::IUnknown);
impl ICLRPolicyManager {
    pub unsafe fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultAction)(::windows::core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimeout)(::windows::core::Interface::as_raw(self), operation, dwmilliseconds).ok()
    }
    pub unsafe fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActionOnTimeout)(::windows::core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimeoutAndAction)(::windows::core::Interface::as_raw(self), operation, dwmilliseconds, action).ok()
    }
    pub unsafe fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActionOnFailure)(::windows::core::Interface::as_raw(self), failure, action).ok()
    }
    pub unsafe fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUnhandledExceptionPolicy)(::windows::core::Interface::as_raw(self), policy).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRPolicyManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRPolicyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRPolicyManager {}
impl ::core::fmt::Debug for ICLRPolicyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRPolicyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRPolicyManager {
    type Vtable = ICLRPolicyManager_Vtbl;
}
impl ::core::clone::Clone for ICLRPolicyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRPolicyManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d290010_d781_45da_a6f8_aa5d711a730e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRPolicyManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub SetActionOnTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT,
    pub SetTimeoutAndAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows::core::HRESULT,
    pub SetActionOnFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::HRESULT,
    pub SetUnhandledExceptionPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: EClrUnhandledException) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRProbingAssemblyEnum(::windows::core::IUnknown);
impl ICLRProbingAssemblyEnum {
    pub unsafe fn Get(&self, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRProbingAssemblyEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRProbingAssemblyEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRProbingAssemblyEnum {}
impl ::core::fmt::Debug for ICLRProbingAssemblyEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRProbingAssemblyEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRProbingAssemblyEnum {
    type Vtable = ICLRProbingAssemblyEnum_Vtbl;
}
impl ::core::clone::Clone for ICLRProbingAssemblyEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRProbingAssemblyEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0c5fb1f_416b_4f97_81f4_7ac7dc24dd5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRProbingAssemblyEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRProfiling(::windows::core::IUnknown);
impl ICLRProfiling {
    pub unsafe fn AttachProfiler<P0>(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows::core::GUID, wszprofilerpath: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AttachProfiler)(::windows::core::Interface::as_raw(self), dwprofileeprocessid, dwmillisecondsmax, pclsidprofiler, wszprofilerpath.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRProfiling, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRProfiling {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRProfiling {}
impl ::core::fmt::Debug for ICLRProfiling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRProfiling").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRProfiling {
    type Vtable = ICLRProfiling_Vtbl;
}
impl ::core::clone::Clone for ICLRProfiling {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRProfiling {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb349abe3_b56f_4689_bfcd_76bf39d888ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRProfiling_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AttachProfiler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows::core::GUID, wszprofilerpath: ::windows::core::PCWSTR, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRReferenceAssemblyEnum(::windows::core::IUnknown);
impl ICLRReferenceAssemblyEnum {
    pub unsafe fn Get(&self, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRReferenceAssemblyEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRReferenceAssemblyEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRReferenceAssemblyEnum {}
impl ::core::fmt::Debug for ICLRReferenceAssemblyEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRReferenceAssemblyEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRReferenceAssemblyEnum {
    type Vtable = ICLRReferenceAssemblyEnum_Vtbl;
}
impl ::core::clone::Clone for ICLRReferenceAssemblyEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRReferenceAssemblyEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd509cb5d_cf32_4876_ae61_67770cf91973);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRReferenceAssemblyEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRRuntimeHost(::windows::core::IUnknown);
impl ICLRRuntimeHost {
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetHostControl<P0>(&self, phostcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IHostControl>,
    {
        (::windows::core::Interface::vtable(self).SetHostControl)(::windows::core::Interface::as_raw(self), phostcontrol.into_param().abi()).ok()
    }
    pub unsafe fn GetCLRControl(&self) -> ::windows::core::Result<ICLRControl> {
        let mut result__ = ::windows::core::zeroed::<ICLRControl>();
        (::windows::core::Interface::vtable(self).GetCLRControl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnloadAppDomain<P0>(&self, dwappdomainid: u32, fwaituntildone: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).UnloadAppDomain)(::windows::core::Interface::as_raw(self), dwappdomainid, fwaituntildone.into_param().abi()).ok()
    }
    pub unsafe fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExecuteInAppDomain)(::windows::core::Interface::as_raw(self), dwappdomainid, pcallback, cookie).ok()
    }
    pub unsafe fn GetCurrentAppDomainId(&self, pdwappdomainid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentAppDomainId)(::windows::core::Interface::as_raw(self), pdwappdomainid).ok()
    }
    pub unsafe fn ExecuteApplication<P0>(&self, pwzappfullname: P0, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows::core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows::core::PCWSTR, preturnvalue: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ExecuteApplication)(::windows::core::Interface::as_raw(self), pwzappfullname.into_param().abi(), dwmanifestpaths, ppwzmanifestpaths, dwactivationdata, ppwzactivationdata, preturnvalue).ok()
    }
    pub unsafe fn ExecuteInDefaultAppDomain<P0, P1, P2, P3>(&self, pwzassemblypath: P0, pwztypename: P1, pwzmethodname: P2, pwzargument: P3, preturnvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ExecuteInDefaultAppDomain)(::windows::core::Interface::as_raw(self), pwzassemblypath.into_param().abi(), pwztypename.into_param().abi(), pwzmethodname.into_param().abi(), pwzargument.into_param().abi(), preturnvalue).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRRuntimeHost, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRRuntimeHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRRuntimeHost {}
impl ::core::fmt::Debug for ICLRRuntimeHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRRuntimeHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRRuntimeHost {
    type Vtable = ICLRRuntimeHost_Vtbl;
}
impl ::core::clone::Clone for ICLRRuntimeHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRRuntimeHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90f1a06c_7712_4762_86b5_7a5eba6bdb02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRRuntimeHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHostControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phostcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCLRControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclrcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UnloadAppDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnloadAppDomain: usize,
    pub ExecuteInAppDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentAppDomainId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwappdomainid: *mut u32) -> ::windows::core::HRESULT,
    pub ExecuteApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzappfullname: ::windows::core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows::core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows::core::PCWSTR, preturnvalue: *mut i32) -> ::windows::core::HRESULT,
    pub ExecuteInDefaultAppDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzassemblypath: ::windows::core::PCWSTR, pwztypename: ::windows::core::PCWSTR, pwzmethodname: ::windows::core::PCWSTR, pwzargument: ::windows::core::PCWSTR, preturnvalue: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRRuntimeInfo(::windows::core::IUnknown);
impl ICLRRuntimeInfo {
    pub unsafe fn GetVersionString(&self, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVersionString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwzbuffer), pcchbuffer).ok()
    }
    pub unsafe fn GetRuntimeDirectory(&self, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRuntimeDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwzbuffer), pcchbuffer).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoaded<P0>(&self, hndprocess: P0, pbloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).IsLoaded)(::windows::core::Interface::as_raw(self), hndprocess.into_param().abi(), pbloaded).ok()
    }
    pub unsafe fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadErrorString)(::windows::core::Interface::as_raw(self), iresourceid, ::core::mem::transmute(pwzbuffer), pcchbuffer, ilocaleid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadLibraryA<P0>(&self, pwzdllname: P0, phndmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadLibraryA)(::windows::core::Interface::as_raw(self), pwzdllname.into_param().abi(), phndmodule).ok()
    }
    pub unsafe fn GetProcAddress<P0>(&self, pszprocname: P0, ppproc: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetProcAddress)(::windows::core::Interface::as_raw(self), pszprocname.into_param().abi(), ppproc).ok()
    }
    pub unsafe fn GetInterface(&self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInterface)(::windows::core::Interface::as_raw(self), rclsid, riid, ppunk).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoadable(&self, pbloadable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsLoadable)(::windows::core::Interface::as_raw(self), pbloadable).ok()
    }
    pub unsafe fn SetDefaultStartupFlags<P0>(&self, dwstartupflags: u32, pwzhostconfigfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDefaultStartupFlags)(::windows::core::Interface::as_raw(self), dwstartupflags, pwzhostconfigfile.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows::core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultStartupFlags)(::windows::core::Interface::as_raw(self), pdwstartupflags, ::core::mem::transmute(pwzhostconfigfile), pcchhostconfigfile).ok()
    }
    pub unsafe fn BindAsLegacyV2Runtime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindAsLegacyV2Runtime)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStarted(&self, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsStarted)(::windows::core::Interface::as_raw(self), pbstarted, pdwstartupflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRRuntimeInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRRuntimeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRRuntimeInfo {}
impl ::core::fmt::Debug for ICLRRuntimeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRRuntimeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRRuntimeInfo {
    type Vtable = ICLRRuntimeInfo_Vtbl;
}
impl ::core::clone::Clone for ICLRRuntimeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRRuntimeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd39d1d2_ba2f_486a_89b0_b4b0cb466891);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRRuntimeInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub GetRuntimeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLoaded: usize,
    pub LoadErrorString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iresourceid: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadLibraryA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzdllname: ::windows::core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadLibraryA: usize,
    pub GetProcAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszprocname: ::windows::core::PCSTR, ppproc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLoadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbloadable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLoadable: usize,
    pub SetDefaultStartupFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstartupflags: u32, pwzhostconfigfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDefaultStartupFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows::core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows::core::HRESULT,
    pub BindAsLegacyV2Runtime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStarted: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRStrongName(::windows::core::IUnknown);
impl ICLRStrongName {
    pub unsafe fn GetHashFromAssemblyFile<P0>(&self, pszfilepath: P0, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetHashFromAssemblyFile)(::windows::core::Interface::as_raw(self), pszfilepath.into_param().abi(), pihashalg, pbhash, cchhash, pchhash).ok()
    }
    pub unsafe fn GetHashFromAssemblyFileW<P0>(&self, pwzfilepath: P0, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetHashFromAssemblyFileW)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), pihashalg, pbhash, cchhash, pchhash).ok()
    }
    pub unsafe fn GetHashFromBlob(&self, pbblob: *mut u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHashFromBlob)(::windows::core::Interface::as_raw(self), pbblob, cchblob, pihashalg, pbhash, cchhash, pchhash).ok()
    }
    pub unsafe fn GetHashFromFile<P0>(&self, pszfilepath: P0, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetHashFromFile)(::windows::core::Interface::as_raw(self), pszfilepath.into_param().abi(), pihashalg, pbhash, cchhash, pchhash).ok()
    }
    pub unsafe fn GetHashFromFileW<P0>(&self, pwzfilepath: P0, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetHashFromFileW)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), pihashalg, pbhash, cchhash, pchhash).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHashFromHandle<P0>(&self, hfile: P0, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).GetHashFromHandle)(::windows::core::Interface::as_raw(self), hfile.into_param().abi(), pihashalg, pbhash, cchhash, pchhash).ok()
    }
    pub unsafe fn StrongNameCompareAssemblies<P0, P1>(&self, pwzassembly1: P0, pwzassembly2: P1, pdwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameCompareAssemblies)(::windows::core::Interface::as_raw(self), pwzassembly1.into_param().abi(), pwzassembly2.into_param().abi(), pdwresult).ok()
    }
    pub unsafe fn StrongNameFreeBuffer(&self, pbmemory: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StrongNameFreeBuffer)(::windows::core::Interface::as_raw(self), pbmemory).ok()
    }
    pub unsafe fn StrongNameGetBlob<P0>(&self, pwzfilepath: P0, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameGetBlob)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), pbblob, pcbblob).ok()
    }
    pub unsafe fn StrongNameGetBlobFromImage(&self, pbbase: *mut u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StrongNameGetBlobFromImage)(::windows::core::Interface::as_raw(self), pbbase, dwlength, pbblob, pcbblob).ok()
    }
    pub unsafe fn StrongNameGetPublicKey<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameGetPublicKey)(::windows::core::Interface::as_raw(self), pwzkeycontainer.into_param().abi(), pbkeyblob, cbkeyblob, ppbpublickeyblob, pcbpublickeyblob).ok()
    }
    pub unsafe fn StrongNameHashSize(&self, ulhashalg: u32, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StrongNameHashSize)(::windows::core::Interface::as_raw(self), ulhashalg, pcbsize).ok()
    }
    pub unsafe fn StrongNameKeyDelete<P0>(&self, pwzkeycontainer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameKeyDelete)(::windows::core::Interface::as_raw(self), pwzkeycontainer.into_param().abi()).ok()
    }
    pub unsafe fn StrongNameKeyGen<P0>(&self, pwzkeycontainer: P0, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameKeyGen)(::windows::core::Interface::as_raw(self), pwzkeycontainer.into_param().abi(), dwflags, ppbkeyblob, pcbkeyblob).ok()
    }
    pub unsafe fn StrongNameKeyGenEx<P0>(&self, pwzkeycontainer: P0, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameKeyGenEx)(::windows::core::Interface::as_raw(self), pwzkeycontainer.into_param().abi(), dwflags, dwkeysize, ppbkeyblob, pcbkeyblob).ok()
    }
    pub unsafe fn StrongNameKeyInstall<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *mut u8, cbkeyblob: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameKeyInstall)(::windows::core::Interface::as_raw(self), pwzkeycontainer.into_param().abi(), pbkeyblob, cbkeyblob).ok()
    }
    pub unsafe fn StrongNameSignatureGeneration<P0, P1>(&self, pwzfilepath: P0, pwzkeycontainer: P1, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameSignatureGeneration)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), pwzkeycontainer.into_param().abi(), pbkeyblob, cbkeyblob, ppbsignatureblob, pcbsignatureblob).ok()
    }
    pub unsafe fn StrongNameSignatureGenerationEx<P0, P1>(&self, wszfilepath: P0, wszkeycontainer: P1, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameSignatureGenerationEx)(::windows::core::Interface::as_raw(self), wszfilepath.into_param().abi(), wszkeycontainer.into_param().abi(), pbkeyblob, cbkeyblob, ppbsignatureblob, pcbsignatureblob, dwflags).ok()
    }
    pub unsafe fn StrongNameSignatureSize(&self, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StrongNameSignatureSize)(::windows::core::Interface::as_raw(self), pbpublickeyblob, cbpublickeyblob, pcbsize).ok()
    }
    pub unsafe fn StrongNameSignatureVerification<P0>(&self, pwzfilepath: P0, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameSignatureVerification)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), dwinflags, pdwoutflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StrongNameSignatureVerificationEx<P0, P1>(&self, pwzfilepath: P0, fforceverification: P1, pfwasverified: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).StrongNameSignatureVerificationEx)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), fforceverification.into_param().abi(), pfwasverified).ok()
    }
    pub unsafe fn StrongNameSignatureVerificationFromImage(&self, pbbase: *mut u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StrongNameSignatureVerificationFromImage)(::windows::core::Interface::as_raw(self), pbbase, dwlength, dwinflags, pdwoutflags).ok()
    }
    pub unsafe fn StrongNameTokenFromAssembly<P0>(&self, pwzfilepath: P0, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameTokenFromAssembly)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), ppbstrongnametoken, pcbstrongnametoken).ok()
    }
    pub unsafe fn StrongNameTokenFromAssemblyEx<P0>(&self, pwzfilepath: P0, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameTokenFromAssemblyEx)(::windows::core::Interface::as_raw(self), pwzfilepath.into_param().abi(), ppbstrongnametoken, pcbstrongnametoken, ppbpublickeyblob, pcbpublickeyblob).ok()
    }
    pub unsafe fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StrongNameTokenFromPublicKey)(::windows::core::Interface::as_raw(self), pbpublickeyblob, cbpublickeyblob, ppbstrongnametoken, pcbstrongnametoken).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRStrongName, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRStrongName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRStrongName {}
impl ::core::fmt::Debug for ICLRStrongName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRStrongName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRStrongName {
    type Vtable = ICLRStrongName_Vtbl;
}
impl ::core::clone::Clone for ICLRStrongName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRStrongName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fd93ccf_3280_4391_b3a9_96e1cde77c8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRStrongName_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetHashFromAssemblyFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilepath: ::windows::core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT,
    pub GetHashFromAssemblyFileW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT,
    pub GetHashFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbblob: *mut u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT,
    pub GetHashFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilepath: ::windows::core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT,
    pub GetHashFromFileW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHashFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHashFromHandle: usize,
    pub StrongNameCompareAssemblies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzassembly1: ::windows::core::PCWSTR, pwzassembly2: ::windows::core::PCWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameFreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmemory: *mut u8) -> ::windows::core::HRESULT,
    pub StrongNameGetBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameGetBlobFromImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbase: *mut u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameGetPublicKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameHashSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulhashalg: u32, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameKeyDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub StrongNameKeyGen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameKeyGenEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameKeyInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32) -> ::windows::core::HRESULT,
    pub StrongNameSignatureGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameSignatureGenerationEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, wszkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub StrongNameSignatureSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameSignatureVerification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StrongNameSignatureVerificationEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pfwasverified: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StrongNameSignatureVerificationEx: usize,
    pub StrongNameSignatureVerificationFromImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbase: *mut u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameTokenFromAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameTokenFromAssemblyEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::HRESULT,
    pub StrongNameTokenFromPublicKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRStrongName2(::windows::core::IUnknown);
impl ICLRStrongName2 {
    pub unsafe fn StrongNameGetPublicKeyEx<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameGetPublicKeyEx)(::windows::core::Interface::as_raw(self), pwzkeycontainer.into_param().abi(), pbkeyblob, cbkeyblob, ppbpublickeyblob, pcbpublickeyblob, uhashalgid, ureserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StrongNameSignatureVerificationEx2<P0, P1>(&self, wszfilepath: P0, fforceverification: P1, pbecmapublickey: *mut u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).StrongNameSignatureVerificationEx2)(::windows::core::Interface::as_raw(self), wszfilepath.into_param().abi(), fforceverification.into_param().abi(), pbecmapublickey, cbecmapublickey, pfwasverified).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRStrongName2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRStrongName2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRStrongName2 {}
impl ::core::fmt::Debug for ICLRStrongName2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRStrongName2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRStrongName2 {
    type Vtable = ICLRStrongName2_Vtbl;
}
impl ::core::clone::Clone for ICLRStrongName2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRStrongName2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc22ed5c5_4b59_4975_90eb_85ea55c0069b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRStrongName2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StrongNameGetPublicKeyEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StrongNameSignatureVerificationEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *mut u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StrongNameSignatureVerificationEx2: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRStrongName3(::windows::core::IUnknown);
impl ICLRStrongName3 {
    pub unsafe fn StrongNameDigestGenerate<P0>(&self, wszfilepath: P0, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameDigestGenerate)(::windows::core::Interface::as_raw(self), wszfilepath.into_param().abi(), ppbdigestblob, pcbdigestblob, dwflags).ok()
    }
    pub unsafe fn StrongNameDigestSign<P0>(&self, wszkeycontainer: P0, pbkeyblob: *mut u8, cbkeyblob: u32, pbdigestblob: *mut u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameDigestSign)(::windows::core::Interface::as_raw(self), wszkeycontainer.into_param().abi(), pbkeyblob, cbkeyblob, pbdigestblob, cbdigestblob, hashalgid, ppbsignatureblob, pcbsignatureblob, dwflags).ok()
    }
    pub unsafe fn StrongNameDigestEmbed<P0>(&self, wszfilepath: P0, pbsignatureblob: *mut u8, cbsignatureblob: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).StrongNameDigestEmbed)(::windows::core::Interface::as_raw(self), wszfilepath.into_param().abi(), pbsignatureblob, cbsignatureblob).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRStrongName3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRStrongName3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRStrongName3 {}
impl ::core::fmt::Debug for ICLRStrongName3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRStrongName3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRStrongName3 {
    type Vtable = ICLRStrongName3_Vtbl;
}
impl ::core::clone::Clone for ICLRStrongName3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRStrongName3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22c7089b_bbd3_414a_b698_210f263f1fed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRStrongName3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StrongNameDigestGenerate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub StrongNameDigestSign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, pbdigestblob: *mut u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub StrongNameDigestEmbed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, pbsignatureblob: *mut u8, cbsignatureblob: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRSyncManager(::windows::core::IUnknown);
impl ICLRSyncManager {
    pub unsafe fn GetMonitorOwner(&self, cookie: usize) -> ::windows::core::Result<IHostTask> {
        let mut result__ = ::windows::core::zeroed::<IHostTask>();
        (::windows::core::Interface::vtable(self).GetMonitorOwner)(::windows::core::Interface::as_raw(self), cookie, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRWLockOwnerIterator(&self, cookie: usize, piterator: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateRWLockOwnerIterator)(::windows::core::Interface::as_raw(self), cookie, piterator).ok()
    }
    pub unsafe fn GetRWLockOwnerNext(&self, iterator: usize) -> ::windows::core::Result<IHostTask> {
        let mut result__ = ::windows::core::zeroed::<IHostTask>();
        (::windows::core::Interface::vtable(self).GetRWLockOwnerNext)(::windows::core::Interface::as_raw(self), iterator, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteRWLockOwnerIterator)(::windows::core::Interface::as_raw(self), iterator).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRSyncManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRSyncManager {}
impl ::core::fmt::Debug for ICLRSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRSyncManager {
    type Vtable = ICLRSyncManager_Vtbl;
}
impl ::core::clone::Clone for ICLRSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRSyncManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ff199d_ad21_48f9_a16c_f24ebbb8727d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRSyncManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMonitorOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRWLockOwnerIterator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: usize, piterator: *mut usize) -> ::windows::core::HRESULT,
    pub GetRWLockOwnerNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iterator: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteRWLockOwnerIterator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iterator: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRTask(::windows::core::IUnknown);
impl ICLRTask {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SwitchIn<P0>(&self, threadhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).SwitchIn)(::windows::core::Interface::as_raw(self), threadhandle.into_param().abi()).ok()
    }
    pub unsafe fn SwitchOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SwitchOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMemStats(&self, memusage: *mut COR_GC_THREAD_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMemStats)(::windows::core::Interface::as_raw(self), memusage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reset<P0>(&self, ffull: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self), ffull.into_param().abi()).ok()
    }
    pub unsafe fn ExitTask(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExitTask)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RudeAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RudeAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsPriorityScheduling(&self, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NeedsPriorityScheduling)(::windows::core::Interface::as_raw(self), pbneedspriorityscheduling).ok()
    }
    pub unsafe fn YieldTask(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).YieldTask)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LocksHeld(&self, plockcount: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LocksHeld)(::windows::core::Interface::as_raw(self), plockcount).ok()
    }
    pub unsafe fn SetTaskIdentifier(&self, asked: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskIdentifier)(::windows::core::Interface::as_raw(self), asked).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRTask, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRTask {}
impl ::core::fmt::Debug for ICLRTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRTask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRTask {
    type Vtable = ICLRTask_Vtbl;
}
impl ::core::clone::Clone for ICLRTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRTask {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28e66a4a_9906_4225_b231_9187c3eb8611);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRTask_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SwitchIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SwitchIn: usize,
    pub SwitchOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMemStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memusage: *mut COR_GC_THREAD_STATS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffull: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Reset: usize,
    pub ExitTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RudeAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NeedsPriorityScheduling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NeedsPriorityScheduling: usize,
    pub YieldTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocksHeld: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plockcount: *mut usize) -> ::windows::core::HRESULT,
    pub SetTaskIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asked: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRTask2(::windows::core::IUnknown);
impl ICLRTask2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SwitchIn<P0>(&self, threadhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.SwitchIn)(::windows::core::Interface::as_raw(self), threadhandle.into_param().abi()).ok()
    }
    pub unsafe fn SwitchOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SwitchOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMemStats(&self, memusage: *mut COR_GC_THREAD_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMemStats)(::windows::core::Interface::as_raw(self), memusage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reset<P0>(&self, ffull: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Reset)(::windows::core::Interface::as_raw(self), ffull.into_param().abi()).ok()
    }
    pub unsafe fn ExitTask(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExitTask)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RudeAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RudeAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsPriorityScheduling(&self, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.NeedsPriorityScheduling)(::windows::core::Interface::as_raw(self), pbneedspriorityscheduling).ok()
    }
    pub unsafe fn YieldTask(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.YieldTask)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LocksHeld(&self, plockcount: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LocksHeld)(::windows::core::Interface::as_raw(self), plockcount).ok()
    }
    pub unsafe fn SetTaskIdentifier(&self, asked: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetTaskIdentifier)(::windows::core::Interface::as_raw(self), asked).ok()
    }
    pub unsafe fn BeginPreventAsyncAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPreventAsyncAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndPreventAsyncAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndPreventAsyncAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRTask2, ::windows::core::IUnknown, ICLRTask);
impl ::core::cmp::PartialEq for ICLRTask2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRTask2 {}
impl ::core::fmt::Debug for ICLRTask2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRTask2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRTask2 {
    type Vtable = ICLRTask2_Vtbl;
}
impl ::core::clone::Clone for ICLRTask2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRTask2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28e66a4a_9906_4225_b231_9187c3eb8612);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRTask2_Vtbl {
    pub base__: ICLRTask_Vtbl,
    pub BeginPreventAsyncAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndPreventAsyncAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICLRTaskManager(::windows::core::IUnknown);
impl ICLRTaskManager {
    pub unsafe fn CreateTask(&self) -> ::windows::core::Result<ICLRTask> {
        let mut result__ = ::windows::core::zeroed::<ICLRTask>();
        (::windows::core::Interface::vtable(self).CreateTask)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentTask(&self) -> ::windows::core::Result<ICLRTask> {
        let mut result__ = ::windows::core::zeroed::<ICLRTask>();
        (::windows::core::Interface::vtable(self).GetCurrentTask)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUILocale(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUILocale)(::windows::core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLocale(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLocale)(::windows::core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn GetCurrentTaskType(&self, ptasktype: *mut ETaskType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentTaskType)(::windows::core::Interface::as_raw(self), ptasktype).ok()
    }
}
::windows::imp::interface_hierarchy!(ICLRTaskManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICLRTaskManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICLRTaskManager {}
impl ::core::fmt::Debug for ICLRTaskManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICLRTaskManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICLRTaskManager {
    type Vtable = ICLRTaskManager_Vtbl;
}
impl ::core::clone::Clone for ICLRTaskManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICLRTaskManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4862efbe_3ae5_44f8_8feb_346190ee8a34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRTaskManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetUILocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT,
    pub GetCurrentTaskType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptasktype: *mut ETaskType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICatalogServices(::windows::core::IUnknown);
impl ICatalogServices {
    pub unsafe fn Autodone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Autodone)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotAutodone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotAutodone)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICatalogServices, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICatalogServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatalogServices {}
impl ::core::fmt::Debug for ICatalogServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICatalogServices {
    type Vtable = ICatalogServices_Vtbl;
}
impl ::core::clone::Clone for ICatalogServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICatalogServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04c6be1e_1db1_4058_ab7a_700cccfbf254);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Autodone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotAutodone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICorConfiguration(::windows::core::IUnknown);
impl ICorConfiguration {
    pub unsafe fn SetGCThreadControl<P0>(&self, pgcthreadcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IGCThreadControl>,
    {
        (::windows::core::Interface::vtable(self).SetGCThreadControl)(::windows::core::Interface::as_raw(self), pgcthreadcontrol.into_param().abi()).ok()
    }
    pub unsafe fn SetGCHostControl<P0>(&self, pgchostcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IGCHostControl>,
    {
        (::windows::core::Interface::vtable(self).SetGCHostControl)(::windows::core::Interface::as_raw(self), pgchostcontrol.into_param().abi()).ok()
    }
    pub unsafe fn SetDebuggerThreadControl<P0>(&self, pdebuggerthreadcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebuggerThreadControl>,
    {
        (::windows::core::Interface::vtable(self).SetDebuggerThreadControl)(::windows::core::Interface::as_raw(self), pdebuggerthreadcontrol.into_param().abi()).ok()
    }
    pub unsafe fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDebuggerSpecialThread)(::windows::core::Interface::as_raw(self), dwspecialthreadid).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorConfiguration, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorConfiguration {}
impl ::core::fmt::Debug for ICorConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorConfiguration {
    type Vtable = ICorConfiguration_Vtbl;
}
impl ::core::clone::Clone for ICorConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c2b07a5_1e98_11d3_872f_00c04f79ed0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorConfiguration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGCThreadControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgcthreadcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGCHostControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgchostcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDebuggerThreadControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdebuggerthreadcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddDebuggerSpecialThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwspecialthreadid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICorRuntimeHost(::windows::core::IUnknown);
impl ICorRuntimeHost {
    pub unsafe fn CreateLogicalThreadState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateLogicalThreadState)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteLogicalThreadState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteLogicalThreadState)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SwitchInLogicalThreadState(&self, pfibercookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SwitchInLogicalThreadState)(::windows::core::Interface::as_raw(self), pfibercookie).ok()
    }
    pub unsafe fn SwitchOutLogicalThreadState(&self, pfibercookie: *mut *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SwitchOutLogicalThreadState)(::windows::core::Interface::as_raw(self), pfibercookie).ok()
    }
    pub unsafe fn LocksHeldByLogicalThread(&self, pcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LocksHeldByLogicalThread)(::windows::core::Interface::as_raw(self), pcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapFile<P0>(&self, hfile: P0, hmapaddress: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).MapFile)(::windows::core::Interface::as_raw(self), hfile.into_param().abi(), hmapaddress).ok()
    }
    pub unsafe fn GetConfiguration(&self) -> ::windows::core::Result<ICorConfiguration> {
        let mut result__ = ::windows::core::zeroed::<ICorConfiguration>();
        (::windows::core::Interface::vtable(self).GetConfiguration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDomain<P0, P1>(&self, pwzfriendlyname: P0, pidentityarray: P1) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateDomain)(::windows::core::Interface::as_raw(self), pwzfriendlyname.into_param().abi(), pidentityarray.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultDomain(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetDefaultDomain)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDomains(&self, henum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDomains)(::windows::core::Interface::as_raw(self), henum).ok()
    }
    pub unsafe fn NextDomain(&self, henum: *mut ::core::ffi::c_void, pappdomain: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NextDomain)(::windows::core::Interface::as_raw(self), henum, ::core::mem::transmute(pappdomain)).ok()
    }
    pub unsafe fn CloseEnum(&self, henum: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseEnum)(::windows::core::Interface::as_raw(self), henum).ok()
    }
    pub unsafe fn CreateDomainEx<P0, P1, P2>(&self, pwzfriendlyname: P0, psetup: P1, pevidence: P2) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateDomainEx)(::windows::core::Interface::as_raw(self), pwzfriendlyname.into_param().abi(), psetup.into_param().abi(), pevidence.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDomainSetup(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateDomainSetup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEvidence(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateEvidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnloadDomain<P0>(&self, pappdomain: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).UnloadDomain)(::windows::core::Interface::as_raw(self), pappdomain.into_param().abi()).ok()
    }
    pub unsafe fn CurrentDomain(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CurrentDomain)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ICorRuntimeHost, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorRuntimeHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorRuntimeHost {}
impl ::core::fmt::Debug for ICorRuntimeHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorRuntimeHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorRuntimeHost {
    type Vtable = ICorRuntimeHost_Vtbl;
}
impl ::core::clone::Clone for ICorRuntimeHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorRuntimeHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb2f6722_ab3a_11d2_9c40_00c04fa30a3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorRuntimeHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateLogicalThreadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteLogicalThreadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SwitchInLogicalThreadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfibercookie: *mut u32) -> ::windows::core::HRESULT,
    pub SwitchOutLogicalThreadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfibercookie: *mut *mut u32) -> ::windows::core::HRESULT,
    pub LocksHeldByLogicalThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MapFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapFile: usize,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows::core::PCWSTR, pidentityarray: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NextDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDomainEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows::core::PCWSTR, psetup: *mut ::core::ffi::c_void, pevidence: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDomainSetup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappdomainsetup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEvidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevidence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnloadDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ICorThreadpool(::windows::core::IUnknown);
impl ICorThreadpool {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub unsafe fn CorRegisterWaitForSingleObject<P0, P1>(&self, phnewwaitobject: *mut super::super::Foundation::HANDLE, hwaitobject: P0, callback: super::Threading::WAITORTIMERCALLBACK, context: *mut ::core::ffi::c_void, timeout: u32, executeonlyonce: P1, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CorRegisterWaitForSingleObject)(::windows::core::Interface::as_raw(self), phnewwaitobject, hwaitobject.into_param().abi(), callback, context, timeout, executeonlyonce.into_param().abi(), result).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CorUnregisterWait<P0, P1>(&self, hwaitobject: P0, completionevent: P1, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).CorUnregisterWait)(::windows::core::Interface::as_raw(self), hwaitobject.into_param().abi(), completionevent.into_param().abi(), result).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub unsafe fn CorQueueUserWorkItem<P0>(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, executeonlyonce: P0, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CorQueueUserWorkItem)(::windows::core::Interface::as_raw(self), function, context, executeonlyonce.into_param().abi(), result).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub unsafe fn CorCreateTimer(&self, phnewtimer: *mut super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *mut ::core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CorCreateTimer)(::windows::core::Interface::as_raw(self), phnewtimer, callback, parameter, duetime, period, result).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CorChangeTimer<P0>(&self, timer: P0, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).CorChangeTimer)(::windows::core::Interface::as_raw(self), timer.into_param().abi(), duetime, period, result).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CorDeleteTimer<P0, P1>(&self, timer: P0, completionevent: P1, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).CorDeleteTimer)(::windows::core::Interface::as_raw(self), timer.into_param().abi(), completionevent.into_param().abi(), result).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn CorBindIoCompletionCallback<P0>(&self, filehandle: P0, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).CorBindIoCompletionCallback)(::windows::core::Interface::as_raw(self), filehandle.into_param().abi(), callback).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub unsafe fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CorCallOrQueueUserWorkItem)(::windows::core::Interface::as_raw(self), function, context, result).ok()
    }
    pub unsafe fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CorSetMaxThreads)(::windows::core::Interface::as_raw(self), maxworkerthreads, maxiocompletionthreads).ok()
    }
    pub unsafe fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CorGetMaxThreads)(::windows::core::Interface::as_raw(self), maxworkerthreads, maxiocompletionthreads).ok()
    }
    pub unsafe fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CorGetAvailableThreads)(::windows::core::Interface::as_raw(self), availableworkerthreads, availableiocompletionthreads).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorThreadpool, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorThreadpool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorThreadpool {}
impl ::core::fmt::Debug for ICorThreadpool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorThreadpool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorThreadpool {
    type Vtable = ICorThreadpool_Vtbl;
}
impl ::core::clone::Clone for ICorThreadpool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorThreadpool {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84680d3a_b2c1_46e8_acc2_dbc0a359159a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorThreadpool_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub CorRegisterWaitForSingleObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phnewwaitobject: *mut super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *mut ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Threading")))]
    CorRegisterWaitForSingleObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CorUnregisterWait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CorUnregisterWait: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub CorQueueUserWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Threading")))]
    CorQueueUserWorkItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub CorCreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phnewtimer: *mut super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *mut ::core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Threading")))]
    CorCreateTimer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CorChangeTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CorChangeTimer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CorDeleteTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CorDeleteTimer: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub CorBindIoCompletionCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    CorBindIoCompletionCallback: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub CorCallOrQueueUserWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Threading")))]
    CorCallOrQueueUserWorkItem: usize,
    pub CorSetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows::core::HRESULT,
    pub CorGetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows::core::HRESULT,
    pub CorGetAvailableThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IDebuggerInfo(::windows::core::IUnknown);
impl IDebuggerInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDebuggerAttached(&self, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDebuggerAttached)(::windows::core::Interface::as_raw(self), pbattached).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebuggerInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebuggerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebuggerInfo {}
impl ::core::fmt::Debug for IDebuggerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebuggerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebuggerInfo {
    type Vtable = IDebuggerInfo_Vtbl;
}
impl ::core::clone::Clone for IDebuggerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebuggerInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf24142d_a47d_4d24_a66d_8c2141944e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebuggerInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDebuggerAttached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDebuggerAttached: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IDebuggerThreadControl(::windows::core::IUnknown);
impl IDebuggerThreadControl {
    pub unsafe fn ThreadIsBlockingForDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadIsBlockingForDebugger)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseAllRuntimeThreads(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseAllRuntimeThreads)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartBlockingForDebugger(&self, dwunused: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartBlockingForDebugger)(::windows::core::Interface::as_raw(self), dwunused).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebuggerThreadControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebuggerThreadControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebuggerThreadControl {}
impl ::core::fmt::Debug for IDebuggerThreadControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebuggerThreadControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebuggerThreadControl {
    type Vtable = IDebuggerThreadControl_Vtbl;
}
impl ::core::clone::Clone for IDebuggerThreadControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebuggerThreadControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23d86786_0bb5_4774_8fb5_e3522add6246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebuggerThreadControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ThreadIsBlockingForDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseAllRuntimeThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartBlockingForDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwunused: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IGCHost(::windows::core::IUnknown);
impl IGCHost {
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGCStartupLimits)(::windows::core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
    pub unsafe fn Collect(&self, generation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Collect)(::windows::core::Interface::as_raw(self), generation).ok()
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStats)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetThreadStats(&self, pfibercookie: *mut u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadStats)(::windows::core::Interface::as_raw(self), pfibercookie, pstats).ok()
    }
    pub unsafe fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVirtualMemLimit)(::windows::core::Interface::as_raw(self), sztmaxvirtualmemmb).ok()
    }
}
::windows::imp::interface_hierarchy!(IGCHost, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IGCHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGCHost {}
impl ::core::fmt::Debug for IGCHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGCHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGCHost {
    type Vtable = IGCHost_Vtbl;
}
impl ::core::clone::Clone for IGCHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGCHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfac34f6e_0dcd_47b5_8021_531bc5ecca63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGCStartupLimits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows::core::HRESULT,
    pub Collect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows::core::HRESULT,
    pub GetStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows::core::HRESULT,
    pub GetThreadStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfibercookie: *mut u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows::core::HRESULT,
    pub SetVirtualMemLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IGCHost2(::windows::core::IUnknown);
impl IGCHost2 {
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetGCStartupLimits)(::windows::core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
    pub unsafe fn Collect(&self, generation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Collect)(::windows::core::Interface::as_raw(self), generation).ok()
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStats)(::windows::core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetThreadStats(&self, pfibercookie: *mut u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetThreadStats)(::windows::core::Interface::as_raw(self), pfibercookie, pstats).ok()
    }
    pub unsafe fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetVirtualMemLimit)(::windows::core::Interface::as_raw(self), sztmaxvirtualmemmb).ok()
    }
    pub unsafe fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGCStartupLimitsEx)(::windows::core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
}
::windows::imp::interface_hierarchy!(IGCHost2, ::windows::core::IUnknown, IGCHost);
impl ::core::cmp::PartialEq for IGCHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGCHost2 {}
impl ::core::fmt::Debug for IGCHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGCHost2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGCHost2 {
    type Vtable = IGCHost2_Vtbl;
}
impl ::core::clone::Clone for IGCHost2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGCHost2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1d70cec_2dbe_4e2f_9291_fdf81438a1df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCHost2_Vtbl {
    pub base__: IGCHost_Vtbl,
    pub SetGCStartupLimitsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IGCHostControl(::windows::core::IUnknown);
impl IGCHostControl {
    pub unsafe fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestVirtualMemLimit)(::windows::core::Interface::as_raw(self), sztmaxvirtualmemmb, psztnewmaxvirtualmemmb).ok()
    }
}
::windows::imp::interface_hierarchy!(IGCHostControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IGCHostControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGCHostControl {}
impl ::core::fmt::Debug for IGCHostControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGCHostControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGCHostControl {
    type Vtable = IGCHostControl_Vtbl;
}
impl ::core::clone::Clone for IGCHostControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGCHostControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5513d564_8374_4cb9_aed9_0083f4160a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCHostControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RequestVirtualMemLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IGCThreadControl(::windows::core::IUnknown);
impl IGCThreadControl {
    pub unsafe fn ThreadIsBlockingForSuspension(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadIsBlockingForSuspension)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionStarting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspensionStarting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionEnding(&self, generation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspensionEnding)(::windows::core::Interface::as_raw(self), generation).ok()
    }
}
::windows::imp::interface_hierarchy!(IGCThreadControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IGCThreadControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGCThreadControl {}
impl ::core::fmt::Debug for IGCThreadControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGCThreadControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGCThreadControl {
    type Vtable = IGCThreadControl_Vtbl;
}
impl ::core::clone::Clone for IGCThreadControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGCThreadControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf31d1788_c397_4725_87a5_6af3472c2791);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCThreadControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ThreadIsBlockingForSuspension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspensionStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspensionEnding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostAssemblyManager(::windows::core::IUnknown);
impl IHostAssemblyManager {
    pub unsafe fn GetNonHostStoreAssemblies(&self) -> ::windows::core::Result<ICLRAssemblyReferenceList> {
        let mut result__ = ::windows::core::zeroed::<ICLRAssemblyReferenceList>();
        (::windows::core::Interface::vtable(self).GetNonHostStoreAssemblies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAssemblyStore(&self) -> ::windows::core::Result<IHostAssemblyStore> {
        let mut result__ = ::windows::core::zeroed::<IHostAssemblyStore>();
        (::windows::core::Interface::vtable(self).GetAssemblyStore)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IHostAssemblyManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostAssemblyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostAssemblyManager {}
impl ::core::fmt::Debug for IHostAssemblyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostAssemblyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostAssemblyManager {
    type Vtable = IHostAssemblyManager_Vtbl;
}
impl ::core::clone::Clone for IHostAssemblyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostAssemblyManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x613dabd7_62b2_493e_9e65_c1e32a1e0c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostAssemblyManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNonHostStoreAssemblies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAssemblyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppassemblystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostAssemblyStore(::windows::core::IUnknown);
impl IHostAssemblyStore {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProvideAssembly(&self, pbindinfo: *mut AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProvideAssembly)(::windows::core::Interface::as_raw(self), pbindinfo, passemblyid, pcontext, ::core::mem::transmute(ppstmassemblyimage), ::core::mem::transmute(ppstmpdb)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProvideModule(&self, pbindinfo: *mut ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProvideModule)(::windows::core::Interface::as_raw(self), pbindinfo, pdwmoduleid, ::core::mem::transmute(ppstmmoduleimage), ::core::mem::transmute(ppstmpdb)).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostAssemblyStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostAssemblyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostAssemblyStore {}
impl ::core::fmt::Debug for IHostAssemblyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostAssemblyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostAssemblyStore {
    type Vtable = IHostAssemblyStore_Vtbl;
}
impl ::core::clone::Clone for IHostAssemblyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostAssemblyStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b102a88_3f7f_496d_8fa2_c35374e01af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostAssemblyStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ProvideAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindinfo: *mut AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProvideAssembly: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProvideModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindinfo: *mut ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProvideModule: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostAutoEvent(::windows::core::IUnknown);
impl IHostAutoEvent {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::windows::core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn Set(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostAutoEvent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostAutoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostAutoEvent {}
impl ::core::fmt::Debug for IHostAutoEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostAutoEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostAutoEvent {
    type Vtable = IHostAutoEvent_Vtbl;
}
impl ::core::clone::Clone for IHostAutoEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostAutoEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50b0cfce_4063_4278_9673_e5cb4ed0bdb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostAutoEvent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostControl(::windows::core::IUnknown);
impl IHostControl {
    pub unsafe fn GetHostManager(&self, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHostManager)(::windows::core::Interface::as_raw(self), riid, ppobject).ok()
    }
    pub unsafe fn SetAppDomainManager<P0>(&self, dwappdomainid: u32, punkappdomainmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetAppDomainManager)(::windows::core::Interface::as_raw(self), dwappdomainid, punkappdomainmanager.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostControl {}
impl ::core::fmt::Debug for IHostControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostControl {
    type Vtable = IHostControl_Vtbl;
}
impl ::core::clone::Clone for IHostControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02ca073c_7079_4860_880a_c2f7a449c991);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetHostManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAppDomainManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappdomainid: u32, punkappdomainmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostCrst(::windows::core::IUnknown);
impl IHostCrst {
    pub unsafe fn Enter(&self, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enter)(::windows::core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn Leave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Leave)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryEnter(&self, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TryEnter)(::windows::core::Interface::as_raw(self), option, pbsucceeded).ok()
    }
    pub unsafe fn SetSpinCount(&self, dwspincount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSpinCount)(::windows::core::Interface::as_raw(self), dwspincount).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostCrst, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostCrst {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostCrst {}
impl ::core::fmt::Debug for IHostCrst {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostCrst").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostCrst {
    type Vtable = IHostCrst_Vtbl;
}
impl ::core::clone::Clone for IHostCrst {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostCrst {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6df710a6_26a4_4a65_8cd5_7237b8bda8dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostCrst_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: u32) -> ::windows::core::HRESULT,
    pub Leave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TryEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TryEnter: usize,
    pub SetSpinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwspincount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostGCManager(::windows::core::IUnknown);
impl IHostGCManager {
    pub unsafe fn ThreadIsBlockingForSuspension(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadIsBlockingForSuspension)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionStarting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspensionStarting)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionEnding(&self, generation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspensionEnding)(::windows::core::Interface::as_raw(self), generation).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostGCManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostGCManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostGCManager {}
impl ::core::fmt::Debug for IHostGCManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostGCManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostGCManager {
    type Vtable = IHostGCManager_Vtbl;
}
impl ::core::clone::Clone for IHostGCManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostGCManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d4ec34e_f248_457b_b603_255faaba0d21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostGCManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ThreadIsBlockingForSuspension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspensionStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspensionEnding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostIoCompletionManager(::windows::core::IUnknown);
impl IHostIoCompletionManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateIoCompletionPort(&self, phport: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateIoCompletionPort)(::windows::core::Interface::as_raw(self), phport).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseIoCompletionPort<P0>(&self, hport: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).CloseIoCompletionPort)(::windows::core::Interface::as_raw(self), hport.into_param().abi()).ok()
    }
    pub unsafe fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxThreads)(::windows::core::Interface::as_raw(self), dwmaxiocompletionthreads).ok()
    }
    pub unsafe fn GetMaxThreads(&self, pdwmaxiocompletionthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaxThreads)(::windows::core::Interface::as_raw(self), pdwmaxiocompletionthreads).ok()
    }
    pub unsafe fn GetAvailableThreads(&self, pdwavailableiocompletionthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAvailableThreads)(::windows::core::Interface::as_raw(self), pdwavailableiocompletionthreads).ok()
    }
    pub unsafe fn GetHostOverlappedSize(&self, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHostOverlappedSize)(::windows::core::Interface::as_raw(self), pcbsize).ok()
    }
    pub unsafe fn SetCLRIoCompletionManager<P0>(&self, pmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICLRIoCompletionManager>,
    {
        (::windows::core::Interface::vtable(self).SetCLRIoCompletionManager)(::windows::core::Interface::as_raw(self), pmanager.into_param().abi()).ok()
    }
    pub unsafe fn InitializeHostOverlapped(&self, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeHostOverlapped)(::windows::core::Interface::as_raw(self), pvoverlapped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Bind<P0, P1>(&self, hport: P0, hhandle: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Bind)(::windows::core::Interface::as_raw(self), hport.into_param().abi(), hhandle.into_param().abi()).ok()
    }
    pub unsafe fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinThreads)(::windows::core::Interface::as_raw(self), dwminiocompletionthreads).ok()
    }
    pub unsafe fn GetMinThreads(&self, pdwminiocompletionthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMinThreads)(::windows::core::Interface::as_raw(self), pdwminiocompletionthreads).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostIoCompletionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostIoCompletionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostIoCompletionManager {}
impl ::core::fmt::Debug for IHostIoCompletionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostIoCompletionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostIoCompletionManager {
    type Vtable = IHostIoCompletionManager_Vtbl;
}
impl ::core::clone::Clone for IHostIoCompletionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostIoCompletionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bde9d80_ec06_41d6_83e6_22580effcc20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostIoCompletionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateIoCompletionPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phport: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateIoCompletionPort: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseIoCompletionPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseIoCompletionPort: usize,
    pub SetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxiocompletionthreads: u32) -> ::windows::core::HRESULT,
    pub GetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxiocompletionthreads: *mut u32) -> ::windows::core::HRESULT,
    pub GetAvailableThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwavailableiocompletionthreads: *mut u32) -> ::windows::core::HRESULT,
    pub GetHostOverlappedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetCLRIoCompletionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InitializeHostOverlapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bind: usize,
    pub SetMinThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows::core::HRESULT,
    pub GetMinThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostMalloc(::windows::core::IUnknown);
impl IHostMalloc {
    pub unsafe fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Alloc)(::windows::core::Interface::as_raw(self), cbsize, ecriticallevel, ppmem).ok()
    }
    pub unsafe fn DebugAlloc<P0>(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: P0, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).DebugAlloc)(::windows::core::Interface::as_raw(self), cbsize, ecriticallevel, pszfilename.into_param().abi(), ilineno, ppmem).ok()
    }
    pub unsafe fn Free(&self, pmem: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Free)(::windows::core::Interface::as_raw(self), pmem).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostMalloc, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostMalloc {}
impl ::core::fmt::Debug for IHostMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostMalloc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostMalloc {
    type Vtable = IHostMalloc_Vtbl;
}
impl ::core::clone::Clone for IHostMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostMalloc {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1831991c_cc53_4a31_b218_04e910446479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostMalloc_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DebugAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: ::windows::core::PCSTR, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostManualEvent(::windows::core::IUnknown);
impl IHostManualEvent {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::windows::core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Set(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostManualEvent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostManualEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostManualEvent {}
impl ::core::fmt::Debug for IHostManualEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostManualEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostManualEvent {
    type Vtable = IHostManualEvent_Vtbl;
}
impl ::core::clone::Clone for IHostManualEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostManualEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bf4ec38_affe_4fb9_85a6_525268f15b54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostManualEvent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostMemoryManager(::windows::core::IUnknown);
impl IHostMemoryManager {
    pub unsafe fn CreateMalloc(&self, dwmalloctype: u32) -> ::windows::core::Result<IHostMalloc> {
        let mut result__ = ::windows::core::zeroed::<IHostMalloc>();
        (::windows::core::Interface::vtable(self).CreateMalloc)(::windows::core::Interface::as_raw(self), dwmalloctype, &mut result__).from_abi(result__)
    }
    pub unsafe fn VirtualAlloc(&self, paddress: *mut ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VirtualAlloc)(::windows::core::Interface::as_raw(self), paddress, dwsize, flallocationtype, flprotect, ecriticallevel, ppmem).ok()
    }
    pub unsafe fn VirtualFree(&self, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VirtualFree)(::windows::core::Interface::as_raw(self), lpaddress, dwsize, dwfreetype).ok()
    }
    pub unsafe fn VirtualQuery(&self, lpaddress: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VirtualQuery)(::windows::core::Interface::as_raw(self), lpaddress, lpbuffer, dwlength, presult).ok()
    }
    pub unsafe fn VirtualProtect(&self, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VirtualProtect)(::windows::core::Interface::as_raw(self), lpaddress, dwsize, flnewprotect, pfloldprotect).ok()
    }
    pub unsafe fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMemoryLoad)(::windows::core::Interface::as_raw(self), pmemoryload, pavailablebytes).ok()
    }
    pub unsafe fn RegisterMemoryNotificationCallback<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICLRMemoryNotificationCallback>,
    {
        (::windows::core::Interface::vtable(self).RegisterMemoryNotificationCallback)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn NeedsVirtualAddressSpace(&self, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NeedsVirtualAddressSpace)(::windows::core::Interface::as_raw(self), startaddress, size).ok()
    }
    pub unsafe fn AcquiredVirtualAddressSpace(&self, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquiredVirtualAddressSpace)(::windows::core::Interface::as_raw(self), startaddress, size).ok()
    }
    pub unsafe fn ReleasedVirtualAddressSpace(&self, startaddress: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleasedVirtualAddressSpace)(::windows::core::Interface::as_raw(self), startaddress).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostMemoryManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostMemoryManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostMemoryManager {}
impl ::core::fmt::Debug for IHostMemoryManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostMemoryManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostMemoryManager {
    type Vtable = IHostMemoryManager_Vtbl;
}
impl ::core::clone::Clone for IHostMemoryManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostMemoryManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bc698d1_f9e3_4460_9cde_d04248e9fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostMemoryManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmalloctype: u32, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VirtualAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VirtualFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows::core::HRESULT,
    pub VirtualQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpaddress: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows::core::HRESULT,
    pub VirtualProtect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> ::windows::core::HRESULT,
    pub GetMemoryLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows::core::HRESULT,
    pub RegisterMemoryNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NeedsVirtualAddressSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::HRESULT,
    pub AcquiredVirtualAddressSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::HRESULT,
    pub ReleasedVirtualAddressSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startaddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostPolicyManager(::windows::core::IUnknown);
impl IHostPolicyManager {
    pub unsafe fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDefaultAction)(::windows::core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTimeout)(::windows::core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnFailure)(::windows::core::Interface::as_raw(self), failure, action).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostPolicyManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostPolicyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostPolicyManager {}
impl ::core::fmt::Debug for IHostPolicyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostPolicyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostPolicyManager {
    type Vtable = IHostPolicyManager_Vtbl;
}
impl ::core::clone::Clone for IHostPolicyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostPolicyManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ae49844_b1e3_4683_ba7c_1e8212ea3b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostPolicyManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT,
    pub OnTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT,
    pub OnFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostSecurityContext(::windows::core::IUnknown);
impl IHostSecurityContext {
    pub unsafe fn Capture(&self) -> ::windows::core::Result<IHostSecurityContext> {
        let mut result__ = ::windows::core::zeroed::<IHostSecurityContext>();
        (::windows::core::Interface::vtable(self).Capture)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IHostSecurityContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostSecurityContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostSecurityContext {}
impl ::core::fmt::Debug for IHostSecurityContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostSecurityContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostSecurityContext {
    type Vtable = IHostSecurityContext_Vtbl;
}
impl ::core::clone::Clone for IHostSecurityContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostSecurityContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e573ce4_0343_4423_98d7_6318348a1d3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSecurityContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Capture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclonedcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostSecurityManager(::windows::core::IUnknown);
impl IHostSecurityManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImpersonateLoggedOnUser<P0>(&self, htoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).ImpersonateLoggedOnUser)(::windows::core::Interface::as_raw(self), htoken.into_param().abi()).ok()
    }
    pub unsafe fn RevertToSelf(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevertToSelf)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenThreadToken<P0>(&self, dwdesiredaccess: u32, bopenasself: P0, phthreadtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OpenThreadToken)(::windows::core::Interface::as_raw(self), dwdesiredaccess, bopenasself.into_param().abi(), phthreadtoken).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetThreadToken<P0>(&self, htoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).SetThreadToken)(::windows::core::Interface::as_raw(self), htoken.into_param().abi()).ok()
    }
    pub unsafe fn GetSecurityContext(&self, econtexttype: EContextType) -> ::windows::core::Result<IHostSecurityContext> {
        let mut result__ = ::windows::core::zeroed::<IHostSecurityContext>();
        (::windows::core::Interface::vtable(self).GetSecurityContext)(::windows::core::Interface::as_raw(self), econtexttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSecurityContext<P0>(&self, econtexttype: EContextType, psecuritycontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IHostSecurityContext>,
    {
        (::windows::core::Interface::vtable(self).SetSecurityContext)(::windows::core::Interface::as_raw(self), econtexttype, psecuritycontext.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostSecurityManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostSecurityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostSecurityManager {}
impl ::core::fmt::Debug for IHostSecurityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostSecurityManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostSecurityManager {
    type Vtable = IHostSecurityManager_Vtbl;
}
impl ::core::clone::Clone for IHostSecurityManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostSecurityManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75ad2468_a349_4d02_a764_76a68aee0c4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSecurityManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ImpersonateLoggedOnUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImpersonateLoggedOnUser: usize,
    pub RevertToSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenThreadToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenThreadToken: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetThreadToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetThreadToken: usize,
    pub GetSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, econtexttype: EContextType, ppsecuritycontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, econtexttype: EContextType, psecuritycontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostSemaphore(::windows::core::IUnknown);
impl IHostSemaphore {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::windows::core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn ReleaseSemaphore(&self, lreleasecount: i32, lppreviouscount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseSemaphore)(::windows::core::Interface::as_raw(self), lreleasecount, lppreviouscount).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostSemaphore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostSemaphore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostSemaphore {}
impl ::core::fmt::Debug for IHostSemaphore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostSemaphore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostSemaphore {
    type Vtable = IHostSemaphore_Vtbl;
}
impl ::core::clone::Clone for IHostSemaphore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostSemaphore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x855efd47_cc09_463a_a97d_16acab882661);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSemaphore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT,
    pub ReleaseSemaphore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lreleasecount: i32, lppreviouscount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostSyncManager(::windows::core::IUnknown);
impl IHostSyncManager {
    pub unsafe fn SetCLRSyncManager<P0>(&self, pmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICLRSyncManager>,
    {
        (::windows::core::Interface::vtable(self).SetCLRSyncManager)(::windows::core::Interface::as_raw(self), pmanager.into_param().abi()).ok()
    }
    pub unsafe fn CreateCrst(&self) -> ::windows::core::Result<IHostCrst> {
        let mut result__ = ::windows::core::zeroed::<IHostCrst>();
        (::windows::core::Interface::vtable(self).CreateCrst)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> ::windows::core::Result<IHostCrst> {
        let mut result__ = ::windows::core::zeroed::<IHostCrst>();
        (::windows::core::Interface::vtable(self).CreateCrstWithSpinCount)(::windows::core::Interface::as_raw(self), dwspincount, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAutoEvent(&self) -> ::windows::core::Result<IHostAutoEvent> {
        let mut result__ = ::windows::core::zeroed::<IHostAutoEvent>();
        (::windows::core::Interface::vtable(self).CreateAutoEvent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateManualEvent<P0>(&self, binitialstate: P0) -> ::windows::core::Result<IHostManualEvent>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IHostManualEvent>();
        (::windows::core::Interface::vtable(self).CreateManualEvent)(::windows::core::Interface::as_raw(self), binitialstate.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMonitorEvent(&self, cookie: usize) -> ::windows::core::Result<IHostAutoEvent> {
        let mut result__ = ::windows::core::zeroed::<IHostAutoEvent>();
        (::windows::core::Interface::vtable(self).CreateMonitorEvent)(::windows::core::Interface::as_raw(self), cookie, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRWLockWriterEvent(&self, cookie: usize) -> ::windows::core::Result<IHostAutoEvent> {
        let mut result__ = ::windows::core::zeroed::<IHostAutoEvent>();
        (::windows::core::Interface::vtable(self).CreateRWLockWriterEvent)(::windows::core::Interface::as_raw(self), cookie, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRWLockReaderEvent<P0>(&self, binitialstate: P0, cookie: usize) -> ::windows::core::Result<IHostManualEvent>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IHostManualEvent>();
        (::windows::core::Interface::vtable(self).CreateRWLockReaderEvent)(::windows::core::Interface::as_raw(self), binitialstate.into_param().abi(), cookie, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> ::windows::core::Result<IHostSemaphore> {
        let mut result__ = ::windows::core::zeroed::<IHostSemaphore>();
        (::windows::core::Interface::vtable(self).CreateSemaphoreA)(::windows::core::Interface::as_raw(self), dwinitial, dwmax, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IHostSyncManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostSyncManager {}
impl ::core::fmt::Debug for IHostSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostSyncManager {
    type Vtable = IHostSyncManager_Vtbl;
}
impl ::core::clone::Clone for IHostSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostSyncManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x234330c7_5f10_4f20_9615_5122dab7a0ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSyncManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetCLRSyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCrst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCrstWithSpinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwspincount: u32, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAutoEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateManualEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateManualEvent: usize,
    pub CreateMonitorEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRWLockWriterEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRWLockReaderEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRWLockReaderEvent: usize,
    pub CreateSemaphoreA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinitial: u32, dwmax: u32, ppsemaphore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostTask(::windows::core::IUnknown);
impl IHostTask {
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Alert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Alert)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Join(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Join)(::windows::core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn SetPriority(&self, newpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), newpriority).ok()
    }
    pub unsafe fn GetPriority(&self, ppriority: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPriority)(::windows::core::Interface::as_raw(self), ppriority).ok()
    }
    pub unsafe fn SetCLRTask<P0>(&self, pclrtask: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICLRTask>,
    {
        (::windows::core::Interface::vtable(self).SetCLRTask)(::windows::core::Interface::as_raw(self), pclrtask.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostTask, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostTask {}
impl ::core::fmt::Debug for IHostTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostTask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostTask {
    type Vtable = IHostTask_Vtbl;
}
impl ::core::clone::Clone for IHostTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostTask {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2275828_c4b1_4b55_82c9_92135f74df1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostTask_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Alert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Join: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newpriority: i32) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetCLRTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclrtask: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostTaskManager(::windows::core::IUnknown);
impl IHostTaskManager {
    pub unsafe fn GetCurrentTask(&self) -> ::windows::core::Result<IHostTask> {
        let mut result__ = ::windows::core::zeroed::<IHostTask>();
        (::windows::core::Interface::vtable(self).GetCurrentTask)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Threading\"`*"]
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *mut ::core::ffi::c_void, pptask: *mut ::core::option::Option<IHostTask>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateTask)(::windows::core::Interface::as_raw(self), dwstacksize, pstartaddress, pparameter, ::core::mem::transmute(pptask)).ok()
    }
    pub unsafe fn Sleep(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Sleep)(::windows::core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn SwitchToTask(&self, option: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SwitchToTask)(::windows::core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn SetUILocale(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUILocale)(::windows::core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLocale(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLocale)(::windows::core::Interface::as_raw(self), lcid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallNeedsHostHook(&self, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CallNeedsHostHook)(::windows::core::Interface::as_raw(self), target, pbcallneedshosthook).ok()
    }
    pub unsafe fn LeaveRuntime(&self, target: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LeaveRuntime)(::windows::core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn EnterRuntime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnterRuntime)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReverseLeaveRuntime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReverseLeaveRuntime)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReverseEnterRuntime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReverseEnterRuntime)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginDelayAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginDelayAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndDelayAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndDelayAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginThreadAffinity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginThreadAffinity)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndThreadAffinity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndThreadAffinity)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStackGuarantee(&self, guarantee: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStackGuarantee)(::windows::core::Interface::as_raw(self), guarantee).ok()
    }
    pub unsafe fn GetStackGuarantee(&self, pguarantee: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStackGuarantee)(::windows::core::Interface::as_raw(self), pguarantee).ok()
    }
    pub unsafe fn SetCLRTaskManager<P0>(&self, ppmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICLRTaskManager>,
    {
        (::windows::core::Interface::vtable(self).SetCLRTaskManager)(::windows::core::Interface::as_raw(self), ppmanager.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostTaskManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostTaskManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostTaskManager {}
impl ::core::fmt::Debug for IHostTaskManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostTaskManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostTaskManager {
    type Vtable = IHostTaskManager_Vtbl;
}
impl ::core::clone::Clone for IHostTaskManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostTaskManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997ff24c_43b7_4352_8667_0dc04fafd354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostTaskManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Threading")]
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *mut ::core::ffi::c_void, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CreateTask: usize,
    pub Sleep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT,
    pub SwitchToTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: u32) -> ::windows::core::HRESULT,
    pub SetUILocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CallNeedsHostHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallNeedsHostHook: usize,
    pub LeaveRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: usize) -> ::windows::core::HRESULT,
    pub EnterRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReverseLeaveRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReverseEnterRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginDelayAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndDelayAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginThreadAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndThreadAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStackGuarantee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guarantee: u32) -> ::windows::core::HRESULT,
    pub GetStackGuarantee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguarantee: *mut u32) -> ::windows::core::HRESULT,
    pub SetCLRTaskManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IHostThreadpoolManager(::windows::core::IUnknown);
impl IHostThreadpoolManager {
    #[doc = "*Required features: `\"Win32_System_Threading\"`*"]
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueueUserWorkItem)(::windows::core::Interface::as_raw(self), function, context, flags).ok()
    }
    pub unsafe fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxThreads)(::windows::core::Interface::as_raw(self), dwmaxworkerthreads).ok()
    }
    pub unsafe fn GetMaxThreads(&self, pdwmaxworkerthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaxThreads)(::windows::core::Interface::as_raw(self), pdwmaxworkerthreads).ok()
    }
    pub unsafe fn GetAvailableThreads(&self, pdwavailableworkerthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAvailableThreads)(::windows::core::Interface::as_raw(self), pdwavailableworkerthreads).ok()
    }
    pub unsafe fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinThreads)(::windows::core::Interface::as_raw(self), dwminiocompletionthreads).ok()
    }
    pub unsafe fn GetMinThreads(&self, pdwminiocompletionthreads: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMinThreads)(::windows::core::Interface::as_raw(self), pdwminiocompletionthreads).ok()
    }
}
::windows::imp::interface_hierarchy!(IHostThreadpoolManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHostThreadpoolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostThreadpoolManager {}
impl ::core::fmt::Debug for IHostThreadpoolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostThreadpoolManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHostThreadpoolManager {
    type Vtable = IHostThreadpoolManager_Vtbl;
}
impl ::core::clone::Clone for IHostThreadpoolManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHostThreadpoolManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x983d50e2_cb15_466b_80fc_845dc6e8c5fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostThreadpoolManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Threading")]
    pub QueueUserWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    QueueUserWorkItem: usize,
    pub SetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxworkerthreads: u32) -> ::windows::core::HRESULT,
    pub GetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxworkerthreads: *mut u32) -> ::windows::core::HRESULT,
    pub GetAvailableThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwavailableworkerthreads: *mut u32) -> ::windows::core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows::core::HRESULT,
    pub GetMinThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IManagedObject(::windows::core::IUnknown);
impl IManagedObject {
    pub unsafe fn GetSerializedBuffer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetSerializedBuffer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectIdentity(&self, pbstrguid: *mut ::windows::core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectIdentity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrguid), appdomainid, pccw).ok()
    }
}
::windows::imp::interface_hierarchy!(IManagedObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IManagedObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedObject {}
impl ::core::fmt::Debug for IManagedObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IManagedObject {
    type Vtable = IManagedObject_Vtbl;
}
impl ::core::clone::Clone for IManagedObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IManagedObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3fcc19e_a970_11d2_8b5a_00a0c9b7c9c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSerializedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetObjectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, appdomainid: *mut i32, pccw: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct IObjectHandle(::windows::core::IUnknown);
impl IObjectHandle {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Unwrap(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Unwrap)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IObjectHandle, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IObjectHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectHandle {}
impl ::core::fmt::Debug for IObjectHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectHandle {
    type Vtable = IObjectHandle_Vtbl;
}
impl ::core::clone::Clone for IObjectHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IObjectHandle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc460e2b4_e199_412a_8456_84dc3e4838c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectHandle_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Unwrap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppv: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Unwrap: usize,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ITypeName(::windows::core::IUnknown);
impl ITypeName {
    pub unsafe fn GetNameCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNameCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNames(&self, count: u32, rgbsznames: *mut ::windows::core::BSTR, pcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNames)(::windows::core::Interface::as_raw(self), count, ::core::mem::transmute(rgbsznames), pcount).ok()
    }
    pub unsafe fn GetTypeArgumentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetTypeArgumentCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeArguments(&self, count: u32, rgparguments: *mut ::core::option::Option<ITypeName>, pcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTypeArguments)(::windows::core::Interface::as_raw(self), count, ::core::mem::transmute(rgparguments), pcount).ok()
    }
    pub unsafe fn GetModifierLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetModifierLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetModifiers)(::windows::core::Interface::as_raw(self), count, rgmodifiers, pcount).ok()
    }
    pub unsafe fn GetAssemblyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetAssemblyName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITypeName, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITypeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeName {}
impl ::core::fmt::Debug for ITypeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeName {
    type Vtable = ITypeName_Vtbl;
}
impl ::core::clone::Clone for ITypeName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITypeName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeName_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, rgbsznames: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeArgumentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, rgparguments: *mut *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetModifierLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetAssemblyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgbszassemblynames: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ITypeNameBuilder(::windows::core::IUnknown);
impl ITypeNameBuilder {
    pub unsafe fn OpenGenericArguments(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenGenericArguments)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseGenericArguments(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseGenericArguments)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenGenericArgument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenGenericArgument)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseGenericArgument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseGenericArgument)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddName<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddName)(::windows::core::Interface::as_raw(self), szname.into_param().abi()).ok()
    }
    pub unsafe fn AddPointer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPointer)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddByRef(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddByRef)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddSzArray(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSzArray)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddArray(&self, rank: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddArray)(::windows::core::Interface::as_raw(self), rank).ok()
    }
    pub unsafe fn AddAssemblySpec<P0>(&self, szassemblyspec: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAssemblySpec)(::windows::core::Interface::as_raw(self), szassemblyspec.into_param().abi()).ok()
    }
    pub unsafe fn ToString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ToString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITypeNameBuilder, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITypeNameBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeNameBuilder {}
impl ::core::fmt::Debug for ITypeNameBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeNameBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeNameBuilder {
    type Vtable = ITypeNameBuilder_Vtbl;
}
impl ::core::clone::Clone for ITypeNameBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITypeNameBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00523);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeNameBuilder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenGenericArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseGenericArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenGenericArgument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseGenericArgument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddByRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddSzArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rank: u32) -> ::windows::core::HRESULT,
    pub AddAssemblySpec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szassemblyspec: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstringrepresentation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
pub struct ITypeNameFactory(::windows::core::IUnknown);
impl ITypeNameFactory {
    pub unsafe fn ParseTypeName<P0>(&self, szname: P0, perror: *mut u32, pptypename: *mut ::core::option::Option<ITypeName>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ParseTypeName)(::windows::core::Interface::as_raw(self), szname.into_param().abi(), perror, ::core::mem::transmute(pptypename)).ok()
    }
    pub unsafe fn GetTypeNameBuilder(&self) -> ::windows::core::Result<ITypeNameBuilder> {
        let mut result__ = ::windows::core::zeroed::<ITypeNameBuilder>();
        (::windows::core::Interface::vtable(self).GetTypeNameBuilder)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITypeNameFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITypeNameFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeNameFactory {}
impl ::core::fmt::Debug for ITypeNameFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeNameFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeNameFactory {
    type Vtable = ITypeNameFactory_Vtbl;
}
impl ::core::clone::Clone for ITypeNameFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITypeNameFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00521);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeNameFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ParseTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, perror: *mut u32, pptypename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTypeNameBuilder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const BucketParamLength: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const BucketParamsCount: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLRRuntimeHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90f1a06e_7712_4762_86b5_7a5eba6bdb02);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_ASSEMBLY_BUILD_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_ASSEMBLY_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_ASSEMBLY_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_BUILD_VERSION: u32 = 30319u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_CLRDebugging: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbacc578d_fbdd_48a4_969f_02d932b74634);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_CLRDebuggingLegacy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf8395b5_a4ba_450b_a77c_a9a47762c520);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_CLRMetaHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9280188d_0e8e_4867_b30c_7fa83884e8de);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_CLRMetaHostPolicy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ebcd49a_1b47_4a61_b13a_4a03701e594b);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_CLRProfiling: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd097ed8_733e_43fe_8ed7_a95ff9a8448c);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_CLRStrongName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb79b0acd_f5cd_409b_b5a5_a16244610b92);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ComCallUnmarshal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f281000_e95a_11d2_886b_00c04f869f04);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ComCallUnmarshalV4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45fb4600_e6e8_4928_b25e_50476ff79425);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CorRuntimeHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb2f6723_ab3a_11d2_9c40_00c04fa30a3e);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const DEPRECATED_CLR_API_MESG: ::windows::core::PCSTR = ::windows::core::s!("This API has been deprecated. Refer to https://go.microsoft.com/fwlink/?LinkId=143720 for more details.");
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const LIBID_mscoree: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5477469e_83b1_11d2_8b49_00a0c9b7c9c4);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TypeNameFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00525);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPDOMAIN_SECURITY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const APPDOMAIN_SECURITY_DEFAULT: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const APPDOMAIN_SECURITY_SANDBOXED: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const APPDOMAIN_SECURITY_FORBID_CROSSAD_REVERSE_PINVOKE: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const APPDOMAIN_FORCE_TRIVIAL_WAIT_OPERATIONS: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(8i32);
impl ::core::marker::Copy for APPDOMAIN_SECURITY_FLAGS {}
impl ::core::clone::Clone for APPDOMAIN_SECURITY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPDOMAIN_SECURITY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APPDOMAIN_SECURITY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APPDOMAIN_SECURITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPDOMAIN_SECURITY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BucketParameterIndex(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter1: BucketParameterIndex = BucketParameterIndex(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter2: BucketParameterIndex = BucketParameterIndex(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter3: BucketParameterIndex = BucketParameterIndex(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter4: BucketParameterIndex = BucketParameterIndex(3i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter5: BucketParameterIndex = BucketParameterIndex(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter6: BucketParameterIndex = BucketParameterIndex(5i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter7: BucketParameterIndex = BucketParameterIndex(6i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter8: BucketParameterIndex = BucketParameterIndex(7i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Parameter9: BucketParameterIndex = BucketParameterIndex(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const InvalidBucketParamIndex: BucketParameterIndex = BucketParameterIndex(9i32);
impl ::core::marker::Copy for BucketParameterIndex {}
impl ::core::clone::Clone for BucketParameterIndex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BucketParameterIndex {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BucketParameterIndex {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BucketParameterIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BucketParameterIndex").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLR_DEBUGGING_PROCESS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_DEBUGGING_MANAGED_EVENT_PENDING: CLR_DEBUGGING_PROCESS_FLAGS = CLR_DEBUGGING_PROCESS_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_DEBUGGING_MANAGED_EVENT_DEBUGGER_LAUNCH: CLR_DEBUGGING_PROCESS_FLAGS = CLR_DEBUGGING_PROCESS_FLAGS(2i32);
impl ::core::marker::Copy for CLR_DEBUGGING_PROCESS_FLAGS {}
impl ::core::clone::Clone for CLR_DEBUGGING_PROCESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLR_DEBUGGING_PROCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLR_DEBUGGING_PROCESS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLR_DEBUGGING_PROCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLR_DEBUGGING_PROCESS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLSID_RESOLUTION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_RESOLUTION_DEFAULT: CLSID_RESOLUTION_FLAGS = CLSID_RESOLUTION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLSID_RESOLUTION_REGISTERED: CLSID_RESOLUTION_FLAGS = CLSID_RESOLUTION_FLAGS(1i32);
impl ::core::marker::Copy for CLSID_RESOLUTION_FLAGS {}
impl ::core::clone::Clone for CLSID_RESOLUTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLSID_RESOLUTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLSID_RESOLUTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLSID_RESOLUTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLSID_RESOLUTION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_GC_STAT_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const COR_GC_COUNTS: COR_GC_STAT_TYPES = COR_GC_STAT_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const COR_GC_MEMORYUSAGE: COR_GC_STAT_TYPES = COR_GC_STAT_TYPES(2i32);
impl ::core::marker::Copy for COR_GC_STAT_TYPES {}
impl ::core::clone::Clone for COR_GC_STAT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_GC_STAT_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_GC_STAT_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_GC_STAT_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_GC_STAT_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_GC_THREAD_STATS_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const COR_GC_THREAD_HAS_PROMOTED_BYTES: COR_GC_THREAD_STATS_TYPES = COR_GC_THREAD_STATS_TYPES(1i32);
impl ::core::marker::Copy for COR_GC_THREAD_STATS_TYPES {}
impl ::core::clone::Clone for COR_GC_THREAD_STATS_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_GC_THREAD_STATS_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_GC_THREAD_STATS_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_GC_THREAD_STATS_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_GC_THREAD_STATS_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EApiCategories(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eNoChecks: EApiCategories = EApiCategories(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSynchronization: EApiCategories = EApiCategories(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSharedState: EApiCategories = EApiCategories(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eExternalProcessMgmt: EApiCategories = EApiCategories(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSelfAffectingProcessMgmt: EApiCategories = EApiCategories(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eExternalThreading: EApiCategories = EApiCategories(16i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSelfAffectingThreading: EApiCategories = EApiCategories(32i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSecurityInfrastructure: EApiCategories = EApiCategories(64i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eUI: EApiCategories = EApiCategories(128i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eMayLeakOnAbort: EApiCategories = EApiCategories(256i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eAll: EApiCategories = EApiCategories(511i32);
impl ::core::marker::Copy for EApiCategories {}
impl ::core::clone::Clone for EApiCategories {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EApiCategories {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EApiCategories {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EApiCategories {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EApiCategories").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EBindPolicyLevels(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyLevelNone: EBindPolicyLevels = EBindPolicyLevels(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyLevelRetargetable: EBindPolicyLevels = EBindPolicyLevels(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyUnifiedToCLR: EBindPolicyLevels = EBindPolicyLevels(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyLevelApp: EBindPolicyLevels = EBindPolicyLevels(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyLevelPublisher: EBindPolicyLevels = EBindPolicyLevels(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyLevelHost: EBindPolicyLevels = EBindPolicyLevels(16i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyLevelAdmin: EBindPolicyLevels = EBindPolicyLevels(32i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const ePolicyPortability: EBindPolicyLevels = EBindPolicyLevels(64i32);
impl ::core::marker::Copy for EBindPolicyLevels {}
impl ::core::clone::Clone for EBindPolicyLevels {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EBindPolicyLevels {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EBindPolicyLevels {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EBindPolicyLevels {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EBindPolicyLevels").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ECLRAssemblyIdentityFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const CLR_ASSEMBLY_IDENTITY_FLAGS_DEFAULT: ECLRAssemblyIdentityFlags = ECLRAssemblyIdentityFlags(0i32);
impl ::core::marker::Copy for ECLRAssemblyIdentityFlags {}
impl ::core::clone::Clone for ECLRAssemblyIdentityFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ECLRAssemblyIdentityFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ECLRAssemblyIdentityFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ECLRAssemblyIdentityFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ECLRAssemblyIdentityFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EClrEvent(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Event_DomainUnload: EClrEvent = EClrEvent(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Event_ClrDisabled: EClrEvent = EClrEvent(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Event_MDAFired: EClrEvent = EClrEvent(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const Event_StackOverflow: EClrEvent = EClrEvent(3i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const MaxClrEvent: EClrEvent = EClrEvent(4i32);
impl ::core::marker::Copy for EClrEvent {}
impl ::core::clone::Clone for EClrEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EClrEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EClrEvent {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EClrEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EClrEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EClrFailure(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_NonCriticalResource: EClrFailure = EClrFailure(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_CriticalResource: EClrFailure = EClrFailure(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_FatalRuntime: EClrFailure = EClrFailure(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_OrphanedLock: EClrFailure = EClrFailure(3i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_StackOverflow: EClrFailure = EClrFailure(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_AccessViolation: EClrFailure = EClrFailure(5i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const FAIL_CodeContract: EClrFailure = EClrFailure(6i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const MaxClrFailure: EClrFailure = EClrFailure(7i32);
impl ::core::marker::Copy for EClrFailure {}
impl ::core::clone::Clone for EClrFailure {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EClrFailure {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EClrFailure {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EClrFailure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EClrFailure").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EClrOperation(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_ThreadAbort: EClrOperation = EClrOperation(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_ThreadRudeAbortInNonCriticalRegion: EClrOperation = EClrOperation(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_ThreadRudeAbortInCriticalRegion: EClrOperation = EClrOperation(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_AppDomainUnload: EClrOperation = EClrOperation(3i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_AppDomainRudeUnload: EClrOperation = EClrOperation(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_ProcessExit: EClrOperation = EClrOperation(5i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const OPR_FinalizerRun: EClrOperation = EClrOperation(6i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const MaxClrOperation: EClrOperation = EClrOperation(7i32);
impl ::core::marker::Copy for EClrOperation {}
impl ::core::clone::Clone for EClrOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EClrOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EClrOperation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EClrOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EClrOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EClrUnhandledException(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eRuntimeDeterminedPolicy: EClrUnhandledException = EClrUnhandledException(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eHostDeterminedPolicy: EClrUnhandledException = EClrUnhandledException(1i32);
impl ::core::marker::Copy for EClrUnhandledException {}
impl ::core::clone::Clone for EClrUnhandledException {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EClrUnhandledException {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EClrUnhandledException {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EClrUnhandledException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EClrUnhandledException").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EContextType(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eCurrentContext: EContextType = EContextType(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eRestrictedContext: EContextType = EContextType(1i32);
impl ::core::marker::Copy for EContextType {}
impl ::core::clone::Clone for EContextType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EContextType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EContextType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EContextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EContextType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ECustomDumpFlavor(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const DUMP_FLAVOR_Mini: ECustomDumpFlavor = ECustomDumpFlavor(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const DUMP_FLAVOR_CriticalCLRState: ECustomDumpFlavor = ECustomDumpFlavor(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const DUMP_FLAVOR_NonHeapCLRState: ECustomDumpFlavor = ECustomDumpFlavor(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const DUMP_FLAVOR_Default: ECustomDumpFlavor = ECustomDumpFlavor(0i32);
impl ::core::marker::Copy for ECustomDumpFlavor {}
impl ::core::clone::Clone for ECustomDumpFlavor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ECustomDumpFlavor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ECustomDumpFlavor {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ECustomDumpFlavor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ECustomDumpFlavor").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ECustomDumpItemKind(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const DUMP_ITEM_None: ECustomDumpItemKind = ECustomDumpItemKind(0i32);
impl ::core::marker::Copy for ECustomDumpItemKind {}
impl ::core::clone::Clone for ECustomDumpItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ECustomDumpItemKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ECustomDumpItemKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ECustomDumpItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ECustomDumpItemKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EHostApplicationPolicy(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_APPLICATION_BINDING_POLICY: EHostApplicationPolicy = EHostApplicationPolicy(1i32);
impl ::core::marker::Copy for EHostApplicationPolicy {}
impl ::core::clone::Clone for EHostApplicationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EHostApplicationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EHostApplicationPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EHostApplicationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EHostApplicationPolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EHostBindingPolicyModifyFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_BINDING_POLICY_MODIFY_DEFAULT: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_BINDING_POLICY_MODIFY_CHAIN: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_BINDING_POLICY_MODIFY_REMOVE: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_BINDING_POLICY_MODIFY_MAX: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(3i32);
impl ::core::marker::Copy for EHostBindingPolicyModifyFlags {}
impl ::core::clone::Clone for EHostBindingPolicyModifyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EHostBindingPolicyModifyFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EHostBindingPolicyModifyFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EHostBindingPolicyModifyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EHostBindingPolicyModifyFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EInitializeNewDomainFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eInitializeNewDomainFlags_None: EInitializeNewDomainFlags = EInitializeNewDomainFlags(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eInitializeNewDomainFlags_NoSecurityChanges: EInitializeNewDomainFlags = EInitializeNewDomainFlags(2i32);
impl ::core::marker::Copy for EInitializeNewDomainFlags {}
impl ::core::clone::Clone for EInitializeNewDomainFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EInitializeNewDomainFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EInitializeNewDomainFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EInitializeNewDomainFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EInitializeNewDomainFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EMemoryAvailable(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eMemoryAvailableLow: EMemoryAvailable = EMemoryAvailable(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eMemoryAvailableNeutral: EMemoryAvailable = EMemoryAvailable(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eMemoryAvailableHigh: EMemoryAvailable = EMemoryAvailable(3i32);
impl ::core::marker::Copy for EMemoryAvailable {}
impl ::core::clone::Clone for EMemoryAvailable {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMemoryAvailable {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EMemoryAvailable {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EMemoryAvailable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMemoryAvailable").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EMemoryCriticalLevel(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eTaskCritical: EMemoryCriticalLevel = EMemoryCriticalLevel(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eAppDomainCritical: EMemoryCriticalLevel = EMemoryCriticalLevel(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eProcessCritical: EMemoryCriticalLevel = EMemoryCriticalLevel(2i32);
impl ::core::marker::Copy for EMemoryCriticalLevel {}
impl ::core::clone::Clone for EMemoryCriticalLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMemoryCriticalLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EMemoryCriticalLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EMemoryCriticalLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMemoryCriticalLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPolicyAction(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eNoAction: EPolicyAction = EPolicyAction(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eThrowException: EPolicyAction = EPolicyAction(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eAbortThread: EPolicyAction = EPolicyAction(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eRudeAbortThread: EPolicyAction = EPolicyAction(3i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eUnloadAppDomain: EPolicyAction = EPolicyAction(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eRudeUnloadAppDomain: EPolicyAction = EPolicyAction(5i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eExitProcess: EPolicyAction = EPolicyAction(6i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eFastExitProcess: EPolicyAction = EPolicyAction(7i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eRudeExitProcess: EPolicyAction = EPolicyAction(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eDisableRuntime: EPolicyAction = EPolicyAction(9i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const MaxPolicyAction: EPolicyAction = EPolicyAction(10i32);
impl ::core::marker::Copy for EPolicyAction {}
impl ::core::clone::Clone for EPolicyAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPolicyAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EPolicyAction {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EPolicyAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPolicyAction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESymbolReadingPolicy(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSymbolReadingNever: ESymbolReadingPolicy = ESymbolReadingPolicy(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSymbolReadingAlways: ESymbolReadingPolicy = ESymbolReadingPolicy(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const eSymbolReadingFullTrustOnly: ESymbolReadingPolicy = ESymbolReadingPolicy(2i32);
impl ::core::marker::Copy for ESymbolReadingPolicy {}
impl ::core::clone::Clone for ESymbolReadingPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESymbolReadingPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ESymbolReadingPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ESymbolReadingPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESymbolReadingPolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETaskType(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_DEBUGGERHELPER: ETaskType = ETaskType(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_GC: ETaskType = ETaskType(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_FINALIZER: ETaskType = ETaskType(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_THREADPOOL_TIMER: ETaskType = ETaskType(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_THREADPOOL_GATE: ETaskType = ETaskType(16i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_THREADPOOL_WORKER: ETaskType = ETaskType(32i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_THREADPOOL_IOCOMPLETION: ETaskType = ETaskType(64i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_ADUNLOAD: ETaskType = ETaskType(128i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_USER: ETaskType = ETaskType(256i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_THREADPOOL_WAIT: ETaskType = ETaskType(512i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const TT_UNKNOWN: ETaskType = ETaskType(-2147483648i32);
impl ::core::marker::Copy for ETaskType {}
impl ::core::clone::Clone for ETaskType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETaskType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ETaskType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ETaskType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETaskType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HOST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_TYPE_DEFAULT: HOST_TYPE = HOST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_TYPE_APPLAUNCH: HOST_TYPE = HOST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const HOST_TYPE_CORFLAG: HOST_TYPE = HOST_TYPE(2i32);
impl ::core::marker::Copy for HOST_TYPE {}
impl ::core::clone::Clone for HOST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HOST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HOST_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HOST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MALLOC_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const MALLOC_THREADSAFE: MALLOC_TYPE = MALLOC_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const MALLOC_EXECUTABLE: MALLOC_TYPE = MALLOC_TYPE(2i32);
impl ::core::marker::Copy for MALLOC_TYPE {}
impl ::core::clone::Clone for MALLOC_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MALLOC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MALLOC_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MALLOC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MALLOC_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct METAHOST_CONFIG_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_UNSET: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_TRUE: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_FALSE: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_MASK: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(3i32);
impl ::core::marker::Copy for METAHOST_CONFIG_FLAGS {}
impl ::core::clone::Clone for METAHOST_CONFIG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for METAHOST_CONFIG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for METAHOST_CONFIG_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for METAHOST_CONFIG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("METAHOST_CONFIG_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct METAHOST_POLICY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_HIGHCOMPAT: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_APPLY_UPGRADE_POLICY: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_EMULATE_EXE_LAUNCH: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_SHOW_ERROR_DIALOG: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_USE_PROCESS_IMAGE_PATH: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_ENSURE_SKU_SUPPORTED: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const METAHOST_POLICY_IGNORE_ERROR_MODE: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(4096i32);
impl ::core::marker::Copy for METAHOST_POLICY_FLAGS {}
impl ::core::clone::Clone for METAHOST_POLICY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for METAHOST_POLICY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for METAHOST_POLICY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for METAHOST_POLICY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("METAHOST_POLICY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RUNTIME_INFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_UPGRADE_VERSION: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_REQUEST_IA64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_REQUEST_AMD64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_REQUEST_X86: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_DONT_RETURN_DIRECTORY: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_DONT_RETURN_VERSION: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_DONT_SHOW_ERROR_DIALOG: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const RUNTIME_INFO_IGNORE_ERROR_MODE: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(4096i32);
impl ::core::marker::Copy for RUNTIME_INFO_FLAGS {}
impl ::core::clone::Clone for RUNTIME_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RUNTIME_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RUNTIME_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RUNTIME_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RUNTIME_INFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STARTUP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_CONCURRENT_GC: STARTUP_FLAGS = STARTUP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LOADER_OPTIMIZATION_MASK: STARTUP_FLAGS = STARTUP_FLAGS(6i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LOADER_OPTIMIZATION_SINGLE_DOMAIN: STARTUP_FLAGS = STARTUP_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN: STARTUP_FLAGS = STARTUP_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN_HOST: STARTUP_FLAGS = STARTUP_FLAGS(6i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LOADER_SAFEMODE: STARTUP_FLAGS = STARTUP_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LOADER_SETPREFERENCE: STARTUP_FLAGS = STARTUP_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_SERVER_GC: STARTUP_FLAGS = STARTUP_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_HOARD_GC_VM: STARTUP_FLAGS = STARTUP_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_SINGLE_VERSION_HOSTING_INTERFACE: STARTUP_FLAGS = STARTUP_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_LEGACY_IMPERSONATION: STARTUP_FLAGS = STARTUP_FLAGS(65536i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_DISABLE_COMMITTHREADSTACK: STARTUP_FLAGS = STARTUP_FLAGS(131072i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_ALWAYSFLOW_IMPERSONATION: STARTUP_FLAGS = STARTUP_FLAGS(262144i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_TRIM_GC_COMMIT: STARTUP_FLAGS = STARTUP_FLAGS(524288i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_ETW: STARTUP_FLAGS = STARTUP_FLAGS(1048576i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const STARTUP_ARM: STARTUP_FLAGS = STARTUP_FLAGS(4194304i32);
impl ::core::marker::Copy for STARTUP_FLAGS {}
impl ::core::clone::Clone for STARTUP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STARTUP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STARTUP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STARTUP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTUP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StackOverflowType(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const SO_Managed: StackOverflowType = StackOverflowType(0i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const SO_ClrEngine: StackOverflowType = StackOverflowType(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const SO_Other: StackOverflowType = StackOverflowType(2i32);
impl ::core::marker::Copy for StackOverflowType {}
impl ::core::clone::Clone for StackOverflowType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StackOverflowType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StackOverflowType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StackOverflowType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StackOverflowType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WAIT_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const WAIT_MSGPUMP: WAIT_OPTION = WAIT_OPTION(1i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const WAIT_ALERTABLE: WAIT_OPTION = WAIT_OPTION(2i32);
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub const WAIT_NOTINDEADLOCK: WAIT_OPTION = WAIT_OPTION(4i32);
impl ::core::marker::Copy for WAIT_OPTION {}
impl ::core::clone::Clone for WAIT_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WAIT_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WAIT_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WAIT_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WAIT_OPTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct AssemblyBindInfo {
    pub dwAppDomainId: u32,
    pub lpReferencedIdentity: ::windows::core::PCWSTR,
    pub lpPostPolicyIdentity: ::windows::core::PCWSTR,
    pub ePolicyLevel: u32,
}
impl ::core::marker::Copy for AssemblyBindInfo {}
impl ::core::clone::Clone for AssemblyBindInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AssemblyBindInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AssemblyBindInfo").field("dwAppDomainId", &self.dwAppDomainId).field("lpReferencedIdentity", &self.lpReferencedIdentity).field("lpPostPolicyIdentity", &self.lpPostPolicyIdentity).field("ePolicyLevel", &self.ePolicyLevel).finish()
    }
}
impl ::windows::core::TypeKind for AssemblyBindInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AssemblyBindInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwAppDomainId == other.dwAppDomainId && self.lpReferencedIdentity == other.lpReferencedIdentity && self.lpPostPolicyIdentity == other.lpPostPolicyIdentity && self.ePolicyLevel == other.ePolicyLevel
    }
}
impl ::core::cmp::Eq for AssemblyBindInfo {}
impl ::core::default::Default for AssemblyBindInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BucketParameters {
    pub fInited: super::super::Foundation::BOOL,
    pub pszEventTypeName: [u16; 255],
    pub pszParams: [u16; 2550],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BucketParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BucketParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BucketParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BucketParameters").field("fInited", &self.fInited).field("pszEventTypeName", &self.pszEventTypeName).field("pszParams", &self.pszParams).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for BucketParameters {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BucketParameters {
    fn eq(&self, other: &Self) -> bool {
        self.fInited == other.fInited && self.pszEventTypeName == other.pszEventTypeName && self.pszParams == other.pszParams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BucketParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BucketParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct CLR_DEBUGGING_VERSION {
    pub wStructVersion: u16,
    pub wMajor: u16,
    pub wMinor: u16,
    pub wBuild: u16,
    pub wRevision: u16,
}
impl ::core::marker::Copy for CLR_DEBUGGING_VERSION {}
impl ::core::clone::Clone for CLR_DEBUGGING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLR_DEBUGGING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLR_DEBUGGING_VERSION").field("wStructVersion", &self.wStructVersion).field("wMajor", &self.wMajor).field("wMinor", &self.wMinor).field("wBuild", &self.wBuild).field("wRevision", &self.wRevision).finish()
    }
}
impl ::windows::core::TypeKind for CLR_DEBUGGING_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLR_DEBUGGING_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.wStructVersion == other.wStructVersion && self.wMajor == other.wMajor && self.wMinor == other.wMinor && self.wBuild == other.wBuild && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for CLR_DEBUGGING_VERSION {}
impl ::core::default::Default for CLR_DEBUGGING_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct COR_GC_STATS {
    pub Flags: u32,
    pub ExplicitGCCount: usize,
    pub GenCollectionsTaken: [usize; 3],
    pub CommittedKBytes: usize,
    pub ReservedKBytes: usize,
    pub Gen0HeapSizeKBytes: usize,
    pub Gen1HeapSizeKBytes: usize,
    pub Gen2HeapSizeKBytes: usize,
    pub LargeObjectHeapSizeKBytes: usize,
    pub KBytesPromotedFromGen0: usize,
    pub KBytesPromotedFromGen1: usize,
}
impl ::core::marker::Copy for COR_GC_STATS {}
impl ::core::clone::Clone for COR_GC_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_GC_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_GC_STATS")
            .field("Flags", &self.Flags)
            .field("ExplicitGCCount", &self.ExplicitGCCount)
            .field("GenCollectionsTaken", &self.GenCollectionsTaken)
            .field("CommittedKBytes", &self.CommittedKBytes)
            .field("ReservedKBytes", &self.ReservedKBytes)
            .field("Gen0HeapSizeKBytes", &self.Gen0HeapSizeKBytes)
            .field("Gen1HeapSizeKBytes", &self.Gen1HeapSizeKBytes)
            .field("Gen2HeapSizeKBytes", &self.Gen2HeapSizeKBytes)
            .field("LargeObjectHeapSizeKBytes", &self.LargeObjectHeapSizeKBytes)
            .field("KBytesPromotedFromGen0", &self.KBytesPromotedFromGen0)
            .field("KBytesPromotedFromGen1", &self.KBytesPromotedFromGen1)
            .finish()
    }
}
impl ::windows::core::TypeKind for COR_GC_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_GC_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.ExplicitGCCount == other.ExplicitGCCount && self.GenCollectionsTaken == other.GenCollectionsTaken && self.CommittedKBytes == other.CommittedKBytes && self.ReservedKBytes == other.ReservedKBytes && self.Gen0HeapSizeKBytes == other.Gen0HeapSizeKBytes && self.Gen1HeapSizeKBytes == other.Gen1HeapSizeKBytes && self.Gen2HeapSizeKBytes == other.Gen2HeapSizeKBytes && self.LargeObjectHeapSizeKBytes == other.LargeObjectHeapSizeKBytes && self.KBytesPromotedFromGen0 == other.KBytesPromotedFromGen0 && self.KBytesPromotedFromGen1 == other.KBytesPromotedFromGen1
    }
}
impl ::core::cmp::Eq for COR_GC_STATS {}
impl ::core::default::Default for COR_GC_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct COR_GC_THREAD_STATS {
    pub PerThreadAllocation: u64,
    pub Flags: u32,
}
impl ::core::marker::Copy for COR_GC_THREAD_STATS {}
impl ::core::clone::Clone for COR_GC_THREAD_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_GC_THREAD_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_GC_THREAD_STATS").field("PerThreadAllocation", &self.PerThreadAllocation).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for COR_GC_THREAD_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_GC_THREAD_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.PerThreadAllocation == other.PerThreadAllocation && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for COR_GC_THREAD_STATS {}
impl ::core::default::Default for COR_GC_THREAD_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct CustomDumpItem {
    pub itemKind: ECustomDumpItemKind,
    pub Anonymous: CustomDumpItem_0,
}
impl ::core::marker::Copy for CustomDumpItem {}
impl ::core::clone::Clone for CustomDumpItem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CustomDumpItem {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CustomDumpItem {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub union CustomDumpItem_0 {
    pub pReserved: usize,
}
impl ::core::marker::Copy for CustomDumpItem_0 {}
impl ::core::clone::Clone for CustomDumpItem_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CustomDumpItem_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CustomDumpItem_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct MDAInfo {
    pub lpMDACaption: ::windows::core::PCWSTR,
    pub lpMDAMessage: ::windows::core::PCWSTR,
    pub lpStackTrace: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for MDAInfo {}
impl ::core::clone::Clone for MDAInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MDAInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDAInfo").field("lpMDACaption", &self.lpMDACaption).field("lpMDAMessage", &self.lpMDAMessage).field("lpStackTrace", &self.lpStackTrace).finish()
    }
}
impl ::windows::core::TypeKind for MDAInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MDAInfo {
    fn eq(&self, other: &Self) -> bool {
        self.lpMDACaption == other.lpMDACaption && self.lpMDAMessage == other.lpMDAMessage && self.lpStackTrace == other.lpStackTrace
    }
}
impl ::core::cmp::Eq for MDAInfo {}
impl ::core::default::Default for MDAInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub struct ModuleBindInfo {
    pub dwAppDomainId: u32,
    pub lpAssemblyIdentity: ::windows::core::PCWSTR,
    pub lpModuleName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for ModuleBindInfo {}
impl ::core::clone::Clone for ModuleBindInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ModuleBindInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ModuleBindInfo").field("dwAppDomainId", &self.dwAppDomainId).field("lpAssemblyIdentity", &self.lpAssemblyIdentity).field("lpModuleName", &self.lpModuleName).finish()
    }
}
impl ::windows::core::TypeKind for ModuleBindInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ModuleBindInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwAppDomainId == other.dwAppDomainId && self.lpAssemblyIdentity == other.lpAssemblyIdentity && self.lpModuleName == other.lpModuleName
    }
}
impl ::core::cmp::Eq for ModuleBindInfo {}
impl ::core::default::Default for ModuleBindInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct StackOverflowInfo {
    pub soType: StackOverflowType,
    pub pExceptionInfo: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for StackOverflowInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for StackOverflowInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for StackOverflowInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StackOverflowInfo").field("soType", &self.soType).field("pExceptionInfo", &self.pExceptionInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for StackOverflowInfo {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for StackOverflowInfo {
    fn eq(&self, other: &Self) -> bool {
        self.soType == other.soType && self.pExceptionInfo == other.pExceptionInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for StackOverflowInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for StackOverflowInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type CLRCreateInstanceFnPtr = ::core::option::Option<unsafe extern "system" fn(clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type CallbackThreadSetFnPtr = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type CallbackThreadUnsetFnPtr = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type CreateInterfaceFnPtr = ::core::option::Option<unsafe extern "system" fn(clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type FExecuteInAppDomainCallback = ::core::option::Option<unsafe extern "system" fn(cookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type FLockClrVersionCallback = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type PTLS_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(__midl____midl_itf_mscoree_0000_00040005: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`*"]
pub type RuntimeLoadedCallbackFnPtr = ::core::option::Option<unsafe extern "system" fn(pruntimeinfo: ::core::option::Option<ICLRRuntimeInfo>, pfncallbackthreadset: CallbackThreadSetFnPtr, pfncallbackthreadunset: CallbackThreadUnsetFnPtr) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
