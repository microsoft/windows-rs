#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct ApplicationDataCreateDisposition(pub i32);
impl ApplicationDataCreateDisposition {
    pub const Always: Self = Self(0i32);
    pub const Existing: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ApplicationDataLocality(pub i32);
impl ApplicationDataLocality {
    pub const Local: Self = Self(0i32);
    pub const Roaming: Self = Self(1i32);
    pub const Temporary: Self = Self(2i32);
    pub const LocalCache: Self = Self(3i32);
    pub const SharedLocal: Self = Self(4i32);
}
#[repr(transparent)]
pub struct ApplicationDataSetVersionHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreationCollisionOption(pub i32);
impl CreationCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
    pub const OpenIfExists: Self = Self(3i32);
}
#[repr(transparent)]
pub struct FileAccessMode(pub i32);
impl FileAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
#[repr(transparent)]
pub struct FileAttributes(pub u32);
impl FileAttributes {
    pub const Normal: Self = Self(0u32);
    pub const ReadOnly: Self = Self(1u32);
    pub const Directory: Self = Self(16u32);
    pub const Archive: Self = Self(32u32);
    pub const Temporary: Self = Self(256u32);
    pub const LocallyIncomplete: Self = Self(512u32);
}
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
#[repr(transparent)]
pub struct KnownFolderId(pub i32);
impl KnownFolderId {
    pub const AppCaptures: Self = Self(0i32);
    pub const CameraRoll: Self = Self(1i32);
    pub const DocumentsLibrary: Self = Self(2i32);
    pub const HomeGroup: Self = Self(3i32);
    pub const MediaServerDevices: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const Objects3D: Self = Self(6i32);
    pub const PicturesLibrary: Self = Self(7i32);
    pub const Playlists: Self = Self(8i32);
    pub const RecordedCalls: Self = Self(9i32);
    pub const RemovableDevices: Self = Self(10i32);
    pub const SavedPictures: Self = Self(11i32);
    pub const Screenshots: Self = Self(12i32);
    pub const VideosLibrary: Self = Self(13i32);
    pub const AllAppMods: Self = Self(14i32);
    pub const CurrentAppMods: Self = Self(15i32);
    pub const DownloadsFolder: Self = Self(16i32);
}
#[repr(transparent)]
pub struct KnownFoldersAccessStatus(pub i32);
impl KnownFoldersAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
    pub const AllowedPerAppFolder: Self = Self(5i32);
}
#[repr(transparent)]
pub struct KnownLibraryId(pub i32);
impl KnownLibraryId {
    pub const Music: Self = Self(0i32);
    pub const Pictures: Self = Self(1i32);
    pub const Videos: Self = Self(2i32);
    pub const Documents: Self = Self(3i32);
}
#[repr(transparent)]
pub struct NameCollisionOption(pub i32);
impl NameCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SetVersionDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetVersionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageDeleteOption(pub i32);
impl StorageDeleteOption {
    pub const Default: Self = Self(0i32);
    pub const PermanentDelete: Self = Self(1i32);
}
#[repr(transparent)]
pub struct StorageFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemTypes(pub u32);
impl StorageItemTypes {
    pub const None: Self = Self(0u32);
    pub const File: Self = Self(1u32);
    pub const Folder: Self = Self(2u32);
}
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
#[repr(transparent)]
pub struct StorageLibraryChangeType(pub i32);
impl StorageLibraryChangeType {
    pub const Created: Self = Self(0i32);
    pub const Deleted: Self = Self(1i32);
    pub const MovedOrRenamed: Self = Self(2i32);
    pub const ContentsChanged: Self = Self(3i32);
    pub const MovedOutOfLibrary: Self = Self(4i32);
    pub const MovedIntoLibrary: Self = Self(5i32);
    pub const ContentsReplaced: Self = Self(6i32);
    pub const IndexingStatusChanged: Self = Self(7i32);
    pub const EncryptionChanged: Self = Self(8i32);
    pub const ChangeTrackingLost: Self = Self(9i32);
}
#[repr(transparent)]
pub struct StorageLibraryLastChangeId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageOpenOptions(pub u32);
impl StorageOpenOptions {
    pub const None: Self = Self(0u32);
    pub const AllowOnlyReaders: Self = Self(1u32);
    pub const AllowReadersAndWriters: Self = Self(2u32);
}
#[repr(transparent)]
pub struct StorageProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageStreamTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamedFileDataRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamedFileDataRequestedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamedFileFailureMode(pub i32);
impl StreamedFileFailureMode {
    pub const Failed: Self = Self(0i32);
    pub const CurrentlyUnavailable: Self = Self(1i32);
    pub const Incomplete: Self = Self(2i32);
}
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
