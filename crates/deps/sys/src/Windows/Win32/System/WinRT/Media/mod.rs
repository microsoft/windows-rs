#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CLSID_AudioFrameNativeFactory();
    fn CLSID_VideoFrameNativeFactory();
    fn IAudioFrameNative();
    fn IAudioFrameNativeFactory();
    fn IVideoFrameNative();
    fn IVideoFrameNativeFactory();
}
