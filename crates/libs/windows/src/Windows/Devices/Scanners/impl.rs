#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultScanSource(&self) -> ::windows::core::Result<ImageScannerScanSource>;
    fn IsScanSourceSupported(&self, value: ImageScannerScanSource) -> ::windows::core::Result<bool>;
    fn FlatbedConfiguration(&self) -> ::windows::core::Result<ImageScannerFlatbedConfiguration>;
    fn FeederConfiguration(&self) -> ::windows::core::Result<ImageScannerFeederConfiguration>;
    fn AutoConfiguration(&self) -> ::windows::core::Result<ImageScannerAutoConfiguration>;
    fn IsPreviewSupported(&self, scansource: ImageScannerScanSource) -> ::windows::core::Result<bool>;
    fn ScanPreviewToStreamAsync(&self, scansource: ImageScannerScanSource, targetstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>>;
    fn ScanFilesToFolderAsync(&self, scansource: ImageScannerScanSource, storagefolder: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerFeederConfigurationImpl: Sized + IImageScannerFormatConfigurationImpl + IImageScannerSourceConfigurationImpl {
    fn CanAutoDetectPageSize(&self) -> ::windows::core::Result<bool>;
    fn AutoDetectPageSize(&self) -> ::windows::core::Result<bool>;
    fn SetAutoDetectPageSize(&self, value: bool) -> ::windows::core::Result<()>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Graphics::Printing::PrintMediaSize>;
    fn SetPageSize(&self, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::Result<()>;
    fn PageOrientation(&self) -> ::windows::core::Result<super::super::Graphics::Printing::PrintOrientation>;
    fn SetPageOrientation(&self, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::Result<()>;
    fn PageSizeDimensions(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn IsPageSizeSupported(&self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::Result<bool>;
    fn MaxNumberOfPages(&self) -> ::windows::core::Result<u32>;
    fn SetMaxNumberOfPages(&self, value: u32) -> ::windows::core::Result<()>;
    fn CanScanDuplex(&self) -> ::windows::core::Result<bool>;
    fn Duplex(&self) -> ::windows::core::Result<bool>;
    fn SetDuplex(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanScanAhead(&self) -> ::windows::core::Result<bool>;
    fn ScanAhead(&self) -> ::windows::core::Result<bool>;
    fn SetScanAhead(&self, value: bool) -> ::windows::core::Result<()>;
}
pub trait IImageScannerFormatConfigurationImpl: Sized {
    fn DefaultFormat(&self) -> ::windows::core::Result<ImageScannerFormat>;
    fn Format(&self) -> ::windows::core::Result<ImageScannerFormat>;
    fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::core::Result<()>;
    fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerPreviewResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Format(&self) -> ::windows::core::Result<ImageScannerFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerScanResultImpl: Sized {
    fn ScannedFiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
pub trait IImageScannerSourceConfigurationImpl: Sized + IImageScannerFormatConfigurationImpl {
    fn MinScanArea(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MaxScanArea(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SelectedScanRegion(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSelectedScanRegion(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn AutoCroppingMode(&self) -> ::windows::core::Result<ImageScannerAutoCroppingMode>;
    fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<()>;
    fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<bool>;
    fn MinResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn MaxResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn OpticalResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DesiredResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn SetDesiredResolution(&self, value: &ImageScannerResolution) -> ::windows::core::Result<()>;
    fn ActualResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DefaultColorMode(&self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn ColorMode(&self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows::core::Result<()>;
    fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows::core::Result<bool>;
    fn MinBrightness(&self) -> ::windows::core::Result<i32>;
    fn MaxBrightness(&self) -> ::windows::core::Result<i32>;
    fn BrightnessStep(&self) -> ::windows::core::Result<u32>;
    fn DefaultBrightness(&self) -> ::windows::core::Result<i32>;
    fn Brightness(&self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinContrast(&self) -> ::windows::core::Result<i32>;
    fn MaxContrast(&self) -> ::windows::core::Result<i32>;
    fn ContrastStep(&self) -> ::windows::core::Result<u32>;
    fn DefaultContrast(&self) -> ::windows::core::Result<i32>;
    fn Contrast(&self) -> ::windows::core::Result<i32>;
    fn SetContrast(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageScanner>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
