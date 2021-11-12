#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNAuthenticatorGetAssertion(hwnd: super::super::Foundation::HWND, pwszrpid: super::super::Foundation::PWSTR, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion: *mut *mut WEBAUTHN_ASSERTION) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNAuthenticatorMakeCredential(
        hwnd: super::super::Foundation::HWND,
        prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION,
        puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION,
        ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
        pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA,
        pwebauthnmakecredentialoptions: *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS,
        ppwebauthncredentialattestation: *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION,
    ) -> ::windows_sys::core::HRESULT;
    pub fn WebAuthNCancelCurrentOperation(pcancellationid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: *const WEBAUTHN_CREDENTIAL_ATTESTATION);
    pub fn WebAuthNGetApiVersionNumber() -> u32;
    pub fn WebAuthNGetCancellationId(pcancellationid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNGetErrorName(hr: ::windows_sys::core::HRESULT) -> super::super::Foundation::PWSTR;
    pub fn WebAuthNGetW3CExceptionDOMError(hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn WsAbandonCall(serviceproxy: *const WS_SERVICE_PROXY, callid: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAbandonMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAbortChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAbortListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAbortServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAbortServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAcceptChannel(listener: *const WS_LISTENER, channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddCustomHeader(message: *const WS_MESSAGE, headerdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, headerattributes: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddErrorString(error: *const WS_ERROR, string: *const WS_STRING) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddressMessage(message: *const WS_MESSAGE, address: *const WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAlloc(heap: *const WS_HEAP, size: usize, ptr: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsAsyncExecute(asyncstate: *const WS_ASYNC_STATE, operation: WS_ASYNC_FUNCTION, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCall(serviceproxy: *const WS_SERVICE_PROXY, operation: *const WS_OPERATION_DESCRIPTION, arguments: *const *const ::core::ffi::c_void, heap: *const WS_HEAP, callproperties: *const WS_CALL_PROPERTY, callpropertycount: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCheckMustUnderstandHeaders(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCloseChannel(channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCloseListener(listener: *const WS_LISTENER, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCloseServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCloseServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCombineUrl(baseurl: *const WS_STRING, referenceurl: *const WS_STRING, flags: u32, heap: *const WS_HEAP, resulturl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCopyError(source: *const WS_ERROR, destination: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCopyNode(writer: *const WS_XML_WRITER, reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateChannel(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: *const WS_CHANNEL_PROPERTY, propertycount: u32, securitydescription: *const WS_SECURITY_DESCRIPTION, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateChannelForListener(listener: *const WS_LISTENER, properties: *const WS_CHANNEL_PROPERTY, propertycount: u32, channel: *mut *mut WS_CHANNEL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateError(properties: *const WS_ERROR_PROPERTY, propertycount: u32, error: *mut *mut WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateFaultFromError(error: *const WS_ERROR, faulterrorcode: ::windows_sys::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, heap: *const WS_HEAP, fault: *mut WS_FAULT) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateHeap(maxsize: usize, trimsize: usize, properties: *const WS_HEAP_PROPERTY, propertycount: u32, heap: *mut *mut WS_HEAP, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateListener(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, properties: *const WS_LISTENER_PROPERTY, propertycount: u32, securitydescription: *const WS_SECURITY_DESCRIPTION, listener: *mut *mut WS_LISTENER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateMessage(envelopeversion: WS_ENVELOPE_VERSION, addressingversion: WS_ADDRESSING_VERSION, properties: *const WS_MESSAGE_PROPERTY, propertycount: u32, message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateMessageForChannel(channel: *const WS_CHANNEL, properties: *const WS_MESSAGE_PROPERTY, propertycount: u32, message: *mut *mut WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateMetadata(properties: *const WS_METADATA_PROPERTY, propertycount: u32, metadata: *mut *mut WS_METADATA, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateReader(properties: *const WS_XML_READER_PROPERTY, propertycount: u32, reader: *mut *mut WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateServiceEndpointFromTemplate(
        channeltype: WS_CHANNEL_TYPE,
        properties: *const WS_SERVICE_ENDPOINT_PROPERTY,
        propertycount: u32,
        addressurl: *const WS_STRING,
        contract: *const WS_SERVICE_CONTRACT,
        authorizationcallback: WS_SERVICE_SECURITY_CALLBACK,
        heap: *const WS_HEAP,
        templatetype: WS_BINDING_TEMPLATE_TYPE,
        templatevalue: *const ::core::ffi::c_void,
        templatesize: u32,
        templatedescription: *const ::core::ffi::c_void,
        templatedescriptionsize: u32,
        serviceendpoint: *mut *mut WS_SERVICE_ENDPOINT,
        error: *const WS_ERROR,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateServiceHost(endpoints: *const *const WS_SERVICE_ENDPOINT, endpointcount: u16, serviceproperties: *const WS_SERVICE_PROPERTY, servicepropertycount: u32, servicehost: *mut *mut WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateServiceProxy(channeltype: WS_CHANNEL_TYPE, channelbinding: WS_CHANNEL_BINDING, securitydescription: *const WS_SECURITY_DESCRIPTION, properties: *const WS_PROXY_PROPERTY, propertycount: u32, channelproperties: *const WS_CHANNEL_PROPERTY, channelpropertycount: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateServiceProxyFromTemplate(channeltype: WS_CHANNEL_TYPE, properties: *const WS_PROXY_PROPERTY, propertycount: u32, templatetype: WS_BINDING_TEMPLATE_TYPE, templatevalue: *const ::core::ffi::c_void, templatesize: u32, templatedescription: *const ::core::ffi::c_void, templatedescriptionsize: u32, serviceproxy: *mut *mut WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateWriter(properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, writer: *mut *mut WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateXmlBuffer(heap: *const WS_HEAP, properties: *const WS_XML_BUFFER_PROPERTY, propertycount: u32, buffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsCreateXmlSecurityToken(tokenxml: *const WS_XML_BUFFER, tokenkey: *const WS_SECURITY_KEY_HANDLE, properties: *const WS_XML_SECURITY_TOKEN_PROPERTY, propertycount: u32, token: *mut *mut WS_SECURITY_TOKEN, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsDateTimeToFileTime(datetime: *const WS_DATETIME, filetime: *mut super::super::Foundation::FILETIME, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsDecodeUrl(url: *const WS_STRING, flags: u32, heap: *const WS_HEAP, outurl: *mut *mut WS_URL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsEncodeUrl(url: *const WS_URL, flags: u32, heap: *const WS_HEAP, outurl: *mut WS_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsEndReaderCanonicalization(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsEndWriterCanonicalization(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsFileTimeToDateTime(filetime: *const super::super::Foundation::FILETIME, datetime: *mut WS_DATETIME, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsFillBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsFillReader(reader: *const WS_XML_READER, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsFindAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, required: super::super::Foundation::BOOL, attributeindex: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsFlushBody(message: *const WS_MESSAGE, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsFlushWriter(writer: *const WS_XML_WRITER, minsize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsFreeChannel(channel: *const WS_CHANNEL);
    pub fn WsFreeError(error: *const WS_ERROR);
    pub fn WsFreeHeap(heap: *const WS_HEAP);
    pub fn WsFreeListener(listener: *const WS_LISTENER);
    pub fn WsFreeMessage(message: *const WS_MESSAGE);
    pub fn WsFreeMetadata(metadata: *const WS_METADATA);
    pub fn WsFreeReader(reader: *const WS_XML_READER);
    pub fn WsFreeSecurityToken(token: *const WS_SECURITY_TOKEN);
    pub fn WsFreeServiceHost(servicehost: *const WS_SERVICE_HOST);
    pub fn WsFreeServiceProxy(serviceproxy: *const WS_SERVICE_PROXY);
    pub fn WsFreeWriter(writer: *const WS_XML_WRITER);
    pub fn WsGetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetCustomHeader(message: *const WS_MESSAGE, customheaderdescription: *const WS_ELEMENT_DESCRIPTION, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetDictionary(encoding: WS_ENCODING, dictionary: *mut *mut WS_XML_DICTIONARY, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetErrorString(error: *const WS_ERROR, index: u32, string: *mut WS_STRING) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WsGetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WsGetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetHeaderAttributes(message: *const WS_MESSAGE, reader: *const WS_XML_READER, headerattributes: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetHeapProperty(heap: *const WS_HEAP, id: WS_HEAP_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, repeatingoption: WS_REPEATING_HEADER_OPTION, headerindex: u32, valuetype: WS_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMetadataEndpoints(metadata: *const WS_METADATA, endpoints: *mut WS_METADATA_ENDPOINTS, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetMetadataProperty(metadata: *const WS_METADATA, id: WS_METADATA_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMissingMetadataDocumentAddress(metadata: *const WS_METADATA, address: *mut *mut WS_ENDPOINT_ADDRESS, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetNamespaceFromPrefix(reader: *const WS_XML_READER, prefix: *const WS_XML_STRING, required: super::super::Foundation::BOOL, ns: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetOperationContextProperty(context: *const WS_OPERATION_CONTEXT, id: WS_OPERATION_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetPolicyAlternativeCount(policy: *const WS_POLICY, count: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetPolicyProperty(policy: *const WS_POLICY, id: WS_POLICY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetPrefixFromNamespace(writer: *const WS_XML_WRITER, ns: *const WS_XML_STRING, required: super::super::Foundation::BOOL, prefix: *mut *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetReaderNode(xmlreader: *const WS_XML_READER, node: *mut *mut WS_XML_NODE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetReaderPosition(reader: *const WS_XML_READER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetReaderProperty(reader: *const WS_XML_READER, id: WS_XML_READER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetSecurityContextProperty(securitycontext: *const WS_SECURITY_CONTEXT, id: WS_SECURITY_CONTEXT_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetSecurityTokenProperty(securitytoken: *const WS_SECURITY_TOKEN, id: WS_SECURITY_TOKEN_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetServiceHostProperty(servicehost: *const WS_SERVICE_HOST, id: WS_SERVICE_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetServiceProxyProperty(serviceproxy: *const WS_SERVICE_PROXY, id: WS_PROXY_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *mut WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsGetWriterProperty(writer: *const WS_XML_WRITER, id: WS_XML_WRITER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetXmlAttribute(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, heap: *const WS_HEAP, valuechars: *mut *mut u16, valuecharcount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsInitializeMessage(message: *const WS_MESSAGE, initialization: WS_MESSAGE_INITIALIZATION, sourcemessage: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsMarkHeaderAsUnderstood(message: *const WS_MESSAGE, headerposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMatchPolicyAlternative(policy: *const WS_POLICY, alternativeindex: u32, policyconstraints: *const WS_POLICY_CONSTRAINTS, matchrequired: super::super::Foundation::BOOL, heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMoveReader(reader: *const WS_XML_READER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMoveWriter(writer: *const WS_XML_WRITER, moveto: WS_MOVE_TO, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenChannel(channel: *const WS_CHANNEL, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenListener(listener: *const WS_LISTENER, url: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsOpenServiceHost(servicehost: *const WS_SERVICE_HOST, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, address: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsPullBytes(writer: *const WS_XML_WRITER, callback: WS_PULL_BYTES_CALLBACK, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsPushBytes(writer: *const WS_XML_WRITER, callback: WS_PUSH_BYTES_CALLBACK, callbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadArray(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *mut ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, actualitemcount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadAttribute(reader: *const WS_XML_READER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadBytes(reader: *const WS_XML_READER, bytes: *mut ::core::ffi::c_void, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadChars(reader: *const WS_XML_READER, chars: super::super::Foundation::PWSTR, maxcharcount: u32, actualcharcount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadCharsUtf8(reader: *const WS_XML_READER, bytes: *mut u8, maxbytecount: u32, actualbytecount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadElement(reader: *const WS_XML_READER, elementdescription: *const WS_ELEMENT_DESCRIPTION, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadEndAttribute(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadEndElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadEndpointAddressExtension(reader: *const WS_XML_READER, endpointaddress: *const WS_ENDPOINT_ADDRESS, extensiontype: WS_ENDPOINT_ADDRESS_EXTENSION_TYPE, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadEnvelopeStart(message: *const WS_MESSAGE, reader: *const WS_XML_READER, donecallback: WS_MESSAGE_DONE_CALLBACK, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadMetadata(metadata: *const WS_METADATA, reader: *const WS_XML_READER, url: *const WS_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadQualifiedName(reader: *const WS_XML_READER, heap: *const WS_HEAP, prefix: *mut WS_XML_STRING, localname: *mut WS_XML_STRING, ns: *mut WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadStartAttribute(reader: *const WS_XML_READER, attributeindex: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadStartElement(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadToStartElement(reader: *const WS_XML_READER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadType(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, readoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadValue(reader: *const WS_XML_READER, valuetype: WS_VALUE_TYPE, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadXmlBuffer(reader: *const WS_XML_READER, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsReadXmlBufferFromBytes(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, bytes: *const ::core::ffi::c_void, bytecount: u32, heap: *const WS_HEAP, xmlbuffer: *mut *mut WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReceiveMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescriptions: *const *const WS_MESSAGE_DESCRIPTION, messagedescriptioncount: u32, receiveoption: WS_RECEIVE_OPTION, readbodyoption: WS_READ_OPTION, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, index: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsRegisterOperationForCancel(context: *const WS_OPERATION_CONTEXT, cancelcallback: WS_OPERATION_CANCEL_CALLBACK, freestatecallback: WS_OPERATION_FREE_STATE_CALLBACK, userstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRemoveCustomHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, headerns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsRemoveHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRemoveMappedHeader(message: *const WS_MESSAGE, headername: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsRemoveNode(nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
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
        asynccontext: *const WS_ASYNC_CONTEXT,
        error: *const WS_ERROR,
    ) -> ::windows_sys::core::HRESULT;
    pub fn WsRequestSecurityToken(channel: *const WS_CHANNEL, properties: *const WS_REQUEST_SECURITY_TOKEN_PROPERTY, propertycount: u32, token: *mut *mut WS_SECURITY_TOKEN, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetChannel(channel: *const WS_CHANNEL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetError(error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetHeap(heap: *const WS_HEAP, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetListener(listener: *const WS_LISTENER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetMessage(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetMetadata(metadata: *const WS_METADATA, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetServiceHost(servicehost: *const WS_SERVICE_HOST, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsResetServiceProxy(serviceproxy: *const WS_SERVICE_PROXY, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsRevokeSecurityContext(securitycontext: *const WS_SECURITY_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSendFaultMessageForError(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, faulterror: *const WS_ERROR, faulterrorcode: ::windows_sys::core::HRESULT, faultdisclosure: WS_FAULT_DISCLOSURE, requestmessage: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSendMessage(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, messagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, bodyvalue: *const ::core::ffi::c_void, bodyvaluesize: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSendReplyMessage(channel: *const WS_CHANNEL, replymessage: *const WS_MESSAGE, replymessagedescription: *const WS_MESSAGE_DESCRIPTION, writeoption: WS_WRITE_OPTION, replybodyvalue: *const ::core::ffi::c_void, replybodyvaluesize: u32, requestmessage: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetChannelProperty(channel: *const WS_CHANNEL, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetErrorProperty(error: *const WS_ERROR, id: WS_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSetFaultErrorDetail(error: *const WS_ERROR, faultdetaildescription: *const WS_FAULT_DETAIL_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WsSetFaultErrorProperty(error: *const WS_ERROR, id: WS_FAULT_ERROR_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WsSetHeader(message: *const WS_MESSAGE, headertype: WS_HEADER_TYPE, valuetype: WS_TYPE, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetInput(reader: *const WS_XML_READER, encoding: *const WS_XML_READER_ENCODING, input: *const WS_XML_READER_INPUT, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetInputToBuffer(reader: *const WS_XML_READER, buffer: *const WS_XML_BUFFER, properties: *const WS_XML_READER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetListenerProperty(listener: *const WS_LISTENER, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetMessageProperty(message: *const WS_MESSAGE, id: WS_MESSAGE_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetOutput(writer: *const WS_XML_WRITER, encoding: *const WS_XML_WRITER_ENCODING, output: *const WS_XML_WRITER_OUTPUT, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetOutputToBuffer(writer: *const WS_XML_WRITER, buffer: *const WS_XML_BUFFER, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetReaderPosition(reader: *const WS_XML_READER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSetWriterPosition(writer: *const WS_XML_WRITER, nodeposition: *const WS_XML_NODE_POSITION, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsShutdownSessionChannel(channel: *const WS_CHANNEL, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsSkipNode(reader: *const WS_XML_READER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsStartReaderCanonicalization(reader: *const WS_XML_READER, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, properties: *const WS_XML_CANONICALIZATION_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsStartWriterCanonicalization(writer: *const WS_XML_WRITER, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, properties: *const WS_XML_CANONICALIZATION_PROPERTY, propertycount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsTrimXmlWhitespace(chars: super::super::Foundation::PWSTR, charcount: u32, trimmedchars: *mut *mut u16, trimmedcount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsVerifyXmlNCName(ncnamechars: super::super::Foundation::PWSTR, ncnamecharcount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteArray(writer: *const WS_XML_WRITER, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, valuetype: WS_VALUE_TYPE, array: *const ::core::ffi::c_void, arraysize: u32, itemoffset: u32, itemcount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteAttribute(writer: *const WS_XML_WRITER, attributedescription: *const WS_ATTRIBUTE_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteBody(message: *const WS_MESSAGE, bodydescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteBytes(writer: *const WS_XML_WRITER, bytes: *const ::core::ffi::c_void, bytecount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteChars(writer: *const WS_XML_WRITER, chars: super::super::Foundation::PWSTR, charcount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteCharsUtf8(writer: *const WS_XML_WRITER, bytes: *const u8, bytecount: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteElement(writer: *const WS_XML_WRITER, elementdescription: *const WS_ELEMENT_DESCRIPTION, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteEndAttribute(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteEndCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteEndElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteEndStartElement(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteEnvelopeEnd(message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteEnvelopeStart(message: *const WS_MESSAGE, writer: *const WS_XML_WRITER, donecallback: WS_MESSAGE_DONE_CALLBACK, donecallbackstate: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteMessageEnd(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteMessageStart(channel: *const WS_CHANNEL, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteNode(writer: *const WS_XML_WRITER, node: *const WS_XML_NODE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteQualifiedName(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteStartAttribute(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteStartCData(writer: *const WS_XML_WRITER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteStartElement(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, localname: *const WS_XML_STRING, ns: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteText(writer: *const WS_XML_WRITER, text: *const WS_XML_TEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteType(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, r#type: WS_TYPE, typedescription: *const ::core::ffi::c_void, writeoption: WS_WRITE_OPTION, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteValue(writer: *const WS_XML_WRITER, valuetype: WS_VALUE_TYPE, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteXmlBuffer(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    pub fn WsWriteXmlBufferToBytes(writer: *const WS_XML_WRITER, xmlbuffer: *const WS_XML_BUFFER, encoding: *const WS_XML_WRITER_ENCODING, properties: *const WS_XML_WRITER_PROPERTY, propertycount: u32, heap: *const WS_HEAP, bytes: *mut *mut ::core::ffi::c_void, bytecount: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteXmlnsAttribute(writer: *const WS_XML_WRITER, prefix: *const WS_XML_STRING, ns: *const WS_XML_STRING, singlequote: super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsXmlStringEquals(string1: *const WS_XML_STRING, string2: *const WS_XML_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IContentPrefetcherTaskTrigger(pub *mut ::core::ffi::c_void);
pub const WEBAUTHN_API_CURRENT_VERSION: u32 = 3u32;
pub const WEBAUTHN_API_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_API_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_API_VERSION_3: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_ASSERTION(i32);
pub const WEBAUTHN_ASSERTION_CURRENT_VERSION: u32 = 2u32;
pub const WEBAUTHN_ASSERTION_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_ASSERTION_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY: u32 = 0u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT: u32 = 3u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT: u32 = 2u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE: u32 = 1u32;
pub const WEBAUTHN_ATTESTATION_DECODE_COMMON: u32 = 1u32;
pub const WEBAUTHN_ATTESTATION_DECODE_NONE: u32 = 0u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: u32 = 0u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS(i32);
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: u32 = 5u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS(i32);
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: u32 = 4u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CLIENT_DATA(i32);
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_COMMON_ATTESTATION(i32);
pub const WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256: i32 = -7i32;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P384_WITH_SHA384: i32 = -35i32;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P521_WITH_SHA512: i32 = -36i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256: i32 = -257i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA384: i32 = -258i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA512: i32 = -259i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA256: i32 = -37i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA384: i32 = -38i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA512: i32 = -39i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS(i32);
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CREDENTIAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CREDENTIALS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION(i32);
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: u32 = 4u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CREDENTIAL_EX(i32);
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CREDENTIAL_LIST(i32);
#[repr(C)]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION(i32);
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE: u32 = 3u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET: u32 = 1u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE: u32 = 0u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET: u32 = 2u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR: u32 = 9u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA: u32 = 3u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER: u32 = 4u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE: u32 = 7u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS: u32 = 6u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE: u32 = 0u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND: u32 = 5u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED: u32 = 2u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR: u32 = 8u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN(i32);
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: u32 = 4u32;
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 31u32;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16u32;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2u32;
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8u32;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_EXTENSION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_EXTENSIONS(i32);
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: u32 = 0u32;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: u32 = 2u32;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: u32 = 1u32;
pub const WEBAUTHN_MAX_USER_ID_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION(i32);
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION(i32);
pub const WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_USER_VERIFICATION_ANY: u32 = 0u32;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL: u32 = 1u32;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST: u32 = 2u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIRED: u32 = 3u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY: u32 = 0u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED: u32 = 3u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED: u32 = 2u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED: u32 = 1u32;
#[repr(C)]
pub struct WEBAUTHN_X5C(i32);
#[repr(C)]
pub struct WS_ABANDON_MESSAGE_CALLBACK(i32);
#[repr(C)]
pub struct WS_ABORT_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_ABORT_LISTENER_CALLBACK(i32);
#[repr(C)]
pub struct WS_ACCEPT_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_ADDRESSING_VERSION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ANY_ATTRIBUTE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ANY_ATTRIBUTES(i32);
#[repr(C)]
pub struct WS_ASYNC_CALLBACK(i32);
#[repr(C)]
pub struct WS_ASYNC_CONTEXT(i32);
#[repr(C)]
pub struct WS_ASYNC_FUNCTION(i32);
#[repr(C)]
pub struct WS_ASYNC_OPERATION(i32);
#[repr(C)]
pub struct WS_ASYNC_STATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ATTRIBUTE_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_BINDING_TEMPLATE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_BOOL_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_BUFFERS(i32);
#[repr(C)]
pub struct WS_BYTES(i32);
#[repr(C)]
pub struct WS_BYTES_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_BYTE_ARRAY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_CALLBACK_MODEL(i32);
#[repr(C)]
pub struct WS_CALL_PROPERTY(i32);
#[repr(C)]
pub struct WS_CALL_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE(i32);
#[repr(C)]
pub struct WS_CERTIFICATE_VALIDATION_CALLBACK(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT(i32);
#[repr(C)]
pub struct WS_CERT_CREDENTIAL(i32);
#[repr(C)]
pub struct WS_CERT_CREDENTIAL_TYPE(i32);
#[repr(C)]
pub struct WS_CERT_ENDPOINT_IDENTITY(i32);
pub const WS_CERT_FAILURE_CN_MISMATCH: i32 = 1i32;
pub const WS_CERT_FAILURE_INVALID_DATE: i32 = 2i32;
pub const WS_CERT_FAILURE_REVOCATION_OFFLINE: i32 = 16i32;
pub const WS_CERT_FAILURE_UNTRUSTED_ROOT: i32 = 4i32;
pub const WS_CERT_FAILURE_WRONG_USAGE: i32 = 8i32;
#[repr(C)]
pub struct WS_CERT_ISSUER_LIST_NOTIFICATION_CALLBACK(i32);
#[repr(C)]
pub struct WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WS_CERT_SIGNED_SAML_AUTHENTICATOR(i32);
#[repr(C)]
pub struct WS_CHANNEL(i32);
#[repr(C)]
pub struct WS_CHANNEL_BINDING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CHANNEL_DECODER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CHANNEL_ENCODER(i32);
#[repr(C)]
pub struct WS_CHANNEL_PROPERTIES(i32);
#[repr(C)]
pub struct WS_CHANNEL_PROPERTY(i32);
#[repr(C)]
pub struct WS_CHANNEL_PROPERTY_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_CHANNEL_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_CHANNEL_STATE(i32);
#[repr(C)]
pub struct WS_CHANNEL_TYPE(i32);
#[repr(C)]
pub struct WS_CHARSET(i32);
#[repr(C)]
pub struct WS_CHAR_ARRAY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_CLOSE_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_CLOSE_LISTENER_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CONTRACT_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_COOKIE_MODE(i32);
#[repr(C)]
pub struct WS_CREATE_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_CREATE_CHANNEL_FOR_LISTENER_CALLBACK(i32);
#[repr(C)]
pub struct WS_CREATE_DECODER_CALLBACK(i32);
#[repr(C)]
pub struct WS_CREATE_ENCODER_CALLBACK(i32);
#[repr(C)]
pub struct WS_CREATE_LISTENER_CALLBACK(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WS_CUSTOM_CERT_CREDENTIAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CUSTOM_CHANNEL_CALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CUSTOM_HTTP_PROXY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CUSTOM_LISTENER_CALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_CUSTOM_TYPE_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_DATETIME(i32);
#[repr(C)]
pub struct WS_DATETIME_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_DATETIME_FORMAT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_DECIMAL_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_DECODER_DECODE_CALLBACK(i32);
#[repr(C)]
pub struct WS_DECODER_END_CALLBACK(i32);
#[repr(C)]
pub struct WS_DECODER_GET_CONTENT_TYPE_CALLBACK(i32);
#[repr(C)]
pub struct WS_DECODER_START_CALLBACK(i32);
#[repr(C)]
pub struct WS_DEFAULT_VALUE(i32);
#[repr(C)]
pub struct WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_DISALLOWED_USER_AGENT_SUBSTRINGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_DNS_ENDPOINT_IDENTITY(i32);
#[repr(C)]
pub struct WS_DOUBLE_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_DURATION(i32);
#[repr(C)]
pub struct WS_DURATION_COMPARISON_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_DURATION_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_DYNAMIC_STRING_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ELEMENT_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_ENCODER_ENCODE_CALLBACK(i32);
#[repr(C)]
pub struct WS_ENCODER_END_CALLBACK(i32);
#[repr(C)]
pub struct WS_ENCODER_GET_CONTENT_TYPE_CALLBACK(i32);
#[repr(C)]
pub struct WS_ENCODER_START_CALLBACK(i32);
#[repr(C)]
pub struct WS_ENCODING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ENDPOINT_ADDRESS(i32);
#[repr(C)]
pub struct WS_ENDPOINT_ADDRESS_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_ENDPOINT_ADDRESS_EXTENSION_TYPE(i32);
#[repr(C)]
pub struct WS_ENDPOINT_IDENTITY(i32);
#[repr(C)]
pub struct WS_ENDPOINT_IDENTITY_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ENDPOINT_POLICY_EXTENSION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ENUM_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ENUM_VALUE(i32);
#[repr(C)]
pub struct WS_ENVELOPE_VERSION(i32);
#[repr(C)]
pub struct WS_ERROR(i32);
#[repr(C)]
pub struct WS_ERROR_PROPERTY(i32);
#[repr(C)]
pub struct WS_ERROR_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_EXCEPTION_CODE(i32);
#[repr(C)]
pub struct WS_EXTENDED_PROTECTION_POLICY(i32);
#[repr(C)]
pub struct WS_EXTENDED_PROTECTION_SCENARIO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_FAULT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_FAULT_CODE(i32);
#[repr(C)]
pub struct WS_FAULT_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_FAULT_DETAIL_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_FAULT_DISCLOSURE(i32);
#[repr(C)]
pub struct WS_FAULT_ERROR_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_FAULT_REASON(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_FIELD_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_FIELD_MAPPING(i32);
pub const WS_FIELD_NILLABLE: i32 = 4i32;
pub const WS_FIELD_NILLABLE_ITEM: i32 = 8i32;
pub const WS_FIELD_OPTIONAL: i32 = 2i32;
pub const WS_FIELD_OTHER_NAMESPACE: i32 = 16i32;
pub const WS_FIELD_POINTER: i32 = 1i32;
#[repr(C)]
pub struct WS_FLOAT_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_FREE_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_FREE_DECODER_CALLBACK(i32);
#[repr(C)]
pub struct WS_FREE_ENCODER_CALLBACK(i32);
#[repr(C)]
pub struct WS_FREE_LISTENER_CALLBACK(i32);
#[repr(C)]
pub struct WS_GET_CERT_CALLBACK(i32);
#[repr(C)]
pub struct WS_GET_CHANNEL_PROPERTY_CALLBACK(i32);
#[repr(C)]
pub struct WS_GET_LISTENER_PROPERTY_CALLBACK(i32);
#[repr(C)]
pub struct WS_GUID_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_HEADER_TYPE(i32);
#[repr(C)]
pub struct WS_HEAP(i32);
#[repr(C)]
pub struct WS_HEAP_PROPERTIES(i32);
#[repr(C)]
pub struct WS_HEAP_PROPERTY(i32);
#[repr(C)]
pub struct WS_HEAP_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HOST_NAMES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTPS_URL(i32);
#[repr(C)]
pub struct WS_HTTP_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION(i32);
pub const WS_HTTP_HEADER_AUTH_SCHEME_BASIC: i32 = 2i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_DIGEST: i32 = 4i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_NEGOTIATE: i32 = 16i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_NONE: i32 = 1i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_NTLM: i32 = 8i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_PASSPORT: i32 = 32i32;
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_TARGET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTP_HEADER_MAPPING(i32);
pub const WS_HTTP_HEADER_MAPPING_COMMA_SEPARATOR: i32 = 1i32;
pub const WS_HTTP_HEADER_MAPPING_QUOTED_VALUE: i32 = 4i32;
pub const WS_HTTP_HEADER_MAPPING_SEMICOLON_SEPARATOR: i32 = 2i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTP_MESSAGE_MAPPING(i32);
#[repr(C)]
pub struct WS_HTTP_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_HTTP_PROXY_SETTING_MODE(i32);
#[repr(C)]
pub struct WS_HTTP_REDIRECT_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTP_REDIRECT_CALLBACK_CONTEXT(i32);
pub const WS_HTTP_REQUEST_MAPPING_VERB: i32 = 2i32;
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_CODE: i32 = 1i32;
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_TEXT: i32 = 2i32;
#[repr(C)]
pub struct WS_HTTP_SSL_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_HTTP_URL(i32);
#[repr(C)]
pub struct WS_INT16_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_INT32_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_INT64_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_INT8_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_IP_VERSION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_IS_DEFAULT_VALUE_CALLBACK(i32);
#[repr(C)]
pub struct WS_ITEM_RANGE(i32);
#[repr(C)]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_LISTENER(i32);
#[repr(C)]
pub struct WS_LISTENER_PROPERTIES(i32);
#[repr(C)]
pub struct WS_LISTENER_PROPERTY(i32);
#[repr(C)]
pub struct WS_LISTENER_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_LISTENER_STATE(i32);
pub const WS_MATCH_URL_DNS_FULLY_QUALIFIED_HOST: i32 = 2i32;
pub const WS_MATCH_URL_DNS_HOST: i32 = 1i32;
pub const WS_MATCH_URL_EXACT_PATH: i32 = 64i32;
pub const WS_MATCH_URL_HOST_ADDRESSES: i32 = 16i32;
pub const WS_MATCH_URL_LOCAL_HOST: i32 = 8i32;
pub const WS_MATCH_URL_NETBIOS_HOST: i32 = 4i32;
pub const WS_MATCH_URL_NO_QUERY: i32 = 256i32;
pub const WS_MATCH_URL_PORT: i32 = 32i32;
pub const WS_MATCH_URL_PREFIX_PATH: i32 = 128i32;
pub const WS_MATCH_URL_THIS_HOST: i32 = 31i32;
#[repr(C)]
pub struct WS_MESSAGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_MESSAGE_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_MESSAGE_DONE_CALLBACK(i32);
#[repr(C)]
pub struct WS_MESSAGE_INITIALIZATION(i32);
#[repr(C)]
pub struct WS_MESSAGE_PROPERTIES(i32);
#[repr(C)]
pub struct WS_MESSAGE_PROPERTY(i32);
#[repr(C)]
pub struct WS_MESSAGE_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_MESSAGE_SECURITY_USAGE(i32);
#[repr(C)]
pub struct WS_MESSAGE_STATE(i32);
#[repr(C)]
pub struct WS_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_METADATA_ENDPOINT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_METADATA_ENDPOINTS(i32);
#[repr(C)]
pub struct WS_METADATA_EXCHANGE_TYPE(i32);
#[repr(C)]
pub struct WS_METADATA_PROPERTY(i32);
#[repr(C)]
pub struct WS_METADATA_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_METADATA_STATE(i32);
#[repr(C)]
pub struct WS_MOVE_TO(i32);
pub const WS_MUST_UNDERSTAND_HEADER_ATTRIBUTE: i32 = 1i32;
#[repr(C)]
pub struct WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_NETPIPE_URL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_NETTCP_URL(i32);
#[repr(C)]
pub struct WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL(i32);
#[repr(C)]
pub struct WS_OPEN_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_OPEN_LISTENER_CALLBACK(i32);
#[repr(C)]
pub struct WS_OPERATION_CANCEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_OPERATION_CONTEXT(i32);
#[repr(C)]
pub struct WS_OPERATION_CONTEXT_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_OPERATION_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_OPERATION_FREE_STATE_CALLBACK(i32);
#[repr(C)]
pub struct WS_OPERATION_STYLE(i32);
#[repr(C)]
pub struct WS_PARAMETER_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_PARAMETER_TYPE(i32);
#[repr(C)]
pub struct WS_POLICY(i32);
#[repr(C)]
pub struct WS_POLICY_CONSTRAINTS(i32);
#[repr(C)]
pub struct WS_POLICY_EXTENSION(i32);
#[repr(C)]
pub struct WS_POLICY_EXTENSION_TYPE(i32);
#[repr(C)]
pub struct WS_POLICY_PROPERTIES(i32);
#[repr(C)]
pub struct WS_POLICY_PROPERTY(i32);
#[repr(C)]
pub struct WS_POLICY_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_POLICY_STATE(i32);
#[repr(C)]
pub struct WS_PROTECTION_LEVEL(i32);
#[repr(C)]
pub struct WS_PROXY_MESSAGE_CALLBACK(i32);
#[repr(C)]
pub struct WS_PROXY_MESSAGE_CALLBACK_CONTEXT(i32);
#[repr(C)]
pub struct WS_PROXY_PROPERTY(i32);
#[repr(C)]
pub struct WS_PROXY_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_PULL_BYTES_CALLBACK(i32);
#[repr(C)]
pub struct WS_PUSH_BYTES_CALLBACK(i32);
#[repr(C)]
pub struct WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE(i32);
#[repr(C)]
pub struct WS_READ_CALLBACK(i32);
#[repr(C)]
pub struct WS_READ_MESSAGE_END_CALLBACK(i32);
#[repr(C)]
pub struct WS_READ_MESSAGE_START_CALLBACK(i32);
#[repr(C)]
pub struct WS_READ_OPTION(i32);
#[repr(C)]
pub struct WS_READ_TYPE_CALLBACK(i32);
#[repr(C)]
pub struct WS_RECEIVE_OPTION(i32);
pub const WS_RELAY_HEADER_ATTRIBUTE: i32 = 2i32;
#[repr(C)]
pub struct WS_REPEATING_HEADER_OPTION(i32);
#[repr(C)]
pub struct WS_REQUEST_SECURITY_TOKEN_ACTION(i32);
#[repr(C)]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY(i32);
#[repr(C)]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_RESET_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_RESET_LISTENER_CALLBACK(i32);
#[repr(C)]
pub struct WS_RSA_ENDPOINT_IDENTITY(i32);
#[repr(C)]
pub struct WS_SAML_AUTHENTICATOR(i32);
#[repr(C)]
pub struct WS_SAML_AUTHENTICATOR_TYPE(i32);
#[repr(C)]
pub struct WS_SAML_MESSAGE_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_SECURE_CONVERSATION_VERSION(i32);
#[repr(C)]
pub struct WS_SECURE_PROTOCOL(i32);
#[repr(C)]
pub struct WS_SECURITY_ALGORITHM_ID(i32);
#[repr(C)]
pub struct WS_SECURITY_ALGORITHM_PROPERTY(i32);
#[repr(C)]
pub struct WS_SECURITY_ALGORITHM_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SECURITY_ALGORITHM_SUITE(i32);
#[repr(C)]
pub struct WS_SECURITY_ALGORITHM_SUITE_NAME(i32);
#[repr(C)]
pub struct WS_SECURITY_BEARER_KEY_TYPE_VERSION(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_CONSTRAINT_TYPE(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_PROPERTIES(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_PROPERTY(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_PROPERTY_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SECURITY_BINDING_TYPE(i32);
#[repr(C)]
pub struct WS_SECURITY_CONSTRAINTS(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_PROPERTY(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_SECURITY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_SECURITY_HEADER_LAYOUT(i32);
#[repr(C)]
pub struct WS_SECURITY_HEADER_VERSION(i32);
#[repr(C)]
pub struct WS_SECURITY_KEY_ENTROPY_MODE(i32);
#[repr(C)]
pub struct WS_SECURITY_KEY_HANDLE(i32);
#[repr(C)]
pub struct WS_SECURITY_KEY_HANDLE_TYPE(i32);
#[repr(C)]
pub struct WS_SECURITY_KEY_TYPE(i32);
#[repr(C)]
pub struct WS_SECURITY_PROPERTIES(i32);
#[repr(C)]
pub struct WS_SECURITY_PROPERTY(i32);
#[repr(C)]
pub struct WS_SECURITY_PROPERTY_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_SECURITY_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SECURITY_TIMESTAMP_USAGE(i32);
#[repr(C)]
pub struct WS_SECURITY_TOKEN(i32);
#[repr(C)]
pub struct WS_SECURITY_TOKEN_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SECURITY_TOKEN_REFERENCE_MODE(i32);
#[repr(C)]
pub struct WS_SERVICE_ACCEPT_CHANNEL_CALLBACK(i32);
#[repr(C)]
pub struct WS_SERVICE_CANCEL_REASON(i32);
#[repr(C)]
pub struct WS_SERVICE_CLOSE_CHANNEL_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SERVICE_CONTRACT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SERVICE_ENDPOINT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SERVICE_ENDPOINT_METADATA(i32);
#[repr(C)]
pub struct WS_SERVICE_ENDPOINT_PROPERTY(i32);
#[repr(C)]
pub struct WS_SERVICE_ENDPOINT_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SERVICE_HOST(i32);
#[repr(C)]
pub struct WS_SERVICE_HOST_STATE(i32);
#[repr(C)]
pub struct WS_SERVICE_MESSAGE_RECEIVE_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SERVICE_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SERVICE_METADATA_DOCUMENT(i32);
pub const WS_SERVICE_OPERATION_MESSAGE_NILLABLE_ELEMENT: i32 = 1i32;
#[repr(C)]
pub struct WS_SERVICE_PROPERTY(i32);
#[repr(C)]
pub struct WS_SERVICE_PROPERTY_ACCEPT_CALLBACK(i32);
#[repr(C)]
pub struct WS_SERVICE_PROPERTY_CLOSE_CALLBACK(i32);
#[repr(C)]
pub struct WS_SERVICE_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_SERVICE_PROXY(i32);
#[repr(C)]
pub struct WS_SERVICE_PROXY_STATE(i32);
#[repr(C)]
pub struct WS_SERVICE_SECURITY_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SERVICE_SECURITY_IDENTITIES(i32);
#[repr(C)]
pub struct WS_SERVICE_STUB_CALLBACK(i32);
#[repr(C)]
pub struct WS_SET_CHANNEL_PROPERTY_CALLBACK(i32);
#[repr(C)]
pub struct WS_SET_LISTENER_PROPERTY_CALLBACK(i32);
#[repr(C)]
pub struct WS_SHUTDOWN_SESSION_CHANNEL_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SOAPUDP_URL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SPN_ENDPOINT_IDENTITY(i32);
#[repr(C)]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_STRING(i32);
#[repr(C)]
pub struct WS_STRING_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_STRING_USERNAME_CREDENTIAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL(i32);
pub const WS_STRUCT_ABSTRACT: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_STRUCT_DESCRIPTION(i32);
pub const WS_STRUCT_IGNORE_TRAILING_ELEMENT_CONTENT: i32 = 2i32;
pub const WS_STRUCT_IGNORE_UNHANDLED_ATTRIBUTES: i32 = 4i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_SUBJECT_NAME_CERT_CREDENTIAL(i32);
#[repr(C)]
pub struct WS_TCP_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_TCP_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_POLICY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_THUMBPRINT_CERT_CREDENTIAL(i32);
#[repr(C)]
pub struct WS_TIMESPAN(i32);
#[repr(C)]
pub struct WS_TIMESPAN_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_TRACE_API(i32);
#[repr(C)]
pub struct WS_TRANSFER_MODE(i32);
#[repr(C)]
pub struct WS_TRUST_VERSION(i32);
#[repr(C)]
pub struct WS_TYPE(i32);
#[repr(C)]
pub struct WS_TYPE_MAPPING(i32);
#[repr(C)]
pub struct WS_UINT16_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_UINT32_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_UINT64_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_UINT8_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_UNION_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_UNION_FIELD_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_UNIQUE_ID(i32);
#[repr(C)]
pub struct WS_UNIQUE_ID_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_UNKNOWN_ENDPOINT_IDENTITY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_UPN_ENDPOINT_IDENTITY(i32);
#[repr(C)]
pub struct WS_URL(i32);
pub const WS_URL_FLAGS_ALLOW_HOST_WILDCARDS: i32 = 1i32;
pub const WS_URL_FLAGS_NO_PATH_COLLAPSE: i32 = 2i32;
pub const WS_URL_FLAGS_ZERO_TERMINATE: i32 = 4i32;
#[repr(C)]
pub struct WS_URL_SCHEME_TYPE(i32);
#[repr(C)]
pub struct WS_USERNAME_CREDENTIAL(i32);
#[repr(C)]
pub struct WS_USERNAME_CREDENTIAL_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT(i32);
#[repr(C)]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE(i32);
#[repr(C)]
pub struct WS_UTF8_ARRAY_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_VALIDATE_PASSWORD_CALLBACK(i32);
#[repr(C)]
pub struct WS_VALIDATE_SAML_CALLBACK(i32);
#[repr(C)]
pub struct WS_VALUE_TYPE(i32);
#[repr(C)]
pub struct WS_VOID_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL(i32);
#[repr(C)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE(i32);
#[repr(C)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_PACKAGE(i32);
#[repr(C)]
pub struct WS_WRITE_CALLBACK(i32);
#[repr(C)]
pub struct WS_WRITE_MESSAGE_END_CALLBACK(i32);
#[repr(C)]
pub struct WS_WRITE_MESSAGE_START_CALLBACK(i32);
#[repr(C)]
pub struct WS_WRITE_OPTION(i32);
#[repr(C)]
pub struct WS_WRITE_TYPE_CALLBACK(i32);
#[repr(C)]
pub struct WS_WSZ_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_ATTRIBUTE(i32);
#[repr(C)]
pub struct WS_XML_BASE64_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_BOOL_TEXT(i32);
#[repr(C)]
pub struct WS_XML_BUFFER(i32);
#[repr(C)]
pub struct WS_XML_BUFFER_PROPERTY(i32);
#[repr(C)]
pub struct WS_XML_BUFFER_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_XML_CANONICALIZATION_ALGORITHM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES(i32);
#[repr(C)]
pub struct WS_XML_CANONICALIZATION_PROPERTY(i32);
#[repr(C)]
pub struct WS_XML_CANONICALIZATION_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_COMMENT_NODE(i32);
#[repr(C)]
pub struct WS_XML_DATETIME_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_DECIMAL_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_DICTIONARY(i32);
#[repr(C)]
pub struct WS_XML_DOUBLE_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_ELEMENT_NODE(i32);
#[repr(C)]
pub struct WS_XML_FLOAT_TEXT(i32);
#[repr(C)]
pub struct WS_XML_GUID_TEXT(i32);
#[repr(C)]
pub struct WS_XML_INT32_TEXT(i32);
#[repr(C)]
pub struct WS_XML_INT64_TEXT(i32);
#[repr(C)]
pub struct WS_XML_LIST_TEXT(i32);
#[repr(C)]
pub struct WS_XML_NODE(i32);
#[repr(C)]
pub struct WS_XML_NODE_POSITION(i32);
#[repr(C)]
pub struct WS_XML_NODE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_QNAME(i32);
#[repr(C)]
pub struct WS_XML_QNAME_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_QNAME_TEXT(i32);
#[repr(C)]
pub struct WS_XML_READER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_READER_BINARY_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_READER_BUFFER_INPUT(i32);
#[repr(C)]
pub struct WS_XML_READER_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_READER_ENCODING_TYPE(i32);
#[repr(C)]
pub struct WS_XML_READER_INPUT(i32);
#[repr(C)]
pub struct WS_XML_READER_INPUT_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_READER_MTOM_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_READER_PROPERTIES(i32);
#[repr(C)]
pub struct WS_XML_READER_PROPERTY(i32);
#[repr(C)]
pub struct WS_XML_READER_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_XML_READER_RAW_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_READER_STREAM_INPUT(i32);
#[repr(C)]
pub struct WS_XML_READER_TEXT_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_SECURITY_TOKEN_PROPERTY(i32);
#[repr(C)]
pub struct WS_XML_SECURITY_TOKEN_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_STRING(i32);
#[repr(C)]
pub struct WS_XML_STRING_DESCRIPTION(i32);
#[repr(C)]
pub struct WS_XML_TEXT(i32);
#[repr(C)]
pub struct WS_XML_TEXT_NODE(i32);
#[repr(C)]
pub struct WS_XML_TEXT_TYPE(i32);
#[repr(C)]
pub struct WS_XML_TIMESPAN_TEXT(i32);
#[repr(C)]
pub struct WS_XML_TOKEN_MESSAGE_SECURITY_BINDING(i32);
#[repr(C)]
pub struct WS_XML_UINT64_TEXT(i32);
#[repr(C)]
pub struct WS_XML_UNIQUE_ID_TEXT(i32);
#[repr(C)]
pub struct WS_XML_UTF16_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_UTF8_TEXT(i32);
#[repr(C)]
pub struct WS_XML_WRITER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_WRITER_BINARY_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_WRITER_BUFFER_OUTPUT(i32);
#[repr(C)]
pub struct WS_XML_WRITER_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_WRITER_ENCODING_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WS_XML_WRITER_MTOM_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_WRITER_OUTPUT(i32);
#[repr(C)]
pub struct WS_XML_WRITER_OUTPUT_TYPE(i32);
#[repr(C)]
pub struct WS_XML_WRITER_PROPERTIES(i32);
#[repr(C)]
pub struct WS_XML_WRITER_PROPERTY(i32);
#[repr(C)]
pub struct WS_XML_WRITER_PROPERTY_ID(i32);
#[repr(C)]
pub struct WS_XML_WRITER_RAW_ENCODING(i32);
#[repr(C)]
pub struct WS_XML_WRITER_STREAM_OUTPUT(i32);
#[repr(C)]
pub struct WS_XML_WRITER_TEXT_ENCODING(i32);
