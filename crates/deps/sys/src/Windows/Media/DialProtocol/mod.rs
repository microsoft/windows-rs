#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DialApp();
    fn DialAppLaunchResult();
    fn DialAppState();
    fn DialAppStateDetails();
    fn DialAppStopResult();
    fn DialDevice();
    fn DialDeviceDisplayStatus();
    fn DialDevicePicker();
    fn DialDevicePickerFilter();
    fn DialDeviceSelectedEventArgs();
    fn DialDisconnectButtonClickedEventArgs();
    fn DialReceiverApp();
    fn IDialApp();
    fn IDialAppStateDetails();
    fn IDialDevice();
    fn IDialDevice2();
    fn IDialDevicePicker();
    fn IDialDevicePickerFilter();
    fn IDialDeviceSelectedEventArgs();
    fn IDialDeviceStatics();
    fn IDialDisconnectButtonClickedEventArgs();
    fn IDialReceiverApp();
    fn IDialReceiverApp2();
    fn IDialReceiverAppStatics();
}
