#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CommonFileQuery(pub i32);
impl CommonFileQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const OrderByName: Self = Self(1i32);
    pub const OrderByTitle: Self = Self(2i32);
    pub const OrderByMusicProperties: Self = Self(3i32);
    pub const OrderBySearchRank: Self = Self(4i32);
    pub const OrderByDate: Self = Self(5i32);
}
impl ::core::marker::Copy for CommonFileQuery {}
impl ::core::clone::Clone for CommonFileQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommonFolderQuery(pub i32);
impl CommonFolderQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const GroupByYear: Self = Self(100i32);
    pub const GroupByMonth: Self = Self(101i32);
    pub const GroupByArtist: Self = Self(102i32);
    pub const GroupByAlbum: Self = Self(103i32);
    pub const GroupByAlbumArtist: Self = Self(104i32);
    pub const GroupByComposer: Self = Self(105i32);
    pub const GroupByGenre: Self = Self(106i32);
    pub const GroupByPublishedYear: Self = Self(107i32);
    pub const GroupByRating: Self = Self(108i32);
    pub const GroupByTag: Self = Self(109i32);
    pub const GroupByAuthor: Self = Self(110i32);
    pub const GroupByType: Self = Self(111i32);
}
impl ::core::marker::Copy for CommonFolderQuery {}
impl ::core::clone::Clone for CommonFolderQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentIndexer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentIndexer {}
impl ::core::clone::Clone for ContentIndexer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentIndexerQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentIndexerQuery {}
impl ::core::clone::Clone for ContentIndexerQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DateStackOption(pub i32);
impl DateStackOption {
    pub const None: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Month: Self = Self(2i32);
}
impl ::core::marker::Copy for DateStackOption {}
impl ::core::clone::Clone for DateStackOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FolderDepth(pub i32);
impl FolderDepth {
    pub const Shallow: Self = Self(0i32);
    pub const Deep: Self = Self(1i32);
}
impl ::core::marker::Copy for FolderDepth {}
impl ::core::clone::Clone for FolderDepth {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentIndexer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentIndexer {}
impl ::core::clone::Clone for IContentIndexer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentIndexerQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentIndexerQuery {}
impl ::core::clone::Clone for IContentIndexerQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentIndexerQueryOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentIndexerQueryOperations {}
impl ::core::clone::Clone for IContentIndexerQueryOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentIndexerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentIndexerStatics {}
impl ::core::clone::Clone for IContentIndexerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIndexableContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIndexableContent {}
impl ::core::clone::Clone for IIndexableContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQueryOptions {}
impl ::core::clone::Clone for IQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQueryOptionsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQueryOptionsFactory {}
impl ::core::clone::Clone for IQueryOptionsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQueryOptionsWithProviderFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQueryOptionsWithProviderFilter {}
impl ::core::clone::Clone for IQueryOptionsWithProviderFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFileQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFileQueryResult {}
impl ::core::clone::Clone for IStorageFileQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFileQueryResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFileQueryResult2 {}
impl ::core::clone::Clone for IStorageFileQueryResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolderQueryOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolderQueryOperations {}
impl ::core::clone::Clone for IStorageFolderQueryOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolderQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolderQueryResult {}
impl ::core::clone::Clone for IStorageFolderQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemQueryResult {}
impl ::core::clone::Clone for IStorageItemQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeTrackerTriggerDetails {}
impl ::core::clone::Clone for IStorageLibraryChangeTrackerTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryContentChangedTriggerDetails {}
impl ::core::clone::Clone for IStorageLibraryContentChangedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageQueryResultBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageQueryResultBase {}
impl ::core::clone::Clone for IStorageQueryResultBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IValueAndLanguage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IValueAndLanguage {}
impl ::core::clone::Clone for IValueAndLanguage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IndexableContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IndexableContent {}
impl ::core::clone::Clone for IndexableContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: Self = Self(0i32);
    pub const NotIndexed: Self = Self(1i32);
    pub const PartiallyIndexed: Self = Self(2i32);
    pub const FullyIndexed: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexedState {}
impl ::core::clone::Clone for IndexedState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IndexerOption(pub i32);
impl IndexerOption {
    pub const UseIndexerWhenAvailable: Self = Self(0i32);
    pub const OnlyUseIndexer: Self = Self(1i32);
    pub const DoNotUseIndexer: Self = Self(2i32);
    pub const OnlyUseIndexerAndOptimizeForIndexedProperties: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexerOption {}
impl ::core::clone::Clone for IndexerOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QueryOptions {}
impl ::core::clone::Clone for QueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SortEntry {
    pub PropertyName: ::windows_sys::core::HSTRING,
    pub AscendingOrder: bool,
}
impl ::core::marker::Copy for SortEntry {}
impl ::core::clone::Clone for SortEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SortEntryVector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SortEntryVector {}
impl ::core::clone::Clone for SortEntryVector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageFileQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageFileQueryResult {}
impl ::core::clone::Clone for StorageFileQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageFolderQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageFolderQueryResult {}
impl ::core::clone::Clone for StorageFolderQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageItemQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageItemQueryResult {}
impl ::core::clone::Clone for StorageItemQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryChangeTrackerTriggerDetails {}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryContentChangedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryContentChangedTriggerDetails {}
impl ::core::clone::Clone for StorageLibraryContentChangedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ValueAndLanguage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ValueAndLanguage {}
impl ::core::clone::Clone for ValueAndLanguage {
    fn clone(&self) -> Self {
        *self
    }
}
