#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNAuthenticatorGetAssertion();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNAuthenticatorMakeCredential();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNCancelCurrentOperation();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNFreeAssertion();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNFreeCredentialAttestation();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNGetApiVersionNumber();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNGetCancellationId();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNGetErrorName();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WebAuthNGetW3CExceptionDOMError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbandonCall();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbandonMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortServiceHost();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAbortServiceProxy();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAcceptChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddCustomHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddErrorString();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddMappedHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsAddressMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAlloc();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsAsyncExecute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCall();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCheckMustUnderstandHeaders();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseServiceHost();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCloseServiceProxy();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCombineUrl();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCopyError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCopyNode();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateChannelForListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateFaultFromError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateHeap();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateMessageForChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateMetadata();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateReader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateServiceEndpointFromTemplate();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsCreateServiceHost();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateServiceProxy();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateServiceProxyFromTemplate();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateWriter();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateXmlBuffer();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsCreateXmlSecurityToken();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsDateTimeToFileTime();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsDecodeUrl();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsEncodeUrl();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsEndReaderCanonicalization();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsEndWriterCanonicalization();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsFileTimeToDateTime();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFillBody();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFillReader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsFindAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFlushBody();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFlushWriter();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeHeap();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeMetadata();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeReader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeSecurityToken();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeServiceHost();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeServiceProxy();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsFreeWriter();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetChannelProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetCustomHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetDictionary();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetErrorProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetErrorString();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetFaultErrorDetail();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetFaultErrorProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetHeaderAttributes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetHeapProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetListenerProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMappedHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetMessageProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMetadataEndpoints();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetMetadataProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetMissingMetadataDocumentAddress();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetNamespaceFromPrefix();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetOperationContextProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetPolicyAlternativeCount();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetPolicyProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetPrefixFromNamespace();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetReaderNode();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetReaderPosition();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetReaderProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetSecurityContextProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetSecurityTokenProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetServiceHostProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetServiceProxyProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetWriterPosition();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsGetWriterProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsGetXmlAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsInitializeMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsMarkHeaderAsUnderstood();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMatchPolicyAlternative();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMoveReader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsMoveWriter();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsOpenServiceHost();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsOpenServiceProxy();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsPullBytes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsPushBytes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadArray();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadBody();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadBytes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadChars();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadCharsUtf8();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEndAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEndElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadEndpointAddressExtension();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEnvelopeEnd();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadEnvelopeStart();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadMessageEnd();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadMessageStart();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadMetadata();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadNode();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadQualifiedName();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadStartAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadStartElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReadToStartElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadType();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadValue();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadXmlBuffer();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsReadXmlBufferFromBytes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsReceiveMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRegisterOperationForCancel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRemoveCustomHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRemoveHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRemoveMappedHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRemoveNode();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsRequestReply();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRequestSecurityToken();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetHeap();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetListener();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetMetadata();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetServiceHost();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsResetServiceProxy();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsRevokeSecurityContext();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSendFaultMessageForError();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSendMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSendReplyMessage();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetChannelProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetErrorProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsSetFaultErrorDetail();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetFaultErrorProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetHeader();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetInput();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetInputToBuffer();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetListenerProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetMessageProperty();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetOutput();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetOutputToBuffer();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetReaderPosition();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSetWriterPosition();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsShutdownSessionChannel();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsSkipNode();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsStartReaderCanonicalization();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsStartWriterCanonicalization();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsTrimXmlWhitespace();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsVerifyXmlNCName();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteArray();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteBody();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteBytes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteChars();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteCharsUtf8();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndCData();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEndStartElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEnvelopeEnd();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteEnvelopeStart();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteMessageEnd();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteMessageStart();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteNode();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteQualifiedName();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteStartAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteStartCData();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteStartElement();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteText();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteType();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteValue();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteXmlBuffer();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`*"]
    pub fn WsWriteXmlBufferToBytes();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsWriteXmlnsAttribute();
    #[doc = "*Required features: `Win32_Networking_WindowsWebServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WsXmlStringEquals();
}
