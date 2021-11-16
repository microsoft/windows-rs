#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for IContentPrefetcherTaskTrigger {}
impl ::core::clone::Clone for IContentPrefetcherTaskTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_API_CURRENT_VERSION: u32 = 3u32;
pub const WEBAUTHN_API_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_API_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_API_VERSION_3: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_ASSERTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_ASSERTION {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub dwUserVerificationRequirement: u32,
    pub dwFlags: u32,
    pub pwszU2fAppId: super::super::Foundation::PWSTR,
    pub pbU2fAppId: *mut super::super::Foundation::BOOL,
    pub pCancellationId: *mut ::windows_sys::core::GUID,
    pub pAllowCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: u32 = 5u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: u32 = 5u32;
#[repr(C)]
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
    pub pCancellationId: *mut ::windows_sys::core::GUID,
    pub pExcludeCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: u32 = 4u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub pwszHashAlgId: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: super::super::Foundation::PWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub cX5c: u32,
    pub pX5c: *mut WEBAUTHN_X5C,
    pub pwszVer: super::super::Foundation::PWSTR,
    pub cbCertInfo: u32,
    pub pbCertInfo: *mut u8,
    pub cbPubArea: u32,
    pub pbPubArea: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_COMMON_ATTESTATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_COMMON_ATTESTATION {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: super::super::Foundation::PWSTR,
    pub lAlg: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: u32,
    pub pCredentialParameters: *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: u32,
    pub pCredentials: *mut WEBAUTHN_CREDENTIAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: super::super::Foundation::PWSTR,
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
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: u32 = 4u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: super::super::Foundation::PWSTR,
    pub dwTransports: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut *mut WEBAUTHN_CREDENTIAL_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_CREDENTIAL_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_CREDENTIAL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: u32 = 4u32;
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 31u32;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16u32;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2u32;
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8u32;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: super::super::Foundation::PWSTR,
    pub cbExtension: u32,
    pub pvExtension: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: *mut WEBAUTHN_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_EXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_EXTENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: u32 = 0u32;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: u32 = 2u32;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: u32 = 1u32;
pub const WEBAUTHN_MAX_USER_ID_LENGTH: u32 = 64u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: super::super::Foundation::PWSTR,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszIcon: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_RP_ENTITY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszIcon: super::super::Foundation::PWSTR,
    pub pwszDisplayName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WEBAUTHN_USER_ENTITY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub type WS_ABANDON_MESSAGE_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_ABORT_CHANNEL_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_ABORT_LISTENER_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_ACCEPT_CHANNEL_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_ADDRESSING_VERSION_0_9: i32 = 1i32;
pub const WS_ADDRESSING_VERSION_1_0: i32 = 2i32;
pub const WS_ADDRESSING_VERSION_TRANSPORT: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
pub type WS_ASYNC_CALLBACK = unsafe extern "system" fn(errorcode: ::windows_sys::core::HRESULT, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void);
#[repr(C)]
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
pub type WS_ASYNC_FUNCTION = unsafe extern "system" fn(hr: ::windows_sys::core::HRESULT, callbackmodel: WS_CALLBACK_MODEL, callbackstate: *const ::core::ffi::c_void, next: *mut WS_ASYNC_OPERATION, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub struct WS_ASYNC_OPERATION {
    pub function: WS_ASYNC_FUNCTION,
}
impl ::core::marker::Copy for WS_ASYNC_OPERATION {}
impl ::core::clone::Clone for WS_ASYNC_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
pub const WS_HTTP_BINDING_TEMPLATE_TYPE: i32 = 0i32;
pub const WS_HTTP_SSL_BINDING_TEMPLATE_TYPE: i32 = 1i32;
pub const WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE_TYPE: i32 = 2i32;
pub const WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE_TYPE: i32 = 3i32;
pub const WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE_TYPE: i32 = 4i32;
pub const WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE_TYPE: i32 = 5i32;
pub const WS_TCP_BINDING_TEMPLATE_TYPE: i32 = 6i32;
pub const WS_TCP_SSPI_BINDING_TEMPLATE_TYPE: i32 = 7i32;
pub const WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE_TYPE: i32 = 8i32;
pub const WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE_TYPE: i32 = 9i32;
pub const WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: i32 = 10i32;
pub const WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: i32 = 11i32;
pub const WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: i32 = 12i32;
pub const WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE_TYPE: i32 = 13i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_SHORT_CALLBACK: i32 = 0i32;
pub const WS_LONG_CALLBACK: i32 = 1i32;
#[repr(C)]
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
pub const WS_CALL_PROPERTY_CHECK_MUST_UNDERSTAND: i32 = 0i32;
pub const WS_CALL_PROPERTY_SEND_MESSAGE_CONTEXT: i32 = 1i32;
pub const WS_CALL_PROPERTY_RECEIVE_MESSAGE_CONTEXT: i32 = 2i32;
pub const WS_CALL_PROPERTY_CALL_ID: i32 = 3i32;
#[repr(C)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type WS_CERTIFICATE_VALIDATION_CALLBACK = unsafe extern "system" fn(certcontext: *const super::super::Security::Cryptography::CERT_CONTEXT, state: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
pub struct WS_CERT_CREDENTIAL {
    pub credentialType: WS_CERT_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_CERT_CREDENTIAL {}
impl ::core::clone::Clone for WS_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_SUBJECT_NAME_CERT_CREDENTIAL_TYPE: i32 = 1i32;
pub const WS_THUMBPRINT_CERT_CREDENTIAL_TYPE: i32 = 2i32;
pub const WS_CUSTOM_CERT_CREDENTIAL_TYPE: i32 = 3i32;
#[repr(C)]
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
pub const WS_CERT_FAILURE_CN_MISMATCH: i32 = 1i32;
pub const WS_CERT_FAILURE_INVALID_DATE: i32 = 2i32;
pub const WS_CERT_FAILURE_REVOCATION_OFFLINE: i32 = 16i32;
pub const WS_CERT_FAILURE_UNTRUSTED_ROOT: i32 = 4i32;
pub const WS_CERT_FAILURE_WRONG_USAGE: i32 = 8i32;
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type WS_CERT_ISSUER_LIST_NOTIFICATION_CALLBACK = unsafe extern "system" fn(certissuerlistnotificationcallbackstate: *const ::core::ffi::c_void, issuerlist: *const super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    pub authenticator: WS_SAML_AUTHENTICATOR,
    pub trustedIssuerCerts: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub trustedIssuerCertCount: u32,
    pub decryptionCert: *mut super::super::Security::Cryptography::CERT_CONTEXT,
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
#[repr(C)]
pub struct WS_CHANNEL(pub u8);
pub const WS_HTTP_CHANNEL_BINDING: i32 = 0i32;
pub const WS_TCP_CHANNEL_BINDING: i32 = 1i32;
pub const WS_UDP_CHANNEL_BINDING: i32 = 2i32;
pub const WS_CUSTOM_CHANNEL_BINDING: i32 = 3i32;
pub const WS_NAMEDPIPE_CHANNEL_BINDING: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CHANNEL_DECODER {
    pub createContext: *mut ::core::ffi::c_void,
    pub createDecoderCallback: WS_CREATE_DECODER_CALLBACK,
    pub decoderGetContentTypeCallback: WS_DECODER_GET_CONTENT_TYPE_CALLBACK,
    pub decoderStartCallback: WS_DECODER_START_CALLBACK,
    pub decoderDecodeCallback: WS_DECODER_DECODE_CALLBACK,
    pub decoderEndCallback: WS_DECODER_END_CALLBACK,
    pub freeDecoderCallback: WS_FREE_DECODER_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CHANNEL_DECODER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CHANNEL_DECODER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CHANNEL_ENCODER {
    pub createContext: *mut ::core::ffi::c_void,
    pub createEncoderCallback: WS_CREATE_ENCODER_CALLBACK,
    pub encoderGetContentTypeCallback: WS_ENCODER_GET_CONTENT_TYPE_CALLBACK,
    pub encoderStartCallback: WS_ENCODER_START_CALLBACK,
    pub encoderEncodeCallback: WS_ENCODER_ENCODE_CALLBACK,
    pub encoderEndCallback: WS_ENCODER_END_CALLBACK,
    pub freeEncoderCallback: WS_FREE_ENCODER_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CHANNEL_ENCODER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CHANNEL_ENCODER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    pub channelProperty: WS_CHANNEL_PROPERTY,
}
impl ::core::marker::Copy for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_CHANNEL_PROPERTY_MAX_BUFFERED_MESSAGE_SIZE: i32 = 0i32;
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_MESSAGE_SIZE: i32 = 1i32;
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_START_SIZE: i32 = 2i32;
pub const WS_CHANNEL_PROPERTY_MAX_STREAMED_FLUSH_SIZE: i32 = 3i32;
pub const WS_CHANNEL_PROPERTY_ENCODING: i32 = 4i32;
pub const WS_CHANNEL_PROPERTY_ENVELOPE_VERSION: i32 = 5i32;
pub const WS_CHANNEL_PROPERTY_ADDRESSING_VERSION: i32 = 6i32;
pub const WS_CHANNEL_PROPERTY_MAX_SESSION_DICTIONARY_SIZE: i32 = 7i32;
pub const WS_CHANNEL_PROPERTY_STATE: i32 = 8i32;
pub const WS_CHANNEL_PROPERTY_ASYNC_CALLBACK_MODEL: i32 = 9i32;
pub const WS_CHANNEL_PROPERTY_IP_VERSION: i32 = 10i32;
pub const WS_CHANNEL_PROPERTY_RESOLVE_TIMEOUT: i32 = 11i32;
pub const WS_CHANNEL_PROPERTY_CONNECT_TIMEOUT: i32 = 12i32;
pub const WS_CHANNEL_PROPERTY_SEND_TIMEOUT: i32 = 13i32;
pub const WS_CHANNEL_PROPERTY_RECEIVE_RESPONSE_TIMEOUT: i32 = 14i32;
pub const WS_CHANNEL_PROPERTY_RECEIVE_TIMEOUT: i32 = 15i32;
pub const WS_CHANNEL_PROPERTY_CLOSE_TIMEOUT: i32 = 16i32;
pub const WS_CHANNEL_PROPERTY_ENABLE_TIMEOUTS: i32 = 17i32;
pub const WS_CHANNEL_PROPERTY_TRANSFER_MODE: i32 = 18i32;
pub const WS_CHANNEL_PROPERTY_MULTICAST_INTERFACE: i32 = 19i32;
pub const WS_CHANNEL_PROPERTY_MULTICAST_HOPS: i32 = 20i32;
pub const WS_CHANNEL_PROPERTY_REMOTE_ADDRESS: i32 = 21i32;
pub const WS_CHANNEL_PROPERTY_REMOTE_IP_ADDRESS: i32 = 22i32;
pub const WS_CHANNEL_PROPERTY_HTTP_CONNECTION_ID: i32 = 23i32;
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_CALLBACKS: i32 = 24i32;
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_PARAMETERS: i32 = 25i32;
pub const WS_CHANNEL_PROPERTY_CUSTOM_CHANNEL_INSTANCE: i32 = 26i32;
pub const WS_CHANNEL_PROPERTY_TRANSPORT_URL: i32 = 27i32;
pub const WS_CHANNEL_PROPERTY_NO_DELAY: i32 = 28i32;
pub const WS_CHANNEL_PROPERTY_SEND_KEEP_ALIVES: i32 = 29i32;
pub const WS_CHANNEL_PROPERTY_KEEP_ALIVE_TIME: i32 = 30i32;
pub const WS_CHANNEL_PROPERTY_KEEP_ALIVE_INTERVAL: i32 = 31i32;
pub const WS_CHANNEL_PROPERTY_MAX_HTTP_SERVER_CONNECTIONS: i32 = 32i32;
pub const WS_CHANNEL_PROPERTY_IS_SESSION_SHUT_DOWN: i32 = 33i32;
pub const WS_CHANNEL_PROPERTY_CHANNEL_TYPE: i32 = 34i32;
pub const WS_CHANNEL_PROPERTY_TRIM_BUFFERED_MESSAGE_SIZE: i32 = 35i32;
pub const WS_CHANNEL_PROPERTY_ENCODER: i32 = 36i32;
pub const WS_CHANNEL_PROPERTY_DECODER: i32 = 37i32;
pub const WS_CHANNEL_PROPERTY_PROTECTION_LEVEL: i32 = 38i32;
pub const WS_CHANNEL_PROPERTY_COOKIE_MODE: i32 = 39i32;
pub const WS_CHANNEL_PROPERTY_HTTP_PROXY_SETTING_MODE: i32 = 40i32;
pub const WS_CHANNEL_PROPERTY_CUSTOM_HTTP_PROXY: i32 = 41i32;
pub const WS_CHANNEL_PROPERTY_HTTP_MESSAGE_MAPPING: i32 = 42i32;
pub const WS_CHANNEL_PROPERTY_ENABLE_HTTP_REDIRECT: i32 = 43i32;
pub const WS_CHANNEL_PROPERTY_HTTP_REDIRECT_CALLBACK_CONTEXT: i32 = 44i32;
pub const WS_CHANNEL_PROPERTY_FAULTS_AS_ERRORS: i32 = 45i32;
pub const WS_CHANNEL_PROPERTY_ALLOW_UNSECURED_FAULTS: i32 = 46i32;
pub const WS_CHANNEL_PROPERTY_HTTP_SERVER_SPN: i32 = 47i32;
pub const WS_CHANNEL_PROPERTY_HTTP_PROXY_SPN: i32 = 48i32;
pub const WS_CHANNEL_PROPERTY_MAX_HTTP_REQUEST_HEADERS_BUFFER_SIZE: i32 = 49i32;
pub const WS_CHANNEL_STATE_CREATED: i32 = 0i32;
pub const WS_CHANNEL_STATE_OPENING: i32 = 1i32;
pub const WS_CHANNEL_STATE_ACCEPTING: i32 = 2i32;
pub const WS_CHANNEL_STATE_OPEN: i32 = 3i32;
pub const WS_CHANNEL_STATE_FAULTED: i32 = 4i32;
pub const WS_CHANNEL_STATE_CLOSING: i32 = 5i32;
pub const WS_CHANNEL_STATE_CLOSED: i32 = 6i32;
pub const WS_CHANNEL_TYPE_INPUT: i32 = 1i32;
pub const WS_CHANNEL_TYPE_OUTPUT: i32 = 2i32;
pub const WS_CHANNEL_TYPE_SESSION: i32 = 4i32;
pub const WS_CHANNEL_TYPE_INPUT_SESSION: i32 = 5i32;
pub const WS_CHANNEL_TYPE_OUTPUT_SESSION: i32 = 6i32;
pub const WS_CHANNEL_TYPE_DUPLEX: i32 = 3i32;
pub const WS_CHANNEL_TYPE_DUPLEX_SESSION: i32 = 7i32;
pub const WS_CHANNEL_TYPE_REQUEST: i32 = 8i32;
pub const WS_CHANNEL_TYPE_REPLY: i32 = 16i32;
pub const WS_CHARSET_AUTO: i32 = 0i32;
pub const WS_CHARSET_UTF8: i32 = 1i32;
pub const WS_CHARSET_UTF16LE: i32 = 2i32;
pub const WS_CHARSET_UTF16BE: i32 = 3i32;
#[repr(C)]
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
pub type WS_CLOSE_CHANNEL_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_CLOSE_LISTENER_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
pub const WS_MANUAL_COOKIE_MODE: i32 = 1i32;
pub const WS_AUTO_COOKIE_MODE: i32 = 2i32;
pub type WS_CREATE_CHANNEL_CALLBACK = unsafe extern "system" fn(channeltype: WS_CHANNEL_TYPE, channelparameters: *const ::core::ffi::c_void, channelparameterssize: u32, channelinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_CREATE_CHANNEL_FOR_LISTENER_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, channelparameters: *const ::core::ffi::c_void, channelparameterssize: u32, channelinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_CREATE_DECODER_CALLBACK = unsafe extern "system" fn(createcontext: *const ::core::ffi::c_void, readcallback: WS_READ_CALLBACK, readcontext: *const ::core::ffi::c_void, decodercontext: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_CREATE_ENCODER_CALLBACK = unsafe extern "system" fn(createcontext: *const ::core::ffi::c_void, writecallback: WS_WRITE_CALLBACK, writecontext: *const ::core::ffi::c_void, encodercontext: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_CREATE_LISTENER_CALLBACK = unsafe extern "system" fn(channeltype: WS_CHANNEL_TYPE, listenerparameters: *const ::core::ffi::c_void, listenerparameterssize: u32, listenerinstance: *mut *mut ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CUSTOM_CHANNEL_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_CUSTOM_HTTP_PROXY {
    pub servers: WS_STRING,
    pub bypass: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CUSTOM_HTTP_PROXY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CUSTOM_HTTP_PROXY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_CUSTOM_LISTENER_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_CUSTOM_LISTENER_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_DATETIME_FORMAT_UTC: i32 = 0i32;
pub const WS_DATETIME_FORMAT_LOCAL: i32 = 1i32;
pub const WS_DATETIME_FORMAT_NONE: i32 = 2i32;
#[repr(C)]
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
pub type WS_DECODER_DECODE_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, maxlength: u32, length: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_DECODER_END_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WS_DECODER_GET_CONTENT_TYPE_CALLBACK = unsafe extern "system" fn(decodercontext: *const ::core::ffi::c_void, contenttype: *const WS_STRING, contentencoding: *const WS_STRING, newcontenttype: *mut WS_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_DECODER_START_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
pub struct WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
}
impl ::core::marker::Copy for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    pub subStringCount: u32,
    pub subStrings: *mut *mut WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_DNS_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub dns: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_DNS_ENDPOINT_IDENTITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_DNS_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
pub type WS_DURATION_COMPARISON_CALLBACK = unsafe extern "system" fn(duration1: *const WS_DURATION, duration2: *const WS_DURATION, result: *mut i32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
pub type WS_DYNAMIC_STRING_CALLBACK = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, string: *const WS_XML_STRING, found: *mut super::super::Foundation::BOOL, id: *mut u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
pub type WS_ENCODER_ENCODE_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, buffers: *const WS_BYTES, count: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_ENCODER_END_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WS_ENCODER_GET_CONTENT_TYPE_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, contenttype: *const WS_STRING, newcontenttype: *mut WS_STRING, contentencoding: *mut WS_STRING, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_ENCODER_START_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_ENCODING_XML_BINARY_1: i32 = 0i32;
pub const WS_ENCODING_XML_BINARY_SESSION_1: i32 = 1i32;
pub const WS_ENCODING_XML_MTOM_UTF8: i32 = 2i32;
pub const WS_ENCODING_XML_MTOM_UTF16BE: i32 = 3i32;
pub const WS_ENCODING_XML_MTOM_UTF16LE: i32 = 4i32;
pub const WS_ENCODING_XML_UTF8: i32 = 5i32;
pub const WS_ENCODING_XML_UTF16BE: i32 = 6i32;
pub const WS_ENCODING_XML_UTF16LE: i32 = 7i32;
pub const WS_ENCODING_RAW: i32 = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_ENDPOINT_ADDRESS {
    pub url: WS_STRING,
    pub headers: *mut WS_XML_BUFFER,
    pub extensions: *mut WS_XML_BUFFER,
    pub identity: *mut WS_ENDPOINT_IDENTITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_ENDPOINT_ADDRESS_DESCRIPTION {
    pub addressingVersion: WS_ADDRESSING_VERSION,
}
impl ::core::marker::Copy for WS_ENDPOINT_ADDRESS_DESCRIPTION {}
impl ::core::clone::Clone for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_ENDPOINT_ADDRESS_EXTENSION_METADATA_ADDRESS: i32 = 1i32;
#[repr(C)]
pub struct WS_ENDPOINT_IDENTITY {
    pub identityType: WS_ENDPOINT_IDENTITY_TYPE,
}
impl ::core::marker::Copy for WS_ENDPOINT_IDENTITY {}
impl ::core::clone::Clone for WS_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_DNS_ENDPOINT_IDENTITY_TYPE: i32 = 1i32;
pub const WS_UPN_ENDPOINT_IDENTITY_TYPE: i32 = 2i32;
pub const WS_SPN_ENDPOINT_IDENTITY_TYPE: i32 = 3i32;
pub const WS_RSA_ENDPOINT_IDENTITY_TYPE: i32 = 4i32;
pub const WS_CERT_ENDPOINT_IDENTITY_TYPE: i32 = 5i32;
pub const WS_UNKNOWN_ENDPOINT_IDENTITY_TYPE: i32 = 6i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_ENVELOPE_VERSION_SOAP_1_1: i32 = 1i32;
pub const WS_ENVELOPE_VERSION_SOAP_1_2: i32 = 2i32;
pub const WS_ENVELOPE_VERSION_NONE: i32 = 3i32;
#[repr(C)]
pub struct WS_ERROR(pub u8);
#[repr(C)]
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
pub const WS_ERROR_PROPERTY_STRING_COUNT: i32 = 0i32;
pub const WS_ERROR_PROPERTY_ORIGINAL_ERROR_CODE: i32 = 1i32;
pub const WS_ERROR_PROPERTY_LANGID: i32 = 2i32;
pub const WS_EXCEPTION_CODE_USAGE_FAILURE: i32 = -1069744128i32;
pub const WS_EXCEPTION_CODE_INTERNAL_FAILURE: i32 = -1069744127i32;
pub const WS_EXTENDED_PROTECTION_POLICY_NEVER: i32 = 1i32;
pub const WS_EXTENDED_PROTECTION_POLICY_WHEN_SUPPORTED: i32 = 2i32;
pub const WS_EXTENDED_PROTECTION_POLICY_ALWAYS: i32 = 3i32;
pub const WS_EXTENDED_PROTECTION_SCENARIO_BOUND_SERVER: i32 = 1i32;
pub const WS_EXTENDED_PROTECTION_SCENARIO_TERMINATED_SSL: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_FAULT_DESCRIPTION {
    pub envelopeVersion: WS_ENVELOPE_VERSION,
}
impl ::core::marker::Copy for WS_FAULT_DESCRIPTION {}
impl ::core::clone::Clone for WS_FAULT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_MINIMAL_FAULT_DISCLOSURE: i32 = 0i32;
pub const WS_FULL_FAULT_DISCLOSURE: i32 = 1i32;
pub const WS_FAULT_ERROR_PROPERTY_FAULT: i32 = 0i32;
pub const WS_FAULT_ERROR_PROPERTY_ACTION: i32 = 1i32;
pub const WS_FAULT_ERROR_PROPERTY_HEADER: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_FAULT_REASON {
    pub text: WS_STRING,
    pub lang: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_FAULT_REASON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_TYPE_ATTRIBUTE_FIELD_MAPPING: i32 = 0i32;
pub const WS_ATTRIBUTE_FIELD_MAPPING: i32 = 1i32;
pub const WS_ELEMENT_FIELD_MAPPING: i32 = 2i32;
pub const WS_REPEATING_ELEMENT_FIELD_MAPPING: i32 = 3i32;
pub const WS_TEXT_FIELD_MAPPING: i32 = 4i32;
pub const WS_NO_FIELD_MAPPING: i32 = 5i32;
pub const WS_XML_ATTRIBUTE_FIELD_MAPPING: i32 = 6i32;
pub const WS_ELEMENT_CHOICE_FIELD_MAPPING: i32 = 7i32;
pub const WS_REPEATING_ELEMENT_CHOICE_FIELD_MAPPING: i32 = 8i32;
pub const WS_ANY_ELEMENT_FIELD_MAPPING: i32 = 9i32;
pub const WS_REPEATING_ANY_ELEMENT_FIELD_MAPPING: i32 = 10i32;
pub const WS_ANY_CONTENT_FIELD_MAPPING: i32 = 11i32;
pub const WS_ANY_ATTRIBUTES_FIELD_MAPPING: i32 = 12i32;
pub const WS_FIELD_NILLABLE: i32 = 4i32;
pub const WS_FIELD_NILLABLE_ITEM: i32 = 8i32;
pub const WS_FIELD_OPTIONAL: i32 = 2i32;
pub const WS_FIELD_OTHER_NAMESPACE: i32 = 16i32;
pub const WS_FIELD_POINTER: i32 = 1i32;
#[repr(C)]
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
pub type WS_FREE_CHANNEL_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void);
pub type WS_FREE_DECODER_CALLBACK = unsafe extern "system" fn(decodercontext: *const ::core::ffi::c_void);
pub type WS_FREE_ENCODER_CALLBACK = unsafe extern "system" fn(encodercontext: *const ::core::ffi::c_void);
pub type WS_FREE_LISTENER_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type WS_GET_CERT_CALLBACK = unsafe extern "system" fn(getcertcallbackstate: *const ::core::ffi::c_void, targetaddress: *const WS_ENDPOINT_ADDRESS, viauri: *const WS_STRING, cert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_GET_CHANNEL_PROPERTY_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, id: WS_CHANNEL_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_GET_LISTENER_PROPERTY_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, id: WS_LISTENER_PROPERTY_ID, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub struct WS_GUID_DESCRIPTION {
    pub value: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WS_GUID_DESCRIPTION {}
impl ::core::clone::Clone for WS_GUID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_ACTION_HEADER: i32 = 1i32;
pub const WS_TO_HEADER: i32 = 2i32;
pub const WS_MESSAGE_ID_HEADER: i32 = 3i32;
pub const WS_RELATES_TO_HEADER: i32 = 4i32;
pub const WS_FROM_HEADER: i32 = 5i32;
pub const WS_REPLY_TO_HEADER: i32 = 6i32;
pub const WS_FAULT_TO_HEADER: i32 = 7i32;
#[repr(C)]
pub struct WS_HEAP(pub u8);
#[repr(C)]
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
#[repr(C)]
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
pub const WS_HEAP_PROPERTY_MAX_SIZE: i32 = 0i32;
pub const WS_HEAP_PROPERTY_TRIM_SIZE: i32 = 1i32;
pub const WS_HEAP_PROPERTY_REQUESTED_SIZE: i32 = 2i32;
pub const WS_HEAP_PROPERTY_ACTUAL_SIZE: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HOST_NAMES {
    pub hostNames: *mut WS_STRING,
    pub hostNameCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HOST_NAMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HOST_NAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTPS_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTPS_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTPS_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_HTTP_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_HTTP_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
pub const WS_HTTP_HEADER_AUTH_SCHEME_BASIC: i32 = 2i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_DIGEST: i32 = 4i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_NEGOTIATE: i32 = 16i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_NONE: i32 = 1i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_NTLM: i32 = 8i32;
pub const WS_HTTP_HEADER_AUTH_SCHEME_PASSPORT: i32 = 32i32;
#[repr(C)]
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
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_HTTP_HEADER_AUTH_TARGET_SERVICE: i32 = 1i32;
pub const WS_HTTP_HEADER_AUTH_TARGET_PROXY: i32 = 2i32;
#[repr(C)]
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
pub const WS_HTTP_HEADER_MAPPING_COMMA_SEPARATOR: i32 = 1i32;
pub const WS_HTTP_HEADER_MAPPING_QUOTED_VALUE: i32 = 4i32;
pub const WS_HTTP_HEADER_MAPPING_SEMICOLON_SEPARATOR: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
pub struct WS_HTTP_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_HTTP_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_HTTP_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_HTTP_PROXY_SETTING_MODE_AUTO: i32 = 1i32;
pub const WS_HTTP_PROXY_SETTING_MODE_NONE: i32 = 2i32;
pub const WS_HTTP_PROXY_SETTING_MODE_CUSTOM: i32 = 3i32;
#[cfg(feature = "Win32_Foundation")]
pub type WS_HTTP_REDIRECT_CALLBACK = unsafe extern "system" fn(state: *const ::core::ffi::c_void, originalurl: *const WS_STRING, newurl: *const WS_STRING) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    pub callback: WS_HTTP_REDIRECT_CALLBACK,
    pub state: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_HTTP_REQUEST_MAPPING_VERB: i32 = 2i32;
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_CODE: i32 = 1i32;
pub const WS_HTTP_RESPONSE_MAPPING_STATUS_TEXT: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sslTransportSecurityBinding: WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_HTTP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_HTTP_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_HTTP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_INT8_DESCRIPTION {
    pub minValue: super::super::Foundation::CHAR,
    pub maxValue: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_INT8_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_INT8_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_IP_VERSION_4: i32 = 1i32;
pub const WS_IP_VERSION_6: i32 = 2i32;
pub const WS_IP_VERSION_AUTO: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
pub type WS_IS_DEFAULT_VALUE_CALLBACK = unsafe extern "system" fn(descriptiondata: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, defaultvalue: *const ::core::ffi::c_void, valuesize: u32, isdefault: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_LISTENER(pub u8);
#[repr(C)]
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
#[repr(C)]
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
pub const WS_LISTENER_PROPERTY_LISTEN_BACKLOG: i32 = 0i32;
pub const WS_LISTENER_PROPERTY_IP_VERSION: i32 = 1i32;
pub const WS_LISTENER_PROPERTY_STATE: i32 = 2i32;
pub const WS_LISTENER_PROPERTY_ASYNC_CALLBACK_MODEL: i32 = 3i32;
pub const WS_LISTENER_PROPERTY_CHANNEL_TYPE: i32 = 4i32;
pub const WS_LISTENER_PROPERTY_CHANNEL_BINDING: i32 = 5i32;
pub const WS_LISTENER_PROPERTY_CONNECT_TIMEOUT: i32 = 6i32;
pub const WS_LISTENER_PROPERTY_IS_MULTICAST: i32 = 7i32;
pub const WS_LISTENER_PROPERTY_MULTICAST_INTERFACES: i32 = 8i32;
pub const WS_LISTENER_PROPERTY_MULTICAST_LOOPBACK: i32 = 9i32;
pub const WS_LISTENER_PROPERTY_CLOSE_TIMEOUT: i32 = 10i32;
pub const WS_LISTENER_PROPERTY_TO_HEADER_MATCHING_OPTIONS: i32 = 11i32;
pub const WS_LISTENER_PROPERTY_TRANSPORT_URL_MATCHING_OPTIONS: i32 = 12i32;
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_CALLBACKS: i32 = 13i32;
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_PARAMETERS: i32 = 14i32;
pub const WS_LISTENER_PROPERTY_CUSTOM_LISTENER_INSTANCE: i32 = 15i32;
pub const WS_LISTENER_PROPERTY_DISALLOWED_USER_AGENT: i32 = 16i32;
pub const WS_LISTENER_STATE_CREATED: i32 = 0i32;
pub const WS_LISTENER_STATE_OPENING: i32 = 1i32;
pub const WS_LISTENER_STATE_OPEN: i32 = 2i32;
pub const WS_LISTENER_STATE_FAULTED: i32 = 3i32;
pub const WS_LISTENER_STATE_CLOSING: i32 = 4i32;
pub const WS_LISTENER_STATE_CLOSED: i32 = 5i32;
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
pub struct WS_MESSAGE(pub u8);
#[repr(C)]
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
pub type WS_MESSAGE_DONE_CALLBACK = unsafe extern "system" fn(donecallbackstate: *const ::core::ffi::c_void);
pub const WS_BLANK_MESSAGE: i32 = 0i32;
pub const WS_DUPLICATE_MESSAGE: i32 = 1i32;
pub const WS_REQUEST_MESSAGE: i32 = 2i32;
pub const WS_REPLY_MESSAGE: i32 = 3i32;
pub const WS_FAULT_MESSAGE: i32 = 4i32;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_MESSAGE_PROPERTY_STATE: i32 = 0i32;
pub const WS_MESSAGE_PROPERTY_HEAP: i32 = 1i32;
pub const WS_MESSAGE_PROPERTY_ENVELOPE_VERSION: i32 = 2i32;
pub const WS_MESSAGE_PROPERTY_ADDRESSING_VERSION: i32 = 3i32;
pub const WS_MESSAGE_PROPERTY_HEADER_BUFFER: i32 = 4i32;
pub const WS_MESSAGE_PROPERTY_HEADER_POSITION: i32 = 5i32;
pub const WS_MESSAGE_PROPERTY_BODY_READER: i32 = 6i32;
pub const WS_MESSAGE_PROPERTY_BODY_WRITER: i32 = 7i32;
pub const WS_MESSAGE_PROPERTY_IS_ADDRESSED: i32 = 8i32;
pub const WS_MESSAGE_PROPERTY_HEAP_PROPERTIES: i32 = 9i32;
pub const WS_MESSAGE_PROPERTY_XML_READER_PROPERTIES: i32 = 10i32;
pub const WS_MESSAGE_PROPERTY_XML_WRITER_PROPERTIES: i32 = 11i32;
pub const WS_MESSAGE_PROPERTY_IS_FAULT: i32 = 12i32;
pub const WS_MESSAGE_PROPERTY_MAX_PROCESSED_HEADERS: i32 = 13i32;
pub const WS_MESSAGE_PROPERTY_USERNAME: i32 = 14i32;
pub const WS_MESSAGE_PROPERTY_ENCODED_CERT: i32 = 15i32;
pub const WS_MESSAGE_PROPERTY_TRANSPORT_SECURITY_WINDOWS_TOKEN: i32 = 16i32;
pub const WS_MESSAGE_PROPERTY_HTTP_HEADER_AUTH_WINDOWS_TOKEN: i32 = 17i32;
pub const WS_MESSAGE_PROPERTY_MESSAGE_SECURITY_WINDOWS_TOKEN: i32 = 18i32;
pub const WS_MESSAGE_PROPERTY_SAML_ASSERTION: i32 = 19i32;
pub const WS_MESSAGE_PROPERTY_SECURITY_CONTEXT: i32 = 20i32;
pub const WS_MESSAGE_PROPERTY_PROTECTION_LEVEL: i32 = 21i32;
pub const WS_SUPPORTING_MESSAGE_SECURITY_USAGE: i32 = 1i32;
pub const WS_MESSAGE_STATE_EMPTY: i32 = 1i32;
pub const WS_MESSAGE_STATE_INITIALIZED: i32 = 2i32;
pub const WS_MESSAGE_STATE_READING: i32 = 3i32;
pub const WS_MESSAGE_STATE_WRITING: i32 = 4i32;
pub const WS_MESSAGE_STATE_DONE: i32 = 5i32;
#[repr(C)]
pub struct WS_METADATA(pub u8);
#[repr(C)]
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
#[repr(C)]
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
pub const WS_METADATA_EXCHANGE_TYPE_NONE: i32 = 0i32;
pub const WS_METADATA_EXCHANGE_TYPE_MEX: i32 = 1i32;
pub const WS_METADATA_EXCHANGE_TYPE_HTTP_GET: i32 = 2i32;
#[repr(C)]
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
pub const WS_METADATA_PROPERTY_STATE: i32 = 1i32;
pub const WS_METADATA_PROPERTY_HEAP_PROPERTIES: i32 = 2i32;
pub const WS_METADATA_PROPERTY_POLICY_PROPERTIES: i32 = 3i32;
pub const WS_METADATA_PROPERTY_HEAP_REQUESTED_SIZE: i32 = 4i32;
pub const WS_METADATA_PROPERTY_MAX_DOCUMENTS: i32 = 5i32;
pub const WS_METADATA_PROPERTY_HOST_NAMES: i32 = 6i32;
pub const WS_METADATA_PROPERTY_VERIFY_HOST_NAMES: i32 = 7i32;
pub const WS_METADATA_STATE_CREATED: i32 = 1i32;
pub const WS_METADATA_STATE_RESOLVED: i32 = 2i32;
pub const WS_METADATA_STATE_FAULTED: i32 = 3i32;
pub const WS_MOVE_TO_ROOT_ELEMENT: i32 = 0i32;
pub const WS_MOVE_TO_NEXT_ELEMENT: i32 = 1i32;
pub const WS_MOVE_TO_PREVIOUS_ELEMENT: i32 = 2i32;
pub const WS_MOVE_TO_CHILD_ELEMENT: i32 = 3i32;
pub const WS_MOVE_TO_END_ELEMENT: i32 = 4i32;
pub const WS_MOVE_TO_PARENT_ELEMENT: i32 = 5i32;
pub const WS_MOVE_TO_NEXT_NODE: i32 = 6i32;
pub const WS_MOVE_TO_PREVIOUS_NODE: i32 = 7i32;
pub const WS_MOVE_TO_FIRST_NODE: i32 = 8i32;
pub const WS_MOVE_TO_BOF: i32 = 9i32;
pub const WS_MOVE_TO_EOF: i32 = 10i32;
pub const WS_MOVE_TO_CHILD_NODE: i32 = 11i32;
pub const WS_MUST_UNDERSTAND_HEADER_ATTRIBUTE: i32 = 1i32;
#[repr(C)]
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
#[repr(C)]
pub struct WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    pub keyHandle: WS_SECURITY_KEY_HANDLE,
    pub asymmetricKey: usize,
}
impl ::core::marker::Copy for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_NETPIPE_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_NETPIPE_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_NETPIPE_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_NETTCP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_NETTCP_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_NETTCP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub type WS_OPEN_CHANNEL_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, endpointaddress: *const WS_ENDPOINT_ADDRESS, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type WS_OPEN_LISTENER_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, url: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_OPERATION_CANCEL_CALLBACK = unsafe extern "system" fn(reason: WS_SERVICE_CANCEL_REASON, state: *const ::core::ffi::c_void);
#[repr(C)]
pub struct WS_OPERATION_CONTEXT(pub u8);
pub const WS_OPERATION_CONTEXT_PROPERTY_CHANNEL: i32 = 0i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_CONTRACT_DESCRIPTION: i32 = 1i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_HOST_USER_STATE: i32 = 2i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_CHANNEL_USER_STATE: i32 = 3i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_INPUT_MESSAGE: i32 = 4i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_OUTPUT_MESSAGE: i32 = 5i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_HEAP: i32 = 6i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_LISTENER: i32 = 7i32;
pub const WS_OPERATION_CONTEXT_PROPERTY_ENDPOINT_ADDRESS: i32 = 8i32;
#[repr(C)]
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
pub type WS_OPERATION_FREE_STATE_CALLBACK = unsafe extern "system" fn(state: *const ::core::ffi::c_void);
pub const WS_NON_RPC_LITERAL_OPERATION: i32 = 0i32;
pub const WS_RPC_LITERAL_OPERATION: i32 = 1i32;
#[repr(C)]
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
pub const WS_PARAMETER_TYPE_NORMAL: i32 = 0i32;
pub const WS_PARAMETER_TYPE_ARRAY: i32 = 1i32;
pub const WS_PARAMETER_TYPE_ARRAY_COUNT: i32 = 2i32;
pub const WS_PARAMETER_TYPE_MESSAGES: i32 = 3i32;
#[repr(C)]
pub struct WS_POLICY(pub u8);
#[repr(C)]
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
#[repr(C)]
pub struct WS_POLICY_EXTENSION {
    pub r#type: WS_POLICY_EXTENSION_TYPE,
}
impl ::core::marker::Copy for WS_POLICY_EXTENSION {}
impl ::core::clone::Clone for WS_POLICY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_ENDPOINT_POLICY_EXTENSION_TYPE: i32 = 1i32;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_POLICY_PROPERTY_STATE: i32 = 1i32;
pub const WS_POLICY_PROPERTY_MAX_ALTERNATIVES: i32 = 2i32;
pub const WS_POLICY_PROPERTY_MAX_DEPTH: i32 = 3i32;
pub const WS_POLICY_PROPERTY_MAX_EXTENSIONS: i32 = 4i32;
pub const WS_POLICY_STATE_CREATED: i32 = 1i32;
pub const WS_POLICY_STATE_FAULTED: i32 = 2i32;
pub const WS_PROTECTION_LEVEL_NONE: i32 = 1i32;
pub const WS_PROTECTION_LEVEL_SIGN: i32 = 2i32;
pub const WS_PROTECTION_LEVEL_SIGN_AND_ENCRYPT: i32 = 3i32;
pub type WS_PROXY_MESSAGE_CALLBACK = unsafe extern "system" fn(message: *const WS_MESSAGE, heap: *const WS_HEAP, state: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_PROXY_PROPERTY_CALL_TIMEOUT: i32 = 0i32;
pub const WS_PROXY_PROPERTY_MESSAGE_PROPERTIES: i32 = 1i32;
pub const WS_PROXY_PROPERTY_MAX_CALL_POOL_SIZE: i32 = 2i32;
pub const WS_PROXY_PROPERTY_STATE: i32 = 3i32;
pub const WS_PROXY_PROPERTY_MAX_PENDING_CALLS: i32 = 4i32;
pub const WS_PROXY_PROPERTY_MAX_CLOSE_TIMEOUT: i32 = 5i32;
pub const WS_PROXY_FAULT_LANG_ID: i32 = 6i32;
pub type WS_PULL_BYTES_CALLBACK = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, bytes: *mut ::core::ffi::c_void, maxsize: u32, actualsize: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_PUSH_BYTES_CALLBACK = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, writecallback: WS_WRITE_CALLBACK, writecallbackstate: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
pub type WS_READ_CALLBACK = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, bytes: *mut ::core::ffi::c_void, maxsize: u32, actualsize: *mut u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_READ_MESSAGE_END_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_READ_MESSAGE_START_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_READ_REQUIRED_VALUE: i32 = 1i32;
pub const WS_READ_REQUIRED_POINTER: i32 = 2i32;
pub const WS_READ_OPTIONAL_POINTER: i32 = 3i32;
pub const WS_READ_NILLABLE_POINTER: i32 = 4i32;
pub const WS_READ_NILLABLE_VALUE: i32 = 5i32;
pub type WS_READ_TYPE_CALLBACK = unsafe extern "system" fn(reader: *const WS_XML_READER, typemapping: WS_TYPE_MAPPING, descriptiondata: *const ::core::ffi::c_void, heap: *const WS_HEAP, value: *mut ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_RECEIVE_REQUIRED_MESSAGE: i32 = 1i32;
pub const WS_RECEIVE_OPTIONAL_MESSAGE: i32 = 2i32;
pub const WS_RELAY_HEADER_ATTRIBUTE: i32 = 2i32;
pub const WS_REPEATING_HEADER: i32 = 1i32;
pub const WS_SINGLETON_HEADER: i32 = 2i32;
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_ISSUE: i32 = 1i32;
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_NEW_CONTEXT: i32 = 2i32;
pub const WS_REQUEST_SECURITY_TOKEN_ACTION_RENEW_CONTEXT: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    pub requestSecurityTokenProperty: WS_REQUEST_SECURITY_TOKEN_PROPERTY,
}
impl ::core::marker::Copy for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_APPLIES_TO: i32 = 1i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_TRUST_VERSION: i32 = 2i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_SECURE_CONVERSATION_VERSION: i32 = 3i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_TYPE: i32 = 4i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_REQUEST_ACTION: i32 = 5i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_EXISTING_TOKEN: i32 = 6i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_TYPE: i32 = 7i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_SIZE: i32 = 8i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_ISSUED_TOKEN_KEY_ENTROPY: i32 = 9i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_LOCAL_REQUEST_PARAMETERS: i32 = 10i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_SERVICE_REQUEST_PARAMETERS: i32 = 11i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_MESSAGE_PROPERTIES: i32 = 12i32;
pub const WS_REQUEST_SECURITY_TOKEN_PROPERTY_BEARER_KEY_TYPE_VERSION: i32 = 13i32;
pub type WS_RESET_CHANNEL_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_RESET_LISTENER_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
pub struct WS_SAML_AUTHENTICATOR {
    pub authenticatorType: WS_SAML_AUTHENTICATOR_TYPE,
}
impl ::core::marker::Copy for WS_SAML_AUTHENTICATOR {}
impl ::core::clone::Clone for WS_SAML_AUTHENTICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_CERT_SIGNED_SAML_AUTHENTICATOR_TYPE: i32 = 1i32;
#[repr(C)]
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
pub const WS_SECURE_CONVERSATION_VERSION_FEBRUARY_2005: i32 = 1i32;
pub const WS_SECURE_CONVERSATION_VERSION_1_3: i32 = 2i32;
pub const WS_SECURE_PROTOCOL_SSL2: i32 = 1i32;
pub const WS_SECURE_PROTOCOL_SSL3: i32 = 2i32;
pub const WS_SECURE_PROTOCOL_TLS1_0: i32 = 4i32;
pub const WS_SECURE_PROTOCOL_TLS1_1: i32 = 8i32;
pub const WS_SECURE_PROTOCOL_TLS1_2: i32 = 16i32;
pub const WS_SECURITY_ALGORITHM_DEFAULT: i32 = 0i32;
pub const WS_SECURITY_ALGORITHM_CANONICALIZATION_EXCLUSIVE: i32 = 1i32;
pub const WS_SECURITY_ALGORITHM_CANONICALIZATION_EXCLUSIVE_WITH_COMMENTS: i32 = 2i32;
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA1: i32 = 3i32;
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_256: i32 = 4i32;
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_384: i32 = 5i32;
pub const WS_SECURITY_ALGORITHM_DIGEST_SHA_512: i32 = 6i32;
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA1: i32 = 7i32;
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_256: i32 = 8i32;
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_384: i32 = 9i32;
pub const WS_SECURITY_ALGORITHM_SYMMETRIC_SIGNATURE_HMAC_SHA_512: i32 = 10i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA1: i32 = 11i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_DSA_SHA1: i32 = 12i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_256: i32 = 13i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_384: i32 = 14i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_SIGNATURE_RSA_SHA_512: i32 = 15i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_KEYWRAP_RSA_1_5: i32 = 16i32;
pub const WS_SECURITY_ALGORITHM_ASYMMETRIC_KEYWRAP_RSA_OAEP: i32 = 17i32;
pub const WS_SECURITY_ALGORITHM_KEY_DERIVATION_P_SHA1: i32 = 18i32;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256: i32 = 1i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192: i32 = 2i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128: i32 = 3i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_RSA15: i32 = 4i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_RSA15: i32 = 5i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_RSA15: i32 = 6i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_SHA256: i32 = 7i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_SHA256: i32 = 8i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_SHA256: i32 = 9i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC256_SHA256_RSA15: i32 = 10i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC192_SHA256_RSA15: i32 = 11i32;
pub const WS_SECURITY_ALGORITHM_SUITE_NAME_BASIC128_SHA256_RSA15: i32 = 12i32;
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ORIGINAL_SPECIFICATION: i32 = 1i32;
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ORIGINAL_SCHEMA: i32 = 2i32;
pub const WS_SECURITY_BEARER_KEY_TYPE_VERSION_1_3_ERRATA_01: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 1i32;
pub const WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 2i32;
pub const WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 3i32;
pub const WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 4i32;
pub const WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 5i32;
pub const WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 6i32;
pub const WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 7i32;
pub const WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT_TYPE: i32 = 8i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    pub securityBindingProperty: WS_SECURITY_BINDING_PROPERTY,
}
impl ::core::marker::Copy for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_SECURITY_BINDING_PROPERTY_REQUIRE_SSL_CLIENT_CERT: i32 = 1i32;
pub const WS_SECURITY_BINDING_PROPERTY_WINDOWS_INTEGRATED_AUTH_PACKAGE: i32 = 2i32;
pub const WS_SECURITY_BINDING_PROPERTY_REQUIRE_SERVER_AUTH: i32 = 3i32;
pub const WS_SECURITY_BINDING_PROPERTY_ALLOW_ANONYMOUS_CLIENTS: i32 = 4i32;
pub const WS_SECURITY_BINDING_PROPERTY_ALLOWED_IMPERSONATION_LEVEL: i32 = 5i32;
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_SCHEME: i32 = 6i32;
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_TARGET: i32 = 7i32;
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_BASIC_REALM: i32 = 8i32;
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_DIGEST_REALM: i32 = 9i32;
pub const WS_SECURITY_BINDING_PROPERTY_HTTP_HEADER_AUTH_DIGEST_DOMAIN: i32 = 10i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_KEY_SIZE: i32 = 11i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_KEY_ENTROPY_MODE: i32 = 12i32;
pub const WS_SECURITY_BINDING_PROPERTY_MESSAGE_PROPERTIES: i32 = 13i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_MAX_PENDING_CONTEXTS: i32 = 14i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_MAX_ACTIVE_CONTEXTS: i32 = 15i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURE_CONVERSATION_VERSION: i32 = 16i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_SUPPORT_RENEW: i32 = 17i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_RENEWAL_INTERVAL: i32 = 18i32;
pub const WS_SECURITY_BINDING_PROPERTY_SECURITY_CONTEXT_ROLLOVER_INTERVAL: i32 = 19i32;
pub const WS_SECURITY_BINDING_PROPERTY_CERT_FAILURES_TO_IGNORE: i32 = 20i32;
pub const WS_SECURITY_BINDING_PROPERTY_DISABLE_CERT_REVOCATION_CHECK: i32 = 21i32;
pub const WS_SECURITY_BINDING_PROPERTY_DISALLOWED_SECURE_PROTOCOLS: i32 = 22i32;
pub const WS_SECURITY_BINDING_PROPERTY_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT: i32 = 23i32;
pub const WS_SSL_TRANSPORT_SECURITY_BINDING_TYPE: i32 = 1i32;
pub const WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TYPE: i32 = 2i32;
pub const WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TYPE: i32 = 3i32;
pub const WS_USERNAME_MESSAGE_SECURITY_BINDING_TYPE: i32 = 4i32;
pub const WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TYPE: i32 = 5i32;
pub const WS_XML_TOKEN_MESSAGE_SECURITY_BINDING_TYPE: i32 = 6i32;
pub const WS_SAML_MESSAGE_SECURITY_BINDING_TYPE: i32 = 7i32;
pub const WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TYPE: i32 = 8i32;
pub const WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING_TYPE: i32 = 9i32;
#[repr(C)]
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
#[repr(C)]
pub struct WS_SECURITY_CONTEXT(pub u8);
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_SECURITY_CONTEXT_PROPERTY_IDENTIFIER: i32 = 1i32;
pub const WS_SECURITY_CONTEXT_PROPERTY_USERNAME: i32 = 2i32;
pub const WS_SECURITY_CONTEXT_PROPERTY_MESSAGE_SECURITY_WINDOWS_TOKEN: i32 = 3i32;
pub const WS_SECURITY_CONTEXT_PROPERTY_SAML_ASSERTION: i32 = 4i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_SECURITY_HEADER_LAYOUT_STRICT: i32 = 1i32;
pub const WS_SECURITY_HEADER_LAYOUT_LAX: i32 = 2i32;
pub const WS_SECURITY_HEADER_LAYOUT_LAX_WITH_TIMESTAMP_FIRST: i32 = 3i32;
pub const WS_SECURITY_HEADER_LAYOUT_LAX_WITH_TIMESTAMP_LAST: i32 = 4i32;
pub const WS_SECURITY_HEADER_VERSION_1_0: i32 = 1i32;
pub const WS_SECURITY_HEADER_VERSION_1_1: i32 = 2i32;
pub const WS_SECURITY_KEY_ENTROPY_MODE_CLIENT_ONLY: i32 = 1i32;
pub const WS_SECURITY_KEY_ENTROPY_MODE_SERVER_ONLY: i32 = 2i32;
pub const WS_SECURITY_KEY_ENTROPY_MODE_COMBINED: i32 = 3i32;
#[repr(C)]
pub struct WS_SECURITY_KEY_HANDLE {
    pub keyHandleType: WS_SECURITY_KEY_HANDLE_TYPE,
}
impl ::core::marker::Copy for WS_SECURITY_KEY_HANDLE {}
impl ::core::clone::Clone for WS_SECURITY_KEY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE_TYPE: i32 = 1i32;
pub const WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE_TYPE: i32 = 2i32;
pub const WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE_TYPE: i32 = 3i32;
pub const WS_SECURITY_KEY_TYPE_NONE: i32 = 1i32;
pub const WS_SECURITY_KEY_TYPE_SYMMETRIC: i32 = 2i32;
pub const WS_SECURITY_KEY_TYPE_ASYMMETRIC: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    pub securityProperty: WS_SECURITY_PROPERTY,
}
impl ::core::marker::Copy for WS_SECURITY_PROPERTY_CONSTRAINT_0 {}
impl ::core::clone::Clone for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_SECURITY_PROPERTY_TRANSPORT_PROTECTION_LEVEL: i32 = 1i32;
pub const WS_SECURITY_PROPERTY_ALGORITHM_SUITE: i32 = 2i32;
pub const WS_SECURITY_PROPERTY_ALGORITHM_SUITE_NAME: i32 = 3i32;
pub const WS_SECURITY_PROPERTY_MAX_ALLOWED_LATENCY: i32 = 4i32;
pub const WS_SECURITY_PROPERTY_TIMESTAMP_VALIDITY_DURATION: i32 = 5i32;
pub const WS_SECURITY_PROPERTY_MAX_ALLOWED_CLOCK_SKEW: i32 = 6i32;
pub const WS_SECURITY_PROPERTY_TIMESTAMP_USAGE: i32 = 7i32;
pub const WS_SECURITY_PROPERTY_SECURITY_HEADER_LAYOUT: i32 = 8i32;
pub const WS_SECURITY_PROPERTY_SECURITY_HEADER_VERSION: i32 = 9i32;
pub const WS_SECURITY_PROPERTY_EXTENDED_PROTECTION_POLICY: i32 = 10i32;
pub const WS_SECURITY_PROPERTY_EXTENDED_PROTECTION_SCENARIO: i32 = 11i32;
pub const WS_SECURITY_PROPERTY_SERVICE_IDENTITIES: i32 = 12i32;
pub const WS_SECURITY_TIMESTAMP_USAGE_ALWAYS: i32 = 1i32;
pub const WS_SECURITY_TIMESTAMP_USAGE_NEVER: i32 = 2i32;
pub const WS_SECURITY_TIMESTAMP_USAGE_REQUESTS_ONLY: i32 = 3i32;
#[repr(C)]
pub struct WS_SECURITY_TOKEN(pub u8);
pub const WS_SECURITY_TOKEN_PROPERTY_KEY_TYPE: i32 = 1i32;
pub const WS_SECURITY_TOKEN_PROPERTY_VALID_FROM_TIME: i32 = 2i32;
pub const WS_SECURITY_TOKEN_PROPERTY_VALID_TILL_TIME: i32 = 3i32;
pub const WS_SECURITY_TOKEN_PROPERTY_SERIALIZED_XML: i32 = 4i32;
pub const WS_SECURITY_TOKEN_PROPERTY_ATTACHED_REFERENCE_XML: i32 = 5i32;
pub const WS_SECURITY_TOKEN_PROPERTY_UNATTACHED_REFERENCE_XML: i32 = 6i32;
pub const WS_SECURITY_TOKEN_PROPERTY_SYMMETRIC_KEY: i32 = 7i32;
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_LOCAL_ID: i32 = 1i32;
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_XML_BUFFER: i32 = 2i32;
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_CERT_THUMBPRINT: i32 = 3i32;
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_SECURITY_CONTEXT_ID: i32 = 4i32;
pub const WS_SECURITY_TOKEN_REFERENCE_MODE_SAML_ASSERTION_ID: i32 = 5i32;
pub type WS_SERVICE_ACCEPT_CHANNEL_CALLBACK = unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, channelstate: *mut *mut ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_SERVICE_HOST_ABORT: i32 = 0i32;
pub const WS_SERVICE_CHANNEL_FAULTED: i32 = 1i32;
pub type WS_SERVICE_CLOSE_CHANNEL_CALLBACK = unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, asynccontext: *const WS_ASYNC_CONTEXT) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_CONTRACT {
    pub contractDescription: *mut WS_CONTRACT_DESCRIPTION,
    pub defaultMessageHandlerCallback: WS_SERVICE_MESSAGE_RECEIVE_CALLBACK,
    pub methodTable: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_CONTRACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_CONTRACT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_ENDPOINT {
    pub address: WS_ENDPOINT_ADDRESS,
    pub channelBinding: WS_CHANNEL_BINDING,
    pub channelType: WS_CHANNEL_TYPE,
    pub securityDescription: *mut WS_SECURITY_DESCRIPTION,
    pub contract: *mut WS_SERVICE_CONTRACT,
    pub authorizationCallback: WS_SERVICE_SECURITY_CALLBACK,
    pub properties: *mut WS_SERVICE_ENDPOINT_PROPERTY,
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_SERVICE_ENDPOINT_PROPERTY_ACCEPT_CHANNEL_CALLBACK: i32 = 0i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_CLOSE_CHANNEL_CALLBACK: i32 = 1i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_ACCEPTING_CHANNELS: i32 = 2i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CONCURRENCY: i32 = 3i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_BODY_HEAP_MAX_SIZE: i32 = 4i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_BODY_HEAP_TRIM_SIZE: i32 = 5i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_MESSAGE_PROPERTIES: i32 = 6i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CALL_POOL_SIZE: i32 = 7i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CHANNEL_POOL_SIZE: i32 = 8i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_LISTENER_PROPERTIES: i32 = 9i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_CHECK_MUST_UNDERSTAND: i32 = 10i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA_EXCHANGE_TYPE: i32 = 11i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA: i32 = 12i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_METADATA_EXCHANGE_URL_SUFFIX: i32 = 13i32;
pub const WS_SERVICE_ENDPOINT_PROPERTY_MAX_CHANNELS: i32 = 14i32;
#[repr(C)]
pub struct WS_SERVICE_HOST(pub u8);
pub const WS_SERVICE_HOST_STATE_CREATED: i32 = 0i32;
pub const WS_SERVICE_HOST_STATE_OPENING: i32 = 1i32;
pub const WS_SERVICE_HOST_STATE_OPEN: i32 = 2i32;
pub const WS_SERVICE_HOST_STATE_CLOSING: i32 = 3i32;
pub const WS_SERVICE_HOST_STATE_CLOSED: i32 = 4i32;
pub const WS_SERVICE_HOST_STATE_FAULTED: i32 = 5i32;
pub type WS_SERVICE_MESSAGE_RECEIVE_CALLBACK = unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_SERVICE_OPERATION_MESSAGE_NILLABLE_ELEMENT: i32 = 1i32;
#[repr(C)]
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
#[repr(C)]
pub struct WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    pub callback: WS_SERVICE_ACCEPT_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    pub callback: WS_SERVICE_CLOSE_CHANNEL_CALLBACK,
}
impl ::core::marker::Copy for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {}
impl ::core::clone::Clone for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_SERVICE_PROPERTY_HOST_USER_STATE: i32 = 0i32;
pub const WS_SERVICE_PROPERTY_FAULT_DISCLOSURE: i32 = 1i32;
pub const WS_SERVICE_PROPERTY_FAULT_LANGID: i32 = 2i32;
pub const WS_SERVICE_PROPERTY_HOST_STATE: i32 = 3i32;
pub const WS_SERVICE_PROPERTY_METADATA: i32 = 4i32;
pub const WS_SERVICE_PROPERTY_CLOSE_TIMEOUT: i32 = 5i32;
#[repr(C)]
pub struct WS_SERVICE_PROXY(pub u8);
pub const WS_SERVICE_PROXY_STATE_CREATED: i32 = 0i32;
pub const WS_SERVICE_PROXY_STATE_OPENING: i32 = 1i32;
pub const WS_SERVICE_PROXY_STATE_OPEN: i32 = 2i32;
pub const WS_SERVICE_PROXY_STATE_CLOSING: i32 = 3i32;
pub const WS_SERVICE_PROXY_STATE_CLOSED: i32 = 4i32;
pub const WS_SERVICE_PROXY_STATE_FAULTED: i32 = 5i32;
#[cfg(feature = "Win32_Foundation")]
pub type WS_SERVICE_SECURITY_CALLBACK = unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, authorized: *mut super::super::Foundation::BOOL, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SERVICE_SECURITY_IDENTITIES {
    pub serviceIdentities: *mut WS_STRING,
    pub serviceIdentityCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SERVICE_SECURITY_IDENTITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SERVICE_SECURITY_IDENTITIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WS_SERVICE_STUB_CALLBACK = unsafe extern "system" fn(context: *const WS_OPERATION_CONTEXT, frame: *const ::core::ffi::c_void, callback: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_SET_CHANNEL_PROPERTY_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, id: WS_CHANNEL_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_SET_LISTENER_PROPERTY_CALLBACK = unsafe extern "system" fn(listenerinstance: *const ::core::ffi::c_void, id: WS_LISTENER_PROPERTY_ID, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_SHUTDOWN_SESSION_CHANNEL_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SOAPUDP_URL {
    pub url: WS_URL,
    pub host: WS_STRING,
    pub port: u16,
    pub portAsString: WS_STRING,
    pub path: WS_STRING,
    pub query: WS_STRING,
    pub fragment: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SOAPUDP_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SOAPUDP_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SPN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub spn: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SPN_ENDPOINT_IDENTITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SPN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
pub struct WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
}
impl ::core::marker::Copy for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_STRING {
    pub length: u32,
    pub chars: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_STRING_USERNAME_CREDENTIAL {
    pub credential: WS_USERNAME_CREDENTIAL,
    pub username: WS_STRING,
    pub password: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_STRING_USERNAME_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_STRING_USERNAME_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credential: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL,
    pub username: WS_STRING,
    pub password: WS_STRING,
    pub domain: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_STRUCT_ABSTRACT: i32 = 1i32;
#[repr(C)]
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
pub const WS_STRUCT_IGNORE_TRAILING_ELEMENT_CONTENT: i32 = 2i32;
pub const WS_STRUCT_IGNORE_UNHANDLED_ATTRIBUTES: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_SUBJECT_NAME_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub storeLocation: u32,
    pub storeName: WS_STRING,
    pub subjectName: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_SUBJECT_NAME_CERT_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_TCP_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_TCP_BINDING_TEMPLATE {}
impl ::core::clone::Clone for WS_TCP_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_TCP_POLICY_DESCRIPTION {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
}
impl ::core::marker::Copy for WS_TCP_POLICY_DESCRIPTION {}
impl ::core::clone::Clone for WS_TCP_POLICY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    pub bindingConstraint: WS_SECURITY_BINDING_CONSTRAINT,
}
impl ::core::marker::Copy for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
impl ::core::clone::Clone for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    pub channelProperties: WS_CHANNEL_PROPERTIES,
    pub securityProperties: WS_SECURITY_PROPERTIES,
    pub sspiTransportSecurityBinding: WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE,
    pub usernameMessageSecurityBinding: WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE,
    pub securityContextSecurityBinding: WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_THUMBPRINT_CERT_CREDENTIAL {
    pub credential: WS_CERT_CREDENTIAL,
    pub storeLocation: u32,
    pub storeName: WS_STRING,
    pub thumbprint: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_THUMBPRINT_CERT_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_TIMESPAN {
    pub ticks: i64,
}
impl ::core::marker::Copy for WS_TIMESPAN {}
impl ::core::clone::Clone for WS_TIMESPAN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_TRACE_API_NONE: i32 = -1i32;
pub const WS_TRACE_API_START_READER_CANONICALIZATION: i32 = 0i32;
pub const WS_TRACE_API_END_READER_CANONICALIZATION: i32 = 1i32;
pub const WS_TRACE_API_START_WRITER_CANONICALIZATION: i32 = 2i32;
pub const WS_TRACE_API_END_WRITER_CANONICALIZATION: i32 = 3i32;
pub const WS_TRACE_API_CREATE_XML_BUFFER: i32 = 4i32;
pub const WS_TRACE_API_REMOVE_NODE: i32 = 5i32;
pub const WS_TRACE_API_CREATE_READER: i32 = 6i32;
pub const WS_TRACE_API_SET_INPUT: i32 = 7i32;
pub const WS_TRACE_API_SET_INPUT_TO_BUFFER: i32 = 8i32;
pub const WS_TRACE_API_FREE_XML_READER: i32 = 9i32;
pub const WS_TRACE_API_GET_READER_PROPERTY: i32 = 10i32;
pub const WS_TRACE_API_GET_READER_NODE: i32 = 11i32;
pub const WS_TRACE_API_FILL_READER: i32 = 12i32;
pub const WS_TRACE_API_READ_START_ELEMENT: i32 = 13i32;
pub const WS_TRACE_API_READ_TO_START_ELEMENT: i32 = 14i32;
pub const WS_TRACE_API_READ_START_ATTRIBUTE: i32 = 15i32;
pub const WS_TRACE_API_READ_END_ATTRIBUTE: i32 = 16i32;
pub const WS_TRACE_API_READ_NODE: i32 = 17i32;
pub const WS_TRACE_API_SKIP_NODE: i32 = 18i32;
pub const WS_TRACE_API_READ_END_ELEMENT: i32 = 19i32;
pub const WS_TRACE_API_FIND_ATTRIBUTE: i32 = 20i32;
pub const WS_TRACE_API_READ_ELEMENT_VALUE: i32 = 21i32;
pub const WS_TRACE_API_READ_CHARS: i32 = 22i32;
pub const WS_TRACE_API_READ_CHARS_UTF8: i32 = 23i32;
pub const WS_TRACE_API_READ_BYTES: i32 = 24i32;
pub const WS_TRACE_API_READ_ARRAY: i32 = 25i32;
pub const WS_TRACE_API_GET_READER_POSITION: i32 = 26i32;
pub const WS_TRACE_API_SET_READER_POSITION: i32 = 27i32;
pub const WS_TRACE_API_MOVE_READER: i32 = 28i32;
pub const WS_TRACE_API_CREATE_WRITER: i32 = 29i32;
pub const WS_TRACE_API_FREE_XML_WRITER: i32 = 30i32;
pub const WS_TRACE_API_SET_OUTPUT: i32 = 31i32;
pub const WS_TRACE_API_SET_OUTPUT_TO_BUFFER: i32 = 32i32;
pub const WS_TRACE_API_GET_WRITER_PROPERTY: i32 = 33i32;
pub const WS_TRACE_API_FLUSH_WRITER: i32 = 34i32;
pub const WS_TRACE_API_WRITE_START_ELEMENT: i32 = 35i32;
pub const WS_TRACE_API_WRITE_END_START_ELEMENT: i32 = 36i32;
pub const WS_TRACE_API_WRITE_XMLNS_ATTRIBUTE: i32 = 37i32;
pub const WS_TRACE_API_WRITE_START_ATTRIBUTE: i32 = 38i32;
pub const WS_TRACE_API_WRITE_END_ATTRIBUTE: i32 = 39i32;
pub const WS_TRACE_API_WRITE_VALUE: i32 = 40i32;
pub const WS_TRACE_API_WRITE_XML_BUFFER: i32 = 41i32;
pub const WS_TRACE_API_READ_XML_BUFFER: i32 = 42i32;
pub const WS_TRACE_API_WRITE_XML_BUFFER_TO_BYTES: i32 = 43i32;
pub const WS_TRACE_API_READ_XML_BUFFER_FROM_BYTES: i32 = 44i32;
pub const WS_TRACE_API_WRITE_ARRAY: i32 = 45i32;
pub const WS_TRACE_API_WRITE_QUALIFIED_NAME: i32 = 46i32;
pub const WS_TRACE_API_WRITE_CHARS: i32 = 47i32;
pub const WS_TRACE_API_WRITE_CHARS_UTF8: i32 = 48i32;
pub const WS_TRACE_API_WRITE_BYTES: i32 = 49i32;
pub const WS_TRACE_API_PUSH_BYTES: i32 = 50i32;
pub const WS_TRACE_API_PULL_BYTES: i32 = 51i32;
pub const WS_TRACE_API_WRITE_END_ELEMENT: i32 = 52i32;
pub const WS_TRACE_API_WRITE_TEXT: i32 = 53i32;
pub const WS_TRACE_API_WRITE_START_CDATA: i32 = 54i32;
pub const WS_TRACE_API_WRITE_END_CDATA: i32 = 55i32;
pub const WS_TRACE_API_WRITE_NODE: i32 = 56i32;
pub const WS_TRACE_API_PREFIX_FROM_NAMESPACE: i32 = 57i32;
pub const WS_TRACE_API_GET_WRITER_POSITION: i32 = 58i32;
pub const WS_TRACE_API_SET_WRITER_POSITION: i32 = 59i32;
pub const WS_TRACE_API_MOVE_WRITER: i32 = 60i32;
pub const WS_TRACE_API_TRIM_XML_WHITESPACE: i32 = 61i32;
pub const WS_TRACE_API_VERIFY_XML_NCNAME: i32 = 62i32;
pub const WS_TRACE_API_XML_STRING_EQUALS: i32 = 63i32;
pub const WS_TRACE_API_NAMESPACE_FROM_PREFIX: i32 = 64i32;
pub const WS_TRACE_API_READ_QUALIFIED_NAME: i32 = 65i32;
pub const WS_TRACE_API_GET_XML_ATTRIBUTE: i32 = 66i32;
pub const WS_TRACE_API_COPY_NODE: i32 = 67i32;
pub const WS_TRACE_API_ASYNC_EXECUTE: i32 = 68i32;
pub const WS_TRACE_API_CREATE_CHANNEL: i32 = 69i32;
pub const WS_TRACE_API_OPEN_CHANNEL: i32 = 70i32;
pub const WS_TRACE_API_SEND_MESSAGE: i32 = 71i32;
pub const WS_TRACE_API_RECEIVE_MESSAGE: i32 = 72i32;
pub const WS_TRACE_API_REQUEST_REPLY: i32 = 73i32;
pub const WS_TRACE_API_SEND_REPLY_MESSAGE: i32 = 74i32;
pub const WS_TRACE_API_SEND_FAULT_MESSAGE_FOR_ERROR: i32 = 75i32;
pub const WS_TRACE_API_GET_CHANNEL_PROPERTY: i32 = 76i32;
pub const WS_TRACE_API_SET_CHANNEL_PROPERTY: i32 = 77i32;
pub const WS_TRACE_API_WRITE_MESSAGE_START: i32 = 78i32;
pub const WS_TRACE_API_WRITE_MESSAGE_END: i32 = 79i32;
pub const WS_TRACE_API_READ_MESSAGE_START: i32 = 80i32;
pub const WS_TRACE_API_READ_MESSAGE_END: i32 = 81i32;
pub const WS_TRACE_API_CLOSE_CHANNEL: i32 = 82i32;
pub const WS_TRACE_API_ABORT_CHANNEL: i32 = 83i32;
pub const WS_TRACE_API_FREE_CHANNEL: i32 = 84i32;
pub const WS_TRACE_API_RESET_CHANNEL: i32 = 85i32;
pub const WS_TRACE_API_ABANDON_MESSAGE: i32 = 86i32;
pub const WS_TRACE_API_SHUTDOWN_SESSION_CHANNEL: i32 = 87i32;
pub const WS_TRACE_API_GET_CONTEXT_PROPERTY: i32 = 88i32;
pub const WS_TRACE_API_GET_DICTIONARY: i32 = 89i32;
pub const WS_TRACE_API_READ_ENDPOINT_ADDRESS_EXTENSION: i32 = 90i32;
pub const WS_TRACE_API_CREATE_ERROR: i32 = 91i32;
pub const WS_TRACE_API_ADD_ERROR_STRING: i32 = 92i32;
pub const WS_TRACE_API_GET_ERROR_STRING: i32 = 93i32;
pub const WS_TRACE_API_COPY_ERROR: i32 = 94i32;
pub const WS_TRACE_API_GET_ERROR_PROPERTY: i32 = 95i32;
pub const WS_TRACE_API_SET_ERROR_PROPERTY: i32 = 96i32;
pub const WS_TRACE_API_RESET_ERROR: i32 = 97i32;
pub const WS_TRACE_API_FREE_ERROR: i32 = 98i32;
pub const WS_TRACE_API_GET_FAULT_ERROR_PROPERTY: i32 = 99i32;
pub const WS_TRACE_API_SET_FAULT_ERROR_PROPERTY: i32 = 100i32;
pub const WS_TRACE_API_CREATE_FAULT_FROM_ERROR: i32 = 101i32;
pub const WS_TRACE_API_SET_FAULT_ERROR_DETAIL: i32 = 102i32;
pub const WS_TRACE_API_GET_FAULT_ERROR_DETAIL: i32 = 103i32;
pub const WS_TRACE_API_CREATE_HEAP: i32 = 104i32;
pub const WS_TRACE_API_ALLOC: i32 = 105i32;
pub const WS_TRACE_API_GET_HEAP_PROPERTY: i32 = 106i32;
pub const WS_TRACE_API_RESET_HEAP: i32 = 107i32;
pub const WS_TRACE_API_FREE_HEAP: i32 = 108i32;
pub const WS_TRACE_API_CREATE_LISTENER: i32 = 109i32;
pub const WS_TRACE_API_OPEN_LISTENER: i32 = 110i32;
pub const WS_TRACE_API_ACCEPT_CHANNEL: i32 = 111i32;
pub const WS_TRACE_API_CLOSE_LISTENER: i32 = 112i32;
pub const WS_TRACE_API_ABORT_LISTENER: i32 = 113i32;
pub const WS_TRACE_API_RESET_LISTENER: i32 = 114i32;
pub const WS_TRACE_API_FREE_LISTENER: i32 = 115i32;
pub const WS_TRACE_API_GET_LISTENER_PROPERTY: i32 = 116i32;
pub const WS_TRACE_API_SET_LISTENER_PROPERTY: i32 = 117i32;
pub const WS_TRACE_API_CREATE_CHANNEL_FOR_LISTENER: i32 = 118i32;
pub const WS_TRACE_API_CREATE_MESSAGE: i32 = 119i32;
pub const WS_TRACE_API_CREATE_MESSAGE_FOR_CHANNEL: i32 = 120i32;
pub const WS_TRACE_API_INITIALIZE_MESSAGE: i32 = 121i32;
pub const WS_TRACE_API_RESET_MESSAGE: i32 = 122i32;
pub const WS_TRACE_API_FREE_MESSAGE: i32 = 123i32;
pub const WS_TRACE_API_GET_HEADER_ATTRIBUTES: i32 = 124i32;
pub const WS_TRACE_API_GET_HEADER: i32 = 125i32;
pub const WS_TRACE_API_GET_CUSTOM_HEADER: i32 = 126i32;
pub const WS_TRACE_API_REMOVE_HEADER: i32 = 127i32;
pub const WS_TRACE_API_SET_HEADER: i32 = 128i32;
pub const WS_TRACE_API_REMOVE_CUSTOM_HEADER: i32 = 129i32;
pub const WS_TRACE_API_ADD_CUSTOM_HEADER: i32 = 130i32;
pub const WS_TRACE_API_ADD_MAPPED_HEADER: i32 = 131i32;
pub const WS_TRACE_API_REMOVE_MAPPED_HEADER: i32 = 132i32;
pub const WS_TRACE_API_GET_MAPPED_HEADER: i32 = 133i32;
pub const WS_TRACE_API_WRITE_BODY: i32 = 134i32;
pub const WS_TRACE_API_READ_BODY: i32 = 135i32;
pub const WS_TRACE_API_WRITE_ENVELOPE_START: i32 = 136i32;
pub const WS_TRACE_API_WRITE_ENVELOPE_END: i32 = 137i32;
pub const WS_TRACE_API_READ_ENVELOPE_START: i32 = 138i32;
pub const WS_TRACE_API_READ_ENVELOPE_END: i32 = 139i32;
pub const WS_TRACE_API_GET_MESSAGE_PROPERTY: i32 = 140i32;
pub const WS_TRACE_API_SET_MESSAGE_PROPERTY: i32 = 141i32;
pub const WS_TRACE_API_ADDRESS_MESSAGE: i32 = 142i32;
pub const WS_TRACE_API_CHECK_MUST_UNDERSTAND_HEADERS: i32 = 143i32;
pub const WS_TRACE_API_MARK_HEADER_AS_UNDERSTOOD: i32 = 144i32;
pub const WS_TRACE_API_FILL_BODY: i32 = 145i32;
pub const WS_TRACE_API_FLUSH_BODY: i32 = 146i32;
pub const WS_TRACE_API_REQUEST_SECURITY_TOKEN: i32 = 147i32;
pub const WS_TRACE_API_GET_SECURITY_TOKEN_PROPERTY: i32 = 148i32;
pub const WS_TRACE_API_CREATE_XML_SECURITY_TOKEN: i32 = 149i32;
pub const WS_TRACE_API_FREE_SECURITY_TOKEN: i32 = 150i32;
pub const WS_TRACE_API_REVOKE_SECURITY_CONTEXT: i32 = 151i32;
pub const WS_TRACE_API_GET_SECURITY_CONTEXT_PROPERTY: i32 = 152i32;
pub const WS_TRACE_API_READ_ELEMENT_TYPE: i32 = 153i32;
pub const WS_TRACE_API_READ_ATTRIBUTE_TYPE: i32 = 154i32;
pub const WS_TRACE_API_READ_TYPE: i32 = 155i32;
pub const WS_TRACE_API_WRITE_ELEMENT_TYPE: i32 = 156i32;
pub const WS_TRACE_API_WRITE_ATTRIBUTE_TYPE: i32 = 157i32;
pub const WS_TRACE_API_WRITE_TYPE: i32 = 158i32;
pub const WS_TRACE_API_SERVICE_REGISTER_FOR_CANCEL: i32 = 159i32;
pub const WS_TRACE_API_GET_SERVICE_HOST_PROPERTY: i32 = 160i32;
pub const WS_TRACE_API_CREATE_SERVICE_HOST: i32 = 161i32;
pub const WS_TRACE_API_OPEN_SERVICE_HOST: i32 = 162i32;
pub const WS_TRACE_API_CLOSE_SERVICE_HOST: i32 = 163i32;
pub const WS_TRACE_API_ABORT_SERVICE_HOST: i32 = 164i32;
pub const WS_TRACE_API_FREE_SERVICE_HOST: i32 = 165i32;
pub const WS_TRACE_API_RESET_SERVICE_HOST: i32 = 166i32;
pub const WS_TRACE_API_GET_SERVICE_PROXY_PROPERTY: i32 = 167i32;
pub const WS_TRACE_API_CREATE_SERVICE_PROXY: i32 = 168i32;
pub const WS_TRACE_API_OPEN_SERVICE_PROXY: i32 = 169i32;
pub const WS_TRACE_API_CLOSE_SERVICE_PROXY: i32 = 170i32;
pub const WS_TRACE_API_ABORT_SERVICE_PROXY: i32 = 171i32;
pub const WS_TRACE_API_FREE_SERVICE_PROXY: i32 = 172i32;
pub const WS_TRACE_API_RESET_SERVICE_PROXY: i32 = 173i32;
pub const WS_TRACE_API_ABORT_CALL: i32 = 174i32;
pub const WS_TRACE_API_CALL: i32 = 175i32;
pub const WS_TRACE_API_DECODE_URL: i32 = 176i32;
pub const WS_TRACE_API_ENCODE_URL: i32 = 177i32;
pub const WS_TRACE_API_COMBINE_URL: i32 = 178i32;
pub const WS_TRACE_API_DATETIME_TO_FILETIME: i32 = 179i32;
pub const WS_TRACE_API_FILETIME_TO_DATETIME: i32 = 180i32;
pub const WS_TRACE_API_DUMP_MEMORY: i32 = 181i32;
pub const WS_TRACE_API_SET_AUTOFAIL: i32 = 182i32;
pub const WS_TRACE_API_CREATE_METADATA: i32 = 183i32;
pub const WS_TRACE_API_READ_METADATA: i32 = 184i32;
pub const WS_TRACE_API_FREE_METADATA: i32 = 185i32;
pub const WS_TRACE_API_RESET_METADATA: i32 = 186i32;
pub const WS_TRACE_API_GET_METADATA_PROPERTY: i32 = 187i32;
pub const WS_TRACE_API_GET_MISSING_METADATA_DOCUMENT_ADDRESS: i32 = 188i32;
pub const WS_TRACE_API_GET_METADATA_ENDPOINTS: i32 = 189i32;
pub const WS_TRACE_API_MATCH_POLICY_ALTERNATIVE: i32 = 190i32;
pub const WS_TRACE_API_GET_POLICY_PROPERTY: i32 = 191i32;
pub const WS_TRACE_API_GET_POLICY_ALTERNATIVE_COUNT: i32 = 192i32;
pub const WS_TRACE_API_WS_CREATE_SERVICE_PROXY_FROM_TEMPLATE: i32 = 193i32;
pub const WS_TRACE_API_WS_CREATE_SERVICE_HOST_FROM_TEMPLATE: i32 = 194i32;
pub const WS_STREAMED_INPUT_TRANSFER_MODE: i32 = 1i32;
pub const WS_STREAMED_OUTPUT_TRANSFER_MODE: i32 = 2i32;
pub const WS_BUFFERED_TRANSFER_MODE: i32 = 0i32;
pub const WS_STREAMED_TRANSFER_MODE: i32 = 3i32;
pub const WS_TRUST_VERSION_FEBRUARY_2005: i32 = 1i32;
pub const WS_TRUST_VERSION_1_3: i32 = 2i32;
pub const WS_BOOL_TYPE: i32 = 0i32;
pub const WS_INT8_TYPE: i32 = 1i32;
pub const WS_INT16_TYPE: i32 = 2i32;
pub const WS_INT32_TYPE: i32 = 3i32;
pub const WS_INT64_TYPE: i32 = 4i32;
pub const WS_UINT8_TYPE: i32 = 5i32;
pub const WS_UINT16_TYPE: i32 = 6i32;
pub const WS_UINT32_TYPE: i32 = 7i32;
pub const WS_UINT64_TYPE: i32 = 8i32;
pub const WS_FLOAT_TYPE: i32 = 9i32;
pub const WS_DOUBLE_TYPE: i32 = 10i32;
pub const WS_DECIMAL_TYPE: i32 = 11i32;
pub const WS_DATETIME_TYPE: i32 = 12i32;
pub const WS_TIMESPAN_TYPE: i32 = 13i32;
pub const WS_GUID_TYPE: i32 = 14i32;
pub const WS_UNIQUE_ID_TYPE: i32 = 15i32;
pub const WS_STRING_TYPE: i32 = 16i32;
pub const WS_WSZ_TYPE: i32 = 17i32;
pub const WS_BYTES_TYPE: i32 = 18i32;
pub const WS_XML_STRING_TYPE: i32 = 19i32;
pub const WS_XML_QNAME_TYPE: i32 = 20i32;
pub const WS_XML_BUFFER_TYPE: i32 = 21i32;
pub const WS_CHAR_ARRAY_TYPE: i32 = 22i32;
pub const WS_UTF8_ARRAY_TYPE: i32 = 23i32;
pub const WS_BYTE_ARRAY_TYPE: i32 = 24i32;
pub const WS_DESCRIPTION_TYPE: i32 = 25i32;
pub const WS_STRUCT_TYPE: i32 = 26i32;
pub const WS_CUSTOM_TYPE: i32 = 27i32;
pub const WS_ENDPOINT_ADDRESS_TYPE: i32 = 28i32;
pub const WS_FAULT_TYPE: i32 = 29i32;
pub const WS_VOID_TYPE: i32 = 30i32;
pub const WS_ENUM_TYPE: i32 = 31i32;
pub const WS_DURATION_TYPE: i32 = 32i32;
pub const WS_UNION_TYPE: i32 = 33i32;
pub const WS_ANY_ATTRIBUTES_TYPE: i32 = 34i32;
pub const WS_ELEMENT_TYPE_MAPPING: i32 = 1i32;
pub const WS_ATTRIBUTE_TYPE_MAPPING: i32 = 2i32;
pub const WS_ELEMENT_CONTENT_TYPE_MAPPING: i32 = 3i32;
pub const WS_ANY_ELEMENT_TYPE_MAPPING: i32 = 4i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_UNIQUE_ID {
    pub uri: WS_STRING,
    pub guid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_UNIQUE_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_UNIQUE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_UPN_ENDPOINT_IDENTITY {
    pub identity: WS_ENDPOINT_IDENTITY,
    pub upn: WS_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_UPN_ENDPOINT_IDENTITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_UPN_ENDPOINT_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_URL {
    pub scheme: WS_URL_SCHEME_TYPE,
}
impl ::core::marker::Copy for WS_URL {}
impl ::core::clone::Clone for WS_URL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_URL_FLAGS_ALLOW_HOST_WILDCARDS: i32 = 1i32;
pub const WS_URL_FLAGS_NO_PATH_COLLAPSE: i32 = 2i32;
pub const WS_URL_FLAGS_ZERO_TERMINATE: i32 = 4i32;
pub const WS_URL_HTTP_SCHEME_TYPE: i32 = 0i32;
pub const WS_URL_HTTPS_SCHEME_TYPE: i32 = 1i32;
pub const WS_URL_NETTCP_SCHEME_TYPE: i32 = 2i32;
pub const WS_URL_SOAPUDP_SCHEME_TYPE: i32 = 3i32;
pub const WS_URL_NETPIPE_SCHEME_TYPE: i32 = 4i32;
#[repr(C)]
pub struct WS_USERNAME_CREDENTIAL {
    pub credentialType: WS_USERNAME_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_USERNAME_CREDENTIAL {}
impl ::core::clone::Clone for WS_USERNAME_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_STRING_USERNAME_CREDENTIAL_TYPE: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING {
    pub binding: WS_SECURITY_BINDING,
    pub bindingUsage: WS_MESSAGE_SECURITY_USAGE,
    pub clientCredential: *mut WS_USERNAME_CREDENTIAL,
    pub passwordValidator: WS_VALIDATE_PASSWORD_CALLBACK,
    pub passwordValidatorCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    pub securityBindingProperties: WS_SECURITY_BINDING_PROPERTIES,
    pub clientCredential: *mut WS_USERNAME_CREDENTIAL,
    pub passwordValidator: WS_VALIDATE_PASSWORD_CALLBACK,
    pub passwordValidatorCallbackState: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub type WS_VALIDATE_PASSWORD_CALLBACK = unsafe extern "system" fn(passwordvalidatorcallbackstate: *const ::core::ffi::c_void, username: *const WS_STRING, password: *const WS_STRING, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_VALIDATE_SAML_CALLBACK = unsafe extern "system" fn(samlvalidatorcallbackstate: *const ::core::ffi::c_void, samlassertion: *const WS_XML_BUFFER, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_BOOL_VALUE_TYPE: i32 = 0i32;
pub const WS_INT8_VALUE_TYPE: i32 = 1i32;
pub const WS_INT16_VALUE_TYPE: i32 = 2i32;
pub const WS_INT32_VALUE_TYPE: i32 = 3i32;
pub const WS_INT64_VALUE_TYPE: i32 = 4i32;
pub const WS_UINT8_VALUE_TYPE: i32 = 5i32;
pub const WS_UINT16_VALUE_TYPE: i32 = 6i32;
pub const WS_UINT32_VALUE_TYPE: i32 = 7i32;
pub const WS_UINT64_VALUE_TYPE: i32 = 8i32;
pub const WS_FLOAT_VALUE_TYPE: i32 = 9i32;
pub const WS_DOUBLE_VALUE_TYPE: i32 = 10i32;
pub const WS_DECIMAL_VALUE_TYPE: i32 = 11i32;
pub const WS_DATETIME_VALUE_TYPE: i32 = 12i32;
pub const WS_TIMESPAN_VALUE_TYPE: i32 = 13i32;
pub const WS_GUID_VALUE_TYPE: i32 = 14i32;
pub const WS_DURATION_VALUE_TYPE: i32 = 15i32;
#[repr(C)]
pub struct WS_VOID_DESCRIPTION {
    pub size: u32,
}
impl ::core::marker::Copy for WS_VOID_DESCRIPTION {}
impl ::core::clone::Clone for WS_VOID_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    pub credentialType: WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE,
}
impl ::core::marker::Copy for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::clone::Clone for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: i32 = 1i32;
pub const WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: i32 = 2i32;
pub const WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE: i32 = 3i32;
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_KERBEROS: i32 = 1i32;
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_NTLM: i32 = 2i32;
pub const WS_WINDOWS_INTEGRATED_AUTH_PACKAGE_SPNEGO: i32 = 3i32;
pub type WS_WRITE_CALLBACK = unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, buffers: *const WS_BYTES, count: u32, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_WRITE_MESSAGE_END_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub type WS_WRITE_MESSAGE_START_CALLBACK = unsafe extern "system" fn(channelinstance: *const ::core::ffi::c_void, message: *const WS_MESSAGE, asynccontext: *const WS_ASYNC_CONTEXT, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
pub const WS_WRITE_REQUIRED_VALUE: i32 = 1i32;
pub const WS_WRITE_REQUIRED_POINTER: i32 = 2i32;
pub const WS_WRITE_NILLABLE_VALUE: i32 = 3i32;
pub const WS_WRITE_NILLABLE_POINTER: i32 = 4i32;
pub type WS_WRITE_TYPE_CALLBACK = unsafe extern "system" fn(writer: *const WS_XML_WRITER, typemapping: WS_TYPE_MAPPING, descriptiondata: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, valuesize: u32, error: *const WS_ERROR) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_BUFFER(pub u8);
#[repr(C)]
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
pub const WS_EXCLUSIVE_XML_CANONICALIZATION_ALGORITHM: i32 = 0i32;
pub const WS_EXCLUSIVE_WITH_COMMENTS_XML_CANONICALIZATION_ALGORITHM: i32 = 1i32;
pub const WS_INCLUSIVE_XML_CANONICALIZATION_ALGORITHM: i32 = 2i32;
pub const WS_INCLUSIVE_WITH_COMMENTS_XML_CANONICALIZATION_ALGORITHM: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_XML_CANONICALIZATION_PROPERTY_ALGORITHM: i32 = 0i32;
pub const WS_XML_CANONICALIZATION_PROPERTY_INCLUSIVE_PREFIXES: i32 = 1i32;
pub const WS_XML_CANONICALIZATION_PROPERTY_OMITTED_ELEMENT: i32 = 2i32;
pub const WS_XML_CANONICALIZATION_PROPERTY_OUTPUT_BUFFER_SIZE: i32 = 3i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WS_XML_DICTIONARY {
    pub guid: ::windows_sys::core::GUID,
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_GUID_TEXT {
    pub text: WS_XML_TEXT,
    pub value: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WS_XML_GUID_TEXT {}
impl ::core::clone::Clone for WS_XML_GUID_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_NODE {
    pub nodeType: WS_XML_NODE_TYPE,
}
impl ::core::marker::Copy for WS_XML_NODE {}
impl ::core::clone::Clone for WS_XML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_XML_NODE_TYPE_ELEMENT: i32 = 1i32;
pub const WS_XML_NODE_TYPE_TEXT: i32 = 2i32;
pub const WS_XML_NODE_TYPE_END_ELEMENT: i32 = 3i32;
pub const WS_XML_NODE_TYPE_COMMENT: i32 = 4i32;
pub const WS_XML_NODE_TYPE_CDATA: i32 = 6i32;
pub const WS_XML_NODE_TYPE_END_CDATA: i32 = 7i32;
pub const WS_XML_NODE_TYPE_EOF: i32 = 8i32;
pub const WS_XML_NODE_TYPE_BOF: i32 = 9i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_READER(pub u8);
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_READER_ENCODING {
    pub encodingType: WS_XML_READER_ENCODING_TYPE,
}
impl ::core::marker::Copy for WS_XML_READER_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_XML_READER_ENCODING_TYPE_TEXT: i32 = 1i32;
pub const WS_XML_READER_ENCODING_TYPE_BINARY: i32 = 2i32;
pub const WS_XML_READER_ENCODING_TYPE_MTOM: i32 = 3i32;
pub const WS_XML_READER_ENCODING_TYPE_RAW: i32 = 4i32;
#[repr(C)]
pub struct WS_XML_READER_INPUT {
    pub inputType: WS_XML_READER_INPUT_TYPE,
}
impl ::core::marker::Copy for WS_XML_READER_INPUT {}
impl ::core::clone::Clone for WS_XML_READER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_XML_READER_INPUT_TYPE_BUFFER: i32 = 1i32;
pub const WS_XML_READER_INPUT_TYPE_STREAM: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_XML_READER_PROPERTY_MAX_DEPTH: i32 = 0i32;
pub const WS_XML_READER_PROPERTY_ALLOW_FRAGMENT: i32 = 1i32;
pub const WS_XML_READER_PROPERTY_MAX_ATTRIBUTES: i32 = 2i32;
pub const WS_XML_READER_PROPERTY_READ_DECLARATION: i32 = 3i32;
pub const WS_XML_READER_PROPERTY_CHARSET: i32 = 4i32;
pub const WS_XML_READER_PROPERTY_ROW: i32 = 5i32;
pub const WS_XML_READER_PROPERTY_COLUMN: i32 = 6i32;
pub const WS_XML_READER_PROPERTY_UTF8_TRIM_SIZE: i32 = 7i32;
pub const WS_XML_READER_PROPERTY_STREAM_BUFFER_SIZE: i32 = 8i32;
pub const WS_XML_READER_PROPERTY_IN_ATTRIBUTE: i32 = 9i32;
pub const WS_XML_READER_PROPERTY_STREAM_MAX_ROOT_MIME_PART_SIZE: i32 = 10i32;
pub const WS_XML_READER_PROPERTY_STREAM_MAX_MIME_HEADERS_SIZE: i32 = 11i32;
pub const WS_XML_READER_PROPERTY_MAX_MIME_PARTS: i32 = 12i32;
pub const WS_XML_READER_PROPERTY_ALLOW_INVALID_CHARACTER_REFERENCES: i32 = 13i32;
pub const WS_XML_READER_PROPERTY_MAX_NAMESPACES: i32 = 14i32;
#[repr(C)]
pub struct WS_XML_READER_RAW_ENCODING {
    pub encoding: WS_XML_READER_ENCODING,
}
impl ::core::marker::Copy for WS_XML_READER_RAW_ENCODING {}
impl ::core::clone::Clone for WS_XML_READER_RAW_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
pub const WS_XML_SECURITY_TOKEN_PROPERTY_ATTACHED_REFERENCE: i32 = 1i32;
pub const WS_XML_SECURITY_TOKEN_PROPERTY_UNATTACHED_REFERENCE: i32 = 2i32;
pub const WS_XML_SECURITY_TOKEN_PROPERTY_VALID_FROM_TIME: i32 = 3i32;
pub const WS_XML_SECURITY_TOKEN_PROPERTY_VALID_TILL_TIME: i32 = 4i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_TEXT {
    pub textType: WS_XML_TEXT_TYPE,
}
impl ::core::marker::Copy for WS_XML_TEXT {}
impl ::core::clone::Clone for WS_XML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const WS_XML_TEXT_TYPE_UTF8: i32 = 1i32;
pub const WS_XML_TEXT_TYPE_UTF16: i32 = 2i32;
pub const WS_XML_TEXT_TYPE_BASE64: i32 = 3i32;
pub const WS_XML_TEXT_TYPE_BOOL: i32 = 4i32;
pub const WS_XML_TEXT_TYPE_INT32: i32 = 5i32;
pub const WS_XML_TEXT_TYPE_INT64: i32 = 6i32;
pub const WS_XML_TEXT_TYPE_UINT64: i32 = 7i32;
pub const WS_XML_TEXT_TYPE_FLOAT: i32 = 8i32;
pub const WS_XML_TEXT_TYPE_DOUBLE: i32 = 9i32;
pub const WS_XML_TEXT_TYPE_DECIMAL: i32 = 10i32;
pub const WS_XML_TEXT_TYPE_GUID: i32 = 11i32;
pub const WS_XML_TEXT_TYPE_UNIQUE_ID: i32 = 12i32;
pub const WS_XML_TEXT_TYPE_DATETIME: i32 = 13i32;
pub const WS_XML_TEXT_TYPE_TIMESPAN: i32 = 14i32;
pub const WS_XML_TEXT_TYPE_QNAME: i32 = 15i32;
pub const WS_XML_TEXT_TYPE_LIST: i32 = 16i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_UNIQUE_ID_TEXT {
    pub text: WS_XML_TEXT,
    pub value: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WS_XML_UNIQUE_ID_TEXT {}
impl ::core::clone::Clone for WS_XML_UNIQUE_ID_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_WRITER(pub u8);
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_WRITER_BUFFER_OUTPUT {
    pub output: WS_XML_WRITER_OUTPUT,
}
impl ::core::marker::Copy for WS_XML_WRITER_BUFFER_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_BUFFER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WS_XML_WRITER_ENCODING {
    pub encodingType: WS_XML_WRITER_ENCODING_TYPE,
}
impl ::core::marker::Copy for WS_XML_WRITER_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_XML_WRITER_ENCODING_TYPE_TEXT: i32 = 1i32;
pub const WS_XML_WRITER_ENCODING_TYPE_BINARY: i32 = 2i32;
pub const WS_XML_WRITER_ENCODING_TYPE_MTOM: i32 = 3i32;
pub const WS_XML_WRITER_ENCODING_TYPE_RAW: i32 = 4i32;
#[repr(C)]
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
#[repr(C)]
pub struct WS_XML_WRITER_OUTPUT {
    pub outputType: WS_XML_WRITER_OUTPUT_TYPE,
}
impl ::core::marker::Copy for WS_XML_WRITER_OUTPUT {}
impl ::core::clone::Clone for WS_XML_WRITER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WS_XML_WRITER_OUTPUT_TYPE_BUFFER: i32 = 1i32;
pub const WS_XML_WRITER_OUTPUT_TYPE_STREAM: i32 = 2i32;
#[repr(C)]
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
#[repr(C)]
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
pub const WS_XML_WRITER_PROPERTY_MAX_DEPTH: i32 = 0i32;
pub const WS_XML_WRITER_PROPERTY_ALLOW_FRAGMENT: i32 = 1i32;
pub const WS_XML_WRITER_PROPERTY_MAX_ATTRIBUTES: i32 = 2i32;
pub const WS_XML_WRITER_PROPERTY_WRITE_DECLARATION: i32 = 3i32;
pub const WS_XML_WRITER_PROPERTY_INDENT: i32 = 4i32;
pub const WS_XML_WRITER_PROPERTY_BUFFER_TRIM_SIZE: i32 = 5i32;
pub const WS_XML_WRITER_PROPERTY_CHARSET: i32 = 6i32;
pub const WS_XML_WRITER_PROPERTY_BUFFERS: i32 = 7i32;
pub const WS_XML_WRITER_PROPERTY_BUFFER_MAX_SIZE: i32 = 8i32;
pub const WS_XML_WRITER_PROPERTY_BYTES: i32 = 9i32;
pub const WS_XML_WRITER_PROPERTY_IN_ATTRIBUTE: i32 = 10i32;
pub const WS_XML_WRITER_PROPERTY_MAX_MIME_PARTS_BUFFER_SIZE: i32 = 11i32;
pub const WS_XML_WRITER_PROPERTY_INITIAL_BUFFER: i32 = 12i32;
pub const WS_XML_WRITER_PROPERTY_ALLOW_INVALID_CHARACTER_REFERENCES: i32 = 13i32;
pub const WS_XML_WRITER_PROPERTY_MAX_NAMESPACES: i32 = 14i32;
pub const WS_XML_WRITER_PROPERTY_BYTES_WRITTEN: i32 = 15i32;
pub const WS_XML_WRITER_PROPERTY_BYTES_TO_CLOSE: i32 = 16i32;
pub const WS_XML_WRITER_PROPERTY_COMPRESS_EMPTY_ELEMENTS: i32 = 17i32;
pub const WS_XML_WRITER_PROPERTY_EMIT_UNCOMPRESSED_EMPTY_ELEMENTS: i32 = 18i32;
#[repr(C)]
pub struct WS_XML_WRITER_RAW_ENCODING {
    pub encoding: WS_XML_WRITER_ENCODING,
}
impl ::core::marker::Copy for WS_XML_WRITER_RAW_ENCODING {}
impl ::core::clone::Clone for WS_XML_WRITER_RAW_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
