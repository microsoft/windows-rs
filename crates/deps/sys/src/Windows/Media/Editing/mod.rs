#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BackgroundAudioTrack();
    fn EmbeddedAudioTrack();
    fn IBackgroundAudioTrack();
    fn IBackgroundAudioTrackStatics();
    fn IEmbeddedAudioTrack();
    fn IMediaClip();
    fn IMediaClipStatics();
    fn IMediaClipStatics2();
    fn IMediaComposition();
    fn IMediaComposition2();
    fn IMediaCompositionStatics();
    fn IMediaOverlay();
    fn IMediaOverlayFactory();
    fn IMediaOverlayLayer();
    fn IMediaOverlayLayerFactory();
    fn MediaClip();
    fn MediaComposition();
    fn MediaOverlay();
    fn MediaOverlayLayer();
    fn MediaTrimmingPreference();
    fn VideoFramePrecision();
}
