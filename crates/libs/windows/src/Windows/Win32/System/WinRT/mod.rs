#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64) -> ::windows::core::Result<ServerInformation> {
    ::windows::core::link ! ( "ole32.dll""system" fn CoDecodeProxy ( dwclientpid : u32 , ui64proxyaddress : u64 , pserverinformation : *mut ServerInformation ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoDecodeProxy(dwclientpid, ui64proxyaddress, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateControlInput<T>() -> ::windows::core::Result<T>
where
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "windows.ui.dll""cdecl" fn CreateControlInput ( riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateControlInput(&<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateControlInputEx<P0, T>(pcorewindow: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "windows.ui.dll""cdecl" fn CreateControlInputEx ( pcorewindow : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateControlInputEx(pcorewindow.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"System\"`*"]
#[cfg(feature = "System")]
#[inline]
pub unsafe fn CreateDispatcherQueueController(options: DispatcherQueueOptions) -> ::windows::core::Result<super::super::super::System::DispatcherQueueController> {
    ::windows::core::link ! ( "coremessaging.dll""system" fn CreateDispatcherQueueController ( options : DispatcherQueueOptions , dispatcherqueuecontroller : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateDispatcherQueueController(::core::mem::transmute(options), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<P0, T>(filepath: P0, accessmode: u32) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "api-ms-win-shcore-stream-winrt-l1-1-0.dll""system" fn CreateRandomAccessStreamOnFile ( filepath : :: windows::core::PCWSTR , accessmode : u32 , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateRandomAccessStreamOnFile(filepath.into().abi(), accessmode, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<P0, T>(stream: P0, options: BSOS_OPTIONS) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "api-ms-win-shcore-stream-winrt-l1-1-0.dll""system" fn CreateRandomAccessStreamOverStream ( stream : * mut::core::ffi::c_void , options : BSOS_OPTIONS , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateRandomAccessStreamOverStream(stream.into().abi(), options, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<P0, T>(randomaccessstream: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "api-ms-win-shcore-stream-winrt-l1-1-0.dll""system" fn CreateStreamOverRandomAccessStream ( randomaccessstream : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateStreamOverRandomAccessStream(randomaccessstream.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> ::windows::core::Result<IRestrictedErrorInfo> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn GetRestrictedErrorInfo ( pprestrictederrorinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetRestrictedErrorInfo(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows::core::HSTRING) {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserFree ( param0 : *const u32 , param1 : *const * mut::core::ffi::c_void ) -> ( ) );
    HSTRING_UserFree(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows::core::HSTRING) {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserFree64 ( param0 : *const u32 , param1 : *const * mut::core::ffi::c_void ) -> ( ) );
    HSTRING_UserFree64(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows::core::HSTRING) -> *mut u8 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const * mut::core::ffi::c_void ) -> *mut u8 );
    HSTRING_UserMarshal(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows::core::HSTRING) -> *mut u8 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const * mut::core::ffi::c_void ) -> *mut u8 );
    HSTRING_UserMarshal64(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows::core::HSTRING) -> u32 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const * mut::core::ffi::c_void ) -> u32 );
    HSTRING_UserSize(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows::core::HSTRING) -> u32 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const * mut::core::ffi::c_void ) -> u32 );
    HSTRING_UserSize64(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows::core::HSTRING) -> *mut u8 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut * mut::core::ffi::c_void ) -> *mut u8 );
    HSTRING_UserUnmarshal(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows::core::HSTRING) -> *mut u8 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn HSTRING_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut * mut::core::ffi::c_void ) -> *mut u8 );
    HSTRING_UserUnmarshal64(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn IsErrorPropagationEnabled ( ) -> super::super::Foundation:: BOOL );
    IsErrorPropagationEnabled()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "rometadata.dll""system" fn MetaDataGetDispenser ( rclsid : *const :: windows::core::GUID , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    MetaDataGetDispenser(rclsid, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoActivateInstance(activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoActivateInstance ( activatableclassid : * mut::core::ffi::c_void , instance : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoActivateInstance(::core::mem::transmute_copy(activatableclassid), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoCaptureErrorContext ( hr : :: windows::core::HRESULT ) -> :: windows::core::HRESULT );
    RoCaptureErrorContext(hr).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoClearError() {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoClearError ( ) -> ( ) );
    RoClearError()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: ::windows::core::HRESULT) {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoFailFastWithErrorContext ( hrerror : :: windows::core::HRESULT ) -> ( ) );
    RoFailFastWithErrorContext(hrerror)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra<P0>(extra: P0)
where
    P0: ::std::convert::Into<ROPARAMIIDHANDLE>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll""system" fn RoFreeParameterizedTypeExtra ( extra : ROPARAMIIDHANDLE ) -> ( ) );
    RoFreeParameterizedTypeExtra(extra.into())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetActivationFactory<T>(activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<T>
where
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoGetActivationFactory ( activatableclassid : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , factory : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetActivationFactory(::core::mem::transmute_copy(activatableclassid), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetAgileReference<P0>(options: AgileReferenceOptions, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<IAgileReference>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn RoGetAgileReference ( options : AgileReferenceOptions , riid : *const :: windows::core::GUID , punk : * mut::core::ffi::c_void , ppagilereference : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetAgileReference(options, riid, punk.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoGetApartmentIdentifier ( apartmentidentifier : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetApartmentIdentifier(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_System_Com_Marshal\"`*"]
#[cfg(feature = "Win32_System_Com_Marshal")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> ::windows::core::Result<super::Com::Marshal::IMarshal> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-robuffer-l1-1-0.dll""system" fn RoGetBufferMarshaler ( buffermarshaler : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetBufferMarshaler(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoGetErrorReportingFlags ( pflags : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetErrorReportingFlags(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows::core::HRESULT) -> ::windows::core::Result<IRestrictedErrorInfo> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoGetMatchingRestrictedErrorInfo ( hrin : :: windows::core::HRESULT , pprestrictederrorinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetMatchingRestrictedErrorInfo(hrin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<P0>(nameelements: &[::windows::core::PCWSTR], metadatalocator: P0, iid: *mut ::windows::core::GUID, pextra: ::core::option::Option<*mut ROPARAMIIDHANDLE>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRoMetaDataLocator>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll""system" fn RoGetParameterizedTypeInstanceIID ( nameelementcount : u32 , nameelements : *const :: windows::core::PCWSTR , metadatalocator : * mut::core::ffi::c_void , iid : *mut :: windows::core::GUID , pextra : *mut ROPARAMIIDHANDLE ) -> :: windows::core::HRESULT );
    RoGetParameterizedTypeInstanceIID(nameelements.len() as _, ::core::mem::transmute(nameelements.as_ptr()), metadatalocator.into().abi(), iid, ::core::mem::transmute(pextra.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetServerActivatableClasses(servername: &::windows::core::HSTRING, activatableclassids: *mut *mut ::windows::core::HSTRING, count: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-registration-l1-1-0.dll""system" fn RoGetServerActivatableClasses ( servername : * mut::core::ffi::c_void , activatableclassids : *mut *mut :: windows::core::HSTRING , count : *mut u32 ) -> :: windows::core::HRESULT );
    RoGetServerActivatableClasses(::core::mem::transmute_copy(servername), activatableclassids, count).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoInitialize ( inittype : RO_INIT_TYPE ) -> :: windows::core::HRESULT );
    RoInitialize(inittype).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoInspectCapturedStackBackTrace ( targeterrorinfoaddress : usize , machine : u16 , readmemorycallback : PINSPECT_MEMORY_CALLBACK , context : *const ::core::ffi::c_void , framecount : *mut u32 , targetbacktraceaddress : *mut usize ) -> :: windows::core::HRESULT );
    RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, readmemorycallback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), framecount, targetbacktraceaddress).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<usize> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoInspectThreadErrorInfo ( targettebaddress : usize , machine : u16 , readmemorycallback : PINSPECT_MEMORY_CALLBACK , context : *const ::core::ffi::c_void , targeterrorinfoaddress : *mut usize ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoInspectThreadErrorInfo(targettebaddress, machine, readmemorycallback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoOriginateError(error: ::windows::core::HRESULT, message: &::windows::core::HSTRING) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoOriginateError ( error : :: windows::core::HRESULT , message : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RoOriginateError(error, ::core::mem::transmute_copy(message))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoOriginateErrorW(error: ::windows::core::HRESULT, cchmax: u32, message: ::core::option::Option<&[u16; 512]>) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoOriginateErrorW ( error : :: windows::core::HRESULT , cchmax : u32 , message : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    RoOriginateErrorW(error, cchmax, ::core::mem::transmute(message.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoOriginateLanguageException<P0>(error: ::windows::core::HRESULT, message: &::windows::core::HSTRING, languageexception: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoOriginateLanguageException ( error : :: windows::core::HRESULT , message : * mut::core::ffi::c_void , languageexception : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RoOriginateLanguageException(error, ::core::mem::transmute_copy(message), languageexception.into().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature<P0>(extra: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<ROPARAMIIDHANDLE>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll""system" fn RoParameterizedTypeExtraGetTypeSignature ( extra : ROPARAMIIDHANDLE ) -> :: windows::core::PSTR );
    RoParameterizedTypeExtraGetTypeSignature(extra.into())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const ::windows::core::HSTRING, activationfactorycallbacks: *const isize, count: u32) -> ::windows::core::Result<isize> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoRegisterActivationFactories ( activatableclassids : *const * mut::core::ffi::c_void , activationfactorycallbacks : *const isize , count : u32 , cookie : *mut isize ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoRegisterActivationFactories(::core::mem::transmute(activatableclassids), activationfactorycallbacks, count, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<P0>(callbackobject: P0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IApartmentShutdown>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoRegisterForApartmentShutdown ( callbackobject : * mut::core::ffi::c_void , apartmentidentifier : *mut u64 , regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE ) -> :: windows::core::HRESULT );
    RoRegisterForApartmentShutdown(callbackobject.into().abi(), apartmentidentifier, regcookie).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoReportFailedDelegate<P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<IRestrictedErrorInfo>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoReportFailedDelegate ( punkdelegate : * mut::core::ffi::c_void , prestrictederrorinfo : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    RoReportFailedDelegate(punkdelegate.into().abi(), prestrictederrorinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoReportUnhandledError<P0>(prestrictederrorinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRestrictedErrorInfo>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-1.dll""system" fn RoReportUnhandledError ( prestrictederrorinfo : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    RoReportUnhandledError(prestrictederrorinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<P0>(reference: P0) -> ::windows::core::Result<IRestrictedErrorInfo>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoResolveRestrictedErrorInfoReference ( reference : :: windows::core::PCWSTR , pprestrictederrorinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoResolveRestrictedErrorInfoReference(reference.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: isize) {
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoRevokeActivationFactories ( cookie : isize ) -> ( ) );
    RoRevokeActivationFactories(cookie)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoSetErrorReportingFlags ( flags : u32 ) -> :: windows::core::HRESULT );
    RoSetErrorReportingFlags(flags).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoTransformError(olderror: ::windows::core::HRESULT, newerror: ::windows::core::HRESULT, message: &::windows::core::HSTRING) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoTransformError ( olderror : :: windows::core::HRESULT , newerror : :: windows::core::HRESULT , message : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RoTransformError(olderror, newerror, ::core::mem::transmute_copy(message))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoTransformErrorW(olderror: ::windows::core::HRESULT, newerror: ::windows::core::HRESULT, cchmax: u32, message: ::core::option::Option<&[u16; 512]>) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn RoTransformErrorW ( olderror : :: windows::core::HRESULT , newerror : :: windows::core::HRESULT , cchmax : u32 , message : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    RoTransformErrorW(olderror, newerror, cchmax, ::core::mem::transmute(message.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoUninitialize() {
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoUninitialize ( ) -> ( ) );
    RoUninitialize()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown<P0>(regcookie: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-l1-1-0.dll""system" fn RoUnregisterForApartmentShutdown ( regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE ) -> :: windows::core::HRESULT );
    RoUnregisterForApartmentShutdown(regcookie.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn SetRestrictedErrorInfo<P0>(prestrictederrorinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRestrictedErrorInfo>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-error-l1-1-0.dll""system" fn SetRestrictedErrorInfo ( prestrictederrorinfo : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SetRestrictedErrorInfo(prestrictederrorinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsCompareStringOrdinal(string1: &::windows::core::HSTRING, string2: &::windows::core::HSTRING) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsCompareStringOrdinal ( string1 : * mut::core::ffi::c_void , string2 : * mut::core::ffi::c_void , result : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsCompareStringOrdinal(::core::mem::transmute_copy(string1), ::core::mem::transmute_copy(string2), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsConcatString(string1: &::windows::core::HSTRING, string2: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsConcatString ( string1 : * mut::core::ffi::c_void , string2 : * mut::core::ffi::c_void , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsConcatString(::core::mem::transmute_copy(string1), ::core::mem::transmute_copy(string2), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsCreateString(sourcestring: ::core::option::Option<&[u16]>) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsCreateString ( sourcestring : :: windows::core::PCWSTR , length : u32 , string : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsCreateString(::core::mem::transmute(sourcestring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), sourcestring.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsCreateStringReference<P0>(sourcestring: P0, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows::core::HSTRING) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsCreateStringReference ( sourcestring : :: windows::core::PCWSTR , length : u32 , hstringheader : *mut HSTRING_HEADER , string : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WindowsCreateStringReference(sourcestring.into().abi(), length, hstringheader, ::core::mem::transmute(string)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsDeleteString(string: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsDeleteString ( string : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WindowsDeleteString(::core::mem::transmute_copy(string)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsDeleteStringBuffer<P0>(bufferhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HSTRING_BUFFER>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsDeleteStringBuffer ( bufferhandle : HSTRING_BUFFER ) -> :: windows::core::HRESULT );
    WindowsDeleteStringBuffer(bufferhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsDuplicateString(string: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsDuplicateString ( string : * mut::core::ffi::c_void , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsDuplicateString(::core::mem::transmute_copy(string), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsGetStringLen(string: &::windows::core::HSTRING) -> u32 {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsGetStringLen ( string : * mut::core::ffi::c_void ) -> u32 );
    WindowsGetStringLen(::core::mem::transmute_copy(string))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsGetStringRawBuffer(string: &::windows::core::HSTRING, length: ::core::option::Option<*mut u32>) -> ::windows::core::PWSTR {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsGetStringRawBuffer ( string : * mut::core::ffi::c_void , length : *mut u32 ) -> :: windows::core::PWSTR );
    WindowsGetStringRawBuffer(::core::mem::transmute_copy(string), ::core::mem::transmute(length.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>, length: *mut u32, targetstringaddress: *mut usize) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsInspectString ( targethstring : usize , machine : u16 , callback : PINSPECT_HSTRING_CALLBACK , context : *const ::core::ffi::c_void , length : *mut u32 , targetstringaddress : *mut usize ) -> :: windows::core::HRESULT );
    WindowsInspectString(targethstring, machine, callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), length, targetstringaddress).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: ::core::option::Option<*const ::core::ffi::c_void>, length: *mut u32, targetstringaddress: *mut u64) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-1.dll""system" fn WindowsInspectString2 ( targethstring : u64 , machine : u16 , callback : PINSPECT_HSTRING_CALLBACK2 , context : *const ::core::ffi::c_void , length : *mut u32 , targetstringaddress : *mut u64 ) -> :: windows::core::HRESULT );
    WindowsInspectString2(targethstring, machine, callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), length, targetstringaddress).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowsIsStringEmpty(string: &::windows::core::HSTRING) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsIsStringEmpty ( string : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    WindowsIsStringEmpty(::core::mem::transmute_copy(string))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsPreallocateStringBuffer ( length : u32 , charbuffer : *mut *mut u16 , bufferhandle : *mut HSTRING_BUFFER ) -> :: windows::core::HRESULT );
    WindowsPreallocateStringBuffer(length, charbuffer, bufferhandle).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsPromoteStringBuffer<P0>(bufferhandle: P0) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<HSTRING_BUFFER>,
{
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsPromoteStringBuffer ( bufferhandle : HSTRING_BUFFER , string : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsPromoteStringBuffer(bufferhandle.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsReplaceString(string: &::windows::core::HSTRING, stringreplaced: &::windows::core::HSTRING, stringreplacewith: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsReplaceString ( string : * mut::core::ffi::c_void , stringreplaced : * mut::core::ffi::c_void , stringreplacewith : * mut::core::ffi::c_void , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsReplaceString(::core::mem::transmute_copy(string), ::core::mem::transmute_copy(stringreplaced), ::core::mem::transmute_copy(stringreplacewith), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull(string: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsStringHasEmbeddedNull ( string : * mut::core::ffi::c_void , hasembednull : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsStringHasEmbeddedNull(::core::mem::transmute_copy(string), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsSubstring(string: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsSubstring ( string : * mut::core::ffi::c_void , startindex : u32 , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsSubstring(::core::mem::transmute_copy(string), startindex, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength(string: &::windows::core::HSTRING, startindex: u32, length: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsSubstringWithSpecifiedLength ( string : * mut::core::ffi::c_void , startindex : u32 , length : u32 , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsSubstringWithSpecifiedLength(::core::mem::transmute_copy(string), startindex, length, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsTrimStringEnd(string: &::windows::core::HSTRING, trimstring: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsTrimStringEnd ( string : * mut::core::ffi::c_void , trimstring : * mut::core::ffi::c_void , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsTrimStringEnd(::core::mem::transmute_copy(string), ::core::mem::transmute_copy(trimstring), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsTrimStringStart(string: &::windows::core::HSTRING, trimstring: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
    ::windows::core::link ! ( "api-ms-win-core-winrt-string-l1-1-0.dll""system" fn WindowsTrimStringStart ( string : * mut::core::ffi::c_void , trimstring : * mut::core::ffi::c_void , newstring : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsTrimStringStart(::core::mem::transmute_copy(string), ::core::mem::transmute_copy(trimstring), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IAccountsSettingsPaneInterop(::windows::core::IUnknown);
impl IAccountsSettingsPaneInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowManageAccountsForWindowAsync<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ShowManageAccountsForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowAddAccountForWindowAsync<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ShowAddAccountForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAccountsSettingsPaneInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IAccountsSettingsPaneInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccountsSettingsPaneInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccountsSettingsPaneInterop {}
impl ::core::fmt::Debug for IAccountsSettingsPaneInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccountsSettingsPaneInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAccountsSettingsPaneInterop {
    type Vtable = IAccountsSettingsPaneInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3ee12ad_3865_4362_9746_b75a682df0e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowManageAccountsForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowAddAccountForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IActivationFactory(::windows::core::IUnknown);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActivateInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IActivationFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IActivationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFactory {}
impl ::core::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IActivationFactory {
    type Vtable = IActivationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IAgileReference(::windows::core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Resolve)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAgileReference, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAgileReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileReference {}
impl ::core::fmt::Debug for IAgileReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IAgileReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IApartmentShutdown(::windows::core::IUnknown);
impl IApartmentShutdown {
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        (::windows::core::Vtable::vtable(self).OnUninitialize)(::windows::core::Vtable::as_raw(self), ui64apartmentidentifier)
    }
}
::windows::core::interface_hierarchy!(IApartmentShutdown, ::windows::core::IUnknown);
impl ::core::clone::Clone for IApartmentShutdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApartmentShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApartmentShutdown {}
impl ::core::fmt::Debug for IApartmentShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApartmentShutdown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IApartmentShutdown {
    type Vtable = IApartmentShutdown_Vtbl;
}
unsafe impl ::windows::core::Interface for IApartmentShutdown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2f05a09_27a2_42b5_bc0e_ac163ef49d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentShutdown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64),
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IAppServiceConnectionExtendedExecution(::windows::core::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    pub unsafe fn OpenForExtendedExecutionAsync<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenForExtendedExecutionAsync)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppServiceConnectionExtendedExecution, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppServiceConnectionExtendedExecution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppServiceConnectionExtendedExecution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppServiceConnectionExtendedExecution {}
impl ::core::fmt::Debug for IAppServiceConnectionExtendedExecution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppServiceConnectionExtendedExecution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppServiceConnectionExtendedExecution {
    type Vtable = IAppServiceConnectionExtendedExecution_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppServiceConnectionExtendedExecution {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65219584_f9cb_4ae3_81f9_a28a6ca450d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionExtendedExecution_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IBufferByteAccess(::windows::core::IUnknown);
impl IBufferByteAccess {
    pub unsafe fn Buffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Buffer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IBufferByteAccess, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBufferByteAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBufferByteAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBufferByteAccess {}
impl ::core::fmt::Debug for IBufferByteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBufferByteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IBufferByteAccess {
    type Vtable = IBufferByteAccess_Vtbl;
}
unsafe impl ::windows::core::Interface for IBufferByteAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fef_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICastingController(::windows::core::IUnknown);
impl ICastingController {
    pub unsafe fn Initialize<P0, P1>(&self, castingengine: P0, castingsource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), castingengine.into().abi(), castingsource.into().abi()).ok()
    }
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Connect)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Disconnect)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Advise<P0>(&self, eventhandler: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICastingEventHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), eventhandler.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnAdvise(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnAdvise)(::windows::core::Vtable::as_raw(self), cookie).ok()
    }
}
::windows::core::interface_hierarchy!(ICastingController, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICastingController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingController {}
impl ::core::fmt::Debug for ICastingController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICastingController {
    type Vtable = ICastingController_Vtbl;
}
unsafe impl ::windows::core::Interface for ICastingController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a56423_a664_4fbd_8b43_409a45e8d9a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingController_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICastingEventHandler(::windows::core::IUnknown);
impl ICastingEventHandler {
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnStateChanged)(::windows::core::Vtable::as_raw(self), newstate).ok()
    }
    pub unsafe fn OnError<P0>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).OnError)(::windows::core::Vtable::as_raw(self), errorstatus, errormessage.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ICastingEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICastingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingEventHandler {}
impl ::core::fmt::Debug for ICastingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICastingEventHandler {
    type Vtable = ICastingEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for ICastingEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc79a6cb7_bebd_47a6_a2ad_4d45ad79c7bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::HRESULT,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICastingSourceInfo(::windows::core::IUnknown);
impl ICastingSourceInfo {
    pub unsafe fn GetController(&self) -> ::windows::core::Result<ICastingController> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetController)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ICastingSourceInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICastingSourceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingSourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingSourceInfo {}
impl ::core::fmt::Debug for ICastingSourceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingSourceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICastingSourceInfo {
    type Vtable = ICastingSourceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ICastingSourceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45101ab7_7c3a_4bce_9500_12c09024b298);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSourceInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controller: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperties: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreInputInterop(::windows::core::IUnknown);
impl ICoreInputInterop {
    pub unsafe fn SetInputSource<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetInputSource)(::windows::core::Vtable::as_raw(self), value.into().abi()).ok()
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMessageHandled)(::windows::core::Vtable::as_raw(self), value).ok()
    }
}
::windows::core::interface_hierarchy!(ICoreInputInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICoreInputInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreInputInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreInputInterop {}
impl ::core::fmt::Debug for ICoreInputInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreInputInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICoreInputInterop {
    type Vtable = ICoreInputInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreInputInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40bfe3e3_b75a_4479_ac96_475365749bb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreWindowAdapterInterop(::windows::core::IUnknown);
impl ICoreWindowAdapterInterop {
    pub unsafe fn AppActivationClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AppActivationClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ApplicationViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ApplicationViewClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CoreApplicationViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CoreApplicationViewClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HoloViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HoloViewClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PositionerClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PositionerClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SystemNavigationClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SystemNavigationClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TitleBarClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TitleBarClientAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWindowClientAdapter<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetWindowClientAdapter)(::windows::core::Vtable::as_raw(self), value.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ICoreWindowAdapterInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICoreWindowAdapterInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowAdapterInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowAdapterInterop {}
impl ::core::fmt::Debug for ICoreWindowAdapterInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowAdapterInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICoreWindowAdapterInterop {
    type Vtable = ICoreWindowAdapterInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreWindowAdapterInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowAdapterInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppActivationClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CoreApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HoloViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PositionerClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemNavigationClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TitleBarClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreWindowComponentInterop(::windows::core::IUnknown);
impl ICoreWindowComponentInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureComponentInput<P0, P1>(&self, hostviewinstanceid: u32, hwndhost: P0, inputsourcevisual: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ConfigureComponentInput)(::windows::core::Vtable::as_raw(self), hostviewinstanceid, hwndhost.into(), inputsourcevisual.into().abi()).ok()
    }
    pub unsafe fn GetViewInstanceId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewInstanceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ICoreWindowComponentInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICoreWindowComponentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowComponentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowComponentInterop {}
impl ::core::fmt::Debug for ICoreWindowComponentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowComponentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICoreWindowComponentInterop {
    type Vtable = ICoreWindowComponentInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreWindowComponentInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0576ab31_a310_4c40_ba31_fd37e0298dfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowComponentInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureComponentInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureComponentInput: usize,
    pub GetViewInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, componentviewinstanceid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreWindowInterop(::windows::core::IUnknown);
impl ICoreWindowInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMessageHandled)(::windows::core::Vtable::as_raw(self), value).ok()
    }
}
::windows::core::interface_hierarchy!(ICoreWindowInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICoreWindowInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowInterop {}
impl ::core::fmt::Debug for ICoreWindowInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICoreWindowInterop {
    type Vtable = ICoreWindowInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreWindowInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowHandle: usize,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICorrelationVectorInformation(::windows::core::IUnknown);
impl ICorrelationVectorInformation {
    pub unsafe fn LastCorrelationVectorForThread(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastCorrelationVectorForThread)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NextCorrelationVectorForThread(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NextCorrelationVectorForThread)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNextCorrelationVectorForThread(&self, cv: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNextCorrelationVectorForThread)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(cv)).ok()
    }
}
::windows::core::interface_hierarchy!(ICorrelationVectorInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICorrelationVectorInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICorrelationVectorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorrelationVectorInformation {}
impl ::core::fmt::Debug for ICorrelationVectorInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorrelationVectorInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICorrelationVectorInformation {
    type Vtable = ICorrelationVectorInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for ICorrelationVectorInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83c78b3c_d88b_4950_aa6e_22b8d22aabd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICorrelationVectorSource(::windows::core::IUnknown);
impl ICorrelationVectorSource {
    pub unsafe fn CorrelationVector(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CorrelationVector)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ICorrelationVectorSource, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICorrelationVectorSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICorrelationVectorSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorrelationVectorSource {}
impl ::core::fmt::Debug for ICorrelationVectorSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorrelationVectorSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICorrelationVectorSource {
    type Vtable = ICorrelationVectorSource_Vtbl;
}
unsafe impl ::windows::core::Interface for ICorrelationVectorSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x152b8a3b_b9b9_4685_b56e_974847bc7545);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CorrelationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IDragDropManagerInterop(::windows::core::IUnknown);
impl IDragDropManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDragDropManagerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IDragDropManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDragDropManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragDropManagerInterop {}
impl ::core::fmt::Debug for IDragDropManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragDropManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDragDropManagerInterop {
    type Vtable = IDragDropManagerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IDragDropManagerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad8cba7_4c01_4dac_9074_827894292d63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IHolographicSpaceInterop(::windows::core::IUnknown);
impl IHolographicSpaceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateForWindow)(::windows::core::Vtable::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IHolographicSpaceInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IHolographicSpaceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicSpaceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicSpaceInterop {}
impl ::core::fmt::Debug for IHolographicSpaceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicSpaceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IHolographicSpaceInterop {
    type Vtable = IHolographicSpaceInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IHolographicSpaceInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IInputPaneInterop(::windows::core::IUnknown);
impl IInputPaneInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IInputPaneInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInputPaneInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInputPaneInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPaneInterop {}
impl ::core::fmt::Debug for IInputPaneInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPaneInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IInputPaneInterop {
    type Vtable = IInputPaneInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputPaneInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cf2c57_9195_4931_8332_f0b409e916af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(::windows::core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLanguageException)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ILanguageExceptionErrorInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ILanguageExceptionErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageExceptionErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(::windows::core::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguageException)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousLanguageExceptionErrorInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).CapturePropagationContext)(::windows::core::Vtable::as_raw(self), languageexception.into().abi()).ok()
    }
    pub unsafe fn GetPropagationContextHead(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropagationContextHead)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ILanguageExceptionErrorInfo2, ::windows::core::IUnknown, ILanguageExceptionErrorInfo);
impl ::core::clone::Clone for ILanguageExceptionErrorInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo2 {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageExceptionErrorInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5746e5c4_5b97_424c_b620_2822915734dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionStackBackTrace(::windows::core::IUnknown);
impl ILanguageExceptionStackBackTrace {
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStackBackTrace)(::windows::core::Vtable::as_raw(self), maxframestocapture, stackbacktrace, framescaptured).ok()
    }
}
::windows::core::interface_hierarchy!(ILanguageExceptionStackBackTrace, ::windows::core::IUnknown);
impl ::core::clone::Clone for ILanguageExceptionStackBackTrace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionStackBackTrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionStackBackTrace {}
impl ::core::fmt::Debug for ILanguageExceptionStackBackTrace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionStackBackTrace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ILanguageExceptionStackBackTrace {
    type Vtable = ILanguageExceptionStackBackTrace_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageExceptionStackBackTrace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbe53fb5_f967_4258_8d34_42f5e25833de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionStackBackTrace_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStackBackTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionTransform(::windows::core::IUnknown);
impl ILanguageExceptionTransform {
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> ::windows::core::Result<IRestrictedErrorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformedRestrictedErrorInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ILanguageExceptionTransform, ::windows::core::IUnknown);
impl ::core::clone::Clone for ILanguageExceptionTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionTransform {}
impl ::core::fmt::Debug for ILanguageExceptionTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ILanguageExceptionTransform {
    type Vtable = ILanguageExceptionTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageExceptionTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeb5a271_a6cd_45ce_880a_696706badc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionTransform_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IMemoryBufferByteAccess(::windows::core::IUnknown);
impl IMemoryBufferByteAccess {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBuffer)(::windows::core::Vtable::as_raw(self), value, capacity).ok()
    }
}
::windows::core::interface_hierarchy!(IMemoryBufferByteAccess, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMemoryBufferByteAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMemoryBufferByteAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBufferByteAccess {}
impl ::core::fmt::Debug for IMemoryBufferByteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBufferByteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMemoryBufferByteAccess {
    type Vtable = IMemoryBufferByteAccess_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryBufferByteAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferByteAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IMessageDispatcher(::windows::core::IUnknown);
impl IMessageDispatcher {
    pub unsafe fn PumpMessages(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PumpMessages)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IMessageDispatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IMessageDispatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMessageDispatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageDispatcher {}
impl ::core::fmt::Debug for IMessageDispatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageDispatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IMessageDispatcher {
    type Vtable = IMessageDispatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IMessageDispatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f84c8f_cfd0_4cd6_b66b_c5d26ff1689d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDispatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PumpMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IPlayToManagerInterop(::windows::core::IUnknown);
impl IPlayToManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPlayToUIForWindow<P0>(&self, appwindow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).ShowPlayToUIForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IPlayToManagerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPlayToManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayToManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayToManagerInterop {}
impl ::core::fmt::Debug for IPlayToManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayToManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPlayToManagerInterop {
    type Vtable = IPlayToManagerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayToManagerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24394699_1f2c_4eb3_8cd7_0ec1da42a540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPlayToUIForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IRestrictedErrorInfo(::windows::core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: *mut ::windows::core::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut ::windows::core::BSTR, capabilitysid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetErrorDetails)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(description), error, ::core::mem::transmute(restricteddescription), ::core::mem::transmute(capabilitysid)).ok()
    }
    pub unsafe fn GetReference(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRestrictedErrorInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRestrictedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedErrorInfo {}
impl ::core::fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for IRestrictedErrorInfo {}
unsafe impl ::core::marker::Sync for IRestrictedErrorInfo {}
unsafe impl ::windows::core::Vtable for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IRestrictedErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut *mut ::core::ffi::c_void, error: *mut ::windows::core::HRESULT, restricteddescription: *mut *mut ::core::ffi::c_void, capabilitysid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IRoMetaDataLocator(::std::ptr::NonNull<::std::ffi::c_void>);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<P0, P1>(&self, nameelement: P0, metadatadestination: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IRoSimpleMetaDataBuilder>>,
    {
        (::windows::core::Vtable::vtable(self).Locate)(::windows::core::Vtable::as_raw(self), nameelement.into().abi(), metadatadestination.into().abi()).ok()
    }
}
impl ::core::clone::Clone for IRoMetaDataLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRoMetaDataLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoMetaDataLocator {}
impl ::core::fmt::Debug for IRoMetaDataLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoMetaDataLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRoMetaDataLocator {
    type Vtable = IRoMetaDataLocator_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nameelement: ::windows::core::PCWSTR, metadatadestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IRoSimpleMetaDataBuilder(::std::ptr::NonNull<::std::ffi::c_void>);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetWinRtInterface)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetDelegate(&self, iid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDelegate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetInterfaceGroupSimpleDefault)(::windows::core::Vtable::as_raw(self), name.into().abi(), defaultinterfacename.into().abi(), ::core::mem::transmute(defaultinterfaceiid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetInterfaceGroupParameterizedDefault)(::windows::core::Vtable::as_raw(self), name.into().abi(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(defaultinterfacenameelements.as_ptr())).ok()
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetRuntimeClassSimpleDefault)(::windows::core::Vtable::as_raw(self), name.into().abi(), defaultinterfacename.into().abi(), ::core::mem::transmute(defaultinterfaceiid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetRuntimeClassParameterizedDefault)(::windows::core::Vtable::as_raw(self), name.into().abi(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(defaultinterfacenameelements.as_ptr())).ok()
    }
    pub unsafe fn SetStruct<P0>(&self, name: P0, fieldtypenames: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetStruct)(::windows::core::Vtable::as_raw(self), name.into().abi(), fieldtypenames.len() as _, ::core::mem::transmute(fieldtypenames.as_ptr())).ok()
    }
    pub unsafe fn SetEnum<P0, P1>(&self, name: P0, basetype: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetEnum)(::windows::core::Vtable::as_raw(self), name.into().abi(), basetype.into().abi()).ok()
    }
    pub unsafe fn SetParameterizedInterface(&self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameterizedInterface)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(piid), numargs).ok()
    }
    pub unsafe fn SetParameterizedDelegate(&self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameterizedDelegate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(piid), numargs).ok()
    }
}
impl ::core::clone::Clone for IRoSimpleMetaDataBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRoSimpleMetaDataBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoSimpleMetaDataBuilder {}
impl ::core::fmt::Debug for IRoSimpleMetaDataBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoSimpleMetaDataBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRoSimpleMetaDataBuilder {
    type Vtable = IRoSimpleMetaDataBuilder_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, defaultinterfacename: ::windows::core::PCWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, defaultinterfacename: ::windows::core::PCWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, numfields: u32, fieldtypenames: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, basetype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IShareWindowCommandEventArgsInterop(::windows::core::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IShareWindowCommandEventArgsInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for IShareWindowCommandEventArgsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareWindowCommandEventArgsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareWindowCommandEventArgsInterop {}
impl ::core::fmt::Debug for IShareWindowCommandEventArgsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareWindowCommandEventArgsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IShareWindowCommandEventArgsInterop {
    type Vtable = IShareWindowCommandEventArgsInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IShareWindowCommandEventArgsInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6571a721_643d_43d4_aca4_6b6f5f30f1ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IShareWindowCommandSourceInterop(::windows::core::IUnknown);
impl IShareWindowCommandSourceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IShareWindowCommandSourceInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for IShareWindowCommandSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareWindowCommandSourceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareWindowCommandSourceInterop {}
impl ::core::fmt::Debug for IShareWindowCommandSourceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareWindowCommandSourceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IShareWindowCommandSourceInterop {
    type Vtable = IShareWindowCommandSourceInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IShareWindowCommandSourceInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x461a191f_8424_43a6_a0fa_3451a22f56ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ISpatialInteractionManagerInterop(::windows::core::IUnknown);
impl ISpatialInteractionManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISpatialInteractionManagerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ISpatialInteractionManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialInteractionManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialInteractionManagerInterop {}
impl ::core::fmt::Debug for ISpatialInteractionManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialInteractionManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISpatialInteractionManagerInterop {
    type Vtable = ISpatialInteractionManagerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialInteractionManagerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsInterop(::windows::core::IUnknown);
impl ISystemMediaTransportControlsInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISystemMediaTransportControlsInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ISystemMediaTransportControlsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMediaTransportControlsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMediaTransportControlsInterop {}
impl ::core::fmt::Debug for ISystemMediaTransportControlsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMediaTransportControlsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISystemMediaTransportControlsInterop {
    type Vtable = ISystemMediaTransportControlsInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddb0472d_c911_4a1f_86d9_dc3d71a95f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUIViewSettingsInterop(::windows::core::IUnknown);
impl IUIViewSettingsInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIViewSettingsInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IUIViewSettingsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIViewSettingsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIViewSettingsInterop {}
impl ::core::fmt::Debug for IUIViewSettingsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIViewSettingsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUIViewSettingsInterop {
    type Vtable = IUIViewSettingsInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIViewSettingsInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3694dbf9_8f68_44be_8ff5_195c98ede8a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettingsInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserActivityInterop(::windows::core::IUnknown);
impl IUserActivityInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSessionForWindow<P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSessionForWindow)(::windows::core::Vtable::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUserActivityInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IUserActivityInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityInterop {}
impl ::core::fmt::Debug for IUserActivityInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUserActivityInterop {
    type Vtable = IUserActivityInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ade314d_0e0a_40d9_824c_9a088a50059f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSessionForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSessionForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserActivityRequestManagerInterop(::windows::core::IUnknown);
impl IUserActivityRequestManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUserActivityRequestManagerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IUserActivityRequestManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityRequestManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityRequestManagerInterop {}
impl ::core::fmt::Debug for IUserActivityRequestManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityRequestManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUserActivityRequestManagerInterop {
    type Vtable = IUserActivityRequestManagerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivityRequestManagerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd69f876_9699_4715_9095_e37ea30dfa1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserActivitySourceHostInterop(::windows::core::IUnknown);
impl IUserActivitySourceHostInterop {
    pub unsafe fn SetActivitySourceHost(&self, activitysourcehost: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetActivitySourceHost)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(activitysourcehost)).ok()
    }
}
::windows::core::interface_hierarchy!(IUserActivitySourceHostInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IUserActivitySourceHostInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivitySourceHostInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivitySourceHostInterop {}
impl ::core::fmt::Debug for IUserActivitySourceHostInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivitySourceHostInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUserActivitySourceHostInterop {
    type Vtable = IUserActivitySourceHostInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserActivitySourceHostInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc15df8bc_8844_487a_b85b_7578e0f61419);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySourceHostInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetActivitySourceHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitysourcehost: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserConsentVerifierInterop(::windows::core::IUnknown);
impl IUserConsentVerifierInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestVerificationForWindowAsync<P0, T>(&self, appwindow: P0, message: &::windows::core::HSTRING) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestVerificationForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(message), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUserConsentVerifierInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IUserConsentVerifierInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserConsentVerifierInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserConsentVerifierInterop {}
impl ::core::fmt::Debug for IUserConsentVerifierInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserConsentVerifierInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUserConsentVerifierInterop {
    type Vtable = IUserConsentVerifierInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserConsentVerifierInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e050c3_4e74_441a_8dc0_b81104df949c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestVerificationForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IWeakReference(::windows::core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Resolve)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWeakReference, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWeakReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReference {}
impl ::core::fmt::Debug for IWeakReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IWeakReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IWeakReferenceSource(::windows::core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> ::windows::core::Result<IWeakReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWeakReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWeakReferenceSource, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWeakReferenceSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReferenceSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReferenceSource {}
impl ::core::fmt::Debug for IWeakReferenceSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReferenceSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IWeakReferenceSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weakreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerInterop(::windows::core::IUnknown);
impl IWebAuthenticationCoreManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestTokenForWindowAsync<P0, P1, T>(&self, appwindow: P0, request: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestTokenForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), request.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<P0, P1, P2, T>(&self, appwindow: P0, request: P1, webaccount: P2) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestTokenWithWebAccountForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), request.into().abi(), webaccount.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWebAuthenticationCoreManagerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IWebAuthenticationCoreManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAuthenticationCoreManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationCoreManagerInterop {}
impl ::core::fmt::Debug for IWebAuthenticationCoreManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAuthenticationCoreManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWebAuthenticationCoreManagerInterop {
    type Vtable = IWebAuthenticationCoreManagerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4b8e804_811e_4436_b69c_44cb67b72084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestTokenForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestTokenWithWebAccountForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_CastingTypes: ::windows::core::PCWSTR = ::windows::w!("CastingTypes");
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: ::windows::core::PCWSTR = ::windows::w!("PreferredSourceUriScheme");
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_ProtectedMedia: ::windows::core::PCWSTR = ::windows::w!("ProtectedMedia");
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTIVATIONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
impl ::core::marker::Copy for ACTIVATIONTYPE {}
impl ::core::clone::Clone for ACTIVATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTIVATIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATIONTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AgileReferenceOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
impl ::core::marker::Copy for AgileReferenceOptions {}
impl ::core::clone::Clone for AgileReferenceOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AgileReferenceOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AgileReferenceOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AgileReferenceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AgileReferenceOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BSOS_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
impl ::core::marker::Copy for BSOS_OPTIONS {}
impl ::core::clone::Clone for BSOS_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BSOS_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BSOS_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BSOS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BSOS_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
impl ::core::marker::Copy for CASTING_CONNECTION_ERROR_STATUS {}
impl ::core::clone::Clone for CASTING_CONNECTION_ERROR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASTING_CONNECTION_ERROR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CASTING_CONNECTION_ERROR_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASTING_CONNECTION_ERROR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_ERROR_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CASTING_CONNECTION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
impl ::core::marker::Copy for CASTING_CONNECTION_STATE {}
impl ::core::clone::Clone for CASTING_CONNECTION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASTING_CONNECTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CASTING_CONNECTION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASTING_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
impl ::core::marker::Copy for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {}
impl ::core::clone::Clone for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_APARTMENTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
impl ::core::marker::Copy for DISPATCHERQUEUE_THREAD_TYPE {}
impl ::core::clone::Clone for DISPATCHERQUEUE_THREAD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCHERQUEUE_THREAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISPATCHERQUEUE_THREAD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCHERQUEUE_THREAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RO_ERROR_REPORTING_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8u32);
impl ::core::marker::Copy for RO_ERROR_REPORTING_FLAGS {}
impl ::core::clone::Clone for RO_ERROR_REPORTING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RO_ERROR_REPORTING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RO_ERROR_REPORTING_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RO_ERROR_REPORTING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RO_ERROR_REPORTING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RO_INIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
impl ::core::marker::Copy for RO_INIT_TYPE {}
impl ::core::clone::Clone for RO_INIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RO_INIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RO_INIT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RO_INIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RO_INIT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TrustLevel(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const FullTrust: TrustLevel = TrustLevel(2i32);
impl ::core::marker::Copy for TrustLevel {}
impl ::core::clone::Clone for TrustLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TrustLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TrustLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for TrustLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrustLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub isize);
impl APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {}
impl ::core::fmt::Debug for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APARTMENT_SHUTDOWN_REGISTRATION_COOKIE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>> for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn from(optional: ::core::option::Option<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>) -> APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
impl ::core::marker::Copy for DispatcherQueueOptions {}
impl ::core::clone::Clone for DispatcherQueueOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DispatcherQueueOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DispatcherQueueOptions").field("dwSize", &self.dwSize).field("threadType", &self.threadType).field("apartmentType", &self.apartmentType).finish()
    }
}
unsafe impl ::windows::core::Abi for DispatcherQueueOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DispatcherQueueOptions {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.threadType == other.threadType && self.apartmentType == other.apartmentType
    }
}
impl ::core::cmp::Eq for DispatcherQueueOptions {}
impl ::core::default::Default for DispatcherQueueOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EventRegistrationToken").field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for EventRegistrationToken {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::core::cmp::Eq for EventRegistrationToken {}
impl ::core::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HSTRING_BUFFER(pub isize);
impl HSTRING_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSTRING_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSTRING_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSTRING_BUFFER {}
impl ::core::fmt::Debug for HSTRING_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSTRING_BUFFER").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HSTRING_BUFFER>> for HSTRING_BUFFER {
    fn from(optional: ::core::option::Option<HSTRING_BUFFER>) -> HSTRING_BUFFER {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HSTRING_BUFFER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct HSTRING_HEADER {
    pub flags: u32,
    pub length: u32,
    pub padding1: u32,
    pub padding2: u32,
    pub data: isize,
}
impl ::core::marker::Copy for HSTRING_HEADER {}
impl ::core::clone::Clone for HSTRING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSTRING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSTRING_HEADER").field("flags", &self.flags).field("length", &self.length).field("padding1", &self.padding1).field("padding2", &self.padding2).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for HSTRING_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSTRING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.length == other.length && self.padding1 == other.padding1 && self.padding2 == other.padding2 && self.data == other.data
    }
}
impl ::core::cmp::Eq for HSTRING_HEADER {}
impl ::core::default::Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ROPARAMIIDHANDLE(pub isize);
impl ROPARAMIIDHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for ROPARAMIIDHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ROPARAMIIDHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ROPARAMIIDHANDLE {}
impl ::core::fmt::Debug for ROPARAMIIDHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROPARAMIIDHANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<ROPARAMIIDHANDLE>> for ROPARAMIIDHANDLE {
    fn from(optional: ::core::option::Option<ROPARAMIIDHANDLE>) -> ROPARAMIIDHANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for ROPARAMIIDHANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
impl ::core::marker::Copy for ServerInformation {}
impl ::core::clone::Clone for ServerInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ServerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ServerInformation").field("dwServerPid", &self.dwServerPid).field("dwServerTid", &self.dwServerTid).field("ui64ServerAddress", &self.ui64ServerAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for ServerInformation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ServerInformation {
    fn eq(&self, other: &Self) -> bool {
        self.dwServerPid == other.dwServerPid && self.dwServerTid == other.dwServerTid && self.ui64ServerAddress == other.ui64ServerAddress
    }
}
impl ::core::cmp::Eq for ServerInformation {}
impl ::core::default::Default for ServerInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_HSTRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_HSTRING_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
