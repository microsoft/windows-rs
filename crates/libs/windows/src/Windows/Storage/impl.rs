#[cfg(feature = "implement_exclusive")]
pub trait IAppDataPathsImpl: Sized {
    fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDataPathsStaticsImpl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<AppDataPaths>;
    fn GetDefault(&self) -> ::windows::core::Result<AppDataPaths>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataImpl: Sized {
    fn Version(&self) -> ::windows::core::Result<u32>;
    fn SetVersionAsync(&self, desiredversion: u32, handler: &::core::option::Option<ApplicationDataSetVersionHandler>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ClearAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ClearAsync(&self, locality: ApplicationDataLocality) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LocalSettings(&self) -> ::windows::core::Result<ApplicationDataContainer>;
    fn RoamingSettings(&self) -> ::windows::core::Result<ApplicationDataContainer>;
    fn LocalFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn RoamingFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn TemporaryFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn DataChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveDataChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SignalDataChanged(&self) -> ::windows::core::Result<()>;
    fn RoamingStorageQuota(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationData2Impl: Sized {
    fn LocalCacheFolder(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationData3Impl: Sized {
    fn GetPublisherCacheFolder(&self, foldername: &::windows::core::HSTRING) -> ::windows::core::Result<StorageFolder>;
    fn ClearPublisherCacheFolderAsync(&self, foldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn SharedLocalFolder(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataContainerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Locality(&self) -> ::windows::core::Result<ApplicationDataLocality>;
    fn Values(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
    fn Containers(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>>;
    fn CreateContainer(&self, name: &::windows::core::HSTRING, disposition: ApplicationDataCreateDisposition) -> ::windows::core::Result<ApplicationDataContainer>;
    fn DeleteContainer(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<ApplicationData>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataStatics2Impl: Sized {
    fn GetForUserAsync(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ApplicationData>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileManagerStaticsImpl: Sized {
    fn DeferUpdates(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<()>;
    fn CompleteUpdatesAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadsFolderStaticsImpl: Sized {
    fn CreateFileAsync(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsync(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFileWithCollisionOptionAsync(&self, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderWithCollisionOptionAsync(&self, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadsFolderStatics2Impl: Sized {
    fn CreateFileForUserAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderForUserAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFileForUserWithCollisionOptionAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderForUserWithCollisionOptionAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileIOStaticsImpl: Sized {
    fn ReadTextAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReadTextWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn WriteTextAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteTextWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadLinesAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn ReadLinesWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn WriteLinesAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteLinesWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadBufferAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>>;
    fn WriteBufferAsync(&self, file: &::core::option::Option<IStorageFile>, buffer: &::core::option::Option<Streams::IBuffer>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteBytesAsync(&self, file: &::core::option::Option<IStorageFile>, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersCameraRollStaticsImpl: Sized {
    fn CameraRoll(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersPlaylistsStaticsImpl: Sized {
    fn Playlists(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersSavedPicturesStaticsImpl: Sized {
    fn SavedPictures(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStaticsImpl: Sized {
    fn MusicLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn PicturesLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn VideosLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn DocumentsLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn HomeGroup(&self) -> ::windows::core::Result<StorageFolder>;
    fn RemovableDevices(&self) -> ::windows::core::Result<StorageFolder>;
    fn MediaServerDevices(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics2Impl: Sized {
    fn Objects3D(&self) -> ::windows::core::Result<StorageFolder>;
    fn AppCaptures(&self) -> ::windows::core::Result<StorageFolder>;
    fn RecordedCalls(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics3Impl: Sized {
    fn GetFolderForUserAsync(&self, user: &::core::option::Option<super::System::User>, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics4Impl: Sized {
    fn RequestAccessAsync(&self, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>;
    fn RequestAccessForUserAsync(&self, user: &::core::option::Option<super::System::User>, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>;
    fn GetFolderAsync(&self, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIOStaticsImpl: Sized {
    fn ReadTextAsync(&self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReadTextWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn WriteTextAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteTextWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadLinesAsync(&self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn ReadLinesWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn WriteLinesAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteLinesWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadBufferAsync(&self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>>;
    fn WriteBufferAsync(&self, absolutepath: &::windows::core::HSTRING, buffer: &::core::option::Option<Streams::IBuffer>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteBytesAsync(&self, absolutepath: &::windows::core::HSTRING, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetVersionDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetVersionRequestImpl: Sized {
    fn CurrentVersion(&self) -> ::windows::core::Result<u32>;
    fn DesiredVersion(&self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&self) -> ::windows::core::Result<SetVersionDeferral>;
}
#[cfg(feature = "Storage_Streams")]
pub trait IStorageFileImpl: Sized + IInputStreamReferenceImpl + IRandomAccessStreamReferenceImpl + IStorageItemImpl {
    fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
    fn CopyOverloadDefaultNameAndOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverloadDefaultOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverload(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyAndReplaceAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultNameAndOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverload(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveAndReplaceAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
pub trait IStorageFile2Impl: Sized {
    fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
}
pub trait IStorageFilePropertiesWithAvailabilityImpl: Sized {
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileStaticsImpl: Sized {
    fn GetFileFromPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFileFromApplicationUriAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateStreamedFileAsync(&self, displaynamewithextension: &::windows::core::HSTRING, datarequested: &::core::option::Option<StreamedFileDataRequestedHandler>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn ReplaceWithStreamedFileAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>, datarequested: &::core::option::Option<StreamedFileDataRequestedHandler>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateStreamedFileFromUriAsync(&self, displaynamewithextension: &::windows::core::HSTRING, uri: &::core::option::Option<super::Foundation::Uri>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn ReplaceWithStreamedFileFromUriAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>, uri: &::core::option::Option<super::Foundation::Uri>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileStatics2Impl: Sized {
    fn GetFileFromPathForUserAsync(&self, user: &::core::option::Option<super::System::User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
}
pub trait IStorageFolderImpl: Sized + IStorageItemImpl {
    fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFileAsync(&self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFolderAsync(&self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetFileAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>;
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>;
    fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>;
}
pub trait IStorageFolder2Impl: Sized {
    fn TryGetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolder3Impl: Sized {
    fn TryGetChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolderStaticsImpl: Sized {
    fn GetFolderFromPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolderStatics2Impl: Sized {
    fn GetFolderFromPathForUserAsync(&self, user: &::core::option::Option<super::System::User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
pub trait IStorageItemImpl: Sized {
    fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn RenameAsync(&self, desiredname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Attributes(&self) -> ::windows::core::Result<FileAttributes>;
    fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
}
pub trait IStorageItem2Impl: Sized + IStorageItemImpl {
    fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn IsEqual(&self, item: &::core::option::Option<IStorageItem>) -> ::windows::core::Result<bool>;
}
pub trait IStorageItemPropertiesImpl: Sized {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties>;
}
pub trait IStorageItemProperties2Impl: Sized + IStorageItemPropertiesImpl {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
}
pub trait IStorageItemPropertiesWithProviderImpl: Sized + IStorageItemPropertiesImpl {
    fn Provider(&self) -> ::windows::core::Result<StorageProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryImpl: Sized {
    fn RequestAddFolderAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn RequestRemoveFolderAsync(&self, folder: &::core::option::Option<StorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn Folders(&self) -> ::windows::core::Result<super::Foundation::Collections::IObservableVector<StorageFolder>>;
    fn SaveFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn DefinitionChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveDefinitionChanged(&self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibrary2Impl: Sized {
    fn ChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibrary3Impl: Sized {
    fn AreFolderSuggestionsAvailableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<StorageLibraryChangeType>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PreviousPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
    fn GetStorageItemAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>>;
    fn AcceptChangesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeReader2Impl: Sized {
    fn GetLastChangeId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerImpl: Sized {
    fn GetChangeReader(&self) -> ::windows::core::Result<StorageLibraryChangeReader>;
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTracker2Impl: Sized {
    fn EnableWithOptions(&self, options: &::core::option::Option<StorageLibraryChangeTrackerOptions>) -> ::windows::core::Result<()>;
    fn Disable(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerOptionsImpl: Sized {
    fn TrackChangeDetails(&self) -> ::windows::core::Result<bool>;
    fn SetTrackChangeDetails(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryLastChangeIdImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryLastChangeIdStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryStaticsImpl: Sized {
    fn GetLibraryAsync(&self, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryStatics2Impl: Sized {
    fn GetLibraryForUserAsync(&self, user: &::core::option::Option<super::System::User>, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProvider2Impl: Sized + IStorageProviderImpl {
    fn IsPropertySupportedForPartialFileAsync(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageStreamTransactionImpl: Sized + IClosableImpl {
    fn Stream(&self) -> ::windows::core::Result<Streams::IRandomAccessStream>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
pub trait IStreamedFileDataRequestImpl: Sized {
    fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemAudioPropertiesImpl: Sized {
    fn EncodingBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDataPathsImpl: Sized {
    fn Fonts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Public(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDesktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDocuments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDownloads(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicMusic(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicVideos(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn System(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemHost(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemX86(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemX64(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemArm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserProfiles(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Windows(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDataPathsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SystemDataPaths>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemGPSPropertiesImpl: Sized {
    fn LatitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LongitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemImagePropertiesImpl: Sized {
    fn HorizontalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VerticalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaPropertiesImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Producer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Writer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Year(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMusicPropertiesImpl: Sized {
    fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Composer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Conductor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemPhotoPropertiesImpl: Sized {
    fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateTaken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PeopleNames(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemPropertiesImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ItemNameDisplay(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Keywords(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rating(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Audio(&self) -> ::windows::core::Result<SystemAudioProperties>;
    fn GPS(&self) -> ::windows::core::Result<SystemGPSProperties>;
    fn Media(&self) -> ::windows::core::Result<SystemMediaProperties>;
    fn Music(&self) -> ::windows::core::Result<SystemMusicProperties>;
    fn Photo(&self) -> ::windows::core::Result<SystemPhotoProperties>;
    fn Video(&self) -> ::windows::core::Result<SystemVideoProperties>;
    fn Image(&self) -> ::windows::core::Result<SystemImageProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemVideoPropertiesImpl: Sized {
    fn Director(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameHeight(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameWidth(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TotalBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataPathsImpl: Sized {
    fn CameraRoll(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Downloads(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppDataLow(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Music(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pictures(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Profile(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recent(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SavedPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Screenshots(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Templates(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Videos(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataPathsStaticsImpl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<UserDataPaths>;
    fn GetDefault(&self) -> ::windows::core::Result<UserDataPaths>;
}
