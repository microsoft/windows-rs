#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Storage_AccessCache")]
pub mod AccessCache;
#[cfg(feature = "Storage_BulkAccess")]
pub mod BulkAccess;
#[cfg(feature = "Storage_Compression")]
pub mod Compression;
#[cfg(feature = "Storage_FileProperties")]
pub mod FileProperties;
#[cfg(feature = "Storage_Pickers")]
pub mod Pickers;
#[cfg(feature = "Storage_Provider")]
pub mod Provider;
#[cfg(feature = "Storage_Search")]
pub mod Search;
#[cfg(feature = "Storage_Streams")]
pub mod Streams;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppDataPaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationDataCompositeValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationDataContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationDataContainerSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ApplicationDataCreateDisposition(i32);
#[repr(C)]
pub struct ApplicationDataLocality(i32);
#[repr(transparent)]
pub struct ApplicationDataSetVersionHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CreationCollisionOption(i32);
#[repr(C)]
pub struct FileAccessMode(i32);
#[repr(C)]
pub struct FileAttributes(i32);
#[repr(transparent)]
pub struct IAppDataPaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDataPathsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationData2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationData3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDataContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDataStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDataStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICachedFileManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadsFolderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadsFolderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileIOStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersCameraRollStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersPlaylistsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersSavedPicturesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFoldersStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIOStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetVersionDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetVersionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFilePropertiesWithAvailability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFileStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFileStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolder3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemPropertiesWithProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibrary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibrary2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibrary3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeTracker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryLastChangeId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryLastChangeIdStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProvider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageStreamTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamedFileDataRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemAudioProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemDataPaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemDataPathsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemGPSProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemImageProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMusicProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemPhotoProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemVideoProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataPaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataPathsStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct KnownFolderId(i32);
#[repr(C)]
pub struct KnownFoldersAccessStatus(i32);
#[repr(C)]
pub struct KnownLibraryId(i32);
#[repr(C)]
pub struct NameCollisionOption(i32);
#[repr(transparent)]
pub struct SetVersionDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetVersionRequest(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StorageDeleteOption(i32);
#[repr(transparent)]
pub struct StorageFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageFolder(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StorageItemTypes(i32);
#[repr(transparent)]
pub struct StorageLibrary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StorageLibraryChangeType(i32);
#[repr(transparent)]
pub struct StorageLibraryLastChangeId(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StorageOpenOptions(i32);
#[repr(transparent)]
pub struct StorageProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageStreamTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamedFileDataRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamedFileDataRequestedHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StreamedFileFailureMode(i32);
#[repr(transparent)]
pub struct SystemAudioProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemDataPaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemGPSProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemImageProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMusicProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemPhotoProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemVideoProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataPaths(pub *mut ::core::ffi::c_void);
