#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[link(name = "windows")]
extern "system" {
    fn IWebViewControl();
    fn IWebViewControl2();
    fn IWebViewControlContentLoadingEventArgs();
    fn IWebViewControlDOMContentLoadedEventArgs();
    fn IWebViewControlDeferredPermissionRequest();
    fn IWebViewControlLongRunningScriptDetectedEventArgs();
    fn IWebViewControlNavigationCompletedEventArgs();
    fn IWebViewControlNavigationStartingEventArgs();
    fn IWebViewControlNewWindowRequestedEventArgs();
    fn IWebViewControlNewWindowRequestedEventArgs2();
    fn IWebViewControlPermissionRequest();
    fn IWebViewControlPermissionRequestedEventArgs();
    fn IWebViewControlScriptNotifyEventArgs();
    fn IWebViewControlSettings();
    fn IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs();
    fn IWebViewControlUnviewableContentIdentifiedEventArgs();
    fn IWebViewControlWebResourceRequestedEventArgs();
    fn WebViewControlContentLoadingEventArgs();
    fn WebViewControlDOMContentLoadedEventArgs();
    fn WebViewControlDeferredPermissionRequest();
    fn WebViewControlLongRunningScriptDetectedEventArgs();
    fn WebViewControlNavigationCompletedEventArgs();
    fn WebViewControlNavigationStartingEventArgs();
    fn WebViewControlNewWindowRequestedEventArgs();
    fn WebViewControlPermissionRequest();
    fn WebViewControlPermissionRequestedEventArgs();
    fn WebViewControlPermissionState();
    fn WebViewControlPermissionType();
    fn WebViewControlScriptNotifyEventArgs();
    fn WebViewControlSettings();
    fn WebViewControlUnsupportedUriSchemeIdentifiedEventArgs();
    fn WebViewControlUnviewableContentIdentifiedEventArgs();
    fn WebViewControlWebResourceRequestedEventArgs();
}
