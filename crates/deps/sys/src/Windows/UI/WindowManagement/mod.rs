#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {
    fn AppWindow();
    fn AppWindowChangedEventArgs();
    fn AppWindowCloseRequestedEventArgs();
    fn AppWindowClosedEventArgs();
    fn AppWindowClosedReason();
    fn AppWindowFrame();
    fn AppWindowFrameStyle();
    fn AppWindowPlacement();
    fn AppWindowPresentationConfiguration();
    fn AppWindowPresentationKind();
    fn AppWindowPresenter();
    fn AppWindowTitleBar();
    fn AppWindowTitleBarOcclusion();
    fn AppWindowTitleBarVisibility();
    fn CompactOverlayPresentationConfiguration();
    fn DefaultPresentationConfiguration();
    fn DisplayRegion();
    fn FullScreenPresentationConfiguration();
    fn IAppWindow();
    fn IAppWindowChangedEventArgs();
    fn IAppWindowCloseRequestedEventArgs();
    fn IAppWindowClosedEventArgs();
    fn IAppWindowFrame();
    fn IAppWindowFrameStyle();
    fn IAppWindowPlacement();
    fn IAppWindowPresentationConfiguration();
    fn IAppWindowPresentationConfigurationFactory();
    fn IAppWindowPresenter();
    fn IAppWindowStatics();
    fn IAppWindowTitleBar();
    fn IAppWindowTitleBarOcclusion();
    fn IAppWindowTitleBarVisibility();
    fn ICompactOverlayPresentationConfiguration();
    fn IDefaultPresentationConfiguration();
    fn IDisplayRegion();
    fn IFullScreenPresentationConfiguration();
    fn IWindowServicesStatics();
    fn IWindowingEnvironment();
    fn IWindowingEnvironmentAddedEventArgs();
    fn IWindowingEnvironmentChangedEventArgs();
    fn IWindowingEnvironmentRemovedEventArgs();
    fn IWindowingEnvironmentStatics();
    fn WindowServices();
    fn WindowingEnvironment();
    fn WindowingEnvironmentAddedEventArgs();
    fn WindowingEnvironmentChangedEventArgs();
    fn WindowingEnvironmentKind();
    fn WindowingEnvironmentRemovedEventArgs();
}
