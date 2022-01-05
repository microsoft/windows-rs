#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IControlChannelTriggerImpl: Sized + IClosableImpl {
    fn ControlChannelTriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32>;
    fn SetServerKeepAliveIntervalInMinutes(&self, value: u32) -> ::windows::core::Result<()>;
    fn CurrentKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32>;
    fn TransportObject(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn KeepAliveTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn PushNotificationTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn UsingTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn WaitForPushEnabled(&self) -> ::windows::core::Result<ControlChannelTriggerStatus>;
    fn DecreaseNetworkKeepAliveInterval(&self) -> ::windows::core::Result<()>;
    fn FlushTransport(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlChannelTrigger2Impl: Sized {
    fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool>;
}
pub trait IControlChannelTriggerEventDetailsImpl: Sized {
    fn ControlChannelTrigger(&self) -> ::windows::core::Result<ControlChannelTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlChannelTriggerFactoryImpl: Sized {
    fn CreateControlChannelTrigger(&self, channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32) -> ::windows::core::Result<ControlChannelTrigger>;
    fn CreateControlChannelTriggerEx(&self, channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType) -> ::windows::core::Result<ControlChannelTrigger>;
}
pub trait IControlChannelTriggerResetEventDetailsImpl: Sized {
    fn ResetReason(&self) -> ::windows::core::Result<ControlChannelTriggerResetReason>;
    fn HardwareSlotReset(&self) -> ::windows::core::Result<bool>;
    fn SoftwareSlotReset(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDatagramSocketImpl: Sized + IClosableImpl {
    fn Control(&self) -> ::windows::core::Result<DatagramSocketControl>;
    fn Information(&self) -> ::windows::core::Result<DatagramSocketInformation>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithEndpointPairAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindServiceNameAsync(&self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindEndpointAsync(&self, localhostname: &::core::option::Option<super::HostName>, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn JoinMulticastGroup(&self, host: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn GetOutputStreamAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>;
    fn GetOutputStreamWithEndpointPairAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>;
    fn MessageReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDatagramSocket2Impl: Sized + IClosableImpl {
    fn BindServiceNameAndAdapterAsync(&self, localservicename: &::windows::core::HSTRING, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocket3Impl: Sized {
    fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>, keepalivetime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControlImpl: Sized {
    fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl2Impl: Sized {
    fn InboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetInboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn DontFragment(&self) -> ::windows::core::Result<bool>;
    fn SetDontFragment(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl3Impl: Sized {
    fn MulticastOnly(&self) -> ::windows::core::Result<bool>;
    fn SetMulticastOnly(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketInformationImpl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketMessageReceivedEventArgsImpl: Sized {
    fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader>;
    fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketStaticsImpl: Sized {
    fn GetEndpointPairsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
    fn GetEndpointPairsWithSortOptionsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMessageWebSocketImpl: Sized + IClosableImpl + IWebSocketImpl {
    fn Control(&self) -> ::windows::core::Result<MessageWebSocketControl>;
    fn Information(&self) -> ::windows::core::Result<MessageWebSocketInformation>;
    fn MessageReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMessageWebSocket2Impl: Sized + IClosableImpl + IMessageWebSocketImpl + IWebSocketImpl {
    fn ServerCustomValidationRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessageWebSocket3Impl: Sized {
    fn SendNonfinalFrameAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn SendFinalFrameAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessageWebSocketControlImpl: Sized + IWebSocketControlImpl {
    fn MaxMessageSize(&self) -> ::windows::core::Result<u32>;
    fn SetMaxMessageSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn MessageType(&self) -> ::windows::core::Result<SocketMessageType>;
    fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessageWebSocketControl2Impl: Sized {
    fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDesiredUnsolicitedPongInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ReceiveMode(&self) -> ::windows::core::Result<MessageWebSocketReceiveMode>;
    fn SetReceiveMode(&self, value: MessageWebSocketReceiveMode) -> ::windows::core::Result<()>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessageWebSocketMessageReceivedEventArgsImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SocketMessageType>;
    fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader>;
    fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessageWebSocketMessageReceivedEventArgs2Impl: Sized + IMessageWebSocketMessageReceivedEventArgsImpl {
    fn IsMessageComplete(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IServerMessageWebSocketImpl: Sized + IClosableImpl {
    fn MessageReceived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Control(&self) -> ::windows::core::Result<ServerMessageWebSocketControl>;
    fn Information(&self) -> ::windows::core::Result<ServerMessageWebSocketInformation>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn Closed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerMessageWebSocketControlImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SocketMessageType>;
    fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerMessageWebSocketInformationImpl: Sized {
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IServerStreamWebSocketImpl: Sized + IClosableImpl {
    fn Information(&self) -> ::windows::core::Result<ServerStreamWebSocketInformation>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn Closed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerStreamWebSocketInformationImpl: Sized {
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityContextImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityContextFactoryImpl: Sized {
    fn Create(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<SocketActivityContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityInformationImpl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SocketKind(&self) -> ::windows::core::Result<SocketActivityKind>;
    fn Context(&self) -> ::windows::core::Result<SocketActivityContext>;
    fn DatagramSocket(&self) -> ::windows::core::Result<DatagramSocket>;
    fn StreamSocket(&self) -> ::windows::core::Result<StreamSocket>;
    fn StreamSocketListener(&self) -> ::windows::core::Result<StreamSocketListener>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityInformationStaticsImpl: Sized {
    fn AllSockets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityTriggerDetailsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<SocketActivityTriggerReason>;
    fn SocketInformation(&self) -> ::windows::core::Result<SocketActivityInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<SocketErrorStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketImpl: Sized + IClosableImpl {
    fn Control(&self) -> ::windows::core::Result<StreamSocketControl>;
    fn Information(&self) -> ::windows::core::Result<StreamSocketInformation>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectWithEndpointPairAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithEndpointPairAndProtectionLevelAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithProtectionLevelAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpgradeToSslAsync(&self, protectionlevel: SocketProtectionLevel, validationhostname: &::core::option::Option<super::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocket2Impl: Sized + IClosableImpl {
    fn ConnectWithProtectionLevelAndAdapterAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocket3Impl: Sized {
    fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>, keepalivetime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControlImpl: Sized {
    fn NoDelay(&self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepAlive(&self) -> ::windows::core::Result<bool>;
    fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControl2Impl: Sized {
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControl3Impl: Sized {
    fn SerializeConnectionAttempts(&self) -> ::windows::core::Result<bool>;
    fn SetSerializeConnectionAttempts(&self, value: bool) -> ::windows::core::Result<()>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControl4Impl: Sized {
    fn MinProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel>;
    fn SetMinProtectionLevel(&self, value: SocketProtectionLevel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketInformationImpl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName>;
    fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoundTripTimeStatistics(&self) -> ::windows::core::Result<RoundTripTimeStatistics>;
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel>;
    fn SessionKey(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketInformation2Impl: Sized {
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketListenerImpl: Sized + IClosableImpl {
    fn Control(&self) -> ::windows::core::Result<StreamSocketListenerControl>;
    fn Information(&self) -> ::windows::core::Result<StreamSocketListenerInformation>;
    fn BindServiceNameAsync(&self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindEndpointAsync(&self, localhostname: &::core::option::Option<super::HostName>, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectionReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketListener2Impl: Sized + IClosableImpl {
    fn BindServiceNameWithProtectionLevelAsync(&self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindServiceNameWithProtectionLevelAndAdapterAsync(&self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListener3Impl: Sized {
    fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerConnectionReceivedEventArgsImpl: Sized {
    fn Socket(&self) -> ::windows::core::Result<StreamSocket>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerControlImpl: Sized {
    fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerControl2Impl: Sized {
    fn NoDelay(&self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepAlive(&self) -> ::windows::core::Result<bool>;
    fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerInformationImpl: Sized {
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketStaticsImpl: Sized {
    fn GetEndpointPairsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
    fn GetEndpointPairsWithSortOptionsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamWebSocketImpl: Sized + IClosableImpl + IWebSocketImpl {
    fn Control(&self) -> ::windows::core::Result<StreamWebSocketControl>;
    fn Information(&self) -> ::windows::core::Result<StreamWebSocketInformation>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamWebSocket2Impl: Sized + IClosableImpl + IStreamWebSocketImpl + IWebSocketImpl {
    fn ServerCustomValidationRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamWebSocketControlImpl: Sized + IWebSocketControlImpl {
    fn NoDelay(&self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamWebSocketControl2Impl: Sized {
    fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDesiredUnsolicitedPongInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IWebSocketImpl: Sized + IClosableImpl {
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Closed(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebSocketClosedEventArgsImpl: Sized {
    fn Code(&self) -> ::windows::core::Result<u16>;
    fn Reason(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IWebSocketControlImpl: Sized {
    fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
pub trait IWebSocketControl2Impl: Sized + IWebSocketControlImpl {
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebSocketErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus>;
}
pub trait IWebSocketInformationImpl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IWebSocketInformation2Impl: Sized + IWebSocketInformationImpl {
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebSocketServerCustomValidationRequestedEventArgsImpl: Sized {
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
