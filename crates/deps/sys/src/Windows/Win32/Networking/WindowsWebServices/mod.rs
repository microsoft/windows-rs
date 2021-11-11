#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNAuthenticatorGetAssertion(hwnd: super::super::Foundation::HWND, pwszrpid: super::super::Foundation::PWSTR, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion: *mut *mut WEBAUTHN_ASSERTION) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNAuthenticatorMakeCredential(
        hwnd: super::super::Foundation::HWND,
        prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION,
        puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION,
        ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
        pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA,
        pwebauthnmakecredentialoptions: *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS,
        ppwebauthncredentialattestation: *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNCancelCurrentOperation(pcancellationid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: *const WEBAUTHN_CREDENTIAL_ATTESTATION);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNGetApiVersionNumber() -> u32;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNGetCancellationId(pcancellationid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNGetErrorName(hr: ::windows::runtime::HRESULT) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNGetW3CExceptionDOMError(hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbandonCall(serviceproxy: *const WS_SERVICE_PROXY, callid: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbandonMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAcceptChannel(listener: *const WS_LISTENER, channel: *const WS_CHANNEL, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddCustomHeader(message: *const WS_MESSAGE, headerdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, headerattributes: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddErrorString(error: *const WS_ERROR, string: *const WS_STRING) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddressMessage(message: *const WS_MESSAGE, address: *const WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAlloc(heap: *const WS_HEAP, size: usize, ptr: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAsyncExecute(asyncstate: *const WS_ASYNC_STATE, operation: ::windows::runtime::RawPtr, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCall(serviceproxy: *const WS_SERVICE_PROXY, operation: *const ::core::mem::ManuallyDrop<WS_OPERATION_DESCRIPTION>, arguments: *const *const ::core::ffi::c_void, heap: *const WS_HEAP, callproperties: *const WS_CALL_PROPERTY, callpropertycount: u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCheckMustUnderstandHeaders(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseChannel(channel: *const WS_CHANNEL, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseListener(listener: *const WS_LISTENER, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCombineUrl(baseurl: *const WS_STRING, referenceurl: *const WS_STRING, flags: u32, heap: *const WS_HEAP, resulturl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCopyError(source: *const WS_ERROR, destination: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCopyNode(writer: *const WS_XML_WRITER, reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateChannel(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: *const WS_CHANNEL_PROPERTY, propertycount: u32, securitydescription: *const WS_SECURITY_DESCRIPTION, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateChannelForListener(listener: *const WS_LISTENER, properties: *const WS_CHANNEL_PROPERTY, propertycount: u32, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateError(properties: *const WS_ERROR_PROPERTY, propertycount: u32, error: *mut *mut WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateFaultFromError(error: *const WS_ERROR, faulterrorcode: ::windows::runtime::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, heap: *const WS_HEAP, fault: *mut WS_FAULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateHeap(maxsize: usize, trimsize: usize, properties: *const WS_HEAP_PROPERTY, propertycount: u32, heap: *mut *mut WS_HEAP, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateListener(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: *const WS_LISTENER_PROPERTY, propertycount: u32, securitydescription: *const WS_SECURITY_DESCRIPTION, listener: *mut *mut WS_LISTENER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateMessage(envelopeversion: WS_ENVELOPE_VERSION, addressingversion: WS_ADDRESSING_VERSION, properties: *const WS_MESSAGE_PROPERTY, propertycount: u32, message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateMessageForChannel(channel: *const WS_CHANNEL, properties: *const WS_MESSAGE_PROPERTY, propertycount: u32, message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateMetadata(properties: *const WS_METADATA_PROPERTY, propertycount: u32, metadata: *mut *mut WS_METADATA, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateReader(properties: *const WS_XML_READER_PROPERTY, propertycount: u32, reader: *mut *mut WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateServiceEndpointFromTemplate(
        channeltype: WS_CHANNEL_TYPE,
        properties: *const WS_SERVICE_ENDPOINT_PROPERTY,
        propertycount: u32,
        addressurl: *const WS_STRING,
        contract: *const ::core::mem::ManuallyDrop<WS_SERVICE_CONTRACT>,
        authorizationcallback: ::windows::runtime::RawPtr,
        heap: *const WS_HEAP,
        templatetype: WS_BINDING_TEMPLATE_TYPE,
        templatevalue: *const ::core::ffi::c_void,
        templatesize: u32,
        templatedescription: *const ::core::ffi::c_void,
        templatedescriptionsize: u32,
        serviceendpoint: *mut *mut WS_SERVICE_ENDPOINT,
        error: *const WS_ERROR,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateServiceHost(endpoints: *const *const WS_SERVICE_ENDPOINT, endpointcount: u16, serviceproperties: *const WS_SERVICE_PROPERTY, servicepropertycount: u32, servicehost: *mut *mut WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateServiceProxy(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, securitydescription: *const WS_SECURITY_DESCRIPTION, properties: *const WS_PROXY_PROPERTY, propertycount: u32, channelproperties: *const WS_CHANNEL_PROPERTY, channelpropertycount: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateServiceProxyFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: *const WS_PROXY_PROPERTY, propertycount: u32, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: *const ::core::ffi::c_void, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateWriter(properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, writer: *mut *mut WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateXmlBuffer(heap: *const WS_HEAP, properties: *const WS_XML_BUFFER_PROPERTY, propertycount: u32, buffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateXmlSecurityToken(tokenxml: *const WS_XML_BUFFER, tokenkey: *const WS_SECURITY_KEY_HANDLE, properties: *const WS_XML_SECURITY_TOKEN_PROPERTY, propertycount: u32, token: *mut *mut WS_SECURITY_TOKEN, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsDateTimeToFileTime(datetime: *const WS_DATETIME, filetime: *mut super::super::Foundation::FILETIME, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsDecodeUrl(url: *const WS_STRING, flags: u32, heap: *const WS_HEAP, outurl: *mut *mut WS_URL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsEncodeUrl(url: *const WS_URL, flags: u32, heap: *const WS_HEAP, outurl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsEndReaderCanonicalization(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsEndWriterCanonicalization(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsFileTimeToDateTime(filetime: *const super::super::Foundation::FILETIME, datetime: *mut WS_DATETIME, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFillBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFillReader(reader: *const WS_XML_READER, minsize: u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsFindAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, required: super::super::Foundation::BOOL, attributeindex: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFlushBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFlushWriter(writer: *const WS_XML_WRITER, minsize: u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeChannel(channel: *const WS_CHANNEL);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeError(error: *const WS_ERROR);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeHeap(heap: *const WS_HEAP);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeListener(listener: *const WS_LISTENER);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeMessage(message: *const WS_MESSAGE);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeMetadata(metadata: *const WS_METADATA);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeReader(reader: *const WS_XML_READER);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeSecurityToken(token: *const WS_SECURITY_TOKEN);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeServiceHost(servicehost: *const WS_SERVICE_HOST);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeServiceProxy(serviceproxy: *const WS_SERVICE_PROXY);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeWriter(writer: *const WS_XML_WRITER);
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetCustomHeader(message: *const WS_MESSAGE, customheaderdescription: *const WS_ELEMENT_DESCRIPTION, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetDictionary(encoding: WS_ENCODING, dictionary: *mut *mut WS_XML_DICTIONARY, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetErrorString(error: *const WS_ERROR, index: u32, string: *mut WS_STRING) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetHeaderAttributes(message: *const WS_MESSAGE, reader: *const WS_XML_READER, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetHeapProperty(heap: *const WS_HEAP, id: WS_HEAP_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMetadataEndpoints(metadata: *const WS_METADATA, endpoints: *mut WS_METADATA_ENDPOINTS, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetMetadataProperty(metadata: *const WS_METADATA, id: WS_METADATA_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMissingMetadataDocumentAddress(metadata: *const WS_METADATA, address: *mut *mut WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetNamespaceFromPrefix(reader: *const WS_XML_READER, prefix: *const WS_XML_STRING, required: super::super::Foundation::BOOL, ns: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetOperationContextProperty(context: *const WS_OPERATION_CONTEXT, id: WS_OPERATION_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetPolicyAlternativeCount(policy: *const WS_POLICY, count: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetPolicyProperty(policy: *const WS_POLICY, id: WS_POLICY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetPrefixFromNamespace(writer: *const WS_XML_WRITER, ns: *const WS_XML_STRING, required: super::super::Foundation::BOOL, prefix: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetReaderNode(xmlreader: *const WS_XML_READER, node: *mut *mut WS_XML_NODE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetReaderPosition(reader: *const WS_XML_READER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetReaderProperty(reader: *const WS_XML_READER, id: WS_XML_READER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetSecurityContextProperty(securitycontext: *const WS_SECURITY_CONTEXT, id: WS_SECURITY_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetSecurityTokenProperty(securitytoken: *const WS_SECURITY_TOKEN, id: WS_SECURITY_TOKEN_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetServiceHostProperty(servicehost: *const WS_SERVICE_HOST, id: WS_SERVICE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetServiceProxyProperty(serviceproxy: *const WS_SERVICE_PROXY, id: WS_PROXY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetWriterProperty(writer: *const WS_XML_WRITER, id: WS_XML_WRITER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetXmlAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, heap: *const WS_HEAP, valuechars: *mut *mut u16, valuecharcount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsInitializeMessage(message: *const WS_MESSAGE, initialization: WS_MESSAGE_INITIALIZATION, sourcemessage: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsMarkHeaderAsUnderstood(message: *const WS_MESSAGE, headerposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMatchPolicyAlternative(policy: *const WS_POLICY, alternativeindex: u32, policyconstraints: *const WS_POLICY_CONSTRAINTS, matchrequired: super::super::Foundation::BOOL, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMoveReader(reader: *const WS_XML_READER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMoveWriter(writer: *const WS_XML_WRITER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenChannel(channel: *const WS_CHANNEL, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenListener(listener: *const WS_LISTENER, url: *const WS_STRING, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsOpenServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, address: *const WS_ENDPOINT_ADDRESS, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsPullBytes(writer: *const WS_XML_WRITER, callback: ::windows::runtime::RawPtr, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsPushBytes(writer: *const WS_XML_WRITER, callback: ::windows::runtime::RawPtr, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadArray(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *mut ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, actualitemcount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadAttribute(reader: *const WS_XML_READER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadBytes(reader: *const WS_XML_READER, bytes: *mut ::core::ffi::c_void, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadChars(reader: *const WS_XML_READER, chars: super::super::Foundation::PWSTR, maxcharcount: u32, actualcharcount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadCharsUtf8(reader: *const WS_XML_READER, bytes: *mut u8, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadElement(reader: *const WS_XML_READER, elementdescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEndAttribute(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEndElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadEndpointAddressExtension(reader: *const WS_XML_READER, endpointaddress: *const WS_ENDPOINT_ADDRESS, extensiontype: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEnvelopeStart(message: *const WS_MESSAGE, reader: *const WS_XML_READER, donecallback: ::windows::runtime::RawPtr, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadMetadata(metadata: *const WS_METADATA, reader: *const WS_XML_READER, url: *const WS_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadQualifiedName(reader: *const WS_XML_READER, heap: *const WS_HEAP, prefix: *mut WS_XML_STRING, localname: *mut WS_XML_STRING, ns: *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadStartAttribute(reader: *const WS_XML_READER, attributeindex: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadStartElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadToStartElement(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadType(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadValue(reader: *const WS_XML_READER, valuetype: WS_VALUE_TYPE, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadXmlBuffer(reader: *const WS_XML_READER, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadXmlBufferFromBytes(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, bytes: *const ::core::ffi::c_void, bytecount: u32, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReceiveMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescriptions: *const *const WS_MESSAGE_DESCRIPTION, messagedescriptioncount: u32, receiveoption: WS_RECEIVE_OPTION, readbodyoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, index: *mut u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRegisterOperationForCancel(context: *const WS_OPERATION_CONTEXT, cancelcallback: ::windows::runtime::RawPtr, freestatecallback: ::windows::runtime::RawPtr, userstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRemoveCustomHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, headerns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRemoveHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRemoveMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRemoveNode(nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRequestReply(
        channel: *const WS_CHANNEL,
        requestmessage: *const WS_MESSAGE,
        requestmessagedescription: *const WS_MESSAGE_DESCRIPTION,
        writeoption: WS_WRITE_OPTION,
        requestbodyvalue: *const ::core::ffi::c_void,
        requestbodyvaluesize: u32,
        replymessage: *const WS_MESSAGE,
        replymessagedescription: *const WS_MESSAGE_DESCRIPTION,
        readoption: WS_READ_OPTION,
        heap: *const WS_HEAP,
        value: *mut ::core::ffi::c_void,
        valuesize: u32,
        asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>,
        error: *const WS_ERROR,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRequestSecurityToken(channel: *const WS_CHANNEL, properties: *const WS_REQUEST_SECURITY_TOKEN_PROPERTY, propertycount: u32, token: *mut *mut WS_SECURITY_TOKEN, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetError(error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetHeap(heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetMessage(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetMetadata(metadata: *const WS_METADATA, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRevokeSecurityContext(securitycontext: *const WS_SECURITY_CONTEXT, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSendFaultMessageForError(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, faulterror: *const WS_ERROR, faulterrorcode: ::windows::runtime::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, requestmessage: *const WS_MESSAGE, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSendMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, bodyvalue: *const ::core::ffi::c_void, bodyvaluesize: u32, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSendReplyMessage(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, replybodyvalue: *const ::core::ffi::c_void, replybodyvaluesize: u32, requestmessage: *const WS_MESSAGE, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetInput(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, input: *const WS_XML_READER_INPUT, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetInputToBuffer(reader: *const WS_XML_READER, buffer: *const WS_XML_BUFFER, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetOutput(writer: *const WS_XML_WRITER, encoding: *const WS_XML_WRITER_ENCODING, output: *const WS_XML_WRITER_OUTPUT, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetOutputToBuffer(writer: *const WS_XML_WRITER, buffer: *const WS_XML_BUFFER, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetReaderPosition(reader: *const WS_XML_READER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsShutdownSessionChannel(channel: *const WS_CHANNEL, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSkipNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsStartReaderCanonicalization(reader: *const WS_XML_READER, writecallback: ::windows::runtime::RawPtr, writecallbackstate: *const ::core::ffi::c_void, properties: *const WS_XML_CANONICALIZATION_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsStartWriterCanonicalization(writer: *const WS_XML_WRITER, writecallback: ::windows::runtime::RawPtr, writecallbackstate: *const ::core::ffi::c_void, properties: *const WS_XML_CANONICALIZATION_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsTrimXmlWhitespace(chars: super::super::Foundation::PWSTR, charcount: u32, trimmedchars: *mut *mut u16, trimmedcount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsVerifyXmlNCName(ncnamechars: super::super::Foundation::PWSTR, ncnamecharcount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteArray(writer: *const WS_XML_WRITER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *const ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteAttribute(writer: *const WS_XML_WRITER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteBytes(writer: *const WS_XML_WRITER, bytes: *const ::core::ffi::c_void, bytecount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteChars(writer: *const WS_XML_WRITER, chars: super::super::Foundation::PWSTR, charcount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteCharsUtf8(writer: *const WS_XML_WRITER, bytes: *const u8, bytecount: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteElement(writer: *const WS_XML_WRITER, elementdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndAttribute(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndStartElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEnvelopeStart(message: *const WS_MESSAGE, writer: *const WS_XML_WRITER, donecallback: ::windows::runtime::RawPtr, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const ::core::mem::ManuallyDrop<WS_ASYNC_CONTEXT>, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteNode(writer: *const WS_XML_WRITER, node: *const WS_XML_NODE, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteQualifiedName(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteStartAttribute(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteStartCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteStartElement(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteText(writer: *const WS_XML_WRITER, text: *const WS_XML_TEXT, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteType(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteValue(writer: *const WS_XML_WRITER, valuetype: WS_VALUE_TYPE, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteXmlBuffer(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteXmlBufferToBytes(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, encoding: *const WS_XML_WRITER_ENCODING, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, heap: *const WS_HEAP, bytes: *mut *mut ::core::ffi::c_void, bytecount: *mut u32, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteXmlnsAttribute(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsXmlStringEquals(string1: *const WS_XML_STRING, string2: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows::runtime::HRESULT;
}
