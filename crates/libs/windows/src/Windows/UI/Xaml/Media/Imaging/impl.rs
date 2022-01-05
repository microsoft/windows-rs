#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageImpl: Sized {
    fn CreateOptions(&self) -> ::windows::core::Result<BitmapCreateOptions>;
    fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows::core::Result<()>;
    fn UriSource(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DecodePixelWidth(&self) -> ::windows::core::Result<i32>;
    fn SetDecodePixelWidth(&self, value: i32) -> ::windows::core::Result<()>;
    fn DecodePixelHeight(&self) -> ::windows::core::Result<i32>;
    fn SetDecodePixelHeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn DownloadProgress(&self, handler: &::core::option::Option<DownloadProgressEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgress(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageFailed(&self, handler: &::core::option::Option<super::super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImage2Impl: Sized {
    fn DecodePixelType(&self) -> ::windows::core::Result<DecodePixelType>;
    fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImage3Impl: Sized {
    fn IsAnimatedBitmap(&self) -> ::windows::core::Result<bool>;
    fn IsPlaying(&self) -> ::windows::core::Result<bool>;
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageFactoryImpl: Sized {
    fn CreateInstanceWithUriSource(&self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<BitmapImage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStaticsImpl: Sized {
    fn CreateOptionsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn UriSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DecodePixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DecodePixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStatics2Impl: Sized {
    fn DecodePixelTypeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapImageStatics3Impl: Sized {
    fn IsAnimatedBitmapProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPlayingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceImpl: Sized {
    fn PixelWidth(&self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&self) -> ::windows::core::Result<i32>;
    fn SetSource(&self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetSourceAsync(&self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BitmapSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapSourceStaticsImpl: Sized {
    fn PixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadProgressEventArgsImpl: Sized {
    fn Progress(&self) -> ::windows::core::Result<i32>;
    fn SetProgress(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderTargetBitmapImpl: Sized {
    fn PixelWidth(&self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&self) -> ::windows::core::Result<i32>;
    fn RenderAsync(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn RenderToSizeAsync(&self, element: &::core::option::Option<super::super::UIElement>, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetPixelsAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderTargetBitmapStaticsImpl: Sized {
    fn PixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapSourceImpl: Sized {
    fn SetBitmapAsync(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISurfaceImageSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISurfaceImageSourceFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&self, pixelwidth: i32, pixelheight: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&self, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceImpl: Sized {
    fn UriSource(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn RasterizePixelWidth(&self) -> ::windows::core::Result<f64>;
    fn SetRasterizePixelWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn RasterizePixelHeight(&self) -> ::windows::core::Result<f64>;
    fn SetRasterizePixelHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OpenFailed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSourceAsync(&self, streamsource: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
    fn CreateInstanceWithUriSource(&self, urisource: &::core::option::Option<super::super::super::super::Foundation::Uri>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SvgImageSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceFailedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SvgImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceOpenedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISvgImageSourceStaticsImpl: Sized {
    fn UriSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RasterizePixelHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualSurfaceImageSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVirtualSurfaceImageSourceFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<VirtualSurfaceImageSource>;
    fn CreateInstanceWithDimensionsAndOpacity(&self, pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows::core::Result<VirtualSurfaceImageSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWriteableBitmapImpl: Sized {
    fn PixelBuffer(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn Invalidate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWriteableBitmapFactoryImpl: Sized {
    fn CreateInstanceWithDimensions(&self, pixelwidth: i32, pixelheight: i32) -> ::windows::core::Result<WriteableBitmap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlRenderingBackgroundTask>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRenderingBackgroundTaskOverridesImpl: Sized {
    fn OnRun(&self, taskinstance: &::core::option::Option<super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
