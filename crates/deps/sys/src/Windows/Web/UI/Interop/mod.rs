#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWebViewControlAcceleratorKeyPressedEventArgs();
    fn IWebViewControlMoveFocusRequestedEventArgs();
    fn IWebViewControlProcess();
    fn IWebViewControlProcessFactory();
    fn IWebViewControlProcessOptions();
    fn IWebViewControlSite();
    fn IWebViewControlSite2();
    fn WebViewControl();
    fn WebViewControlAcceleratorKeyPressedEventArgs();
    fn WebViewControlAcceleratorKeyRoutingStage();
    fn WebViewControlMoveFocusReason();
    fn WebViewControlMoveFocusRequestedEventArgs();
    fn WebViewControlProcess();
    fn WebViewControlProcessCapabilityState();
    fn WebViewControlProcessOptions();
}
