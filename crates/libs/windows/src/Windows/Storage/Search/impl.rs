#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerImpl: Sized {
    fn AddAsync(&self, indexablecontent: &::core::option::Option<IIndexableContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateAsync(&self, indexablecontent: &::core::option::Option<IIndexableContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self, contentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteMultipleAsync(&self, contentids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetrievePropertiesAsync(&self, contentid: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn Revision(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerQueryImpl: Sized {
    fn GetCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>;
    fn GetPropertiesRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>;
    fn GetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>;
    fn GetRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>;
    fn QueryFolder(&self) -> ::windows::core::Result<super::StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerQueryOperationsImpl: Sized {
    fn CreateQueryWithSortOrderAndLanguage(&self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, sortorder: &::core::option::Option<super::super::Foundation::Collections::IIterable<SortEntry>>, searchfilterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<ContentIndexerQuery>;
    fn CreateQueryWithSortOrder(&self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, sortorder: &::core::option::Option<super::super::Foundation::Collections::IIterable<SortEntry>>) -> ::windows::core::Result<ContentIndexerQuery>;
    fn CreateQuery(&self, searchfilter: &::windows::core::HSTRING, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<ContentIndexerQuery>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentIndexerStaticsImpl: Sized {
    fn GetIndexerWithName(&self, indexname: &::windows::core::HSTRING) -> ::windows::core::Result<ContentIndexer>;
    fn GetIndexer(&self) -> ::windows::core::Result<ContentIndexer>;
}
pub trait IIndexableContentImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Stream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&self, value: &::core::option::Option<super::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StreamContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQueryOptionsImpl: Sized {
    fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn FolderDepth(&self) -> ::windows::core::Result<FolderDepth>;
    fn SetFolderDepth(&self, value: FolderDepth) -> ::windows::core::Result<()>;
    fn ApplicationSearchFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetApplicationSearchFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserSearchFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserSearchFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IndexerOption(&self) -> ::windows::core::Result<IndexerOption>;
    fn SetIndexerOption(&self, value: IndexerOption) -> ::windows::core::Result<()>;
    fn SortOrder(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SortEntry>>;
    fn GroupPropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateStackOption(&self) -> ::windows::core::Result<DateStackOption>;
    fn SaveToString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LoadFromString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetThumbnailPrefetch(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<()>;
    fn SetPropertyPrefetch(&self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQueryOptionsFactoryImpl: Sized {
    fn CreateCommonFileQuery(&self, query: CommonFileQuery, filetypefilter: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<QueryOptions>;
    fn CreateCommonFolderQuery(&self, query: CommonFolderQuery) -> ::windows::core::Result<QueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQueryOptionsWithProviderFilterImpl: Sized {
    fn StorageProviderIdFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetFilesAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileQueryResult2Impl: Sized + IStorageQueryResultBaseImpl {
    fn GetMatchingPropertiesWithRanges(&self, file: &::core::option::Option<super::StorageFile>) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>>;
}
pub trait IStorageFolderQueryOperationsImpl: Sized {
    fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&self) -> ::windows::core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<bool>;
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows::core::Result<bool>;
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolderQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetFoldersAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemQueryResultImpl: Sized + IStorageQueryResultBaseImpl {
    fn GetItemsAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerTriggerDetailsImpl: Sized {
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn ChangeTracker(&self) -> ::windows::core::Result<super::StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTriggerDetailsImpl: Sized {
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn CreateModifiedSinceQuery(&self, lastquerytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<StorageItemQueryResult>;
}
pub trait IStorageQueryResultBaseImpl: Sized {
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn ContentsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OptionsChanged(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindStartIndexAsync(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&self, newqueryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IValueAndLanguageImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
