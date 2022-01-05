#[cfg(feature = "implement_exclusive")]
pub trait IFileInformationFactoryImpl: Sized {
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>;
    fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>;
    fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>;
    fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>;
    fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>;
    fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>;
    fn GetVirtualizedItemsVector(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetVirtualizedFilesVector(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetVirtualizedFoldersVector(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileInformationFactoryFactoryImpl: Sized {
    fn CreateWithMode(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<FileInformationFactory>;
    fn CreateWithModeAndSize(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows::core::Result<FileInformationFactory>;
    fn CreateWithModeAndSizeAndOptions(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<FileInformationFactory>;
    fn CreateWithModeAndSizeAndOptionsAndFlags(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows::core::Result<FileInformationFactory>;
}
pub trait IStorageItemInformationImpl: Sized {
    fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties>;
    fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties>;
    fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties>;
    fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail>;
    fn ThumbnailUpdated(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThumbnailUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesUpdated(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
