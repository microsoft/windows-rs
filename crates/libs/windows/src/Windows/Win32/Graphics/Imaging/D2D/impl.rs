pub trait IWICImageEncoderImpl: Sized {
    fn WriteFrame();
    fn WriteFrameThumbnail();
    fn WriteThumbnail();
}
pub trait IWICImagingFactory2Impl: Sized + IWICImagingFactoryImpl {
    fn CreateImageEncoder();
}
