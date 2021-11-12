#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CommonFileQuery(i32);
pub struct CommonFolderQuery(i32);
pub struct ContentIndexer(i32);
pub struct ContentIndexerQuery(i32);
pub struct DateStackOption(i32);
pub struct FolderDepth(i32);
pub struct IContentIndexer(pub *mut ::core::ffi::c_void);
pub struct IContentIndexerQuery(pub *mut ::core::ffi::c_void);
pub struct IContentIndexerQueryOperations(pub *mut ::core::ffi::c_void);
pub struct IContentIndexerStatics(pub *mut ::core::ffi::c_void);
pub struct IIndexableContent(pub *mut ::core::ffi::c_void);
pub struct IQueryOptions(pub *mut ::core::ffi::c_void);
pub struct IQueryOptionsFactory(pub *mut ::core::ffi::c_void);
pub struct IQueryOptionsWithProviderFilter(pub *mut ::core::ffi::c_void);
pub struct IStorageFileQueryResult(pub *mut ::core::ffi::c_void);
pub struct IStorageFileQueryResult2(pub *mut ::core::ffi::c_void);
pub struct IStorageFolderQueryOperations(pub *mut ::core::ffi::c_void);
pub struct IStorageFolderQueryResult(pub *mut ::core::ffi::c_void);
pub struct IStorageItemQueryResult(pub *mut ::core::ffi::c_void);
pub struct IStorageLibraryChangeTrackerTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IStorageLibraryContentChangedTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IStorageQueryResultBase(pub *mut ::core::ffi::c_void);
pub struct IValueAndLanguage(pub *mut ::core::ffi::c_void);
pub struct IndexableContent(i32);
pub struct IndexedState(i32);
pub struct IndexerOption(i32);
pub struct QueryOptions(i32);
pub struct SortEntry(i32);
pub struct SortEntryVector(i32);
pub struct StorageFileQueryResult(i32);
pub struct StorageFolderQueryResult(i32);
pub struct StorageItemQueryResult(i32);
pub struct StorageLibraryChangeTrackerTriggerDetails(i32);
pub struct StorageLibraryContentChangedTriggerDetails(i32);
pub struct ValueAndLanguage(i32);
