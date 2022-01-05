pub trait IWSDAddressImpl: Sized {
    fn Serialize();
    fn Deserialize();
}
pub trait IWSDAsyncCallbackImpl: Sized {
    fn AsyncOperationComplete();
}
pub trait IWSDAsyncResultImpl: Sized {
    fn SetCallback();
    fn SetWaitHandle();
    fn HasCompleted();
    fn GetAsyncState();
    fn Abort();
    fn GetEvent();
    fn GetEndpointProxy();
}
pub trait IWSDAttachmentImpl: Sized {}
pub trait IWSDDeviceHostImpl: Sized {
    fn Init();
    fn Start();
    fn Stop();
    fn Terminate();
    fn RegisterPortType();
    fn SetMetadata();
    fn RegisterService();
    fn RetireService();
    fn AddDynamicService();
    fn RemoveDynamicService();
    fn SetServiceDiscoverable();
    fn SignalEvent();
}
pub trait IWSDDeviceHostNotifyImpl: Sized {
    fn GetService();
}
pub trait IWSDDeviceProxyImpl: Sized {
    fn Init();
    fn BeginGetMetadata();
    fn EndGetMetadata();
    fn GetHostMetadata();
    fn GetThisModelMetadata();
    fn GetThisDeviceMetadata();
    fn GetAllMetadata();
    fn GetServiceProxyById();
    fn GetServiceProxyByType();
    fn GetEndpointProxy();
}
pub trait IWSDEndpointProxyImpl: Sized {
    fn SendOneWayRequest();
    fn SendTwoWayRequest();
    fn SendTwoWayRequestAsync();
    fn AbortAsyncOperation();
    fn ProcessFault();
    fn GetErrorInfo();
    fn GetFaultInfo();
}
pub trait IWSDEventingStatusImpl: Sized {
    fn SubscriptionRenewed();
    fn SubscriptionRenewalFailed();
    fn SubscriptionEnded();
}
pub trait IWSDHttpAddressImpl: Sized + IWSDTransportAddressImpl + IWSDAddressImpl {
    fn GetSecure();
    fn SetSecure();
    fn GetPath();
    fn SetPath();
}
pub trait IWSDHttpAuthParametersImpl: Sized {
    fn GetClientAccessToken();
    fn GetAuthType();
}
pub trait IWSDHttpMessageParametersImpl: Sized + IWSDMessageParametersImpl {
    fn SetInboundHttpHeaders();
    fn GetInboundHttpHeaders();
    fn SetOutboundHttpHeaders();
    fn GetOutboundHttpHeaders();
    fn SetID();
    fn GetID();
    fn SetContext();
    fn GetContext();
    fn Clear();
}
pub trait IWSDInboundAttachmentImpl: Sized + IWSDAttachmentImpl {
    fn Read();
    fn Close();
}
pub trait IWSDMessageParametersImpl: Sized {
    fn GetLocalAddress();
    fn SetLocalAddress();
    fn GetRemoteAddress();
    fn SetRemoteAddress();
    fn GetLowerParameters();
}
pub trait IWSDMetadataExchangeImpl: Sized {
    fn GetMetadata();
}
pub trait IWSDOutboundAttachmentImpl: Sized + IWSDAttachmentImpl {
    fn Write();
    fn Close();
    fn Abort();
}
pub trait IWSDSSLClientCertificateImpl: Sized {
    fn GetClientCertificate();
    fn GetMappedAccessToken();
}
pub trait IWSDScopeMatchingRuleImpl: Sized {
    fn GetScopeRule();
    fn MatchScopes();
}
pub trait IWSDServiceMessagingImpl: Sized {
    fn SendResponse();
    fn FaultRequest();
}
pub trait IWSDServiceProxyImpl: Sized + IWSDMetadataExchangeImpl {
    fn BeginGetMetadata();
    fn EndGetMetadata();
    fn GetServiceMetadata();
    fn SubscribeToOperation();
    fn UnsubscribeToOperation();
    fn SetEventingStatusCallback();
    fn GetEndpointProxy();
}
pub trait IWSDServiceProxyEventingImpl: Sized + IWSDServiceProxyImpl + IWSDMetadataExchangeImpl {
    fn SubscribeToMultipleOperations();
    fn BeginSubscribeToMultipleOperations();
    fn EndSubscribeToMultipleOperations();
    fn UnsubscribeToMultipleOperations();
    fn BeginUnsubscribeToMultipleOperations();
    fn EndUnsubscribeToMultipleOperations();
    fn RenewMultipleOperations();
    fn BeginRenewMultipleOperations();
    fn EndRenewMultipleOperations();
    fn GetStatusForMultipleOperations();
    fn BeginGetStatusForMultipleOperations();
    fn EndGetStatusForMultipleOperations();
}
pub trait IWSDSignaturePropertyImpl: Sized {
    fn IsMessageSigned();
    fn IsMessageSignatureTrusted();
    fn GetKeyInfo();
    fn GetSignature();
    fn GetSignedInfoHash();
}
pub trait IWSDTransportAddressImpl: Sized + IWSDAddressImpl {
    fn GetPort();
    fn SetPort();
    fn GetTransportAddress();
    fn GetTransportAddressEx();
    fn SetTransportAddress();
}
pub trait IWSDUdpAddressImpl: Sized + IWSDTransportAddressImpl + IWSDAddressImpl {
    fn SetSockaddr();
    fn GetSockaddr();
    fn SetExclusive();
    fn GetExclusive();
    fn SetMessageType();
    fn GetMessageType();
    fn SetTTL();
    fn GetTTL();
    fn SetAlias();
    fn GetAlias();
}
pub trait IWSDUdpMessageParametersImpl: Sized + IWSDMessageParametersImpl {
    fn SetRetransmitParams();
    fn GetRetransmitParams();
}
pub trait IWSDXMLContextImpl: Sized {
    fn AddNamespace();
    fn AddNameToNamespace();
    fn SetNamespaces();
    fn SetTypes();
}
pub trait IWSDiscoveredServiceImpl: Sized {
    fn GetEndpointReference();
    fn GetTypes();
    fn GetScopes();
    fn GetXAddrs();
    fn GetMetadataVersion();
    fn GetExtendedDiscoXML();
    fn GetProbeResolveTag();
    fn GetRemoteTransportAddress();
    fn GetLocalTransportAddress();
    fn GetLocalInterfaceGUID();
    fn GetInstanceId();
}
pub trait IWSDiscoveryProviderImpl: Sized {
    fn SetAddressFamily();
    fn Attach();
    fn Detach();
    fn SearchById();
    fn SearchByAddress();
    fn SearchByType();
    fn GetXMLContext();
}
pub trait IWSDiscoveryProviderNotifyImpl: Sized {
    fn Add();
    fn Remove();
    fn SearchFailed();
    fn SearchComplete();
}
pub trait IWSDiscoveryPublisherImpl: Sized {
    fn SetAddressFamily();
    fn RegisterNotificationSink();
    fn UnRegisterNotificationSink();
    fn Publish();
    fn UnPublish();
    fn MatchProbe();
    fn MatchResolve();
    fn PublishEx();
    fn MatchProbeEx();
    fn MatchResolveEx();
    fn RegisterScopeMatchingRule();
    fn UnRegisterScopeMatchingRule();
    fn GetXMLContext();
}
pub trait IWSDiscoveryPublisherNotifyImpl: Sized {
    fn ProbeHandler();
    fn ResolveHandler();
}
