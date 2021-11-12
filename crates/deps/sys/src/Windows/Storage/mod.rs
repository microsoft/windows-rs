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
    pub const Always: ApplicationDataCreateDisposition = ApplicationDataCreateDisposition(0i32);
    pub const Existing: ApplicationDataCreateDisposition = ApplicationDataCreateDisposition(1i32);
}
#[repr(transparent)]
pub struct ApplicationDataLocality(pub i32);
impl ApplicationDataLocality {
    pub const Local: ApplicationDataLocality = ApplicationDataLocality(0i32);
    pub const Roaming: ApplicationDataLocality = ApplicationDataLocality(1i32);
    pub const Temporary: ApplicationDataLocality = ApplicationDataLocality(2i32);
    pub const LocalCache: ApplicationDataLocality = ApplicationDataLocality(3i32);
    pub const SharedLocal: ApplicationDataLocality = ApplicationDataLocality(4i32);
}
#[repr(transparent)]
pub struct ApplicationDataSetVersionHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreationCollisionOption(pub i32);
impl CreationCollisionOption {
    pub const GenerateUniqueName: CreationCollisionOption = CreationCollisionOption(0i32);
    pub const ReplaceExisting: CreationCollisionOption = CreationCollisionOption(1i32);
    pub const FailIfExists: CreationCollisionOption = CreationCollisionOption(2i32);
    pub const OpenIfExists: CreationCollisionOption = CreationCollisionOption(3i32);
}
#[repr(transparent)]
pub struct FileAccessMode(pub i32);
impl FileAccessMode {
    pub const Read: FileAccessMode = FileAccessMode(0i32);
    pub const ReadWrite: FileAccessMode = FileAccessMode(1i32);
}
#[repr(transparent)]
pub struct FileAttributes(pub u32);
impl FileAttributes {
    pub const Normal: FileAttributes = FileAttributes(0u32);
    pub const ReadOnly: FileAttributes = FileAttributes(1u32);
    pub const Directory: FileAttributes = FileAttributes(16u32);
    pub const Archive: FileAttributes = FileAttributes(32u32);
    pub const Temporary: FileAttributes = FileAttributes(256u32);
    pub const LocallyIncomplete: FileAttributes = FileAttributes(512u32);
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
    pub const AppCaptures: KnownFolderId = KnownFolderId(0i32);
    pub const CameraRoll: KnownFolderId = KnownFolderId(1i32);
    pub const DocumentsLibrary: KnownFolderId = KnownFolderId(2i32);
    pub const HomeGroup: KnownFolderId = KnownFolderId(3i32);
    pub const MediaServerDevices: KnownFolderId = KnownFolderId(4i32);
    pub const MusicLibrary: KnownFolderId = KnownFolderId(5i32);
    pub const Objects3D: KnownFolderId = KnownFolderId(6i32);
    pub const PicturesLibrary: KnownFolderId = KnownFolderId(7i32);
    pub const Playlists: KnownFolderId = KnownFolderId(8i32);
    pub const RecordedCalls: KnownFolderId = KnownFolderId(9i32);
    pub const RemovableDevices: KnownFolderId = KnownFolderId(10i32);
    pub const SavedPictures: KnownFolderId = KnownFolderId(11i32);
    pub const Screenshots: KnownFolderId = KnownFolderId(12i32);
    pub const VideosLibrary: KnownFolderId = KnownFolderId(13i32);
    pub const AllAppMods: KnownFolderId = KnownFolderId(14i32);
    pub const CurrentAppMods: KnownFolderId = KnownFolderId(15i32);
    pub const DownloadsFolder: KnownFolderId = KnownFolderId(16i32);
}
#[repr(transparent)]
pub struct KnownFoldersAccessStatus(pub i32);
impl KnownFoldersAccessStatus {
    pub const DeniedBySystem: KnownFoldersAccessStatus = KnownFoldersAccessStatus(0i32);
    pub const NotDeclaredByApp: KnownFoldersAccessStatus = KnownFoldersAccessStatus(1i32);
    pub const DeniedByUser: KnownFoldersAccessStatus = KnownFoldersAccessStatus(2i32);
    pub const UserPromptRequired: KnownFoldersAccessStatus = KnownFoldersAccessStatus(3i32);
    pub const Allowed: KnownFoldersAccessStatus = KnownFoldersAccessStatus(4i32);
    pub const AllowedPerAppFolder: KnownFoldersAccessStatus = KnownFoldersAccessStatus(5i32);
}
#[repr(transparent)]
pub struct KnownLibraryId(pub i32);
impl KnownLibraryId {
    pub const Music: KnownLibraryId = KnownLibraryId(0i32);
    pub const Pictures: KnownLibraryId = KnownLibraryId(1i32);
    pub const Videos: KnownLibraryId = KnownLibraryId(2i32);
    pub const Documents: KnownLibraryId = KnownLibraryId(3i32);
}
#[repr(transparent)]
pub struct NameCollisionOption(pub i32);
impl NameCollisionOption {
    pub const GenerateUniqueName: NameCollisionOption = NameCollisionOption(0i32);
    pub const ReplaceExisting: NameCollisionOption = NameCollisionOption(1i32);
    pub const FailIfExists: NameCollisionOption = NameCollisionOption(2i32);
}
#[repr(transparent)]
pub struct SetVersionDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetVersionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageDeleteOption(pub i32);
impl StorageDeleteOption {
    pub const Default: StorageDeleteOption = StorageDeleteOption(0i32);
    pub const PermanentDelete: StorageDeleteOption = StorageDeleteOption(1i32);
}
#[repr(transparent)]
pub struct StorageFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemTypes(pub u32);
impl StorageItemTypes {
    pub const None: StorageItemTypes = StorageItemTypes(0u32);
    pub const File: StorageItemTypes = StorageItemTypes(1u32);
    pub const Folder: StorageItemTypes = StorageItemTypes(2u32);
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
    pub const Created: StorageLibraryChangeType = StorageLibraryChangeType(0i32);
    pub const Deleted: StorageLibraryChangeType = StorageLibraryChangeType(1i32);
    pub const MovedOrRenamed: StorageLibraryChangeType = StorageLibraryChangeType(2i32);
    pub const ContentsChanged: StorageLibraryChangeType = StorageLibraryChangeType(3i32);
    pub const MovedOutOfLibrary: StorageLibraryChangeType = StorageLibraryChangeType(4i32);
    pub const MovedIntoLibrary: StorageLibraryChangeType = StorageLibraryChangeType(5i32);
    pub const ContentsReplaced: StorageLibraryChangeType = StorageLibraryChangeType(6i32);
    pub const IndexingStatusChanged: StorageLibraryChangeType = StorageLibraryChangeType(7i32);
    pub const EncryptionChanged: StorageLibraryChangeType = StorageLibraryChangeType(8i32);
    pub const ChangeTrackingLost: StorageLibraryChangeType = StorageLibraryChangeType(9i32);
}
#[repr(transparent)]
pub struct StorageLibraryLastChangeId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageOpenOptions(pub u32);
impl StorageOpenOptions {
    pub const None: StorageOpenOptions = StorageOpenOptions(0u32);
    pub const AllowOnlyReaders: StorageOpenOptions = StorageOpenOptions(1u32);
    pub const AllowReadersAndWriters: StorageOpenOptions = StorageOpenOptions(2u32);
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
    pub const Failed: StreamedFileFailureMode = StreamedFileFailureMode(0i32);
    pub const CurrentlyUnavailable: StreamedFileFailureMode = StreamedFileFailureMode(1i32);
    pub const Incomplete: StreamedFileFailureMode = StreamedFileFailureMode(2i32);
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
