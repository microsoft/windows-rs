#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BandwidthStatistics {
    pub OutboundBitsPerSecond: u64,
    pub InboundBitsPerSecond: u64,
    pub OutboundBitsPerSecondInstability: u64,
    pub InboundBitsPerSecondInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
impl ::core::marker::Copy for BandwidthStatistics {}
impl ::core::clone::Clone for BandwidthStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ControlChannelTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ControlChannelTrigger {}
impl ::core::clone::Clone for ControlChannelTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ControlChannelTriggerResetReason(pub i32);
impl ControlChannelTriggerResetReason {
    pub const FastUserSwitched: Self = Self(0i32);
    pub const LowPowerExit: Self = Self(1i32);
    pub const QuietHoursExit: Self = Self(2i32);
    pub const ApplicationRestart: Self = Self(3i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResetReason {}
impl ::core::clone::Clone for ControlChannelTriggerResetReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ControlChannelTriggerResourceType(pub i32);
impl ControlChannelTriggerResourceType {
    pub const RequestSoftwareSlot: Self = Self(0i32);
    pub const RequestHardwareSlot: Self = Self(1i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResourceType {}
impl ::core::clone::Clone for ControlChannelTriggerResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ControlChannelTriggerStatus(pub i32);
impl ControlChannelTriggerStatus {
    pub const HardwareSlotRequested: Self = Self(0i32);
    pub const SoftwareSlotAllocated: Self = Self(1i32);
    pub const HardwareSlotAllocated: Self = Self(2i32);
    pub const PolicyError: Self = Self(3i32);
    pub const SystemError: Self = Self(4i32);
    pub const TransportDisconnected: Self = Self(5i32);
    pub const ServiceUnavailable: Self = Self(6i32);
}
impl ::core::marker::Copy for ControlChannelTriggerStatus {}
impl ::core::clone::Clone for ControlChannelTriggerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatagramSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatagramSocket {}
impl ::core::clone::Clone for DatagramSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatagramSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatagramSocketControl {}
impl ::core::clone::Clone for DatagramSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatagramSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatagramSocketInformation {}
impl ::core::clone::Clone for DatagramSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatagramSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatagramSocketMessageReceivedEventArgs {}
impl ::core::clone::Clone for DatagramSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlChannelTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlChannelTrigger {}
impl ::core::clone::Clone for IControlChannelTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlChannelTrigger2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlChannelTrigger2 {}
impl ::core::clone::Clone for IControlChannelTrigger2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlChannelTriggerEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlChannelTriggerEventDetails {}
impl ::core::clone::Clone for IControlChannelTriggerEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlChannelTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlChannelTriggerFactory {}
impl ::core::clone::Clone for IControlChannelTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlChannelTriggerResetEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlChannelTriggerResetEventDetails {}
impl ::core::clone::Clone for IControlChannelTriggerResetEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocket {}
impl ::core::clone::Clone for IDatagramSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocket2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocket2 {}
impl ::core::clone::Clone for IDatagramSocket2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocket3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocket3 {}
impl ::core::clone::Clone for IDatagramSocket3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocketControl {}
impl ::core::clone::Clone for IDatagramSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocketControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocketControl2 {}
impl ::core::clone::Clone for IDatagramSocketControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocketControl3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocketControl3 {}
impl ::core::clone::Clone for IDatagramSocketControl3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocketInformation {}
impl ::core::clone::Clone for IDatagramSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocketMessageReceivedEventArgs {}
impl ::core::clone::Clone for IDatagramSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatagramSocketStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatagramSocketStatics {}
impl ::core::clone::Clone for IDatagramSocketStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocket {}
impl ::core::clone::Clone for IMessageWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocket2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocket2 {}
impl ::core::clone::Clone for IMessageWebSocket2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocket3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocket3 {}
impl ::core::clone::Clone for IMessageWebSocket3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocketControl {}
impl ::core::clone::Clone for IMessageWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocketControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocketControl2 {}
impl ::core::clone::Clone for IMessageWebSocketControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocketMessageReceivedEventArgs {}
impl ::core::clone::Clone for IMessageWebSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageWebSocketMessageReceivedEventArgs2 {}
impl ::core::clone::Clone for IMessageWebSocketMessageReceivedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServerMessageWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServerMessageWebSocket {}
impl ::core::clone::Clone for IServerMessageWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServerMessageWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServerMessageWebSocketControl {}
impl ::core::clone::Clone for IServerMessageWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServerMessageWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServerMessageWebSocketInformation {}
impl ::core::clone::Clone for IServerMessageWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServerStreamWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServerStreamWebSocket {}
impl ::core::clone::Clone for IServerStreamWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServerStreamWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServerStreamWebSocketInformation {}
impl ::core::clone::Clone for IServerStreamWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketActivityContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketActivityContext {}
impl ::core::clone::Clone for ISocketActivityContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketActivityContextFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketActivityContextFactory {}
impl ::core::clone::Clone for ISocketActivityContextFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketActivityInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketActivityInformation {}
impl ::core::clone::Clone for ISocketActivityInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketActivityInformationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketActivityInformationStatics {}
impl ::core::clone::Clone for ISocketActivityInformationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketActivityTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketActivityTriggerDetails {}
impl ::core::clone::Clone for ISocketActivityTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketErrorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketErrorStatics {}
impl ::core::clone::Clone for ISocketErrorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocket {}
impl ::core::clone::Clone for IStreamSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocket2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocket2 {}
impl ::core::clone::Clone for IStreamSocket2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocket3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocket3 {}
impl ::core::clone::Clone for IStreamSocket3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketControl {}
impl ::core::clone::Clone for IStreamSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketControl2 {}
impl ::core::clone::Clone for IStreamSocketControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketControl3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketControl3 {}
impl ::core::clone::Clone for IStreamSocketControl3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketControl4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketControl4 {}
impl ::core::clone::Clone for IStreamSocketControl4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketInformation {}
impl ::core::clone::Clone for IStreamSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketInformation2 {}
impl ::core::clone::Clone for IStreamSocketInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListener {}
impl ::core::clone::Clone for IStreamSocketListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListener2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListener2 {}
impl ::core::clone::Clone for IStreamSocketListener2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListener3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListener3 {}
impl ::core::clone::Clone for IStreamSocketListener3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListenerConnectionReceivedEventArgs {}
impl ::core::clone::Clone for IStreamSocketListenerConnectionReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListenerControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListenerControl {}
impl ::core::clone::Clone for IStreamSocketListenerControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListenerControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListenerControl2 {}
impl ::core::clone::Clone for IStreamSocketListenerControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketListenerInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketListenerInformation {}
impl ::core::clone::Clone for IStreamSocketListenerInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamSocketStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamSocketStatics {}
impl ::core::clone::Clone for IStreamSocketStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamWebSocket {}
impl ::core::clone::Clone for IStreamWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamWebSocket2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamWebSocket2 {}
impl ::core::clone::Clone for IStreamWebSocket2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamWebSocketControl {}
impl ::core::clone::Clone for IStreamWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamWebSocketControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamWebSocketControl2 {}
impl ::core::clone::Clone for IStreamWebSocketControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocket {}
impl ::core::clone::Clone for IWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketClosedEventArgs {}
impl ::core::clone::Clone for IWebSocketClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketControl {}
impl ::core::clone::Clone for IWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketControl2 {}
impl ::core::clone::Clone for IWebSocketControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketErrorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketErrorStatics {}
impl ::core::clone::Clone for IWebSocketErrorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketInformation {}
impl ::core::clone::Clone for IWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketInformation2 {}
impl ::core::clone::Clone for IWebSocketInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebSocketServerCustomValidationRequestedEventArgs {}
impl ::core::clone::Clone for IWebSocketServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MessageWebSocket {}
impl ::core::clone::Clone for MessageWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MessageWebSocketControl {}
impl ::core::clone::Clone for MessageWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MessageWebSocketInformation {}
impl ::core::clone::Clone for MessageWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageWebSocketMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MessageWebSocketMessageReceivedEventArgs {}
impl ::core::clone::Clone for MessageWebSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageWebSocketReceiveMode(pub i32);
impl MessageWebSocketReceiveMode {
    pub const FullMessage: Self = Self(0i32);
    pub const PartialMessage: Self = Self(1i32);
}
impl ::core::marker::Copy for MessageWebSocketReceiveMode {}
impl ::core::clone::Clone for MessageWebSocketReceiveMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RoundTripTimeStatistics {
    pub Variance: u32,
    pub Max: u32,
    pub Min: u32,
    pub Sum: u32,
}
impl ::core::marker::Copy for RoundTripTimeStatistics {}
impl ::core::clone::Clone for RoundTripTimeStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServerMessageWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServerMessageWebSocket {}
impl ::core::clone::Clone for ServerMessageWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServerMessageWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServerMessageWebSocketControl {}
impl ::core::clone::Clone for ServerMessageWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServerMessageWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServerMessageWebSocketInformation {}
impl ::core::clone::Clone for ServerMessageWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServerStreamWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServerStreamWebSocket {}
impl ::core::clone::Clone for ServerStreamWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServerStreamWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServerStreamWebSocketInformation {}
impl ::core::clone::Clone for ServerStreamWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityConnectedStandbyAction(pub i32);
impl SocketActivityConnectedStandbyAction {
    pub const DoNotWake: Self = Self(0i32);
    pub const Wake: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketActivityConnectedStandbyAction {}
impl ::core::clone::Clone for SocketActivityConnectedStandbyAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocketActivityContext {}
impl ::core::clone::Clone for SocketActivityContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocketActivityInformation {}
impl ::core::clone::Clone for SocketActivityInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityKind(pub i32);
impl SocketActivityKind {
    pub const None: Self = Self(0i32);
    pub const StreamSocketListener: Self = Self(1i32);
    pub const DatagramSocket: Self = Self(2i32);
    pub const StreamSocket: Self = Self(3i32);
}
impl ::core::marker::Copy for SocketActivityKind {}
impl ::core::clone::Clone for SocketActivityKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocketActivityTriggerDetails {}
impl ::core::clone::Clone for SocketActivityTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityTriggerReason(pub i32);
impl SocketActivityTriggerReason {
    pub const None: Self = Self(0i32);
    pub const SocketActivity: Self = Self(1i32);
    pub const ConnectionAccepted: Self = Self(2i32);
    pub const KeepAliveTimerExpired: Self = Self(3i32);
    pub const SocketClosed: Self = Self(4i32);
}
impl ::core::marker::Copy for SocketActivityTriggerReason {}
impl ::core::clone::Clone for SocketActivityTriggerReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketErrorStatus(pub i32);
impl SocketErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const OperationAborted: Self = Self(1i32);
    pub const HttpInvalidServerResponse: Self = Self(2i32);
    pub const ConnectionTimedOut: Self = Self(3i32);
    pub const AddressFamilyNotSupported: Self = Self(4i32);
    pub const SocketTypeNotSupported: Self = Self(5i32);
    pub const HostNotFound: Self = Self(6i32);
    pub const NoDataRecordOfRequestedType: Self = Self(7i32);
    pub const NonAuthoritativeHostNotFound: Self = Self(8i32);
    pub const ClassTypeNotFound: Self = Self(9i32);
    pub const AddressAlreadyInUse: Self = Self(10i32);
    pub const CannotAssignRequestedAddress: Self = Self(11i32);
    pub const ConnectionRefused: Self = Self(12i32);
    pub const NetworkIsUnreachable: Self = Self(13i32);
    pub const UnreachableHost: Self = Self(14i32);
    pub const NetworkIsDown: Self = Self(15i32);
    pub const NetworkDroppedConnectionOnReset: Self = Self(16i32);
    pub const SoftwareCausedConnectionAbort: Self = Self(17i32);
    pub const ConnectionResetByPeer: Self = Self(18i32);
    pub const HostIsDown: Self = Self(19i32);
    pub const NoAddressesFound: Self = Self(20i32);
    pub const TooManyOpenFiles: Self = Self(21i32);
    pub const MessageTooLong: Self = Self(22i32);
    pub const CertificateExpired: Self = Self(23i32);
    pub const CertificateUntrustedRoot: Self = Self(24i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(25i32);
    pub const CertificateWrongUsage: Self = Self(26i32);
    pub const CertificateRevoked: Self = Self(27i32);
    pub const CertificateNoRevocationCheck: Self = Self(28i32);
    pub const CertificateRevocationServerOffline: Self = Self(29i32);
    pub const CertificateIsInvalid: Self = Self(30i32);
}
impl ::core::marker::Copy for SocketErrorStatus {}
impl ::core::clone::Clone for SocketErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketMessageType(pub i32);
impl SocketMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Utf8: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketMessageType {}
impl ::core::clone::Clone for SocketMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketProtectionLevel(pub i32);
impl SocketProtectionLevel {
    pub const PlainSocket: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const SslAllowNullEncryption: Self = Self(2i32);
    pub const BluetoothEncryptionAllowNullAuthentication: Self = Self(3i32);
    pub const BluetoothEncryptionWithAuthentication: Self = Self(4i32);
    pub const Ssl3AllowWeakEncryption: Self = Self(5i32);
    pub const Tls10: Self = Self(6i32);
    pub const Tls11: Self = Self(7i32);
    pub const Tls12: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for SocketProtectionLevel {}
impl ::core::clone::Clone for SocketProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketQualityOfService(pub i32);
impl SocketQualityOfService {
    pub const Normal: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketQualityOfService {}
impl ::core::clone::Clone for SocketQualityOfService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketSslErrorSeverity(pub i32);
impl SocketSslErrorSeverity {
    pub const None: Self = Self(0i32);
    pub const Ignorable: Self = Self(1i32);
    pub const Fatal: Self = Self(2i32);
}
impl ::core::marker::Copy for SocketSslErrorSeverity {}
impl ::core::clone::Clone for SocketSslErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocket {}
impl ::core::clone::Clone for StreamSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocketControl {}
impl ::core::clone::Clone for StreamSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocketInformation {}
impl ::core::clone::Clone for StreamSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocketListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocketListener {}
impl ::core::clone::Clone for StreamSocketListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocketListenerConnectionReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocketListenerConnectionReceivedEventArgs {}
impl ::core::clone::Clone for StreamSocketListenerConnectionReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocketListenerControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocketListenerControl {}
impl ::core::clone::Clone for StreamSocketListenerControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamSocketListenerInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamSocketListenerInformation {}
impl ::core::clone::Clone for StreamSocketListenerInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamWebSocket(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamWebSocket {}
impl ::core::clone::Clone for StreamWebSocket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamWebSocketControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamWebSocketControl {}
impl ::core::clone::Clone for StreamWebSocketControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamWebSocketInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamWebSocketInformation {}
impl ::core::clone::Clone for StreamWebSocketInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebSocketClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebSocketClosedEventArgs {}
impl ::core::clone::Clone for WebSocketClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebSocketKeepAlive(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebSocketKeepAlive {}
impl ::core::clone::Clone for WebSocketKeepAlive {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebSocketServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebSocketServerCustomValidationRequestedEventArgs {}
impl ::core::clone::Clone for WebSocketServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
