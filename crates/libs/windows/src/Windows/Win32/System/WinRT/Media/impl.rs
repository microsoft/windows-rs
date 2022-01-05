pub trait IAudioFrameNativeImpl: Sized {
    fn GetData();
}
pub trait IAudioFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
pub trait IVideoFrameNativeImpl: Sized {
    fn GetData();
    fn GetDevice();
}
pub trait IVideoFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
