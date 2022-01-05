pub trait ISoftwareBitmapNativeImpl: Sized {
    fn GetData();
}
pub trait ISoftwareBitmapNativeFactoryImpl: Sized {
    fn CreateFromWICBitmap();
    fn CreateFromMF2DBuffer2();
}
