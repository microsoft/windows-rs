#[cfg(feature = "implement_exclusive")]
pub trait IPdfDocumentImpl: Sized {
    fn GetPage(&self, pageindex: u32) -> ::windows::core::Result<PdfPage>;
    fn PageCount(&self) -> ::windows::core::Result<u32>;
    fn IsPasswordProtected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfDocumentStaticsImpl: Sized {
    fn LoadFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromFileWithPasswordAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, password: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromStreamAsync(&self, inputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
    fn LoadFromStreamWithPasswordAsync(&self, inputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, password: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfPageImpl: Sized {
    fn RenderToStreamAsync(&self, outputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RenderWithOptionsToStreamAsync(&self, outputstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, options: &::core::option::Option<PdfPageRenderOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PreparePageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Index(&self) -> ::windows::core::Result<u32>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Dimensions(&self) -> ::windows::core::Result<PdfPageDimensions>;
    fn Rotation(&self) -> ::windows::core::Result<PdfPageRotation>;
    fn PreferredZoom(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfPageDimensionsImpl: Sized {
    fn MediaBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CropBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn BleedBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TrimBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ArtBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPdfPageRenderOptionsImpl: Sized {
    fn SourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSourceRect(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn DestinationWidth(&self) -> ::windows::core::Result<u32>;
    fn SetDestinationWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn DestinationHeight(&self) -> ::windows::core::Result<u32>;
    fn SetDestinationHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn IsIgnoringHighContrast(&self) -> ::windows::core::Result<bool>;
    fn SetIsIgnoringHighContrast(&self, value: bool) -> ::windows::core::Result<()>;
    fn BitmapEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetBitmapEncoderId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
