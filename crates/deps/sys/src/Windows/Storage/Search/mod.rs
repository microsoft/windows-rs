#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CommonFileQuery(pub i32);
impl CommonFileQuery {
    pub const DefaultQuery: CommonFileQuery = CommonFileQuery(0i32);
    pub const OrderByName: CommonFileQuery = CommonFileQuery(1i32);
    pub const OrderByTitle: CommonFileQuery = CommonFileQuery(2i32);
    pub const OrderByMusicProperties: CommonFileQuery = CommonFileQuery(3i32);
    pub const OrderBySearchRank: CommonFileQuery = CommonFileQuery(4i32);
    pub const OrderByDate: CommonFileQuery = CommonFileQuery(5i32);
}
#[repr(transparent)]
pub struct CommonFolderQuery(pub i32);
impl CommonFolderQuery {
    pub const DefaultQuery: CommonFolderQuery = CommonFolderQuery(0i32);
    pub const GroupByYear: CommonFolderQuery = CommonFolderQuery(100i32);
    pub const GroupByMonth: CommonFolderQuery = CommonFolderQuery(101i32);
    pub const GroupByArtist: CommonFolderQuery = CommonFolderQuery(102i32);
    pub const GroupByAlbum: CommonFolderQuery = CommonFolderQuery(103i32);
    pub const GroupByAlbumArtist: CommonFolderQuery = CommonFolderQuery(104i32);
    pub const GroupByComposer: CommonFolderQuery = CommonFolderQuery(105i32);
    pub const GroupByGenre: CommonFolderQuery = CommonFolderQuery(106i32);
    pub const GroupByPublishedYear: CommonFolderQuery = CommonFolderQuery(107i32);
    pub const GroupByRating: CommonFolderQuery = CommonFolderQuery(108i32);
    pub const GroupByTag: CommonFolderQuery = CommonFolderQuery(109i32);
    pub const GroupByAuthor: CommonFolderQuery = CommonFolderQuery(110i32);
    pub const GroupByType: CommonFolderQuery = CommonFolderQuery(111i32);
}
#[repr(transparent)]
pub struct ContentIndexer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentIndexerQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DateStackOption(pub i32);
impl DateStackOption {
    pub const None: DateStackOption = DateStackOption(0i32);
    pub const Year: DateStackOption = DateStackOption(1i32);
    pub const Month: DateStackOption = DateStackOption(2i32);
}
#[repr(transparent)]
pub struct FolderDepth(pub i32);
impl FolderDepth {
    pub const Shallow: FolderDepth = FolderDepth(0i32);
    pub const Deep: FolderDepth = FolderDepth(1i32);
}
#[repr(transparent)]
pub struct IContentIndexer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentIndexerQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentIndexerQueryOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentIndexerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIndexableContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryOptionsWithProviderFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFileQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFileQueryResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolderQueryOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolderQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageQueryResultBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValueAndLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IndexableContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: IndexedState = IndexedState(0i32);
    pub const NotIndexed: IndexedState = IndexedState(1i32);
    pub const PartiallyIndexed: IndexedState = IndexedState(2i32);
    pub const FullyIndexed: IndexedState = IndexedState(3i32);
}
#[repr(transparent)]
pub struct IndexerOption(pub i32);
impl IndexerOption {
    pub const UseIndexerWhenAvailable: IndexerOption = IndexerOption(0i32);
    pub const OnlyUseIndexer: IndexerOption = IndexerOption(1i32);
    pub const DoNotUseIndexer: IndexerOption = IndexerOption(2i32);
    pub const OnlyUseIndexerAndOptimizeForIndexedProperties: IndexerOption = IndexerOption(3i32);
}
#[repr(transparent)]
pub struct QueryOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SortEntry(i32);
#[repr(transparent)]
pub struct SortEntryVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageFileQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageFolderQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryContentChangedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ValueAndLanguage(pub *mut ::core::ffi::c_void);
