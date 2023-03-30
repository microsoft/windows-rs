#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNAuthenticatorGetAssertion<P0, P1>(hwnd: P0, pwszrpid: P1, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: ::core::option::Option<*const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS>) -> ::windows::core::Result<*mut WEBAUTHN_ASSERTION>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNAuthenticatorGetAssertion ( hwnd : super::super::Foundation:: HWND , pwszrpid : ::windows::core::PCWSTR , pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA , pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS , ppwebauthnassertion : *mut *mut WEBAUTHN_ASSERTION ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut WEBAUTHN_ASSERTION>();
    WebAuthNAuthenticatorGetAssertion(hwnd.into_param().abi(), pwszrpid.into_param().abi(), pwebauthnclientdata, ::core::mem::transmute(pwebauthngetassertionoptions.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNAuthenticatorMakeCredential<P0>(hwnd: P0, prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions: ::core::option::Option<*const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS>) -> ::windows::core::Result<*mut WEBAUTHN_CREDENTIAL_ATTESTATION>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNAuthenticatorMakeCredential ( hwnd : super::super::Foundation:: HWND , prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION , puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION , ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS , pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA , pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS , ppwebauthncredentialattestation : *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut WEBAUTHN_CREDENTIAL_ATTESTATION>();
    WebAuthNAuthenticatorMakeCredential(hwnd.into_param().abi(), prpinformation, puserinformation, ppubkeycredparams, pwebauthnclientdata, ::core::mem::transmute(pwebauthnmakecredentialoptions.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNCancelCurrentOperation(pcancellationid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNCancelCurrentOperation ( pcancellationid : *const ::windows::core::GUID ) -> ::windows::core::HRESULT );
    WebAuthNCancelCurrentOperation(pcancellationid).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION) {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNFreeAssertion ( pwebauthnassertion : *const WEBAUTHN_ASSERTION ) -> ( ) );
    WebAuthNFreeAssertion(pwebauthnassertion)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: ::core::option::Option<*const WEBAUTHN_CREDENTIAL_ATTESTATION>) {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNFreeCredentialAttestation ( pwebauthncredentialattestation : *const WEBAUTHN_CREDENTIAL_ATTESTATION ) -> ( ) );
    WebAuthNFreeCredentialAttestation(::core::mem::transmute(pwebauthncredentialattestation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetApiVersionNumber() -> u32 {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNGetApiVersionNumber ( ) -> u32 );
    WebAuthNGetApiVersionNumber()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetCancellationId() -> ::windows::core::Result<::windows::core::GUID> {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNGetCancellationId ( pcancellationid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    WebAuthNGetCancellationId(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetErrorName(hr: ::windows::core::HRESULT) -> ::windows::core::PCWSTR {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNGetErrorName ( hr : ::windows::core::HRESULT ) -> ::windows::core::PCWSTR );
    WebAuthNGetErrorName(hr)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WebAuthNGetW3CExceptionDOMError(hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNGetW3CExceptionDOMError ( hr : ::windows::core::HRESULT ) -> ::windows::core::HRESULT );
    WebAuthNGetW3CExceptionDOMError(hr).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "webauthn.dll""system" fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable ( pbisuserverifyingplatformauthenticatoravailable : *mut super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
    WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbandonCall(serviceproxy: *const WS_SERVICE_PROXY, callid: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAbandonCall ( serviceproxy : *const WS_SERVICE_PROXY , callid : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAbandonCall(serviceproxy, callid, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbandonMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAbandonMessage ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAbandonMessage(channel, message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortChannel(channel: *const WS_CHANNEL, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAbortChannel ( channel : *const WS_CHANNEL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAbortChannel(channel, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortListener(listener: *const WS_LISTENER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAbortListener ( listener : *const WS_LISTENER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAbortListener(listener, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortServiceHost(servicehost: *const WS_SERVICE_HOST, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAbortServiceHost ( servicehost : *const WS_SERVICE_HOST , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAbortServiceHost(servicehost, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAbortServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAbortServiceProxy ( serviceproxy : *const WS_SERVICE_PROXY , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAbortServiceProxy(serviceproxy, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAcceptChannel(listener: *const WS_LISTENER, channel: *const WS_CHANNEL, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAcceptChannel ( listener : *const WS_LISTENER , channel : *const WS_CHANNEL , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAcceptChannel(listener, channel, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsAddCustomHeader(message: *const WS_MESSAGE, headerdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, headerattributes: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAddCustomHeader ( message : *const WS_MESSAGE , headerdescription : *const WS_ELEMENT_DESCRIPTION , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , headerattributes : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAddCustomHeader(message, headerdescription, writeoption, value, valuesize, headerattributes, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAddErrorString(error: *const WS_ERROR, string: *const WS_STRING) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAddErrorString ( error : *const WS_ERROR , string : *const WS_STRING ) -> ::windows::core::HRESULT );
    WsAddErrorString(error, string).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsAddMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAddMappedHeader ( message : *const WS_MESSAGE , headername : *const WS_XML_STRING , valuetype : WS_TYPE , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAddMappedHeader(message, headername, valuetype, writeoption, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAddressMessage(message: *const WS_MESSAGE, address: ::core::option::Option<*const WS_ENDPOINT_ADDRESS>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAddressMessage ( message : *const WS_MESSAGE , address : *const WS_ENDPOINT_ADDRESS , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAddressMessage(message, ::core::mem::transmute(address.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAlloc(heap: *const WS_HEAP, size: usize, ptr: *mut *mut ::core::ffi::c_void, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAlloc ( heap : *const WS_HEAP , size : usize , ptr : *mut *mut ::core::ffi::c_void , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAlloc(heap, size, ptr, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsAsyncExecute(asyncstate: *const WS_ASYNC_STATE, operation: WS_ASYNC_FUNCTION, callbackmodel: WS_CALLBACK_MODEL, callbackstate: ::core::option::Option<*const ::core::ffi::c_void>, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsAsyncExecute ( asyncstate : *const WS_ASYNC_STATE , operation : WS_ASYNC_FUNCTION , callbackmodel : WS_CALLBACK_MODEL , callbackstate : *const ::core::ffi::c_void , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsAsyncExecute(asyncstate, operation, callbackmodel, ::core::mem::transmute(callbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCall(serviceproxy: *const WS_SERVICE_PROXY, operation: *const WS_OPERATION_DESCRIPTION, arguments: ::core::option::Option<*const *const ::core::ffi::c_void>, heap: *const WS_HEAP, callproperties: ::core::option::Option<&[WS_CALL_PROPERTY]>, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCall ( serviceproxy : *const WS_SERVICE_PROXY , operation : *const WS_OPERATION_DESCRIPTION , arguments : *const *const ::core::ffi::c_void , heap : *const WS_HEAP , callproperties : *const WS_CALL_PROPERTY , callpropertycount : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCall(serviceproxy, operation, ::core::mem::transmute(arguments.unwrap_or(::std::ptr::null())), heap, ::core::mem::transmute(callproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), callproperties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCheckMustUnderstandHeaders(message: *const WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCheckMustUnderstandHeaders ( message : *const WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCheckMustUnderstandHeaders(message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseChannel(channel: *const WS_CHANNEL, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCloseChannel ( channel : *const WS_CHANNEL , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCloseChannel(channel, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseListener(listener: *const WS_LISTENER, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCloseListener ( listener : *const WS_LISTENER , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCloseListener(listener, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCloseServiceHost ( servicehost : *const WS_SERVICE_HOST , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCloseServiceHost(servicehost, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCloseServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCloseServiceProxy ( serviceproxy : *const WS_SERVICE_PROXY , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCloseServiceProxy(serviceproxy, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCombineUrl(baseurl: *const WS_STRING, referenceurl: *const WS_STRING, flags: u32, heap: *const WS_HEAP, resulturl: *mut WS_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCombineUrl ( baseurl : *const WS_STRING , referenceurl : *const WS_STRING , flags : u32 , heap : *const WS_HEAP , resulturl : *mut WS_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCombineUrl(baseurl, referenceurl, flags, heap, resulturl, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCopyError(source: *const WS_ERROR, destination: *const WS_ERROR) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCopyError ( source : *const WS_ERROR , destination : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCopyError(source, destination).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCopyNode(writer: *const WS_XML_WRITER, reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCopyNode ( writer : *const WS_XML_WRITER , reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCopyNode(writer, reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateChannel(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: ::core::option::Option<&[WS_CHANNEL_PROPERTY]>, securitydescription: ::core::option::Option<*const WS_SECURITY_DESCRIPTION>, channel: *mut *mut WS_CHANNEL, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateChannel ( channeltype : WS_CHANNEL_TYPE , channelbinding : WS_CHANNEL_BINDING , properties : *const WS_CHANNEL_PROPERTY , propertycount : u32 , securitydescription : *const WS_SECURITY_DESCRIPTION , channel : *mut *mut WS_CHANNEL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateChannel(channeltype, channelbinding, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(securitydescription.unwrap_or(::std::ptr::null())), channel, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateChannelForListener(listener: *const WS_LISTENER, properties: ::core::option::Option<&[WS_CHANNEL_PROPERTY]>, channel: *mut *mut WS_CHANNEL, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateChannelForListener ( listener : *const WS_LISTENER , properties : *const WS_CHANNEL_PROPERTY , propertycount : u32 , channel : *mut *mut WS_CHANNEL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateChannelForListener(listener, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), channel, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateError(properties: ::core::option::Option<&[WS_ERROR_PROPERTY]>) -> ::windows::core::Result<*mut WS_ERROR> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateError ( properties : *const WS_ERROR_PROPERTY , propertycount : u32 , error : *mut *mut WS_ERROR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut WS_ERROR>();
    WsCreateError(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCreateFaultFromError(error: *const WS_ERROR, faulterrorcode: ::windows::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, heap: *const WS_HEAP, fault: *mut WS_FAULT) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateFaultFromError ( error : *const WS_ERROR , faulterrorcode : ::windows::core::HRESULT , faultdisclosure : WS_FAULT_DISCLOSURE , heap : *const WS_HEAP , fault : *mut WS_FAULT ) -> ::windows::core::HRESULT );
    WsCreateFaultFromError(error, faulterrorcode, faultdisclosure, heap, fault).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateHeap(maxsize: usize, trimsize: usize, properties: ::core::option::Option<*const WS_HEAP_PROPERTY>, propertycount: u32, heap: *mut *mut WS_HEAP, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateHeap ( maxsize : usize , trimsize : usize , properties : *const WS_HEAP_PROPERTY , propertycount : u32 , heap : *mut *mut WS_HEAP , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateHeap(maxsize, trimsize, ::core::mem::transmute(properties.unwrap_or(::std::ptr::null())), propertycount, heap, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateListener(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: ::core::option::Option<&[WS_LISTENER_PROPERTY]>, securitydescription: ::core::option::Option<*const WS_SECURITY_DESCRIPTION>, listener: *mut *mut WS_LISTENER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateListener ( channeltype : WS_CHANNEL_TYPE , channelbinding : WS_CHANNEL_BINDING , properties : *const WS_LISTENER_PROPERTY , propertycount : u32 , securitydescription : *const WS_SECURITY_DESCRIPTION , listener : *mut *mut WS_LISTENER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateListener(channeltype, channelbinding, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(securitydescription.unwrap_or(::std::ptr::null())), listener, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateMessage(envelopeversion: WS_ENVELOPE_VERSION, addressingversion: WS_ADDRESSING_VERSION, properties: ::core::option::Option<&[WS_MESSAGE_PROPERTY]>, message: *mut *mut WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateMessage ( envelopeversion : WS_ENVELOPE_VERSION , addressingversion : WS_ADDRESSING_VERSION , properties : *const WS_MESSAGE_PROPERTY , propertycount : u32 , message : *mut *mut WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateMessage(envelopeversion, addressingversion, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateMessageForChannel(channel: *const WS_CHANNEL, properties: ::core::option::Option<&[WS_MESSAGE_PROPERTY]>, message: *mut *mut WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateMessageForChannel ( channel : *const WS_CHANNEL , properties : *const WS_MESSAGE_PROPERTY , propertycount : u32 , message : *mut *mut WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateMessageForChannel(channel, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateMetadata(properties: ::core::option::Option<&[WS_METADATA_PROPERTY]>, metadata: *mut *mut WS_METADATA, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateMetadata ( properties : *const WS_METADATA_PROPERTY , propertycount : u32 , metadata : *mut *mut WS_METADATA , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateMetadata(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), metadata, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateReader(properties: ::core::option::Option<&[WS_XML_READER_PROPERTY]>, reader: *mut *mut WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateReader ( properties : *const WS_XML_READER_PROPERTY , propertycount : u32 , reader : *mut *mut WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateReader(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCreateServiceEndpointFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: ::core::option::Option<&[WS_SERVICE_ENDPOINT_PROPERTY]>, addressurl: ::core::option::Option<*const WS_STRING>, contract: *const WS_SERVICE_CONTRACT, authorizationcallback: WS_SERVICE_SECURITY_CALLBACK, heap: *const WS_HEAP, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: ::core::option::Option<*const ::core::ffi::c_void>, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceendpoint: *mut *mut WS_SERVICE_ENDPOINT, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateServiceEndpointFromTemplate ( channeltype : WS_CHANNEL_TYPE , properties : *const WS_SERVICE_ENDPOINT_PROPERTY , propertycount : u32 , addressurl : *const WS_STRING , contract : *const WS_SERVICE_CONTRACT , authorizationcallback : WS_SERVICE_SECURITY_CALLBACK , heap : *const WS_HEAP , templatetype : WS_BINDING_TEMPLATE_TYPE , templatevalue : *const ::core::ffi::c_void , templatesize : u32 , templatedescription : *const ::core::ffi::c_void , templatedescriptionsize : u32 , serviceendpoint : *mut *mut WS_SERVICE_ENDPOINT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateServiceEndpointFromTemplate(
        channeltype,
        ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        properties.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(addressurl.unwrap_or(::std::ptr::null())),
        contract,
        authorizationcallback,
        heap,
        templatetype,
        ::core::mem::transmute(templatevalue.unwrap_or(::std::ptr::null())),
        templatesize,
        templatedescription,
        templatedescriptionsize,
        serviceendpoint,
        ::core::mem::transmute(error.unwrap_or(::std::ptr::null())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsCreateServiceHost(endpoints: ::core::option::Option<&[*const WS_SERVICE_ENDPOINT]>, serviceproperties: ::core::option::Option<&[WS_SERVICE_PROPERTY]>, servicehost: *mut *mut WS_SERVICE_HOST, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateServiceHost ( endpoints : *const *const WS_SERVICE_ENDPOINT , endpointcount : u16 , serviceproperties : *const WS_SERVICE_PROPERTY , servicepropertycount : u32 , servicehost : *mut *mut WS_SERVICE_HOST , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateServiceHost(::core::mem::transmute(endpoints.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), endpoints.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(serviceproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), serviceproperties.as_deref().map_or(0, |slice| slice.len() as _), servicehost, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateServiceProxy(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, securitydescription: ::core::option::Option<*const WS_SECURITY_DESCRIPTION>, properties: ::core::option::Option<&[WS_PROXY_PROPERTY]>, channelproperties: ::core::option::Option<&[WS_CHANNEL_PROPERTY]>, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateServiceProxy ( channeltype : WS_CHANNEL_TYPE , channelbinding : WS_CHANNEL_BINDING , securitydescription : *const WS_SECURITY_DESCRIPTION , properties : *const WS_PROXY_PROPERTY , propertycount : u32 , channelproperties : *const WS_CHANNEL_PROPERTY , channelpropertycount : u32 , serviceproxy : *mut *mut WS_SERVICE_PROXY , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateServiceProxy(
        channeltype,
        channelbinding,
        ::core::mem::transmute(securitydescription.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        properties.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(channelproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        channelproperties.as_deref().map_or(0, |slice| slice.len() as _),
        serviceproxy,
        ::core::mem::transmute(error.unwrap_or(::std::ptr::null())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateServiceProxyFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: ::core::option::Option<&[WS_PROXY_PROPERTY]>, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: ::core::option::Option<*const ::core::ffi::c_void>, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateServiceProxyFromTemplate ( channeltype : WS_CHANNEL_TYPE , properties : *const WS_PROXY_PROPERTY , propertycount : u32 , templatetype : WS_BINDING_TEMPLATE_TYPE , templatevalue : *const ::core::ffi::c_void , templatesize : u32 , templatedescription : *const ::core::ffi::c_void , templatedescriptionsize : u32 , serviceproxy : *mut *mut WS_SERVICE_PROXY , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateServiceProxyFromTemplate(channeltype, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), templatetype, ::core::mem::transmute(templatevalue.unwrap_or(::std::ptr::null())), templatesize, templatedescription, templatedescriptionsize, serviceproxy, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateWriter(properties: ::core::option::Option<&[WS_XML_WRITER_PROPERTY]>, writer: *mut *mut WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateWriter ( properties : *const WS_XML_WRITER_PROPERTY , propertycount : u32 , writer : *mut *mut WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateWriter(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateXmlBuffer(heap: *const WS_HEAP, properties: ::core::option::Option<&[WS_XML_BUFFER_PROPERTY]>, buffer: *mut *mut WS_XML_BUFFER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateXmlBuffer ( heap : *const WS_HEAP , properties : *const WS_XML_BUFFER_PROPERTY , propertycount : u32 , buffer : *mut *mut WS_XML_BUFFER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateXmlBuffer(heap, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), buffer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsCreateXmlSecurityToken(tokenxml: ::core::option::Option<*const WS_XML_BUFFER>, tokenkey: ::core::option::Option<*const WS_SECURITY_KEY_HANDLE>, properties: ::core::option::Option<&[WS_XML_SECURITY_TOKEN_PROPERTY]>, token: *mut *mut WS_SECURITY_TOKEN, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsCreateXmlSecurityToken ( tokenxml : *const WS_XML_BUFFER , tokenkey : *const WS_SECURITY_KEY_HANDLE , properties : *const WS_XML_SECURITY_TOKEN_PROPERTY , propertycount : u32 , token : *mut *mut WS_SECURITY_TOKEN , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsCreateXmlSecurityToken(::core::mem::transmute(tokenxml.unwrap_or(::std::ptr::null())), ::core::mem::transmute(tokenkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), token, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsDateTimeToFileTime(datetime: *const WS_DATETIME, filetime: *mut super::super::Foundation::FILETIME, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsDateTimeToFileTime ( datetime : *const WS_DATETIME , filetime : *mut super::super::Foundation:: FILETIME , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsDateTimeToFileTime(datetime, filetime, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsDecodeUrl(url: *const WS_STRING, flags: u32, heap: *const WS_HEAP, outurl: *mut *mut WS_URL, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsDecodeUrl ( url : *const WS_STRING , flags : u32 , heap : *const WS_HEAP , outurl : *mut *mut WS_URL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsDecodeUrl(url, flags, heap, outurl, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsEncodeUrl(url: *const WS_URL, flags: u32, heap: *const WS_HEAP, outurl: *mut WS_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsEncodeUrl ( url : *const WS_URL , flags : u32 , heap : *const WS_HEAP , outurl : *mut WS_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsEncodeUrl(url, flags, heap, outurl, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsEndReaderCanonicalization(reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsEndReaderCanonicalization ( reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsEndReaderCanonicalization(reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsEndWriterCanonicalization(writer: *const WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsEndWriterCanonicalization ( writer : *const WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsEndWriterCanonicalization(writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsFileTimeToDateTime(filetime: *const super::super::Foundation::FILETIME, datetime: *mut WS_DATETIME, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFileTimeToDateTime ( filetime : *const super::super::Foundation:: FILETIME , datetime : *mut WS_DATETIME , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsFileTimeToDateTime(filetime, datetime, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFillBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFillBody ( message : *const WS_MESSAGE , minsize : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsFillBody(message, minsize, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFillReader(reader: *const WS_XML_READER, minsize: u32, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFillReader ( reader : *const WS_XML_READER , minsize : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsFillReader(reader, minsize, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsFindAttribute<P0>(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, required: P0, attributeindex: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFindAttribute ( reader : *const WS_XML_READER , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , required : super::super::Foundation:: BOOL , attributeindex : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsFindAttribute(reader, localname, ns, required.into_param().abi(), attributeindex, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFlushBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFlushBody ( message : *const WS_MESSAGE , minsize : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsFlushBody(message, minsize, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFlushWriter(writer: *const WS_XML_WRITER, minsize: u32, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFlushWriter ( writer : *const WS_XML_WRITER , minsize : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsFlushWriter(writer, minsize, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeChannel(channel: *const WS_CHANNEL) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeChannel ( channel : *const WS_CHANNEL ) -> ( ) );
    WsFreeChannel(channel)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeError(error: *const WS_ERROR) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeError ( error : *const WS_ERROR ) -> ( ) );
    WsFreeError(error)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeHeap(heap: *const WS_HEAP) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeHeap ( heap : *const WS_HEAP ) -> ( ) );
    WsFreeHeap(heap)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeListener(listener: *const WS_LISTENER) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeListener ( listener : *const WS_LISTENER ) -> ( ) );
    WsFreeListener(listener)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeMessage(message: *const WS_MESSAGE) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeMessage ( message : *const WS_MESSAGE ) -> ( ) );
    WsFreeMessage(message)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeMetadata(metadata: *const WS_METADATA) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeMetadata ( metadata : *const WS_METADATA ) -> ( ) );
    WsFreeMetadata(metadata)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeReader(reader: *const WS_XML_READER) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeReader ( reader : *const WS_XML_READER ) -> ( ) );
    WsFreeReader(reader)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeSecurityToken(token: *const WS_SECURITY_TOKEN) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeSecurityToken ( token : *const WS_SECURITY_TOKEN ) -> ( ) );
    WsFreeSecurityToken(token)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeServiceHost(servicehost: *const WS_SERVICE_HOST) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeServiceHost ( servicehost : *const WS_SERVICE_HOST ) -> ( ) );
    WsFreeServiceHost(servicehost)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeServiceProxy(serviceproxy: *const WS_SERVICE_PROXY) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeServiceProxy ( serviceproxy : *const WS_SERVICE_PROXY ) -> ( ) );
    WsFreeServiceProxy(serviceproxy)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsFreeWriter(writer: *const WS_XML_WRITER) {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsFreeWriter ( writer : *const WS_XML_WRITER ) -> ( ) );
    WsFreeWriter(writer)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetChannelProperty ( channel : *const WS_CHANNEL , id : WS_CHANNEL_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetChannelProperty(channel, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetCustomHeader(message: *const WS_MESSAGE, customheaderdescription: *const WS_ELEMENT_DESCRIPTION, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, headerattributes: ::core::option::Option<*mut u32>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetCustomHeader ( message : *const WS_MESSAGE , customheaderdescription : *const WS_ELEMENT_DESCRIPTION , repeatingoption : WS_REPEATING_HEADER_OPTION , headerindex : u32 , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , headerattributes : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetCustomHeader(message, customheaderdescription, repeatingoption, headerindex, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(headerattributes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetDictionary(encoding: WS_ENCODING, dictionary: *mut *mut WS_XML_DICTIONARY, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetDictionary ( encoding : WS_ENCODING , dictionary : *mut *mut WS_XML_DICTIONARY , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetDictionary(encoding, dictionary, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetErrorProperty ( error : *const WS_ERROR , id : WS_ERROR_PROPERTY_ID , buffer : *mut ::core::ffi::c_void , buffersize : u32 ) -> ::windows::core::HRESULT );
    WsGetErrorProperty(error, id, buffer, buffersize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetErrorString(error: *const WS_ERROR, index: u32) -> ::windows::core::Result<WS_STRING> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetErrorString ( error : *const WS_ERROR , index : u32 , string : *mut WS_STRING ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<WS_STRING>();
    WsGetErrorString(error, index, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetFaultErrorDetail ( error : *const WS_ERROR , faultdetaildescription : *const WS_FAULT_DETAIL_DESCRIPTION , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 ) -> ::windows::core::HRESULT );
    WsGetFaultErrorDetail(error, faultdetaildescription, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetFaultErrorProperty ( error : *const WS_ERROR , id : WS_FAULT_ERROR_PROPERTY_ID , buffer : *mut ::core::ffi::c_void , buffersize : u32 ) -> ::windows::core::HRESULT );
    WsGetFaultErrorProperty(error, id, buffer, buffersize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetHeader ( message : *const WS_MESSAGE , headertype : WS_HEADER_TYPE , valuetype : WS_TYPE , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetHeader(message, headertype, valuetype, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetHeaderAttributes(message: *const WS_MESSAGE, reader: *const WS_XML_READER, headerattributes: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetHeaderAttributes ( message : *const WS_MESSAGE , reader : *const WS_XML_READER , headerattributes : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetHeaderAttributes(message, reader, headerattributes, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetHeapProperty(heap: *const WS_HEAP, id: WS_HEAP_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetHeapProperty ( heap : *const WS_HEAP , id : WS_HEAP_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetHeapProperty(heap, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetListenerProperty ( listener : *const WS_LISTENER , id : WS_LISTENER_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetListenerProperty(listener, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetMappedHeader ( message : *const WS_MESSAGE , headername : *const WS_XML_STRING , repeatingoption : WS_REPEATING_HEADER_OPTION , headerindex : u32 , valuetype : WS_TYPE , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetMappedHeader(message, headername, repeatingoption, headerindex, valuetype, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetMessageProperty ( message : *const WS_MESSAGE , id : WS_MESSAGE_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetMessageProperty(message, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetMetadataEndpoints(metadata: *const WS_METADATA, endpoints: *mut WS_METADATA_ENDPOINTS, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetMetadataEndpoints ( metadata : *const WS_METADATA , endpoints : *mut WS_METADATA_ENDPOINTS , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetMetadataEndpoints(metadata, endpoints, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetMetadataProperty(metadata: *const WS_METADATA, id: WS_METADATA_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetMetadataProperty ( metadata : *const WS_METADATA , id : WS_METADATA_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetMetadataProperty(metadata, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetMissingMetadataDocumentAddress(metadata: *const WS_METADATA, address: *mut *mut WS_ENDPOINT_ADDRESS, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetMissingMetadataDocumentAddress ( metadata : *const WS_METADATA , address : *mut *mut WS_ENDPOINT_ADDRESS , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetMissingMetadataDocumentAddress(metadata, address, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetNamespaceFromPrefix<P0>(reader: *const WS_XML_READER, prefix: *const WS_XML_STRING, required: P0, ns: *mut *mut WS_XML_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetNamespaceFromPrefix ( reader : *const WS_XML_READER , prefix : *const WS_XML_STRING , required : super::super::Foundation:: BOOL , ns : *mut *mut WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetNamespaceFromPrefix(reader, prefix, required.into_param().abi(), ns, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetOperationContextProperty(context: *const WS_OPERATION_CONTEXT, id: WS_OPERATION_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetOperationContextProperty ( context : *const WS_OPERATION_CONTEXT , id : WS_OPERATION_CONTEXT_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetOperationContextProperty(context, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetPolicyAlternativeCount(policy: *const WS_POLICY, count: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetPolicyAlternativeCount ( policy : *const WS_POLICY , count : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetPolicyAlternativeCount(policy, count, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetPolicyProperty(policy: *const WS_POLICY, id: WS_POLICY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetPolicyProperty ( policy : *const WS_POLICY , id : WS_POLICY_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetPolicyProperty(policy, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetPrefixFromNamespace<P0>(writer: *const WS_XML_WRITER, ns: *const WS_XML_STRING, required: P0, prefix: *mut *mut WS_XML_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetPrefixFromNamespace ( writer : *const WS_XML_WRITER , ns : *const WS_XML_STRING , required : super::super::Foundation:: BOOL , prefix : *mut *mut WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetPrefixFromNamespace(writer, ns, required.into_param().abi(), prefix, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetReaderNode(xmlreader: *const WS_XML_READER, node: *mut *mut WS_XML_NODE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetReaderNode ( xmlreader : *const WS_XML_READER , node : *mut *mut WS_XML_NODE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetReaderNode(xmlreader, node, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetReaderPosition(reader: *const WS_XML_READER, nodeposition: *mut WS_XML_NODE_POSITION, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetReaderPosition ( reader : *const WS_XML_READER , nodeposition : *mut WS_XML_NODE_POSITION , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetReaderPosition(reader, nodeposition, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetReaderProperty(reader: *const WS_XML_READER, id: WS_XML_READER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetReaderProperty ( reader : *const WS_XML_READER , id : WS_XML_READER_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetReaderProperty(reader, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetSecurityContextProperty(securitycontext: *const WS_SECURITY_CONTEXT, id: WS_SECURITY_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetSecurityContextProperty ( securitycontext : *const WS_SECURITY_CONTEXT , id : WS_SECURITY_CONTEXT_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetSecurityContextProperty(securitycontext, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetSecurityTokenProperty(securitytoken: *const WS_SECURITY_TOKEN, id: WS_SECURITY_TOKEN_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, heap: ::core::option::Option<*const WS_HEAP>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetSecurityTokenProperty ( securitytoken : *const WS_SECURITY_TOKEN , id : WS_SECURITY_TOKEN_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , heap : *const WS_HEAP , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetSecurityTokenProperty(securitytoken, id, value, valuesize, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetServiceHostProperty(servicehost: *const WS_SERVICE_HOST, id: WS_SERVICE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetServiceHostProperty ( servicehost : *const WS_SERVICE_HOST , id : WS_SERVICE_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetServiceHostProperty(servicehost, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetServiceProxyProperty(serviceproxy: *const WS_SERVICE_PROXY, id: WS_PROXY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetServiceProxyProperty ( serviceproxy : *const WS_SERVICE_PROXY , id : WS_PROXY_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetServiceProxyProperty(serviceproxy, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *mut WS_XML_NODE_POSITION, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetWriterPosition ( writer : *const WS_XML_WRITER , nodeposition : *mut WS_XML_NODE_POSITION , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetWriterPosition(writer, nodeposition, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsGetWriterProperty(writer: *const WS_XML_WRITER, id: WS_XML_WRITER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetWriterProperty ( writer : *const WS_XML_WRITER , id : WS_XML_WRITER_PROPERTY_ID , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetWriterProperty(writer, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsGetXmlAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, heap: *const WS_HEAP, valuechars: ::core::option::Option<*mut *mut u16>, valuecharcount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsGetXmlAttribute ( reader : *const WS_XML_READER , localname : *const WS_XML_STRING , heap : *const WS_HEAP , valuechars : *mut *mut u16 , valuecharcount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsGetXmlAttribute(reader, localname, heap, ::core::mem::transmute(valuechars.unwrap_or(::std::ptr::null_mut())), valuecharcount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsInitializeMessage(message: *const WS_MESSAGE, initialization: WS_MESSAGE_INITIALIZATION, sourcemessage: ::core::option::Option<*const WS_MESSAGE>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsInitializeMessage ( message : *const WS_MESSAGE , initialization : WS_MESSAGE_INITIALIZATION , sourcemessage : *const WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsInitializeMessage(message, initialization, ::core::mem::transmute(sourcemessage.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsMarkHeaderAsUnderstood(message: *const WS_MESSAGE, headerposition: *const WS_XML_NODE_POSITION, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsMarkHeaderAsUnderstood ( message : *const WS_MESSAGE , headerposition : *const WS_XML_NODE_POSITION , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsMarkHeaderAsUnderstood(message, headerposition, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsMatchPolicyAlternative<P0>(policy: *const WS_POLICY, alternativeindex: u32, policyconstraints: *const WS_POLICY_CONSTRAINTS, matchrequired: P0, heap: *const WS_HEAP, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "webservices.dll""system" fn WsMatchPolicyAlternative ( policy : *const WS_POLICY , alternativeindex : u32 , policyconstraints : *const WS_POLICY_CONSTRAINTS , matchrequired : super::super::Foundation:: BOOL , heap : *const WS_HEAP , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsMatchPolicyAlternative(policy, alternativeindex, policyconstraints, matchrequired.into_param().abi(), heap, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsMoveReader(reader: *const WS_XML_READER, moveto: WS_MOVE_TO, found: ::core::option::Option<*mut super::super::Foundation::BOOL>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsMoveReader ( reader : *const WS_XML_READER , moveto : WS_MOVE_TO , found : *mut super::super::Foundation:: BOOL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsMoveReader(reader, moveto, ::core::mem::transmute(found.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsMoveWriter(writer: *const WS_XML_WRITER, moveto: WS_MOVE_TO, found: ::core::option::Option<*mut super::super::Foundation::BOOL>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsMoveWriter ( writer : *const WS_XML_WRITER , moveto : WS_MOVE_TO , found : *mut super::super::Foundation:: BOOL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsMoveWriter(writer, moveto, ::core::mem::transmute(found.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenChannel(channel: *const WS_CHANNEL, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsOpenChannel ( channel : *const WS_CHANNEL , endpointaddress : *const WS_ENDPOINT_ADDRESS , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsOpenChannel(channel, endpointaddress, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenListener(listener: *const WS_LISTENER, url: *const WS_STRING, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsOpenListener ( listener : *const WS_LISTENER , url : *const WS_STRING , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsOpenListener(listener, url, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsOpenServiceHost ( servicehost : *const WS_SERVICE_HOST , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsOpenServiceHost(servicehost, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsOpenServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, address: *const WS_ENDPOINT_ADDRESS, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsOpenServiceProxy ( serviceproxy : *const WS_SERVICE_PROXY , address : *const WS_ENDPOINT_ADDRESS , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsOpenServiceProxy(serviceproxy, address, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsPullBytes(writer: *const WS_XML_WRITER, callback: WS_PULL_BYTES_CALLBACK, callbackstate: ::core::option::Option<*const ::core::ffi::c_void>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsPullBytes ( writer : *const WS_XML_WRITER , callback : WS_PULL_BYTES_CALLBACK , callbackstate : *const ::core::ffi::c_void , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsPullBytes(writer, callback, ::core::mem::transmute(callbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsPushBytes(writer: *const WS_XML_WRITER, callback: WS_PUSH_BYTES_CALLBACK, callbackstate: ::core::option::Option<*const ::core::ffi::c_void>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsPushBytes ( writer : *const WS_XML_WRITER , callback : WS_PUSH_BYTES_CALLBACK , callbackstate : *const ::core::ffi::c_void , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsPushBytes(writer, callback, ::core::mem::transmute(callbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadArray(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: ::core::option::Option<*mut ::core::ffi::c_void>, arraysize: u32, itemoffset: u32, itemcount: u32, actualitemcount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadArray ( reader : *const WS_XML_READER , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , valuetype : WS_VALUE_TYPE , array : *mut ::core::ffi::c_void , arraysize : u32 , itemoffset : u32 , itemcount : u32 , actualitemcount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadArray(reader, localname, ns, valuetype, ::core::mem::transmute(array.unwrap_or(::std::ptr::null_mut())), arraysize, itemoffset, itemcount, actualitemcount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadAttribute(reader: *const WS_XML_READER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadAttribute ( reader : *const WS_XML_READER , attributedescription : *const WS_ATTRIBUTE_DESCRIPTION , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadAttribute(reader, attributedescription, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadBody ( message : *const WS_MESSAGE , bodydescription : *const WS_ELEMENT_DESCRIPTION , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadBody(message, bodydescription, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadBytes(reader: *const WS_XML_READER, bytes: *mut ::core::ffi::c_void, maxbytecount: u32, actualbytecount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadBytes ( reader : *const WS_XML_READER , bytes : *mut ::core::ffi::c_void , maxbytecount : u32 , actualbytecount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadBytes(reader, bytes, maxbytecount, actualbytecount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadChars(reader: *const WS_XML_READER, chars: &mut [u16], actualcharcount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadChars ( reader : *const WS_XML_READER , chars : ::windows::core::PWSTR , maxcharcount : u32 , actualcharcount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadChars(reader, ::core::mem::transmute(chars.as_ptr()), chars.len() as _, actualcharcount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadCharsUtf8(reader: *const WS_XML_READER, bytes: &mut [u8], actualbytecount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadCharsUtf8 ( reader : *const WS_XML_READER , bytes : *mut u8 , maxbytecount : u32 , actualbytecount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadCharsUtf8(reader, ::core::mem::transmute(bytes.as_ptr()), bytes.len() as _, actualbytecount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadElement(reader: *const WS_XML_READER, elementdescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadElement ( reader : *const WS_XML_READER , elementdescription : *const WS_ELEMENT_DESCRIPTION , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadElement(reader, elementdescription, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEndAttribute(reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadEndAttribute ( reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadEndAttribute(reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEndElement(reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadEndElement ( reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadEndElement(reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEndpointAddressExtension(reader: *const WS_XML_READER, endpointaddress: *const WS_ENDPOINT_ADDRESS, extensiontype: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadEndpointAddressExtension ( reader : *const WS_XML_READER , endpointaddress : *const WS_ENDPOINT_ADDRESS , extensiontype : WS_ENDPOINT_ADDRESS_EXTENSION_TYPE , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadEndpointAddressExtension(reader, endpointaddress, extensiontype, readoption, heap, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEnvelopeEnd(message: *const WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadEnvelopeEnd ( message : *const WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadEnvelopeEnd(message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadEnvelopeStart(message: *const WS_MESSAGE, reader: *const WS_XML_READER, donecallback: WS_MESSAGE_DONE_CALLBACK, donecallbackstate: ::core::option::Option<*const ::core::ffi::c_void>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadEnvelopeStart ( message : *const WS_MESSAGE , reader : *const WS_XML_READER , donecallback : WS_MESSAGE_DONE_CALLBACK , donecallbackstate : *const ::core::ffi::c_void , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadEnvelopeStart(message, reader, donecallback, ::core::mem::transmute(donecallbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadMessageEnd ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadMessageEnd(channel, message, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadMessageStart ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadMessageStart(channel, message, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadMetadata(metadata: *const WS_METADATA, reader: *const WS_XML_READER, url: *const WS_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadMetadata ( metadata : *const WS_METADATA , reader : *const WS_XML_READER , url : *const WS_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadMetadata(metadata, reader, url, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadNode(reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadNode ( reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadNode(reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadQualifiedName(reader: *const WS_XML_READER, heap: *const WS_HEAP, prefix: ::core::option::Option<*mut WS_XML_STRING>, localname: *mut WS_XML_STRING, ns: ::core::option::Option<*mut WS_XML_STRING>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadQualifiedName ( reader : *const WS_XML_READER , heap : *const WS_HEAP , prefix : *mut WS_XML_STRING , localname : *mut WS_XML_STRING , ns : *mut WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadQualifiedName(reader, heap, ::core::mem::transmute(prefix.unwrap_or(::std::ptr::null_mut())), localname, ::core::mem::transmute(ns.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadStartAttribute(reader: *const WS_XML_READER, attributeindex: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadStartAttribute ( reader : *const WS_XML_READER , attributeindex : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadStartAttribute(reader, attributeindex, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadStartElement(reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadStartElement ( reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadStartElement(reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReadToStartElement(reader: *const WS_XML_READER, localname: ::core::option::Option<*const WS_XML_STRING>, ns: ::core::option::Option<*const WS_XML_STRING>, found: ::core::option::Option<*mut super::super::Foundation::BOOL>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadToStartElement ( reader : *const WS_XML_READER , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , found : *mut super::super::Foundation:: BOOL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadToStartElement(reader, ::core::mem::transmute(localname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ns.unwrap_or(::std::ptr::null())), ::core::mem::transmute(found.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadType(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: ::core::option::Option<*const ::core::ffi::c_void>, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadType ( reader : *const WS_XML_READER , typemapping : WS_TYPE_MAPPING , r#type : WS_TYPE , typedescription : *const ::core::ffi::c_void , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadType(reader, typemapping, r#type, ::core::mem::transmute(typedescription.unwrap_or(::std::ptr::null())), readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadValue(reader: *const WS_XML_READER, valuetype: WS_VALUE_TYPE, value: *mut ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadValue ( reader : *const WS_XML_READER , valuetype : WS_VALUE_TYPE , value : *mut ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadValue(reader, valuetype, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadXmlBuffer(reader: *const WS_XML_READER, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadXmlBuffer ( reader : *const WS_XML_READER , heap : *const WS_HEAP , xmlbuffer : *mut *mut WS_XML_BUFFER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadXmlBuffer(reader, heap, xmlbuffer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsReadXmlBufferFromBytes(reader: *const WS_XML_READER, encoding: ::core::option::Option<*const WS_XML_READER_ENCODING>, properties: ::core::option::Option<&[WS_XML_READER_PROPERTY]>, bytes: *const ::core::ffi::c_void, bytecount: u32, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReadXmlBufferFromBytes ( reader : *const WS_XML_READER , encoding : *const WS_XML_READER_ENCODING , properties : *const WS_XML_READER_PROPERTY , propertycount : u32 , bytes : *const ::core::ffi::c_void , bytecount : u32 , heap : *const WS_HEAP , xmlbuffer : *mut *mut WS_XML_BUFFER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReadXmlBufferFromBytes(reader, ::core::mem::transmute(encoding.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), bytes, bytecount, heap, xmlbuffer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsReceiveMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescriptions: &[*const WS_MESSAGE_DESCRIPTION], receiveoption: WS_RECEIVE_OPTION, readbodyoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: *mut ::core::ffi::c_void, valuesize: u32, index: ::core::option::Option<*mut u32>, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsReceiveMessage ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , messagedescriptions : *const *const WS_MESSAGE_DESCRIPTION , messagedescriptioncount : u32 , receiveoption : WS_RECEIVE_OPTION , readbodyoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , index : *mut u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsReceiveMessage(channel, message, ::core::mem::transmute(messagedescriptions.as_ptr()), messagedescriptions.len() as _, receiveoption, readbodyoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), value, valuesize, ::core::mem::transmute(index.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRegisterOperationForCancel(context: *const WS_OPERATION_CONTEXT, cancelcallback: WS_OPERATION_CANCEL_CALLBACK, freestatecallback: WS_OPERATION_FREE_STATE_CALLBACK, userstate: ::core::option::Option<*const ::core::ffi::c_void>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRegisterOperationForCancel ( context : *const WS_OPERATION_CONTEXT , cancelcallback : WS_OPERATION_CANCEL_CALLBACK , freestatecallback : WS_OPERATION_FREE_STATE_CALLBACK , userstate : *const ::core::ffi::c_void , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRegisterOperationForCancel(context, cancelcallback, freestatecallback, ::core::mem::transmute(userstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsRemoveCustomHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, headerns: *const WS_XML_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRemoveCustomHeader ( message : *const WS_MESSAGE , headername : *const WS_XML_STRING , headerns : *const WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRemoveCustomHeader(message, headername, headerns, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRemoveHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRemoveHeader ( message : *const WS_MESSAGE , headertype : WS_HEADER_TYPE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRemoveHeader(message, headertype, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsRemoveMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRemoveMappedHeader ( message : *const WS_MESSAGE , headername : *const WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRemoveMappedHeader(message, headername, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRemoveNode(nodeposition: *const WS_XML_NODE_POSITION, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRemoveNode ( nodeposition : *const WS_XML_NODE_POSITION , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRemoveNode(nodeposition, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsRequestReply(channel: *const WS_CHANNEL, requestmessage: *const WS_MESSAGE, requestmessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, requestbodyvalue: ::core::option::Option<*const ::core::ffi::c_void>, requestbodyvaluesize: u32, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, readoption: WS_READ_OPTION, heap: ::core::option::Option<*const WS_HEAP>, value: ::core::option::Option<*mut ::core::ffi::c_void>, valuesize: u32, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRequestReply ( channel : *const WS_CHANNEL , requestmessage : *const WS_MESSAGE , requestmessagedescription : *const WS_MESSAGE_DESCRIPTION , writeoption : WS_WRITE_OPTION , requestbodyvalue : *const ::core::ffi::c_void , requestbodyvaluesize : u32 , replymessage : *const WS_MESSAGE , replymessagedescription : *const WS_MESSAGE_DESCRIPTION , readoption : WS_READ_OPTION , heap : *const WS_HEAP , value : *mut ::core::ffi::c_void , valuesize : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRequestReply(channel, requestmessage, requestmessagedescription, writeoption, ::core::mem::transmute(requestbodyvalue.unwrap_or(::std::ptr::null())), requestbodyvaluesize, replymessage, replymessagedescription, readoption, ::core::mem::transmute(heap.unwrap_or(::std::ptr::null())), ::core::mem::transmute(value.unwrap_or(::std::ptr::null_mut())), valuesize, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRequestSecurityToken(channel: *const WS_CHANNEL, properties: ::core::option::Option<&[WS_REQUEST_SECURITY_TOKEN_PROPERTY]>, token: *mut *mut WS_SECURITY_TOKEN, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRequestSecurityToken ( channel : *const WS_CHANNEL , properties : *const WS_REQUEST_SECURITY_TOKEN_PROPERTY , propertycount : u32 , token : *mut *mut WS_SECURITY_TOKEN , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRequestSecurityToken(channel, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), token, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetChannel(channel: *const WS_CHANNEL, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetChannel ( channel : *const WS_CHANNEL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetChannel(channel, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetError(error: *const WS_ERROR) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetError ( error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetError(error).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetHeap(heap: *const WS_HEAP, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetHeap ( heap : *const WS_HEAP , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetHeap(heap, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetListener(listener: *const WS_LISTENER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetListener ( listener : *const WS_LISTENER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetListener(listener, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetMessage(message: *const WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetMessage ( message : *const WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetMessage(message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetMetadata(metadata: *const WS_METADATA, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetMetadata ( metadata : *const WS_METADATA , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetMetadata(metadata, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetServiceHost(servicehost: *const WS_SERVICE_HOST, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetServiceHost ( servicehost : *const WS_SERVICE_HOST , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetServiceHost(servicehost, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsResetServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsResetServiceProxy ( serviceproxy : *const WS_SERVICE_PROXY , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsResetServiceProxy(serviceproxy, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsRevokeSecurityContext(securitycontext: *const WS_SECURITY_CONTEXT, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsRevokeSecurityContext ( securitycontext : *const WS_SECURITY_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsRevokeSecurityContext(securitycontext, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSendFaultMessageForError(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, faulterror: *const WS_ERROR, faulterrorcode: ::windows::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, requestmessage: *const WS_MESSAGE, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSendFaultMessageForError ( channel : *const WS_CHANNEL , replymessage : *const WS_MESSAGE , faulterror : *const WS_ERROR , faulterrorcode : ::windows::core::HRESULT , faultdisclosure : WS_FAULT_DISCLOSURE , requestmessage : *const WS_MESSAGE , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSendFaultMessageForError(channel, replymessage, faulterror, faulterrorcode, faultdisclosure, requestmessage, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsSendMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, bodyvalue: ::core::option::Option<*const ::core::ffi::c_void>, bodyvaluesize: u32, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSendMessage ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , messagedescription : *const WS_MESSAGE_DESCRIPTION , writeoption : WS_WRITE_OPTION , bodyvalue : *const ::core::ffi::c_void , bodyvaluesize : u32 , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSendMessage(channel, message, messagedescription, writeoption, ::core::mem::transmute(bodyvalue.unwrap_or(::std::ptr::null())), bodyvaluesize, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsSendReplyMessage(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, replybodyvalue: ::core::option::Option<*const ::core::ffi::c_void>, replybodyvaluesize: u32, requestmessage: *const WS_MESSAGE, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSendReplyMessage ( channel : *const WS_CHANNEL , replymessage : *const WS_MESSAGE , replymessagedescription : *const WS_MESSAGE_DESCRIPTION , writeoption : WS_WRITE_OPTION , replybodyvalue : *const ::core::ffi::c_void , replybodyvaluesize : u32 , requestmessage : *const WS_MESSAGE , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSendReplyMessage(channel, replymessage, replymessagedescription, writeoption, ::core::mem::transmute(replybodyvalue.unwrap_or(::std::ptr::null())), replybodyvaluesize, requestmessage, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetChannelProperty ( channel : *const WS_CHANNEL , id : WS_CHANNEL_PROPERTY_ID , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetChannelProperty(channel, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetErrorProperty ( error : *const WS_ERROR , id : WS_ERROR_PROPERTY_ID , value : *const ::core::ffi::c_void , valuesize : u32 ) -> ::windows::core::HRESULT );
    WsSetErrorProperty(error, id, value, valuesize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsSetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: ::core::option::Option<*const ::core::ffi::c_void>, valuesize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetFaultErrorDetail ( error : *const WS_ERROR , faultdetaildescription : *const WS_FAULT_DETAIL_DESCRIPTION , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 ) -> ::windows::core::HRESULT );
    WsSetFaultErrorDetail(error, faultdetaildescription, writeoption, ::core::mem::transmute(value.unwrap_or(::std::ptr::null())), valuesize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetFaultErrorProperty ( error : *const WS_ERROR , id : WS_FAULT_ERROR_PROPERTY_ID , value : *const ::core::ffi::c_void , valuesize : u32 ) -> ::windows::core::HRESULT );
    WsSetFaultErrorProperty(error, id, value, valuesize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetHeader ( message : *const WS_MESSAGE , headertype : WS_HEADER_TYPE , valuetype : WS_TYPE , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetHeader(message, headertype, valuetype, writeoption, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetInput(reader: *const WS_XML_READER, encoding: ::core::option::Option<*const WS_XML_READER_ENCODING>, input: ::core::option::Option<*const WS_XML_READER_INPUT>, properties: ::core::option::Option<&[WS_XML_READER_PROPERTY]>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetInput ( reader : *const WS_XML_READER , encoding : *const WS_XML_READER_ENCODING , input : *const WS_XML_READER_INPUT , properties : *const WS_XML_READER_PROPERTY , propertycount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetInput(reader, ::core::mem::transmute(encoding.unwrap_or(::std::ptr::null())), ::core::mem::transmute(input.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetInputToBuffer(reader: *const WS_XML_READER, buffer: *const WS_XML_BUFFER, properties: ::core::option::Option<&[WS_XML_READER_PROPERTY]>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetInputToBuffer ( reader : *const WS_XML_READER , buffer : *const WS_XML_BUFFER , properties : *const WS_XML_READER_PROPERTY , propertycount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetInputToBuffer(reader, buffer, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetListenerProperty ( listener : *const WS_LISTENER , id : WS_LISTENER_PROPERTY_ID , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetListenerProperty(listener, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetMessageProperty ( message : *const WS_MESSAGE , id : WS_MESSAGE_PROPERTY_ID , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetMessageProperty(message, id, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetOutput(writer: *const WS_XML_WRITER, encoding: ::core::option::Option<*const WS_XML_WRITER_ENCODING>, output: ::core::option::Option<*const WS_XML_WRITER_OUTPUT>, properties: ::core::option::Option<&[WS_XML_WRITER_PROPERTY]>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetOutput ( writer : *const WS_XML_WRITER , encoding : *const WS_XML_WRITER_ENCODING , output : *const WS_XML_WRITER_OUTPUT , properties : *const WS_XML_WRITER_PROPERTY , propertycount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetOutput(writer, ::core::mem::transmute(encoding.unwrap_or(::std::ptr::null())), ::core::mem::transmute(output.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetOutputToBuffer(writer: *const WS_XML_WRITER, buffer: *const WS_XML_BUFFER, properties: ::core::option::Option<&[WS_XML_WRITER_PROPERTY]>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetOutputToBuffer ( writer : *const WS_XML_WRITER , buffer : *const WS_XML_BUFFER , properties : *const WS_XML_WRITER_PROPERTY , propertycount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetOutputToBuffer(writer, buffer, ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetReaderPosition(reader: *const WS_XML_READER, nodeposition: *const WS_XML_NODE_POSITION, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetReaderPosition ( reader : *const WS_XML_READER , nodeposition : *const WS_XML_NODE_POSITION , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetReaderPosition(reader, nodeposition, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *const WS_XML_NODE_POSITION, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSetWriterPosition ( writer : *const WS_XML_WRITER , nodeposition : *const WS_XML_NODE_POSITION , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSetWriterPosition(writer, nodeposition, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsShutdownSessionChannel(channel: *const WS_CHANNEL, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsShutdownSessionChannel ( channel : *const WS_CHANNEL , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsShutdownSessionChannel(channel, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsSkipNode(reader: *const WS_XML_READER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsSkipNode ( reader : *const WS_XML_READER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsSkipNode(reader, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsStartReaderCanonicalization(reader: *const WS_XML_READER, writecallback: WS_WRITE_CALLBACK, writecallbackstate: ::core::option::Option<*const ::core::ffi::c_void>, properties: ::core::option::Option<&[WS_XML_CANONICALIZATION_PROPERTY]>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsStartReaderCanonicalization ( reader : *const WS_XML_READER , writecallback : WS_WRITE_CALLBACK , writecallbackstate : *const ::core::ffi::c_void , properties : *const WS_XML_CANONICALIZATION_PROPERTY , propertycount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsStartReaderCanonicalization(reader, writecallback, ::core::mem::transmute(writecallbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsStartWriterCanonicalization(writer: *const WS_XML_WRITER, writecallback: WS_WRITE_CALLBACK, writecallbackstate: ::core::option::Option<*const ::core::ffi::c_void>, properties: ::core::option::Option<&[WS_XML_CANONICALIZATION_PROPERTY]>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsStartWriterCanonicalization ( writer : *const WS_XML_WRITER , writecallback : WS_WRITE_CALLBACK , writecallbackstate : *const ::core::ffi::c_void , properties : *const WS_XML_CANONICALIZATION_PROPERTY , propertycount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsStartWriterCanonicalization(writer, writecallback, ::core::mem::transmute(writecallbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsTrimXmlWhitespace(chars: &[u16], trimmedchars: *mut *mut u16, trimmedcount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsTrimXmlWhitespace ( chars : ::windows::core::PCWSTR , charcount : u32 , trimmedchars : *mut *mut u16 , trimmedcount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsTrimXmlWhitespace(::core::mem::transmute(chars.as_ptr()), chars.len() as _, trimmedchars, trimmedcount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsVerifyXmlNCName(ncnamechars: &[u16], error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsVerifyXmlNCName ( ncnamechars : ::windows::core::PCWSTR , ncnamecharcount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsVerifyXmlNCName(::core::mem::transmute(ncnamechars.as_ptr()), ncnamechars.len() as _, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteArray(writer: *const WS_XML_WRITER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: ::core::option::Option<*const ::core::ffi::c_void>, arraysize: u32, itemoffset: u32, itemcount: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteArray ( writer : *const WS_XML_WRITER , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , valuetype : WS_VALUE_TYPE , array : *const ::core::ffi::c_void , arraysize : u32 , itemoffset : u32 , itemcount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteArray(writer, localname, ns, valuetype, ::core::mem::transmute(array.unwrap_or(::std::ptr::null())), arraysize, itemoffset, itemcount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteAttribute(writer: *const WS_XML_WRITER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: ::core::option::Option<*const ::core::ffi::c_void>, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteAttribute ( writer : *const WS_XML_WRITER , attributedescription : *const WS_ATTRIBUTE_DESCRIPTION , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteAttribute(writer, attributedescription, writeoption, ::core::mem::transmute(value.unwrap_or(::std::ptr::null())), valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteBody ( message : *const WS_MESSAGE , bodydescription : *const WS_ELEMENT_DESCRIPTION , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteBody(message, bodydescription, writeoption, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteBytes(writer: *const WS_XML_WRITER, bytes: *const ::core::ffi::c_void, bytecount: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteBytes ( writer : *const WS_XML_WRITER , bytes : *const ::core::ffi::c_void , bytecount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteBytes(writer, bytes, bytecount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteChars(writer: *const WS_XML_WRITER, chars: &[u16], error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteChars ( writer : *const WS_XML_WRITER , chars : ::windows::core::PCWSTR , charcount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteChars(writer, ::core::mem::transmute(chars.as_ptr()), chars.len() as _, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteCharsUtf8(writer: *const WS_XML_WRITER, bytes: &[u8], error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteCharsUtf8 ( writer : *const WS_XML_WRITER , bytes : *const u8 , bytecount : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteCharsUtf8(writer, ::core::mem::transmute(bytes.as_ptr()), bytes.len() as _, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteElement(writer: *const WS_XML_WRITER, elementdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: ::core::option::Option<*const ::core::ffi::c_void>, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteElement ( writer : *const WS_XML_WRITER , elementdescription : *const WS_ELEMENT_DESCRIPTION , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteElement(writer, elementdescription, writeoption, ::core::mem::transmute(value.unwrap_or(::std::ptr::null())), valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndAttribute(writer: *const WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteEndAttribute ( writer : *const WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteEndAttribute(writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndCData(writer: *const WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteEndCData ( writer : *const WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteEndCData(writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndElement(writer: *const WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteEndElement ( writer : *const WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteEndElement(writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEndStartElement(writer: *const WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteEndStartElement ( writer : *const WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteEndStartElement(writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEnvelopeEnd(message: *const WS_MESSAGE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteEnvelopeEnd ( message : *const WS_MESSAGE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteEnvelopeEnd(message, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteEnvelopeStart(message: *const WS_MESSAGE, writer: *const WS_XML_WRITER, donecallback: WS_MESSAGE_DONE_CALLBACK, donecallbackstate: ::core::option::Option<*const ::core::ffi::c_void>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteEnvelopeStart ( message : *const WS_MESSAGE , writer : *const WS_XML_WRITER , donecallback : WS_MESSAGE_DONE_CALLBACK , donecallbackstate : *const ::core::ffi::c_void , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteEnvelopeStart(message, writer, donecallback, ::core::mem::transmute(donecallbackstate.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteMessageEnd ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteMessageEnd(channel, message, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: ::core::option::Option<*const WS_ASYNC_CONTEXT>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteMessageStart ( channel : *const WS_CHANNEL , message : *const WS_MESSAGE , asynccontext : *const WS_ASYNC_CONTEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteMessageStart(channel, message, ::core::mem::transmute(asynccontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteNode(writer: *const WS_XML_WRITER, node: *const WS_XML_NODE, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteNode ( writer : *const WS_XML_WRITER , node : *const WS_XML_NODE , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteNode(writer, node, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteQualifiedName(writer: *const WS_XML_WRITER, prefix: ::core::option::Option<*const WS_XML_STRING>, localname: *const WS_XML_STRING, ns: ::core::option::Option<*const WS_XML_STRING>, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteQualifiedName ( writer : *const WS_XML_WRITER , prefix : *const WS_XML_STRING , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteQualifiedName(writer, ::core::mem::transmute(prefix.unwrap_or(::std::ptr::null())), localname, ::core::mem::transmute(ns.unwrap_or(::std::ptr::null())), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteStartAttribute<P0>(writer: *const WS_XML_WRITER, prefix: ::core::option::Option<*const WS_XML_STRING>, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: P0, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteStartAttribute ( writer : *const WS_XML_WRITER , prefix : *const WS_XML_STRING , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , singlequote : super::super::Foundation:: BOOL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteStartAttribute(writer, ::core::mem::transmute(prefix.unwrap_or(::std::ptr::null())), localname, ns, singlequote.into_param().abi(), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteStartCData(writer: *const WS_XML_WRITER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteStartCData ( writer : *const WS_XML_WRITER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteStartCData(writer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteStartElement(writer: *const WS_XML_WRITER, prefix: ::core::option::Option<*const WS_XML_STRING>, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteStartElement ( writer : *const WS_XML_WRITER , prefix : *const WS_XML_STRING , localname : *const WS_XML_STRING , ns : *const WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteStartElement(writer, ::core::mem::transmute(prefix.unwrap_or(::std::ptr::null())), localname, ns, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteText(writer: *const WS_XML_WRITER, text: *const WS_XML_TEXT, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteText ( writer : *const WS_XML_WRITER , text : *const WS_XML_TEXT , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteText(writer, text, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteType(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: ::core::option::Option<*const ::core::ffi::c_void>, writeoption: WS_WRITE_OPTION, value: ::core::option::Option<*const ::core::ffi::c_void>, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteType ( writer : *const WS_XML_WRITER , typemapping : WS_TYPE_MAPPING , r#type : WS_TYPE , typedescription : *const ::core::ffi::c_void , writeoption : WS_WRITE_OPTION , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteType(writer, typemapping, r#type, ::core::mem::transmute(typedescription.unwrap_or(::std::ptr::null())), writeoption, ::core::mem::transmute(value.unwrap_or(::std::ptr::null())), valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteValue(writer: *const WS_XML_WRITER, valuetype: WS_VALUE_TYPE, value: *const ::core::ffi::c_void, valuesize: u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteValue ( writer : *const WS_XML_WRITER , valuetype : WS_VALUE_TYPE , value : *const ::core::ffi::c_void , valuesize : u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteValue(writer, valuetype, value, valuesize, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteXmlBuffer(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteXmlBuffer ( writer : *const WS_XML_WRITER , xmlbuffer : *const WS_XML_BUFFER , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteXmlBuffer(writer, xmlbuffer, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[inline]
pub unsafe fn WsWriteXmlBufferToBytes(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, encoding: ::core::option::Option<*const WS_XML_WRITER_ENCODING>, properties: ::core::option::Option<&[WS_XML_WRITER_PROPERTY]>, heap: *const WS_HEAP, bytes: *mut *mut ::core::ffi::c_void, bytecount: *mut u32, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteXmlBufferToBytes ( writer : *const WS_XML_WRITER , xmlbuffer : *const WS_XML_BUFFER , encoding : *const WS_XML_WRITER_ENCODING , properties : *const WS_XML_WRITER_PROPERTY , propertycount : u32 , heap : *const WS_HEAP , bytes : *mut *mut ::core::ffi::c_void , bytecount : *mut u32 , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteXmlBufferToBytes(writer, xmlbuffer, ::core::mem::transmute(encoding.unwrap_or(::std::ptr::null())), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), heap, bytes, bytecount, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsWriteXmlnsAttribute<P0>(writer: *const WS_XML_WRITER, prefix: ::core::option::Option<*const WS_XML_STRING>, ns: *const WS_XML_STRING, singlequote: P0, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "webservices.dll""system" fn WsWriteXmlnsAttribute ( writer : *const WS_XML_WRITER , prefix : *const WS_XML_STRING , ns : *const WS_XML_STRING , singlequote : super::super::Foundation:: BOOL , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsWriteXmlnsAttribute(writer, ::core::mem::transmute(prefix.unwrap_or(::std::ptr::null())), ns, singlequote.into_param().abi(), ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WsXmlStringEquals(string1: *const WS_XML_STRING, string2: *const WS_XML_STRING, error: ::core::option::Option<*const WS_ERROR>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "webservices.dll""system" fn WsXmlStringEquals ( string1 : *const WS_XML_STRING , string2 : *const WS_XML_STRING , error : *const WS_ERROR ) -> ::windows::core::HRESULT );
    WsXmlStringEquals(string1, string2, ::core::mem::transmute(error.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
pub struct IContentPrefetcherTaskTrigger(::windows::core::IUnknown);
impl IContentPrefetcherTaskTrigger {
    pub unsafe fn TriggerContentPrefetcherTask<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).TriggerContentPrefetcherTask)(::windows::core::Interface::as_raw(self), packagefullname.into_param().abi()).ok()
    }
    pub unsafe fn IsRegisteredForContentPrefetch<P0>(&self, packagefullname: P0) -> ::windows::core::Result<u8>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).IsRegisteredForContentPrefetch)(::windows::core::Interface::as_raw(self), packagefullname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IContentPrefetcherTaskTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IContentPrefetcherTaskTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentPrefetcherTaskTrigger {}
impl ::core::fmt::Debug for IContentPrefetcherTaskTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentPrefetcherTaskTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContentPrefetcherTaskTrigger {
    type Vtable = IContentPrefetcherTaskTrigger_Vtbl;
}
impl ::core::clone::Clone for IContentPrefetcherTaskTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContentPrefetcherTaskTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b35a14a_6094_4799_a60e_e474e15d4dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcherTaskTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TriggerContentPrefetcherTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub IsRegisteredForContentPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::windows::core::PCWSTR, isregistered: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_CURRENT_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_API_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_CURRENT_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ASSERTION_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_DECODE_COMMON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_DECODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: ::windows::core::PCWSTR = ::windows::core::w!("none");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: ::windows::core::PCWSTR = ::windows::core::w!("packed");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: ::windows::core::PCWSTR = ::windows::core::w!("tpm");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: ::windows::core::PCWSTR = ::windows::core::w!("fido-u2f");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: ::windows::core::PCWSTR = ::windows::core::w!("2.0");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_HMAC_SECRET_VALUES_FLAG: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P384_WITH_SHA384: i32 = -35i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P521_WITH_SHA512: i32 = -36i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256: i32 = -257i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA384: i32 = -258i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA512: i32 = -259i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA256: i32 = -37i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA384: i32 = -38i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA512: i32 = -39i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: ::windows::core::PCWSTR = ::windows::core::w!("public-key");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB: ::windows::core::PCWSTR = ::windows::core::w!("credBlob");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT: ::windows::core::PCWSTR = ::windows::core::w!("credProtect");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET: ::windows::core::PCWSTR = ::windows::core::w!("hmac-secret");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH: ::windows::core::PCWSTR = ::windows::core::w!("minPinLength");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: ::windows::core::PCWSTR = ::windows::core::w!("SHA-256");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: ::windows::core::PCWSTR = ::windows::core::w!("SHA-384");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: ::windows::core::PCWSTR = ::windows::core::w!("SHA-512");
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_MAX_USER_ID_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIRED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_CN_MISMATCH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_INVALID_DATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_REVOCATION_OFFLINE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_UNTRUSTED_ROOT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_FAILURE_WRONG_USAGE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_NILLABLE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_NILLABLE_ITEM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_OPTIONAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_OTHER_NAMESPACE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FIELD_POINTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_BASIC: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_DIGEST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_NEGOTIATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_NONE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_NTLM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SCHEME_PASSPORT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_MAPPING_COMMA_SEPARATOR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_MAPPING_QUOTED_VALUE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_MAPPING_SEMICOLON_SEPARATOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_REQUEST_MAPPING_VERB: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_CODE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_TEXT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_DNS_FULLY_QUALIFIED_HOST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_DNS_HOST: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_EXACT_PATH: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_HOST_ADDRESSES: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_LOCAL_HOST: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_NETBIOS_HOST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_NO_QUERY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_PORT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_PREFIX_PATH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MATCH_URL_THIS_HOST: i32 = 31i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MUST_UNDERSTAND_HEADER_ATTRIBUTE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RELAY_HEADER_ATTRIBUTE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_OPERATION_MESSAGE_NILLABLE_ELEMENT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_ABSTRACT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_IGNORE_TRAILING_ELEMENT_CONTENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_IGNORE_UNHANDLED_ATTRIBUTES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_FLAGS_ALLOW_HOST_WILDCARDS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_FLAGS_NO_PATH_COLLAPSE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_FLAGS_ZERO_TERMINATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_ADDRESSING_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ADDRESSING_VERSION_0_9: WS_ADDRESSING_VERSION = WS_ADDRESSING_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ADDRESSING_VERSION_1_0: WS_ADDRESSING_VERSION = WS_ADDRESSING_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ADDRESSING_VERSION_TRANSPORT: WS_ADDRESSING_VERSION = WS_ADDRESSING_VERSION(3i32);
impl ::core::marker::Copy for WS_ADDRESSING_VERSION {}
impl ::core::clone::Clone for WS_ADDRESSING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ADDRESSING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_ADDRESSING_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_ADDRESSING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ADDRESSING_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_BINDING_TEMPLATE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: WS_BINDING_TEMPLATE_TYPE = WS_BINDING_TEMPLATE_TYPE(13i32);
impl ::core::marker::Copy for WS_BINDING_TEMPLATE_TYPE {}
impl ::core::clone::Clone for WS_BINDING_TEMPLATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_BINDING_TEMPLATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_BINDING_TEMPLATE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_BINDING_TEMPLATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_BINDING_TEMPLATE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CALLBACK_MODEL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SHORT_CALLBACK: WS_CALLBACK_MODEL = WS_CALLBACK_MODEL(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LONG_CALLBACK: WS_CALLBACK_MODEL = WS_CALLBACK_MODEL(1i32);
impl ::core::marker::Copy for WS_CALLBACK_MODEL {}
impl ::core::clone::Clone for WS_CALLBACK_MODEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CALLBACK_MODEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CALLBACK_MODEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CALLBACK_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CALLBACK_MODEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CALL_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_CHECK_MUST_UNDERSTAND: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_SEND_MESSAGE_CONTEXT: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_RECEIVE_MESSAGE_CONTEXT: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CALL_PROPERTY_CALL_ID: WS_CALL_PROPERTY_ID = WS_CALL_PROPERTY_ID(3i32);
impl ::core::marker::Copy for WS_CALL_PROPERTY_ID {}
impl ::core::clone::Clone for WS_CALL_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CALL_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CALL_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CALL_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CALL_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CERT_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SUBJECT_NAME_CERT_CREDENTIAL_TYPE: WS_CERT_CREDENTIAL_TYPE = WS_CERT_CREDENTIAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_THUMBPRINT_CERT_CREDENTIAL_TYPE: WS_CERT_CREDENTIAL_TYPE = WS_CERT_CREDENTIAL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CUSTOM_CERT_CREDENTIAL_TYPE: WS_CERT_CREDENTIAL_TYPE = WS_CERT_CREDENTIAL_TYPE(3i32);
impl ::core::marker::Copy for WS_CERT_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WS_CERT_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CERT_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CERT_CREDENTIAL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CERT_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CERT_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CHANNEL_BINDING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UDP_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CUSTOM_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NAMEDPIPE_CHANNEL_BINDING: WS_CHANNEL_BINDING = WS_CHANNEL_BINDING(4i32);
impl ::core::marker::Copy for WS_CHANNEL_BINDING {}
impl ::core::clone::Clone for WS_CHANNEL_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_BINDING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CHANNEL_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_BINDING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CHANNEL_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_BUFFERED_MESSAGE_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_MESSAGE_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_START_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_FLUSH_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENCODING: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENVELOPE_VERSION: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ADDRESSING_VERSION: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_SESSION_DICTIONARY_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_STATE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ASYNC_CALLBACK_MODEL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_IP_VERSION: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_RESOLVE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CONNECT_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_SEND_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_RECEIVE_RESPONSE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_RECEIVE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CLOSE_TIMEOUT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENABLE_TIMEOUTS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_TRANSFER_MODE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MULTICAST_INTERFACE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MULTICAST_HOPS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_REMOTE_ADDRESS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_REMOTE_IP_ADDRESS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_CONNECTION_ID: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(23i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_CALLBACKS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(24i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_PARAMETERS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(25i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_INSTANCE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(26i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_TRANSPORT_URL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(27i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_NO_DELAY: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(28i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_SEND_KEEP_ALIVES: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(29i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_KEEP_ALIVE_TIME: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(30i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_KEEP_ALIVE_INTERVAL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(31i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_HTTP_SERVER_CONNECTIONS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(32i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_IS_SESSION_SHUT_DOWN: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(33i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CHANNEL_TYPE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(34i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_TRIM_BUFFERED_MESSAGE_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(35i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENCODER: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(36i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_DECODER: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(37i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_PROTECTION_LEVEL: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(38i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_COOKIE_MODE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(39i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_PROXY_SETTING_MODE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(40i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_CUSTOM_HTTP_PROXY: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(41i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_MESSAGE_MAPPING: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(42i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ENABLE_HTTP_REDIRECT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(43i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_REDIRECT_CALLBACK_CONTEXT: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(44i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_FAULTS_AS_ERRORS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(45i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_ALLOW_UNSECURED_FAULTS: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(46i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_SERVER_SPN: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(47i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_HTTP_PROXY_SPN: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(48i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_PROPERTY_MAX_HTTP_REQUEST_HEADERS_BUFFER_SIZE: WS_CHANNEL_PROPERTY_ID = WS_CHANNEL_PROPERTY_ID(49i32);
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_ID {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CHANNEL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_CREATED: WS_CHANNEL_STATE = WS_CHANNEL_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_OPENING: WS_CHANNEL_STATE = WS_CHANNEL_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_ACCEPTING: WS_CHANNEL_STATE = WS_CHANNEL_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_OPEN: WS_CHANNEL_STATE = WS_CHANNEL_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_FAULTED: WS_CHANNEL_STATE = WS_CHANNEL_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_CLOSING: WS_CHANNEL_STATE = WS_CHANNEL_STATE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_STATE_CLOSED: WS_CHANNEL_STATE = WS_CHANNEL_STATE(6i32);
impl ::core::marker::Copy for WS_CHANNEL_STATE {}
impl ::core::clone::Clone for WS_CHANNEL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CHANNEL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CHANNEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_INPUT: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_OUTPUT: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_INPUT_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_OUTPUT_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_DUPLEX: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_DUPLEX_SESSION: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_REQUEST: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHANNEL_TYPE_REPLY: WS_CHANNEL_TYPE = WS_CHANNEL_TYPE(16i32);
impl ::core::marker::Copy for WS_CHANNEL_TYPE {}
impl ::core::clone::Clone for WS_CHANNEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_CHARSET(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_AUTO: WS_CHARSET = WS_CHARSET(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_UTF8: WS_CHARSET = WS_CHARSET(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_UTF16LE: WS_CHARSET = WS_CHARSET(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHARSET_UTF16BE: WS_CHARSET = WS_CHARSET(3i32);
impl ::core::marker::Copy for WS_CHARSET {}
impl ::core::clone::Clone for WS_CHARSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_CHARSET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHARSET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_COOKIE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MANUAL_COOKIE_MODE: WS_COOKIE_MODE = WS_COOKIE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_AUTO_COOKIE_MODE: WS_COOKIE_MODE = WS_COOKIE_MODE(2i32);
impl ::core::marker::Copy for WS_COOKIE_MODE {}
impl ::core::clone::Clone for WS_COOKIE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_COOKIE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_COOKIE_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_COOKIE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_COOKIE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_DATETIME_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_FORMAT_UTC: WS_DATETIME_FORMAT = WS_DATETIME_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_FORMAT_LOCAL: WS_DATETIME_FORMAT = WS_DATETIME_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_FORMAT_NONE: WS_DATETIME_FORMAT = WS_DATETIME_FORMAT(2i32);
impl ::core::marker::Copy for WS_DATETIME_FORMAT {}
impl ::core::clone::Clone for WS_DATETIME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_DATETIME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_DATETIME_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_DATETIME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_DATETIME_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_ENCODING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_BINARY_1: WS_ENCODING = WS_ENCODING(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_BINARY_SESSION_1: WS_ENCODING = WS_ENCODING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_MTOM_UTF8: WS_ENCODING = WS_ENCODING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_MTOM_UTF16BE: WS_ENCODING = WS_ENCODING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_MTOM_UTF16LE: WS_ENCODING = WS_ENCODING(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_UTF8: WS_ENCODING = WS_ENCODING(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_UTF16BE: WS_ENCODING = WS_ENCODING(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_XML_UTF16LE: WS_ENCODING = WS_ENCODING(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENCODING_RAW: WS_ENCODING = WS_ENCODING(8i32);
impl ::core::marker::Copy for WS_ENCODING {}
impl ::core::clone::Clone for WS_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENCODING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_ENDPOINT_ADDRESS_EXTENSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENDPOINT_ADDRESS_EXTENSION_METADATA_ADDRESS: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE = WS_ENDPOINT_ADDRESS_EXTENSION_TYPE(1i32);
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENDPOINT_ADDRESS_EXTENSION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_ENDPOINT_IDENTITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DNS_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UPN_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SPN_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RSA_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UNKNOWN_ENDPOINT_IDENTITY_TYPE: WS_ENDPOINT_IDENTITY_TYPE = WS_ENDPOINT_IDENTITY_TYPE(6i32);
impl ::core::marker::Copy for WS_ENDPOINT_IDENTITY_TYPE {}
impl ::core::clone::Clone for WS_ENDPOINT_IDENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENDPOINT_IDENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_ENDPOINT_IDENTITY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_ENDPOINT_IDENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENDPOINT_IDENTITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_ENVELOPE_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENVELOPE_VERSION_SOAP_1_1: WS_ENVELOPE_VERSION = WS_ENVELOPE_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENVELOPE_VERSION_SOAP_1_2: WS_ENVELOPE_VERSION = WS_ENVELOPE_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENVELOPE_VERSION_NONE: WS_ENVELOPE_VERSION = WS_ENVELOPE_VERSION(3i32);
impl ::core::marker::Copy for WS_ENVELOPE_VERSION {}
impl ::core::clone::Clone for WS_ENVELOPE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ENVELOPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_ENVELOPE_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_ENVELOPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENVELOPE_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_ERROR_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ERROR_PROPERTY_STRING_COUNT: WS_ERROR_PROPERTY_ID = WS_ERROR_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ERROR_PROPERTY_ORIGINAL_ERROR_CODE: WS_ERROR_PROPERTY_ID = WS_ERROR_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ERROR_PROPERTY_LANGID: WS_ERROR_PROPERTY_ID = WS_ERROR_PROPERTY_ID(2i32);
impl ::core::marker::Copy for WS_ERROR_PROPERTY_ID {}
impl ::core::clone::Clone for WS_ERROR_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_ERROR_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_ERROR_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_ERROR_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ERROR_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_EXCEPTION_CODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCEPTION_CODE_USAGE_FAILURE: WS_EXCEPTION_CODE = WS_EXCEPTION_CODE(-1069744128i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCEPTION_CODE_INTERNAL_FAILURE: WS_EXCEPTION_CODE = WS_EXCEPTION_CODE(-1069744127i32);
impl ::core::marker::Copy for WS_EXCEPTION_CODE {}
impl ::core::clone::Clone for WS_EXCEPTION_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_EXCEPTION_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_EXCEPTION_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_EXCEPTION_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXCEPTION_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_EXTENDED_PROTECTION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_POLICY_NEVER: WS_EXTENDED_PROTECTION_POLICY = WS_EXTENDED_PROTECTION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_POLICY_WHEN_SUPPORTED: WS_EXTENDED_PROTECTION_POLICY = WS_EXTENDED_PROTECTION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_POLICY_ALWAYS: WS_EXTENDED_PROTECTION_POLICY = WS_EXTENDED_PROTECTION_POLICY(3i32);
impl ::core::marker::Copy for WS_EXTENDED_PROTECTION_POLICY {}
impl ::core::clone::Clone for WS_EXTENDED_PROTECTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_EXTENDED_PROTECTION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_EXTENDED_PROTECTION_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_EXTENDED_PROTECTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXTENDED_PROTECTION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_EXTENDED_PROTECTION_SCENARIO(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_SCENARIO_BOUND_SERVER: WS_EXTENDED_PROTECTION_SCENARIO = WS_EXTENDED_PROTECTION_SCENARIO(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXTENDED_PROTECTION_SCENARIO_TERMINATED_SSL: WS_EXTENDED_PROTECTION_SCENARIO = WS_EXTENDED_PROTECTION_SCENARIO(2i32);
impl ::core::marker::Copy for WS_EXTENDED_PROTECTION_SCENARIO {}
impl ::core::clone::Clone for WS_EXTENDED_PROTECTION_SCENARIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_EXTENDED_PROTECTION_SCENARIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_EXTENDED_PROTECTION_SCENARIO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_EXTENDED_PROTECTION_SCENARIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXTENDED_PROTECTION_SCENARIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_FAULT_DISCLOSURE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MINIMAL_FAULT_DISCLOSURE: WS_FAULT_DISCLOSURE = WS_FAULT_DISCLOSURE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FULL_FAULT_DISCLOSURE: WS_FAULT_DISCLOSURE = WS_FAULT_DISCLOSURE(1i32);
impl ::core::marker::Copy for WS_FAULT_DISCLOSURE {}
impl ::core::clone::Clone for WS_FAULT_DISCLOSURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_FAULT_DISCLOSURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_FAULT_DISCLOSURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_FAULT_DISCLOSURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FAULT_DISCLOSURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_FAULT_ERROR_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_ERROR_PROPERTY_FAULT: WS_FAULT_ERROR_PROPERTY_ID = WS_FAULT_ERROR_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_ERROR_PROPERTY_ACTION: WS_FAULT_ERROR_PROPERTY_ID = WS_FAULT_ERROR_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_ERROR_PROPERTY_HEADER: WS_FAULT_ERROR_PROPERTY_ID = WS_FAULT_ERROR_PROPERTY_ID(2i32);
impl ::core::marker::Copy for WS_FAULT_ERROR_PROPERTY_ID {}
impl ::core::clone::Clone for WS_FAULT_ERROR_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_FAULT_ERROR_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_FAULT_ERROR_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_FAULT_ERROR_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FAULT_ERROR_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_FIELD_MAPPING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TYPE_ATTRIBUTE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ATTRIBUTE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TEXT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NO_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_ATTRIBUTE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_CHOICE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_ELEMENT_CHOICE_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_ANY_ELEMENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_CONTENT_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ATTRIBUTES_FIELD_MAPPING: WS_FIELD_MAPPING = WS_FIELD_MAPPING(12i32);
impl ::core::marker::Copy for WS_FIELD_MAPPING {}
impl ::core::clone::Clone for WS_FIELD_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_FIELD_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_FIELD_MAPPING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_FIELD_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FIELD_MAPPING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_HEADER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ACTION_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_ID_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RELATES_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FROM_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPLY_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_TO_HEADER: WS_HEADER_TYPE = WS_HEADER_TYPE(7i32);
impl ::core::marker::Copy for WS_HEADER_TYPE {}
impl ::core::clone::Clone for WS_HEADER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HEADER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_HEADER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_HEADER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HEADER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_HEAP_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_MAX_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_TRIM_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_REQUESTED_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HEAP_PROPERTY_ACTUAL_SIZE: WS_HEAP_PROPERTY_ID = WS_HEAP_PROPERTY_ID(3i32);
impl ::core::marker::Copy for WS_HEAP_PROPERTY_ID {}
impl ::core::clone::Clone for WS_HEAP_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HEAP_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_HEAP_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HEAP_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_HTTP_HEADER_AUTH_TARGET(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_TARGET_SERVICE: WS_HTTP_HEADER_AUTH_TARGET = WS_HTTP_HEADER_AUTH_TARGET(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_TARGET_PROXY: WS_HTTP_HEADER_AUTH_TARGET = WS_HTTP_HEADER_AUTH_TARGET(2i32);
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_TARGET {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_TARGET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HTTP_HEADER_AUTH_TARGET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_HTTP_PROXY_SETTING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_PROXY_SETTING_MODE_AUTO: WS_HTTP_PROXY_SETTING_MODE = WS_HTTP_PROXY_SETTING_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_PROXY_SETTING_MODE_NONE: WS_HTTP_PROXY_SETTING_MODE = WS_HTTP_PROXY_SETTING_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_PROXY_SETTING_MODE_CUSTOM: WS_HTTP_PROXY_SETTING_MODE = WS_HTTP_PROXY_SETTING_MODE(3i32);
impl ::core::marker::Copy for WS_HTTP_PROXY_SETTING_MODE {}
impl ::core::clone::Clone for WS_HTTP_PROXY_SETTING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_HTTP_PROXY_SETTING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_HTTP_PROXY_SETTING_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_HTTP_PROXY_SETTING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HTTP_PROXY_SETTING_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_IP_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_IP_VERSION_4: WS_IP_VERSION = WS_IP_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_IP_VERSION_6: WS_IP_VERSION = WS_IP_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_IP_VERSION_AUTO: WS_IP_VERSION = WS_IP_VERSION(3i32);
impl ::core::marker::Copy for WS_IP_VERSION {}
impl ::core::clone::Clone for WS_IP_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_IP_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_IP_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_LISTENER_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_LISTEN_BACKLOG: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_IP_VERSION: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_STATE: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_ASYNC_CALLBACK_MODEL: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CHANNEL_TYPE: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CHANNEL_BINDING: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CONNECT_TIMEOUT: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_IS_MULTICAST: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_MULTICAST_INTERFACES: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_MULTICAST_LOOPBACK: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CLOSE_TIMEOUT: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_TO_HEADER_MATCHING_OPTIONS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_TRANSPORT_URL_MATCHING_OPTIONS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_CALLBACKS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_PARAMETERS: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_INSTANCE: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_PROPERTY_DISALLOWED_USER_AGENT: WS_LISTENER_PROPERTY_ID = WS_LISTENER_PROPERTY_ID(16i32);
impl ::core::marker::Copy for WS_LISTENER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_LISTENER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_LISTENER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_LISTENER_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_LISTENER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_LISTENER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_CREATED: WS_LISTENER_STATE = WS_LISTENER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_OPENING: WS_LISTENER_STATE = WS_LISTENER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_OPEN: WS_LISTENER_STATE = WS_LISTENER_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_FAULTED: WS_LISTENER_STATE = WS_LISTENER_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_CLOSING: WS_LISTENER_STATE = WS_LISTENER_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_LISTENER_STATE_CLOSED: WS_LISTENER_STATE = WS_LISTENER_STATE(5i32);
impl ::core::marker::Copy for WS_LISTENER_STATE {}
impl ::core::clone::Clone for WS_LISTENER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_LISTENER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_LISTENER_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_LISTENER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_LISTENER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_MESSAGE_INITIALIZATION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BLANK_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DUPLICATE_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPLY_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_MESSAGE: WS_MESSAGE_INITIALIZATION = WS_MESSAGE_INITIALIZATION(4i32);
impl ::core::marker::Copy for WS_MESSAGE_INITIALIZATION {}
impl ::core::clone::Clone for WS_MESSAGE_INITIALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_INITIALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_MESSAGE_INITIALIZATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_MESSAGE_INITIALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_INITIALIZATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_MESSAGE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_STATE: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEAP: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_ENVELOPE_VERSION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_ADDRESSING_VERSION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEADER_BUFFER: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEADER_POSITION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_BODY_READER: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_BODY_WRITER: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_IS_ADDRESSED: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HEAP_PROPERTIES: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_XML_READER_PROPERTIES: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_XML_WRITER_PROPERTIES: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_IS_FAULT: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_MAX_PROCESSED_HEADERS: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_USERNAME: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_ENCODED_CERT: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_TRANSPORT_SECURITY_WINDOWS_TOKEN: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_HTTP_HEADER_AUTH_WINDOWS_TOKEN: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_MESSAGE_SECURITY_WINDOWS_TOKEN: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_SAML_ASSERTION: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_SECURITY_CONTEXT: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_PROPERTY_PROTECTION_LEVEL: WS_MESSAGE_PROPERTY_ID = WS_MESSAGE_PROPERTY_ID(21i32);
impl ::core::marker::Copy for WS_MESSAGE_PROPERTY_ID {}
impl ::core::clone::Clone for WS_MESSAGE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_MESSAGE_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_MESSAGE_SECURITY_USAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SUPPORTING_MESSAGE_SECURITY_USAGE: WS_MESSAGE_SECURITY_USAGE = WS_MESSAGE_SECURITY_USAGE(1i32);
impl ::core::marker::Copy for WS_MESSAGE_SECURITY_USAGE {}
impl ::core::clone::Clone for WS_MESSAGE_SECURITY_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_SECURITY_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_MESSAGE_SECURITY_USAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_MESSAGE_SECURITY_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_SECURITY_USAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_MESSAGE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_EMPTY: WS_MESSAGE_STATE = WS_MESSAGE_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_INITIALIZED: WS_MESSAGE_STATE = WS_MESSAGE_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_READING: WS_MESSAGE_STATE = WS_MESSAGE_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_WRITING: WS_MESSAGE_STATE = WS_MESSAGE_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MESSAGE_STATE_DONE: WS_MESSAGE_STATE = WS_MESSAGE_STATE(5i32);
impl ::core::marker::Copy for WS_MESSAGE_STATE {}
impl ::core::clone::Clone for WS_MESSAGE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MESSAGE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_MESSAGE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_MESSAGE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_METADATA_EXCHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_EXCHANGE_TYPE_NONE: WS_METADATA_EXCHANGE_TYPE = WS_METADATA_EXCHANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_EXCHANGE_TYPE_MEX: WS_METADATA_EXCHANGE_TYPE = WS_METADATA_EXCHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_EXCHANGE_TYPE_HTTP_GET: WS_METADATA_EXCHANGE_TYPE = WS_METADATA_EXCHANGE_TYPE(2i32);
impl ::core::marker::Copy for WS_METADATA_EXCHANGE_TYPE {}
impl ::core::clone::Clone for WS_METADATA_EXCHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_METADATA_EXCHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_METADATA_EXCHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_METADATA_EXCHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_EXCHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_METADATA_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_STATE: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_HEAP_PROPERTIES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_POLICY_PROPERTIES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_HEAP_REQUESTED_SIZE: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_MAX_DOCUMENTS: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_HOST_NAMES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_PROPERTY_VERIFY_HOST_NAMES: WS_METADATA_PROPERTY_ID = WS_METADATA_PROPERTY_ID(7i32);
impl ::core::marker::Copy for WS_METADATA_PROPERTY_ID {}
impl ::core::clone::Clone for WS_METADATA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_METADATA_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_METADATA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_STATE_CREATED: WS_METADATA_STATE = WS_METADATA_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_STATE_RESOLVED: WS_METADATA_STATE = WS_METADATA_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_METADATA_STATE_FAULTED: WS_METADATA_STATE = WS_METADATA_STATE(3i32);
impl ::core::marker::Copy for WS_METADATA_STATE {}
impl ::core::clone::Clone for WS_METADATA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_METADATA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_METADATA_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_METADATA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_MOVE_TO(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_ROOT_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_NEXT_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_PREVIOUS_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_CHILD_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_END_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_PARENT_ELEMENT: WS_MOVE_TO = WS_MOVE_TO(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_NEXT_NODE: WS_MOVE_TO = WS_MOVE_TO(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_PREVIOUS_NODE: WS_MOVE_TO = WS_MOVE_TO(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_FIRST_NODE: WS_MOVE_TO = WS_MOVE_TO(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_BOF: WS_MOVE_TO = WS_MOVE_TO(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_EOF: WS_MOVE_TO = WS_MOVE_TO(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_MOVE_TO_CHILD_NODE: WS_MOVE_TO = WS_MOVE_TO(11i32);
impl ::core::marker::Copy for WS_MOVE_TO {}
impl ::core::clone::Clone for WS_MOVE_TO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_MOVE_TO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_MOVE_TO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_MOVE_TO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MOVE_TO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_OPERATION_CONTEXT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_CHANNEL: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_CONTRACT_DESCRIPTION: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_HOST_USER_STATE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_CHANNEL_USER_STATE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_INPUT_MESSAGE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_OUTPUT_MESSAGE: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_HEAP: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_LISTENER: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPERATION_CONTEXT_PROPERTY_ENDPOINT_ADDRESS: WS_OPERATION_CONTEXT_PROPERTY_ID = WS_OPERATION_CONTEXT_PROPERTY_ID(8i32);
impl ::core::marker::Copy for WS_OPERATION_CONTEXT_PROPERTY_ID {}
impl ::core::clone::Clone for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_OPERATION_CONTEXT_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_OPERATION_CONTEXT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_OPERATION_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NON_RPC_LITERAL_OPERATION: WS_OPERATION_STYLE = WS_OPERATION_STYLE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RPC_LITERAL_OPERATION: WS_OPERATION_STYLE = WS_OPERATION_STYLE(1i32);
impl ::core::marker::Copy for WS_OPERATION_STYLE {}
impl ::core::clone::Clone for WS_OPERATION_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_OPERATION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_OPERATION_STYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_OPERATION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_OPERATION_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_NORMAL: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_ARRAY: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_ARRAY_COUNT: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PARAMETER_TYPE_MESSAGES: WS_PARAMETER_TYPE = WS_PARAMETER_TYPE(3i32);
impl ::core::marker::Copy for WS_PARAMETER_TYPE {}
impl ::core::clone::Clone for WS_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_PARAMETER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_POLICY_EXTENSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENDPOINT_POLICY_EXTENSION_TYPE: WS_POLICY_EXTENSION_TYPE = WS_POLICY_EXTENSION_TYPE(1i32);
impl ::core::marker::Copy for WS_POLICY_EXTENSION_TYPE {}
impl ::core::clone::Clone for WS_POLICY_EXTENSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_POLICY_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_POLICY_EXTENSION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_POLICY_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_EXTENSION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_POLICY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_STATE: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_MAX_ALTERNATIVES: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_MAX_DEPTH: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_PROPERTY_MAX_EXTENSIONS: WS_POLICY_PROPERTY_ID = WS_POLICY_PROPERTY_ID(4i32);
impl ::core::marker::Copy for WS_POLICY_PROPERTY_ID {}
impl ::core::clone::Clone for WS_POLICY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_POLICY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_POLICY_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_POLICY_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_STATE_CREATED: WS_POLICY_STATE = WS_POLICY_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_POLICY_STATE_FAULTED: WS_POLICY_STATE = WS_POLICY_STATE(2i32);
impl ::core::marker::Copy for WS_POLICY_STATE {}
impl ::core::clone::Clone for WS_POLICY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_POLICY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_POLICY_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_POLICY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_PROTECTION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROTECTION_LEVEL_NONE: WS_PROTECTION_LEVEL = WS_PROTECTION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROTECTION_LEVEL_SIGN: WS_PROTECTION_LEVEL = WS_PROTECTION_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROTECTION_LEVEL_SIGN_AND_ENCRYPT: WS_PROTECTION_LEVEL = WS_PROTECTION_LEVEL(3i32);
impl ::core::marker::Copy for WS_PROTECTION_LEVEL {}
impl ::core::clone::Clone for WS_PROTECTION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_PROTECTION_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_PROXY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_CALL_TIMEOUT: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MESSAGE_PROPERTIES: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MAX_CALL_POOL_SIZE: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_STATE: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MAX_PENDING_CALLS: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_PROPERTY_MAX_CLOSE_TIMEOUT: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_PROXY_FAULT_LANG_ID: WS_PROXY_PROPERTY_ID = WS_PROXY_PROPERTY_ID(6i32);
impl ::core::marker::Copy for WS_PROXY_PROPERTY_ID {}
impl ::core::clone::Clone for WS_PROXY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_PROXY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_PROXY_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_PROXY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PROXY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_READ_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_REQUIRED_VALUE: WS_READ_OPTION = WS_READ_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_REQUIRED_POINTER: WS_READ_OPTION = WS_READ_OPTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_OPTIONAL_POINTER: WS_READ_OPTION = WS_READ_OPTION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_NILLABLE_POINTER: WS_READ_OPTION = WS_READ_OPTION(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_READ_NILLABLE_VALUE: WS_READ_OPTION = WS_READ_OPTION(5i32);
impl ::core::marker::Copy for WS_READ_OPTION {}
impl ::core::clone::Clone for WS_READ_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_READ_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_READ_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_READ_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_READ_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_RECEIVE_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RECEIVE_REQUIRED_MESSAGE: WS_RECEIVE_OPTION = WS_RECEIVE_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RECEIVE_OPTIONAL_MESSAGE: WS_RECEIVE_OPTION = WS_RECEIVE_OPTION(2i32);
impl ::core::marker::Copy for WS_RECEIVE_OPTION {}
impl ::core::clone::Clone for WS_RECEIVE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_RECEIVE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_RECEIVE_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_RECEIVE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_RECEIVE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_REPEATING_HEADER_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REPEATING_HEADER: WS_REPEATING_HEADER_OPTION = WS_REPEATING_HEADER_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SINGLETON_HEADER: WS_REPEATING_HEADER_OPTION = WS_REPEATING_HEADER_OPTION(2i32);
impl ::core::marker::Copy for WS_REPEATING_HEADER_OPTION {}
impl ::core::clone::Clone for WS_REPEATING_HEADER_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_REPEATING_HEADER_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_REPEATING_HEADER_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_REPEATING_HEADER_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REPEATING_HEADER_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_REQUEST_SECURITY_TOKEN_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_ISSUE: WS_REQUEST_SECURITY_TOKEN_ACTION = WS_REQUEST_SECURITY_TOKEN_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_NEW_CONTEXT: WS_REQUEST_SECURITY_TOKEN_ACTION = WS_REQUEST_SECURITY_TOKEN_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_RENEW_CONTEXT: WS_REQUEST_SECURITY_TOKEN_ACTION = WS_REQUEST_SECURITY_TOKEN_ACTION(3i32);
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_ACTION {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_REQUEST_SECURITY_TOKEN_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REQUEST_SECURITY_TOKEN_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_APPLIES_TO: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_TRUST_VERSION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_SECURE_CONVERSATION_VERSION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_TYPE: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_REQUEST_ACTION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_EXISTING_TOKEN: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_TYPE: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_SIZE: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_ENTROPY: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_LOCAL_REQUEST_PARAMETERS: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_SERVICE_REQUEST_PARAMETERS: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_MESSAGE_PROPERTIES: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_BEARER_KEY_TYPE_VERSION: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID = WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(13i32);
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SAML_AUTHENTICATOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_SIGNED_SAML_AUTHENTICATOR_TYPE: WS_SAML_AUTHENTICATOR_TYPE = WS_SAML_AUTHENTICATOR_TYPE(1i32);
impl ::core::marker::Copy for WS_SAML_AUTHENTICATOR_TYPE {}
impl ::core::clone::Clone for WS_SAML_AUTHENTICATOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SAML_AUTHENTICATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SAML_AUTHENTICATOR_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SAML_AUTHENTICATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SAML_AUTHENTICATOR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURE_CONVERSATION_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_CONVERSATION_VERSION_FEBRUARY_2005: WS_SECURE_CONVERSATION_VERSION = WS_SECURE_CONVERSATION_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_CONVERSATION_VERSION_1_3: WS_SECURE_CONVERSATION_VERSION = WS_SECURE_CONVERSATION_VERSION(2i32);
impl ::core::marker::Copy for WS_SECURE_CONVERSATION_VERSION {}
impl ::core::clone::Clone for WS_SECURE_CONVERSATION_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURE_CONVERSATION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURE_CONVERSATION_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURE_CONVERSATION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURE_CONVERSATION_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURE_PROTOCOL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_SSL2: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_SSL3: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_TLS1_0: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_TLS1_1: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURE_PROTOCOL_TLS1_2: WS_SECURE_PROTOCOL = WS_SECURE_PROTOCOL(16i32);
impl ::core::marker::Copy for WS_SECURE_PROTOCOL {}
impl ::core::clone::Clone for WS_SECURE_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURE_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURE_PROTOCOL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURE_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURE_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_ALGORITHM_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DEFAULT: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_CANONICALIZATION_EXCLUSIVE: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_CANONICALIZATION_EXCLUSIVE_WITH_COMMENTS: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_256: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_384: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_512: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_256: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_384: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_512: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_DSA_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_256: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_384: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_512: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_KEYWRAP_RSA_1_5: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_KEYWRAP_RSA_OAEP: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_KEY_DERIVATION_P_SHA1: WS_SECURITY_ALGORITHM_ID = WS_SECURITY_ALGORITHM_ID(18i32);
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_ID {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_ALGORITHM_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_ALGORITHM_PROPERTY_ID(pub i32);
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_ALGORITHM_SUITE_NAME(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_SHA256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_SHA256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_SHA256: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_SHA256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_SHA256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_SHA256_RSA15: WS_SECURITY_ALGORITHM_SUITE_NAME = WS_SECURITY_ALGORITHM_SUITE_NAME(12i32);
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_SUITE_NAME {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_ALGORITHM_SUITE_NAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_SUITE_NAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_BEARER_KEY_TYPE_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ORIGINAL_SPECIFICATION: WS_SECURITY_BEARER_KEY_TYPE_VERSION = WS_SECURITY_BEARER_KEY_TYPE_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ORIGINAL_SCHEMA: WS_SECURITY_BEARER_KEY_TYPE_VERSION = WS_SECURITY_BEARER_KEY_TYPE_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ERRATA_01: WS_SECURITY_BEARER_KEY_TYPE_VERSION = WS_SECURITY_BEARER_KEY_TYPE_VERSION(3i32);
impl ::core::marker::Copy for WS_SECURITY_BEARER_KEY_TYPE_VERSION {}
impl ::core::clone::Clone for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BEARER_KEY_TYPE_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_BINDING_CONSTRAINT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: WS_SECURITY_BINDING_CONSTRAINT_TYPE = WS_SECURITY_BINDING_CONSTRAINT_TYPE(8i32);
impl ::core::marker::Copy for WS_SECURITY_BINDING_CONSTRAINT_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_CONSTRAINT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_BINDING_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_REQUIRE_SSL_CLIENT_CERT: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_WINDOWS_INTEGRATED_AUTH_PACKAGE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_REQUIRE_SERVER_AUTH: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_ALLOW_ANONYMOUS_CLIENTS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_ALLOWED_IMPERSONATION_LEVEL: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_SCHEME: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_TARGET: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_BASIC_REALM: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_DIGEST_REALM: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_DIGEST_DOMAIN: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_KEY_SIZE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_KEY_ENTROPY_MODE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_MESSAGE_PROPERTIES: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_MAX_PENDING_CONTEXTS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_MAX_ACTIVE_CONTEXTS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURE_CONVERSATION_VERSION: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_SUPPORT_RENEW: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_RENEWAL_INTERVAL: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_ROLLOVER_INTERVAL: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_CERT_FAILURES_TO_IGNORE: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_DISABLE_CERT_REVOCATION_CHECK: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_DISALLOWED_SECURE_PROTOCOLS: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_BINDING_PROPERTY_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT: WS_SECURITY_BINDING_PROPERTY_ID = WS_SECURITY_BINDING_PROPERTY_ID(23i32);
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_BINDING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SSL_TRANSPORT_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_USERNAME_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TOKEN_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SAML_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING_TYPE: WS_SECURITY_BINDING_TYPE = WS_SECURITY_BINDING_TYPE(9i32);
impl ::core::marker::Copy for WS_SECURITY_BINDING_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_CONTEXT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_IDENTIFIER: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_USERNAME: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_MESSAGE_SECURITY_WINDOWS_TOKEN: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_CONTEXT_PROPERTY_SAML_ASSERTION: WS_SECURITY_CONTEXT_PROPERTY_ID = WS_SECURITY_CONTEXT_PROPERTY_ID(4i32);
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_CONTEXT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_HEADER_LAYOUT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_STRICT: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_LAX: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_LAX_WITH_TIMESTAMP_FIRST: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_LAYOUT_LAX_WITH_TIMESTAMP_LAST: WS_SECURITY_HEADER_LAYOUT = WS_SECURITY_HEADER_LAYOUT(4i32);
impl ::core::marker::Copy for WS_SECURITY_HEADER_LAYOUT {}
impl ::core::clone::Clone for WS_SECURITY_HEADER_LAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_HEADER_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_HEADER_LAYOUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_HEADER_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_HEADER_LAYOUT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_HEADER_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_VERSION_1_0: WS_SECURITY_HEADER_VERSION = WS_SECURITY_HEADER_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_HEADER_VERSION_1_1: WS_SECURITY_HEADER_VERSION = WS_SECURITY_HEADER_VERSION(2i32);
impl ::core::marker::Copy for WS_SECURITY_HEADER_VERSION {}
impl ::core::clone::Clone for WS_SECURITY_HEADER_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_HEADER_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_HEADER_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_HEADER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_HEADER_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_KEY_ENTROPY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_ENTROPY_MODE_CLIENT_ONLY: WS_SECURITY_KEY_ENTROPY_MODE = WS_SECURITY_KEY_ENTROPY_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_ENTROPY_MODE_SERVER_ONLY: WS_SECURITY_KEY_ENTROPY_MODE = WS_SECURITY_KEY_ENTROPY_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_ENTROPY_MODE_COMBINED: WS_SECURITY_KEY_ENTROPY_MODE = WS_SECURITY_KEY_ENTROPY_MODE(3i32);
impl ::core::marker::Copy for WS_SECURITY_KEY_ENTROPY_MODE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_ENTROPY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_ENTROPY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_KEY_ENTROPY_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_ENTROPY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_ENTROPY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_KEY_HANDLE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE_TYPE: WS_SECURITY_KEY_HANDLE_TYPE = WS_SECURITY_KEY_HANDLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE_TYPE: WS_SECURITY_KEY_HANDLE_TYPE = WS_SECURITY_KEY_HANDLE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE_TYPE: WS_SECURITY_KEY_HANDLE_TYPE = WS_SECURITY_KEY_HANDLE_TYPE(3i32);
impl ::core::marker::Copy for WS_SECURITY_KEY_HANDLE_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_HANDLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_HANDLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_KEY_HANDLE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_HANDLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_HANDLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_KEY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_TYPE_NONE: WS_SECURITY_KEY_TYPE = WS_SECURITY_KEY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_TYPE_SYMMETRIC: WS_SECURITY_KEY_TYPE = WS_SECURITY_KEY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_KEY_TYPE_ASYMMETRIC: WS_SECURITY_KEY_TYPE = WS_SECURITY_KEY_TYPE(3i32);
impl ::core::marker::Copy for WS_SECURITY_KEY_TYPE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_KEY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_TRANSPORT_PROTECTION_LEVEL: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_ALGORITHM_SUITE: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_ALGORITHM_SUITE_NAME: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_MAX_ALLOWED_LATENCY: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_TIMESTAMP_VALIDITY_DURATION: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_MAX_ALLOWED_CLOCK_SKEW: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_TIMESTAMP_USAGE: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_SECURITY_HEADER_LAYOUT: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_SECURITY_HEADER_VERSION: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_EXTENDED_PROTECTION_POLICY: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_EXTENDED_PROTECTION_SCENARIO: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_PROPERTY_SERVICE_IDENTITIES: WS_SECURITY_PROPERTY_ID = WS_SECURITY_PROPERTY_ID(12i32);
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_TIMESTAMP_USAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TIMESTAMP_USAGE_ALWAYS: WS_SECURITY_TIMESTAMP_USAGE = WS_SECURITY_TIMESTAMP_USAGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TIMESTAMP_USAGE_NEVER: WS_SECURITY_TIMESTAMP_USAGE = WS_SECURITY_TIMESTAMP_USAGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TIMESTAMP_USAGE_REQUESTS_ONLY: WS_SECURITY_TIMESTAMP_USAGE = WS_SECURITY_TIMESTAMP_USAGE(3i32);
impl ::core::marker::Copy for WS_SECURITY_TIMESTAMP_USAGE {}
impl ::core::clone::Clone for WS_SECURITY_TIMESTAMP_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_TIMESTAMP_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_TIMESTAMP_USAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_TIMESTAMP_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TIMESTAMP_USAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_TOKEN_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_KEY_TYPE: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_VALID_FROM_TIME: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_VALID_TILL_TIME: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_SERIALIZED_XML: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_ATTACHED_REFERENCE_XML: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_UNATTACHED_REFERENCE_XML: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_PROPERTY_SYMMETRIC_KEY: WS_SECURITY_TOKEN_PROPERTY_ID = WS_SECURITY_TOKEN_PROPERTY_ID(7i32);
impl ::core::marker::Copy for WS_SECURITY_TOKEN_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_TOKEN_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SECURITY_TOKEN_REFERENCE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_LOCAL_ID: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_XML_BUFFER: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_CERT_THUMBPRINT: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_SECURITY_CONTEXT_ID: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_SAML_ASSERTION_ID: WS_SECURITY_TOKEN_REFERENCE_MODE = WS_SECURITY_TOKEN_REFERENCE_MODE(5i32);
impl ::core::marker::Copy for WS_SECURITY_TOKEN_REFERENCE_MODE {}
impl ::core::clone::Clone for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_TOKEN_REFERENCE_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TOKEN_REFERENCE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SERVICE_CANCEL_REASON(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_ABORT: WS_SERVICE_CANCEL_REASON = WS_SERVICE_CANCEL_REASON(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_CHANNEL_FAULTED: WS_SERVICE_CANCEL_REASON = WS_SERVICE_CANCEL_REASON(1i32);
impl ::core::marker::Copy for WS_SERVICE_CANCEL_REASON {}
impl ::core::clone::Clone for WS_SERVICE_CANCEL_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_CANCEL_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_CANCEL_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SERVICE_CANCEL_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_CANCEL_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SERVICE_ENDPOINT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_ACCEPT_CHANNEL_CALLBACK: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_CLOSE_CHANNEL_CALLBACK: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_ACCEPTING_CHANNELS: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CONCURRENCY: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_BODY_HEAP_MAX_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_BODY_HEAP_TRIM_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MESSAGE_PROPERTIES: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CALL_POOL_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CHANNEL_POOL_SIZE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_LISTENER_PROPERTIES: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_CHECK_MUST_UNDERSTAND: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA_EXCHANGE_TYPE: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA_EXCHANGE_URL_SUFFIX: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CHANNELS: WS_SERVICE_ENDPOINT_PROPERTY_ID = WS_SERVICE_ENDPOINT_PROPERTY_ID(14i32);
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_ENDPOINT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SERVICE_HOST_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_CREATED: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_OPENING: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_OPEN: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_CLOSING: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_CLOSED: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_HOST_STATE_FAULTED: WS_SERVICE_HOST_STATE = WS_SERVICE_HOST_STATE(5i32);
impl ::core::marker::Copy for WS_SERVICE_HOST_STATE {}
impl ::core::clone::Clone for WS_SERVICE_HOST_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_HOST_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_HOST_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SERVICE_HOST_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_HOST_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SERVICE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_HOST_USER_STATE: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_FAULT_DISCLOSURE: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_FAULT_LANGID: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_HOST_STATE: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_METADATA: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROPERTY_CLOSE_TIMEOUT: WS_SERVICE_PROPERTY_ID = WS_SERVICE_PROPERTY_ID(5i32);
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_ID {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_SERVICE_PROXY_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_CREATED: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_OPENING: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_OPEN: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_CLOSING: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_CLOSED: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_SERVICE_PROXY_STATE_FAULTED: WS_SERVICE_PROXY_STATE = WS_SERVICE_PROXY_STATE(5i32);
impl ::core::marker::Copy for WS_SERVICE_PROXY_STATE {}
impl ::core::clone::Clone for WS_SERVICE_PROXY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_SERVICE_PROXY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_PROXY_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_SERVICE_PROXY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_PROXY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_TRACE_API(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_NONE: WS_TRACE_API = WS_TRACE_API(-1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_START_READER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_END_READER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_START_WRITER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_END_WRITER_CANONICALIZATION: WS_TRACE_API = WS_TRACE_API(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_XML_BUFFER: WS_TRACE_API = WS_TRACE_API(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_NODE: WS_TRACE_API = WS_TRACE_API(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_READER: WS_TRACE_API = WS_TRACE_API(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_INPUT: WS_TRACE_API = WS_TRACE_API(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_INPUT_TO_BUFFER: WS_TRACE_API = WS_TRACE_API(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_XML_READER: WS_TRACE_API = WS_TRACE_API(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_READER_PROPERTY: WS_TRACE_API = WS_TRACE_API(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_READER_NODE: WS_TRACE_API = WS_TRACE_API(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FILL_READER: WS_TRACE_API = WS_TRACE_API(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_TO_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_START_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_END_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_NODE: WS_TRACE_API = WS_TRACE_API(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SKIP_NODE: WS_TRACE_API = WS_TRACE_API(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_END_ELEMENT: WS_TRACE_API = WS_TRACE_API(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FIND_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ELEMENT_VALUE: WS_TRACE_API = WS_TRACE_API(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_CHARS: WS_TRACE_API = WS_TRACE_API(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_CHARS_UTF8: WS_TRACE_API = WS_TRACE_API(23i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_BYTES: WS_TRACE_API = WS_TRACE_API(24i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ARRAY: WS_TRACE_API = WS_TRACE_API(25i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_READER_POSITION: WS_TRACE_API = WS_TRACE_API(26i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_READER_POSITION: WS_TRACE_API = WS_TRACE_API(27i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MOVE_READER: WS_TRACE_API = WS_TRACE_API(28i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_WRITER: WS_TRACE_API = WS_TRACE_API(29i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_XML_WRITER: WS_TRACE_API = WS_TRACE_API(30i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_OUTPUT: WS_TRACE_API = WS_TRACE_API(31i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_OUTPUT_TO_BUFFER: WS_TRACE_API = WS_TRACE_API(32i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_WRITER_PROPERTY: WS_TRACE_API = WS_TRACE_API(33i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FLUSH_WRITER: WS_TRACE_API = WS_TRACE_API(34i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(35i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_START_ELEMENT: WS_TRACE_API = WS_TRACE_API(36i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_XMLNS_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(37i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_START_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(38i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(39i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_VALUE: WS_TRACE_API = WS_TRACE_API(40i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_XML_BUFFER: WS_TRACE_API = WS_TRACE_API(41i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_XML_BUFFER: WS_TRACE_API = WS_TRACE_API(42i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_XML_BUFFER_TO_BYTES: WS_TRACE_API = WS_TRACE_API(43i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_XML_BUFFER_FROM_BYTES: WS_TRACE_API = WS_TRACE_API(44i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ARRAY: WS_TRACE_API = WS_TRACE_API(45i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_QUALIFIED_NAME: WS_TRACE_API = WS_TRACE_API(46i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_CHARS: WS_TRACE_API = WS_TRACE_API(47i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_CHARS_UTF8: WS_TRACE_API = WS_TRACE_API(48i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_BYTES: WS_TRACE_API = WS_TRACE_API(49i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_PUSH_BYTES: WS_TRACE_API = WS_TRACE_API(50i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_PULL_BYTES: WS_TRACE_API = WS_TRACE_API(51i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_ELEMENT: WS_TRACE_API = WS_TRACE_API(52i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_TEXT: WS_TRACE_API = WS_TRACE_API(53i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_START_CDATA: WS_TRACE_API = WS_TRACE_API(54i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_END_CDATA: WS_TRACE_API = WS_TRACE_API(55i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_NODE: WS_TRACE_API = WS_TRACE_API(56i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_PREFIX_FROM_NAMESPACE: WS_TRACE_API = WS_TRACE_API(57i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_WRITER_POSITION: WS_TRACE_API = WS_TRACE_API(58i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_WRITER_POSITION: WS_TRACE_API = WS_TRACE_API(59i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MOVE_WRITER: WS_TRACE_API = WS_TRACE_API(60i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_TRIM_XML_WHITESPACE: WS_TRACE_API = WS_TRACE_API(61i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_VERIFY_XML_NCNAME: WS_TRACE_API = WS_TRACE_API(62i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_XML_STRING_EQUALS: WS_TRACE_API = WS_TRACE_API(63i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_NAMESPACE_FROM_PREFIX: WS_TRACE_API = WS_TRACE_API(64i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_QUALIFIED_NAME: WS_TRACE_API = WS_TRACE_API(65i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_XML_ATTRIBUTE: WS_TRACE_API = WS_TRACE_API(66i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_COPY_NODE: WS_TRACE_API = WS_TRACE_API(67i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ASYNC_EXECUTE: WS_TRACE_API = WS_TRACE_API(68i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_CHANNEL: WS_TRACE_API = WS_TRACE_API(69i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_CHANNEL: WS_TRACE_API = WS_TRACE_API(70i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SEND_MESSAGE: WS_TRACE_API = WS_TRACE_API(71i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RECEIVE_MESSAGE: WS_TRACE_API = WS_TRACE_API(72i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REQUEST_REPLY: WS_TRACE_API = WS_TRACE_API(73i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SEND_REPLY_MESSAGE: WS_TRACE_API = WS_TRACE_API(74i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SEND_FAULT_MESSAGE_FOR_ERROR: WS_TRACE_API = WS_TRACE_API(75i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_CHANNEL_PROPERTY: WS_TRACE_API = WS_TRACE_API(76i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_CHANNEL_PROPERTY: WS_TRACE_API = WS_TRACE_API(77i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_MESSAGE_START: WS_TRACE_API = WS_TRACE_API(78i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_MESSAGE_END: WS_TRACE_API = WS_TRACE_API(79i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_MESSAGE_START: WS_TRACE_API = WS_TRACE_API(80i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_MESSAGE_END: WS_TRACE_API = WS_TRACE_API(81i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_CHANNEL: WS_TRACE_API = WS_TRACE_API(82i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_CHANNEL: WS_TRACE_API = WS_TRACE_API(83i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_CHANNEL: WS_TRACE_API = WS_TRACE_API(84i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_CHANNEL: WS_TRACE_API = WS_TRACE_API(85i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABANDON_MESSAGE: WS_TRACE_API = WS_TRACE_API(86i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SHUTDOWN_SESSION_CHANNEL: WS_TRACE_API = WS_TRACE_API(87i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_CONTEXT_PROPERTY: WS_TRACE_API = WS_TRACE_API(88i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_DICTIONARY: WS_TRACE_API = WS_TRACE_API(89i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ENDPOINT_ADDRESS_EXTENSION: WS_TRACE_API = WS_TRACE_API(90i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_ERROR: WS_TRACE_API = WS_TRACE_API(91i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADD_ERROR_STRING: WS_TRACE_API = WS_TRACE_API(92i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_ERROR_STRING: WS_TRACE_API = WS_TRACE_API(93i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_COPY_ERROR: WS_TRACE_API = WS_TRACE_API(94i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(95i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(96i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_ERROR: WS_TRACE_API = WS_TRACE_API(97i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_ERROR: WS_TRACE_API = WS_TRACE_API(98i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_FAULT_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(99i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_FAULT_ERROR_PROPERTY: WS_TRACE_API = WS_TRACE_API(100i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_FAULT_FROM_ERROR: WS_TRACE_API = WS_TRACE_API(101i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_FAULT_ERROR_DETAIL: WS_TRACE_API = WS_TRACE_API(102i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_FAULT_ERROR_DETAIL: WS_TRACE_API = WS_TRACE_API(103i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_HEAP: WS_TRACE_API = WS_TRACE_API(104i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ALLOC: WS_TRACE_API = WS_TRACE_API(105i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_HEAP_PROPERTY: WS_TRACE_API = WS_TRACE_API(106i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_HEAP: WS_TRACE_API = WS_TRACE_API(107i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_HEAP: WS_TRACE_API = WS_TRACE_API(108i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_LISTENER: WS_TRACE_API = WS_TRACE_API(109i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_LISTENER: WS_TRACE_API = WS_TRACE_API(110i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ACCEPT_CHANNEL: WS_TRACE_API = WS_TRACE_API(111i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_LISTENER: WS_TRACE_API = WS_TRACE_API(112i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_LISTENER: WS_TRACE_API = WS_TRACE_API(113i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_LISTENER: WS_TRACE_API = WS_TRACE_API(114i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_LISTENER: WS_TRACE_API = WS_TRACE_API(115i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_LISTENER_PROPERTY: WS_TRACE_API = WS_TRACE_API(116i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_LISTENER_PROPERTY: WS_TRACE_API = WS_TRACE_API(117i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_CHANNEL_FOR_LISTENER: WS_TRACE_API = WS_TRACE_API(118i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_MESSAGE: WS_TRACE_API = WS_TRACE_API(119i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_MESSAGE_FOR_CHANNEL: WS_TRACE_API = WS_TRACE_API(120i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_INITIALIZE_MESSAGE: WS_TRACE_API = WS_TRACE_API(121i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_MESSAGE: WS_TRACE_API = WS_TRACE_API(122i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_MESSAGE: WS_TRACE_API = WS_TRACE_API(123i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_HEADER_ATTRIBUTES: WS_TRACE_API = WS_TRACE_API(124i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_HEADER: WS_TRACE_API = WS_TRACE_API(125i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_CUSTOM_HEADER: WS_TRACE_API = WS_TRACE_API(126i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_HEADER: WS_TRACE_API = WS_TRACE_API(127i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_HEADER: WS_TRACE_API = WS_TRACE_API(128i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_CUSTOM_HEADER: WS_TRACE_API = WS_TRACE_API(129i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADD_CUSTOM_HEADER: WS_TRACE_API = WS_TRACE_API(130i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADD_MAPPED_HEADER: WS_TRACE_API = WS_TRACE_API(131i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REMOVE_MAPPED_HEADER: WS_TRACE_API = WS_TRACE_API(132i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_MAPPED_HEADER: WS_TRACE_API = WS_TRACE_API(133i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_BODY: WS_TRACE_API = WS_TRACE_API(134i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_BODY: WS_TRACE_API = WS_TRACE_API(135i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ENVELOPE_START: WS_TRACE_API = WS_TRACE_API(136i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ENVELOPE_END: WS_TRACE_API = WS_TRACE_API(137i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ENVELOPE_START: WS_TRACE_API = WS_TRACE_API(138i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ENVELOPE_END: WS_TRACE_API = WS_TRACE_API(139i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_MESSAGE_PROPERTY: WS_TRACE_API = WS_TRACE_API(140i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_MESSAGE_PROPERTY: WS_TRACE_API = WS_TRACE_API(141i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ADDRESS_MESSAGE: WS_TRACE_API = WS_TRACE_API(142i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CHECK_MUST_UNDERSTAND_HEADERS: WS_TRACE_API = WS_TRACE_API(143i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MARK_HEADER_AS_UNDERSTOOD: WS_TRACE_API = WS_TRACE_API(144i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FILL_BODY: WS_TRACE_API = WS_TRACE_API(145i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FLUSH_BODY: WS_TRACE_API = WS_TRACE_API(146i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REQUEST_SECURITY_TOKEN: WS_TRACE_API = WS_TRACE_API(147i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SECURITY_TOKEN_PROPERTY: WS_TRACE_API = WS_TRACE_API(148i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_XML_SECURITY_TOKEN: WS_TRACE_API = WS_TRACE_API(149i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_SECURITY_TOKEN: WS_TRACE_API = WS_TRACE_API(150i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_REVOKE_SECURITY_CONTEXT: WS_TRACE_API = WS_TRACE_API(151i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SECURITY_CONTEXT_PROPERTY: WS_TRACE_API = WS_TRACE_API(152i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ELEMENT_TYPE: WS_TRACE_API = WS_TRACE_API(153i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_ATTRIBUTE_TYPE: WS_TRACE_API = WS_TRACE_API(154i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_TYPE: WS_TRACE_API = WS_TRACE_API(155i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ELEMENT_TYPE: WS_TRACE_API = WS_TRACE_API(156i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_ATTRIBUTE_TYPE: WS_TRACE_API = WS_TRACE_API(157i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WRITE_TYPE: WS_TRACE_API = WS_TRACE_API(158i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SERVICE_REGISTER_FOR_CANCEL: WS_TRACE_API = WS_TRACE_API(159i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SERVICE_HOST_PROPERTY: WS_TRACE_API = WS_TRACE_API(160i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(161i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(162i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(163i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(164i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(165i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_SERVICE_HOST: WS_TRACE_API = WS_TRACE_API(166i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_SERVICE_PROXY_PROPERTY: WS_TRACE_API = WS_TRACE_API(167i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(168i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_OPEN_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(169i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CLOSE_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(170i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(171i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(172i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_SERVICE_PROXY: WS_TRACE_API = WS_TRACE_API(173i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ABORT_CALL: WS_TRACE_API = WS_TRACE_API(174i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CALL: WS_TRACE_API = WS_TRACE_API(175i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_DECODE_URL: WS_TRACE_API = WS_TRACE_API(176i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_ENCODE_URL: WS_TRACE_API = WS_TRACE_API(177i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_COMBINE_URL: WS_TRACE_API = WS_TRACE_API(178i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_DATETIME_TO_FILETIME: WS_TRACE_API = WS_TRACE_API(179i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FILETIME_TO_DATETIME: WS_TRACE_API = WS_TRACE_API(180i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_DUMP_MEMORY: WS_TRACE_API = WS_TRACE_API(181i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_SET_AUTOFAIL: WS_TRACE_API = WS_TRACE_API(182i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_CREATE_METADATA: WS_TRACE_API = WS_TRACE_API(183i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_READ_METADATA: WS_TRACE_API = WS_TRACE_API(184i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_FREE_METADATA: WS_TRACE_API = WS_TRACE_API(185i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_RESET_METADATA: WS_TRACE_API = WS_TRACE_API(186i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_METADATA_PROPERTY: WS_TRACE_API = WS_TRACE_API(187i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_MISSING_METADATA_DOCUMENT_ADDRESS: WS_TRACE_API = WS_TRACE_API(188i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_METADATA_ENDPOINTS: WS_TRACE_API = WS_TRACE_API(189i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_MATCH_POLICY_ALTERNATIVE: WS_TRACE_API = WS_TRACE_API(190i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_POLICY_PROPERTY: WS_TRACE_API = WS_TRACE_API(191i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_GET_POLICY_ALTERNATIVE_COUNT: WS_TRACE_API = WS_TRACE_API(192i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WS_CREATE_SERVICE_PROXY_FROM_TEMPLATE: WS_TRACE_API = WS_TRACE_API(193i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRACE_API_WS_CREATE_SERVICE_HOST_FROM_TEMPLATE: WS_TRACE_API = WS_TRACE_API(194i32);
impl ::core::marker::Copy for WS_TRACE_API {}
impl ::core::clone::Clone for WS_TRACE_API {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TRACE_API {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_TRACE_API {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_TRACE_API {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRACE_API").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_TRANSFER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STREAMED_INPUT_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STREAMED_OUTPUT_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BUFFERED_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STREAMED_TRANSFER_MODE: WS_TRANSFER_MODE = WS_TRANSFER_MODE(3i32);
impl ::core::marker::Copy for WS_TRANSFER_MODE {}
impl ::core::clone::Clone for WS_TRANSFER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TRANSFER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_TRANSFER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_TRANSFER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRANSFER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_TRUST_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRUST_VERSION_FEBRUARY_2005: WS_TRUST_VERSION = WS_TRUST_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TRUST_VERSION_1_3: WS_TRUST_VERSION = WS_TRUST_VERSION(2i32);
impl ::core::marker::Copy for WS_TRUST_VERSION {}
impl ::core::clone::Clone for WS_TRUST_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TRUST_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_TRUST_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_TRUST_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRUST_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BOOL_TYPE: WS_TYPE = WS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT8_TYPE: WS_TYPE = WS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT16_TYPE: WS_TYPE = WS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT32_TYPE: WS_TYPE = WS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT64_TYPE: WS_TYPE = WS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT8_TYPE: WS_TYPE = WS_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT16_TYPE: WS_TYPE = WS_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT32_TYPE: WS_TYPE = WS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT64_TYPE: WS_TYPE = WS_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FLOAT_TYPE: WS_TYPE = WS_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DOUBLE_TYPE: WS_TYPE = WS_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DECIMAL_TYPE: WS_TYPE = WS_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_TYPE: WS_TYPE = WS_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TIMESPAN_TYPE: WS_TYPE = WS_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_GUID_TYPE: WS_TYPE = WS_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UNIQUE_ID_TYPE: WS_TYPE = WS_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRING_TYPE: WS_TYPE = WS_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WSZ_TYPE: WS_TYPE = WS_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BYTES_TYPE: WS_TYPE = WS_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_STRING_TYPE: WS_TYPE = WS_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_QNAME_TYPE: WS_TYPE = WS_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_BUFFER_TYPE: WS_TYPE = WS_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CHAR_ARRAY_TYPE: WS_TYPE = WS_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UTF8_ARRAY_TYPE: WS_TYPE = WS_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BYTE_ARRAY_TYPE: WS_TYPE = WS_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DESCRIPTION_TYPE: WS_TYPE = WS_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRUCT_TYPE: WS_TYPE = WS_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_CUSTOM_TYPE: WS_TYPE = WS_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENDPOINT_ADDRESS_TYPE: WS_TYPE = WS_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FAULT_TYPE: WS_TYPE = WS_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_VOID_TYPE: WS_TYPE = WS_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ENUM_TYPE: WS_TYPE = WS_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DURATION_TYPE: WS_TYPE = WS_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UNION_TYPE: WS_TYPE = WS_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ATTRIBUTES_TYPE: WS_TYPE = WS_TYPE(34i32);
impl ::core::marker::Copy for WS_TYPE {}
impl ::core::clone::Clone for WS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_TYPE_MAPPING(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ATTRIBUTE_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ELEMENT_CONTENT_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_ANY_ELEMENT_TYPE_MAPPING: WS_TYPE_MAPPING = WS_TYPE_MAPPING(4i32);
impl ::core::marker::Copy for WS_TYPE_MAPPING {}
impl ::core::clone::Clone for WS_TYPE_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_TYPE_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_TYPE_MAPPING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_TYPE_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TYPE_MAPPING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_URL_SCHEME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_HTTP_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_HTTPS_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_NETTCP_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_SOAPUDP_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_URL_NETPIPE_SCHEME_TYPE: WS_URL_SCHEME_TYPE = WS_URL_SCHEME_TYPE(4i32);
impl ::core::marker::Copy for WS_URL_SCHEME_TYPE {}
impl ::core::clone::Clone for WS_URL_SCHEME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_URL_SCHEME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_URL_SCHEME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_URL_SCHEME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_URL_SCHEME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_USERNAME_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRING_USERNAME_CREDENTIAL_TYPE: WS_USERNAME_CREDENTIAL_TYPE = WS_USERNAME_CREDENTIAL_TYPE(1i32);
impl ::core::marker::Copy for WS_USERNAME_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WS_USERNAME_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_USERNAME_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_USERNAME_CREDENTIAL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_USERNAME_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_USERNAME_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_VALUE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_BOOL_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT8_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT16_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT32_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INT64_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT8_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT16_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT32_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_UINT64_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_FLOAT_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DOUBLE_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DECIMAL_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DATETIME_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_TIMESPAN_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_GUID_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DURATION_VALUE_TYPE: WS_VALUE_TYPE = WS_VALUE_TYPE(15i32);
impl ::core::marker::Copy for WS_VALUE_TYPE {}
impl ::core::clone::Clone for WS_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_VALUE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_VALUE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE = WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE = WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE = WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(3i32);
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_KERBEROS: WS_WINDOWS_INTEGRATED_AUTH_PACKAGE = WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_NTLM: WS_WINDOWS_INTEGRATED_AUTH_PACKAGE = WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_SPNEGO: WS_WINDOWS_INTEGRATED_AUTH_PACKAGE = WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(3i32);
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WINDOWS_INTEGRATED_AUTH_PACKAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_WRITE_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_REQUIRED_VALUE: WS_WRITE_OPTION = WS_WRITE_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_REQUIRED_POINTER: WS_WRITE_OPTION = WS_WRITE_OPTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_NILLABLE_VALUE: WS_WRITE_OPTION = WS_WRITE_OPTION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_WRITE_NILLABLE_POINTER: WS_WRITE_OPTION = WS_WRITE_OPTION(4i32);
impl ::core::marker::Copy for WS_WRITE_OPTION {}
impl ::core::clone::Clone for WS_WRITE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_WRITE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_WRITE_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_WRITE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WRITE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_BUFFER_PROPERTY_ID(pub i32);
impl ::core::marker::Copy for WS_XML_BUFFER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_BUFFER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_BUFFER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_BUFFER_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_BUFFER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_BUFFER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_CANONICALIZATION_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCLUSIVE_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_EXCLUSIVE_WITH_COMMENTS_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INCLUSIVE_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_INCLUSIVE_WITH_COMMENTS_XML_CANONICALIZATION_ALGORITHM: WS_XML_CANONICALIZATION_ALGORITHM = WS_XML_CANONICALIZATION_ALGORITHM(3i32);
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_ALGORITHM {}
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_CANONICALIZATION_ALGORITHM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_CANONICALIZATION_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_CANONICALIZATION_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_ALGORITHM: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_INCLUSIVE_PREFIXES: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_OMITTED_ELEMENT: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_CANONICALIZATION_PROPERTY_OUTPUT_BUFFER_SIZE: WS_XML_CANONICALIZATION_PROPERTY_ID = WS_XML_CANONICALIZATION_PROPERTY_ID(3i32);
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_CANONICALIZATION_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_CANONICALIZATION_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_NODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_ELEMENT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_TEXT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_END_ELEMENT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_COMMENT: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_CDATA: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_END_CDATA: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_EOF: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_NODE_TYPE_BOF: WS_XML_NODE_TYPE = WS_XML_NODE_TYPE(9i32);
impl ::core::marker::Copy for WS_XML_NODE_TYPE {}
impl ::core::clone::Clone for WS_XML_NODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_NODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_NODE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_NODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_NODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_READER_ENCODING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_TEXT: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_BINARY: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_MTOM: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_ENCODING_TYPE_RAW: WS_XML_READER_ENCODING_TYPE = WS_XML_READER_ENCODING_TYPE(4i32);
impl ::core::marker::Copy for WS_XML_READER_ENCODING_TYPE {}
impl ::core::clone::Clone for WS_XML_READER_ENCODING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_READER_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_ENCODING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_READER_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_ENCODING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_READER_INPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_INPUT_TYPE_BUFFER: WS_XML_READER_INPUT_TYPE = WS_XML_READER_INPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_INPUT_TYPE_STREAM: WS_XML_READER_INPUT_TYPE = WS_XML_READER_INPUT_TYPE(2i32);
impl ::core::marker::Copy for WS_XML_READER_INPUT_TYPE {}
impl ::core::clone::Clone for WS_XML_READER_INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_READER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_INPUT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_READER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_INPUT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_READER_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_DEPTH: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_ALLOW_FRAGMENT: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_ATTRIBUTES: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_READ_DECLARATION: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_CHARSET: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_ROW: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_COLUMN: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_UTF8_TRIM_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_STREAM_BUFFER_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_IN_ATTRIBUTE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_STREAM_MAX_ROOT_MIME_PART_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_STREAM_MAX_MIME_HEADERS_SIZE: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_MIME_PARTS: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_ALLOW_INVALID_CHARACTER_REFERENCES: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_READER_PROPERTY_MAX_NAMESPACES: WS_XML_READER_PROPERTY_ID = WS_XML_READER_PROPERTY_ID(14i32);
impl ::core::marker::Copy for WS_XML_READER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_READER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_READER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_SECURITY_TOKEN_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_ATTACHED_REFERENCE: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_UNATTACHED_REFERENCE: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_VALID_FROM_TIME: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_SECURITY_TOKEN_PROPERTY_VALID_TILL_TIME: WS_XML_SECURITY_TOKEN_PROPERTY_ID = WS_XML_SECURITY_TOKEN_PROPERTY_ID(4i32);
impl ::core::marker::Copy for WS_XML_SECURITY_TOKEN_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_TEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UTF8: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UTF16: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_BASE64: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_BOOL: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_INT32: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_INT64: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UINT64: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_FLOAT: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_DOUBLE: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_DECIMAL: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_GUID: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_UNIQUE_ID: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_DATETIME: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_TIMESPAN: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_QNAME: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_TEXT_TYPE_LIST: WS_XML_TEXT_TYPE = WS_XML_TEXT_TYPE(16i32);
impl ::core::marker::Copy for WS_XML_TEXT_TYPE {}
impl ::core::clone::Clone for WS_XML_TEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_TEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_TEXT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_TEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_TEXT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_WRITER_ENCODING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_TEXT: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_BINARY: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_MTOM: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_ENCODING_TYPE_RAW: WS_XML_WRITER_ENCODING_TYPE = WS_XML_WRITER_ENCODING_TYPE(4i32);
impl ::core::marker::Copy for WS_XML_WRITER_ENCODING_TYPE {}
impl ::core::clone::Clone for WS_XML_WRITER_ENCODING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_WRITER_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_ENCODING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_WRITER_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_ENCODING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_WRITER_OUTPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_OUTPUT_TYPE_BUFFER: WS_XML_WRITER_OUTPUT_TYPE = WS_XML_WRITER_OUTPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_OUTPUT_TYPE_STREAM: WS_XML_WRITER_OUTPUT_TYPE = WS_XML_WRITER_OUTPUT_TYPE(2i32);
impl ::core::marker::Copy for WS_XML_WRITER_OUTPUT_TYPE {}
impl ::core::clone::Clone for WS_XML_WRITER_OUTPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_WRITER_OUTPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_OUTPUT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_WRITER_OUTPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_OUTPUT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WS_XML_WRITER_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_DEPTH: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_ALLOW_FRAGMENT: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_ATTRIBUTES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_WRITE_DECLARATION: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_INDENT: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BUFFER_TRIM_SIZE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_CHARSET: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BUFFERS: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BUFFER_MAX_SIZE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BYTES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_IN_ATTRIBUTE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_MIME_PARTS_BUFFER_SIZE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_INITIAL_BUFFER: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_ALLOW_INVALID_CHARACTER_REFERENCES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_MAX_NAMESPACES: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BYTES_WRITTEN: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_BYTES_TO_CLOSE: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_COMPRESS_EMPTY_ELEMENTS: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub const WS_XML_WRITER_PROPERTY_EMIT_UNCOMPRESSED_EMPTY_ELEMENTS: WS_XML_WRITER_PROPERTY_ID = WS_XML_WRITER_PROPERTY_ID(18i32);
impl ::core::marker::Copy for WS_XML_WRITER_PROPERTY_ID {}
impl ::core::clone::Clone for WS_XML_WRITER_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WS_XML_WRITER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_ASSERTION {
    pub dwVersion: u32,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: *mut u8,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub Credential: WEBAUTHN_CREDENTIAL,
    pub cbUserId: u32,
    pub pbUserId: *mut u8,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
    pub dwCredLargeBlobStatus: u32,
    pub pHmacSecret: *mut WEBAUTHN_HMAC_SECRET_SALT,
}
impl ::core::marker::Copy for WEBAUTHN_ASSERTION {}
impl ::core::clone::Clone for WEBAUTHN_ASSERTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_ASSERTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_ASSERTION")
            .field("dwVersion", &self.dwVersion)
            .field("cbAuthenticatorData", &self.cbAuthenticatorData)
            .field("pbAuthenticatorData", &self.pbAuthenticatorData)
            .field("cbSignature", &self.cbSignature)
            .field("pbSignature", &self.pbSignature)
            .field("Credential", &self.Credential)
            .field("cbUserId", &self.cbUserId)
            .field("pbUserId", &self.pbUserId)
            .field("Extensions", &self.Extensions)
            .field("cbCredLargeBlob", &self.cbCredLargeBlob)
            .field("pbCredLargeBlob", &self.pbCredLargeBlob)
            .field("dwCredLargeBlobStatus", &self.dwCredLargeBlobStatus)
            .field("pHmacSecret", &self.pHmacSecret)
            .finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_ASSERTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_ASSERTION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbAuthenticatorData == other.cbAuthenticatorData && self.pbAuthenticatorData == other.pbAuthenticatorData && self.cbSignature == other.cbSignature && self.pbSignature == other.pbSignature && self.Credential == other.Credential && self.cbUserId == other.cbUserId && self.pbUserId == other.pbUserId && self.Extensions == other.Extensions && self.cbCredLargeBlob == other.cbCredLargeBlob && self.pbCredLargeBlob == other.pbCredLargeBlob && self.dwCredLargeBlobStatus == other.dwCredLargeBlobStatus && self.pHmacSecret == other.pHmacSecret
    }
}
impl ::core::cmp::Eq for WEBAUTHN_ASSERTION {}
impl ::core::default::Default for WEBAUTHN_ASSERTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub dwUserVerificationRequirement: u32,
    pub dwFlags: u32,
    pub pwszU2fAppId: ::windows::core::PCWSTR,
    pub pbU2fAppId: *mut super::super::Foundation::BOOL,
    pub pCancellationId: *mut ::windows::core::GUID,
    pub pAllowCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
    pub pHmacSecretSaltValues: *mut WEBAUTHN_HMAC_SECRET_SALT_VALUES,
    pub bBrowserInPrivateMode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS")
            .field("dwVersion", &self.dwVersion)
            .field("dwTimeoutMilliseconds", &self.dwTimeoutMilliseconds)
            .field("CredentialList", &self.CredentialList)
            .field("Extensions", &self.Extensions)
            .field("dwAuthenticatorAttachment", &self.dwAuthenticatorAttachment)
            .field("dwUserVerificationRequirement", &self.dwUserVerificationRequirement)
            .field("dwFlags", &self.dwFlags)
            .field("pwszU2fAppId", &self.pwszU2fAppId)
            .field("pbU2fAppId", &self.pbU2fAppId)
            .field("pCancellationId", &self.pCancellationId)
            .field("pAllowCredentialList", &self.pAllowCredentialList)
            .field("dwCredLargeBlobOperation", &self.dwCredLargeBlobOperation)
            .field("cbCredLargeBlob", &self.cbCredLargeBlob)
            .field("pbCredLargeBlob", &self.pbCredLargeBlob)
            .field("pHmacSecretSaltValues", &self.pHmacSecretSaltValues)
            .field("bBrowserInPrivateMode", &self.bBrowserInPrivateMode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwTimeoutMilliseconds == other.dwTimeoutMilliseconds
            && self.CredentialList == other.CredentialList
            && self.Extensions == other.Extensions
            && self.dwAuthenticatorAttachment == other.dwAuthenticatorAttachment
            && self.dwUserVerificationRequirement == other.dwUserVerificationRequirement
            && self.dwFlags == other.dwFlags
            && self.pwszU2fAppId == other.pwszU2fAppId
            && self.pbU2fAppId == other.pbU2fAppId
            && self.pCancellationId == other.pCancellationId
            && self.pAllowCredentialList == other.pAllowCredentialList
            && self.dwCredLargeBlobOperation == other.dwCredLargeBlobOperation
            && self.cbCredLargeBlob == other.cbCredLargeBlob
            && self.pbCredLargeBlob == other.pbCredLargeBlob
            && self.pHmacSecretSaltValues == other.pHmacSecretSaltValues
            && self.bBrowserInPrivateMode == other.bBrowserInPrivateMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub bRequireResidentKey: super::super::Foundation::BOOL,
    pub dwUserVerificationRequirement: u32,
    pub dwAttestationConveyancePreference: u32,
    pub dwFlags: u32,
    pub pCancellationId: *mut ::windows::core::GUID,
    pub pExcludeCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: super::super::Foundation::BOOL,
    pub bBrowserInPrivateMode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS")
            .field("dwVersion", &self.dwVersion)
            .field("dwTimeoutMilliseconds", &self.dwTimeoutMilliseconds)
            .field("CredentialList", &self.CredentialList)
            .field("Extensions", &self.Extensions)
            .field("dwAuthenticatorAttachment", &self.dwAuthenticatorAttachment)
            .field("bRequireResidentKey", &self.bRequireResidentKey)
            .field("dwUserVerificationRequirement", &self.dwUserVerificationRequirement)
            .field("dwAttestationConveyancePreference", &self.dwAttestationConveyancePreference)
            .field("dwFlags", &self.dwFlags)
            .field("pCancellationId", &self.pCancellationId)
            .field("pExcludeCredentialList", &self.pExcludeCredentialList)
            .field("dwEnterpriseAttestation", &self.dwEnterpriseAttestation)
            .field("dwLargeBlobSupport", &self.dwLargeBlobSupport)
            .field("bPreferResidentKey", &self.bPreferResidentKey)
            .field("bBrowserInPrivateMode", &self.bBrowserInPrivateMode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwTimeoutMilliseconds == other.dwTimeoutMilliseconds
            && self.CredentialList == other.CredentialList
            && self.Extensions == other.Extensions
            && self.dwAuthenticatorAttachment == other.dwAuthenticatorAttachment
            && self.bRequireResidentKey == other.bRequireResidentKey
            && self.dwUserVerificationRequirement == other.dwUserVerificationRequirement
            && self.dwAttestationConveyancePreference == other.dwAttestationConveyancePreference
            && self.dwFlags == other.dwFlags
            && self.pCancellationId == other.pCancellationId
            && self.pExcludeCredentialList == other.pExcludeCredentialList
            && self.dwEnterpriseAttestation == other.dwEnterpriseAttestation
            && self.dwLargeBlobSupport == other.dwLargeBlobSupport
            && self.bPreferResidentKey == other.bPreferResidentKey
            && self.bBrowserInPrivateMode == other.bBrowserInPrivateMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub pwszHashAlgId: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_CLIENT_DATA {}
impl ::core::clone::Clone for WEBAUTHN_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CLIENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CLIENT_DATA").field("dwVersion", &self.dwVersion).field("cbClientDataJSON", &self.cbClientDataJSON).field("pbClientDataJSON", &self.pbClientDataJSON).field("pwszHashAlgId", &self.pwszHashAlgId).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CLIENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CLIENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbClientDataJSON == other.cbClientDataJSON && self.pbClientDataJSON == other.pbClientDataJSON && self.pwszHashAlgId == other.pwszHashAlgId
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CLIENT_DATA {}
impl ::core::default::Default for WEBAUTHN_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: ::windows::core::PCWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub cX5c: u32,
    pub pX5c: *mut WEBAUTHN_X5C,
    pub pwszVer: ::windows::core::PCWSTR,
    pub cbCertInfo: u32,
    pub pbCertInfo: *mut u8,
    pub cbPubArea: u32,
    pub pbPubArea: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_COMMON_ATTESTATION {}
impl ::core::clone::Clone for WEBAUTHN_COMMON_ATTESTATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_COMMON_ATTESTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COMMON_ATTESTATION").field("dwVersion", &self.dwVersion).field("pwszAlg", &self.pwszAlg).field("lAlg", &self.lAlg).field("cbSignature", &self.cbSignature).field("pbSignature", &self.pbSignature).field("cX5c", &self.cX5c).field("pX5c", &self.pX5c).field("pwszVer", &self.pwszVer).field("cbCertInfo", &self.cbCertInfo).field("pbCertInfo", &self.pbCertInfo).field("cbPubArea", &self.cbPubArea).field("pbPubArea", &self.pbPubArea).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_COMMON_ATTESTATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_COMMON_ATTESTATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszAlg == other.pwszAlg && self.lAlg == other.lAlg && self.cbSignature == other.cbSignature && self.pbSignature == other.pbSignature && self.cX5c == other.cX5c && self.pX5c == other.pX5c && self.pwszVer == other.pwszVer && self.cbCertInfo == other.cbCertInfo && self.pbCertInfo == other.pbCertInfo && self.cbPubArea == other.cbPubArea && self.pbPubArea == other.pbPubArea
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COMMON_ATTESTATION {}
impl ::core::default::Default for WEBAUTHN_COMMON_ATTESTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: ::windows::core::PCWSTR,
    pub lAlg: i32,
}
impl ::core::marker::Copy for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {}
impl ::core::clone::Clone for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COSE_CREDENTIAL_PARAMETER").field("dwVersion", &self.dwVersion).field("pwszCredentialType", &self.pwszCredentialType).field("lAlg", &self.lAlg).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszCredentialType == other.pwszCredentialType && self.lAlg == other.lAlg
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {}
impl ::core::default::Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: u32,
    pub pCredentialParameters: *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
impl ::core::marker::Copy for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {}
impl ::core::clone::Clone for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COSE_CREDENTIAL_PARAMETERS").field("cCredentialParameters", &self.cCredentialParameters).field("pCredentialParameters", &self.pCredentialParameters).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentialParameters == other.cCredentialParameters && self.pCredentialParameters == other.pCredentialParameters
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {}
impl ::core::default::Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszCredentialType", &self.pwszCredentialType).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.pbId == other.pbId && self.pwszCredentialType == other.pwszCredentialType
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL {}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: u32,
    pub pCredentials: *mut WEBAUTHN_CREDENTIAL,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIALS {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIALS").field("cCredentials", &self.cCredentials).field("pCredentials", &self.pCredentials).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIALS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentials == other.cCredentials && self.pCredentials == other.pCredentials
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIALS {}
impl ::core::default::Default for WEBAUTHN_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: ::windows::core::PCWSTR,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: *mut u8,
    pub cbAttestation: u32,
    pub pbAttestation: *mut u8,
    pub dwAttestationDecodeType: u32,
    pub pvAttestationDecode: *mut ::core::ffi::c_void,
    pub cbAttestationObject: u32,
    pub pbAttestationObject: *mut u8,
    pub cbCredentialId: u32,
    pub pbCredentialId: *mut u8,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwUsedTransport: u32,
    pub bEpAtt: super::super::Foundation::BOOL,
    pub bLargeBlobSupported: super::super::Foundation::BOOL,
    pub bResidentKey: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_ATTESTATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_ATTESTATION")
            .field("dwVersion", &self.dwVersion)
            .field("pwszFormatType", &self.pwszFormatType)
            .field("cbAuthenticatorData", &self.cbAuthenticatorData)
            .field("pbAuthenticatorData", &self.pbAuthenticatorData)
            .field("cbAttestation", &self.cbAttestation)
            .field("pbAttestation", &self.pbAttestation)
            .field("dwAttestationDecodeType", &self.dwAttestationDecodeType)
            .field("pvAttestationDecode", &self.pvAttestationDecode)
            .field("cbAttestationObject", &self.cbAttestationObject)
            .field("pbAttestationObject", &self.pbAttestationObject)
            .field("cbCredentialId", &self.cbCredentialId)
            .field("pbCredentialId", &self.pbCredentialId)
            .field("Extensions", &self.Extensions)
            .field("dwUsedTransport", &self.dwUsedTransport)
            .field("bEpAtt", &self.bEpAtt)
            .field("bLargeBlobSupported", &self.bLargeBlobSupported)
            .field("bResidentKey", &self.bResidentKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIAL_ATTESTATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.pwszFormatType == other.pwszFormatType
            && self.cbAuthenticatorData == other.cbAuthenticatorData
            && self.pbAuthenticatorData == other.pbAuthenticatorData
            && self.cbAttestation == other.cbAttestation
            && self.pbAttestation == other.pbAttestation
            && self.dwAttestationDecodeType == other.dwAttestationDecodeType
            && self.pvAttestationDecode == other.pvAttestationDecode
            && self.cbAttestationObject == other.cbAttestationObject
            && self.pbAttestationObject == other.pbAttestationObject
            && self.cbCredentialId == other.cbCredentialId
            && self.pbCredentialId == other.pbCredentialId
            && self.Extensions == other.Extensions
            && self.dwUsedTransport == other.dwUsedTransport
            && self.bEpAtt == other.bEpAtt
            && self.bLargeBlobSupported == other.bLargeBlobSupported
            && self.bResidentKey == other.bResidentKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_ATTESTATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_DETAILS {
    pub dwVersion: u32,
    pub cbCredentialID: u32,
    pub pbCredentialID: *mut u8,
    pub pRpInformation: *mut WEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub bRemovable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_DETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_DETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_DETAILS").field("dwVersion", &self.dwVersion).field("cbCredentialID", &self.cbCredentialID).field("pbCredentialID", &self.pbCredentialID).field("pRpInformation", &self.pRpInformation).field("pUserInformation", &self.pUserInformation).field("bRemovable", &self.bRemovable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIAL_DETAILS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_DETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbCredentialID == other.cbCredentialID && self.pbCredentialID == other.pbCredentialID && self.pRpInformation == other.pRpInformation && self.pUserInformation == other.pUserInformation && self.bRemovable == other.bRemovable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_DETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    pub cCredentialDetails: u32,
    pub ppCredentialDetails: *mut *mut WEBAUTHN_CREDENTIAL_DETAILS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_DETAILS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_DETAILS_LIST").field("cCredentialDetails", &self.cCredentialDetails).field("ppCredentialDetails", &self.ppCredentialDetails).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentialDetails == other.cCredentialDetails && self.ppCredentialDetails == other.ppCredentialDetails
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_DETAILS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: ::windows::core::PCWSTR,
    pub dwTransports: u32,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_EX {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_EX").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszCredentialType", &self.pwszCredentialType).field("dwTransports", &self.dwTransports).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIAL_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.pbId == other.pbId && self.pwszCredentialType == other.pwszCredentialType && self.dwTransports == other.dwTransports
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_EX {}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut *mut WEBAUTHN_CREDENTIAL_EX,
}
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_LIST {}
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_LIST").field("cCredentials", &self.cCredentials).field("ppCredentials", &self.ppCredentials).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CREDENTIAL_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentials == other.cCredentials && self.ppCredentials == other.ppCredentials
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_LIST {}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION {
    pub cbCredBlob: u32,
    pub pbCredBlob: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_CRED_BLOB_EXTENSION {}
impl ::core::clone::Clone for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_BLOB_EXTENSION").field("cbCredBlob", &self.cbCredBlob).field("pbCredBlob", &self.pbCredBlob).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CRED_BLOB_EXTENSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.cbCredBlob == other.cbCredBlob && self.pbCredBlob == other.pbCredBlob
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CRED_BLOB_EXTENSION {}
impl ::core::default::Default for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: u32,
    pub bRequireCredProtect: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_PROTECT_EXTENSION_IN").field("dwCredProtect", &self.dwCredProtect).field("bRequireCredProtect", &self.bRequireCredProtect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn eq(&self, other: &Self) -> bool {
        self.dwCredProtect == other.dwCredProtect && self.bRequireCredProtect == other.bRequireCredProtect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    pub cbCredID: u32,
    pub pbCredID: *mut u8,
    pub pHmacSecretSalt: *mut WEBAUTHN_HMAC_SECRET_SALT,
}
impl ::core::marker::Copy for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {}
impl ::core::clone::Clone for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT").field("cbCredID", &self.cbCredID).field("pbCredID", &self.pbCredID).field("pHmacSecretSalt", &self.pHmacSecretSalt).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    fn eq(&self, other: &Self) -> bool {
        self.cbCredID == other.cbCredID && self.pbCredID == other.pbCredID && self.pHmacSecretSalt == other.pHmacSecretSalt
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {}
impl ::core::default::Default for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: ::windows::core::PCWSTR,
    pub cbExtension: u32,
    pub pvExtension: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WEBAUTHN_EXTENSION {}
impl ::core::clone::Clone for WEBAUTHN_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_EXTENSION").field("pwszExtensionIdentifier", &self.pwszExtensionIdentifier).field("cbExtension", &self.cbExtension).field("pvExtension", &self.pvExtension).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_EXTENSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.pwszExtensionIdentifier == other.pwszExtensionIdentifier && self.cbExtension == other.cbExtension && self.pvExtension == other.pvExtension
    }
}
impl ::core::cmp::Eq for WEBAUTHN_EXTENSION {}
impl ::core::default::Default for WEBAUTHN_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: *mut WEBAUTHN_EXTENSION,
}
impl ::core::marker::Copy for WEBAUTHN_EXTENSIONS {}
impl ::core::clone::Clone for WEBAUTHN_EXTENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_EXTENSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_EXTENSIONS").field("cExtensions", &self.cExtensions).field("pExtensions", &self.pExtensions).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_EXTENSIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_EXTENSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cExtensions == other.cExtensions && self.pExtensions == other.pExtensions
    }
}
impl ::core::cmp::Eq for WEBAUTHN_EXTENSIONS {}
impl ::core::default::Default for WEBAUTHN_EXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    pub dwVersion: u32,
    pub pwszRpId: ::windows::core::PCWSTR,
    pub bBrowserInPrivateMode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_GET_CREDENTIALS_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_GET_CREDENTIALS_OPTIONS").field("dwVersion", &self.dwVersion).field("pwszRpId", &self.pwszRpId).field("bBrowserInPrivateMode", &self.bBrowserInPrivateMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszRpId == other.pwszRpId && self.bBrowserInPrivateMode == other.bBrowserInPrivateMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_GET_CREDENTIALS_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_HMAC_SECRET_SALT {
    pub cbFirst: u32,
    pub pbFirst: *mut u8,
    pub cbSecond: u32,
    pub pbSecond: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_HMAC_SECRET_SALT {}
impl ::core::clone::Clone for WEBAUTHN_HMAC_SECRET_SALT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_HMAC_SECRET_SALT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_HMAC_SECRET_SALT").field("cbFirst", &self.cbFirst).field("pbFirst", &self.pbFirst).field("cbSecond", &self.cbSecond).field("pbSecond", &self.pbSecond).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_HMAC_SECRET_SALT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_HMAC_SECRET_SALT {
    fn eq(&self, other: &Self) -> bool {
        self.cbFirst == other.cbFirst && self.pbFirst == other.pbFirst && self.cbSecond == other.cbSecond && self.pbSecond == other.pbSecond
    }
}
impl ::core::cmp::Eq for WEBAUTHN_HMAC_SECRET_SALT {}
impl ::core::default::Default for WEBAUTHN_HMAC_SECRET_SALT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    pub pGlobalHmacSalt: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub cCredWithHmacSecretSaltList: u32,
    pub pCredWithHmacSecretSaltList: *mut WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT,
}
impl ::core::marker::Copy for WEBAUTHN_HMAC_SECRET_SALT_VALUES {}
impl ::core::clone::Clone for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_HMAC_SECRET_SALT_VALUES").field("pGlobalHmacSalt", &self.pGlobalHmacSalt).field("cCredWithHmacSecretSaltList", &self.cCredWithHmacSecretSaltList).field("pCredWithHmacSecretSaltList", &self.pCredWithHmacSecretSaltList).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    fn eq(&self, other: &Self) -> bool {
        self.pGlobalHmacSalt == other.pGlobalHmacSalt && self.cCredWithHmacSecretSaltList == other.cCredWithHmacSecretSaltList && self.pCredWithHmacSecretSaltList == other.pCredWithHmacSecretSaltList
    }
}
impl ::core::cmp::Eq for WEBAUTHN_HMAC_SECRET_SALT_VALUES {}
impl ::core::default::Default for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: ::windows::core::PCWSTR,
    pub pwszName: ::windows::core::PCWSTR,
    pub pwszIcon: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_RP_ENTITY_INFORMATION {}
impl ::core::clone::Clone for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_RP_ENTITY_INFORMATION").field("dwVersion", &self.dwVersion).field("pwszId", &self.pwszId).field("pwszName", &self.pwszName).field("pwszIcon", &self.pwszIcon).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_RP_ENTITY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszId == other.pwszId && self.pwszName == other.pwszName && self.pwszIcon == other.pwszIcon
    }
}
impl ::core::cmp::Eq for WEBAUTHN_RP_ENTITY_INFORMATION {}
impl ::core::default::Default for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszName: ::windows::core::PCWSTR,
    pub pwszIcon: ::windows::core::PCWSTR,
    pub pwszDisplayName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WEBAUTHN_USER_ENTITY_INFORMATION {}
impl ::core::clone::Clone for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_USER_ENTITY_INFORMATION").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszName", &self.pwszName).field("pwszIcon", &self.pwszIcon).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_USER_ENTITY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.pbId == other.pbId && self.pwszName == other.pwszName && self.pwszIcon == other.pwszIcon && self.pwszDisplayName == other.pwszDisplayName
    }
}
impl ::core::cmp::Eq for WEBAUTHN_USER_ENTITY_INFORMATION {}
impl ::core::default::Default for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WEBAUTHN_X5C {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for WEBAUTHN_X5C {}
impl ::core::clone::Clone for WEBAUTHN_X5C {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEBAUTHN_X5C {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_X5C").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::windows::core::TypeKind for WEBAUTHN_X5C {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEBAUTHN_X5C {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for WEBAUTHN_X5C {}
impl ::core::default::Default for WEBAUTHN_X5C {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ANY_ATTRIBUTE {
    pub localName: WS_XML_STRING,
    pub ns: WS_XML_STRING,
    pub value: *mut WS_XML_TEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ANY_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ANY_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ANY_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ANY_ATTRIBUTE").field("localName", &self.localName).field("ns", &self.ns).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ANY_ATTRIBUTE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ANY_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.localName == other.localName && self.ns == other.ns && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ANY_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ANY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ANY_ATTRIBUTES {
    pub attributes: *mut WS_ANY_ATTRIBUTE,
    pub attributeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ANY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ANY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ANY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ANY_ATTRIBUTES").field("attributes", &self.attributes).field("attributeCount", &self.attributeCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ANY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ANY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.attributes == other.attributes && self.attributeCount == other.attributeCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ANY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ANY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ASYNC_CONTEXT {
    pub callback: WS_ASYNC_CALLBACK,
    pub callbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_ASYNC_CONTEXT {}
impl ::core::clone::Clone for WS_ASYNC_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ASYNC_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_CONTEXT").field("callbackState", &self.callbackState).finish()
    }
}
impl ::windows::core::TypeKind for WS_ASYNC_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_ASYNC_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ASYNC_OPERATION {
    pub function: WS_ASYNC_FUNCTION,
}
impl ::core::marker::Copy for WS_ASYNC_OPERATION {}
impl ::core::clone::Clone for WS_ASYNC_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ASYNC_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_OPERATION").finish()
    }
}
impl ::windows::core::TypeKind for WS_ASYNC_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_ASYNC_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ASYNC_STATE {
    pub internal0: *mut ::core::ffi::c_void,
    pub internal1: *mut ::core::ffi::c_void,
    pub internal2: *mut ::core::ffi::c_void,
    pub internal3: *mut ::core::ffi::c_void,
    pub internal4: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_ASYNC_STATE {}
impl ::core::clone::Clone for WS_ASYNC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ASYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_STATE").field("internal0", &self.internal0).field("internal1", &self.internal1).field("internal2", &self.internal2).field("internal3", &self.internal3).field("internal4", &self.internal4).finish()
    }
}
impl ::windows::core::TypeKind for WS_ASYNC_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_ASYNC_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.internal0 == other.internal0 && self.internal1 == other.internal1 && self.internal2 == other.internal2 && self.internal3 == other.internal3 && self.internal4 == other.internal4
    }
}
impl ::core::cmp::Eq for WS_ASYNC_STATE {}
impl ::core::default::Default for WS_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ATTRIBUTE_DESCRIPTION {
    pub attributeLocalName: *mut WS_XML_STRING,
    pub attributeNs: *mut WS_XML_STRING,
    pub r#type: WS_TYPE,
    pub typeDescription: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ATTRIBUTE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ATTRIBUTE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ATTRIBUTE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ATTRIBUTE_DESCRIPTION").field("attributeLocalName", &self.attributeLocalName).field("attributeNs", &self.attributeNs).field("type", &self.r#type).field("typeDescription", &self.typeDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ATTRIBUTE_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ATTRIBUTE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.attributeLocalName == other.attributeLocalName && self.attributeNs == other.attributeNs && self.r#type == other.r#type && self.typeDescription == other.typeDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ATTRIBUTE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ATTRIBUTE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_BOOL_DESCRIPTION {
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_BOOL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_BOOL_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_BOOL_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BOOL_DESCRIPTION").field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_BOOL_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_BOOL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_BOOL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_BOOL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BUFFERS {
    pub bufferCount: u32,
    pub buffers: *mut WS_BYTES,
}
impl ::core::marker::Copy for WS_BUFFERS {}
impl ::core::clone::Clone for WS_BUFFERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BUFFERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BUFFERS").field("bufferCount", &self.bufferCount).field("buffers", &self.buffers).finish()
    }
}
impl ::windows::core::TypeKind for WS_BUFFERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_BUFFERS {
    fn eq(&self, other: &Self) -> bool {
        self.bufferCount == other.bufferCount && self.buffers == other.buffers
    }
}
impl ::core::cmp::Eq for WS_BUFFERS {}
impl ::core::default::Default for WS_BUFFERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BYTES {
    pub length: u32,
    pub bytes: *mut u8,
}
impl ::core::marker::Copy for WS_BYTES {}
impl ::core::clone::Clone for WS_BYTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BYTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTES").field("length", &self.length).field("bytes", &self.bytes).finish()
    }
}
impl ::windows::core::TypeKind for WS_BYTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_BYTES {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.bytes == other.bytes
    }
}
impl ::core::cmp::Eq for WS_BYTES {}
impl ::core::default::Default for WS_BYTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BYTES_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_BYTES_DESCRIPTION {}
impl ::core::clone::Clone for WS_BYTES_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BYTES_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTES_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_BYTES_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_BYTES_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_BYTES_DESCRIPTION {}
impl ::core::default::Default for WS_BYTES_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_BYTE_ARRAY_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_BYTE_ARRAY_DESCRIPTION {}
impl ::core::clone::Clone for WS_BYTE_ARRAY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_BYTE_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTE_ARRAY_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_BYTE_ARRAY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_BYTE_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_BYTE_ARRAY_DESCRIPTION {}
impl ::core::default::Default for WS_BYTE_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CALL_PROPERTY {
    pub id: WS_CALL_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_CALL_PROPERTY {}
impl ::core::clone::Clone for WS_CALL_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CALL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CALL_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_CALL_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CALL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_CALL_PROPERTY {}
impl ::core::default::Default for WS_CALL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub provider: usize,
    pub keySpec: u32,
}
impl ::core::marker::Copy for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("provider", &self.provider).field("keySpec", &self.keySpec).finish()
    }
}
impl ::windows::core::TypeKind for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandle == other.keyHandle && self.provider == other.provider && self.keySpec == other.keySpec
    }
}
impl ::core::cmp::Eq for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::default::Default for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    pub callback: WS_CERTIFICATE_VALIDATION_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT").field("state", &self.state).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CERT_CREDENTIAL {
    pub credentialType: WS_CERT_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
impl ::windows::core::TypeKind for WS_CERT_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credentialType == other.credentialType
    }
}
impl ::core::cmp::Eq for WS_CERT_CREDENTIAL {}
impl ::core::default::Default for WS_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CERT_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub rawCertificateData: WS_BYTES,
}
impl ::core::marker::Copy for WS_CERT_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_CERT_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CERT_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_ENDPOINT_IDENTITY").field("identity", &self.identity).field("rawCertificateData", &self.rawCertificateData).finish()
    }
}
impl ::windows::core::TypeKind for WS_CERT_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CERT_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.rawCertificateData == other.rawCertificateData
    }
}
impl ::core::cmp::Eq for WS_CERT_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_CERT_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::windows::core::TypeKind for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    pub authenticator: WS_SAML_AUTHENTICATOR,
    pub trustedIssuerCerts: *const *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub trustedIssuerCertCount: u32,
    pub decryptionCert: *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub samlValidator: WS_VALIDATE_SAML_CALLBACK,
    pub samlValidatorCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WS_CERT_SIGNED_SAML_AUTHENTICATOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_SIGNED_SAML_AUTHENTICATOR").field("authenticator", &self.authenticator).field("trustedIssuerCerts", &self.trustedIssuerCerts).field("trustedIssuerCertCount", &self.trustedIssuerCertCount).field("decryptionCert", &self.decryptionCert).field("samlValidatorCallbackState", &self.samlValidatorCallbackState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_CHANNEL(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_DECODER {
    pub createContext: *mut ::core::ffi::c_void,
    pub createDecoderCallback: WS_CREATE_DECODER_CALLBACK,
    pub decoderGetContentTypeCallback: WS_DECODER_GET_CONTENT_TYPE_CALLBACK,
    pub decoderStartCallback: WS_DECODER_START_CALLBACK,
    pub decoderDecodeCallback: WS_DECODER_DECODE_CALLBACK,
    pub decoderEndCallback: WS_DECODER_END_CALLBACK,
    pub freeDecoderCallback: WS_FREE_DECODER_CALLBACK,
}
impl ::core::marker::Copy for WS_CHANNEL_DECODER {}
impl ::core::clone::Clone for WS_CHANNEL_DECODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_DECODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_DECODER").field("createContext", &self.createContext).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_DECODER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_CHANNEL_DECODER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_ENCODER {
    pub createContext: *mut ::core::ffi::c_void,
    pub createEncoderCallback: WS_CREATE_ENCODER_CALLBACK,
    pub encoderGetContentTypeCallback: WS_ENCODER_GET_CONTENT_TYPE_CALLBACK,
    pub encoderStartCallback: WS_ENCODER_START_CALLBACK,
    pub encoderEncodeCallback: WS_ENCODER_ENCODE_CALLBACK,
    pub encoderEndCallback: WS_ENCODER_END_CALLBACK,
    pub freeEncoderCallback: WS_FREE_ENCODER_CALLBACK,
}
impl ::core::marker::Copy for WS_CHANNEL_ENCODER {}
impl ::core::clone::Clone for WS_CHANNEL_ENCODER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_ENCODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_ENCODER").field("createContext", &self.createContext).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_ENCODER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_CHANNEL_ENCODER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTIES {
    pub properties: *mut WS_CHANNEL_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTIES {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTIES {}
impl ::core::default::Default for WS_CHANNEL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTY {
    pub id: WS_CHANNEL_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY {}
impl ::core::default::Default for WS_CHANNEL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTY_CONSTRAINT {
    pub id: WS_CHANNEL_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_CHANNEL_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_PROPERTY_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    pub channelProperty: WS_CHANNEL_PROPERTY,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY_CONSTRAINT_0").field("channelProperty", &self.channelProperty).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperty == other.channelProperty
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CHAR_ARRAY_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_CHAR_ARRAY_DESCRIPTION {}
impl ::core::clone::Clone for WS_CHAR_ARRAY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CHAR_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHAR_ARRAY_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_CHAR_ARRAY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CHAR_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_CHAR_ARRAY_DESCRIPTION {}
impl ::core::default::Default for WS_CHAR_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CONTRACT_DESCRIPTION {
    pub operationCount: u32,
    pub operations: *mut *mut WS_OPERATION_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CONTRACT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CONTRACT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_CONTRACT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CONTRACT_DESCRIPTION").field("operationCount", &self.operationCount).field("operations", &self.operations).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_CONTRACT_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_CONTRACT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.operationCount == other.operationCount && self.operations == other.operations
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_CONTRACT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_CONTRACT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub struct WS_CUSTOM_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub getCertCallback: WS_GET_CERT_CALLBACK,
    pub getCertCallbackState: *mut ::core::ffi::c_void,
    pub certIssuerListNotificationCallback: WS_CERT_ISSUER_LIST_NOTIFICATION_CALLBACK,
    pub certIssuerListNotificationCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WS_CUSTOM_CERT_CREDENTIAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WS_CUSTOM_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WS_CUSTOM_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_CERT_CREDENTIAL").field("credential", &self.credential).field("getCertCallbackState", &self.getCertCallbackState).field("certIssuerListNotificationCallbackState", &self.certIssuerListNotificationCallbackState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for WS_CUSTOM_CERT_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CUSTOM_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CUSTOM_CHANNEL_CALLBACKS {
    pub createChannelCallback: WS_CREATE_CHANNEL_CALLBACK,
    pub freeChannelCallback: WS_FREE_CHANNEL_CALLBACK,
    pub resetChannelCallback: WS_RESET_CHANNEL_CALLBACK,
    pub openChannelCallback: WS_OPEN_CHANNEL_CALLBACK,
    pub closeChannelCallback: WS_CLOSE_CHANNEL_CALLBACK,
    pub abortChannelCallback: WS_ABORT_CHANNEL_CALLBACK,
    pub getChannelPropertyCallback: WS_GET_CHANNEL_PROPERTY_CALLBACK,
    pub setChannelPropertyCallback: WS_SET_CHANNEL_PROPERTY_CALLBACK,
    pub writeMessageStartCallback: WS_WRITE_MESSAGE_START_CALLBACK,
    pub writeMessageEndCallback: WS_WRITE_MESSAGE_END_CALLBACK,
    pub readMessageStartCallback: WS_READ_MESSAGE_START_CALLBACK,
    pub readMessageEndCallback: WS_READ_MESSAGE_END_CALLBACK,
    pub abandonMessageCallback: WS_ABANDON_MESSAGE_CALLBACK,
    pub shutdownSessionChannelCallback: WS_SHUTDOWN_SESSION_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_CUSTOM_CHANNEL_CALLBACKS {}
impl ::core::clone::Clone for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_CHANNEL_CALLBACKS").finish()
    }
}
impl ::windows::core::TypeKind for WS_CUSTOM_CHANNEL_CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CUSTOM_HTTP_PROXY {
    pub servers: WS_STRING,
    pub bypass: WS_STRING,
}
impl ::core::marker::Copy for WS_CUSTOM_HTTP_PROXY {}
impl ::core::clone::Clone for WS_CUSTOM_HTTP_PROXY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CUSTOM_HTTP_PROXY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_HTTP_PROXY").field("servers", &self.servers).field("bypass", &self.bypass).finish()
    }
}
impl ::windows::core::TypeKind for WS_CUSTOM_HTTP_PROXY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_CUSTOM_HTTP_PROXY {
    fn eq(&self, other: &Self) -> bool {
        self.servers == other.servers && self.bypass == other.bypass
    }
}
impl ::core::cmp::Eq for WS_CUSTOM_HTTP_PROXY {}
impl ::core::default::Default for WS_CUSTOM_HTTP_PROXY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_CUSTOM_LISTENER_CALLBACKS {
    pub createListenerCallback: WS_CREATE_LISTENER_CALLBACK,
    pub freeListenerCallback: WS_FREE_LISTENER_CALLBACK,
    pub resetListenerCallback: WS_RESET_LISTENER_CALLBACK,
    pub openListenerCallback: WS_OPEN_LISTENER_CALLBACK,
    pub closeListenerCallback: WS_CLOSE_LISTENER_CALLBACK,
    pub abortListenerCallback: WS_ABORT_LISTENER_CALLBACK,
    pub getListenerPropertyCallback: WS_GET_LISTENER_PROPERTY_CALLBACK,
    pub setListenerPropertyCallback: WS_SET_LISTENER_PROPERTY_CALLBACK,
    pub createChannelForListenerCallback: WS_CREATE_CHANNEL_FOR_LISTENER_CALLBACK,
    pub acceptChannelCallback: WS_ACCEPT_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_CUSTOM_LISTENER_CALLBACKS {}
impl ::core::clone::Clone for WS_CUSTOM_LISTENER_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_CUSTOM_LISTENER_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_LISTENER_CALLBACKS").finish()
    }
}
impl ::windows::core::TypeKind for WS_CUSTOM_LISTENER_CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_CUSTOM_LISTENER_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CUSTOM_TYPE_DESCRIPTION {
    pub size: u32,
    pub alignment: u32,
    pub readCallback: WS_READ_TYPE_CALLBACK,
    pub writeCallback: WS_WRITE_TYPE_CALLBACK,
    pub descriptionData: *mut ::core::ffi::c_void,
    pub isDefaultValueCallback: WS_IS_DEFAULT_VALUE_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CUSTOM_TYPE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CUSTOM_TYPE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_CUSTOM_TYPE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_TYPE_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("descriptionData", &self.descriptionData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_CUSTOM_TYPE_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_CUSTOM_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DATETIME {
    pub ticks: u64,
    pub format: WS_DATETIME_FORMAT,
}
impl ::core::marker::Copy for WS_DATETIME {}
impl ::core::clone::Clone for WS_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DATETIME").field("ticks", &self.ticks).field("format", &self.format).finish()
    }
}
impl ::windows::core::TypeKind for WS_DATETIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks && self.format == other.format
    }
}
impl ::core::cmp::Eq for WS_DATETIME {}
impl ::core::default::Default for WS_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DATETIME_DESCRIPTION {
    pub minValue: WS_DATETIME,
    pub maxValue: WS_DATETIME,
}
impl ::core::marker::Copy for WS_DATETIME_DESCRIPTION {}
impl ::core::clone::Clone for WS_DATETIME_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DATETIME_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DATETIME_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_DATETIME_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DATETIME_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_DATETIME_DESCRIPTION {}
impl ::core::default::Default for WS_DATETIME_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DECIMAL_DESCRIPTION {
    pub minValue: super::super::Foundation::DECIMAL,
    pub maxValue: super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DECIMAL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DECIMAL_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_DECIMAL_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DECIMAL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DEFAULT_VALUE {
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_DEFAULT_VALUE {}
impl ::core::clone::Clone for WS_DEFAULT_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DEFAULT_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DEFAULT_VALUE").field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_DEFAULT_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DEFAULT_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_DEFAULT_VALUE {}
impl ::core::default::Default for WS_DEFAULT_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).finish()
    }
}
impl ::windows::core::TypeKind for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential
    }
}
impl ::core::cmp::Eq for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    pub subStringCount: u32,
    pub subStrings: *mut *mut WS_STRING,
}
impl ::core::marker::Copy for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {}
impl ::core::clone::Clone for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DISALLOWED_USER_AGENT_SUBSTRINGS").field("subStringCount", &self.subStringCount).field("subStrings", &self.subStrings).finish()
    }
}
impl ::windows::core::TypeKind for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn eq(&self, other: &Self) -> bool {
        self.subStringCount == other.subStringCount && self.subStrings == other.subStrings
    }
}
impl ::core::cmp::Eq for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {}
impl ::core::default::Default for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DNS_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub dns: WS_STRING,
}
impl ::core::marker::Copy for WS_DNS_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_DNS_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DNS_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DNS_ENDPOINT_IDENTITY").field("identity", &self.identity).field("dns", &self.dns).finish()
    }
}
impl ::windows::core::TypeKind for WS_DNS_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DNS_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.dns == other.dns
    }
}
impl ::core::cmp::Eq for WS_DNS_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_DNS_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_DOUBLE_DESCRIPTION {
    pub minValue: f64,
    pub maxValue: f64,
}
impl ::core::marker::Copy for WS_DOUBLE_DESCRIPTION {}
impl ::core::clone::Clone for WS_DOUBLE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_DOUBLE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DOUBLE_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_DOUBLE_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_DOUBLE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_DOUBLE_DESCRIPTION {}
impl ::core::default::Default for WS_DOUBLE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DURATION {
    pub negative: super::super::Foundation::BOOL,
    pub years: u32,
    pub months: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
    pub ticks: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DURATION").field("negative", &self.negative).field("years", &self.years).field("months", &self.months).field("days", &self.days).field("hours", &self.hours).field("minutes", &self.minutes).field("seconds", &self.seconds).field("milliseconds", &self.milliseconds).field("ticks", &self.ticks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_DURATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative && self.years == other.years && self.months == other.months && self.days == other.days && self.hours == other.hours && self.minutes == other.minutes && self.seconds == other.seconds && self.milliseconds == other.milliseconds && self.ticks == other.ticks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DURATION_DESCRIPTION {
    pub minValue: WS_DURATION,
    pub maxValue: WS_DURATION,
    pub comparer: WS_DURATION_COMPARISON_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DURATION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DURATION_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_DURATION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DURATION_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_DURATION_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DURATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ELEMENT_DESCRIPTION {
    pub elementLocalName: *mut WS_XML_STRING,
    pub elementNs: *mut WS_XML_STRING,
    pub r#type: WS_TYPE,
    pub typeDescription: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ELEMENT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ELEMENT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ELEMENT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ELEMENT_DESCRIPTION").field("elementLocalName", &self.elementLocalName).field("elementNs", &self.elementNs).field("type", &self.r#type).field("typeDescription", &self.typeDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ELEMENT_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ELEMENT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.elementLocalName == other.elementLocalName && self.elementNs == other.elementNs && self.r#type == other.r#type && self.typeDescription == other.typeDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ELEMENT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ELEMENT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ENDPOINT_ADDRESS {
    pub url: WS_STRING,
    pub headers: *mut WS_XML_BUFFER,
    pub extensions: *mut WS_XML_BUFFER,
    pub identity: *mut WS_ENDPOINT_IDENTITY,
}
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_ADDRESS").field("url", &self.url).field("headers", &self.headers).field("extensions", &self.extensions).field("identity", &self.identity).finish()
    }
}
impl ::windows::core::TypeKind for WS_ENDPOINT_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.headers == other.headers && self.extensions == other.extensions && self.identity == other.identity
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_ADDRESS {}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ENDPOINT_ADDRESS_DESCRIPTION {
    pub addressingVersion: WS_ADDRESSING_VERSION,
}
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS_DESCRIPTION {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_ADDRESS_DESCRIPTION").field("addressingVersion", &self.addressingVersion).finish()
    }
}
impl ::windows::core::TypeKind for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.addressingVersion == other.addressingVersion
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_ADDRESS_DESCRIPTION {}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ENDPOINT_IDENTITY {
    pub identityType: WS_ENDPOINT_IDENTITY_TYPE,
}
impl ::core::marker::Copy for WS_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_IDENTITY").field("identityType", &self.identityType).finish()
    }
}
impl ::windows::core::TypeKind for WS_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identityType == other.identityType
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENDPOINT_POLICY_EXTENSION {
    pub policyExtension: WS_POLICY_EXTENSION,
    pub assertionName: *mut WS_XML_STRING,
    pub assertionNs: *mut WS_XML_STRING,
    pub out: WS_ENDPOINT_POLICY_EXTENSION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENDPOINT_POLICY_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENDPOINT_POLICY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENDPOINT_POLICY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_POLICY_EXTENSION").field("policyExtension", &self.policyExtension).field("assertionName", &self.assertionName).field("assertionNs", &self.assertionNs).field("out", &self.out).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ENDPOINT_POLICY_EXTENSION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENDPOINT_POLICY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.policyExtension == other.policyExtension && self.assertionName == other.assertionName && self.assertionNs == other.assertionNs && self.out == other.out
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENDPOINT_POLICY_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENDPOINT_POLICY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENDPOINT_POLICY_EXTENSION_0 {
    pub assertionValue: *mut WS_XML_BUFFER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENDPOINT_POLICY_EXTENSION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_POLICY_EXTENSION_0").field("assertionValue", &self.assertionValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ENDPOINT_POLICY_EXTENSION_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.assertionValue == other.assertionValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENDPOINT_POLICY_EXTENSION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENUM_DESCRIPTION {
    pub values: *mut WS_ENUM_VALUE,
    pub valueCount: u32,
    pub maxByteCount: u32,
    pub nameIndices: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENUM_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENUM_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENUM_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENUM_DESCRIPTION").field("values", &self.values).field("valueCount", &self.valueCount).field("maxByteCount", &self.maxByteCount).field("nameIndices", &self.nameIndices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ENUM_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENUM_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.valueCount == other.valueCount && self.maxByteCount == other.maxByteCount && self.nameIndices == other.nameIndices
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENUM_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENUM_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENUM_VALUE {
    pub value: i32,
    pub name: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENUM_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENUM_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENUM_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENUM_VALUE").field("value", &self.value).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ENUM_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENUM_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENUM_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENUM_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_ERROR(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ERROR_PROPERTY {
    pub id: WS_ERROR_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_ERROR_PROPERTY {}
impl ::core::clone::Clone for WS_ERROR_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ERROR_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ERROR_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_ERROR_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_ERROR_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_ERROR_PROPERTY {}
impl ::core::default::Default for WS_ERROR_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT {
    pub code: *mut WS_FAULT_CODE,
    pub reasons: *mut WS_FAULT_REASON,
    pub reasonCount: u32,
    pub actor: WS_STRING,
    pub node: WS_STRING,
    pub detail: *mut WS_XML_BUFFER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT").field("code", &self.code).field("reasons", &self.reasons).field("reasonCount", &self.reasonCount).field("actor", &self.actor).field("node", &self.node).field("detail", &self.detail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_FAULT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.reasons == other.reasons && self.reasonCount == other.reasonCount && self.actor == other.actor && self.node == other.node && self.detail == other.detail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT_CODE {
    pub value: WS_XML_QNAME,
    pub subCode: *mut WS_FAULT_CODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_CODE").field("value", &self.value).field("subCode", &self.subCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_FAULT_CODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.subCode == other.subCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_FAULT_DESCRIPTION {
    pub envelopeVersion: WS_ENVELOPE_VERSION,
}
impl ::core::marker::Copy for WS_FAULT_DESCRIPTION {}
impl ::core::clone::Clone for WS_FAULT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_FAULT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_DESCRIPTION").field("envelopeVersion", &self.envelopeVersion).finish()
    }
}
impl ::windows::core::TypeKind for WS_FAULT_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_FAULT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.envelopeVersion == other.envelopeVersion
    }
}
impl ::core::cmp::Eq for WS_FAULT_DESCRIPTION {}
impl ::core::default::Default for WS_FAULT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT_DETAIL_DESCRIPTION {
    pub action: *mut WS_XML_STRING,
    pub detailElementDescription: *mut WS_ELEMENT_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT_DETAIL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT_DETAIL_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT_DETAIL_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_DETAIL_DESCRIPTION").field("action", &self.action).field("detailElementDescription", &self.detailElementDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_FAULT_DETAIL_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT_DETAIL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.detailElementDescription == other.detailElementDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT_DETAIL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT_DETAIL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_FAULT_REASON {
    pub text: WS_STRING,
    pub lang: WS_STRING,
}
impl ::core::marker::Copy for WS_FAULT_REASON {}
impl ::core::clone::Clone for WS_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_REASON").field("text", &self.text).field("lang", &self.lang).finish()
    }
}
impl ::windows::core::TypeKind for WS_FAULT_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.lang == other.lang
    }
}
impl ::core::cmp::Eq for WS_FAULT_REASON {}
impl ::core::default::Default for WS_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FIELD_DESCRIPTION {
    pub mapping: WS_FIELD_MAPPING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
    pub r#type: WS_TYPE,
    pub typeDescription: *mut ::core::ffi::c_void,
    pub offset: u32,
    pub options: u32,
    pub defaultValue: *mut WS_DEFAULT_VALUE,
    pub countOffset: u32,
    pub itemLocalName: *mut WS_XML_STRING,
    pub itemNs: *mut WS_XML_STRING,
    pub itemRange: *mut WS_ITEM_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FIELD_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FIELD_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FIELD_DESCRIPTION").field("mapping", &self.mapping).field("localName", &self.localName).field("ns", &self.ns).field("type", &self.r#type).field("typeDescription", &self.typeDescription).field("offset", &self.offset).field("options", &self.options).field("defaultValue", &self.defaultValue).field("countOffset", &self.countOffset).field("itemLocalName", &self.itemLocalName).field("itemNs", &self.itemNs).field("itemRange", &self.itemRange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_FIELD_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FIELD_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.mapping == other.mapping && self.localName == other.localName && self.ns == other.ns && self.r#type == other.r#type && self.typeDescription == other.typeDescription && self.offset == other.offset && self.options == other.options && self.defaultValue == other.defaultValue && self.countOffset == other.countOffset && self.itemLocalName == other.itemLocalName && self.itemNs == other.itemNs && self.itemRange == other.itemRange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FIELD_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_FLOAT_DESCRIPTION {
    pub minValue: f32,
    pub maxValue: f32,
}
impl ::core::marker::Copy for WS_FLOAT_DESCRIPTION {}
impl ::core::clone::Clone for WS_FLOAT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_FLOAT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FLOAT_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_FLOAT_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_FLOAT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_FLOAT_DESCRIPTION {}
impl ::core::default::Default for WS_FLOAT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_GUID_DESCRIPTION {
    pub value: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_GUID_DESCRIPTION {}
impl ::core::clone::Clone for WS_GUID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_GUID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_GUID_DESCRIPTION").field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_GUID_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_GUID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_GUID_DESCRIPTION {}
impl ::core::default::Default for WS_GUID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_HEAP(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HEAP_PROPERTIES {
    pub properties: *mut WS_HEAP_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_HEAP_PROPERTIES {}
impl ::core::clone::Clone for WS_HEAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HEAP_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_HEAP_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HEAP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_HEAP_PROPERTIES {}
impl ::core::default::Default for WS_HEAP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HEAP_PROPERTY {
    pub id: WS_HEAP_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_HEAP_PROPERTY {}
impl ::core::clone::Clone for WS_HEAP_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HEAP_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_HEAP_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HEAP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_HEAP_PROPERTY {}
impl ::core::default::Default for WS_HEAP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HOST_NAMES {
    pub hostNames: *mut WS_STRING,
    pub hostNameCount: u32,
}
impl ::core::marker::Copy for WS_HOST_NAMES {}
impl ::core::clone::Clone for WS_HOST_NAMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HOST_NAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HOST_NAMES").field("hostNames", &self.hostNames).field("hostNameCount", &self.hostNameCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_HOST_NAMES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HOST_NAMES {
    fn eq(&self, other: &Self) -> bool {
        self.hostNames == other.hostNames && self.hostNameCount == other.hostNameCount
    }
}
impl ::core::cmp::Eq for WS_HOST_NAMES {}
impl ::core::default::Default for WS_HOST_NAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTPS_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_HTTPS_URL {}
impl ::core::clone::Clone for WS_HTTPS_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTPS_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTPS_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTPS_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTPS_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_HTTPS_URL {}
impl ::core::default::Default for WS_HTTPS_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_HTTP_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_HEADER_MAPPING {
    pub headerName: WS_XML_STRING,
    pub headerMappingOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_HEADER_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_HEADER_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_HTTP_HEADER_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_MAPPING").field("headerName", &self.headerName).field("headerMappingOptions", &self.headerMappingOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_HTTP_HEADER_MAPPING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.headerName == other.headerName && self.headerMappingOptions == other.headerMappingOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_HTTP_HEADER_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_HTTP_HEADER_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_MESSAGE_MAPPING {
    pub requestMappingOptions: u32,
    pub responseMappingOptions: u32,
    pub requestHeaderMappings: *mut *mut WS_HTTP_HEADER_MAPPING,
    pub requestHeaderMappingCount: u32,
    pub responseHeaderMappings: *mut *mut WS_HTTP_HEADER_MAPPING,
    pub responseHeaderMappingCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_MESSAGE_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_MESSAGE_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_HTTP_MESSAGE_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_MESSAGE_MAPPING").field("requestMappingOptions", &self.requestMappingOptions).field("responseMappingOptions", &self.responseMappingOptions).field("requestHeaderMappings", &self.requestHeaderMappings).field("requestHeaderMappingCount", &self.requestHeaderMappingCount).field("responseHeaderMappings", &self.responseHeaderMappings).field("responseHeaderMappingCount", &self.responseHeaderMappingCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_HTTP_MESSAGE_MAPPING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_HTTP_MESSAGE_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.requestMappingOptions == other.requestMappingOptions && self.responseMappingOptions == other.responseMappingOptions && self.requestHeaderMappings == other.requestHeaderMappings && self.requestHeaderMappingCount == other.requestHeaderMappingCount && self.responseHeaderMappings == other.responseHeaderMappings && self.responseHeaderMappingCount == other.responseHeaderMappingCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_HTTP_MESSAGE_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_HTTP_MESSAGE_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_HTTP_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    pub callback: WS_HTTP_REDIRECT_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {}
impl ::core::clone::Clone for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_REDIRECT_CALLBACK_CONTEXT").field("state", &self.state).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub httpHeaderAuthSecurityBinding: WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_HTTP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_HTTP_URL {}
impl ::core::clone::Clone for WS_HTTP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_HTTP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::windows::core::TypeKind for WS_HTTP_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_HTTP_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_HTTP_URL {}
impl ::core::default::Default for WS_HTTP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT16_DESCRIPTION {
    pub minValue: i16,
    pub maxValue: i16,
}
impl ::core::marker::Copy for WS_INT16_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT16_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT16_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT16_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_INT16_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_INT16_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT16_DESCRIPTION {}
impl ::core::default::Default for WS_INT16_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT32_DESCRIPTION {
    pub minValue: i32,
    pub maxValue: i32,
}
impl ::core::marker::Copy for WS_INT32_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT32_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT32_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT32_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_INT32_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_INT32_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT32_DESCRIPTION {}
impl ::core::default::Default for WS_INT32_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT64_DESCRIPTION {
    pub minValue: i64,
    pub maxValue: i64,
}
impl ::core::marker::Copy for WS_INT64_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT64_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT64_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT64_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_INT64_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_INT64_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT64_DESCRIPTION {}
impl ::core::default::Default for WS_INT64_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_INT8_DESCRIPTION {
    pub minValue: u8,
    pub maxValue: u8,
}
impl ::core::marker::Copy for WS_INT8_DESCRIPTION {}
impl ::core::clone::Clone for WS_INT8_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_INT8_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT8_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_INT8_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_INT8_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT8_DESCRIPTION {}
impl ::core::default::Default for WS_INT8_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub claimConstraints: *mut WS_XML_STRING,
    pub claimConstraintCount: u32,
    pub requestSecurityTokenPropertyConstraints: *mut WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT,
    pub requestSecurityTokenPropertyConstraintCount: u32,
    pub out: WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT")
            .field("bindingConstraint", &self.bindingConstraint)
            .field("bindingUsage", &self.bindingUsage)
            .field("claimConstraints", &self.claimConstraints)
            .field("claimConstraintCount", &self.claimConstraintCount)
            .field("requestSecurityTokenPropertyConstraints", &self.requestSecurityTokenPropertyConstraints)
            .field("requestSecurityTokenPropertyConstraintCount", &self.requestSecurityTokenPropertyConstraintCount)
            .field("out", &self.out)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage && self.claimConstraints == other.claimConstraints && self.claimConstraintCount == other.claimConstraintCount && self.requestSecurityTokenPropertyConstraints == other.requestSecurityTokenPropertyConstraints && self.requestSecurityTokenPropertyConstraintCount == other.requestSecurityTokenPropertyConstraintCount && self.out == other.out
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    pub issuerAddress: *mut WS_ENDPOINT_ADDRESS,
    pub requestSecurityTokenTemplate: *mut WS_XML_BUFFER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0").field("issuerAddress", &self.issuerAddress).field("requestSecurityTokenTemplate", &self.requestSecurityTokenTemplate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.issuerAddress == other.issuerAddress && self.requestSecurityTokenTemplate == other.requestSecurityTokenTemplate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_ITEM_RANGE {
    pub minItemCount: u32,
    pub maxItemCount: u32,
}
impl ::core::marker::Copy for WS_ITEM_RANGE {}
impl ::core::clone::Clone for WS_ITEM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_ITEM_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ITEM_RANGE").field("minItemCount", &self.minItemCount).field("maxItemCount", &self.maxItemCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_ITEM_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_ITEM_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.minItemCount == other.minItemCount && self.maxItemCount == other.maxItemCount
    }
}
impl ::core::cmp::Eq for WS_ITEM_RANGE {}
impl ::core::default::Default for WS_ITEM_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::windows::core::TypeKind for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::windows::core::TypeKind for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_LISTENER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_LISTENER_PROPERTIES {
    pub properties: *mut WS_LISTENER_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_LISTENER_PROPERTIES {}
impl ::core::clone::Clone for WS_LISTENER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_LISTENER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_LISTENER_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_LISTENER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_LISTENER_PROPERTIES {}
impl ::core::default::Default for WS_LISTENER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_LISTENER_PROPERTY {
    pub id: WS_LISTENER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_LISTENER_PROPERTY {}
impl ::core::clone::Clone for WS_LISTENER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_LISTENER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_LISTENER_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_LISTENER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_LISTENER_PROPERTY {}
impl ::core::default::Default for WS_LISTENER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_MESSAGE(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_MESSAGE_DESCRIPTION {
    pub action: *mut WS_XML_STRING,
    pub bodyElementDescription: *mut WS_ELEMENT_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_MESSAGE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_MESSAGE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_MESSAGE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_DESCRIPTION").field("action", &self.action).field("bodyElementDescription", &self.bodyElementDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_MESSAGE_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_MESSAGE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.bodyElementDescription == other.bodyElementDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_MESSAGE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_MESSAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_MESSAGE_PROPERTIES {
    pub properties: *mut WS_MESSAGE_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_MESSAGE_PROPERTIES {}
impl ::core::clone::Clone for WS_MESSAGE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_MESSAGE_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_MESSAGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_MESSAGE_PROPERTIES {}
impl ::core::default::Default for WS_MESSAGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_MESSAGE_PROPERTY {
    pub id: WS_MESSAGE_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_MESSAGE_PROPERTY {}
impl ::core::clone::Clone for WS_MESSAGE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_MESSAGE_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_MESSAGE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_MESSAGE_PROPERTY {}
impl ::core::default::Default for WS_MESSAGE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_METADATA(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_METADATA_ENDPOINT {
    pub endpointAddress: WS_ENDPOINT_ADDRESS,
    pub endpointPolicy: *mut WS_POLICY,
    pub portName: *mut WS_XML_STRING,
    pub serviceName: *mut WS_XML_STRING,
    pub serviceNs: *mut WS_XML_STRING,
    pub bindingName: *mut WS_XML_STRING,
    pub bindingNs: *mut WS_XML_STRING,
    pub portTypeName: *mut WS_XML_STRING,
    pub portTypeNs: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_METADATA_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_METADATA_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_METADATA_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_ENDPOINT").field("endpointAddress", &self.endpointAddress).field("endpointPolicy", &self.endpointPolicy).field("portName", &self.portName).field("serviceName", &self.serviceName).field("serviceNs", &self.serviceNs).field("bindingName", &self.bindingName).field("bindingNs", &self.bindingNs).field("portTypeName", &self.portTypeName).field("portTypeNs", &self.portTypeNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_METADATA_ENDPOINT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_METADATA_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.endpointAddress == other.endpointAddress && self.endpointPolicy == other.endpointPolicy && self.portName == other.portName && self.serviceName == other.serviceName && self.serviceNs == other.serviceNs && self.bindingName == other.bindingName && self.bindingNs == other.bindingNs && self.portTypeName == other.portTypeName && self.portTypeNs == other.portTypeNs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_METADATA_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_METADATA_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_METADATA_ENDPOINTS {
    pub endpoints: *mut WS_METADATA_ENDPOINT,
    pub endpointCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_METADATA_ENDPOINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_METADATA_ENDPOINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_METADATA_ENDPOINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_ENDPOINTS").field("endpoints", &self.endpoints).field("endpointCount", &self.endpointCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_METADATA_ENDPOINTS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_METADATA_ENDPOINTS {
    fn eq(&self, other: &Self) -> bool {
        self.endpoints == other.endpoints && self.endpointCount == other.endpointCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_METADATA_ENDPOINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_METADATA_ENDPOINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_METADATA_PROPERTY {
    pub id: WS_METADATA_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_METADATA_PROPERTY {}
impl ::core::clone::Clone for WS_METADATA_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_METADATA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_METADATA_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_METADATA_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_METADATA_PROPERTY {}
impl ::core::default::Default for WS_METADATA_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::default::Default for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub asymmetricKey: super::super::Security::Cryptography::NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("asymmetricKey", &self.asymmetricKey).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::windows::core::TypeKind for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandle == other.keyHandle && self.asymmetricKey == other.asymmetricKey
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_NETPIPE_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_NETPIPE_URL {}
impl ::core::clone::Clone for WS_NETPIPE_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_NETPIPE_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NETPIPE_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::windows::core::TypeKind for WS_NETPIPE_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_NETPIPE_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_NETPIPE_URL {}
impl ::core::default::Default for WS_NETPIPE_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_NETTCP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_NETTCP_URL {}
impl ::core::clone::Clone for WS_NETTCP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_NETTCP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NETTCP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::windows::core::TypeKind for WS_NETTCP_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_NETTCP_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_NETTCP_URL {}
impl ::core::default::Default for WS_NETTCP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
    pub opaqueAuthIdentity: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).field("opaqueAuthIdentity", &self.opaqueAuthIdentity).finish()
    }
}
impl ::windows::core::TypeKind for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.opaqueAuthIdentity == other.opaqueAuthIdentity
    }
}
impl ::core::cmp::Eq for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_OPERATION_CONTEXT(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_OPERATION_DESCRIPTION {
    pub versionInfo: u32,
    pub inputMessageDescription: *mut WS_MESSAGE_DESCRIPTION,
    pub outputMessageDescription: *mut WS_MESSAGE_DESCRIPTION,
    pub inputMessageOptions: u32,
    pub outputMessageOptions: u32,
    pub parameterCount: u16,
    pub parameterDescription: *mut WS_PARAMETER_DESCRIPTION,
    pub stubCallback: WS_SERVICE_STUB_CALLBACK,
    pub style: WS_OPERATION_STYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_OPERATION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_OPERATION_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_OPERATION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_OPERATION_DESCRIPTION").field("versionInfo", &self.versionInfo).field("inputMessageDescription", &self.inputMessageDescription).field("outputMessageDescription", &self.outputMessageDescription).field("inputMessageOptions", &self.inputMessageOptions).field("outputMessageOptions", &self.outputMessageOptions).field("parameterCount", &self.parameterCount).field("parameterDescription", &self.parameterDescription).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_OPERATION_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_OPERATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_PARAMETER_DESCRIPTION {
    pub parameterType: WS_PARAMETER_TYPE,
    pub inputMessageIndex: u16,
    pub outputMessageIndex: u16,
}
impl ::core::marker::Copy for WS_PARAMETER_DESCRIPTION {}
impl ::core::clone::Clone for WS_PARAMETER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_PARAMETER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PARAMETER_DESCRIPTION").field("parameterType", &self.parameterType).field("inputMessageIndex", &self.inputMessageIndex).field("outputMessageIndex", &self.outputMessageIndex).finish()
    }
}
impl ::windows::core::TypeKind for WS_PARAMETER_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_PARAMETER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.parameterType == other.parameterType && self.inputMessageIndex == other.inputMessageIndex && self.outputMessageIndex == other.outputMessageIndex
    }
}
impl ::core::cmp::Eq for WS_PARAMETER_DESCRIPTION {}
impl ::core::default::Default for WS_PARAMETER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_POLICY(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_CONSTRAINTS {
    pub channelBinding: WS_CHANNEL_BINDING,
    pub channelPropertyConstraints: *mut WS_CHANNEL_PROPERTY_CONSTRAINT,
    pub channelPropertyConstraintCount: u32,
    pub securityConstraints: *mut WS_SECURITY_CONSTRAINTS,
    pub policyExtensions: *mut *mut WS_POLICY_EXTENSION,
    pub policyExtensionCount: u32,
}
impl ::core::marker::Copy for WS_POLICY_CONSTRAINTS {}
impl ::core::clone::Clone for WS_POLICY_CONSTRAINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_CONSTRAINTS").field("channelBinding", &self.channelBinding).field("channelPropertyConstraints", &self.channelPropertyConstraints).field("channelPropertyConstraintCount", &self.channelPropertyConstraintCount).field("securityConstraints", &self.securityConstraints).field("policyExtensions", &self.policyExtensions).field("policyExtensionCount", &self.policyExtensionCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_POLICY_CONSTRAINTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_POLICY_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        self.channelBinding == other.channelBinding && self.channelPropertyConstraints == other.channelPropertyConstraints && self.channelPropertyConstraintCount == other.channelPropertyConstraintCount && self.securityConstraints == other.securityConstraints && self.policyExtensions == other.policyExtensions && self.policyExtensionCount == other.policyExtensionCount
    }
}
impl ::core::cmp::Eq for WS_POLICY_CONSTRAINTS {}
impl ::core::default::Default for WS_POLICY_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_EXTENSION {
    pub r#type: WS_POLICY_EXTENSION_TYPE,
}
impl ::core::marker::Copy for WS_POLICY_EXTENSION {}
impl ::core::clone::Clone for WS_POLICY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_EXTENSION").field("type", &self.r#type).finish()
    }
}
impl ::windows::core::TypeKind for WS_POLICY_EXTENSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_POLICY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for WS_POLICY_EXTENSION {}
impl ::core::default::Default for WS_POLICY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_PROPERTIES {
    pub properties: *mut WS_POLICY_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_POLICY_PROPERTIES {}
impl ::core::clone::Clone for WS_POLICY_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_POLICY_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_POLICY_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_POLICY_PROPERTIES {}
impl ::core::default::Default for WS_POLICY_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_POLICY_PROPERTY {
    pub id: WS_POLICY_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_POLICY_PROPERTY {}
impl ::core::clone::Clone for WS_POLICY_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_POLICY_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_POLICY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_POLICY_PROPERTY {}
impl ::core::default::Default for WS_POLICY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    pub callback: WS_PROXY_MESSAGE_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {}
impl ::core::clone::Clone for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PROXY_MESSAGE_CALLBACK_CONTEXT").field("state", &self.state).finish()
    }
}
impl ::windows::core::TypeKind for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_PROXY_PROPERTY {
    pub id: WS_PROXY_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_PROXY_PROPERTY {}
impl ::core::clone::Clone for WS_PROXY_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_PROXY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PROXY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_PROXY_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_PROXY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_PROXY_PROPERTY {}
impl ::core::default::Default for WS_PROXY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub rawKeyBytes: WS_BYTES,
}
impl ::core::marker::Copy for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("rawKeyBytes", &self.rawKeyBytes).finish()
    }
}
impl ::windows::core::TypeKind for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandle == other.keyHandle && self.rawKeyBytes == other.rawKeyBytes
    }
}
impl ::core::cmp::Eq for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::default::Default for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    pub id: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY {}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    pub id: WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::windows::core::TypeKind for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    pub requestSecurityTokenProperty: WS_REQUEST_SECURITY_TOKEN_PROPERTY,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0").field("requestSecurityTokenProperty", &self.requestSecurityTokenProperty).finish()
    }
}
impl ::windows::core::TypeKind for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.requestSecurityTokenProperty == other.requestSecurityTokenProperty
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_RSA_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub modulus: WS_BYTES,
    pub exponent: WS_BYTES,
}
impl ::core::marker::Copy for WS_RSA_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_RSA_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_RSA_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_RSA_ENDPOINT_IDENTITY").field("identity", &self.identity).field("modulus", &self.modulus).field("exponent", &self.exponent).finish()
    }
}
impl ::windows::core::TypeKind for WS_RSA_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_RSA_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.modulus == other.modulus && self.exponent == other.exponent
    }
}
impl ::core::cmp::Eq for WS_RSA_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_RSA_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SAML_AUTHENTICATOR {
    pub authenticatorType: WS_SAML_AUTHENTICATOR_TYPE,
}
impl ::core::marker::Copy for WS_SAML_AUTHENTICATOR {}
impl ::core::clone::Clone for WS_SAML_AUTHENTICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SAML_AUTHENTICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SAML_AUTHENTICATOR").field("authenticatorType", &self.authenticatorType).finish()
    }
}
impl ::windows::core::TypeKind for WS_SAML_AUTHENTICATOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SAML_AUTHENTICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.authenticatorType == other.authenticatorType
    }
}
impl ::core::cmp::Eq for WS_SAML_AUTHENTICATOR {}
impl ::core::default::Default for WS_SAML_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SAML_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub authenticator: *mut WS_SAML_AUTHENTICATOR,
}
impl ::core::marker::Copy for WS_SAML_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SAML_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("authenticator", &self.authenticator).finish()
    }
}
impl ::windows::core::TypeKind for WS_SAML_MESSAGE_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.authenticator == other.authenticator
    }
}
impl ::core::cmp::Eq for WS_SAML_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_ALGORITHM_PROPERTY {
    pub id: WS_SECURITY_ALGORITHM_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_ALGORITHM_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_ALGORITHM_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_ALGORITHM_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_ALGORITHM_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_ALGORITHM_SUITE {
    pub canonicalizationAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub digestAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub symmetricSignatureAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub asymmetricSignatureAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub encryptionAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub keyDerivationAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub symmetricKeyWrapAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub asymmetricKeyWrapAlgorithm: WS_SECURITY_ALGORITHM_ID,
    pub minSymmetricKeyLength: u32,
    pub maxSymmetricKeyLength: u32,
    pub minAsymmetricKeyLength: u32,
    pub maxAsymmetricKeyLength: u32,
    pub properties: *mut WS_SECURITY_ALGORITHM_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_ALGORITHM_SUITE {}
impl ::core::clone::Clone for WS_SECURITY_ALGORITHM_SUITE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_SUITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_ALGORITHM_SUITE")
            .field("canonicalizationAlgorithm", &self.canonicalizationAlgorithm)
            .field("digestAlgorithm", &self.digestAlgorithm)
            .field("symmetricSignatureAlgorithm", &self.symmetricSignatureAlgorithm)
            .field("asymmetricSignatureAlgorithm", &self.asymmetricSignatureAlgorithm)
            .field("encryptionAlgorithm", &self.encryptionAlgorithm)
            .field("keyDerivationAlgorithm", &self.keyDerivationAlgorithm)
            .field("symmetricKeyWrapAlgorithm", &self.symmetricKeyWrapAlgorithm)
            .field("asymmetricKeyWrapAlgorithm", &self.asymmetricKeyWrapAlgorithm)
            .field("minSymmetricKeyLength", &self.minSymmetricKeyLength)
            .field("maxSymmetricKeyLength", &self.maxSymmetricKeyLength)
            .field("minAsymmetricKeyLength", &self.minAsymmetricKeyLength)
            .field("maxAsymmetricKeyLength", &self.maxAsymmetricKeyLength)
            .field("properties", &self.properties)
            .field("propertyCount", &self.propertyCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_ALGORITHM_SUITE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_ALGORITHM_SUITE {
    fn eq(&self, other: &Self) -> bool {
        self.canonicalizationAlgorithm == other.canonicalizationAlgorithm
            && self.digestAlgorithm == other.digestAlgorithm
            && self.symmetricSignatureAlgorithm == other.symmetricSignatureAlgorithm
            && self.asymmetricSignatureAlgorithm == other.asymmetricSignatureAlgorithm
            && self.encryptionAlgorithm == other.encryptionAlgorithm
            && self.keyDerivationAlgorithm == other.keyDerivationAlgorithm
            && self.symmetricKeyWrapAlgorithm == other.symmetricKeyWrapAlgorithm
            && self.asymmetricKeyWrapAlgorithm == other.asymmetricKeyWrapAlgorithm
            && self.minSymmetricKeyLength == other.minSymmetricKeyLength
            && self.maxSymmetricKeyLength == other.maxSymmetricKeyLength
            && self.minAsymmetricKeyLength == other.minAsymmetricKeyLength
            && self.maxAsymmetricKeyLength == other.maxAsymmetricKeyLength
            && self.properties == other.properties
            && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_ALGORITHM_SUITE {}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_SUITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING {
    pub bindingType: WS_SECURITY_BINDING_TYPE,
    pub properties: *mut WS_SECURITY_BINDING_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING").field("bindingType", &self.bindingType).field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.bindingType == other.bindingType && self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING {}
impl ::core::default::Default for WS_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_CONSTRAINT {
    pub r#type: WS_SECURITY_BINDING_CONSTRAINT_TYPE,
    pub propertyConstraints: *mut WS_SECURITY_BINDING_PROPERTY_CONSTRAINT,
    pub propertyConstraintCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_CONSTRAINT").field("type", &self.r#type).field("propertyConstraints", &self.propertyConstraints).field("propertyConstraintCount", &self.propertyConstraintCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.propertyConstraints == other.propertyConstraints && self.propertyConstraintCount == other.propertyConstraintCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTIES {
    pub properties: *mut WS_SECURITY_BINDING_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTIES {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTIES {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTY {
    pub id: WS_SECURITY_BINDING_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    pub id: WS_SECURITY_BINDING_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    pub securityBindingProperty: WS_SECURITY_BINDING_PROPERTY,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0").field("securityBindingProperty", &self.securityBindingProperty).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperty == other.securityBindingProperty
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONSTRAINTS {
    pub securityPropertyConstraints: *mut WS_SECURITY_PROPERTY_CONSTRAINT,
    pub securityPropertyConstraintCount: u32,
    pub securityBindingConstraints: *mut *mut WS_SECURITY_BINDING_CONSTRAINT,
    pub securityBindingConstraintCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_CONSTRAINTS {}
impl ::core::clone::Clone for WS_SECURITY_CONSTRAINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONSTRAINTS").field("securityPropertyConstraints", &self.securityPropertyConstraints).field("securityPropertyConstraintCount", &self.securityPropertyConstraintCount).field("securityBindingConstraints", &self.securityBindingConstraints).field("securityBindingConstraintCount", &self.securityBindingConstraintCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONSTRAINTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        self.securityPropertyConstraints == other.securityPropertyConstraints && self.securityPropertyConstraintCount == other.securityPropertyConstraintCount && self.securityBindingConstraints == other.securityBindingConstraints && self.securityBindingConstraintCount == other.securityBindingConstraintCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONSTRAINTS {}
impl ::core::default::Default for WS_SECURITY_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_SECURITY_CONTEXT(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub bootstrapSecurityDescription: *mut WS_SECURITY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("bootstrapSecurityDescription", &self.bootstrapSecurityDescription).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.bootstrapSecurityDescription == other.bootstrapSecurityDescription
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub bootstrapSecurityConstraint: *mut WS_SECURITY_CONSTRAINTS,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).field("bootstrapSecurityConstraint", &self.bootstrapSecurityConstraint).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage && self.bootstrapSecurityConstraint == other.bootstrapSecurityConstraint
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_PROPERTY {
    pub id: WS_SECURITY_CONTEXT_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityContextMessageSecurityBinding: WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityProperties: WS_SECURITY_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityContextMessageSecurityBinding", &self.securityContextMessageSecurityBinding).field("securityProperties", &self.securityProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityContextMessageSecurityBinding == other.securityContextMessageSecurityBinding && self.securityProperties == other.securityProperties
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    pub securityContextMessageSecurityBinding: WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityProperties: WS_SECURITY_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE").field("securityContextMessageSecurityBinding", &self.securityContextMessageSecurityBinding).field("securityProperties", &self.securityProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityContextMessageSecurityBinding == other.securityContextMessageSecurityBinding && self.securityProperties == other.securityProperties
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_DESCRIPTION {
    pub securityBindings: *mut *mut WS_SECURITY_BINDING,
    pub securityBindingCount: u32,
    pub properties: *mut WS_SECURITY_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SECURITY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_DESCRIPTION").field("securityBindings", &self.securityBindings).field("securityBindingCount", &self.securityBindingCount).field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindings == other.securityBindings && self.securityBindingCount == other.securityBindingCount && self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_DESCRIPTION {}
impl ::core::default::Default for WS_SECURITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_KEY_HANDLE {
    pub keyHandleType: WS_SECURITY_KEY_HANDLE_TYPE,
}
impl ::core::marker::Copy for WS_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_KEY_HANDLE").field("keyHandleType", &self.keyHandleType).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_KEY_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandleType == other.keyHandleType
    }
}
impl ::core::cmp::Eq for WS_SECURITY_KEY_HANDLE {}
impl ::core::default::Default for WS_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTIES {
    pub properties: *mut WS_SECURITY_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTIES {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTIES {}
impl ::core::default::Default for WS_SECURITY_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTY {
    pub id: WS_SECURITY_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY {}
impl ::core::default::Default for WS_SECURITY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTY_CONSTRAINT {
    pub id: WS_SECURITY_PROPERTY_ID,
    pub allowedValues: *mut ::core::ffi::c_void,
    pub allowedValuesSize: u32,
    pub out: WS_SECURITY_PROPERTY_CONSTRAINT_0,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_CONSTRAINT {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_PROPERTY_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY_CONSTRAINT {}
impl ::core::default::Default for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    pub securityProperty: WS_SECURITY_PROPERTY,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY_CONSTRAINT_0").field("securityProperty", &self.securityProperty).finish()
    }
}
impl ::windows::core::TypeKind for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.securityProperty == other.securityProperty
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY_CONSTRAINT_0 {}
impl ::core::default::Default for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_SECURITY_TOKEN(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_CONTRACT {
    pub contractDescription: *const WS_CONTRACT_DESCRIPTION,
    pub defaultMessageHandlerCallback: WS_SERVICE_MESSAGE_RECEIVE_CALLBACK,
    pub methodTable: *const ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_CONTRACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_CONTRACT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_CONTRACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_CONTRACT").field("contractDescription", &self.contractDescription).field("methodTable", &self.methodTable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SERVICE_CONTRACT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_CONTRACT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_ENDPOINT {
    pub address: WS_ENDPOINT_ADDRESS,
    pub channelBinding: WS_CHANNEL_BINDING,
    pub channelType: WS_CHANNEL_TYPE,
    pub securityDescription: *const WS_SECURITY_DESCRIPTION,
    pub contract: *const WS_SERVICE_CONTRACT,
    pub authorizationCallback: WS_SERVICE_SECURITY_CALLBACK,
    pub properties: *const WS_SERVICE_ENDPOINT_PROPERTY,
    pub propertyCount: u32,
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT").field("address", &self.address).field("channelBinding", &self.channelBinding).field("channelType", &self.channelType).field("securityDescription", &self.securityDescription).field("contract", &self.contract).field("properties", &self.properties).field("propertyCount", &self.propertyCount).field("channelProperties", &self.channelProperties).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SERVICE_ENDPOINT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_ENDPOINT_METADATA {
    pub portName: *mut WS_XML_STRING,
    pub bindingName: *mut WS_XML_STRING,
    pub bindingNs: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT_METADATA").field("portName", &self.portName).field("bindingName", &self.bindingName).field("bindingNs", &self.bindingNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SERVICE_ENDPOINT_METADATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.portName == other.portName && self.bindingName == other.bindingName && self.bindingNs == other.bindingNs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_ENDPOINT_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_ENDPOINT_PROPERTY {
    pub id: WS_SERVICE_ENDPOINT_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SERVICE_ENDPOINT_PROPERTY {}
impl ::core::clone::Clone for WS_SERVICE_ENDPOINT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_ENDPOINT_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT_PROPERTY {}
impl ::core::default::Default for WS_SERVICE_ENDPOINT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_SERVICE_HOST(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_METADATA {
    pub documentCount: u32,
    pub documents: *mut *mut WS_SERVICE_METADATA_DOCUMENT,
    pub serviceName: *mut WS_XML_STRING,
    pub serviceNs: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_METADATA").field("documentCount", &self.documentCount).field("documents", &self.documents).field("serviceName", &self.serviceName).field("serviceNs", &self.serviceNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SERVICE_METADATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.documentCount == other.documentCount && self.documents == other.documents && self.serviceName == other.serviceName && self.serviceNs == other.serviceNs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_METADATA_DOCUMENT {
    pub content: *mut WS_XML_STRING,
    pub name: *mut WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_METADATA_DOCUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_METADATA_DOCUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_METADATA_DOCUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_METADATA_DOCUMENT").field("content", &self.content).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SERVICE_METADATA_DOCUMENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_METADATA_DOCUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_METADATA_DOCUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_METADATA_DOCUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_PROPERTY {
    pub id: WS_SERVICE_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SERVICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SERVICE_PROPERTY {}
impl ::core::default::Default for WS_SERVICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    pub callback: WS_SERVICE_ACCEPT_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY_ACCEPT_CALLBACK").finish()
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    pub callback: WS_SERVICE_CLOSE_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY_CLOSE_CALLBACK").finish()
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_SERVICE_PROXY(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SERVICE_SECURITY_IDENTITIES {
    pub serviceIdentities: *mut WS_STRING,
    pub serviceIdentityCount: u32,
}
impl ::core::marker::Copy for WS_SERVICE_SECURITY_IDENTITIES {}
impl ::core::clone::Clone for WS_SERVICE_SECURITY_IDENTITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SERVICE_SECURITY_IDENTITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_SECURITY_IDENTITIES").field("serviceIdentities", &self.serviceIdentities).field("serviceIdentityCount", &self.serviceIdentityCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_SERVICE_SECURITY_IDENTITIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SERVICE_SECURITY_IDENTITIES {
    fn eq(&self, other: &Self) -> bool {
        self.serviceIdentities == other.serviceIdentities && self.serviceIdentityCount == other.serviceIdentityCount
    }
}
impl ::core::cmp::Eq for WS_SERVICE_SECURITY_IDENTITIES {}
impl ::core::default::Default for WS_SERVICE_SECURITY_IDENTITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SOAPUDP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
impl ::core::marker::Copy for WS_SOAPUDP_URL {}
impl ::core::clone::Clone for WS_SOAPUDP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SOAPUDP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SOAPUDP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::windows::core::TypeKind for WS_SOAPUDP_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SOAPUDP_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_SOAPUDP_URL {}
impl ::core::default::Default for WS_SOAPUDP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SPN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub spn: WS_STRING,
}
impl ::core::marker::Copy for WS_SPN_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_SPN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SPN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SPN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("spn", &self.spn).finish()
    }
}
impl ::windows::core::TypeKind for WS_SPN_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SPN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.spn == other.spn
    }
}
impl ::core::cmp::Eq for WS_SPN_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_SPN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub localCertCredential: *mut WS_CERT_CREDENTIAL,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("localCertCredential", &self.localCertCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_SSL_TRANSPORT_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.localCertCredential == other.localCertCredential
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING {}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub out: WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("out", &self.out).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.out == other.out
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    pub clientCertCredentialRequired: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0").field("clientCertCredentialRequired", &self.clientCertCredentialRequired).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.clientCertCredentialRequired == other.clientCertCredentialRequired
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub localCertCredential: *mut WS_CERT_CREDENTIAL,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("localCertCredential", &self.localCertCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.localCertCredential == other.localCertCredential
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING {
    pub length: u32,
    pub chars: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WS_STRING {}
impl ::core::clone::Clone for WS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING").field("length", &self.length).field("chars", &self.chars).finish()
    }
}
impl ::windows::core::TypeKind for WS_STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.chars == other.chars
    }
}
impl ::core::cmp::Eq for WS_STRING {}
impl ::core::default::Default for WS_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_STRING_DESCRIPTION {}
impl ::core::clone::Clone for WS_STRING_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_STRING_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_STRING_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_STRING_DESCRIPTION {}
impl ::core::default::Default for WS_STRING_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING_USERNAME_CREDENTIAL {
    pub credential: WS_USERNAME_CREDENTIAL,
    pub username: WS_STRING,
    pub password: WS_STRING,
}
impl ::core::marker::Copy for WS_STRING_USERNAME_CREDENTIAL {}
impl ::core::clone::Clone for WS_STRING_USERNAME_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING_USERNAME_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_USERNAME_CREDENTIAL").field("credential", &self.credential).field("username", &self.username).field("password", &self.password).finish()
    }
}
impl ::windows::core::TypeKind for WS_STRING_USERNAME_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_STRING_USERNAME_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for WS_STRING_USERNAME_CREDENTIAL {}
impl ::core::default::Default for WS_STRING_USERNAME_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
    pub username: WS_STRING,
    pub password: WS_STRING,
    pub domain: WS_STRING,
}
impl ::core::marker::Copy for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).field("username", &self.username).field("password", &self.password).field("domain", &self.domain).finish()
    }
}
impl ::windows::core::TypeKind for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.username == other.username && self.password == other.password && self.domain == other.domain
    }
}
impl ::core::cmp::Eq for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_STRUCT_DESCRIPTION {
    pub size: u32,
    pub alignment: u32,
    pub fields: *mut *mut WS_FIELD_DESCRIPTION,
    pub fieldCount: u32,
    pub typeLocalName: *mut WS_XML_STRING,
    pub typeNs: *mut WS_XML_STRING,
    pub parentType: *mut WS_STRUCT_DESCRIPTION,
    pub subTypes: *mut *mut WS_STRUCT_DESCRIPTION,
    pub subTypeCount: u32,
    pub structOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_STRUCT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_STRUCT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_STRUCT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRUCT_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("fields", &self.fields).field("fieldCount", &self.fieldCount).field("typeLocalName", &self.typeLocalName).field("typeNs", &self.typeNs).field("parentType", &self.parentType).field("subTypes", &self.subTypes).field("subTypeCount", &self.subTypeCount).field("structOptions", &self.structOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_STRUCT_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_STRUCT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.alignment == other.alignment && self.fields == other.fields && self.fieldCount == other.fieldCount && self.typeLocalName == other.typeLocalName && self.typeNs == other.typeNs && self.parentType == other.parentType && self.subTypes == other.subTypes && self.subTypeCount == other.subTypeCount && self.structOptions == other.structOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_STRUCT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_STRUCT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_SUBJECT_NAME_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub storeLocation: u32,
    pub storeName: WS_STRING,
    pub subjectName: WS_STRING,
}
impl ::core::marker::Copy for WS_SUBJECT_NAME_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SUBJECT_NAME_CERT_CREDENTIAL").field("credential", &self.credential).field("storeLocation", &self.storeLocation).field("storeName", &self.storeName).field("subjectName", &self.subjectName).finish()
    }
}
impl ::windows::core::TypeKind for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.storeLocation == other.storeLocation && self.storeName == other.storeName && self.subjectName == other.subjectName
    }
}
impl ::core::cmp::Eq for WS_SUBJECT_NAME_CERT_CREDENTIAL {}
impl ::core::default::Default for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_TCP_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_TCP_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_TCP_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_TCP_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub kerberosApreqMessageSecurityBinding: WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION,
}
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::windows::core::TypeKind for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_THUMBPRINT_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub storeLocation: u32,
    pub storeName: WS_STRING,
    pub thumbprint: WS_STRING,
}
impl ::core::marker::Copy for WS_THUMBPRINT_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_THUMBPRINT_CERT_CREDENTIAL").field("credential", &self.credential).field("storeLocation", &self.storeLocation).field("storeName", &self.storeName).field("thumbprint", &self.thumbprint).finish()
    }
}
impl ::windows::core::TypeKind for WS_THUMBPRINT_CERT_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.storeLocation == other.storeLocation && self.storeName == other.storeName && self.thumbprint == other.thumbprint
    }
}
impl ::core::cmp::Eq for WS_THUMBPRINT_CERT_CREDENTIAL {}
impl ::core::default::Default for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TIMESPAN {
    pub ticks: i64,
}
impl ::core::marker::Copy for WS_TIMESPAN {}
impl ::core::clone::Clone for WS_TIMESPAN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TIMESPAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TIMESPAN").field("ticks", &self.ticks).finish()
    }
}
impl ::windows::core::TypeKind for WS_TIMESPAN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TIMESPAN {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks
    }
}
impl ::core::cmp::Eq for WS_TIMESPAN {}
impl ::core::default::Default for WS_TIMESPAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_TIMESPAN_DESCRIPTION {
    pub minValue: WS_TIMESPAN,
    pub maxValue: WS_TIMESPAN,
}
impl ::core::marker::Copy for WS_TIMESPAN_DESCRIPTION {}
impl ::core::clone::Clone for WS_TIMESPAN_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_TIMESPAN_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TIMESPAN_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_TIMESPAN_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_TIMESPAN_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_TIMESPAN_DESCRIPTION {}
impl ::core::default::Default for WS_TIMESPAN_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT16_DESCRIPTION {
    pub minValue: u16,
    pub maxValue: u16,
}
impl ::core::marker::Copy for WS_UINT16_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT16_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT16_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT16_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_UINT16_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UINT16_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT16_DESCRIPTION {}
impl ::core::default::Default for WS_UINT16_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT32_DESCRIPTION {
    pub minValue: u32,
    pub maxValue: u32,
}
impl ::core::marker::Copy for WS_UINT32_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT32_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT32_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT32_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_UINT32_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UINT32_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT32_DESCRIPTION {}
impl ::core::default::Default for WS_UINT32_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT64_DESCRIPTION {
    pub minValue: u64,
    pub maxValue: u64,
}
impl ::core::marker::Copy for WS_UINT64_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT64_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT64_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT64_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_UINT64_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UINT64_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT64_DESCRIPTION {}
impl ::core::default::Default for WS_UINT64_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UINT8_DESCRIPTION {
    pub minValue: u8,
    pub maxValue: u8,
}
impl ::core::marker::Copy for WS_UINT8_DESCRIPTION {}
impl ::core::clone::Clone for WS_UINT8_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UINT8_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT8_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows::core::TypeKind for WS_UINT8_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UINT8_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT8_DESCRIPTION {}
impl ::core::default::Default for WS_UINT8_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_UNION_DESCRIPTION {
    pub size: u32,
    pub alignment: u32,
    pub fields: *mut *mut WS_UNION_FIELD_DESCRIPTION,
    pub fieldCount: u32,
    pub enumOffset: u32,
    pub noneEnumValue: i32,
    pub valueIndices: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_UNION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_UNION_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_UNION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNION_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("fields", &self.fields).field("fieldCount", &self.fieldCount).field("enumOffset", &self.enumOffset).field("noneEnumValue", &self.noneEnumValue).field("valueIndices", &self.valueIndices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_UNION_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_UNION_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.alignment == other.alignment && self.fields == other.fields && self.fieldCount == other.fieldCount && self.enumOffset == other.enumOffset && self.noneEnumValue == other.noneEnumValue && self.valueIndices == other.valueIndices
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_UNION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_UNION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_UNION_FIELD_DESCRIPTION {
    pub value: i32,
    pub field: WS_FIELD_DESCRIPTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_UNION_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_UNION_FIELD_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_UNION_FIELD_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNION_FIELD_DESCRIPTION").field("value", &self.value).field("field", &self.field).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_UNION_FIELD_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_UNION_FIELD_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.field == other.field
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_UNION_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_UNION_FIELD_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UNIQUE_ID {
    pub uri: WS_STRING,
    pub guid: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_UNIQUE_ID {}
impl ::core::clone::Clone for WS_UNIQUE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UNIQUE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNIQUE_ID").field("uri", &self.uri).field("guid", &self.guid).finish()
    }
}
impl ::windows::core::TypeKind for WS_UNIQUE_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UNIQUE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri && self.guid == other.guid
    }
}
impl ::core::cmp::Eq for WS_UNIQUE_ID {}
impl ::core::default::Default for WS_UNIQUE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UNIQUE_ID_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_UNIQUE_ID_DESCRIPTION {}
impl ::core::clone::Clone for WS_UNIQUE_ID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UNIQUE_ID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNIQUE_ID_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_UNIQUE_ID_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UNIQUE_ID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_UNIQUE_ID_DESCRIPTION {}
impl ::core::default::Default for WS_UNIQUE_ID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UNKNOWN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub element: *mut WS_XML_BUFFER,
}
impl ::core::marker::Copy for WS_UNKNOWN_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNKNOWN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("element", &self.element).finish()
    }
}
impl ::windows::core::TypeKind for WS_UNKNOWN_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.element == other.element
    }
}
impl ::core::cmp::Eq for WS_UNKNOWN_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UPN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub upn: WS_STRING,
}
impl ::core::marker::Copy for WS_UPN_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_UPN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UPN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UPN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("upn", &self.upn).finish()
    }
}
impl ::windows::core::TypeKind for WS_UPN_ENDPOINT_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UPN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.upn == other.upn
    }
}
impl ::core::cmp::Eq for WS_UPN_ENDPOINT_IDENTITY {}
impl ::core::default::Default for WS_UPN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_URL {
    pub scheme: WS_URL_SCHEME_TYPE,
}
impl ::core::marker::Copy for WS_URL {}
impl ::core::clone::Clone for WS_URL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_URL").field("scheme", &self.scheme).finish()
    }
}
impl ::windows::core::TypeKind for WS_URL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_URL {
    fn eq(&self, other: &Self) -> bool {
        self.scheme == other.scheme
    }
}
impl ::core::cmp::Eq for WS_URL {}
impl ::core::default::Default for WS_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_CREDENTIAL {
    pub credentialType: WS_USERNAME_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_USERNAME_CREDENTIAL {}
impl ::core::clone::Clone for WS_USERNAME_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
impl ::windows::core::TypeKind for WS_USERNAME_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_USERNAME_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credentialType == other.credentialType
    }
}
impl ::core::cmp::Eq for WS_USERNAME_CREDENTIAL {}
impl ::core::default::Default for WS_USERNAME_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub clientCredential: *mut WS_USERNAME_CREDENTIAL,
    pub passwordValidator: WS_VALIDATE_PASSWORD_CALLBACK,
    pub passwordValidatorCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("clientCredential", &self.clientCredential).field("passwordValidatorCallbackState", &self.passwordValidatorCallbackState).finish()
    }
}
impl ::windows::core::TypeKind for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::windows::core::TypeKind for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::windows::core::TypeKind for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_USERNAME_CREDENTIAL,
    pub passwordValidator: WS_VALIDATE_PASSWORD_CALLBACK,
    pub passwordValidatorCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).field("passwordValidatorCallbackState", &self.passwordValidatorCallbackState).finish()
    }
}
impl ::windows::core::TypeKind for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_UTF8_ARRAY_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_UTF8_ARRAY_DESCRIPTION {}
impl ::core::clone::Clone for WS_UTF8_ARRAY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_UTF8_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UTF8_ARRAY_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_UTF8_ARRAY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_UTF8_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_UTF8_ARRAY_DESCRIPTION {}
impl ::core::default::Default for WS_UTF8_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_VOID_DESCRIPTION {
    pub size: u32,
}
impl ::core::marker::Copy for WS_VOID_DESCRIPTION {}
impl ::core::clone::Clone for WS_VOID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_VOID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_VOID_DESCRIPTION").field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for WS_VOID_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_VOID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}
impl ::core::cmp::Eq for WS_VOID_DESCRIPTION {}
impl ::core::default::Default for WS_VOID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credentialType: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
impl ::windows::core::TypeKind for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credentialType == other.credentialType
    }
}
impl ::core::cmp::Eq for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_WSZ_DESCRIPTION {
    pub minCharCount: u32,
    pub maxCharCount: u32,
}
impl ::core::marker::Copy for WS_WSZ_DESCRIPTION {}
impl ::core::clone::Clone for WS_WSZ_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_WSZ_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_WSZ_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_WSZ_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_WSZ_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_WSZ_DESCRIPTION {}
impl ::core::default::Default for WS_WSZ_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_ATTRIBUTE {
    pub singleQuote: u8,
    pub isXmlNs: u8,
    pub prefix: *mut WS_XML_STRING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
    pub value: *mut WS_XML_TEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_ATTRIBUTE").field("singleQuote", &self.singleQuote).field("isXmlNs", &self.isXmlNs).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_ATTRIBUTE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.singleQuote == other.singleQuote && self.isXmlNs == other.isXmlNs && self.prefix == other.prefix && self.localName == other.localName && self.ns == other.ns && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_BASE64_TEXT {
    pub text: WS_XML_TEXT,
    pub bytes: *mut u8,
    pub length: u32,
}
impl ::core::marker::Copy for WS_XML_BASE64_TEXT {}
impl ::core::clone::Clone for WS_XML_BASE64_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_BASE64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BASE64_TEXT").field("text", &self.text).field("bytes", &self.bytes).field("length", &self.length).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_BASE64_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_BASE64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.bytes == other.bytes && self.length == other.length
    }
}
impl ::core::cmp::Eq for WS_XML_BASE64_TEXT {}
impl ::core::default::Default for WS_XML_BASE64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_BOOL_TEXT {
    pub text: WS_XML_TEXT,
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_BOOL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_BOOL_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_BOOL_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BOOL_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_BOOL_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_BOOL_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_BOOL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_BOOL_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_XML_BUFFER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_BUFFER_PROPERTY {
    pub id: WS_XML_BUFFER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_BUFFER_PROPERTY {}
impl ::core::clone::Clone for WS_XML_BUFFER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_BUFFER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BUFFER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_BUFFER_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_BUFFER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_BUFFER_PROPERTY {}
impl ::core::default::Default for WS_XML_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    pub prefixCount: u32,
    pub prefixes: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES").field("prefixCount", &self.prefixCount).field("prefixes", &self.prefixes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn eq(&self, other: &Self) -> bool {
        self.prefixCount == other.prefixCount && self.prefixes == other.prefixes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_CANONICALIZATION_PROPERTY {
    pub id: WS_XML_CANONICALIZATION_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_CANONICALIZATION_PROPERTY {}
impl ::core::clone::Clone for WS_XML_CANONICALIZATION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_CANONICALIZATION_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_CANONICALIZATION_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_CANONICALIZATION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_CANONICALIZATION_PROPERTY {}
impl ::core::default::Default for WS_XML_CANONICALIZATION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_COMMENT_NODE {
    pub node: WS_XML_NODE,
    pub value: WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_COMMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_COMMENT_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_COMMENT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_COMMENT_NODE").field("node", &self.node).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_COMMENT_NODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_COMMENT_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_COMMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_COMMENT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_DATETIME_TEXT {
    pub text: WS_XML_TEXT,
    pub value: WS_DATETIME,
}
impl ::core::marker::Copy for WS_XML_DATETIME_TEXT {}
impl ::core::clone::Clone for WS_XML_DATETIME_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_DATETIME_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DATETIME_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_DATETIME_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_DATETIME_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_DATETIME_TEXT {}
impl ::core::default::Default for WS_XML_DATETIME_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_DECIMAL_TEXT {
    pub text: WS_XML_TEXT,
    pub value: super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_DECIMAL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_DECIMAL_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_DECIMAL_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_DECIMAL_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_DICTIONARY {
    pub guid: ::windows::core::GUID,
    pub strings: *mut WS_XML_STRING,
    pub stringCount: u32,
    pub isConst: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_DICTIONARY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_DICTIONARY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_DICTIONARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DICTIONARY").field("guid", &self.guid).field("strings", &self.strings).field("stringCount", &self.stringCount).field("isConst", &self.isConst).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_DICTIONARY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_DICTIONARY {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.strings == other.strings && self.stringCount == other.stringCount && self.isConst == other.isConst
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_DICTIONARY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_DICTIONARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_DOUBLE_TEXT {
    pub text: WS_XML_TEXT,
    pub value: f64,
}
impl ::core::marker::Copy for WS_XML_DOUBLE_TEXT {}
impl ::core::clone::Clone for WS_XML_DOUBLE_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_DOUBLE_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DOUBLE_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_DOUBLE_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_DOUBLE_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_DOUBLE_TEXT {}
impl ::core::default::Default for WS_XML_DOUBLE_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_ELEMENT_NODE {
    pub node: WS_XML_NODE,
    pub prefix: *mut WS_XML_STRING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
    pub attributeCount: u32,
    pub attributes: *mut *mut WS_XML_ATTRIBUTE,
    pub isEmpty: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_ELEMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_ELEMENT_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_ELEMENT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_ELEMENT_NODE").field("node", &self.node).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).field("attributeCount", &self.attributeCount).field("attributes", &self.attributes).field("isEmpty", &self.isEmpty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_ELEMENT_NODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_ELEMENT_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.prefix == other.prefix && self.localName == other.localName && self.ns == other.ns && self.attributeCount == other.attributeCount && self.attributes == other.attributes && self.isEmpty == other.isEmpty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_ELEMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_ELEMENT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_FLOAT_TEXT {
    pub text: WS_XML_TEXT,
    pub value: f32,
}
impl ::core::marker::Copy for WS_XML_FLOAT_TEXT {}
impl ::core::clone::Clone for WS_XML_FLOAT_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_FLOAT_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_FLOAT_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_FLOAT_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_FLOAT_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_FLOAT_TEXT {}
impl ::core::default::Default for WS_XML_FLOAT_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_GUID_TEXT {
    pub text: WS_XML_TEXT,
    pub value: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_XML_GUID_TEXT {}
impl ::core::clone::Clone for WS_XML_GUID_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_GUID_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_GUID_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_GUID_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_GUID_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_GUID_TEXT {}
impl ::core::default::Default for WS_XML_GUID_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_INT32_TEXT {
    pub text: WS_XML_TEXT,
    pub value: i32,
}
impl ::core::marker::Copy for WS_XML_INT32_TEXT {}
impl ::core::clone::Clone for WS_XML_INT32_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_INT32_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_INT32_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_INT32_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_INT32_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_INT32_TEXT {}
impl ::core::default::Default for WS_XML_INT32_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_INT64_TEXT {
    pub text: WS_XML_TEXT,
    pub value: i64,
}
impl ::core::marker::Copy for WS_XML_INT64_TEXT {}
impl ::core::clone::Clone for WS_XML_INT64_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_INT64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_INT64_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_INT64_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_INT64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_INT64_TEXT {}
impl ::core::default::Default for WS_XML_INT64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_LIST_TEXT {
    pub text: WS_XML_TEXT,
    pub itemCount: u32,
    pub items: *mut *mut WS_XML_TEXT,
}
impl ::core::marker::Copy for WS_XML_LIST_TEXT {}
impl ::core::clone::Clone for WS_XML_LIST_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_LIST_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_LIST_TEXT").field("text", &self.text).field("itemCount", &self.itemCount).field("items", &self.items).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_LIST_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_LIST_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.itemCount == other.itemCount && self.items == other.items
    }
}
impl ::core::cmp::Eq for WS_XML_LIST_TEXT {}
impl ::core::default::Default for WS_XML_LIST_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_NODE {
    pub nodeType: WS_XML_NODE_TYPE,
}
impl ::core::marker::Copy for WS_XML_NODE {}
impl ::core::clone::Clone for WS_XML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_NODE").field("nodeType", &self.nodeType).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_NODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.nodeType == other.nodeType
    }
}
impl ::core::cmp::Eq for WS_XML_NODE {}
impl ::core::default::Default for WS_XML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_NODE_POSITION {
    pub buffer: *mut WS_XML_BUFFER,
    pub node: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_XML_NODE_POSITION {}
impl ::core::clone::Clone for WS_XML_NODE_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_NODE_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_NODE_POSITION").field("buffer", &self.buffer).field("node", &self.node).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_NODE_POSITION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_NODE_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.node == other.node
    }
}
impl ::core::cmp::Eq for WS_XML_NODE_POSITION {}
impl ::core::default::Default for WS_XML_NODE_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_QNAME {
    pub localName: WS_XML_STRING,
    pub ns: WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_QNAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_QNAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_QNAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME").field("localName", &self.localName).field("ns", &self.ns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_QNAME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_QNAME {
    fn eq(&self, other: &Self) -> bool {
        self.localName == other.localName && self.ns == other.ns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_QNAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_QNAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_QNAME_DESCRIPTION {
    pub minLocalNameByteCount: u32,
    pub maxLocalNameByteCount: u32,
    pub minNsByteCount: u32,
    pub maxNsByteCount: u32,
}
impl ::core::marker::Copy for WS_XML_QNAME_DESCRIPTION {}
impl ::core::clone::Clone for WS_XML_QNAME_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_QNAME_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME_DESCRIPTION").field("minLocalNameByteCount", &self.minLocalNameByteCount).field("maxLocalNameByteCount", &self.maxLocalNameByteCount).field("minNsByteCount", &self.minNsByteCount).field("maxNsByteCount", &self.maxNsByteCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_QNAME_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_QNAME_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minLocalNameByteCount == other.minLocalNameByteCount && self.maxLocalNameByteCount == other.maxLocalNameByteCount && self.minNsByteCount == other.minNsByteCount && self.maxNsByteCount == other.maxNsByteCount
    }
}
impl ::core::cmp::Eq for WS_XML_QNAME_DESCRIPTION {}
impl ::core::default::Default for WS_XML_QNAME_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_QNAME_TEXT {
    pub text: WS_XML_TEXT,
    pub prefix: *mut WS_XML_STRING,
    pub localName: *mut WS_XML_STRING,
    pub ns: *mut WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_QNAME_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_QNAME_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_QNAME_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME_TEXT").field("text", &self.text).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_QNAME_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_QNAME_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.prefix == other.prefix && self.localName == other.localName && self.ns == other.ns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_QNAME_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_QNAME_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_XML_READER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_READER_BINARY_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
    pub staticDictionary: *mut WS_XML_DICTIONARY,
    pub dynamicDictionary: *mut WS_XML_DICTIONARY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_READER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_READER_BINARY_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_READER_BINARY_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_BINARY_ENCODING").field("encoding", &self.encoding).field("staticDictionary", &self.staticDictionary).field("dynamicDictionary", &self.dynamicDictionary).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_READER_BINARY_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_READER_BINARY_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.staticDictionary == other.staticDictionary && self.dynamicDictionary == other.dynamicDictionary
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_READER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_READER_BINARY_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_BUFFER_INPUT {
    pub input: WS_XML_READER_INPUT,
    pub encodedData: *mut ::core::ffi::c_void,
    pub encodedDataSize: u32,
}
impl ::core::marker::Copy for WS_XML_READER_BUFFER_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_BUFFER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_BUFFER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_BUFFER_INPUT").field("input", &self.input).field("encodedData", &self.encodedData).field("encodedDataSize", &self.encodedDataSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_BUFFER_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_BUFFER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input && self.encodedData == other.encodedData && self.encodedDataSize == other.encodedDataSize
    }
}
impl ::core::cmp::Eq for WS_XML_READER_BUFFER_INPUT {}
impl ::core::default::Default for WS_XML_READER_BUFFER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_ENCODING {
    pub encodingType: WS_XML_READER_ENCODING_TYPE,
}
impl ::core::marker::Copy for WS_XML_READER_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_ENCODING").field("encodingType", &self.encodingType).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encodingType == other.encodingType
    }
}
impl ::core::cmp::Eq for WS_XML_READER_ENCODING {}
impl ::core::default::Default for WS_XML_READER_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_INPUT {
    pub inputType: WS_XML_READER_INPUT_TYPE,
}
impl ::core::marker::Copy for WS_XML_READER_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_INPUT").field("inputType", &self.inputType).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.inputType == other.inputType
    }
}
impl ::core::cmp::Eq for WS_XML_READER_INPUT {}
impl ::core::default::Default for WS_XML_READER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_READER_MTOM_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
    pub textEncoding: *mut WS_XML_READER_ENCODING,
    pub readMimeHeader: super::super::Foundation::BOOL,
    pub startInfo: WS_STRING,
    pub boundary: WS_STRING,
    pub startUri: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_READER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_READER_MTOM_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_READER_MTOM_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_MTOM_ENCODING").field("encoding", &self.encoding).field("textEncoding", &self.textEncoding).field("readMimeHeader", &self.readMimeHeader).field("startInfo", &self.startInfo).field("boundary", &self.boundary).field("startUri", &self.startUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_READER_MTOM_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_READER_MTOM_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.textEncoding == other.textEncoding && self.readMimeHeader == other.readMimeHeader && self.startInfo == other.startInfo && self.boundary == other.boundary && self.startUri == other.startUri
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_READER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_READER_MTOM_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_PROPERTIES {
    pub properties: *mut WS_XML_READER_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_XML_READER_PROPERTIES {}
impl ::core::clone::Clone for WS_XML_READER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_XML_READER_PROPERTIES {}
impl ::core::default::Default for WS_XML_READER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_PROPERTY {
    pub id: WS_XML_READER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_READER_PROPERTY {}
impl ::core::clone::Clone for WS_XML_READER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_READER_PROPERTY {}
impl ::core::default::Default for WS_XML_READER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_RAW_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
}
impl ::core::marker::Copy for WS_XML_READER_RAW_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_RAW_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_RAW_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_RAW_ENCODING").field("encoding", &self.encoding).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_RAW_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_RAW_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding
    }
}
impl ::core::cmp::Eq for WS_XML_READER_RAW_ENCODING {}
impl ::core::default::Default for WS_XML_READER_RAW_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_STREAM_INPUT {
    pub input: WS_XML_READER_INPUT,
    pub readCallback: WS_READ_CALLBACK,
    pub readCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_XML_READER_STREAM_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_STREAM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_STREAM_INPUT").field("input", &self.input).field("readCallbackState", &self.readCallbackState).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_STREAM_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_XML_READER_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_READER_TEXT_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
    pub charSet: WS_CHARSET,
}
impl ::core::marker::Copy for WS_XML_READER_TEXT_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_TEXT_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_READER_TEXT_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_TEXT_ENCODING").field("encoding", &self.encoding).field("charSet", &self.charSet).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_READER_TEXT_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_READER_TEXT_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.charSet == other.charSet
    }
}
impl ::core::cmp::Eq for WS_XML_READER_TEXT_ENCODING {}
impl ::core::default::Default for WS_XML_READER_TEXT_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_SECURITY_TOKEN_PROPERTY {
    pub id: WS_XML_SECURITY_TOKEN_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_SECURITY_TOKEN_PROPERTY {}
impl ::core::clone::Clone for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_SECURITY_TOKEN_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_SECURITY_TOKEN_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_SECURITY_TOKEN_PROPERTY {}
impl ::core::default::Default for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_STRING {
    pub length: u32,
    pub bytes: *mut u8,
    pub dictionary: *mut WS_XML_DICTIONARY,
    pub id: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_STRING").field("length", &self.length).field("bytes", &self.bytes).field("dictionary", &self.dictionary).field("id", &self.id).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_STRING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.bytes == other.bytes && self.dictionary == other.dictionary && self.id == other.id
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_STRING_DESCRIPTION {
    pub minByteCount: u32,
    pub maxByteCount: u32,
}
impl ::core::marker::Copy for WS_XML_STRING_DESCRIPTION {}
impl ::core::clone::Clone for WS_XML_STRING_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_STRING_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_STRING_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_STRING_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_STRING_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_XML_STRING_DESCRIPTION {}
impl ::core::default::Default for WS_XML_STRING_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TEXT {
    pub textType: WS_XML_TEXT_TYPE,
}
impl ::core::marker::Copy for WS_XML_TEXT {}
impl ::core::clone::Clone for WS_XML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TEXT").field("textType", &self.textType).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.textType == other.textType
    }
}
impl ::core::cmp::Eq for WS_XML_TEXT {}
impl ::core::default::Default for WS_XML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TEXT_NODE {
    pub node: WS_XML_NODE,
    pub text: *mut WS_XML_TEXT,
}
impl ::core::marker::Copy for WS_XML_TEXT_NODE {}
impl ::core::clone::Clone for WS_XML_TEXT_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TEXT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TEXT_NODE").field("node", &self.node).field("text", &self.text).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_TEXT_NODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_TEXT_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.text == other.text
    }
}
impl ::core::cmp::Eq for WS_XML_TEXT_NODE {}
impl ::core::default::Default for WS_XML_TEXT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TIMESPAN_TEXT {
    pub text: WS_XML_TEXT,
    pub value: WS_TIMESPAN,
}
impl ::core::marker::Copy for WS_XML_TIMESPAN_TEXT {}
impl ::core::clone::Clone for WS_XML_TIMESPAN_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TIMESPAN_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TIMESPAN_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_TIMESPAN_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_TIMESPAN_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_TIMESPAN_TEXT {}
impl ::core::default::Default for WS_XML_TIMESPAN_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub xmlToken: *mut WS_SECURITY_TOKEN,
}
impl ::core::marker::Copy for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {}
impl ::core::clone::Clone for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TOKEN_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("xmlToken", &self.xmlToken).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.xmlToken == other.xmlToken
    }
}
impl ::core::cmp::Eq for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {}
impl ::core::default::Default for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_UINT64_TEXT {
    pub text: WS_XML_TEXT,
    pub value: u64,
}
impl ::core::marker::Copy for WS_XML_UINT64_TEXT {}
impl ::core::clone::Clone for WS_XML_UINT64_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_UINT64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UINT64_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_UINT64_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_UINT64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_UINT64_TEXT {}
impl ::core::default::Default for WS_XML_UINT64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_UNIQUE_ID_TEXT {
    pub text: WS_XML_TEXT,
    pub value: ::windows::core::GUID,
}
impl ::core::marker::Copy for WS_XML_UNIQUE_ID_TEXT {}
impl ::core::clone::Clone for WS_XML_UNIQUE_ID_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_UNIQUE_ID_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UNIQUE_ID_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_UNIQUE_ID_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_UNIQUE_ID_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_UNIQUE_ID_TEXT {}
impl ::core::default::Default for WS_XML_UNIQUE_ID_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_UTF16_TEXT {
    pub text: WS_XML_TEXT,
    pub bytes: *mut u8,
    pub byteCount: u32,
}
impl ::core::marker::Copy for WS_XML_UTF16_TEXT {}
impl ::core::clone::Clone for WS_XML_UTF16_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_UTF16_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UTF16_TEXT").field("text", &self.text).field("bytes", &self.bytes).field("byteCount", &self.byteCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_UTF16_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_UTF16_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.bytes == other.bytes && self.byteCount == other.byteCount
    }
}
impl ::core::cmp::Eq for WS_XML_UTF16_TEXT {}
impl ::core::default::Default for WS_XML_UTF16_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_UTF8_TEXT {
    pub text: WS_XML_TEXT,
    pub value: WS_XML_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_UTF8_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_UTF8_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_UTF8_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UTF8_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_UTF8_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_UTF8_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_UTF8_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_UTF8_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WS_XML_WRITER(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_WRITER_BINARY_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
    pub staticDictionary: *mut WS_XML_DICTIONARY,
    pub dynamicStringCallback: WS_DYNAMIC_STRING_CALLBACK,
    pub dynamicStringCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_WRITER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_WRITER_BINARY_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_WRITER_BINARY_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_BINARY_ENCODING").field("encoding", &self.encoding).field("staticDictionary", &self.staticDictionary).field("dynamicStringCallbackState", &self.dynamicStringCallbackState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_WRITER_BINARY_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_WRITER_BINARY_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_BUFFER_OUTPUT {
    pub output: WS_XML_WRITER_OUTPUT,
}
impl ::core::marker::Copy for WS_XML_WRITER_BUFFER_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_BUFFER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_BUFFER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_BUFFER_OUTPUT").field("output", &self.output).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_BUFFER_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_BUFFER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.output == other.output
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_BUFFER_OUTPUT {}
impl ::core::default::Default for WS_XML_WRITER_BUFFER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_ENCODING {
    pub encodingType: WS_XML_WRITER_ENCODING_TYPE,
}
impl ::core::marker::Copy for WS_XML_WRITER_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_ENCODING").field("encodingType", &self.encodingType).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encodingType == other.encodingType
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_ENCODING {}
impl ::core::default::Default for WS_XML_WRITER_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_WRITER_MTOM_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
    pub textEncoding: *mut WS_XML_WRITER_ENCODING,
    pub writeMimeHeader: super::super::Foundation::BOOL,
    pub boundary: WS_STRING,
    pub startInfo: WS_STRING,
    pub startUri: WS_STRING,
    pub maxInlineByteCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_XML_WRITER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_XML_WRITER_MTOM_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_WRITER_MTOM_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_MTOM_ENCODING").field("encoding", &self.encoding).field("textEncoding", &self.textEncoding).field("writeMimeHeader", &self.writeMimeHeader).field("boundary", &self.boundary).field("startInfo", &self.startInfo).field("startUri", &self.startUri).field("maxInlineByteCount", &self.maxInlineByteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WS_XML_WRITER_MTOM_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_WRITER_MTOM_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.textEncoding == other.textEncoding && self.writeMimeHeader == other.writeMimeHeader && self.boundary == other.boundary && self.startInfo == other.startInfo && self.startUri == other.startUri && self.maxInlineByteCount == other.maxInlineByteCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_WRITER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_WRITER_MTOM_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_OUTPUT {
    pub outputType: WS_XML_WRITER_OUTPUT_TYPE,
}
impl ::core::marker::Copy for WS_XML_WRITER_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_OUTPUT").field("outputType", &self.outputType).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.outputType == other.outputType
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_OUTPUT {}
impl ::core::default::Default for WS_XML_WRITER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_PROPERTIES {
    pub properties: *mut WS_XML_WRITER_PROPERTY,
    pub propertyCount: u32,
}
impl ::core::marker::Copy for WS_XML_WRITER_PROPERTIES {}
impl ::core::clone::Clone for WS_XML_WRITER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_PROPERTIES {}
impl ::core::default::Default for WS_XML_WRITER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_PROPERTY {
    pub id: WS_XML_WRITER_PROPERTY_ID,
    pub value: *mut ::core::ffi::c_void,
    pub valueSize: u32,
}
impl ::core::marker::Copy for WS_XML_WRITER_PROPERTY {}
impl ::core::clone::Clone for WS_XML_WRITER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_PROPERTY {}
impl ::core::default::Default for WS_XML_WRITER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_RAW_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
}
impl ::core::marker::Copy for WS_XML_WRITER_RAW_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_RAW_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_RAW_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_RAW_ENCODING").field("encoding", &self.encoding).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_RAW_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_RAW_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_RAW_ENCODING {}
impl ::core::default::Default for WS_XML_WRITER_RAW_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_STREAM_OUTPUT {
    pub output: WS_XML_WRITER_OUTPUT,
    pub writeCallback: WS_WRITE_CALLBACK,
    pub writeCallbackState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WS_XML_WRITER_STREAM_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_STREAM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_STREAM_OUTPUT").field("output", &self.output).field("writeCallbackState", &self.writeCallbackState).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_STREAM_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WS_XML_WRITER_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub struct WS_XML_WRITER_TEXT_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
    pub charSet: WS_CHARSET,
}
impl ::core::marker::Copy for WS_XML_WRITER_TEXT_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_TEXT_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_TEXT_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_TEXT_ENCODING").field("encoding", &self.encoding).field("charSet", &self.charSet).finish()
    }
}
impl ::windows::core::TypeKind for WS_XML_WRITER_TEXT_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_TEXT_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.charSet == other.charSet
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_TEXT_ENCODING {}
impl ::core::default::Default for WS_XML_WRITER_TEXT_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ABANDON_MESSAGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ABORT_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ABORT_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ACCEPT_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ASYNC_CALLBACK = ::core::option::Option<unsafe extern "system" fn(errorcode: ::windows::core::HRESULT, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ASYNC_FUNCTION = ::core::option::Option<unsafe extern "system" fn(hr: ::windows::core::HRESULT, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, next: *mut WS_ASYNC_OPERATION, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type WS_CERTIFICATE_VALIDATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(certcontext: *const super::super::Security::Cryptography::CERT_CONTEXT, state: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type WS_CERT_ISSUER_LIST_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(certissuerlistnotificationcallbackstate: *const ::core::ffi::c_void, issuerlist: *const super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CLOSE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CLOSE_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channeltype: WS_CHANNEL_TYPE, channelparameters: *const ::core::ffi::c_void, channelparameterssize: u32, channelinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_CHANNEL_FOR_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, channelparameters: *const ::core::ffi::c_void, channelparameterssize: u32, channelinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_DECODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(createcontext: *const ::core::ffi::c_void, readcallback: WS_READ_CALLBACK, readcontext: *const ::core::ffi::c_void, decodercontext: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_ENCODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(createcontext: *const ::core::ffi::c_void, writecallback: WS_WRITE_CALLBACK, writecontext: *const ::core::ffi::c_void, encodercontext: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_CREATE_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channeltype: WS_CHANNEL_TYPE, listenerparameters: *const ::core::ffi::c_void, listenerparameterssize: u32, listenerinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_DECODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, maxlength: u32, length: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_GET_CONTENT_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(decodercontext: *const ::core::ffi::c_void, contenttype: *const WS_STRING, contentencoding: *const WS_STRING, newcontenttype: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_DECODER_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_DURATION_COMPARISON_CALLBACK = ::core::option::Option<unsafe extern "system" fn(duration1: *const WS_DURATION, duration2: *const WS_DURATION, result: *mut i32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_DYNAMIC_STRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, string: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, id: *mut u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_ENCODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, buffers: *const WS_BYTES, count: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_GET_CONTENT_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, contenttype: *const WS_STRING, newcontenttype: *mut WS_STRING, contentencoding: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_ENCODER_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_DECODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(decodercontext: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_ENCODER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_FREE_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type WS_GET_CERT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(getcertcallbackstate: *const ::core::ffi::c_void, targetaddress: *const WS_ENDPOINT_ADDRESS, viauri: *const WS_STRING, cert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_GET_CHANNEL_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_GET_LISTENER_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_HTTP_REDIRECT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(state: *const ::core::ffi::c_void, originalurl: *const WS_STRING, newurl: *const WS_STRING) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_IS_DEFAULT_VALUE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(descriptiondata: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, defaultvalue: *const ::core::ffi::c_void, valuesize: u32, isdefault: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_MESSAGE_DONE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(donecallbackstate: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPEN_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPEN_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, url: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPERATION_CANCEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(reason: WS_SERVICE_CANCEL_REASON, state: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_OPERATION_FREE_STATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(state: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_PROXY_MESSAGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(message: *const WS_MESSAGE, heap: *const WS_HEAP, state: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_PULL_BYTES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, bytes: *mut ::core::ffi::c_void, maxsize: u32, actualsize: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_PUSH_BYTES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, bytes: *mut ::core::ffi::c_void, maxsize: u32, actualsize: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_MESSAGE_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_MESSAGE_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_READ_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, descriptiondata: *const ::core::ffi::c_void, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_RESET_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_RESET_LISTENER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_ACCEPT_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, channelstate: *mut *mut ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_CLOSE_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, asynccontext: *const WS_ASYNC_CONTEXT) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_MESSAGE_RECEIVE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WS_SERVICE_SECURITY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, authorized: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SERVICE_STUB_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, frame: *const ::core::ffi::c_void, callback: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SET_CHANNEL_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SET_LISTENER_PROPERTY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_SHUTDOWN_SESSION_CHANNEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_VALIDATE_PASSWORD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(passwordvalidatorcallbackstate: *const ::core::ffi::c_void, username: *const WS_STRING, password: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_VALIDATE_SAML_CALLBACK = ::core::option::Option<unsafe extern "system" fn(samlvalidatorcallbackstate: *const ::core::ffi::c_void, samlassertion: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, buffers: *const WS_BYTES, count: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_MESSAGE_END_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_MESSAGE_START_CALLBACK = ::core::option::Option<unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`*"]
pub type WS_WRITE_TYPE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, descriptiondata: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
