#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn IUserActivity();
    fn IUserActivity2();
    fn IUserActivity3();
    fn IUserActivityAttribution();
    fn IUserActivityAttributionFactory();
    fn IUserActivityChannel();
    fn IUserActivityChannel2();
    fn IUserActivityChannelStatics();
    fn IUserActivityChannelStatics2();
    fn IUserActivityChannelStatics3();
    fn IUserActivityContentInfo();
    fn IUserActivityContentInfoStatics();
    fn IUserActivityFactory();
    fn IUserActivityRequest();
    fn IUserActivityRequestManager();
    fn IUserActivityRequestManagerStatics();
    fn IUserActivityRequestedEventArgs();
    fn IUserActivitySession();
    fn IUserActivitySessionHistoryItem();
    fn IUserActivityStatics();
    fn IUserActivityVisualElements();
    fn IUserActivityVisualElements2();
    fn UserActivity();
    fn UserActivityAttribution();
    fn UserActivityChannel();
    fn UserActivityContentInfo();
    fn UserActivityRequest();
    fn UserActivityRequestManager();
    fn UserActivityRequestedEventArgs();
    fn UserActivitySession();
    fn UserActivitySessionHistoryItem();
    fn UserActivityState();
    fn UserActivityVisualElements();
}
