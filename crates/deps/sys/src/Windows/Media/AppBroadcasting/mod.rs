#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppBroadcastingContract();
    fn AppBroadcastingMonitor();
    fn AppBroadcastingStatus();
    fn AppBroadcastingStatusDetails();
    fn AppBroadcastingUI();
    fn IAppBroadcastingMonitor();
    fn IAppBroadcastingStatus();
    fn IAppBroadcastingStatusDetails();
    fn IAppBroadcastingUI();
    fn IAppBroadcastingUIStatics();
}
