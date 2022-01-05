#[cfg(feature = "implement_exclusive")]
pub trait IDetectedFaceImpl: Sized {
    fn FaceBox(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapBounds>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectorImpl: Sized {
    fn DetectFacesAsync(&self, image: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn DetectFacesWithSearchAreaAsync(&self, image: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, searcharea: &super::super::Graphics::Imaging::BitmapBounds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn MinDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMinDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn MaxDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMaxDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectorStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceDetector>>;
    fn GetSupportedBitmapPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn IsBitmapPixelFormatSupported(&self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceTrackerImpl: Sized {
    fn ProcessNextFrameAsync(&self, videoframe: &::core::option::Option<super::VideoFrame>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>;
    fn MinDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMinDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn MaxDetectableFaceSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetMaxDetectableFaceSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceTrackerStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FaceTracker>>;
    fn GetSupportedBitmapPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn IsBitmapPixelFormatSupported(&self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<bool>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
