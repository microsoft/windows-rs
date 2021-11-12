#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BandwidthStatistics(i32);
#[repr(transparent)]
pub struct ControlChannelTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ControlChannelTriggerContract(i32);
#[repr(transparent)]
pub struct ControlChannelTriggerResetReason(pub i32);
impl ControlChannelTriggerResetReason {
    pub const FastUserSwitched: ControlChannelTriggerResetReason = ControlChannelTriggerResetReason(0i32);
    pub const LowPowerExit: ControlChannelTriggerResetReason = ControlChannelTriggerResetReason(1i32);
    pub const QuietHoursExit: ControlChannelTriggerResetReason = ControlChannelTriggerResetReason(2i32);
    pub const ApplicationRestart: ControlChannelTriggerResetReason = ControlChannelTriggerResetReason(3i32);
}
#[repr(transparent)]
pub struct ControlChannelTriggerResourceType(pub i32);
impl ControlChannelTriggerResourceType {
    pub const RequestSoftwareSlot: ControlChannelTriggerResourceType = ControlChannelTriggerResourceType(0i32);
    pub const RequestHardwareSlot: ControlChannelTriggerResourceType = ControlChannelTriggerResourceType(1i32);
}
#[repr(transparent)]
pub struct ControlChannelTriggerStatus(pub i32);
impl ControlChannelTriggerStatus {
    pub const HardwareSlotRequested: ControlChannelTriggerStatus = ControlChannelTriggerStatus(0i32);
    pub const SoftwareSlotAllocated: ControlChannelTriggerStatus = ControlChannelTriggerStatus(1i32);
    pub const HardwareSlotAllocated: ControlChannelTriggerStatus = ControlChannelTriggerStatus(2i32);
    pub const PolicyError: ControlChannelTriggerStatus = ControlChannelTriggerStatus(3i32);
    pub const SystemError: ControlChannelTriggerStatus = ControlChannelTriggerStatus(4i32);
    pub const TransportDisconnected: ControlChannelTriggerStatus = ControlChannelTriggerStatus(5i32);
    pub const ServiceUnavailable: ControlChannelTriggerStatus = ControlChannelTriggerStatus(6i32);
}
#[repr(transparent)]
pub struct DatagramSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatagramSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatagramSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatagramSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlChannelTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlChannelTrigger2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlChannelTriggerEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlChannelTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlChannelTriggerResetEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocket2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocket3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocketControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocketControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatagramSocketStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocket2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocket3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocketControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServerMessageWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServerMessageWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServerMessageWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServerStreamWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServerStreamWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketActivityContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketActivityContextFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketActivityInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketActivityInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketActivityTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketErrorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocket2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocket3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketControl4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListener2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListener3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListenerControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListenerControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketListenerInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSocketStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamWebSocket2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamWebSocketControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketErrorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageWebSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageWebSocketReceiveMode(pub i32);
impl MessageWebSocketReceiveMode {
    pub const FullMessage: MessageWebSocketReceiveMode = MessageWebSocketReceiveMode(0i32);
    pub const PartialMessage: MessageWebSocketReceiveMode = MessageWebSocketReceiveMode(1i32);
}
#[repr(C)]
pub struct RoundTripTimeStatistics(i32);
#[repr(transparent)]
pub struct ServerMessageWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServerMessageWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServerMessageWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServerStreamWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServerStreamWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocketActivityConnectedStandbyAction(pub i32);
impl SocketActivityConnectedStandbyAction {
    pub const DoNotWake: SocketActivityConnectedStandbyAction = SocketActivityConnectedStandbyAction(0i32);
    pub const Wake: SocketActivityConnectedStandbyAction = SocketActivityConnectedStandbyAction(1i32);
}
#[repr(transparent)]
pub struct SocketActivityContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocketActivityInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocketActivityKind(pub i32);
impl SocketActivityKind {
    pub const None: SocketActivityKind = SocketActivityKind(0i32);
    pub const StreamSocketListener: SocketActivityKind = SocketActivityKind(1i32);
    pub const DatagramSocket: SocketActivityKind = SocketActivityKind(2i32);
    pub const StreamSocket: SocketActivityKind = SocketActivityKind(3i32);
}
#[repr(transparent)]
pub struct SocketActivityTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocketActivityTriggerReason(pub i32);
impl SocketActivityTriggerReason {
    pub const None: SocketActivityTriggerReason = SocketActivityTriggerReason(0i32);
    pub const SocketActivity: SocketActivityTriggerReason = SocketActivityTriggerReason(1i32);
    pub const ConnectionAccepted: SocketActivityTriggerReason = SocketActivityTriggerReason(2i32);
    pub const KeepAliveTimerExpired: SocketActivityTriggerReason = SocketActivityTriggerReason(3i32);
    pub const SocketClosed: SocketActivityTriggerReason = SocketActivityTriggerReason(4i32);
}
#[repr(transparent)]
pub struct SocketErrorStatus(pub i32);
impl SocketErrorStatus {
    pub const Unknown: SocketErrorStatus = SocketErrorStatus(0i32);
    pub const OperationAborted: SocketErrorStatus = SocketErrorStatus(1i32);
    pub const HttpInvalidServerResponse: SocketErrorStatus = SocketErrorStatus(2i32);
    pub const ConnectionTimedOut: SocketErrorStatus = SocketErrorStatus(3i32);
    pub const AddressFamilyNotSupported: SocketErrorStatus = SocketErrorStatus(4i32);
    pub const SocketTypeNotSupported: SocketErrorStatus = SocketErrorStatus(5i32);
    pub const HostNotFound: SocketErrorStatus = SocketErrorStatus(6i32);
    pub const NoDataRecordOfRequestedType: SocketErrorStatus = SocketErrorStatus(7i32);
    pub const NonAuthoritativeHostNotFound: SocketErrorStatus = SocketErrorStatus(8i32);
    pub const ClassTypeNotFound: SocketErrorStatus = SocketErrorStatus(9i32);
    pub const AddressAlreadyInUse: SocketErrorStatus = SocketErrorStatus(10i32);
    pub const CannotAssignRequestedAddress: SocketErrorStatus = SocketErrorStatus(11i32);
    pub const ConnectionRefused: SocketErrorStatus = SocketErrorStatus(12i32);
    pub const NetworkIsUnreachable: SocketErrorStatus = SocketErrorStatus(13i32);
    pub const UnreachableHost: SocketErrorStatus = SocketErrorStatus(14i32);
    pub const NetworkIsDown: SocketErrorStatus = SocketErrorStatus(15i32);
    pub const NetworkDroppedConnectionOnReset: SocketErrorStatus = SocketErrorStatus(16i32);
    pub const SoftwareCausedConnectionAbort: SocketErrorStatus = SocketErrorStatus(17i32);
    pub const ConnectionResetByPeer: SocketErrorStatus = SocketErrorStatus(18i32);
    pub const HostIsDown: SocketErrorStatus = SocketErrorStatus(19i32);
    pub const NoAddressesFound: SocketErrorStatus = SocketErrorStatus(20i32);
    pub const TooManyOpenFiles: SocketErrorStatus = SocketErrorStatus(21i32);
    pub const MessageTooLong: SocketErrorStatus = SocketErrorStatus(22i32);
    pub const CertificateExpired: SocketErrorStatus = SocketErrorStatus(23i32);
    pub const CertificateUntrustedRoot: SocketErrorStatus = SocketErrorStatus(24i32);
    pub const CertificateCommonNameIsIncorrect: SocketErrorStatus = SocketErrorStatus(25i32);
    pub const CertificateWrongUsage: SocketErrorStatus = SocketErrorStatus(26i32);
    pub const CertificateRevoked: SocketErrorStatus = SocketErrorStatus(27i32);
    pub const CertificateNoRevocationCheck: SocketErrorStatus = SocketErrorStatus(28i32);
    pub const CertificateRevocationServerOffline: SocketErrorStatus = SocketErrorStatus(29i32);
    pub const CertificateIsInvalid: SocketErrorStatus = SocketErrorStatus(30i32);
}
#[repr(transparent)]
pub struct SocketMessageType(pub i32);
impl SocketMessageType {
    pub const Binary: SocketMessageType = SocketMessageType(0i32);
    pub const Utf8: SocketMessageType = SocketMessageType(1i32);
}
#[repr(transparent)]
pub struct SocketProtectionLevel(pub i32);
impl SocketProtectionLevel {
    pub const PlainSocket: SocketProtectionLevel = SocketProtectionLevel(0i32);
    pub const Ssl: SocketProtectionLevel = SocketProtectionLevel(1i32);
    pub const SslAllowNullEncryption: SocketProtectionLevel = SocketProtectionLevel(2i32);
    pub const BluetoothEncryptionAllowNullAuthentication: SocketProtectionLevel = SocketProtectionLevel(3i32);
    pub const BluetoothEncryptionWithAuthentication: SocketProtectionLevel = SocketProtectionLevel(4i32);
    pub const Ssl3AllowWeakEncryption: SocketProtectionLevel = SocketProtectionLevel(5i32);
    pub const Tls10: SocketProtectionLevel = SocketProtectionLevel(6i32);
    pub const Tls11: SocketProtectionLevel = SocketProtectionLevel(7i32);
    pub const Tls12: SocketProtectionLevel = SocketProtectionLevel(8i32);
    pub const Unspecified: SocketProtectionLevel = SocketProtectionLevel(9i32);
}
#[repr(transparent)]
pub struct SocketQualityOfService(pub i32);
impl SocketQualityOfService {
    pub const Normal: SocketQualityOfService = SocketQualityOfService(0i32);
    pub const LowLatency: SocketQualityOfService = SocketQualityOfService(1i32);
}
#[repr(transparent)]
pub struct SocketSslErrorSeverity(pub i32);
impl SocketSslErrorSeverity {
    pub const None: SocketSslErrorSeverity = SocketSslErrorSeverity(0i32);
    pub const Ignorable: SocketSslErrorSeverity = SocketSslErrorSeverity(1i32);
    pub const Fatal: SocketSslErrorSeverity = SocketSslErrorSeverity(2i32);
}
#[repr(transparent)]
pub struct StreamSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamSocketListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamSocketListenerConnectionReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamSocketListenerControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamSocketListenerInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamWebSocket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamWebSocketControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamWebSocketInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebSocketClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebSocketKeepAlive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebSocketServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
