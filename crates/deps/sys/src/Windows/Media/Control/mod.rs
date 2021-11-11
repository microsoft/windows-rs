#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CurrentSessionChangedEventArgs();
    fn GlobalSystemMediaTransportControlsSession();
    fn GlobalSystemMediaTransportControlsSessionManager();
    fn GlobalSystemMediaTransportControlsSessionMediaProperties();
    fn GlobalSystemMediaTransportControlsSessionPlaybackControls();
    fn GlobalSystemMediaTransportControlsSessionPlaybackInfo();
    fn GlobalSystemMediaTransportControlsSessionPlaybackStatus();
    fn GlobalSystemMediaTransportControlsSessionTimelineProperties();
    fn ICurrentSessionChangedEventArgs();
    fn IGlobalSystemMediaTransportControlsSession();
    fn IGlobalSystemMediaTransportControlsSessionManager();
    fn IGlobalSystemMediaTransportControlsSessionManagerStatics();
    fn IGlobalSystemMediaTransportControlsSessionMediaProperties();
    fn IGlobalSystemMediaTransportControlsSessionPlaybackControls();
    fn IGlobalSystemMediaTransportControlsSessionPlaybackInfo();
    fn IGlobalSystemMediaTransportControlsSessionTimelineProperties();
    fn IMediaPropertiesChangedEventArgs();
    fn IPlaybackInfoChangedEventArgs();
    fn ISessionsChangedEventArgs();
    fn ITimelinePropertiesChangedEventArgs();
    fn MediaPropertiesChangedEventArgs();
    fn PlaybackInfoChangedEventArgs();
    fn SessionsChangedEventArgs();
    fn TimelinePropertiesChangedEventArgs();
}
