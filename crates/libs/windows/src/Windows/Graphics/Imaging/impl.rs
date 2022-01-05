#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBitmapBufferImpl: Sized + IClosableImpl + IMemoryBufferImpl {
    fn GetPlaneCount(&self) -> ::windows::core::Result<i32>;
    fn GetPlaneDescription(&self, index: i32) -> ::windows::core::Result<BitmapPlaneDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapCodecInformationImpl: Sized {
    fn CodecId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn FileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MimeTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderImpl: Sized {
    fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapPropertiesView>;
    fn DecoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation>;
    fn FrameCount(&self) -> ::windows::core::Result<u32>;
    fn GetPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn GetFrameAsync(&self, frameindex: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapFrame>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderStaticsImpl: Sized {
    fn BmpDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PngDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TiffDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GifDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegXRDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IcoDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDecoderInformationEnumerator(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>;
    fn CreateAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>;
    fn CreateWithIdAsync(&self, decoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapDecoderStatics2Impl: Sized {
    fn HeifDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn WebpDecoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderImpl: Sized {
    fn EncoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation>;
    fn BitmapProperties(&self) -> ::windows::core::Result<BitmapProperties>;
    fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapProperties>;
    fn IsThumbnailGenerated(&self) -> ::windows::core::Result<bool>;
    fn SetIsThumbnailGenerated(&self, value: bool) -> ::windows::core::Result<()>;
    fn GeneratedThumbnailWidth(&self) -> ::windows::core::Result<u32>;
    fn SetGeneratedThumbnailWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn GeneratedThumbnailHeight(&self) -> ::windows::core::Result<u32>;
    fn SetGeneratedThumbnailHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn BitmapTransform(&self) -> ::windows::core::Result<BitmapTransform>;
    fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GoToNextFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GoToNextFrameWithEncodingOptionsAsync(&self, encodingoptions: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderStaticsImpl: Sized {
    fn BmpEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PngEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TiffEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GifEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn JpegXREncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetEncoderInformationEnumerator(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>;
    fn CreateAsync(&self, encoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateWithEncodingOptionsAsync(&self, encoderid: &::windows::core::GUID, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, encodingoptions: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateForTranscodingAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, bitmapdecoder: &::core::option::Option<BitmapDecoder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
    fn CreateForInPlacePropertyEncodingAsync(&self, bitmapdecoder: &::core::option::Option<BitmapDecoder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderStatics2Impl: Sized {
    fn HeifEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapEncoderWithSoftwareBitmapImpl: Sized {
    fn SetSoftwareBitmap(&self, bitmap: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<()>;
}
pub trait IBitmapFrameImpl: Sized {
    fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode>;
    fn DpiX(&self) -> ::windows::core::Result<f64>;
    fn DpiY(&self) -> ::windows::core::Result<f64>;
    fn PixelWidth(&self) -> ::windows::core::Result<u32>;
    fn PixelHeight(&self) -> ::windows::core::Result<u32>;
    fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32>;
    fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32>;
    fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
}
pub trait IBitmapFrameWithSoftwareBitmapImpl: Sized + IBitmapFrameImpl {
    fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &::core::option::Option<BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapPropertiesImpl: Sized + IBitmapPropertiesViewImpl {
    fn SetPropertiesAsync(&self, propertiestoset: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
pub trait IBitmapPropertiesViewImpl: Sized {
    fn GetPropertiesAsync(&self, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTransformImpl: Sized {
    fn ScaledWidth(&self) -> ::windows::core::Result<u32>;
    fn SetScaledWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn ScaledHeight(&self) -> ::windows::core::Result<u32>;
    fn SetScaledHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn InterpolationMode(&self) -> ::windows::core::Result<BitmapInterpolationMode>;
    fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn Flip(&self) -> ::windows::core::Result<BitmapFlip>;
    fn SetFlip(&self, value: BitmapFlip) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<BitmapRotation>;
    fn SetRotation(&self, value: BitmapRotation) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<BitmapBounds>;
    fn SetBounds(&self, value: &BitmapBounds) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTypedValueImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Type(&self) -> ::windows::core::Result<super::super::Foundation::PropertyType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapTypedValueFactoryImpl: Sized {
    fn Create(&self, value: &::core::option::Option<::windows::core::IInspectable>, r#type: super::super::Foundation::PropertyType) -> ::windows::core::Result<BitmapTypedValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPixelDataProviderImpl: Sized {
    fn DetachPixelData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISoftwareBitmapImpl: Sized + IClosableImpl {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode>;
    fn PixelWidth(&self) -> ::windows::core::Result<i32>;
    fn PixelHeight(&self) -> ::windows::core::Result<i32>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetDpiX(&self, value: f64) -> ::windows::core::Result<()>;
    fn DpiX(&self) -> ::windows::core::Result<f64>;
    fn SetDpiY(&self, value: f64) -> ::windows::core::Result<()>;
    fn DpiY(&self) -> ::windows::core::Result<f64>;
    fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> ::windows::core::Result<BitmapBuffer>;
    fn CopyTo(&self, bitmap: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn CopyFromBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CopyToBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetReadOnlyView(&self) -> ::windows::core::Result<SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapFactoryImpl: Sized {
    fn Create(&self, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateWithAlpha(&self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISoftwareBitmapStaticsImpl: Sized {
    fn Copy(&self, source: &::core::option::Option<SoftwareBitmap>) -> ::windows::core::Result<SoftwareBitmap>;
    fn Convert(&self, source: &::core::option::Option<SoftwareBitmap>, format: BitmapPixelFormat) -> ::windows::core::Result<SoftwareBitmap>;
    fn ConvertWithAlpha(&self, source: &::core::option::Option<SoftwareBitmap>, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyFromBuffer(&self, source: &::core::option::Option<super::super::Storage::Streams::IBuffer>, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyWithAlphaFromBuffer(&self, source: &::core::option::Option<super::super::Storage::Streams::IBuffer>, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>;
    fn CreateCopyFromSurfaceAsync(&self, surface: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn CreateCopyWithAlphaFromSurfaceAsync(&self, surface: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>, alpha: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
