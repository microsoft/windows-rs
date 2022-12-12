#[cfg(feature = "Win32_System_Com_CallObj")]
pub mod CallObj;
#[cfg(feature = "Win32_System_Com_ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Win32_System_Com_Events")]
pub mod Events;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_UI")]
pub mod UI;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn BindMoniker<P0>(pmk: P0, grfopt: u32, iidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn BindMoniker ( pmk : * mut::core::ffi::c_void , grfopt : u32 , iidresult : *const :: windows::core::GUID , ppvresult : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    BindMoniker(pmk.into().abi(), grfopt, iidresult, ppvresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CLSIDFromProgID<P0>(lpszprogid: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CLSIDFromProgID ( lpszprogid : :: windows::core::PCWSTR , lpclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CLSIDFromProgID(lpszprogid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CLSIDFromProgIDEx<P0>(lpszprogid: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CLSIDFromProgIDEx ( lpszprogid : :: windows::core::PCWSTR , lpclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CLSIDFromProgIDEx(lpszprogid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CLSIDFromString<P0>(lpsz: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CLSIDFromString ( lpsz : :: windows::core::PCWSTR , pclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CLSIDFromString(lpsz.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    ::windows::core::link ! ( "ole32.dll""system" fn CoAddRefServerProcess ( ) -> u32 );
    CoAddRefServerProcess()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<P0>(punk: P0, lpvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoAllowSetForegroundWindow ( punk : * mut::core::ffi::c_void , lpvreserved : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoAllowSetForegroundWindow(punk.into().abi(), ::core::mem::transmute(lpvreserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoAllowUnmarshalerCLSID ( clsid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    CoAllowUnmarshalerCLSID(clsid).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    ::windows::core::link ! ( "ole32.dll""system" fn CoBuildVersion ( ) -> u32 );
    CoBuildVersion()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoCancelCall ( dwthreadid : u32 , ultimeout : u32 ) -> :: windows::core::HRESULT );
    CoCancelCall(dwthreadid, ultimeout).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCopyProxy<P0>(pproxy: P0) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoCopyProxy ( pproxy : * mut::core::ffi::c_void , ppcopy : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoCopyProxy(pproxy.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<P0>(punkouter: P0) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoCreateFreeThreadedMarshaler ( punkouter : * mut::core::ffi::c_void , ppunkmarshal : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoCreateFreeThreadedMarshaler(punkouter.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoCreateGuid ( pguid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoCreateGuid(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateInstance<P0, T>(rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: CLSCTX) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoCreateInstance ( rclsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , dwclscontext : CLSCTX , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoCreateInstance(rclsid, punkouter.into().abi(), dwclscontext, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateInstanceEx<P0>(clsid: *const ::windows::core::GUID, punkouter: P0, dwclsctx: CLSCTX, pserverinfo: ::core::option::Option<*const COSERVERINFO>, presults: &mut [MULTI_QI]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoCreateInstanceEx ( clsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , dwclsctx : CLSCTX , pserverinfo : *const COSERVERINFO , dwcount : u32 , presults : *mut MULTI_QI ) -> :: windows::core::HRESULT );
    CoCreateInstanceEx(clsid, punkouter.into().abi(), dwclsctx, ::core::mem::transmute(pserverinfo.unwrap_or(::std::ptr::null())), presults.len() as _, ::core::mem::transmute(presults.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateInstanceFromApp<P0>(clsid: *const ::windows::core::GUID, punkouter: P0, dwclsctx: CLSCTX, reserved: ::core::option::Option<*const ::core::ffi::c_void>, presults: &mut [MULTI_QI]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoCreateInstanceFromApp ( clsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , dwclsctx : CLSCTX , reserved : *const ::core::ffi::c_void , dwcount : u32 , presults : *mut MULTI_QI ) -> :: windows::core::HRESULT );
    CoCreateInstanceFromApp(clsid, punkouter.into().abi(), dwclsctx, ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null())), presults.len() as _, ::core::mem::transmute(presults.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoDecrementMTAUsage<P0>(cookie: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<CO_MTA_USAGE_COOKIE>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoDecrementMTAUsage ( cookie : CO_MTA_USAGE_COOKIE ) -> :: windows::core::HRESULT );
    CoDecrementMTAUsage(cookie.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoDisableCallCancellation ( preserved : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoDisableCallCancellation(::core::mem::transmute(preserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoDisconnectContext ( dwtimeout : u32 ) -> :: windows::core::HRESULT );
    CoDisconnectContext(dwtimeout).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoDisconnectObject<P0>(punk: P0, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoDisconnectObject ( punk : * mut::core::ffi::c_void , dwreserved : u32 ) -> :: windows::core::HRESULT );
    CoDisconnectObject(punk.into().abi(), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "ole32.dll""system" fn CoDosDateTimeToFileTime ( ndosdate : u16 , ndostime : u16 , lpfiletime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    CoDosDateTimeToFileTime(ndosdate, ndostime, lpfiletime)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoEnableCallCancellation ( preserved : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoEnableCallCancellation(::core::mem::transmute(preserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoFileTimeNow ( lpfiletime : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoFileTimeNow(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "ole32.dll""system" fn CoFileTimeToDosDateTime ( lpfiletime : *const super::super::Foundation:: FILETIME , lpdosdate : *mut u16 , lpdostime : *mut u16 ) -> super::super::Foundation:: BOOL );
    CoFileTimeToDosDateTime(lpfiletime, lpdosdate, lpdostime)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    ::windows::core::link ! ( "ole32.dll""system" fn CoFreeAllLibraries ( ) -> ( ) );
    CoFreeAllLibraries()
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFreeLibrary<P0>(hinst: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoFreeLibrary ( hinst : super::super::Foundation:: HINSTANCE ) -> ( ) );
    CoFreeLibrary(hinst.into())
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    ::windows::core::link ! ( "ole32.dll""system" fn CoFreeUnusedLibraries ( ) -> ( ) );
    CoFreeUnusedLibraries()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    ::windows::core::link ! ( "ole32.dll""system" fn CoFreeUnusedLibrariesEx ( dwunloaddelay : u32 , dwreserved : u32 ) -> ( ) );
    CoFreeUnusedLibrariesEx(dwunloaddelay, dwreserved)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetApartmentType ( papttype : *mut APTTYPE , paptqualifier : *mut APTTYPEQUALIFIER ) -> :: windows::core::HRESULT );
    CoGetApartmentType(papttype, paptqualifier).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCallContext(riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetCallContext ( riid : *const :: windows::core::GUID , ppinterface : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetCallContext(riid, ppinterface).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCallerTID() -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetCallerTID ( lpdwtid : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoGetCallerTID(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetCancelObject ( dwthreadid : u32 , iid : *const :: windows::core::GUID , ppunk : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetCancelObject(dwthreadid, iid, ppunk).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const ::windows::core::GUID, dwclscontext: CLSCTX, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetClassObject ( rclsid : *const :: windows::core::GUID , dwclscontext : CLSCTX , pvreserved : *const ::core::ffi::c_void , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetClassObject(rclsid, dwclscontext, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetContextToken() -> ::windows::core::Result<usize> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetContextToken ( ptoken : *mut usize ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoGetContextToken(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetCurrentLogicalThreadId ( pguid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoGetCurrentLogicalThreadId(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetCurrentProcess ( ) -> u32 );
    CoGetCurrentProcess()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> ::windows::core::Result<IMalloc> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetMalloc ( dwmemcontext : u32 , ppmalloc : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoGetMalloc(dwmemcontext, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetObject<P0>(pszname: P0, pbindoptions: ::core::option::Option<*const BIND_OPTS>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetObject ( pszname : :: windows::core::PCWSTR , pbindoptions : *const BIND_OPTS , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetObject(pszname.into().abi(), ::core::mem::transmute(pbindoptions.unwrap_or(::std::ptr::null())), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetObjectContext ( riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetObjectContext(riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetPSClsid ( riid : *const :: windows::core::GUID , pclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoGetPSClsid(riid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut super::super::Security::PSECURITY_DESCRIPTOR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetSystemSecurityPermissions ( comsdtype : COMSD , ppsd : *mut super::super::Security:: PSECURITY_DESCRIPTOR ) -> :: windows::core::HRESULT );
    CoGetSystemSecurityPermissions(comsdtype, ppsd).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoGetTreatAsClass ( clsidold : *const :: windows::core::GUID , pclsidnew : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoGetTreatAsClass(clsidold, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoImpersonateClient() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoImpersonateClient ( ) -> :: windows::core::HRESULT );
    CoImpersonateClient().ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> ::windows::core::Result<CO_MTA_USAGE_COOKIE> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoIncrementMTAUsage ( pcookie : *mut CO_MTA_USAGE_COOKIE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoIncrementMTAUsage(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoInitialize(pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoInitialize ( pvreserved : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoInitialize(::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: ::core::option::Option<*const ::core::ffi::c_void>, dwcoinit: COINIT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoInitializeEx ( pvreserved : *const ::core::ffi::c_void , dwcoinit : COINIT ) -> :: windows::core::HRESULT );
    CoInitializeEx(::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), dwcoinit).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoInitializeSecurity<P0>(psecdesc: P0, cauthsvc: i32, asauthsvc: ::core::option::Option<*const SOLE_AUTHENTICATION_SERVICE>, preserved1: ::core::option::Option<*const ::core::ffi::c_void>, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: ::core::option::Option<*const ::core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoInitializeSecurity ( psecdesc : super::super::Security:: PSECURITY_DESCRIPTOR , cauthsvc : i32 , asauthsvc : *const SOLE_AUTHENTICATION_SERVICE , preserved1 : *const ::core::ffi::c_void , dwauthnlevel : RPC_C_AUTHN_LEVEL , dwimplevel : RPC_C_IMP_LEVEL , pauthlist : *const ::core::ffi::c_void , dwcapabilities : EOLE_AUTHENTICATION_CAPABILITIES , preserved3 : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoInitializeSecurity(psecdesc.into(), cauthsvc, ::core::mem::transmute(asauthsvc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved1.unwrap_or(::std::ptr::null())), dwauthnlevel, dwimplevel, ::core::mem::transmute(pauthlist.unwrap_or(::std::ptr::null())), dwcapabilities, ::core::mem::transmute(preserved3.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoInstall<P0, P1>(pbc: P0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoInstall ( pbc : * mut::core::ffi::c_void , dwflags : u32 , pclassspec : *const uCLSSPEC , pquery : *const QUERYCONTEXT , pszcodebase : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    CoInstall(pbc.into().abi(), dwflags, pclassspec, pquery, pszcodebase.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<P0>(pszmachinename: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoInvalidateRemoteMachineBindings ( pszmachinename : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    CoInvalidateRemoteMachineBindings(pszmachinename.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsHandlerConnected<P0>(punk: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoIsHandlerConnected ( punk : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CoIsHandlerConnected(punk.into().abi())
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "ole32.dll""system" fn CoIsOle1Class ( rclsid : *const :: windows::core::GUID ) -> super::super::Foundation:: BOOL );
    CoIsOle1Class(rclsid)
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLoadLibrary<P0, P1>(lpszlibname: P0, bautofree: P1) -> super::super::Foundation::HINSTANCE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoLoadLibrary ( lpszlibname : :: windows::core::PCWSTR , bautofree : super::super::Foundation:: BOOL ) -> super::super::Foundation:: HINSTANCE );
    CoLoadLibrary(lpszlibname.into().abi(), bautofree.into())
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLockObjectExternal<P0, P1, P2>(punk: P0, flock: P1, flastunlockreleases: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoLockObjectExternal ( punk : * mut::core::ffi::c_void , flock : super::super::Foundation:: BOOL , flastunlockreleases : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    CoLockObjectExternal(punk.into().abi(), flock.into(), flastunlockreleases.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoQueryAuthenticationServices ( pcauthsvc : *mut u32 , asauthsvc : *mut *mut SOLE_AUTHENTICATION_SERVICE ) -> :: windows::core::HRESULT );
    CoQueryAuthenticationServices(pcauthsvc, asauthsvc).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: ::core::option::Option<*mut u32>, pauthzsvc: ::core::option::Option<*mut u32>, pserverprincname: ::core::option::Option<*mut ::windows::core::PWSTR>, pauthnlevel: ::core::option::Option<*mut u32>, pimplevel: ::core::option::Option<*mut u32>, pprivs: ::core::option::Option<*mut *mut ::core::ffi::c_void>, pcapabilities: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoQueryClientBlanket ( pauthnsvc : *mut u32 , pauthzsvc : *mut u32 , pserverprincname : *mut :: windows::core::PWSTR , pauthnlevel : *mut u32 , pimplevel : *mut u32 , pprivs : *mut *mut ::core::ffi::c_void , pcapabilities : *mut u32 ) -> :: windows::core::HRESULT );
    CoQueryClientBlanket(
        ::core::mem::transmute(pauthnsvc.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pauthzsvc.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pserverprincname.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pauthnlevel.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pimplevel.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pprivs.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pcapabilities.unwrap_or(::std::ptr::null_mut())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoQueryProxyBlanket<P0>(pproxy: P0, pwauthnsvc: ::core::option::Option<*mut u32>, pauthzsvc: ::core::option::Option<*mut u32>, pserverprincname: ::core::option::Option<*mut ::windows::core::PWSTR>, pauthnlevel: ::core::option::Option<*mut u32>, pimplevel: ::core::option::Option<*mut u32>, pauthinfo: ::core::option::Option<*mut *mut ::core::ffi::c_void>, pcapabilites: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoQueryProxyBlanket ( pproxy : * mut::core::ffi::c_void , pwauthnsvc : *mut u32 , pauthzsvc : *mut u32 , pserverprincname : *mut :: windows::core::PWSTR , pauthnlevel : *mut u32 , pimplevel : *mut u32 , pauthinfo : *mut *mut ::core::ffi::c_void , pcapabilites : *mut u32 ) -> :: windows::core::HRESULT );
    CoQueryProxyBlanket(
        pproxy.into().abi(),
        ::core::mem::transmute(pwauthnsvc.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pauthzsvc.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pserverprincname.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pauthnlevel.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pimplevel.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pauthinfo.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pcapabilites.unwrap_or(::std::ptr::null_mut())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterActivationFilter<P0>(pactivationfilter: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IActivationFilter>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterActivationFilter ( pactivationfilter : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoRegisterActivationFilter(pactivationfilter.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterChannelHook<P0>(extensionuuid: *const ::windows::core::GUID, pchannelhook: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IChannelHook>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterChannelHook ( extensionuuid : *const :: windows::core::GUID , pchannelhook : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoRegisterChannelHook(extensionuuid, pchannelhook.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterClassObject<P0>(rclsid: *const ::windows::core::GUID, punk: P0, dwclscontext: CLSCTX, flags: REGCLS) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterClassObject ( rclsid : *const :: windows::core::GUID , punk : * mut::core::ffi::c_void , dwclscontext : CLSCTX , flags : REGCLS , lpdwregister : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoRegisterClassObject(rclsid, punk.into().abi(), dwclscontext, flags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<P0>(deviceinstanceid: P0) -> ::windows::core::Result<CO_DEVICE_CATALOG_COOKIE>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterDeviceCatalog ( deviceinstanceid : :: windows::core::PCWSTR , cookie : *mut CO_DEVICE_CATALOG_COOKIE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoRegisterDeviceCatalog(deviceinstanceid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterInitializeSpy<P0>(pspy: P0) -> ::windows::core::Result<u64>
where
    P0: ::std::convert::Into<::windows::core::InParam<IInitializeSpy>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterInitializeSpy ( pspy : * mut::core::ffi::c_void , pulicookie : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoRegisterInitializeSpy(pspy.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterMallocSpy<P0>(pmallocspy: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IMallocSpy>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterMallocSpy ( pmallocspy : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoRegisterMallocSpy(pmallocspy.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const ::windows::core::GUID, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterPSClsid ( riid : *const :: windows::core::GUID , rclsid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    CoRegisterPSClsid(riid, rclsid).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterSurrogate<P0>(psurrogate: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<ISurrogate>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRegisterSurrogate ( psurrogate : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoRegisterSurrogate(psurrogate.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    ::windows::core::link ! ( "ole32.dll""system" fn CoReleaseServerProcess ( ) -> u32 );
    CoReleaseServerProcess()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoResumeClassObjects() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoResumeClassObjects ( ) -> :: windows::core::HRESULT );
    CoResumeClassObjects().ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRevertToSelf() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoRevertToSelf ( ) -> :: windows::core::HRESULT );
    CoRevertToSelf().ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoRevokeClassObject ( dwregister : u32 ) -> :: windows::core::HRESULT );
    CoRevokeClassObject(dwregister).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRevokeDeviceCatalog<P0>(cookie: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<CO_DEVICE_CATALOG_COOKIE>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoRevokeDeviceCatalog ( cookie : CO_DEVICE_CATALOG_COOKIE ) -> :: windows::core::HRESULT );
    CoRevokeDeviceCatalog(cookie.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoRevokeInitializeSpy ( ulicookie : u64 ) -> :: windows::core::HRESULT );
    CoRevokeInitializeSpy(ulicookie).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoRevokeMallocSpy ( ) -> :: windows::core::HRESULT );
    CoRevokeMallocSpy().ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSetCancelObject<P0>(punk: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoSetCancelObject ( punk : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoSetCancelObject(punk.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSetProxyBlanket<P0, P1>(pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: P1, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: ::core::option::Option<*const ::core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoSetProxyBlanket ( pproxy : * mut::core::ffi::c_void , dwauthnsvc : u32 , dwauthzsvc : u32 , pserverprincname : :: windows::core::PCWSTR , dwauthnlevel : RPC_C_AUTHN_LEVEL , dwimplevel : RPC_C_IMP_LEVEL , pauthinfo : *const ::core::ffi::c_void , dwcapabilities : EOLE_AUTHENTICATION_CAPABILITIES ) -> :: windows::core::HRESULT );
    CoSetProxyBlanket(pproxy.into().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.into().abi(), dwauthnlevel, dwimplevel, ::core::mem::transmute(pauthinfo.unwrap_or(::std::ptr::null())), dwcapabilities).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSuspendClassObjects() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoSuspendClassObjects ( ) -> :: windows::core::HRESULT );
    CoSuspendClassObjects().ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSwitchCallContext<P0>(pnewobject: P0) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CoSwitchCallContext ( pnewobject : * mut::core::ffi::c_void , ppoldobject : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoSwitchCallContext(pnewobject.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "ole32.dll""system" fn CoTaskMemAlloc ( cb : usize ) -> *mut ::core::ffi::c_void );
    CoTaskMemAlloc(cb)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoTaskMemFree(pv: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows::core::link ! ( "ole32.dll""system" fn CoTaskMemFree ( pv : *const ::core::ffi::c_void ) -> ( ) );
    CoTaskMemFree(::core::mem::transmute(pv.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: ::core::option::Option<*const ::core::ffi::c_void>, cb: usize) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "ole32.dll""system" fn CoTaskMemRealloc ( pv : *const ::core::ffi::c_void , cb : usize ) -> *mut ::core::ffi::c_void );
    CoTaskMemRealloc(::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), cb)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoTestCancel() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoTestCancel ( ) -> :: windows::core::HRESULT );
    CoTestCancel().ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const ::windows::core::GUID, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoTreatAsClass ( clsidold : *const :: windows::core::GUID , clsidnew : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    CoTreatAsClass(clsidold, clsidnew).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoUninitialize() {
    ::windows::core::link ! ( "ole32.dll""system" fn CoUninitialize ( ) -> ( ) );
    CoUninitialize()
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoWaitForMultipleHandles ( dwflags : u32 , dwtimeout : u32 , chandles : u32 , phandles : *const super::super::Foundation:: HANDLE , lpdwindex : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoWaitForMultipleHandles(dwflags, dwtimeout, phandles.len() as _, ::core::mem::transmute(phandles.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoWaitForMultipleObjects ( dwflags : u32 , dwtimeout : u32 , chandles : u32 , phandles : *const super::super::Foundation:: HANDLE , lpdwindex : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoWaitForMultipleObjects(dwflags, dwtimeout, phandles.len() as _, ::core::mem::transmute(phandles.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateAntiMoniker() -> ::windows::core::Result<IMoniker> {
    ::windows::core::link ! ( "ole32.dll""system" fn CreateAntiMoniker ( ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateAntiMoniker(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> ::windows::core::Result<IBindCtx> {
    ::windows::core::link ! ( "ole32.dll""system" fn CreateBindCtx ( reserved : u32 , ppbc : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateBindCtx(reserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IMoniker> {
    ::windows::core::link ! ( "ole32.dll""system" fn CreateClassMoniker ( rclsid : *const :: windows::core::GUID , ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateClassMoniker(rclsid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> ::windows::core::Result<IDataAdviseHolder> {
    ::windows::core::link ! ( "ole32.dll""system" fn CreateDataAdviseHolder ( ppdaholder : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateDataAdviseHolder(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateDataCache<P0>(punkouter: P0, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreateDataCache ( punkouter : * mut::core::ffi::c_void , rclsid : *const :: windows::core::GUID , iid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateDataCache(punkouter.into().abi(), rclsid, iid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateFileMoniker<P0>(lpszpathname: P0) -> ::windows::core::Result<IMoniker>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreateFileMoniker ( lpszpathname : :: windows::core::PCWSTR , ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateFileMoniker(lpszpathname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateGenericComposite<P0, P1>(pmkfirst: P0, pmkrest: P1) -> ::windows::core::Result<IMoniker>
where
    P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreateGenericComposite ( pmkfirst : * mut::core::ffi::c_void , pmkrest : * mut::core::ffi::c_void , ppmkcomposite : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateGenericComposite(pmkfirst.into().abi(), pmkrest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateIUriBuilder<P0>(piuri: P0, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUriBuilder>
where
    P0: ::std::convert::Into<::windows::core::InParam<IUri>>,
{
    ::windows::core::link ! ( "urlmon.dll""system" fn CreateIUriBuilder ( piuri : * mut::core::ffi::c_void , dwflags : u32 , dwreserved : usize , ppiuribuilder : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateIUriBuilder(piuri.into().abi(), dwflags, dwreserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateItemMoniker<P0, P1>(lpszdelim: P0, lpszitem: P1) -> ::windows::core::Result<IMoniker>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreateItemMoniker ( lpszdelim : :: windows::core::PCWSTR , lpszitem : :: windows::core::PCWSTR , ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateItemMoniker(lpszdelim.into().abi(), lpszitem.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateObjrefMoniker<P0>(punk: P0) -> ::windows::core::Result<IMoniker>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreateObjrefMoniker ( punk : * mut::core::ffi::c_void , ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateObjrefMoniker(punk.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreatePointerMoniker<P0>(punk: P0) -> ::windows::core::Result<IMoniker>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreatePointerMoniker ( punk : * mut::core::ffi::c_void , ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreatePointerMoniker(punk.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdProgressIndicator<P0, P1, P2>(hwndparent: P0, psztitle: P1, pibsccaller: P2) -> ::windows::core::Result<IBindStatusCallback>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<IBindStatusCallback>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn CreateStdProgressIndicator ( hwndparent : super::super::Foundation:: HWND , psztitle : :: windows::core::PCWSTR , pibsccaller : * mut::core::ffi::c_void , ppibsc : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateStdProgressIndicator(hwndparent.into(), psztitle.into().abi(), pibsccaller.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateUri<P0>(pwzuri: P0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows::core::Result<IUri>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "urlmon.dll""system" fn CreateUri ( pwzuri : :: windows::core::PCWSTR , dwflags : URI_CREATE_FLAGS , dwreserved : usize , ppuri : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateUri(pwzuri.into().abi(), dwflags, dwreserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateUriFromMultiByteString<P0>(pszansiinputuri: P0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "urlmon.dll""system" fn CreateUriFromMultiByteString ( pszansiinputuri : :: windows::core::PCSTR , dwencodingflags : u32 , dwcodepage : u32 , dwcreateflags : u32 , dwreserved : usize , ppuri : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateUriFromMultiByteString(pszansiinputuri.into().abi(), dwencodingflags, dwcodepage, dwcreateflags, dwreserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateUriWithFragment<P0, P1>(pwzuri: P0, pwzfragment: P1, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "urlmon.dll""system" fn CreateUriWithFragment ( pwzuri : :: windows::core::PCWSTR , pwzfragment : :: windows::core::PCWSTR , dwflags : u32 , dwreserved : usize , ppuri : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateUriWithFragment(pwzuri.into().abi(), pwzfragment.into().abi(), dwflags, dwreserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: ::core::option::Option<*const ::core::ffi::c_void>, pulreserved: ::core::option::Option<*const u32>, appshr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn DcomChannelSetHResult ( pvreserved : *const ::core::ffi::c_void , pulreserved : *const u32 , appshr : :: windows::core::HRESULT ) -> :: windows::core::HRESULT );
    DcomChannelSetHResult(::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pulreserved.unwrap_or(::std::ptr::null())), appshr).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn GetClassFile<P0>(szfilename: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn GetClassFile ( szfilename : :: windows::core::PCWSTR , pclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetClassFile(szfilename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::core::Result<IErrorInfo> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn GetErrorInfo ( dwreserved : u32 , pperrinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetErrorInfo(dwreserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> ::windows::core::Result<IRunningObjectTable> {
    ::windows::core::link ! ( "ole32.dll""system" fn GetRunningObjectTable ( reserved : u32 , pprot : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetRunningObjectTable(reserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn IIDFromString<P0>(lpsz: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn IIDFromString ( lpsz : :: windows::core::PCWSTR , lpiid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    IIDFromString(lpsz.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn MkParseDisplayName<P0, P1>(pbc: P0, szusername: P1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn MkParseDisplayName ( pbc : * mut::core::ffi::c_void , szusername : :: windows::core::PCWSTR , pcheaten : *mut u32 , ppmk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    MkParseDisplayName(pbc.into().abi(), szusername.into().abi(), pcheaten, ::core::mem::transmute(ppmk)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn MonikerCommonPrefixWith<P0, P1>(pmkthis: P0, pmkother: P1) -> ::windows::core::Result<IMoniker>
where
    P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn MonikerCommonPrefixWith ( pmkthis : * mut::core::ffi::c_void , pmkother : * mut::core::ffi::c_void , ppmkcommon : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    MonikerCommonPrefixWith(pmkthis.into().abi(), pmkother.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonikerRelativePathTo<P0, P1, P2>(pmksrc: P0, pmkdest: P1, ppmkrelpath: *mut ::core::option::Option<IMoniker>, dwreserved: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn MonikerRelativePathTo ( pmksrc : * mut::core::ffi::c_void , pmkdest : * mut::core::ffi::c_void , ppmkrelpath : *mut * mut::core::ffi::c_void , dwreserved : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    MonikerRelativePathTo(pmksrc.into().abi(), pmkdest.into().abi(), ::core::mem::transmute(ppmkrelpath), dwreserved.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "ole32.dll""system" fn ProgIDFromCLSID ( clsid : *const :: windows::core::GUID , lplpszprogid : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ProgIDFromCLSID(clsid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn SetErrorInfo<P0>(dwreserved: u32, perrinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IErrorInfo>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn SetErrorInfo ( dwreserved : u32 , perrinfo : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SetErrorInfo(dwreserved, perrinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "ole32.dll""system" fn StringFromCLSID ( rclsid : *const :: windows::core::GUID , lplpsz : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StringFromCLSID(rclsid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: &mut [u16]) -> i32 {
    ::windows::core::link ! ( "ole32.dll""system" fn StringFromGUID2 ( rguid : *const :: windows::core::GUID , lpsz : :: windows::core::PWSTR , cchmax : i32 ) -> i32 );
    StringFromGUID2(rguid, ::core::mem::transmute(lpsz.as_ptr()), lpsz.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "ole32.dll""system" fn StringFromIID ( rclsid : *const :: windows::core::GUID , lplpsz : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StringFromIID(rclsid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIAdviseSink(::windows::core::IUnknown);
impl AsyncIAdviseSink {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Vtable::vtable(self).Begin_OnDataChange)(::windows::core::Vtable::as_raw(self), pformatetc, pstgmed)
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows::core::Vtable::vtable(self).Finish_OnDataChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Vtable::vtable(self).Begin_OnViewChange)(::windows::core::Vtable::as_raw(self), dwaspect, lindex)
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows::core::Vtable::vtable(self).Finish_OnViewChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnRename<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_OnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    pub unsafe fn Finish_OnRename(&self) {
        (::windows::core::Vtable::vtable(self).Finish_OnRename)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnSave(&self) {
        (::windows::core::Vtable::vtable(self).Begin_OnSave)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Finish_OnSave(&self) {
        (::windows::core::Vtable::vtable(self).Finish_OnSave)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnClose(&self) {
        (::windows::core::Vtable::vtable(self).Begin_OnClose)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Finish_OnClose(&self) {
        (::windows::core::Vtable::vtable(self).Finish_OnClose)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(AsyncIAdviseSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink {}
impl ::core::fmt::Debug for AsyncIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSink_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000150_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_OnDataChange: usize,
    pub Finish_OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub Finish_OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void),
    pub Finish_OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Finish_OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Finish_OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIAdviseSink2(::windows::core::IUnknown);
impl AsyncIAdviseSink2 {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Vtable::vtable(self).base__.Begin_OnDataChange)(::windows::core::Vtable::as_raw(self), pformatetc, pstgmed)
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.Finish_OnDataChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Vtable::vtable(self).base__.Begin_OnViewChange)(::windows::core::Vtable::as_raw(self), dwaspect, lindex)
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows::core::Vtable::vtable(self).base__.Finish_OnViewChange)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnRename<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Begin_OnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    pub unsafe fn Finish_OnRename(&self) {
        (::windows::core::Vtable::vtable(self).base__.Finish_OnRename)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnSave(&self) {
        (::windows::core::Vtable::vtable(self).base__.Begin_OnSave)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Finish_OnSave(&self) {
        (::windows::core::Vtable::vtable(self).base__.Finish_OnSave)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnClose(&self) {
        (::windows::core::Vtable::vtable(self).base__.Begin_OnClose)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Finish_OnClose(&self) {
        (::windows::core::Vtable::vtable(self).base__.Finish_OnClose)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_OnLinkSrcChange<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_OnLinkSrcChange)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        (::windows::core::Vtable::vtable(self).Finish_OnLinkSrcChange)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(AsyncIAdviseSink2, ::windows::core::IUnknown, AsyncIAdviseSink);
impl ::core::clone::Clone for AsyncIAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink2 {}
impl ::core::fmt::Debug for AsyncIAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000151_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_Vtbl {
    pub base__: AsyncIAdviseSink_Vtbl,
    pub Begin_OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void),
    pub Finish_OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIMultiQI(::windows::core::IUnknown);
impl AsyncIMultiQI {
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_QueryMultipleInterfaces)(::windows::core::Vtable::as_raw(self), pmqis.len() as _, ::core::mem::transmute(pmqis.as_ptr())).ok()
    }
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_QueryMultipleInterfaces)(::windows::core::Vtable::as_raw(self), pmqis).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIMultiQI, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMultiQI {}
impl ::core::fmt::Debug for AsyncIMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIMultiQI {
    type Vtable = AsyncIMultiQI_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIMultiQI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIPipeByte(::windows::core::IUnknown);
impl AsyncIPipeByte {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Pull)(::windows::core::Vtable::as_raw(self), crequest).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Pull)(::windows::core::Vtable::as_raw(self), buf, pcreturned).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Push)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Push)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIPipeByte, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeByte {}
impl ::core::fmt::Debug for AsyncIPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIPipeByte {
    type Vtable = AsyncIPipeByte_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIPipeByte {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIPipeDouble(::windows::core::IUnknown);
impl AsyncIPipeDouble {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Pull)(::windows::core::Vtable::as_raw(self), crequest).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Pull)(::windows::core::Vtable::as_raw(self), buf, pcreturned).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Push)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Push)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIPipeDouble, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeDouble {}
impl ::core::fmt::Debug for AsyncIPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDouble_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIPipeDouble {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIPipeLong(::windows::core::IUnknown);
impl AsyncIPipeLong {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Pull)(::windows::core::Vtable::as_raw(self), crequest).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Pull)(::windows::core::Vtable::as_raw(self), buf, pcreturned).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Push)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Push)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIPipeLong, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeLong {}
impl ::core::fmt::Debug for AsyncIPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIPipeLong {
    type Vtable = AsyncIPipeLong_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIPipeLong {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIUnknown(::windows::core::IUnknown);
impl AsyncIUnknown {
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_QueryInterface)(::windows::core::Vtable::as_raw(self), riid).ok()
    }
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_QueryInterface)(::windows::core::Vtable::as_raw(self), ppvobject).ok()
    }
    pub unsafe fn Begin_AddRef(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_AddRef)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).Finish_AddRef)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Begin_Release(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Release)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish_Release(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).Finish_Release)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(AsyncIUnknown, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIUnknown {}
impl ::core::fmt::Debug for AsyncIUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncIUnknown {
    type Vtable = AsyncIUnknown_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIUnknown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Begin_Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IActivationFilter(::windows::core::IUnknown);
impl IActivationFilter {
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HandleActivation)(::windows::core::Vtable::as_raw(self), dwactivationtype, rclsid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IActivationFilter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IActivationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFilter {}
impl ::core::fmt::Debug for IActivationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IActivationFilter {
    type Vtable = IActivationFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000017_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandleActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAddrExclusionControl(::windows::core::IUnknown);
impl IAddrExclusionControl {
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCurrentAddrExclusionList)(::windows::core::Vtable::as_raw(self), riid, ppenumerator).ok()
    }
    pub unsafe fn UpdateAddrExclusionList<P0>(&self, penumerator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).UpdateAddrExclusionList)(::windows::core::Vtable::as_raw(self), penumerator.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAddrExclusionControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAddrExclusionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrExclusionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrExclusionControl {}
impl ::core::fmt::Debug for IAddrExclusionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrExclusionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAddrExclusionControl {
    type Vtable = IAddrExclusionControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IAddrExclusionControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000148_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAddrTrackingControl(::windows::core::IUnknown);
impl IAddrTrackingControl {
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnableCOMDynamicAddrTracking)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisableCOMDynamicAddrTracking)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IAddrTrackingControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAddrTrackingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrTrackingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrTrackingControl {}
impl ::core::fmt::Debug for IAddrTrackingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrTrackingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAddrTrackingControl {
    type Vtable = IAddrTrackingControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IAddrTrackingControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000147_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableCOMDynamicAddrTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableCOMDynamicAddrTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAdviseSink(::windows::core::IUnknown);
impl IAdviseSink {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Vtable::vtable(self).OnDataChange)(::windows::core::Vtable::as_raw(self), pformatetc, pstgmed)
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Vtable::vtable(self).OnViewChange)(::windows::core::Vtable::as_raw(self), dwaspect, lindex)
    }
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).OnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    pub unsafe fn OnSave(&self) {
        (::windows::core::Vtable::vtable(self).OnSave)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn OnClose(&self) {
        (::windows::core::Vtable::vtable(self).OnClose)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IAdviseSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink {}
impl ::core::fmt::Debug for IAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAdviseSink {
    type Vtable = IAdviseSink_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdviseSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataChange: usize,
    pub OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void),
    pub OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAdviseSink2(::windows::core::IUnknown);
impl IAdviseSink2 {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Vtable::vtable(self).base__.OnDataChange)(::windows::core::Vtable::as_raw(self), pformatetc, pstgmed)
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Vtable::vtable(self).base__.OnViewChange)(::windows::core::Vtable::as_raw(self), dwaspect, lindex)
    }
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    pub unsafe fn OnSave(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnSave)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn OnClose(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnClose)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn OnLinkSrcChange<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).OnLinkSrcChange)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
}
::windows::core::interface_hierarchy!(IAdviseSink2, ::windows::core::IUnknown, IAdviseSink);
impl ::core::clone::Clone for IAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink2 {}
impl ::core::fmt::Debug for IAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAdviseSink2 {
    type Vtable = IAdviseSink2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdviseSink2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000125_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_Vtbl {
    pub base__: IAdviseSink_Vtbl,
    pub OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAgileObject(::windows::core::IUnknown);
impl IAgileObject {}
::windows::core::interface_hierarchy!(IAgileObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileObject {}
impl ::core::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IAgileObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAsyncManager(::windows::core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CompleteCall)(::windows::core::Vtable::as_raw(self), result).ok()
    }
    pub unsafe fn GetCallContext(&self, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCallContext)(::windows::core::Vtable::as_raw(self), riid, pinterface).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAsyncManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAsyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncManager {}
impl ::core::fmt::Debug for IAsyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAsyncManager {
    type Vtable = IAsyncManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAsyncManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CompleteCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAsyncRpcChannelBuffer(::windows::core::IUnknown);
impl IAsyncRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), pmessage, riid).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendReceive)(::windows::core::Vtable::as_raw(self), pmessage, ::core::mem::transmute(pstatus.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FreeBuffer)(::windows::core::Vtable::as_raw(self), pmessage).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDestCtx)(::windows::core::Vtable::as_raw(self), pdwdestcontext, ppvdestcontext).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IsConnected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProtocolVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Send<P0>(&self, pmsg: *mut RPCOLEMESSAGE, psync: P0, pulstatus: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISynchronize>>,
    {
        (::windows::core::Vtable::vtable(self).Send)(::windows::core::Vtable::as_raw(self), pmsg, psync.into().abi(), pulstatus).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Receive)(::windows::core::Vtable::as_raw(self), pmsg, pulstatus).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDestCtxEx)(::windows::core::Vtable::as_raw(self), pmsg, pdwdestcontext, ::core::mem::transmute(ppvdestcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IAsyncRpcChannelBuffer, ::windows::core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl ::core::clone::Clone for IAsyncRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncRpcChannelBuffer {}
impl ::core::fmt::Debug for IAsyncRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IAsyncRpcChannelBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAuthenticate(::windows::core::IUnknown);
impl IAuthenticate {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Authenticate)(::windows::core::Vtable::as_raw(self), phwnd, pszusername, pszpassword).ok()
    }
}
::windows::core::interface_hierarchy!(IAuthenticate, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticate {}
impl ::core::fmt::Debug for IAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAuthenticate {
    type Vtable = IAuthenticate_Vtbl;
}
unsafe impl ::windows::core::Interface for IAuthenticate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Authenticate: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAuthenticateEx(::windows::core::IUnknown);
impl IAuthenticateEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Authenticate)(::windows::core::Vtable::as_raw(self), phwnd, pszusername, pszpassword).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AuthenticateEx)(::windows::core::Vtable::as_raw(self), phwnd, pszusername, pszpassword, pauthinfo).ok()
    }
}
::windows::core::interface_hierarchy!(IAuthenticateEx, ::windows::core::IUnknown, IAuthenticate);
impl ::core::clone::Clone for IAuthenticateEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticateEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticateEx {}
impl ::core::fmt::Debug for IAuthenticateEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticateEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAuthenticateEx {
    type Vtable = IAuthenticateEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IAuthenticateEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateEx_Vtbl {
    pub base__: IAuthenticate_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticateEx: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindCtx(::windows::core::IUnknown);
impl IBindCtx {
    pub unsafe fn RegisterObjectBound<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterObjectBound)(::windows::core::Vtable::as_raw(self), punk.into().abi()).ok()
    }
    pub unsafe fn RevokeObjectBound<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).RevokeObjectBound)(::windows::core::Vtable::as_raw(self), punk.into().abi()).ok()
    }
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseBoundObjects)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBindOptions)(::windows::core::Vtable::as_raw(self), pbindopts).ok()
    }
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBindOptions)(::windows::core::Vtable::as_raw(self), pbindopts).ok()
    }
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows::core::Result<IRunningObjectTable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRunningObjectTable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterObjectParam<P0, P1>(&self, pszkey: P0, punk: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterObjectParam)(::windows::core::Vtable::as_raw(self), pszkey.into().abi(), punk.into().abi()).ok()
    }
    pub unsafe fn GetObjectParam<P0>(&self, pszkey: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObjectParam)(::windows::core::Vtable::as_raw(self), pszkey.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumObjectParam(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumObjectParam)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RevokeObjectParam<P0>(&self, pszkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).RevokeObjectParam)(::windows::core::Vtable::as_raw(self), pszkey.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IBindCtx, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBindCtx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindCtx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindCtx {}
impl ::core::fmt::Debug for IBindCtx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCtx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBindCtx {
    type Vtable = IBindCtx_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindCtx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterObjectBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeObjectBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseBoundObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBindOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT,
    pub GetBindOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT,
    pub GetRunningObjectTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindHost(::windows::core::IUnknown);
impl IBindHost {
    pub unsafe fn CreateMoniker<P0, P1>(&self, szname: P0, pbc: P1, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).CreateMoniker)(::windows::core::Vtable::as_raw(self), szname.into().abi(), pbc.into().abi(), ::core::mem::transmute(ppmk), dwreserved).ok()
    }
    pub unsafe fn MonikerBindToStorage<P0, P1, P2>(&self, pmk: P0, pbc: P1, pbsc: P2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
        P1: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P2: ::std::convert::Into<::windows::core::InParam<IBindStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).MonikerBindToStorage)(::windows::core::Vtable::as_raw(self), pmk.into().abi(), pbc.into().abi(), pbsc.into().abi(), riid, ppvobj).ok()
    }
    pub unsafe fn MonikerBindToObject<P0, P1, P2>(&self, pmk: P0, pbc: P1, pbsc: P2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
        P1: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P2: ::std::convert::Into<::windows::core::InParam<IBindStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).MonikerBindToObject)(::windows::core::Vtable::as_raw(self), pmk.into().abi(), pbc.into().abi(), pbsc.into().abi(), riid, ppvobj).ok()
    }
}
::windows::core::interface_hierarchy!(IBindHost, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBindHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindHost {}
impl ::core::fmt::Debug for IBindHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBindHost {
    type Vtable = IBindHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
    pub MonikerBindToStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pbsc: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MonikerBindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pbsc: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindStatusCallback(::windows::core::IUnknown);
impl IBindStatusCallback {
    pub unsafe fn OnStartBinding<P0>(&self, dwreserved: u32, pib: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBinding>>,
    {
        (::windows::core::Vtable::vtable(self).OnStartBinding)(::windows::core::Vtable::as_raw(self), dwreserved, pib.into().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnLowResource)(::windows::core::Vtable::as_raw(self), reserved).ok()
    }
    pub unsafe fn OnProgress<P0>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).OnProgress)(::windows::core::Vtable::as_raw(self), ulprogress, ulprogressmax, ulstatuscode, szstatustext.into().abi()).ok()
    }
    pub unsafe fn OnStopBinding<P0>(&self, hresult: ::windows::core::HRESULT, szerror: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).OnStopBinding)(::windows::core::Vtable::as_raw(self), hresult, szerror.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBindInfo)(::windows::core::Vtable::as_raw(self), grfbindf, pbindinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnDataAvailable)(::windows::core::Vtable::as_raw(self), grfbscf, dwsize, pformatetc, pstgmed).ok()
    }
    pub unsafe fn OnObjectAvailable<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).OnObjectAvailable)(::windows::core::Vtable::as_raw(self), riid, punk.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IBindStatusCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBindStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallback {}
impl ::core::fmt::Debug for IBindStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBindStatusCallback {
    type Vtable = IBindStatusCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindStatusCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStartBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub OnLowResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnStopBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfo: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataAvailable: usize,
    pub OnObjectAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindStatusCallbackEx(::windows::core::IUnknown);
impl IBindStatusCallbackEx {
    pub unsafe fn OnStartBinding<P0>(&self, dwreserved: u32, pib: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBinding>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnStartBinding)(::windows::core::Vtable::as_raw(self), dwreserved, pib.into().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnLowResource)(::windows::core::Vtable::as_raw(self), reserved).ok()
    }
    pub unsafe fn OnProgress<P0>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnProgress)(::windows::core::Vtable::as_raw(self), ulprogress, ulprogressmax, ulstatuscode, szstatustext.into().abi()).ok()
    }
    pub unsafe fn OnStopBinding<P0>(&self, hresult: ::windows::core::HRESULT, szerror: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnStopBinding)(::windows::core::Vtable::as_raw(self), hresult, szerror.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBindInfo)(::windows::core::Vtable::as_raw(self), grfbindf, pbindinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnDataAvailable)(::windows::core::Vtable::as_raw(self), grfbscf, dwsize, pformatetc, pstgmed).ok()
    }
    pub unsafe fn OnObjectAvailable<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnObjectAvailable)(::windows::core::Vtable::as_raw(self), riid, punk.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBindInfoEx)(::windows::core::Vtable::as_raw(self), grfbindf, pbindinfo, grfbindf2, pdwreserved).ok()
    }
}
::windows::core::interface_hierarchy!(IBindStatusCallbackEx, ::windows::core::IUnknown, IBindStatusCallback);
impl ::core::clone::Clone for IBindStatusCallbackEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallbackEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallbackEx {}
impl ::core::fmt::Debug for IBindStatusCallbackEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallbackEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindStatusCallbackEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_Vtbl {
    pub base__: IBindStatusCallback_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBinding(::windows::core::IUnknown);
impl IBinding {
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abort)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPriority)(::windows::core::Vtable::as_raw(self), npriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows::core::PWSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBindResult)(::windows::core::Vtable::as_raw(self), pclsidprotocol, pdwresult, pszresult, pdwreserved).ok()
    }
}
::windows::core::interface_hierarchy!(IBinding, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBinding {}
impl ::core::fmt::Debug for IBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBinding {
    type Vtable = IBinding_Vtbl;
}
unsafe impl ::windows::core::Interface for IBinding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub GetBindResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows::core::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBlockingLock(::windows::core::IUnknown);
impl IBlockingLock {
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Lock)(::windows::core::Vtable::as_raw(self), dwtimeout).ok()
    }
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unlock)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IBlockingLock, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBlockingLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBlockingLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBlockingLock {}
impl ::core::fmt::Debug for IBlockingLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockingLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBlockingLock {
    type Vtable = IBlockingLock_Vtbl;
}
unsafe impl ::windows::core::Interface for IBlockingLock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICallFactory(::windows::core::IUnknown);
impl ICallFactory {
    pub unsafe fn CreateCall<P0>(&self, riid: *const ::windows::core::GUID, pctrlunk: P0, riid2: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCall)(::windows::core::Vtable::as_raw(self), riid, pctrlunk.into().abi(), riid2, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ICallFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICallFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFactory {}
impl ::core::fmt::Debug for ICallFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICallFactory {
    type Vtable = ICallFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICallFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICancelMethodCalls(::windows::core::IUnknown);
impl ICancelMethodCalls {
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self), ulseconds).ok()
    }
    pub unsafe fn TestCancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TestCancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ICancelMethodCalls, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICancelMethodCalls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICancelMethodCalls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICancelMethodCalls {}
impl ::core::fmt::Debug for ICancelMethodCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICancelMethodCalls").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICancelMethodCalls {
    type Vtable = ICancelMethodCalls_Vtbl;
}
unsafe impl ::windows::core::Interface for ICancelMethodCalls {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000029_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows::core::HRESULT,
    pub TestCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICatInformation(::windows::core::IUnknown);
impl ICatInformation {
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumCategories)(::windows::core::Vtable::as_raw(self), lcid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows::core::GUID, lcid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCategoryDesc)(::windows::core::Vtable::as_raw(self), rcatid, lcid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumClassesOfCategories(&self, rgcatidimpl: &[::windows::core::GUID], rgcatidreq: &[::windows::core::GUID]) -> ::windows::core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumClassesOfCategories)(::windows::core::Vtable::as_raw(self), rgcatidimpl.len() as _, ::core::mem::transmute(rgcatidimpl.as_ptr()), rgcatidreq.len() as _, ::core::mem::transmute(rgcatidreq.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows::core::GUID, rgcatidimpl: &[::windows::core::GUID], rgcatidreq: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsClassOfCategories)(::windows::core::Vtable::as_raw(self), rclsid, rgcatidimpl.len() as _, ::core::mem::transmute(rgcatidimpl.as_ptr()), rgcatidreq.len() as _, ::core::mem::transmute(rgcatidreq.as_ptr())).ok()
    }
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumImplCategoriesOfClass)(::windows::core::Vtable::as_raw(self), rclsid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumReqCategoriesOfClass)(::windows::core::Vtable::as_raw(self), rclsid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ICatInformation, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICatInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatInformation {}
impl ::core::fmt::Debug for ICatInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICatInformation {
    type Vtable = ICatInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for ICatInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e013_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCategoryDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub EnumClassesOfCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsClassOfCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICatRegister(::windows::core::IUnknown);
impl ICatRegister {
    pub unsafe fn RegisterCategories(&self, rgcategoryinfo: &[CATEGORYINFO]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterCategories)(::windows::core::Vtable::as_raw(self), rgcategoryinfo.len() as _, ::core::mem::transmute(rgcategoryinfo.as_ptr())).ok()
    }
    pub unsafe fn UnRegisterCategories(&self, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnRegisterCategories)(::windows::core::Vtable::as_raw(self), rgcatid.len() as _, ::core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterClassImplCategories)(::windows::core::Vtable::as_raw(self), rclsid, rgcatid.len() as _, ::core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnRegisterClassImplCategories)(::windows::core::Vtable::as_raw(self), rclsid, rgcatid.len() as _, ::core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterClassReqCategories)(::windows::core::Vtable::as_raw(self), rclsid, rgcatid.len() as _, ::core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnRegisterClassReqCategories)(::windows::core::Vtable::as_raw(self), rclsid, rgcatid.len() as _, ::core::mem::transmute(rgcatid.as_ptr())).ok()
    }
}
::windows::core::interface_hierarchy!(ICatRegister, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICatRegister {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatRegister {}
impl ::core::fmt::Debug for ICatRegister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatRegister").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICatRegister {
    type Vtable = ICatRegister_Vtbl;
}
unsafe impl ::windows::core::Interface for ICatRegister {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT,
    pub UnRegisterCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterClassImplCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterClassReqCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IChannelHook(::windows::core::IUnknown);
impl IChannelHook {
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> u32 {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClientGetSize)(::windows::core::Vtable::as_raw(self), uextent, riid, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).ClientFillBuffer)(::windows::core::Vtable::as_raw(self), uextent, riid, pdatasize, pdatabuffer)
    }
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
        (::windows::core::Vtable::vtable(self).ClientNotify)(::windows::core::Vtable::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep, hrfault)
    }
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
        (::windows::core::Vtable::vtable(self).ServerNotify)(::windows::core::Vtable::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep)
    }
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT) -> u32 {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerGetSize)(::windows::core::Vtable::as_raw(self), uextent, riid, hrfault, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
        (::windows::core::Vtable::vtable(self).ServerFillBuffer)(::windows::core::Vtable::as_raw(self), uextent, riid, pdatasize, pdatabuffer, hrfault)
    }
}
::windows::core::interface_hierarchy!(IChannelHook, ::windows::core::IUnknown);
impl ::core::clone::Clone for IChannelHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelHook {}
impl ::core::fmt::Debug for IChannelHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IChannelHook {
    type Vtable = IChannelHook_Vtbl;
}
unsafe impl ::windows::core::Interface for IChannelHook {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ClientGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32),
    pub ClientFillBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void),
    pub ClientNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT),
    pub ServerNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32),
    pub ServerGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32),
    pub ServerFillBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IClassActivator(::windows::core::IUnknown);
impl IClassActivator {
    pub unsafe fn GetClassObject<T>(&self, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClassObject)(::windows::core::Vtable::as_raw(self), rclsid, dwclasscontext, locale, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IClassActivator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IClassActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassActivator {}
impl ::core::fmt::Debug for IClassActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IClassActivator {
    type Vtable = IClassActivator_Vtbl;
}
unsafe impl ::windows::core::Interface for IClassActivator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000140_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClassObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IClassFactory(::windows::core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstance)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockServer<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).LockServer)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IClassFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassFactory {}
impl ::core::fmt::Debug for IClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IClassFactory {
    type Vtable = IClassFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IClassFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockServer: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IClientSecurity(::windows::core::IUnknown);
impl IClientSecurity {
    pub unsafe fn QueryBlanket<P0>(&self, pproxy: P0, pauthnsvc: *mut u32, pauthzsvc: ::core::option::Option<*mut u32>, pserverprincname: *mut *mut u16, pauthnlevel: ::core::option::Option<*mut RPC_C_AUTHN_LEVEL>, pimplevel: ::core::option::Option<*mut RPC_C_IMP_LEVEL>, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: ::core::option::Option<*mut EOLE_AUTHENTICATION_CAPABILITIES>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).QueryBlanket)(::windows::core::Vtable::as_raw(self), pproxy.into().abi(), pauthnsvc, ::core::mem::transmute(pauthzsvc.unwrap_or(::std::ptr::null_mut())), pserverprincname, ::core::mem::transmute(pauthnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pimplevel.unwrap_or(::std::ptr::null_mut())), pauthinfo, ::core::mem::transmute(pcapabilites.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetBlanket<P0, P1>(&self, pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: P1, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: ::core::option::Option<*const ::core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetBlanket)(::windows::core::Vtable::as_raw(self), pproxy.into().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.into().abi(), dwauthnlevel, dwimplevel, ::core::mem::transmute(pauthinfo.unwrap_or(::std::ptr::null())), dwcapabilities).ok()
    }
    pub unsafe fn CopyProxy<P0>(&self, pproxy: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CopyProxy)(::windows::core::Vtable::as_raw(self), pproxy.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IClientSecurity, ::windows::core::IUnknown);
impl ::core::clone::Clone for IClientSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClientSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClientSecurity {}
impl ::core::fmt::Debug for IClientSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClientSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IClientSecurity {
    type Vtable = IClientSecurity_Vtbl;
}
unsafe impl ::windows::core::Interface for IClientSecurity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    pub SetBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows::core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    pub CopyProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IComThreadingInfo(::windows::core::IUnknown);
impl IComThreadingInfo {
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows::core::Result<APTTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentApartmentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows::core::Result<THDTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentThreadType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentLogicalThreadId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCurrentLogicalThreadId)(::windows::core::Vtable::as_raw(self), rguid).ok()
    }
}
::windows::core::interface_hierarchy!(IComThreadingInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IComThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadingInfo {}
impl ::core::fmt::Debug for IComThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IComThreadingInfo {
    type Vtable = IComThreadingInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IComThreadingInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001ce_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentApartmentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows::core::HRESULT,
    pub GetCurrentThreadType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT,
    pub GetCurrentLogicalThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetCurrentLogicalThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IConnectionPoint(::windows::core::IUnknown);
impl IConnectionPoint {
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConnectionInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::core::Result<IConnectionPointContainer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConnectionPointContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Advise<P0>(&self, punksink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), punksink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn EnumConnections(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumConnections)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IConnectionPoint, ::windows::core::IUnknown);
impl ::core::clone::Clone for IConnectionPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPoint {}
impl ::core::fmt::Debug for IConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IConnectionPoint {
    type Vtable = IConnectionPoint_Vtbl;
}
unsafe impl ::windows::core::Interface for IConnectionPoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b286_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetConnectionInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetConnectionPointContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcpc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub EnumConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IConnectionPointContainer(::windows::core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumConnectionPoints)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<IConnectionPoint> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindConnectionPoint)(::windows::core::Vtable::as_raw(self), riid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IConnectionPointContainer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IConnectionPointContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPointContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPointContainer {}
impl ::core::fmt::Debug for IConnectionPointContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPointContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IConnectionPointContainer {
    type Vtable = IConnectionPointContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IConnectionPointContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b284_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindConnectionPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IContextCallback(::windows::core::IUnknown);
impl IContextCallback {
    pub unsafe fn ContextCallback<P0>(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ContextCallback)(::windows::core::Vtable::as_raw(self), pfncallback, pparam, riid, imethod, punk.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IContextCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContextCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextCallback {}
impl ::core::fmt::Debug for IContextCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContextCallback {
    type Vtable = IContextCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IContextCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001da_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ContextCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IDataAdviseHolder(::windows::core::IUnknown);
impl IDataAdviseHolder {
    pub unsafe fn Advise<P0, P1>(&self, pdataobject: P0, pfetc: *const FORMATETC, advf: u32, padvise: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDataObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<IAdviseSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi(), pfetc, advf, padvise.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unadvise)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    pub unsafe fn EnumAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumAdvise)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOnDataChange<P0>(&self, pdataobject: P0, dwreserved: u32, advf: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).SendOnDataChange)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi(), dwreserved, advf).ok()
    }
}
::windows::core::interface_hierarchy!(IDataAdviseHolder, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDataAdviseHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataAdviseHolder {}
impl ::core::fmt::Debug for IDataAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataAdviseHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDataAdviseHolder {
    type Vtable = IDataAdviseHolder_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataAdviseHolder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000110_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, pfetc: *const FORMATETC, advf: u32, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    pub EnumAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendOnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IDataObject(::windows::core::IUnknown);
impl IDataObject {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::core::Result<STGMEDIUM> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetData)(::windows::core::Vtable::as_raw(self), pformatetcin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDataHere)(::windows::core::Vtable::as_raw(self), pformatetc, pmedium).ok()
    }
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).QueryGetData)(::windows::core::Vtable::as_raw(self), pformatetc)
    }
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).GetCanonicalFormatEtc)(::windows::core::Vtable::as_raw(self), pformatectin, pformatetcout)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<P0>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetData)(::windows::core::Vtable::as_raw(self), pformatetc, pmedium, frelease.into()).ok()
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumFormatEtc)(::windows::core::Vtable::as_raw(self), dwdirection, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DAdvise<P0>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAdviseSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DAdvise)(::windows::core::Vtable::as_raw(self), pformatetc, advf, padvsink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DUnadvise)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    pub unsafe fn EnumDAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumDAdvise)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDataObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataObject {}
impl ::core::fmt::Debug for IDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDataObject {
    type Vtable = IDataObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetData: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetDataHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetDataHere: usize,
    pub QueryGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT,
    pub GetCanonicalFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
    pub EnumFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub DUnadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IDispatch(::windows::core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeInfoCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeInfo)(::windows::core::Vtable::as_raw(self), itinfo, lcid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const ::windows::core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetIDsOfNames)(::windows::core::Vtable::as_raw(self), riid, rgsznames, cnames, lcid, rgdispid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: ::core::option::Option<*mut VARIANT>, pexcepinfo: ::core::option::Option<*mut EXCEPINFO>, puargerr: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Invoke)(::windows::core::Vtable::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, ::core::mem::transmute(pvarresult.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pexcepinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(puargerr.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IDispatch, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispatch {}
impl ::core::fmt::Debug for IDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDispatch {
    type Vtable = IDispatch_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const ::windows::core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumCATEGORYINFO(::windows::core::IUnknown);
impl IEnumCATEGORYINFO {
    pub unsafe fn Next(&self, rgelt: &mut [CATEGORYINFO], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumCATEGORYINFO, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumCATEGORYINFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumCATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCATEGORYINFO {}
impl ::core::fmt::Debug for IEnumCATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCATEGORYINFO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFO_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumCATEGORYINFO {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e011_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumConnectionPoints(::windows::core::IUnknown);
impl IEnumConnectionPoints {
    pub unsafe fn Next(&self, ppcp: &mut [::core::option::Option<IConnectionPoint>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppcp.len() as _, ::core::mem::transmute(ppcp.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), cconnections).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumConnectionPoints, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumConnectionPoints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnectionPoints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnectionPoints {}
impl ::core::fmt::Debug for IEnumConnectionPoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnectionPoints").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPoints_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumConnectionPoints {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b285_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumConnections(::windows::core::IUnknown);
impl IEnumConnections {
    pub unsafe fn Next(&self, rgcd: &mut [CONNECTDATA], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgcd.len() as _, ::core::mem::transmute(rgcd.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), cconnections).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumConnections, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnections {}
impl ::core::fmt::Debug for IEnumConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumConnections {
    type Vtable = IEnumConnections_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumConnections {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b287_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumFORMATETC(::windows::core::IUnknown);
impl IEnumFORMATETC {
    pub unsafe fn Next(&self, rgelt: &mut [FORMATETC], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumFORMATETC, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumFORMATETC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFORMATETC {}
impl ::core::fmt::Debug for IEnumFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFORMATETC").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumFORMATETC {
    type Vtable = IEnumFORMATETC_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumFORMATETC {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000103_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumGUID(::windows::core::IUnknown);
impl IEnumGUID {
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::GUID], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumGUID, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumGUID {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumGUID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumGUID {}
impl ::core::fmt::Debug for IEnumGUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumGUID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumGUID {
    type Vtable = IEnumGUID_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumGUID {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumMoniker(::windows::core::IUnknown);
impl IEnumMoniker {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IMoniker>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumMoniker, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMoniker {}
impl ::core::fmt::Debug for IEnumMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumMoniker {
    type Vtable = IEnumMoniker_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumMoniker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000102_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumSTATDATA(::windows::core::IUnknown);
impl IEnumSTATDATA {
    pub unsafe fn Next(&self, rgelt: &mut [STATDATA], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumSTATDATA, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumSTATDATA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATDATA {}
impl ::core::fmt::Debug for IEnumSTATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATDATA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumSTATDATA {
    type Vtable = IEnumSTATDATA_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumSTATDATA {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000105_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumString(::windows::core::IUnknown);
impl IEnumString {
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::PWSTR], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumString, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumString {}
impl ::core::fmt::Debug for IEnumString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumString {
    type Vtable = IEnumString_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumString {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000101_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumUnknown(::windows::core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<::windows::core::IUnknown>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumUnknown, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumUnknown {}
impl ::core::fmt::Debug for IEnumUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumUnknown {
    type Vtable = IEnumUnknown_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumUnknown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000100_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IErrorInfo(::windows::core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHelpFile(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHelpFile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHelpContext(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHelpContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IErrorInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorInfo {}
impl ::core::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IErrorLog(::windows::core::IUnknown);
impl IErrorLog {
    pub unsafe fn AddError<P0>(&self, pszpropname: P0, pexcepinfo: *const EXCEPINFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddError)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pexcepinfo).ok()
    }
}
::windows::core::interface_hierarchy!(IErrorLog, ::windows::core::IUnknown);
impl ::core::clone::Clone for IErrorLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorLog {}
impl ::core::fmt::Debug for IErrorLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorLog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IErrorLog {
    type Vtable = IErrorLog_Vtbl;
}
unsafe impl ::windows::core::Interface for IErrorLog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3127ca40_446e_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IExternalConnection(::windows::core::IUnknown);
impl IExternalConnection {
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).AddConnection)(::windows::core::Vtable::as_raw(self), extconn, reserved)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseConnection<P0>(&self, extconn: u32, reserved: u32, flastreleasecloses: P0) -> u32
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ReleaseConnection)(::windows::core::Vtable::as_raw(self), extconn, reserved, flastreleasecloses.into())
    }
}
::windows::core::interface_hierarchy!(IExternalConnection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IExternalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExternalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExternalConnection {}
impl ::core::fmt::Debug for IExternalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExternalConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IExternalConnection {
    type Vtable = IExternalConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IExternalConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000019_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseConnection: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IFastRundown(::windows::core::IUnknown);
impl IFastRundown {}
::windows::core::interface_hierarchy!(IFastRundown, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFastRundown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFastRundown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFastRundown {}
impl ::core::fmt::Debug for IFastRundown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFastRundown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFastRundown {
    type Vtable = IFastRundown_Vtbl;
}
unsafe impl ::windows::core::Interface for IFastRundown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000040_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IForegroundTransfer(::windows::core::IUnknown);
impl IForegroundTransfer {
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AllowForegroundTransfer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lpvreserved.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IForegroundTransfer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IForegroundTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForegroundTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForegroundTransfer {}
impl ::core::fmt::Debug for IForegroundTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForegroundTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IForegroundTransfer {
    type Vtable = IForegroundTransfer_Vtbl;
}
unsafe impl ::windows::core::Interface for IForegroundTransfer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000145_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AllowForegroundTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IGlobalInterfaceTable(::windows::core::IUnknown);
impl IGlobalInterfaceTable {
    pub unsafe fn RegisterInterfaceInGlobal<P0>(&self, punk: P0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterInterfaceInGlobal)(::windows::core::Vtable::as_raw(self), punk.into().abi(), riid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RevokeInterfaceFromGlobal)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInterfaceFromGlobal)(::windows::core::Vtable::as_raw(self), dwcookie, riid, ppv).ok()
    }
}
::windows::core::interface_hierarchy!(IGlobalInterfaceTable, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGlobalInterfaceTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalInterfaceTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalInterfaceTable {}
impl ::core::fmt::Debug for IGlobalInterfaceTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalInterfaceTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTable_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlobalInterfaceTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000146_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IGlobalOptions(::windows::core::IUnknown);
impl IGlobalOptions {
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Set)(::windows::core::Vtable::as_raw(self), dwproperty, dwvalue).ok()
    }
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Query)(::windows::core::Vtable::as_raw(self), dwproperty, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IGlobalOptions, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGlobalOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalOptions {}
impl ::core::fmt::Debug for IGlobalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IGlobalOptions {
    type Vtable = IGlobalOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlobalOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000015b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IInitializeSpy(::windows::core::IUnknown);
impl IInitializeSpy {
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PreInitialize)(::windows::core::Vtable::as_raw(self), dwcoinit, dwcurthreadaptrefs).ok()
    }
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PostInitialize)(::windows::core::Vtable::as_raw(self), hrcoinit, dwcoinit, dwnewthreadaptrefs).ok()
    }
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PreUninitialize)(::windows::core::Vtable::as_raw(self), dwcurthreadaptrefs).ok()
    }
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PostUninitialize)(::windows::core::Vtable::as_raw(self), dwnewthreadaptrefs).ok()
    }
}
::windows::core::interface_hierarchy!(IInitializeSpy, ::windows::core::IUnknown);
impl ::core::clone::Clone for IInitializeSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitializeSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeSpy {}
impl ::core::fmt::Debug for IInitializeSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IInitializeSpy {
    type Vtable = IInitializeSpy_Vtbl;
}
unsafe impl ::windows::core::Interface for IInitializeSpy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000034_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PreInitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub PostInitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub PreUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub PostUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IInternalUnknown(::windows::core::IUnknown);
impl IInternalUnknown {
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryInternalInterface)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
}
::windows::core::interface_hierarchy!(IInternalUnknown, ::windows::core::IUnknown);
impl ::core::clone::Clone for IInternalUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInternalUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalUnknown {}
impl ::core::fmt::Debug for IInternalUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IInternalUnknown {
    type Vtable = IInternalUnknown_Vtbl;
}
unsafe impl ::windows::core::Interface for IInternalUnknown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000021_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryInternalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMachineGlobalObjectTable(::windows::core::IUnknown);
impl IMachineGlobalObjectTable {
    pub unsafe fn RegisterObject<P0, P1>(&self, clsid: *const ::windows::core::GUID, identifier: P0, object: P1) -> ::windows::core::Result<*mut MachineGlobalObjectTableRegistrationToken__>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterObject)(::windows::core::Vtable::as_raw(self), clsid, identifier.into().abi(), object.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetObject<P0, T>(&self, clsid: *const ::windows::core::GUID, identifier: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObject)(::windows::core::Vtable::as_raw(self), clsid, identifier.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RevokeObject)(::windows::core::Vtable::as_raw(self), token).ok()
    }
}
::windows::core::interface_hierarchy!(IMachineGlobalObjectTable, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMachineGlobalObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMachineGlobalObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineGlobalObjectTable {}
impl ::core::fmt::Debug for IMachineGlobalObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineGlobalObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTable_Vtbl;
}
unsafe impl ::windows::core::Interface for IMachineGlobalObjectTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d709ac_f70b_4421_a96f_d2878fafb00d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: ::windows::core::PCWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMalloc(::windows::core::IUnknown);
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).Alloc)(::windows::core::Vtable::as_raw(self), cb)
    }
    pub unsafe fn Realloc(&self, pv: ::core::option::Option<*const ::core::ffi::c_void>, cb: usize) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).Realloc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), cb)
    }
    pub unsafe fn Free(&self, pv: ::core::option::Option<*const ::core::ffi::c_void>) {
        (::windows::core::Vtable::vtable(self).Free)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn GetSize(&self, pv: ::core::option::Option<*const ::core::ffi::c_void>) -> usize {
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DidAlloc(&self, pv: ::core::option::Option<*const ::core::ffi::c_void>) -> i32 {
        (::windows::core::Vtable::vtable(self).DidAlloc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn HeapMinimize(&self) {
        (::windows::core::Vtable::vtable(self).HeapMinimize)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IMalloc, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMalloc {}
impl ::core::fmt::Debug for IMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMalloc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMalloc {
    type Vtable = IMalloc_Vtbl;
}
unsafe impl ::windows::core::Interface for IMalloc {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000002_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub Realloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize,
    pub DidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMallocSpy(::windows::core::IUnknown);
impl IMallocSpy {
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        (::windows::core::Vtable::vtable(self).PreAlloc)(::windows::core::Vtable::as_raw(self), cbrequest)
    }
    pub unsafe fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).PostAlloc)(::windows::core::Vtable::as_raw(self), pactual)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreFree<P0>(&self, prequest: *const ::core::ffi::c_void, fspyed: P0) -> *mut ::core::ffi::c_void
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PreFree)(::windows::core::Vtable::as_raw(self), prequest, fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostFree<P0>(&self, fspyed: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PostFree)(::windows::core::Vtable::as_raw(self), fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreRealloc<P0>(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: P0) -> usize
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PreRealloc)(::windows::core::Vtable::as_raw(self), prequest, cbrequest, ppnewrequest, fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostRealloc<P0>(&self, pactual: *const ::core::ffi::c_void, fspyed: P0) -> *mut ::core::ffi::c_void
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PostRealloc)(::windows::core::Vtable::as_raw(self), pactual, fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreGetSize<P0>(&self, prequest: *const ::core::ffi::c_void, fspyed: P0) -> *mut ::core::ffi::c_void
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PreGetSize)(::windows::core::Vtable::as_raw(self), prequest, fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostGetSize<P0>(&self, cbactual: usize, fspyed: P0) -> usize
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PostGetSize)(::windows::core::Vtable::as_raw(self), cbactual, fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreDidAlloc<P0>(&self, prequest: *const ::core::ffi::c_void, fspyed: P0) -> *mut ::core::ffi::c_void
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PreDidAlloc)(::windows::core::Vtable::as_raw(self), prequest, fspyed.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostDidAlloc<P0>(&self, prequest: *const ::core::ffi::c_void, fspyed: P0, factual: i32) -> i32
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PostDidAlloc)(::windows::core::Vtable::as_raw(self), prequest, fspyed.into(), factual)
    }
    pub unsafe fn PreHeapMinimize(&self) {
        (::windows::core::Vtable::vtable(self).PreHeapMinimize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PostHeapMinimize(&self) {
        (::windows::core::Vtable::vtable(self).PostHeapMinimize)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IMallocSpy, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMallocSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMallocSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMallocSpy {}
impl ::core::fmt::Debug for IMallocSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMallocSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMallocSpy {
    type Vtable = IMallocSpy_Vtbl;
}
unsafe impl ::windows::core::Interface for IMallocSpy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000001d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PreAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize,
    pub PostAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    #[cfg(feature = "Win32_Foundation")]
    pub PreFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreFree: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    PostFree: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreRealloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreRealloc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostRealloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostRealloc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreGetSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostGetSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreDidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreDidAlloc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostDidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostDidAlloc: usize,
    pub PreHeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PostHeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMoniker(::windows::core::IUnknown);
impl IMoniker {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BindToObject<P0, P1>(&self, pbc: P0, pmktoleft: P1, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).BindToObject)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pmktoleft.into().abi(), riidresult, ppvresult).ok()
    }
    pub unsafe fn BindToStorage<P0, P1>(&self, pbc: P0, pmktoleft: P1, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).BindToStorage)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pmktoleft.into().abi(), riid, ppvobj).ok()
    }
    pub unsafe fn Reduce<P0>(&self, pbc: P0, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).Reduce)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), dwreducehowfar, ::core::mem::transmute(ppmktoleft), ::core::mem::transmute(ppmkreduced)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComposeWith<P0, P1>(&self, pmkright: P0, fonlyifnotgeneric: P1) -> ::windows::core::Result<IMoniker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComposeWith)(::windows::core::Vtable::as_raw(self), pmkright.into().abi(), fonlyifnotgeneric.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enum<P0>(&self, fforward: P0) -> ::windows::core::Result<IEnumMoniker>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Enum)(::windows::core::Vtable::as_raw(self), fforward.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsEqual<P0>(&self, pmkothermoniker: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).IsEqual)(::windows::core::Vtable::as_raw(self), pmkothermoniker.into().abi()).ok()
    }
    pub unsafe fn Hash(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Hash)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsRunning<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pmknewlyrunning: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
        P2: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).IsRunning)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pmktoleft.into().abi(), pmknewlyrunning.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> ::windows::core::Result<super::super::Foundation::FILETIME>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTimeOfLastChange)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pmktoleft.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Inverse(&self) -> ::windows::core::Result<IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Inverse)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CommonPrefixWith<P0>(&self, pmkother: P0) -> ::windows::core::Result<IMoniker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommonPrefixWith)(::windows::core::Vtable::as_raw(self), pmkother.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RelativePathTo<P0>(&self, pmkother: P0) -> ::windows::core::Result<IMoniker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RelativePathTo)(::windows::core::Vtable::as_raw(self), pmkother.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayName<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pmktoleft.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ParseDisplayName<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pszdisplayname: P2, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ParseDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pmktoleft.into().abi(), pszdisplayname.into().abi(), pcheaten, ::core::mem::transmute(ppmkout)).ok()
    }
    pub unsafe fn IsSystemMoniker(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsSystemMoniker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMoniker, ::windows::core::IUnknown, IPersist, IPersistStream);
impl ::core::clone::Clone for IMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMoniker {}
impl ::core::fmt::Debug for IMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMoniker {
    type Vtable = IMoniker_Vtbl;
}
unsafe impl ::windows::core::Interface for IMoniker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_Vtbl {
    pub base__: IPersistStream_Vtbl,
    pub BindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BindToStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reduce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, dwreducehowfar: u32, ppmktoleft: *mut *mut ::core::ffi::c_void, ppmkreduced: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ComposeWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkright: *mut ::core::ffi::c_void, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComposeWith: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enum: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkothermoniker: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows::core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pmknewlyrunning: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimeOfLastChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimeOfLastChange: usize,
    pub Inverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CommonPrefixWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: *mut ::core::ffi::c_void, ppmkprefix: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RelativePathTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: *mut ::core::ffi::c_void, ppmkrelpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, ppszdisplayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub ParseDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pszdisplayname: ::windows::core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSystemMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMultiQI(::windows::core::IUnknown);
impl IMultiQI {
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryMultipleInterfaces)(::windows::core::Vtable::as_raw(self), pmqis.len() as _, ::core::mem::transmute(pmqis.as_ptr())).ok()
    }
}
::windows::core::interface_hierarchy!(IMultiQI, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiQI {}
impl ::core::fmt::Debug for IMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMultiQI {
    type Vtable = IMultiQI_Vtbl;
}
unsafe impl ::windows::core::Interface for IMultiQI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct INoMarshal(::windows::core::IUnknown);
impl INoMarshal {}
::windows::core::interface_hierarchy!(INoMarshal, ::windows::core::IUnknown);
impl ::core::clone::Clone for INoMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INoMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INoMarshal {}
impl ::core::fmt::Debug for INoMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INoMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for INoMarshal {
    type Vtable = INoMarshal_Vtbl;
}
unsafe impl ::windows::core::Interface for INoMarshal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc8691b_c1db_4dc0_855e_65f6c551af49);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IOplockStorage(::windows::core::IUnknown);
impl IOplockStorage {
    pub unsafe fn CreateStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStorageEx)(::windows::core::Vtable::as_raw(self), pwcsname.into().abi(), grfmode, stgfmt, grfattrs, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenStorageEx)(::windows::core::Vtable::as_raw(self), pwcsname.into().abi(), grfmode, stgfmt, grfattrs, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOplockStorage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOplockStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOplockStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockStorage {}
impl ::core::fmt::Debug for IOplockStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOplockStorage {
    type Vtable = IOplockStorage_Vtbl;
}
unsafe impl ::windows::core::Interface for IOplockStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateStorageEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenStorageEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPSFactoryBuffer(::windows::core::IUnknown);
impl IPSFactoryBuffer {
    pub unsafe fn CreateProxy<P0>(&self, punkouter: P0, riid: *const ::windows::core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).CreateProxy)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), riid, ::core::mem::transmute(ppproxy), ppv).ok()
    }
    pub unsafe fn CreateStub<P0>(&self, riid: *const ::windows::core::GUID, punkserver: P0) -> ::windows::core::Result<IRpcStubBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStub)(::windows::core::Vtable::as_raw(self), riid, punkserver.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPSFactoryBuffer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPSFactoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPSFactoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPSFactoryBuffer {}
impl ::core::fmt::Debug for IPSFactoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSFactoryBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPSFactoryBuffer {
    type Vtable = IPSFactoryBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IPSFactoryBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f569d0_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppproxy: *mut *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersist(::windows::core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPersist, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPersist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersist {}
impl ::core::fmt::Debug for IPersist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersist").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPersist {
    type Vtable = IPersist_Vtbl;
}
unsafe impl ::windows::core::Interface for IPersist {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistFile(::windows::core::IUnknown);
impl IPersistFile {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pszfilename: P0, dwmode: STGM) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), pszfilename.into().abi(), dwmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1>(&self, pszfilename: P0, fremember: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), pszfilename.into().abi(), fremember.into()).ok()
    }
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SaveCompleted)(::windows::core::Vtable::as_raw(self), pszfilename.into().abi()).ok()
    }
    pub unsafe fn GetCurFile(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurFile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPersistFile, ::windows::core::IUnknown, IPersist);
impl ::core::clone::Clone for IPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistFile {}
impl ::core::fmt::Debug for IPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPersistFile {
    type Vtable = IPersistFile_Vtbl;
}
unsafe impl ::windows::core::Interface for IPersistFile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, dwmode: STGM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszfilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistMemory(::windows::core::IUnknown);
impl IPersistMemory {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Load(&self, pmem: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmem.as_ptr()), pmem.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0>(&self, pmem: &mut [u8], fcleardirty: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pmem.as_ptr()), fcleardirty.into(), pmem.len() as _).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InitNew)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPersistMemory, ::windows::core::IUnknown, IPersist);
impl ::core::clone::Clone for IPersistMemory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistMemory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMemory {}
impl ::core::fmt::Debug for IPersistMemory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMemory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPersistMemory {
    type Vtable = IPersistMemory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPersistMemory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistStream(::windows::core::IUnknown);
impl IPersistStream {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPersistStream, ::windows::core::IUnknown, IPersist);
impl ::core::clone::Clone for IPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStream {}
impl ::core::fmt::Debug for IPersistStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPersistStream {
    type Vtable = IPersistStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IPersistStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000109_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistStreamInit(::windows::core::IUnknown);
impl IPersistStreamInit {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InitNew)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPersistStreamInit, ::windows::core::IUnknown, IPersist);
impl ::core::clone::Clone for IPersistStreamInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStreamInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStreamInit {}
impl ::core::fmt::Debug for IPersistStreamInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStreamInit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPersistStreamInit {
    type Vtable = IPersistStreamInit_Vtbl;
}
unsafe impl ::windows::core::Interface for IPersistStreamInit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fd52380_4e07_101b_ae2d_08002b2ec713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPipeByte(::windows::core::IUnknown);
impl IPipeByte {
    pub unsafe fn Pull(&self, buf: &mut [u8], pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pull)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _, pcreturned).ok()
    }
    pub unsafe fn Push(&self, buf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Push)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IPipeByte, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeByte {}
impl ::core::fmt::Debug for IPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPipeByte {
    type Vtable = IPipeByte_Vtbl;
}
unsafe impl ::windows::core::Interface for IPipeByte {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPipeDouble(::windows::core::IUnknown);
impl IPipeDouble {
    pub unsafe fn Pull(&self, buf: &mut [f64], pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pull)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _, pcreturned).ok()
    }
    pub unsafe fn Push(&self, buf: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Push)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IPipeDouble, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeDouble {}
impl ::core::fmt::Debug for IPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPipeDouble {
    type Vtable = IPipeDouble_Vtbl;
}
unsafe impl ::windows::core::Interface for IPipeDouble {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPipeLong(::windows::core::IUnknown);
impl IPipeLong {
    pub unsafe fn Pull(&self, buf: &mut [i32], pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pull)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _, pcreturned).ok()
    }
    pub unsafe fn Push(&self, buf: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Push)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buf.as_ptr()), buf.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IPipeLong, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeLong {}
impl ::core::fmt::Debug for IPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPipeLong {
    type Vtable = IPipeLong_Vtbl;
}
unsafe impl ::windows::core::Interface for IPipeLong {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IProcessInitControl(::windows::core::IUnknown);
impl IProcessInitControl {
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ResetInitializerTimeout)(::windows::core::Vtable::as_raw(self), dwsecondsremaining).ok()
    }
}
::windows::core::interface_hierarchy!(IProcessInitControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProcessInitControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessInitControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitControl {}
impl ::core::fmt::Debug for IProcessInitControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProcessInitControl {
    type Vtable = IProcessInitControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IProcessInitControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ResetInitializerTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IProcessLock(::windows::core::IUnknown);
impl IProcessLock {
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).AddRefOnProcess)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).ReleaseRefOnProcess)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IProcessLock, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProcessLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessLock {}
impl ::core::fmt::Debug for IProcessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProcessLock {
    type Vtable = IProcessLock_Vtbl;
}
unsafe impl ::windows::core::Interface for IProcessLock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d5_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddRefOnProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseRefOnProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IProgressNotify(::windows::core::IUnknown);
impl IProgressNotify {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<P0, P1>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: P0, fowner: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnProgress)(::windows::core::Vtable::as_raw(self), dwprogresscurrent, dwprogressmaximum, faccurate.into(), fowner.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IProgressNotify, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProgressNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProgressNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProgressNotify {}
impl ::core::fmt::Debug for IProgressNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProgressNotify {
    type Vtable = IProgressNotify_Vtbl;
}
unsafe impl ::windows::core::Interface for IProgressNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d758a0_4617_11cf_95fc_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnProgress: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IROTData(::windows::core::IUnknown);
impl IROTData {
    pub unsafe fn GetComparisonData(&self, pbdata: &mut [u8], pcbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetComparisonData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbdata.as_ptr()), pbdata.len() as _, pcbdata).ok()
    }
}
::windows::core::interface_hierarchy!(IROTData, ::windows::core::IUnknown);
impl ::core::clone::Clone for IROTData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IROTData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IROTData {}
impl ::core::fmt::Debug for IROTData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IROTData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IROTData {
    type Vtable = IROTData_Vtbl;
}
unsafe impl ::windows::core::Interface for IROTData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29f6bc0_5021_11ce_aa15_00006901293f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetComparisonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IReleaseMarshalBuffers(::windows::core::IUnknown);
impl IReleaseMarshalBuffers {
    pub unsafe fn ReleaseMarshalBuffer<P0>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ReleaseMarshalBuffer)(::windows::core::Vtable::as_raw(self), pmsg, dwflags, pchnl.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IReleaseMarshalBuffers, ::windows::core::IUnknown);
impl ::core::clone::Clone for IReleaseMarshalBuffers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReleaseMarshalBuffers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReleaseMarshalBuffers {}
impl ::core::fmt::Debug for IReleaseMarshalBuffers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReleaseMarshalBuffers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffers_Vtbl;
}
unsafe impl ::windows::core::Interface for IReleaseMarshalBuffers {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb0cb9e8_7996_11d2_872e_0000f8080859);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer(::windows::core::IUnknown);
impl IRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBuffer)(::windows::core::Vtable::as_raw(self), pmessage, riid).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendReceive)(::windows::core::Vtable::as_raw(self), pmessage, ::core::mem::transmute(pstatus.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FreeBuffer)(::windows::core::Vtable::as_raw(self), pmessage).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDestCtx)(::windows::core::Vtable::as_raw(self), pdwdestcontext, ppvdestcontext).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsConnected)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IRpcChannelBuffer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer {}
impl ::core::fmt::Debug for IRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcChannelBuffer {
    type Vtable = IRpcChannelBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56b60_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SendReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub GetDestCtx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer2(::windows::core::IUnknown);
impl IRpcChannelBuffer2 {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), pmessage, riid).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendReceive)(::windows::core::Vtable::as_raw(self), pmessage, ::core::mem::transmute(pstatus.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreeBuffer)(::windows::core::Vtable::as_raw(self), pmessage).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDestCtx)(::windows::core::Vtable::as_raw(self), pdwdestcontext, ppvdestcontext).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsConnected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProtocolVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRpcChannelBuffer2, ::windows::core::IUnknown, IRpcChannelBuffer);
impl ::core::clone::Clone for IRpcChannelBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer2 {}
impl ::core::fmt::Debug for IRpcChannelBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_Vtbl {
    pub base__: IRpcChannelBuffer_Vtbl,
    pub GetProtocolVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer3(::windows::core::IUnknown);
impl IRpcChannelBuffer3 {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), pmessage, riid).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendReceive)(::windows::core::Vtable::as_raw(self), pmessage, ::core::mem::transmute(pstatus.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FreeBuffer)(::windows::core::Vtable::as_raw(self), pmessage).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDestCtx)(::windows::core::Vtable::as_raw(self), pdwdestcontext, ppvdestcontext).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IsConnected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProtocolVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Send)(::windows::core::Vtable::as_raw(self), pmsg, pulstatus).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Receive)(::windows::core::Vtable::as_raw(self), pmsg, ulsize, pulstatus).ok()
    }
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCallContext)(::windows::core::Vtable::as_raw(self), pmsg, riid, pinterface).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDestCtxEx)(::windows::core::Vtable::as_raw(self), pmsg, pdwdestcontext, ::core::mem::transmute(ppvdestcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetState)(::windows::core::Vtable::as_raw(self), pmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterAsync<P0>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAsyncManager>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterAsync)(::windows::core::Vtable::as_raw(self), pmsg, pasyncmgr.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IRpcChannelBuffer3, ::windows::core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl ::core::clone::Clone for IRpcChannelBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer3 {}
impl ::core::fmt::Debug for IRpcChannelBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT,
    pub RegisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcHelper(::windows::core::IUnknown);
impl IRpcHelper {
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDCOMProtocolVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut ::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIIDFromOBJREF)(::windows::core::Vtable::as_raw(self), pobjref, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRpcHelper, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRpcHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcHelper {}
impl ::core::fmt::Debug for IRpcHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcHelper {
    type Vtable = IRpcHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000149_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetIIDFromOBJREF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcOptions(::windows::core::IUnknown);
impl IRpcOptions {
    pub unsafe fn Set<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Set)(::windows::core::Vtable::as_raw(self), pprx.into().abi(), dwproperty, dwvalue).ok()
    }
    pub unsafe fn Query<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Query)(::windows::core::Vtable::as_raw(self), pprx.into().abi(), dwproperty, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRpcOptions, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRpcOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcOptions {}
impl ::core::fmt::Debug for IRpcOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcOptions {
    type Vtable = IRpcOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000144_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcProxyBuffer(::windows::core::IUnknown);
impl IRpcProxyBuffer {
    pub unsafe fn Connect<P0>(&self, prpcchannelbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRpcChannelBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).Connect)(::windows::core::Vtable::as_raw(self), prpcchannelbuffer.into().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        (::windows::core::Vtable::vtable(self).Disconnect)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IRpcProxyBuffer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRpcProxyBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcProxyBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcProxyBuffer {}
impl ::core::fmt::Debug for IRpcProxyBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcProxyBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcProxyBuffer {
    type Vtable = IRpcProxyBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcProxyBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56a34_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prpcchannelbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcStubBuffer(::windows::core::IUnknown);
impl IRpcStubBuffer {
    pub unsafe fn Connect<P0>(&self, punkserver: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Connect)(::windows::core::Vtable::as_raw(self), punkserver.into().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        (::windows::core::Vtable::vtable(self).Disconnect)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Invoke<P0>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRpcChannelBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).Invoke)(::windows::core::Vtable::as_raw(self), _prpcmsg, _prpcchannelbuffer.into().abi()).ok()
    }
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
        (::windows::core::Vtable::vtable(self).IsIIDSupported)(::windows::core::Vtable::as_raw(self), riid)
    }
    pub unsafe fn CountRefs(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).CountRefs)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DebugServerQueryInterface)(::windows::core::Vtable::as_raw(self), ppv).ok()
    }
    pub unsafe fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).DebugServerRelease)(::windows::core::Vtable::as_raw(self), pv)
    }
}
::windows::core::interface_hierarchy!(IRpcStubBuffer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRpcStubBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcStubBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcStubBuffer {}
impl ::core::fmt::Debug for IRpcStubBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcStubBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcStubBuffer {
    type Vtable = IRpcStubBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcStubBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56afc_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsIIDSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer>,
    pub CountRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub DebugServerQueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DebugServerRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcSyntaxNegotiate(::windows::core::IUnknown);
impl IRpcSyntaxNegotiate {
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).NegotiateSyntax)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
}
::windows::core::interface_hierarchy!(IRpcSyntaxNegotiate, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRpcSyntaxNegotiate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcSyntaxNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcSyntaxNegotiate {}
impl ::core::fmt::Debug for IRpcSyntaxNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcSyntaxNegotiate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiate_Vtbl;
}
unsafe impl ::windows::core::Interface for IRpcSyntaxNegotiate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58a08519_24c8_4935_b482_3fd823333a4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NegotiateSyntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRunnableObject(::windows::core::IUnknown);
impl IRunnableObject {
    pub unsafe fn GetRunningClass(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRunningClass)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Run<P0>(&self, pbc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).Run)(::windows::core::Vtable::as_raw(self), pbc.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsRunning)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRunning<P0, P1>(&self, flock: P0, flastunlockcloses: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).LockRunning)(::windows::core::Vtable::as_raw(self), flock.into(), flastunlockcloses.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainedObject<P0>(&self, fcontained: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetContainedObject)(::windows::core::Vtable::as_raw(self), fcontained.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IRunnableObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRunnableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunnableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunnableObject {}
impl ::core::fmt::Debug for IRunnableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunnableObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRunnableObject {
    type Vtable = IRunnableObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IRunnableObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000126_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRunningClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRunning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LockRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockRunning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainedObject: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRunningObjectTable(::windows::core::IUnknown);
impl IRunningObjectTable {
    pub unsafe fn Register<P0, P1>(&self, grfflags: ROT_FLAGS, punkobject: P0, pmkobjectname: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Register)(::windows::core::Vtable::as_raw(self), grfflags, punkobject.into().abi(), pmkobjectname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Revoke)(::windows::core::Vtable::as_raw(self), dwregister).ok()
    }
    pub unsafe fn IsRunning<P0>(&self, pmkobjectname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).IsRunning)(::windows::core::Vtable::as_raw(self), pmkobjectname.into().abi()).ok()
    }
    pub unsafe fn GetObject<P0>(&self, pmkobjectname: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObject)(::windows::core::Vtable::as_raw(self), pmkobjectname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).NoteChangeTime)(::windows::core::Vtable::as_raw(self), dwregister, pfiletime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<P0>(&self, pmkobjectname: P0) -> ::windows::core::Result<super::super::Foundation::FILETIME>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMoniker>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTimeOfLastChange)(::windows::core::Vtable::as_raw(self), pmkobjectname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumRunning(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumRunning)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRunningObjectTable, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRunningObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunningObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunningObjectTable {}
impl ::core::fmt::Debug for IRunningObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRunningObjectTable {
    type Vtable = IRunningObjectTable_Vtbl;
}
unsafe impl ::windows::core::Interface for IRunningObjectTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000010_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: ROT_FLAGS, punkobject: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, pdwregister: *mut u32) -> ::windows::core::HRESULT,
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows::core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NoteChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NoteChangeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimeOfLastChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimeOfLastChange: usize,
    pub EnumRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenummoniker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISequentialStream(::windows::core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
}
::windows::core::interface_hierarchy!(ISequentialStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISequentialStream {}
impl ::core::fmt::Debug for ISequentialStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISequentialStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISequentialStream {
    type Vtable = ISequentialStream_Vtbl;
}
unsafe impl ::windows::core::Interface for ISequentialStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IServerSecurity(::windows::core::IUnknown);
impl IServerSecurity {
    pub unsafe fn QueryBlanket(&self, pauthnsvc: ::core::option::Option<*mut u32>, pauthzsvc: ::core::option::Option<*mut u32>, pserverprincname: *mut *mut u16, pauthnlevel: ::core::option::Option<*mut u32>, pimplevel: ::core::option::Option<*mut u32>, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryBlanket)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pauthnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pauthzsvc.unwrap_or(::std::ptr::null_mut())), pserverprincname, ::core::mem::transmute(pauthnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pimplevel.unwrap_or(::std::ptr::null_mut())), pprivs, ::core::mem::transmute(pcapabilities.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ImpersonateClient(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ImpersonateClient)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RevertToSelf(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RevertToSelf)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsImpersonating)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IServerSecurity, ::windows::core::IUnknown);
impl ::core::clone::Clone for IServerSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServerSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServerSecurity {}
impl ::core::fmt::Debug for IServerSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IServerSecurity {
    type Vtable = IServerSecurity_Vtbl;
}
unsafe impl ::windows::core::Interface for IServerSecurity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub ImpersonateClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsImpersonating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsImpersonating: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IServiceProvider(::windows::core::IUnknown);
impl IServiceProvider {
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryService)(::windows::core::Vtable::as_raw(self), guidservice, riid, ppvobject).ok()
    }
}
::windows::core::interface_hierarchy!(IServiceProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceProvider {}
impl ::core::fmt::Debug for IServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IServiceProvider {
    type Vtable = IServiceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IServiceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5140c1_7436_11ce_8034_00aa006009fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IStdMarshalInfo(::windows::core::IUnknown);
impl IStdMarshalInfo {
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*mut ::core::ffi::c_void>, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetClassForHandler)(::windows::core::Vtable::as_raw(self), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null_mut())), pclsid).ok()
    }
}
::windows::core::interface_hierarchy!(IStdMarshalInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IStdMarshalInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStdMarshalInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStdMarshalInfo {}
impl ::core::fmt::Debug for IStdMarshalInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStdMarshalInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IStdMarshalInfo {
    type Vtable = IStdMarshalInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStdMarshalInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000018_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClassForHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IStream(::windows::core::IUnknown);
impl IStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSize)(::windows::core::Vtable::as_raw(self), libnewsize).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IStream>>,
    {
        (::windows::core::Vtable::vtable(self).CopyTo)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: STGC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Commit)(::windows::core::Vtable::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Revert)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnlockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Stat)(::windows::core::Vtable::as_raw(self), pstatstg, grfstatflag).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IStream, ::windows::core::IUnknown, ISequentialStream);
impl ::core::clone::Clone for IStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStream {}
impl ::core::fmt::Debug for IStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IStream {
    type Vtable = IStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: LOCKTYPE) -> ::windows::core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: STATFLAG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISupportErrorInfo(::windows::core::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InterfaceSupportsErrorInfo)(::windows::core::Vtable::as_raw(self), riid).ok()
    }
}
::windows::core::interface_hierarchy!(ISupportErrorInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISupportErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportErrorInfo {}
impl ::core::fmt::Debug for ISupportErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISupportErrorInfo {
    type Vtable = ISupportErrorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ISupportErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0b3d60_548f_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISurrogate(::windows::core::IUnknown);
impl ISurrogate {
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadDllServer)(::windows::core::Vtable::as_raw(self), clsid).ok()
    }
    pub unsafe fn FreeSurrogate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FreeSurrogate)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ISurrogate, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISurrogate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogate {}
impl ::core::fmt::Debug for ISurrogate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISurrogate {
    type Vtable = ISurrogate_Vtbl;
}
unsafe impl ::windows::core::Interface for ISurrogate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000022_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadDllServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub FreeSurrogate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISurrogateService(::windows::core::IUnknown);
impl ISurrogateService {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<P0>(&self, rguidprocessid: *const ::windows::core::GUID, pprocesslock: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IProcessLock>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Init)(::windows::core::Vtable::as_raw(self), rguidprocessid, pprocesslock.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ApplicationLaunch)(::windows::core::Vtable::as_raw(self), rguidapplid, apptype).ok()
    }
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ApplicationFree)(::windows::core::Vtable::as_raw(self), rguidapplid).ok()
    }
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CatalogRefresh)(::windows::core::Vtable::as_raw(self), ulreserved).ok()
    }
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessShutdown)(::windows::core::Vtable::as_raw(self), shutdowntype).ok()
    }
}
::windows::core::interface_hierarchy!(ISurrogateService, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISurrogateService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogateService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogateService {}
impl ::core::fmt::Debug for ISurrogateService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogateService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISurrogateService {
    type Vtable = ISurrogateService_Vtbl;
}
unsafe impl ::windows::core::Interface for ISurrogateService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d4_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows::core::GUID, pprocesslock: *mut ::core::ffi::c_void, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Init: usize,
    pub ApplicationLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT,
    pub ApplicationFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CatalogRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows::core::HRESULT,
    pub ProcessShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronize(::windows::core::IUnknown);
impl ISynchronize {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Wait)(::windows::core::Vtable::as_raw(self), dwflags, dwmilliseconds).ok()
    }
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Signal)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ISynchronize, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISynchronize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronize {}
impl ::core::fmt::Debug for ISynchronize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISynchronize {
    type Vtable = ISynchronize_Vtbl;
}
unsafe impl ::windows::core::Interface for ISynchronize {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000030_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeContainer(::windows::core::IUnknown);
impl ISynchronizeContainer {
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISynchronize>>,
    {
        (::windows::core::Vtable::vtable(self).AddSynchronize)(::windows::core::Vtable::as_raw(self), psync.into().abi()).ok()
    }
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitMultiple)(::windows::core::Vtable::as_raw(self), dwflags, dwtimeout, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISynchronizeContainer, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISynchronizeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeContainer {}
impl ::core::fmt::Debug for ISynchronizeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISynchronizeContainer {
    type Vtable = ISynchronizeContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for ISynchronizeContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000033_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddSynchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeEvent(::windows::core::IUnknown);
impl ISynchronizeEvent {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventHandle)(::windows::core::Vtable::as_raw(self), ph).ok()
    }
}
::windows::core::interface_hierarchy!(ISynchronizeEvent, ::windows::core::IUnknown, ISynchronizeHandle);
impl ::core::clone::Clone for ISynchronizeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeEvent {}
impl ::core::fmt::Debug for ISynchronizeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISynchronizeEvent {
    type Vtable = ISynchronizeEvent_Vtbl;
}
unsafe impl ::windows::core::Interface for ISynchronizeEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_Vtbl {
    pub base__: ISynchronizeHandle_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeHandle(::windows::core::IUnknown);
impl ISynchronizeHandle {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISynchronizeHandle, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISynchronizeHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeHandle {}
impl ::core::fmt::Debug for ISynchronizeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISynchronizeHandle {
    type Vtable = ISynchronizeHandle_Vtbl;
}
unsafe impl ::windows::core::Interface for ISynchronizeHandle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000031_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeMutex(::windows::core::IUnknown);
impl ISynchronizeMutex {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Wait)(::windows::core::Vtable::as_raw(self), dwflags, dwmilliseconds).ok()
    }
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Signal)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMutex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseMutex)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ISynchronizeMutex, ::windows::core::IUnknown, ISynchronize);
impl ::core::clone::Clone for ISynchronizeMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeMutex {}
impl ::core::fmt::Debug for ISynchronizeMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeMutex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISynchronizeMutex {
    type Vtable = ISynchronizeMutex_Vtbl;
}
unsafe impl ::windows::core::Interface for ISynchronizeMutex {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000025_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_Vtbl {
    pub base__: ISynchronize_Vtbl,
    pub ReleaseMutex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITimeAndNoticeControl(::windows::core::IUnknown);
impl ITimeAndNoticeControl {
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SuppressChanges)(::windows::core::Vtable::as_raw(self), res1, res2).ok()
    }
}
::windows::core::interface_hierarchy!(ITimeAndNoticeControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITimeAndNoticeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITimeAndNoticeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimeAndNoticeControl {}
impl ::core::fmt::Debug for ITimeAndNoticeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeAndNoticeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ITimeAndNoticeControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SuppressChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeComp(::windows::core::IUnknown);
impl ITypeComp {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Bind<P0>(&self, szname: P0, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Bind)(::windows::core::Vtable::as_raw(self), szname.into().abi(), lhashval, wflags, ::core::mem::transmute(pptinfo), pdesckind, pbindptr).ok()
    }
    pub unsafe fn BindType<P0>(&self, szname: P0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).BindType)(::windows::core::Vtable::as_raw(self), szname.into().abi(), lhashval, ::core::mem::transmute(pptinfo), ::core::mem::transmute(pptcomp)).ok()
    }
}
::windows::core::interface_hierarchy!(ITypeComp, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeComp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeComp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeComp {}
impl ::core::fmt::Debug for ITypeComp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeComp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeComp {
    type Vtable = ITypeComp_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeComp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut *mut ::core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Bind: usize,
    pub BindType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, lhashval: u32, pptinfo: *mut *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeInfo(::windows::core::IUnknown);
impl ITypeInfo {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeAttr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeComp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFuncDesc)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVarDesc)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut ::windows::core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetNames)(::windows::core::Vtable::as_raw(self), memid, ::core::mem::transmute(rgbstrnames), cmaxnames, pcnames).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRefTypeOfImplType)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<IMPLTYPEFLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImplTypeFlags)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const ::windows::core::PCWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetIDsOfNames)(::windows::core::Vtable::as_raw(self), rgsznames, cnames, pmemid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Invoke)(::windows::core::Vtable::as_raw(self), pvinstance, memid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr).ok()
    }
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrdocstring: ::core::option::Option<*mut ::windows::core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDocumentation)(::windows::core::Vtable::as_raw(self), memid, ::core::mem::transmute(pbstrname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrdocstring.unwrap_or(::std::ptr::null_mut())), pdwhelpcontext, ::core::mem::transmute(pbstrhelpfile.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrname: ::core::option::Option<*mut ::windows::core::BSTR>, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDllEntry)(::windows::core::Vtable::as_raw(self), memid, invkind, ::core::mem::transmute(pbstrdllname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrname.unwrap_or(::std::ptr::null_mut())), pwordinal).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRefTypeInfo)(::windows::core::Vtable::as_raw(self), hreftype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddressOfMember)(::windows::core::Vtable::as_raw(self), memid, invkind, ppv).ok()
    }
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstance)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMops)(::windows::core::Vtable::as_raw(self), memid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetContainingTypeLib)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pptlib), pindex).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows::core::Vtable::vtable(self).ReleaseTypeAttr)(::windows::core::Vtable::as_raw(self), ptypeattr)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows::core::Vtable::vtable(self).ReleaseFuncDesc)(::windows::core::Vtable::as_raw(self), pfuncdesc)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows::core::Vtable::vtable(self).ReleaseVarDesc)(::windows::core::Vtable::as_raw(self), pvardesc)
    }
}
::windows::core::interface_hierarchy!(ITypeInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo {}
impl ::core::fmt::Debug for ITypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeInfo {
    type Vtable = ITypeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetTypeAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetTypeAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetFuncDesc: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetVarDesc: usize,
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut *mut ::core::ffi::c_void, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT,
    pub GetRefTypeOfImplType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT,
    pub GetImplTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut IMPLTYPEFLAGS) -> ::windows::core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgsznames: *const ::windows::core::PCWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Invoke: usize,
    pub GetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut *mut ::core::ffi::c_void, pbstrdocstring: *mut *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDllEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void, pwordinal: *mut u16) -> ::windows::core::HRESULT,
    pub GetRefTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContainingTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlib: *mut *mut ::core::ffi::c_void, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub ReleaseTypeAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR),
    #[cfg(not(feature = "Win32_System_Ole"))]
    ReleaseTypeAttr: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub ReleaseFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    ReleaseFuncDesc: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub ReleaseVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    ReleaseVarDesc: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeInfo2(::windows::core::IUnknown);
impl ITypeInfo2 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeAttr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeComp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFuncDesc)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVarDesc)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut ::windows::core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNames)(::windows::core::Vtable::as_raw(self), memid, ::core::mem::transmute(rgbstrnames), cmaxnames, pcnames).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRefTypeOfImplType)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<IMPLTYPEFLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImplTypeFlags)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const ::windows::core::PCWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsOfNames)(::windows::core::Vtable::as_raw(self), rgsznames, cnames, pmemid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Invoke)(::windows::core::Vtable::as_raw(self), pvinstance, memid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr).ok()
    }
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrdocstring: ::core::option::Option<*mut ::windows::core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDocumentation)(::windows::core::Vtable::as_raw(self), memid, ::core::mem::transmute(pbstrname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrdocstring.unwrap_or(::std::ptr::null_mut())), pdwhelpcontext, ::core::mem::transmute(pbstrhelpfile.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrname: ::core::option::Option<*mut ::windows::core::BSTR>, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDllEntry)(::windows::core::Vtable::as_raw(self), memid, invkind, ::core::mem::transmute(pbstrdllname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrname.unwrap_or(::std::ptr::null_mut())), pwordinal).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRefTypeInfo)(::windows::core::Vtable::as_raw(self), hreftype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddressOfMember)(::windows::core::Vtable::as_raw(self), memid, invkind, ppv).ok()
    }
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInstance)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMops)(::windows::core::Vtable::as_raw(self), memid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetContainingTypeLib)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pptlib), pindex).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseTypeAttr)(::windows::core::Vtable::as_raw(self), ptypeattr)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseFuncDesc)(::windows::core::Vtable::as_raw(self), pfuncdesc)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseVarDesc)(::windows::core::Vtable::as_raw(self), pvardesc)
    }
    pub unsafe fn GetTypeKind(&self) -> ::windows::core::Result<TYPEKIND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFuncIndexOfMemId)(::windows::core::Vtable::as_raw(self), memid, invkind, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVarIndexOfMemId)(::windows::core::Vtable::as_raw(self), memid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustData)(::windows::core::Vtable::as_raw(self), guid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFuncCustData)(::windows::core::Vtable::as_raw(self), index, guid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParamCustData)(::windows::core::Vtable::as_raw(self), indexfunc, indexparam, guid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVarCustData)(::windows::core::Vtable::as_raw(self), index, guid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImplTypeCustData)(::windows::core::Vtable::as_raw(self), index, guid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: ::core::option::Option<*mut ::windows::core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDocumentation2)(::windows::core::Vtable::as_raw(self), memid, lcid, ::core::mem::transmute(pbstrhelpstring.unwrap_or(::std::ptr::null_mut())), pdwhelpstringcontext, ::core::mem::transmute(pbstrhelpstringdll.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllCustData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllFuncCustData)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllParamCustData)(::windows::core::Vtable::as_raw(self), indexfunc, indexparam, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllVarCustData)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllImplTypeCustData)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITypeInfo2, ::windows::core::IUnknown, ITypeInfo);
impl ::core::clone::Clone for ITypeInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo2 {}
impl ::core::fmt::Debug for ITypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeInfo2 {
    type Vtable = ITypeInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020412_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_Vtbl {
    pub base__: ITypeInfo_Vtbl,
    pub GetTypeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetVarIndexOfMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetFuncCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetParamCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetVarCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetImplTypeCustData: usize,
    pub GetDocumentation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut *mut ::core::ffi::c_void, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllFuncCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllParamCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllVarCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllImplTypeCustData: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLib(::windows::core::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetTypeInfoCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeInfo)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeInfoType)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeInfoOfGuid)(::windows::core::Vtable::as_raw(self), guid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLibAttr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeComp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrdocstring: ::core::option::Option<*mut ::windows::core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDocumentation)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(pbstrname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrdocstring.unwrap_or(::std::ptr::null_mut())), pdwhelpcontext, ::core::mem::transmute(pbstrhelpfile.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(sznamebuf), lhashval, pfname).ok()
    }
    pub unsafe fn FindName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FindName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(sznamebuf), lhashval, ::core::mem::transmute(pptinfo), rgmemid, pcfound).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows::core::Vtable::vtable(self).ReleaseTLibAttr)(::windows::core::Vtable::as_raw(self), ptlibattr)
    }
}
::windows::core::interface_hierarchy!(ITypeLib, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeLib {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib {}
impl ::core::fmt::Debug for ITypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeLib {
    type Vtable = ITypeLib_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeLib {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTypeInfoType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLibAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT,
    pub GetTypeComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut *mut ::core::ffi::c_void, pbstrdocstring: *mut *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsName: usize,
    pub FindName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut *mut ::core::ffi::c_void, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT,
    pub ReleaseTLibAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLib2(::windows::core::IUnknown);
impl ITypeLib2 {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetTypeInfoCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeInfo)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeInfoType)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeInfoOfGuid)(::windows::core::Vtable::as_raw(self), guid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLibAttr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeComp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrdocstring: ::core::option::Option<*mut ::windows::core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDocumentation)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(pbstrname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrdocstring.unwrap_or(::std::ptr::null_mut())), pdwhelpcontext, ::core::mem::transmute(pbstrhelpfile.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(sznamebuf), lhashval, pfname).ok()
    }
    pub unsafe fn FindName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FindName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(sznamebuf), lhashval, ::core::mem::transmute(pptinfo), rgmemid, pcfound).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseTLibAttr)(::windows::core::Vtable::as_raw(self), ptlibattr)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustData)(::windows::core::Vtable::as_raw(self), guid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetLibStatistics)(::windows::core::Vtable::as_raw(self), pcuniquenames, pcchuniquenames).ok()
    }
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: ::core::option::Option<*mut ::windows::core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDocumentation2)(::windows::core::Vtable::as_raw(self), index, lcid, ::core::mem::transmute(pbstrhelpstring.unwrap_or(::std::ptr::null_mut())), pdwhelpstringcontext, ::core::mem::transmute(pbstrhelpstringdll.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllCustData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITypeLib2, ::windows::core::IUnknown, ITypeLib);
impl ::core::clone::Clone for ITypeLib2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib2 {}
impl ::core::fmt::Debug for ITypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeLib2 {
    type Vtable = ITypeLib2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeLib2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020411_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_Vtbl {
    pub base__: ITypeLib_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetCustData: usize,
    pub GetLibStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT,
    pub GetDocumentation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut *mut ::core::ffi::c_void, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllCustData: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLibRegistration(::windows::core::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGuid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLcid(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLcid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWin32Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWin32Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWin64Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWin64Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHelpDir(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHelpDir)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITypeLibRegistration, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeLibRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistration {}
impl ::core::fmt::Debug for ITypeLibRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeLibRegistration {
    type Vtable = ITypeLibRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeLibRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a3e735_02df_4a12_98eb_043ad3600af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLcid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub GetWin32Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin32path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetWin64Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin64path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetHelpDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phelpdir: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLibRegistrationReader(::windows::core::IUnknown);
impl ITypeLibRegistrationReader {
    pub unsafe fn EnumTypeLibRegistrations(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumTypeLibRegistrations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITypeLibRegistrationReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeLibRegistrationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistrationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistrationReader {}
impl ::core::fmt::Debug for ITypeLibRegistrationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistrationReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeLibRegistrationReader {
    type Vtable = ITypeLibRegistrationReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeLibRegistrationReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumTypeLibRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IUri(::windows::core::IUnknown);
impl IUri {
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::windows::core::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyBSTR)(::windows::core::Vtable::as_raw(self), uriprop, ::core::mem::transmute(pbstrproperty), dwflags).ok()
    }
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyLength)(::windows::core::Vtable::as_raw(self), uriprop, pcchproperty, dwflags).ok()
    }
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyDWORD)(::windows::core::Vtable::as_raw(self), uriprop, pdwproperty, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasProperty)(::windows::core::Vtable::as_raw(self), uriprop, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAbsoluteUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAuthority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDisplayUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDomain)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetExtension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFragment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHost)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPathAndQuery)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetQuery)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRawUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSchemeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHostType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetScheme)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetZone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsEqual)(::windows::core::Vtable::as_raw(self), puri.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUri, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUri {}
impl ::core::fmt::Debug for IUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUri").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUri {
    type Vtable = IUri_Vtbl;
}
unsafe impl ::windows::core::Interface for IUri {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa39ee748_6a27_4817_a6f2_13914bef5890);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyBSTR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetPropertyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetPropertyDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasProperty: usize,
    pub GetAbsoluteUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAuthority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthority: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplayUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextension: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfragment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhost: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpassword: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPathAndQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRawUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrschemename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows::core::HRESULT,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows::core::HRESULT,
    pub GetZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqual: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IUriBuilder(::windows::core::IUnknown);
impl IUriBuilder {
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateUriSimple)(::windows::core::Vtable::as_raw(self), dwallowencodingpropertymask, dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateUri)(::windows::core::Vtable::as_raw(self), dwcreateflags, dwallowencodingpropertymask, dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateUriWithFlags)(::windows::core::Vtable::as_raw(self), dwcreateflags, dwuribuilderflags, dwallowencodingpropertymask, dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIUri(&self) -> ::windows::core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIUri<P0>(&self, piuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetIUri)(::windows::core::Vtable::as_raw(self), piuri.into().abi()).ok()
    }
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFragment)(::windows::core::Vtable::as_raw(self), pcchfragment, ppwzfragment).ok()
    }
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetHost)(::windows::core::Vtable::as_raw(self), pcchhost, ppwzhost).ok()
    }
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPassword)(::windows::core::Vtable::as_raw(self), pcchpassword, ppwzpassword).ok()
    }
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPath)(::windows::core::Vtable::as_raw(self), pcchpath, ppwzpath).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPort)(::windows::core::Vtable::as_raw(self), pfhasport, pdwport).ok()
    }
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetQuery)(::windows::core::Vtable::as_raw(self), pcchquery, ppwzquery).ok()
    }
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSchemeName)(::windows::core::Vtable::as_raw(self), pcchschemename, ppwzschemename).ok()
    }
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetUserName)(::windows::core::Vtable::as_raw(self), pcchusername, ppwzusername).ok()
    }
    pub unsafe fn SetFragment<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetFragment)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    pub unsafe fn SetHost<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHost)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetPassword)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    pub unsafe fn SetPath<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetPath)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPort<P0>(&self, fhasport: P0, dwnewvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetPort)(::windows::core::Vtable::as_raw(self), fhasport.into(), dwnewvalue).ok()
    }
    pub unsafe fn SetQuery<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetQuery)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    pub unsafe fn SetSchemeName<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSchemeName)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    pub unsafe fn SetUserName<P0>(&self, pwznewvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetUserName)(::windows::core::Vtable::as_raw(self), pwznewvalue.into().abi()).ok()
    }
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveProperties)(::windows::core::Vtable::as_raw(self), dwpropertymask).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasBeenModified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasBeenModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUriBuilder, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUriBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUriBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilder {}
impl ::core::fmt::Debug for IUriBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUriBuilder {
    type Vtable = IUriBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for IUriBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4221b2e1_8955_46c0_bd5b_de9897565de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateUriSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUriWithFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piuri: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPort: usize,
    pub GetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPort: usize,
    pub SetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasBeenModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasBeenModified: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IUrlMon(::windows::core::IUnknown);
impl IUrlMon {
    pub unsafe fn AsyncGetClassBits<P0, P1, P2, P3>(&self, rclsid: *const ::windows::core::GUID, psztype: P0, pszext: P1, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: P2, pbc: P3, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncGetClassBits)(::windows::core::Vtable::as_raw(self), rclsid, psztype.into().abi(), pszext.into().abi(), dwfileversionms, dwfileversionls, pszcodebase.into().abi(), pbc.into().abi(), dwclasscontext, riid, flags).ok()
    }
}
::windows::core::interface_hierarchy!(IUrlMon, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUrlMon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUrlMon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlMon {}
impl ::core::fmt::Debug for IUrlMon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlMon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUrlMon {
    type Vtable = IUrlMon_Vtbl;
}
unsafe impl ::windows::core::Interface for IUrlMon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000026_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AsyncGetClassBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, psztype: ::windows::core::PCWSTR, pszext: ::windows::core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IWaitMultiple(::windows::core::IUnknown);
impl IWaitMultiple {
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitMultiple)(::windows::core::Vtable::as_raw(self), timeout, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISynchronize>>,
    {
        (::windows::core::Vtable::vtable(self).AddSynchronize)(::windows::core::Vtable::as_raw(self), psync.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWaitMultiple, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWaitMultiple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWaitMultiple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWaitMultiple {}
impl ::core::fmt::Debug for IWaitMultiple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWaitMultiple").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWaitMultiple {
    type Vtable = IWaitMultiple_Vtbl;
}
unsafe impl ::windows::core::Interface for IWaitMultiple {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub WaitMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddSynchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_TOEND: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADVANCED_FEATURE_FLAGS(pub u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_AUTO: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_STATIC: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_EMBEDDED: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_FIXEDSIZE: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(16u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_RECORD: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(32u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_HAVEIID: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(64u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_HAVEVARTYPE: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(128u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_BSTR: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(256u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_UNKNOWN: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(512u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_DISPATCH: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(1024u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_VARIANT: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(2048u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FADF_RESERVED: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(61448u16);
impl ::core::marker::Copy for ADVANCED_FEATURE_FLAGS {}
impl ::core::clone::Clone for ADVANCED_FEATURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVANCED_FEATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ADVANCED_FEATURE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ADVANCED_FEATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVANCED_FEATURE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ADVANCED_FEATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ADVANCED_FEATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADVF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_NODATA: ADVF = ADVF(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
impl ::core::marker::Copy for ADVF {}
impl ::core::clone::Clone for ADVF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ADVF {
    type Abi = Self;
}
impl ::core::fmt::Debug for ADVF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
impl ::core::marker::Copy for APTTYPE {}
impl ::core::clone::Clone for APTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APTTYPEQUALIFIER(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = APTTYPEQUALIFIER(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = APTTYPEQUALIFIER(7i32);
impl ::core::marker::Copy for APTTYPEQUALIFIER {}
impl ::core::clone::Clone for APTTYPEQUALIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPEQUALIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APTTYPEQUALIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for APTTYPEQUALIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPEQUALIFIER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
impl ::core::marker::Copy for ApplicationType {}
impl ::core::clone::Clone for ApplicationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDINFOF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
impl ::core::marker::Copy for BINDINFOF {}
impl ::core::clone::Clone for BINDINFOF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDINFOF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BINDINFOF {
    type Abi = Self;
}
impl ::core::fmt::Debug for BINDINFOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDINFOF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BIND_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
impl ::core::marker::Copy for BIND_FLAGS {}
impl ::core::clone::Clone for BIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BIND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIND_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLCONV(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
impl ::core::marker::Copy for CALLCONV {}
impl ::core::clone::Clone for CALLCONV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLCONV {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CALLCONV {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLCONV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLCONV").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
impl ::core::marker::Copy for CALLTYPE {}
impl ::core::clone::Clone for CALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CALLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLSCTX(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
impl ::core::marker::Copy for CLSCTX {}
impl ::core::clone::Clone for CLSCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLSCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CLSCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLSCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLSCTX").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COINIT(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
impl ::core::marker::Copy for COINIT {}
impl ::core::clone::Clone for COINIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COINIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for COINIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COINITBASE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::core::marker::Copy for COINITBASE {}
impl ::core::clone::Clone for COINITBASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINITBASE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COINITBASE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COINITBASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINITBASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMSD(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
impl ::core::marker::Copy for COMSD {}
impl ::core::clone::Clone for COMSD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMSD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMSD {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMSD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMSD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COWAIT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
impl ::core::marker::Copy for COWAIT_FLAGS {}
impl ::core::clone::Clone for COWAIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COWAIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COWAIT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COWAIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COWAIT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483647i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483646i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483645i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483644i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483643i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483642i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483641i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483640i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483639i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483638i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483637i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483636i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483635i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483634i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483633i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483632i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483631i32);
impl ::core::marker::Copy for CO_MARSHALING_CONTEXT_ATTRIBUTES {}
impl ::core::clone::Clone for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MARSHALING_CONTEXT_ATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CWMO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
impl ::core::marker::Copy for CWMO_FLAGS {}
impl ::core::clone::Clone for CWMO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CWMO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CWMO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CWMO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CWMO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DATADIR(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
impl ::core::marker::Copy for DATADIR {}
impl ::core::clone::Clone for DATADIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DATADIR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DATADIR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DATADIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATADIR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOM_CALL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
impl ::core::marker::Copy for DCOM_CALL_STATE {}
impl ::core::clone::Clone for DCOM_CALL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOM_CALL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOM_CALL_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOM_CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOM_CALL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DESCKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
impl ::core::marker::Copy for DESCKIND {}
impl ::core::clone::Clone for DESCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DESCKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DESCKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DESCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESCKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPATCH_FLAGS(pub u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DISPATCH_METHOD: DISPATCH_FLAGS = DISPATCH_FLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DISPATCH_PROPERTYGET: DISPATCH_FLAGS = DISPATCH_FLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DISPATCH_PROPERTYPUT: DISPATCH_FLAGS = DISPATCH_FLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DISPATCH_PROPERTYPUTREF: DISPATCH_FLAGS = DISPATCH_FLAGS(8u16);
impl ::core::marker::Copy for DISPATCH_FLAGS {}
impl ::core::clone::Clone for DISPATCH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISPATCH_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCH_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DISPATCH_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DISPATCH_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DISPATCH_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DISPATCH_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DISPATCH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DVASPECT(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_OPAQUE: DVASPECT = DVASPECT(16u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_TRANSPARENT: DVASPECT = DVASPECT(32u32);
impl ::core::marker::Copy for DVASPECT {}
impl ::core::clone::Clone for DVASPECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DVASPECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DVASPECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DVASPECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVASPECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(128i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(256i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2048i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(512i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1024i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4096i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8192i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16384i32);
impl ::core::marker::Copy for EOLE_AUTHENTICATION_CAPABILITIES {}
impl ::core::clone::Clone for EOLE_AUTHENTICATION_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOLE_AUTHENTICATION_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EOLE_AUTHENTICATION_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for EOLE_AUTHENTICATION_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOLE_AUTHENTICATION_CAPABILITIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXTCONN(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
impl ::core::marker::Copy for EXTCONN {}
impl ::core::clone::Clone for EXTCONN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXTCONN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EXTCONN {
    type Abi = Self;
}
impl ::core::fmt::Debug for EXTCONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXTCONN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FUNCFLAGS(pub u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = FUNCFLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = FUNCFLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = FUNCFLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = FUNCFLAGS(8u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = FUNCFLAGS(16u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = FUNCFLAGS(32u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = FUNCFLAGS(64u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = FUNCFLAGS(128u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = FUNCFLAGS(256u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = FUNCFLAGS(512u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = FUNCFLAGS(1024u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = FUNCFLAGS(2048u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = FUNCFLAGS(4096u16);
impl ::core::marker::Copy for FUNCFLAGS {}
impl ::core::clone::Clone for FUNCFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FUNCFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FUNCFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FUNCFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FUNCFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FUNCKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
impl ::core::marker::Copy for FUNCKIND {}
impl ::core::clone::Clone for FUNCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FUNCKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FUNCKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for FUNCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FUNCKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_EH_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_EH_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_EH_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_EH_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_EH_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_EH_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(8i32);
impl ::core::marker::Copy for GLOBALOPT_PROPERTIES {}
impl ::core::clone::Clone for GLOBALOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_PROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1024i32);
impl ::core::marker::Copy for GLOBALOPT_RO_FLAGS {}
impl ::core::clone::Clone for GLOBALOPT_RO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_RO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_RO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(1i32);
impl ::core::marker::Copy for GLOBALOPT_RPCTP_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_RPCTP_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RPCTP_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_RPCTP_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_RPCTP_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RPCTP_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_UNMARSHALING_POLICY_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_UNMARSHALING_POLICY_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IDLFLAGS(pub u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IDLFLAG_NONE: IDLFLAGS = IDLFLAGS(0u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IDLFLAG_FIN: IDLFLAGS = IDLFLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IDLFLAG_FOUT: IDLFLAGS = IDLFLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IDLFLAG_FLCID: IDLFLAGS = IDLFLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IDLFLAG_FRETVAL: IDLFLAGS = IDLFLAGS(8u16);
impl ::core::marker::Copy for IDLFLAGS {}
impl ::core::clone::Clone for IDLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IDLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IDLFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IDLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDLFLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IDLFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IDLFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IDLFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IDLFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IDLFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMPLTYPEFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IMPLTYPEFLAG_FDEFAULT: IMPLTYPEFLAGS = IMPLTYPEFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IMPLTYPEFLAG_FSOURCE: IMPLTYPEFLAGS = IMPLTYPEFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IMPLTYPEFLAG_FRESTRICTED: IMPLTYPEFLAGS = IMPLTYPEFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: IMPLTYPEFLAGS = IMPLTYPEFLAGS(8i32);
impl ::core::marker::Copy for IMPLTYPEFLAGS {}
impl ::core::clone::Clone for IMPLTYPEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMPLTYPEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMPLTYPEFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMPLTYPEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMPLTYPEFLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMPLTYPEFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMPLTYPEFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMPLTYPEFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMPLTYPEFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMPLTYPEFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INVOKEKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
impl ::core::marker::Copy for INVOKEKIND {}
impl ::core::clone::Clone for INVOKEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INVOKEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INVOKEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for INVOKEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INVOKEKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOCKTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const LOCK_WRITE: LOCKTYPE = LOCKTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const LOCK_EXCLUSIVE: LOCKTYPE = LOCKTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const LOCK_ONLYONCE: LOCKTYPE = LOCKTYPE(4i32);
impl ::core::marker::Copy for LOCKTYPE {}
impl ::core::clone::Clone for LOCKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LOCKTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOCKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCKTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEMCTX(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
impl ::core::marker::Copy for MEMCTX {}
impl ::core::clone::Clone for MEMCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEMCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEMCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMCTX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MKRREDUCE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_ONE: MKRREDUCE = MKRREDUCE(196608i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_TOUSER: MKRREDUCE = MKRREDUCE(131072i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = MKRREDUCE(65536i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_ALL: MKRREDUCE = MKRREDUCE(0i32);
impl ::core::marker::Copy for MKRREDUCE {}
impl ::core::clone::Clone for MKRREDUCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKRREDUCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MKRREDUCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MKRREDUCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKRREDUCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MKSYS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_NONE: MKSYS = MKSYS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_GENERICCOMPOSITE: MKSYS = MKSYS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_FILEMONIKER: MKSYS = MKSYS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_ANTIMONIKER: MKSYS = MKSYS(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_ITEMMONIKER: MKSYS = MKSYS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_POINTERMONIKER: MKSYS = MKSYS(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_CLASSMONIKER: MKSYS = MKSYS(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_OBJREFMONIKER: MKSYS = MKSYS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_SESSIONMONIKER: MKSYS = MKSYS(9i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_LUAMONIKER: MKSYS = MKSYS(10i32);
impl ::core::marker::Copy for MKSYS {}
impl ::core::clone::Clone for MKSYS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKSYS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MKSYS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MKSYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKSYS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSHCTX(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
impl ::core::marker::Copy for MSHCTX {}
impl ::core::clone::Clone for MSHCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSHCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSHCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHCTX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSHLFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
impl ::core::marker::Copy for MSHLFLAGS {}
impl ::core::clone::Clone for MSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSHLFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHLFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PENDINGMSG(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
impl ::core::marker::Copy for PENDINGMSG {}
impl ::core::clone::Clone for PENDINGMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGMSG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PENDINGMSG {
    type Abi = Self;
}
impl ::core::fmt::Debug for PENDINGMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGMSG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PENDINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
impl ::core::marker::Copy for PENDINGTYPE {}
impl ::core::clone::Clone for PENDINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PENDINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PENDINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REGCLS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
impl ::core::marker::Copy for REGCLS {}
impl ::core::clone::Clone for REGCLS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGCLS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REGCLS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGCLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGCLS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ROT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: ROT_FLAGS = ROT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ROTFLAGS_ALLOWANYCLIENT: ROT_FLAGS = ROT_FLAGS(2u32);
impl ::core::marker::Copy for ROT_FLAGS {}
impl ::core::clone::Clone for ROT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ROT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ROT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ROT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ROT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ROT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ROT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ROT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ROT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPCOPT_PROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
impl ::core::marker::Copy for RPCOPT_PROPERTIES {}
impl ::core::clone::Clone for RPCOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPCOPT_PROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPCOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
impl ::core::marker::Copy for RPCOPT_SERVER_LOCALITY_VALUES {}
impl ::core::clone::Clone for RPCOPT_SERVER_LOCALITY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_SERVER_LOCALITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPCOPT_SERVER_LOCALITY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPCOPT_SERVER_LOCALITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_SERVER_LOCALITY_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
impl ::core::marker::Copy for RPC_C_AUTHN_LEVEL {}
impl ::core::clone::Clone for RPC_C_AUTHN_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_AUTHN_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_AUTHN_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_AUTHN_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_IMP_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
impl ::core::marker::Copy for RPC_C_IMP_LEVEL {}
impl ::core::clone::Clone for RPC_C_IMP_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_IMP_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_IMP_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_IMP_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_IMP_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVERCALL(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
impl ::core::marker::Copy for SERVERCALL {}
impl ::core::clone::Clone for SERVERCALL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVERCALL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVERCALL {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVERCALL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVERCALL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STATFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STATFLAG_DEFAULT: STATFLAG = STATFLAG(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STATFLAG_NONAME: STATFLAG = STATFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STATFLAG_NOOPEN: STATFLAG = STATFLAG(2i32);
impl ::core::marker::Copy for STATFLAG {}
impl ::core::clone::Clone for STATFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STATFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STATFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for STATFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGC(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGC_DEFAULT: STGC = STGC(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGC_OVERWRITE: STGC = STGC(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGC_ONLYIFCURRENT: STGC = STGC(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = STGC(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGC_CONSOLIDATE: STGC = STGC(8u32);
impl ::core::marker::Copy for STGC {}
impl ::core::clone::Clone for STGC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGC {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGC").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STGC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STGC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STGC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STGC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STGC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGM(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_DIRECT: STGM = STGM(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_TRANSACTED: STGM = STGM(65536u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_SIMPLE: STGM = STGM(134217728u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_READ: STGM = STGM(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_WRITE: STGM = STGM(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_READWRITE: STGM = STGM(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_SHARE_DENY_NONE: STGM = STGM(64u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_SHARE_DENY_READ: STGM = STGM(48u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_SHARE_DENY_WRITE: STGM = STGM(32u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_SHARE_EXCLUSIVE: STGM = STGM(16u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_PRIORITY: STGM = STGM(262144u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_DELETEONRELEASE: STGM = STGM(67108864u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_NOSCRATCH: STGM = STGM(1048576u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_CREATE: STGM = STGM(4096u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_CONVERT: STGM = STGM(131072u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_FAILIFTHERE: STGM = STGM(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_NOSNAPSHOT: STGM = STGM(2097152u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGM_DIRECT_SWMR: STGM = STGM(4194304u32);
impl ::core::marker::Copy for STGM {}
impl ::core::clone::Clone for STGM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGM {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGM").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STGM {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STGM {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STGM {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STGM {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STGM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGTY(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_STREAM: STGTY = STGTY(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::core::marker::Copy for STGTY {}
impl ::core::clone::Clone for STGTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STREAM_SEEK(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::core::marker::Copy for STREAM_SEEK {}
impl ::core::clone::Clone for STREAM_SEEK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STREAM_SEEK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STREAM_SEEK {
    type Abi = Self;
}
impl ::core::fmt::Debug for STREAM_SEEK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_SEEK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
impl ::core::marker::Copy for SYSKIND {}
impl ::core::clone::Clone for SYSKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ShutdownType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
impl ::core::marker::Copy for ShutdownType {}
impl ::core::clone::Clone for ShutdownType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ShutdownType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShutdownType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
impl ::core::marker::Copy for THDTYPE {}
impl ::core::clone::Clone for THDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THDTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for THDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THDTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TYMED(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_FILE: TYMED = TYMED(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_GDI: TYMED = TYMED(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_NULL: TYMED = TYMED(0i32);
impl ::core::marker::Copy for TYMED {}
impl ::core::clone::Clone for TYMED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYMED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYMED {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYMED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYMED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TYPEKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
impl ::core::marker::Copy for TYPEKIND {}
impl ::core::clone::Clone for TYPEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYPEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYPEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYPEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYPEKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TYSPEC(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
impl ::core::marker::Copy for TYSPEC {}
impl ::core::clone::Clone for TYSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYSPEC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYSPEC {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYSPEC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct URI_CREATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(131072u32);
impl ::core::marker::Copy for URI_CREATE_FLAGS {}
impl ::core::clone::Clone for URI_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URI_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for URI_CREATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for URI_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URI_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Uri_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = Uri_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = Uri_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = Uri_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = Uri_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = Uri_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = Uri_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = Uri_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = Uri_PROPERTY(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = Uri_PROPERTY(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = Uri_PROPERTY(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = Uri_PROPERTY(9i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = Uri_PROPERTY(10i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = Uri_PROPERTY(11i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = Uri_PROPERTY(12i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = Uri_PROPERTY(13i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = Uri_PROPERTY(14i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = Uri_PROPERTY(14i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = Uri_PROPERTY(15i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = Uri_PROPERTY(15i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = Uri_PROPERTY(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = Uri_PROPERTY(17i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = Uri_PROPERTY(18i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = Uri_PROPERTY(18i32);
impl ::core::marker::Copy for Uri_PROPERTY {}
impl ::core::clone::Clone for Uri_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Uri_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Uri_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for Uri_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri_PROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARENUM(pub u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_EMPTY: VARENUM = VARENUM(0u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_NULL: VARENUM = VARENUM(1u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_I2: VARENUM = VARENUM(2u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_I4: VARENUM = VARENUM(3u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_R4: VARENUM = VARENUM(4u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_R8: VARENUM = VARENUM(5u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_CY: VARENUM = VARENUM(6u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_DATE: VARENUM = VARENUM(7u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_BSTR: VARENUM = VARENUM(8u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_DISPATCH: VARENUM = VARENUM(9u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_ERROR: VARENUM = VARENUM(10u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_BOOL: VARENUM = VARENUM(11u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_VARIANT: VARENUM = VARENUM(12u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UNKNOWN: VARENUM = VARENUM(13u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_DECIMAL: VARENUM = VARENUM(14u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_I1: VARENUM = VARENUM(16u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UI1: VARENUM = VARENUM(17u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UI2: VARENUM = VARENUM(18u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UI4: VARENUM = VARENUM(19u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_I8: VARENUM = VARENUM(20u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UI8: VARENUM = VARENUM(21u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_INT: VARENUM = VARENUM(22u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UINT: VARENUM = VARENUM(23u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_VOID: VARENUM = VARENUM(24u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_HRESULT: VARENUM = VARENUM(25u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_PTR: VARENUM = VARENUM(26u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_SAFEARRAY: VARENUM = VARENUM(27u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_CARRAY: VARENUM = VARENUM(28u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_USERDEFINED: VARENUM = VARENUM(29u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_LPSTR: VARENUM = VARENUM(30u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_LPWSTR: VARENUM = VARENUM(31u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_RECORD: VARENUM = VARENUM(36u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_INT_PTR: VARENUM = VARENUM(37u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_UINT_PTR: VARENUM = VARENUM(38u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_FILETIME: VARENUM = VARENUM(64u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_BLOB: VARENUM = VARENUM(65u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_STREAM: VARENUM = VARENUM(66u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_STORAGE: VARENUM = VARENUM(67u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_STREAMED_OBJECT: VARENUM = VARENUM(68u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_STORED_OBJECT: VARENUM = VARENUM(69u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_BLOB_OBJECT: VARENUM = VARENUM(70u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_CF: VARENUM = VARENUM(71u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_CLSID: VARENUM = VARENUM(72u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_VERSIONED_STREAM: VARENUM = VARENUM(73u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_BSTR_BLOB: VARENUM = VARENUM(4095u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_VECTOR: VARENUM = VARENUM(4096u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_ARRAY: VARENUM = VARENUM(8192u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_BYREF: VARENUM = VARENUM(16384u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_RESERVED: VARENUM = VARENUM(32768u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_ILLEGAL: VARENUM = VARENUM(65535u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_ILLEGALMASKED: VARENUM = VARENUM(4095u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VT_TYPEMASK: VARENUM = VARENUM(4095u16);
impl ::core::marker::Copy for VARENUM {}
impl ::core::clone::Clone for VARENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFLAGS(pub u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FREADONLY: VARFLAGS = VARFLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FSOURCE: VARFLAGS = VARFLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FBINDABLE: VARFLAGS = VARFLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = VARFLAGS(8u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = VARFLAGS(16u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = VARFLAGS(32u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FHIDDEN: VARFLAGS = VARFLAGS(64u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FRESTRICTED: VARFLAGS = VARFLAGS(128u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = VARFLAGS(256u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FUIDEFAULT: VARFLAGS = VARFLAGS(512u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = VARFLAGS(1024u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FREPLACEABLE: VARFLAGS = VARFLAGS(2048u16);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = VARFLAGS(4096u16);
impl ::core::marker::Copy for VARFLAGS {}
impl ::core::clone::Clone for VARFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_CONST: VARKIND = VARKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
impl ::core::marker::Copy for VARKIND {}
impl ::core::clone::Clone for VARKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AUTHENTICATEINFO {}
impl ::core::clone::Clone for AUTHENTICATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHENTICATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICATEINFO").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHENTICATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for AUTHENTICATEINFO {}
impl ::core::default::Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: ::windows::core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: ::windows::core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: ::windows::core::GUID,
    pub pUnk: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for BINDINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for BINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ::std::mem::ManuallyDrop<::core::option::Option<ITypeComp>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for BINDPTR {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for BINDPTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl ::core::marker::Copy for BIND_OPTS {}
impl ::core::clone::Clone for BIND_OPTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS").field("cbStruct", &self.cbStruct).field("grfFlags", &self.grfFlags).field("grfMode", &self.grfMode).field("dwTickCountDeadline", &self.dwTickCountDeadline).finish()
    }
}
unsafe impl ::windows::core::Abi for BIND_OPTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.grfFlags == other.grfFlags && self.grfMode == other.grfMode && self.dwTickCountDeadline == other.dwTickCountDeadline
    }
}
impl ::core::cmp::Eq for BIND_OPTS {}
impl ::core::default::Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl ::core::marker::Copy for BIND_OPTS2 {}
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS2").field("Base", &self.Base).field("dwTrackFlags", &self.dwTrackFlags).field("dwClassContext", &self.dwClassContext).field("locale", &self.locale).field("pServerInfo", &self.pServerInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for BIND_OPTS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.dwTrackFlags == other.dwTrackFlags && self.dwClassContext == other.dwClassContext && self.locale == other.locale && self.pServerInfo == other.pServerInfo
    }
}
impl ::core::cmp::Eq for BIND_OPTS2 {}
impl ::core::default::Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIND_OPTS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS3").field("Base", &self.Base).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIND_OPTS3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.hwnd == other.hwnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl ::core::marker::Copy for BLOB {}
impl ::core::clone::Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLOB").field("cbSize", &self.cbSize).field("pBlobData", &self.pBlobData).finish()
    }
}
unsafe impl ::windows::core::Abi for BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pBlobData == other.pBlobData
    }
}
impl ::core::cmp::Eq for BLOB {}
impl ::core::default::Default for BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for BYTE_BLOB {}
impl ::core::clone::Clone for BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_BLOB").field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows::core::Abi for BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::core::cmp::Eq for BYTE_BLOB {}
impl ::core::default::Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BYTE_SIZEDARR {}
impl ::core::clone::Clone for BYTE_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for BYTE_SIZEDARR {}
impl ::core::default::Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CATEGORYINFO {
    pub catid: ::windows::core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl ::core::marker::Copy for CATEGORYINFO {}
impl ::core::clone::Clone for CATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORYINFO").field("catid", &self.catid).field("lcid", &self.lcid).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for CATEGORYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.catid == other.catid && self.lcid == other.lcid && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for CATEGORYINFO {}
impl ::core::default::Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for COAUTHIDENTITY {}
impl ::core::clone::Clone for COAUTHIDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHIDENTITY").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for COAUTHIDENTITY {}
impl ::core::default::Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: ::windows::core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl ::core::marker::Copy for COAUTHINFO {}
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHINFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pwszServerPrincName", &self.pwszServerPrincName).field("dwAuthnLevel", &self.dwAuthnLevel).field("dwImpersonationLevel", &self.dwImpersonationLevel).field("pAuthIdentityData", &self.pAuthIdentityData).field("dwCapabilities", &self.dwCapabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for COAUTHINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pwszServerPrincName == other.pwszServerPrincName && self.dwAuthnLevel == other.dwAuthnLevel && self.dwImpersonationLevel == other.dwImpersonationLevel && self.pAuthIdentityData == other.pAuthIdentityData && self.dwCapabilities == other.dwCapabilities
    }
}
impl ::core::cmp::Eq for COAUTHINFO {}
impl ::core::default::Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CONNECTDATA {
    pub pUnk: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
    pub dwCookie: u32,
}
impl ::core::clone::Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for CONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDATA").field("pUnk", &self.pUnk).field("dwCookie", &self.dwCookie).finish()
    }
}
unsafe impl ::windows::core::Abi for CONNECTDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::core::cmp::Eq for CONNECTDATA {}
impl ::core::default::Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: ::windows::core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for COSERVERINFO {}
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COSERVERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COSERVERINFO").field("dwReserved1", &self.dwReserved1).field("pwszName", &self.pwszName).field("pAuthInfo", &self.pAuthInfo).field("dwReserved2", &self.dwReserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for COSERVERINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1 && self.pwszName == other.pwszName && self.pAuthInfo == other.pAuthInfo && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for COSERVERINFO {}
impl ::core::default::Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub isize);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_DEVICE_CATALOG_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_DEVICE_CATALOG_COOKIE {}
impl ::core::fmt::Debug for CO_DEVICE_CATALOG_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_DEVICE_CATALOG_COOKIE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<CO_DEVICE_CATALOG_COOKIE>> for CO_DEVICE_CATALOG_COOKIE {
    fn from(optional: ::core::option::Option<CO_DEVICE_CATALOG_COOKIE>) -> CO_DEVICE_CATALOG_COOKIE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for CO_DEVICE_CATALOG_COOKIE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CO_MTA_USAGE_COOKIE(pub isize);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_MTA_USAGE_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_MTA_USAGE_COOKIE {}
impl ::core::fmt::Debug for CO_MTA_USAGE_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MTA_USAGE_COOKIE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<CO_MTA_USAGE_COOKIE>> for CO_MTA_USAGE_COOKIE {
    fn from(optional: ::core::option::Option<CO_MTA_USAGE_COOKIE>) -> CO_MTA_USAGE_COOKIE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for CO_MTA_USAGE_COOKIE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl ::core::marker::Copy for CSPLATFORM {}
impl ::core::clone::Clone for CSPLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSPLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSPLATFORM").field("dwPlatformId", &self.dwPlatformId).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).field("dwProcessorArch", &self.dwProcessorArch).finish()
    }
}
unsafe impl ::windows::core::Abi for CSPLATFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        self.dwPlatformId == other.dwPlatformId && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo && self.dwProcessorArch == other.dwProcessorArch
    }
}
impl ::core::cmp::Eq for CSPLATFORM {}
impl ::core::default::Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for CUSTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTDATA").field("cCustData", &self.cCustData).field("prgCustData", &self.prgCustData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cCustData == other.cCustData && self.prgCustData == other.prgCustData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATAITEM {
    pub guid: ::windows::core::GUID,
    pub varValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATAITEM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl ::core::marker::Copy for CY {}
impl ::core::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CY {
    type Abi = Self;
}
impl ::core::default::Default for CY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl ::core::marker::Copy for CY_0 {}
impl ::core::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CY_0").field("Lo", &self.Lo).field("Hi", &self.Hi).finish()
    }
}
unsafe impl ::windows::core::Abi for CY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Lo == other.Lo && self.Hi == other.Hi
    }
}
impl ::core::cmp::Eq for CY_0 {}
impl ::core::default::Default for CY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ComCallData {}
impl ::core::clone::Clone for ComCallData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComCallData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComCallData").field("dwDispid", &self.dwDispid).field("dwReserved", &self.dwReserved).field("pUserDefined", &self.pUserDefined).finish()
    }
}
unsafe impl ::windows::core::Abi for ComCallData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        self.dwDispid == other.dwDispid && self.dwReserved == other.dwReserved && self.pUserDefined == other.pUserDefined
    }
}
impl ::core::cmp::Eq for ComCallData {}
impl ::core::default::Default for ComCallData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for DISPPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPPARAMS").field("rgvarg", &self.rgvarg).field("rgdispidNamedArgs", &self.rgdispidNamedArgs).field("cArgs", &self.cArgs).field("cNamedArgs", &self.cNamedArgs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for DISPPARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.rgvarg == other.rgvarg && self.rgdispidNamedArgs == other.rgdispidNamedArgs && self.cArgs == other.cArgs && self.cNamedArgs == other.cNamedArgs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl ::core::marker::Copy for DVTARGETDEVICE {}
impl ::core::clone::Clone for DVTARGETDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVTARGETDEVICE").field("tdSize", &self.tdSize).field("tdDriverNameOffset", &self.tdDriverNameOffset).field("tdDeviceNameOffset", &self.tdDeviceNameOffset).field("tdPortNameOffset", &self.tdPortNameOffset).field("tdExtDevmodeOffset", &self.tdExtDevmodeOffset).field("tdData", &self.tdData).finish()
    }
}
unsafe impl ::windows::core::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.tdSize == other.tdSize && self.tdDriverNameOffset == other.tdDriverNameOffset && self.tdDeviceNameOffset == other.tdDeviceNameOffset && self.tdPortNameOffset == other.tdPortNameOffset && self.tdExtDevmodeOffset == other.tdExtDevmodeOffset && self.tdData == other.tdData
    }
}
impl ::core::cmp::Eq for DVTARGETDEVICE {}
impl ::core::default::Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl ::core::marker::Copy for DWORD_BLOB {}
impl ::core::clone::Clone for DWORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_BLOB").field("clSize", &self.clSize).field("alData", &self.alData).finish()
    }
}
unsafe impl ::windows::core::Abi for DWORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.alData == other.alData
    }
}
impl ::core::cmp::Eq for DWORD_BLOB {}
impl ::core::default::Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DWORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl ::core::marker::Copy for DWORD_SIZEDARR {}
impl ::core::clone::Clone for DWORD_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for DWORD_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWORD_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for DWORD_SIZEDARR {}
impl ::core::default::Default for DWORD_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub bstrDescription: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub bstrHelpFile: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl ::core::clone::Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for EXCEPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPINFO").field("wCode", &self.wCode).field("wReserved", &self.wReserved).field("bstrSource", &self.bstrSource).field("bstrDescription", &self.bstrDescription).field("bstrHelpFile", &self.bstrHelpFile).field("dwHelpContext", &self.dwHelpContext).field("pvReserved", &self.pvReserved).field("scode", &self.scode).finish()
    }
}
unsafe impl ::windows::core::Abi for EXCEPINFO {
    type Abi = Self;
}
impl ::core::default::Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for FLAGGED_BYTE_BLOB {}
impl ::core::clone::Clone for FLAGGED_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_BYTE_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows::core::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::core::cmp::Eq for FLAGGED_BYTE_BLOB {}
impl ::core::default::Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for FLAGGED_WORD_BLOB {}
impl ::core::clone::Clone for FLAGGED_WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_WORD_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows::core::Abi for FLAGGED_WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::core::cmp::Eq for FLAGGED_WORD_BLOB {}
impl ::core::default::Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for FLAG_STGMEDIUM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for FORMATETC {}
impl ::core::clone::Clone for FORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
unsafe impl ::windows::core::Abi for FORMATETC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat && self.ptd == other.ptd && self.dwAspect == other.dwAspect && self.lindex == other.lindex && self.tymed == other.tymed
    }
}
impl ::core::cmp::Eq for FORMATETC {}
impl ::core::default::Default for FORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: FUNCFLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for FUNCDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for FUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for GDI_OBJECT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl ::core::marker::Copy for HYPER_SIZEDARR {}
impl ::core::clone::Clone for HYPER_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPER_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for HYPER_SIZEDARR {}
impl ::core::default::Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IContext(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
impl ::core::marker::Copy for IDLDESC {}
impl ::core::clone::Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDLDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDLDESC").field("dwReserved", &self.dwReserved).field("wIDLFlags", &self.wIDLFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for IDLDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved && self.wIDLFlags == other.wIDLFlags
    }
}
impl ::core::cmp::Eq for IDLDESC {}
impl ::core::default::Default for IDLDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IEnumContextProps(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct INTERFACEINFO {
    pub pUnk: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
    pub iid: ::windows::core::GUID,
    pub wMethod: u16,
}
impl ::core::clone::Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEINFO").field("pUnk", &self.pUnk).field("iid", &self.iid).field("wMethod", &self.wMethod).finish()
    }
}
unsafe impl ::windows::core::Abi for INTERFACEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::core::cmp::Eq for INTERFACEINFO {}
impl ::core::default::Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct MULTI_QI {
    pub pIID: *const ::windows::core::GUID,
    pub pItf: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
    pub hr: ::windows::core::HRESULT,
}
impl ::core::clone::Clone for MULTI_QI {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for MULTI_QI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTI_QI").field("pIID", &self.pIID).field("pItf", &self.pItf).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows::core::Abi for MULTI_QI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for MULTI_QI {}
impl ::core::default::Default for MULTI_QI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MachineGlobalObjectTableRegistrationToken__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::default::Default for MachineGlobalObjectTableRegistrationToken__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl ::core::marker::Copy for QUERYCONTEXT {}
impl ::core::clone::Clone for QUERYCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERYCONTEXT").field("dwContext", &self.dwContext).field("Platform", &self.Platform).field("Locale", &self.Locale).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERYCONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwContext == other.dwContext && self.Platform == other.Platform && self.Locale == other.Locale && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo
    }
}
impl ::core::cmp::Eq for QUERYCONTEXT {}
impl ::core::default::Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl ::core::marker::Copy for RPCOLEMESSAGE {}
impl ::core::clone::Clone for RPCOLEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPCOLEMESSAGE").field("reserved1", &self.reserved1).field("dataRepresentation", &self.dataRepresentation).field("Buffer", &self.Buffer).field("cbBuffer", &self.cbBuffer).field("iMethod", &self.iMethod).field("reserved2", &self.reserved2).field("rpcFlags", &self.rpcFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.dataRepresentation == other.dataRepresentation && self.Buffer == other.Buffer && self.cbBuffer == other.cbBuffer && self.iMethod == other.iMethod && self.reserved2 == other.reserved2 && self.rpcFlags == other.rpcFlags
    }
}
impl ::core::cmp::Eq for RPCOLEMESSAGE {}
impl ::core::default::Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct RemSTGMEDIUM {
    pub tymed: TYMED,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemSTGMEDIUM {}
impl ::core::clone::Clone for RemSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSTGMEDIUM").field("tymed", &self.tymed).field("dwHandleType", &self.dwHandleType).field("pData", &self.pData).field("pUnkForRelease", &self.pUnkForRelease).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.dwHandleType == other.dwHandleType && self.pData == other.pData && self.pUnkForRelease == other.pUnkForRelease && self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemSTGMEDIUM {}
impl ::core::default::Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl ::core::marker::Copy for SAFEARRAY {}
impl ::core::clone::Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAY").field("cDims", &self.cDims).field("fFeatures", &self.fFeatures).field("cbElements", &self.cbElements).field("cLocks", &self.cLocks).field("pvData", &self.pvData).field("rgsabound", &self.rgsabound).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cDims == other.cDims && self.fFeatures == other.fFeatures && self.cbElements == other.cbElements && self.cLocks == other.cLocks && self.pvData == other.pvData && self.rgsabound == other.rgsabound
    }
}
impl ::core::cmp::Eq for SAFEARRAY {}
impl ::core::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl ::core::marker::Copy for SAFEARRAYBOUND {}
impl ::core::clone::Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAYBOUND").field("cElements", &self.cElements).field("lLbound", &self.lLbound).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        self.cElements == other.cElements && self.lLbound == other.lLbound
    }
}
impl ::core::cmp::Eq for SAFEARRAYBOUND {}
impl ::core::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SChannelHookCallInfo {
    pub iid: ::windows::core::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows::core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SChannelHookCallInfo {}
impl ::core::clone::Clone for SChannelHookCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SChannelHookCallInfo").field("iid", &self.iid).field("cbSize", &self.cbSize).field("uCausality", &self.uCausality).field("dwServerPid", &self.dwServerPid).field("iMethod", &self.iMethod).field("pObject", &self.pObject).finish()
    }
}
unsafe impl ::windows::core::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid && self.cbSize == other.cbSize && self.uCausality == other.uCausality && self.dwServerPid == other.dwServerPid && self.iMethod == other.iMethod && self.pObject == other.pObject
    }
}
impl ::core::cmp::Eq for SChannelHookCallInfo {}
impl ::core::default::Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_INFO {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_INFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pAuthInfo", &self.pAuthInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pAuthInfo == other.pAuthInfo
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
impl ::core::default::Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_LIST {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_LIST").field("cAuthInfo", &self.cAuthInfo).field("aAuthInfo", &self.aAuthInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cAuthInfo == other.cAuthInfo && self.aAuthInfo == other.aAuthInfo
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
impl ::core::default::Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: ::windows::core::PWSTR,
    pub hr: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_SERVICE").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pPrincipalName", &self.pPrincipalName).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pPrincipalName == other.pPrincipalName && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::default::Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: ::windows::core::ManuallyDrop<IAdviseSink>,
    pub dwConnection: u32,
}
impl ::core::clone::Clone for STATDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for STATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATDATA").field("formatetc", &self.formatetc).field("advf", &self.advf).field("pAdvSink", &self.pAdvSink).field("dwConnection", &self.dwConnection).finish()
    }
}
unsafe impl ::windows::core::Abi for STATDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc && self.advf == other.advf && self.pAdvSink == other.pAdvSink && self.dwConnection == other.dwConnection
    }
}
impl ::core::cmp::Eq for STATDATA {}
impl ::core::default::Default for STATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: ::windows::core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: STGM,
    pub grfLocksSupported: LOCKTYPE,
    pub clsid: ::windows::core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATSTG").field("pwcsName", &self.pwcsName).field("type", &self.r#type).field("cbSize", &self.cbSize).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("grfMode", &self.grfMode).field("grfLocksSupported", &self.grfLocksSupported).field("clsid", &self.clsid).field("grfStateBits", &self.grfStateBits).field("reserved", &self.reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STATSTG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsName == other.pwcsName && self.r#type == other.r#type && self.cbSize == other.cbSize && self.mtime == other.mtime && self.ctime == other.ctime && self.atime == other.atime && self.grfMode == other.grfMode && self.grfLocksSupported == other.grfLocksSupported && self.clsid == other.clsid && self.grfStateBits == other.grfStateBits && self.reserved == other.reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: TYMED,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STGMEDIUM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: ::windows::core::PWSTR,
    pub pstm: ::std::mem::ManuallyDrop<::core::option::Option<IStream>>,
    pub pstg: ::std::mem::ManuallyDrop<::core::option::Option<StructuredStorage::IStorage>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STGMEDIUM_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: ::windows::core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl ::core::marker::Copy for StorageLayout {}
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StorageLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorageLayout").field("LayoutType", &self.LayoutType).field("pwcsElementName", &self.pwcsElementName).field("cOffset", &self.cOffset).field("cBytes", &self.cBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for StorageLayout {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        self.LayoutType == other.LayoutType && self.pwcsElementName == other.pwcsElementName && self.cOffset == other.cOffset && self.cBytes == other.cBytes
    }
}
impl ::core::cmp::Eq for StorageLayout {}
impl ::core::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct TLIBATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl ::core::marker::Copy for TLIBATTR {}
impl ::core::clone::Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TLIBATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TLIBATTR").field("guid", &self.guid).field("lcid", &self.lcid).field("syskind", &self.syskind).field("wMajorVerNum", &self.wMajorVerNum).field("wMinorVerNum", &self.wMinorVerNum).field("wLibFlags", &self.wLibFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for TLIBATTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.lcid == other.lcid && self.syskind == other.syskind && self.wMajorVerNum == other.wMajorVerNum && self.wMinorVerNum == other.wMinorVerNum && self.wLibFlags == other.wLibFlags
    }
}
impl ::core::cmp::Eq for TLIBATTR {}
impl ::core::default::Default for TLIBATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: ::windows::core::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEATTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEATTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: VARENUM,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: ::windows::core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub punkVal: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub pdispVal: ::std::mem::ManuallyDrop<::core::option::Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::windows::core::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: ::windows::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::windows::core::ManuallyDrop<super::Ole::IRecordInfo>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0_0").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for WORD_BLOB {}
impl ::core::clone::Clone for WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WORD_BLOB").field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows::core::Abi for WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::core::cmp::Eq for WORD_BLOB {}
impl ::core::default::Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct WORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl ::core::marker::Copy for WORD_SIZEDARR {}
impl ::core::clone::Clone for WORD_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WORD_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WORD_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for WORD_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WORD_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WORD_SIZEDARR {}
impl ::core::default::Default for WORD_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl ::core::marker::Copy for uCLSSPEC {}
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC {
    type Abi = Self;
}
impl ::core::default::Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub union uCLSSPEC_0 {
    pub clsid: ::windows::core::GUID,
    pub pFileExt: ::windows::core::PWSTR,
    pub pMimeType: ::windows::core::PWSTR,
    pub pProgId: ::windows::core::PWSTR,
    pub pFileName: ::windows::core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl ::core::marker::Copy for uCLSSPEC_0 {}
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
impl ::core::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: ::windows::core::PWSTR,
    pub PolicyId: ::windows::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_0").field("pPackageName", &self.pPackageName).field("PolicyId", &self.PolicyId).finish()
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pPackageName == other.pPackageName && self.PolicyId == other.PolicyId
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_0 {}
impl ::core::default::Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows::core::GUID,
    pub PolicyId: ::windows::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_1").field("ObjectId", &self.ObjectId).field("PolicyId", &self.PolicyId).finish()
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectId == other.ObjectId && self.PolicyId == other.PolicyId
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_1 {}
impl ::core::default::Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl ::core::clone::Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userFLAG_STGMEDIUM").field("ContextFlags", &self.ContextFlags).field("fPassOwnership", &self.fPassOwnership).field("Stgmed", &self.Stgmed).finish()
    }
}
unsafe impl ::windows::core::Abi for userFLAG_STGMEDIUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
impl ::core::cmp::Eq for userFLAG_STGMEDIUM {}
impl ::core::default::Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM").field("pUnkForRelease", &self.pUnkForRelease).finish()
    }
}
unsafe impl ::windows::core::Abi for userSTGMEDIUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM {}
impl ::core::default::Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: ::windows::core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPEXCEPFINO_DEFERRED_FILLIN = ::core::option::Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPFNCANUNLOADNOW = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPFNGETCLASSOBJECT = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type PFNCONTEXTCALL = ::core::option::Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
