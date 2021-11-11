#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FrameNavigationOptions();
    fn IFrameNavigationOptions();
    fn IFrameNavigationOptionsFactory();
    fn INavigatingCancelEventArgs();
    fn INavigatingCancelEventArgs2();
    fn INavigationEventArgs();
    fn INavigationEventArgs2();
    fn INavigationFailedEventArgs();
    fn IPageStackEntry();
    fn IPageStackEntryFactory();
    fn IPageStackEntryStatics();
    fn LoadCompletedEventHandler();
    fn NavigatedEventHandler();
    fn NavigatingCancelEventArgs();
    fn NavigatingCancelEventHandler();
    fn NavigationCacheMode();
    fn NavigationEventArgs();
    fn NavigationFailedEventArgs();
    fn NavigationFailedEventHandler();
    fn NavigationMode();
    fn NavigationStoppedEventHandler();
    fn PageStackEntry();
}
