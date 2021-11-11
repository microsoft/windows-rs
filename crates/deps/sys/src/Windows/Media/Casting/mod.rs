#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CastingConnection();
    fn CastingConnectionErrorOccurredEventArgs();
    fn CastingConnectionErrorStatus();
    fn CastingConnectionState();
    fn CastingDevice();
    fn CastingDevicePicker();
    fn CastingDevicePickerFilter();
    fn CastingDeviceSelectedEventArgs();
    fn CastingPlaybackTypes();
    fn CastingSource();
    fn ICastingConnection();
    fn ICastingConnectionErrorOccurredEventArgs();
    fn ICastingDevice();
    fn ICastingDevicePicker();
    fn ICastingDevicePickerFilter();
    fn ICastingDeviceSelectedEventArgs();
    fn ICastingDeviceStatics();
    fn ICastingSource();
}
