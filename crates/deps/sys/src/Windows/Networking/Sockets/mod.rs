#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BandwidthStatistics(i32);
#[repr(transparent)]
pub struct ControlChannelTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ControlChannelTriggerContract(i32);
#[repr(C)]
pub struct ControlChannelTriggerResetReason(i32);
#[repr(C)]
pub struct ControlChannelTriggerResourceType(i32);
#[repr(C)]
pub struct ControlChannelTriggerStatus(i32);
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
#[repr(C)]
pub struct MessageWebSocketReceiveMode(i32);
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
#[repr(C)]
pub struct SocketActivityConnectedStandbyAction(i32);
#[repr(transparent)]
pub struct SocketActivityContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocketActivityInformation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SocketActivityKind(i32);
#[repr(transparent)]
pub struct SocketActivityTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SocketActivityTriggerReason(i32);
#[repr(transparent)]
pub struct SocketError(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SocketErrorStatus(i32);
#[repr(C)]
pub struct SocketMessageType(i32);
#[repr(C)]
pub struct SocketProtectionLevel(i32);
#[repr(C)]
pub struct SocketQualityOfService(i32);
#[repr(C)]
pub struct SocketSslErrorSeverity(i32);
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
pub struct WebSocketError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebSocketKeepAlive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebSocketServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
