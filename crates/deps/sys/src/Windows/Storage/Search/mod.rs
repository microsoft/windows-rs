#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CommonFileQuery(i32);
#[repr(C)]
pub struct CommonFolderQuery(i32);
#[repr(transparent)]
pub struct ContentIndexer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentIndexerQuery(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DateStackOption(i32);
#[repr(C)]
pub struct FolderDepth(i32);
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
#[repr(C)]
pub struct IndexedState(i32);
#[repr(C)]
pub struct IndexerOption(i32);
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
