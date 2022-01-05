#[cfg(feature = "implement_exclusive")]
pub trait IQuickLinkImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportedDataFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SupportedFileTypes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareOperationImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::DataPackageView>;
    fn QuickLinkId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoveThisQuickLink(&self) -> ::windows::core::Result<()>;
    fn ReportStarted(&self) -> ::windows::core::Result<()>;
    fn ReportDataRetrieved(&self) -> ::windows::core::Result<()>;
    fn ReportSubmittedBackgroundTask(&self) -> ::windows::core::Result<()>;
    fn ReportCompletedWithQuickLink(&self, quicklink: &::core::option::Option<QuickLink>) -> ::windows::core::Result<()>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareOperation2Impl: Sized {
    fn DismissUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareOperation3Impl: Sized {
    fn Contacts(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::Contacts::Contact>>;
}
