#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalConnectionImpl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalConnectionClosedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<DevicePortalConnectionClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalConnectionRequestReceivedEventArgsImpl: Sized {
    fn RequestMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpRequestMessage>;
    fn ResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalConnectionStaticsImpl: Sized {
    fn GetForAppServiceConnection(&self, appserviceconnection: &::core::option::Option<super::super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<DevicePortalConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalWebSocketConnectionImpl: Sized {
    fn GetServerMessageWebSocketForRequest(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>;
    fn GetServerMessageWebSocketForRequest2(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>;
    fn GetServerMessageWebSocketForRequest3(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows::core::HSTRING, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>;
    fn GetServerStreamWebSocketForRequest(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>;
    fn GetServerStreamWebSocketForRequest2(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>, protocol: &::windows::core::HSTRING, outboundbuffersizeinbytes: u32, nodelay: bool) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalWebSocketConnectionRequestReceivedEventArgsImpl: Sized {
    fn IsWebSocketUpgradeRequest(&self) -> ::windows::core::Result<bool>;
    fn WebSocketProtocolsRequested(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
