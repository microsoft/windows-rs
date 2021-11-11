#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppListEntry();
    fn AppRestartFailureReason();
    fn CoreApplication();
    fn CoreApplicationView();
    fn CoreApplicationViewTitleBar();
    fn HostedViewClosingEventArgs();
    fn IAppListEntry();
    fn IAppListEntry2();
    fn IAppListEntry3();
    fn IAppListEntry4();
    fn ICoreApplication();
    fn ICoreApplication2();
    fn ICoreApplication3();
    fn ICoreApplicationExit();
    fn ICoreApplicationUnhandledError();
    fn ICoreApplicationUseCount();
    fn ICoreApplicationView();
    fn ICoreApplicationView2();
    fn ICoreApplicationView3();
    fn ICoreApplicationView5();
    fn ICoreApplicationView6();
    fn ICoreApplicationViewTitleBar();
    fn ICoreImmersiveApplication();
    fn ICoreImmersiveApplication2();
    fn ICoreImmersiveApplication3();
    fn IFrameworkView();
    fn IFrameworkViewSource();
    fn IHostedViewClosingEventArgs();
    fn IUnhandledError();
    fn IUnhandledErrorDetectedEventArgs();
    fn UnhandledError();
    fn UnhandledErrorDetectedEventArgs();
}
