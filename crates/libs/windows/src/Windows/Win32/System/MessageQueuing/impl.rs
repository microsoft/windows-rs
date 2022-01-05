#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplicationImpl: Sized + IDispatchImpl {
    fn MachineIdOfMachineName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplication2Impl: Sized + IMSMQApplicationImpl + IDispatchImpl {
    fn RegisterCertificate();
    fn MachineNameOfMachineId();
    fn MSMQVersionMajor();
    fn MSMQVersionMinor();
    fn MSMQVersionBuild();
    fn IsDsEnabled();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplication3Impl: Sized + IMSMQApplication2Impl + IMSMQApplicationImpl + IDispatchImpl {
    fn ActiveQueues();
    fn PrivateQueues();
    fn DirectoryServiceServer();
    fn IsConnected();
    fn BytesInAllQueues();
    fn SetMachine();
    fn Machine();
    fn Connect();
    fn Disconnect();
    fn Tidy();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQDestinationImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn IsOpen();
    fn IADs();
    fn putref_IADs();
    fn ADsPath();
    fn SetADsPath();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn Destinations();
    fn putref_Destinations();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEventImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent2Impl: Sized + IMSMQEventImpl + IDispatchImpl {
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent3Impl: Sized + IMSMQEvent2Impl + IMSMQEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQManagementImpl: Sized + IDispatchImpl {
    fn Init();
    fn FormatName();
    fn Machine();
    fn MessageCount();
    fn ForeignStatus();
    fn QueueType();
    fn IsLocal();
    fn TransactionalStatus();
    fn BytesInQueue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessageImpl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage2Impl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo_v1();
    fn putref_ResponseQueueInfo_v1();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo_v1();
    fn putref_AdminQueueInfo_v1();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
    fn SenderVersion();
    fn Extension();
    fn SetExtension();
    fn ConnectorTypeGuid();
    fn SetConnectorTypeGuid();
    fn TransactionStatusQueueInfo();
    fn DestinationSymmetricKey();
    fn SetDestinationSymmetricKey();
    fn Signature();
    fn SetSignature();
    fn AuthenticationProviderType();
    fn SetAuthenticationProviderType();
    fn AuthenticationProviderName();
    fn SetAuthenticationProviderName();
    fn SetSenderId();
    fn MsgClass();
    fn SetMsgClass();
    fn Properties();
    fn TransactionId();
    fn IsFirstInTransaction();
    fn IsLastInTransaction();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn ReceivedAuthenticationLevel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage3Impl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo_v1();
    fn putref_ResponseQueueInfo_v1();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo_v1();
    fn putref_AdminQueueInfo_v1();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
    fn SenderVersion();
    fn Extension();
    fn SetExtension();
    fn ConnectorTypeGuid();
    fn SetConnectorTypeGuid();
    fn TransactionStatusQueueInfo();
    fn DestinationSymmetricKey();
    fn SetDestinationSymmetricKey();
    fn Signature();
    fn SetSignature();
    fn AuthenticationProviderType();
    fn SetAuthenticationProviderType();
    fn AuthenticationProviderName();
    fn SetAuthenticationProviderName();
    fn SetSenderId();
    fn MsgClass();
    fn SetMsgClass();
    fn Properties();
    fn TransactionId();
    fn IsFirstInTransaction();
    fn IsLastInTransaction();
    fn ResponseQueueInfo_v2();
    fn putref_ResponseQueueInfo_v2();
    fn AdminQueueInfo_v2();
    fn putref_AdminQueueInfo_v2();
    fn ReceivedAuthenticationLevel();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn ResponseDestination();
    fn putref_ResponseDestination();
    fn Destination();
    fn LookupId();
    fn IsAuthenticated2();
    fn IsFirstInTransaction2();
    fn IsLastInTransaction2();
    fn AttachCurrentSecurityContext2();
    fn SoapEnvelope();
    fn CompoundMessage();
    fn SetSoapHeader();
    fn SetSoapBody();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage4Impl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo_v1();
    fn putref_ResponseQueueInfo_v1();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo_v1();
    fn putref_AdminQueueInfo_v1();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
    fn SenderVersion();
    fn Extension();
    fn SetExtension();
    fn ConnectorTypeGuid();
    fn SetConnectorTypeGuid();
    fn TransactionStatusQueueInfo();
    fn DestinationSymmetricKey();
    fn SetDestinationSymmetricKey();
    fn Signature();
    fn SetSignature();
    fn AuthenticationProviderType();
    fn SetAuthenticationProviderType();
    fn AuthenticationProviderName();
    fn SetAuthenticationProviderName();
    fn SetSenderId();
    fn MsgClass();
    fn SetMsgClass();
    fn Properties();
    fn TransactionId();
    fn IsFirstInTransaction();
    fn IsLastInTransaction();
    fn ResponseQueueInfo_v2();
    fn putref_ResponseQueueInfo_v2();
    fn AdminQueueInfo_v2();
    fn putref_AdminQueueInfo_v2();
    fn ReceivedAuthenticationLevel();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn ResponseDestination();
    fn putref_ResponseDestination();
    fn Destination();
    fn LookupId();
    fn IsAuthenticated2();
    fn IsFirstInTransaction2();
    fn IsLastInTransaction2();
    fn AttachCurrentSecurityContext2();
    fn SoapEnvelope();
    fn CompoundMessage();
    fn SetSoapHeader();
    fn SetSoapBody();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQOutgoingQueueManagementImpl: Sized + IMSMQManagementImpl + IDispatchImpl {
    fn State();
    fn NextHops();
    fn EodGetSendInfo();
    fn Resume();
    fn Pause();
    fn EodResend();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQPrivateDestinationImpl: Sized + IDispatchImpl {
    fn Handle();
    fn SetHandle();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQPrivateEventImpl: Sized + IDispatchImpl {
    fn Hwnd();
    fn FireArrivedEvent();
    fn FireArrivedErrorEvent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueryImpl: Sized + IDispatchImpl {
    fn LookupQueue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery2Impl: Sized + IDispatchImpl {
    fn LookupQueue();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery3Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2();
    fn Properties();
    fn LookupQueue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery4Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2();
    fn Properties();
    fn LookupQueue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueImpl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive();
    fn Peek();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue2Impl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive_v1();
    fn Peek_v1();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent_v1();
    fn PeekNext_v1();
    fn PeekCurrent_v1();
    fn Receive();
    fn Peek();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue3Impl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive_v1();
    fn Peek_v1();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent_v1();
    fn PeekNext_v1();
    fn PeekCurrent_v1();
    fn Receive();
    fn Peek();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
    fn Properties();
    fn Handle2();
    fn ReceiveByLookupId();
    fn ReceiveNextByLookupId();
    fn ReceivePreviousByLookupId();
    fn ReceiveFirstByLookupId();
    fn ReceiveLastByLookupId();
    fn PeekByLookupId();
    fn PeekNextByLookupId();
    fn PeekPreviousByLookupId();
    fn PeekFirstByLookupId();
    fn PeekLastByLookupId();
    fn Purge();
    fn IsOpen2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue4Impl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive_v1();
    fn Peek_v1();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent_v1();
    fn PeekNext_v1();
    fn PeekCurrent_v1();
    fn Receive();
    fn Peek();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
    fn Properties();
    fn Handle2();
    fn ReceiveByLookupId();
    fn ReceiveNextByLookupId();
    fn ReceivePreviousByLookupId();
    fn ReceiveFirstByLookupId();
    fn ReceiveLastByLookupId();
    fn PeekByLookupId();
    fn PeekNextByLookupId();
    fn PeekPreviousByLookupId();
    fn PeekFirstByLookupId();
    fn PeekLastByLookupId();
    fn Purge();
    fn IsOpen2();
    fn ReceiveByLookupIdAllowPeek();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfoImpl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo2Impl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
    fn PathNameDNS();
    fn Properties();
    fn Security();
    fn SetSecurity();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo3Impl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
    fn PathNameDNS();
    fn Properties();
    fn Security();
    fn SetSecurity();
    fn IsTransactional2();
    fn IsWorldReadable2();
    fn MulticastAddress();
    fn SetMulticastAddress();
    fn ADsPath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo4Impl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
    fn PathNameDNS();
    fn Properties();
    fn Security();
    fn SetSecurity();
    fn IsTransactional2();
    fn IsWorldReadable2();
    fn MulticastAddress();
    fn SetMulticastAddress();
    fn ADsPath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfosImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos2Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos3Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos4Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueManagementImpl: Sized + IMSMQManagementImpl + IDispatchImpl {
    fn JournalMessageCount();
    fn BytesInJournal();
    fn EodGetReceiveInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionImpl: Sized + IDispatchImpl {
    fn Transaction();
    fn Commit();
    fn Abort();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction2Impl: Sized + IMSMQTransactionImpl + IDispatchImpl {
    fn InitNew();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction3Impl: Sized + IMSMQTransaction2Impl + IMSMQTransactionImpl + IDispatchImpl {
    fn ITransaction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _DMSMQEventEventsImpl: Sized + IDispatchImpl {}
