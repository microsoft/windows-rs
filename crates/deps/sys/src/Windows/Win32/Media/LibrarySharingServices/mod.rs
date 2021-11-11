#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWindowsMediaLibrarySharingDevice();
    fn IWindowsMediaLibrarySharingDeviceProperties();
    fn IWindowsMediaLibrarySharingDeviceProperty();
    fn IWindowsMediaLibrarySharingDevices();
    fn IWindowsMediaLibrarySharingServices();
    fn WindowsMediaLibrarySharingDeviceAuthorizationStatus();
    fn WindowsMediaLibrarySharingServices();
}
