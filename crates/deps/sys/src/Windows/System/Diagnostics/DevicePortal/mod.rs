#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DevicePortalConnection();
    fn DevicePortalConnectionClosedEventArgs();
    fn DevicePortalConnectionClosedReason();
    fn DevicePortalConnectionRequestReceivedEventArgs();
    fn IDevicePortalConnection();
    fn IDevicePortalConnectionClosedEventArgs();
    fn IDevicePortalConnectionRequestReceivedEventArgs();
    fn IDevicePortalConnectionStatics();
    fn IDevicePortalWebSocketConnection();
    fn IDevicePortalWebSocketConnectionRequestReceivedEventArgs();
}
