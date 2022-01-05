#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportDeleteImportedItemsFromSourceResultImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PhotoImportSession>;
    fn HasSucceeded(&self) -> ::windows::core::Result<bool>;
    fn DeletedItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>;
    fn PhotosCount(&self) -> ::windows::core::Result<u32>;
    fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn VideosCount(&self) -> ::windows::core::Result<u32>;
    fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SidecarsCount(&self) -> ::windows::core::Result<u32>;
    fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SiblingsCount(&self) -> ::windows::core::Result<u32>;
    fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn TotalCount(&self) -> ::windows::core::Result<u32>;
    fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportFindItemsResultImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PhotoImportSession>;
    fn HasSucceeded(&self) -> ::windows::core::Result<bool>;
    fn FoundItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>;
    fn PhotosCount(&self) -> ::windows::core::Result<u32>;
    fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn VideosCount(&self) -> ::windows::core::Result<u32>;
    fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SidecarsCount(&self) -> ::windows::core::Result<u32>;
    fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SiblingsCount(&self) -> ::windows::core::Result<u32>;
    fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn TotalCount(&self) -> ::windows::core::Result<u32>;
    fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SelectAll(&self) -> ::windows::core::Result<()>;
    fn SelectNone(&self) -> ::windows::core::Result<()>;
    fn SelectNewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetImportMode(&self, value: PhotoImportImportMode) -> ::windows::core::Result<()>;
    fn ImportMode(&self) -> ::windows::core::Result<PhotoImportImportMode>;
    fn SelectedPhotosCount(&self) -> ::windows::core::Result<u32>;
    fn SelectedPhotosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SelectedVideosCount(&self) -> ::windows::core::Result<u32>;
    fn SelectedVideosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SelectedSidecarsCount(&self) -> ::windows::core::Result<u32>;
    fn SelectedSidecarsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SelectedSiblingsCount(&self) -> ::windows::core::Result<u32>;
    fn SelectedSiblingsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SelectedTotalCount(&self) -> ::windows::core::Result<u32>;
    fn SelectedTotalSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SelectionChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImportItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>;
    fn ItemImported(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemImported(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportFindItemsResult2Impl: Sized {
    fn AddItemsInDateRangeToSelection(&self, rangestart: &super::super::Foundation::DateTime, rangelength: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportImportItemsResultImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PhotoImportSession>;
    fn HasSucceeded(&self) -> ::windows::core::Result<bool>;
    fn ImportedItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>;
    fn PhotosCount(&self) -> ::windows::core::Result<u32>;
    fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn VideosCount(&self) -> ::windows::core::Result<u32>;
    fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SidecarsCount(&self) -> ::windows::core::Result<u32>;
    fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SiblingsCount(&self) -> ::windows::core::Result<u32>;
    fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn TotalCount(&self) -> ::windows::core::Result<u32>;
    fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn DeleteImportedItemsFromSourceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportItemImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ItemKey(&self) -> ::windows::core::Result<u64>;
    fn ContentType(&self) -> ::windows::core::Result<PhotoImportContentType>;
    fn SizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Sibling(&self) -> ::windows::core::Result<PhotoImportSidecar>;
    fn Sidecars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>;
    fn VideoSegments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportVideoSegment>>;
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ImportedFileNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DeletedFileNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportItem2Impl: Sized {
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportItemImportedEventArgsImpl: Sized {
    fn ImportedItem(&self) -> ::windows::core::Result<PhotoImportItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportManagerStaticsImpl: Sized {
    fn IsSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn FindAllSourcesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhotoImportSource>>>;
    fn GetPendingOperations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportOperationImpl: Sized {
    fn Stage(&self) -> ::windows::core::Result<PhotoImportStage>;
    fn Session(&self) -> ::windows::core::Result<PhotoImportSession>;
    fn ContinueFindingItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>;
    fn ContinueImportingItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>;
    fn ContinueDeletingImportedItemsFromSourceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSelectionChangedEventArgsImpl: Sized {
    fn IsSelectionEmpty(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhotoImportSessionImpl: Sized + IClosableImpl {
    fn Source(&self) -> ::windows::core::Result<PhotoImportSource>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDestinationFolder(&self, value: &::core::option::Option<super::super::Storage::IStorageFolder>) -> ::windows::core::Result<()>;
    fn DestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::IStorageFolder>;
    fn SetAppendSessionDateToDestinationFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn AppendSessionDateToDestinationFolder(&self) -> ::windows::core::Result<bool>;
    fn SetSubfolderCreationMode(&self, value: PhotoImportSubfolderCreationMode) -> ::windows::core::Result<()>;
    fn SubfolderCreationMode(&self) -> ::windows::core::Result<PhotoImportSubfolderCreationMode>;
    fn SetDestinationFileNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DestinationFileNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FindItemsAsync(&self, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSession2Impl: Sized {
    fn SetSubfolderDateFormat(&self, value: PhotoImportSubfolderDateFormat) -> ::windows::core::Result<()>;
    fn SubfolderDateFormat(&self) -> ::windows::core::Result<PhotoImportSubfolderDateFormat>;
    fn SetRememberDeselectedItems(&self, value: bool) -> ::windows::core::Result<()>;
    fn RememberDeselectedItems(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSidecarImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSourceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Model(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionTransport(&self) -> ::windows::core::Result<PhotoImportConnectionTransport>;
    fn Type(&self) -> ::windows::core::Result<PhotoImportSourceType>;
    fn PowerSource(&self) -> ::windows::core::Result<PhotoImportPowerSource>;
    fn BatteryLevelPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn DateTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn StorageMedia(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportStorageMedium>>;
    fn IsLocked(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn IsMassStorage(&self) -> ::windows::core::Result<bool>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn CreateImportSession(&self) -> ::windows::core::Result<PhotoImportSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportSourceStaticsImpl: Sized {
    fn FromIdAsync(&self, sourceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>>;
    fn FromFolderAsync(&self, sourcerootfolder: &::core::option::Option<super::super::Storage::IStorageFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportStorageMediumImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageMediumType(&self) -> ::windows::core::Result<PhotoImportStorageMediumType>;
    fn SupportedAccessMode(&self) -> ::windows::core::Result<PhotoImportAccessMode>;
    fn CapacityInBytes(&self) -> ::windows::core::Result<u64>;
    fn AvailableSpaceInBytes(&self) -> ::windows::core::Result<u64>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoImportVideoSegmentImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Sibling(&self) -> ::windows::core::Result<PhotoImportSidecar>;
    fn Sidecars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>;
}
