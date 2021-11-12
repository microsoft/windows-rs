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
impl ::core::marker::Copy for AppDataPaths {}
impl ::core::clone::Clone for AppDataPaths {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationData {}
impl ::core::clone::Clone for ApplicationData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationDataCompositeValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationDataCompositeValue {}
impl ::core::clone::Clone for ApplicationDataCompositeValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationDataContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationDataContainer {}
impl ::core::clone::Clone for ApplicationDataContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationDataContainerSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationDataContainerSettings {}
impl ::core::clone::Clone for ApplicationDataContainerSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationDataCreateDisposition(pub i32);
impl ApplicationDataCreateDisposition {
    pub const Always: Self = Self(0i32);
    pub const Existing: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationDataCreateDisposition {}
impl ::core::clone::Clone for ApplicationDataCreateDisposition {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for ApplicationDataLocality {}
impl ::core::clone::Clone for ApplicationDataLocality {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationDataSetVersionHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationDataSetVersionHandler {}
impl ::core::clone::Clone for ApplicationDataSetVersionHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreationCollisionOption(pub i32);
impl CreationCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
    pub const OpenIfExists: Self = Self(3i32);
}
impl ::core::marker::Copy for CreationCollisionOption {}
impl ::core::clone::Clone for CreationCollisionOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileAccessMode(pub i32);
impl FileAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for FileAccessMode {}
impl ::core::clone::Clone for FileAccessMode {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for FileAttributes {}
impl ::core::clone::Clone for FileAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDataPaths(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDataPaths {}
impl ::core::clone::Clone for IAppDataPaths {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDataPathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDataPathsStatics {}
impl ::core::clone::Clone for IAppDataPathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationData {}
impl ::core::clone::Clone for IApplicationData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationData2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationData2 {}
impl ::core::clone::Clone for IApplicationData2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationData3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationData3 {}
impl ::core::clone::Clone for IApplicationData3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationDataContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationDataContainer {}
impl ::core::clone::Clone for IApplicationDataContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationDataStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationDataStatics {}
impl ::core::clone::Clone for IApplicationDataStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationDataStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationDataStatics2 {}
impl ::core::clone::Clone for IApplicationDataStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileManagerStatics {}
impl ::core::clone::Clone for ICachedFileManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDownloadsFolderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDownloadsFolderStatics {}
impl ::core::clone::Clone for IDownloadsFolderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDownloadsFolderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDownloadsFolderStatics2 {}
impl ::core::clone::Clone for IDownloadsFolderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileIOStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileIOStatics {}
impl ::core::clone::Clone for IFileIOStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersCameraRollStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersCameraRollStatics {}
impl ::core::clone::Clone for IKnownFoldersCameraRollStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersPlaylistsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersPlaylistsStatics {}
impl ::core::clone::Clone for IKnownFoldersPlaylistsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersSavedPicturesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersSavedPicturesStatics {}
impl ::core::clone::Clone for IKnownFoldersSavedPicturesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersStatics {}
impl ::core::clone::Clone for IKnownFoldersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersStatics2 {}
impl ::core::clone::Clone for IKnownFoldersStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersStatics3 {}
impl ::core::clone::Clone for IKnownFoldersStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownFoldersStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownFoldersStatics4 {}
impl ::core::clone::Clone for IKnownFoldersStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIOStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIOStatics {}
impl ::core::clone::Clone for IPathIOStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetVersionDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetVersionDeferral {}
impl ::core::clone::Clone for ISetVersionDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetVersionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetVersionRequest {}
impl ::core::clone::Clone for ISetVersionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFile {}
impl ::core::clone::Clone for IStorageFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFile2 {}
impl ::core::clone::Clone for IStorageFile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFilePropertiesWithAvailability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFilePropertiesWithAvailability {}
impl ::core::clone::Clone for IStorageFilePropertiesWithAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFileStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFileStatics {}
impl ::core::clone::Clone for IStorageFileStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFileStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFileStatics2 {}
impl ::core::clone::Clone for IStorageFileStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolder {}
impl ::core::clone::Clone for IStorageFolder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolder2 {}
impl ::core::clone::Clone for IStorageFolder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolder3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolder3 {}
impl ::core::clone::Clone for IStorageFolder3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolderStatics {}
impl ::core::clone::Clone for IStorageFolderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolderStatics2 {}
impl ::core::clone::Clone for IStorageFolderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItem {}
impl ::core::clone::Clone for IStorageItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItem2 {}
impl ::core::clone::Clone for IStorageItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemProperties {}
impl ::core::clone::Clone for IStorageItemProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemProperties2 {}
impl ::core::clone::Clone for IStorageItemProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemPropertiesWithProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemPropertiesWithProvider {}
impl ::core::clone::Clone for IStorageItemPropertiesWithProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibrary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibrary {}
impl ::core::clone::Clone for IStorageLibrary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibrary2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibrary2 {}
impl ::core::clone::Clone for IStorageLibrary2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibrary3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibrary3 {}
impl ::core::clone::Clone for IStorageLibrary3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChange {}
impl ::core::clone::Clone for IStorageLibraryChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeReader {}
impl ::core::clone::Clone for IStorageLibraryChangeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeReader2 {}
impl ::core::clone::Clone for IStorageLibraryChangeReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeTracker {}
impl ::core::clone::Clone for IStorageLibraryChangeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeTracker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeTracker2 {}
impl ::core::clone::Clone for IStorageLibraryChangeTracker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeTrackerOptions {}
impl ::core::clone::Clone for IStorageLibraryChangeTrackerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryLastChangeId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryLastChangeId {}
impl ::core::clone::Clone for IStorageLibraryLastChangeId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryLastChangeIdStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryLastChangeIdStatics {}
impl ::core::clone::Clone for IStorageLibraryLastChangeIdStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryStatics {}
impl ::core::clone::Clone for IStorageLibraryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryStatics2 {}
impl ::core::clone::Clone for IStorageLibraryStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProvider {}
impl ::core::clone::Clone for IStorageProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageProvider2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageProvider2 {}
impl ::core::clone::Clone for IStorageProvider2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageStreamTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageStreamTransaction {}
impl ::core::clone::Clone for IStorageStreamTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamedFileDataRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamedFileDataRequest {}
impl ::core::clone::Clone for IStreamedFileDataRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemAudioProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemAudioProperties {}
impl ::core::clone::Clone for ISystemAudioProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemDataPaths(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemDataPaths {}
impl ::core::clone::Clone for ISystemDataPaths {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemDataPathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemDataPathsStatics {}
impl ::core::clone::Clone for ISystemDataPathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemGPSProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemGPSProperties {}
impl ::core::clone::Clone for ISystemGPSProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemImageProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemImageProperties {}
impl ::core::clone::Clone for ISystemImageProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaProperties {}
impl ::core::clone::Clone for ISystemMediaProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMusicProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMusicProperties {}
impl ::core::clone::Clone for ISystemMusicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemPhotoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemPhotoProperties {}
impl ::core::clone::Clone for ISystemPhotoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemProperties {}
impl ::core::clone::Clone for ISystemProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemVideoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemVideoProperties {}
impl ::core::clone::Clone for ISystemVideoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataPaths(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataPaths {}
impl ::core::clone::Clone for IUserDataPaths {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataPathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataPathsStatics {}
impl ::core::clone::Clone for IUserDataPathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for KnownFolderId {}
impl ::core::clone::Clone for KnownFolderId {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for KnownFoldersAccessStatus {}
impl ::core::clone::Clone for KnownFoldersAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KnownLibraryId(pub i32);
impl KnownLibraryId {
    pub const Music: Self = Self(0i32);
    pub const Pictures: Self = Self(1i32);
    pub const Videos: Self = Self(2i32);
    pub const Documents: Self = Self(3i32);
}
impl ::core::marker::Copy for KnownLibraryId {}
impl ::core::clone::Clone for KnownLibraryId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NameCollisionOption(pub i32);
impl NameCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
}
impl ::core::marker::Copy for NameCollisionOption {}
impl ::core::clone::Clone for NameCollisionOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetVersionDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SetVersionDeferral {}
impl ::core::clone::Clone for SetVersionDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetVersionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SetVersionRequest {}
impl ::core::clone::Clone for SetVersionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageDeleteOption(pub i32);
impl StorageDeleteOption {
    pub const Default: Self = Self(0i32);
    pub const PermanentDelete: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageDeleteOption {}
impl ::core::clone::Clone for StorageDeleteOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageFile {}
impl ::core::clone::Clone for StorageFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageFolder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageFolder {}
impl ::core::clone::Clone for StorageFolder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageItemTypes(pub u32);
impl StorageItemTypes {
    pub const None: Self = Self(0u32);
    pub const File: Self = Self(1u32);
    pub const Folder: Self = Self(2u32);
}
impl ::core::marker::Copy for StorageItemTypes {}
impl ::core::clone::Clone for StorageItemTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibrary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibrary {}
impl ::core::clone::Clone for StorageLibrary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryChange {}
impl ::core::clone::Clone for StorageLibraryChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryChangeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryChangeReader {}
impl ::core::clone::Clone for StorageLibraryChangeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryChangeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryChangeTracker {}
impl ::core::clone::Clone for StorageLibraryChangeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryChangeTrackerOptions {}
impl ::core::clone::Clone for StorageLibraryChangeTrackerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for StorageLibraryChangeType {}
impl ::core::clone::Clone for StorageLibraryChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryLastChangeId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryLastChangeId {}
impl ::core::clone::Clone for StorageLibraryLastChangeId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageOpenOptions(pub u32);
impl StorageOpenOptions {
    pub const None: Self = Self(0u32);
    pub const AllowOnlyReaders: Self = Self(1u32);
    pub const AllowReadersAndWriters: Self = Self(2u32);
}
impl ::core::marker::Copy for StorageOpenOptions {}
impl ::core::clone::Clone for StorageOpenOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageProvider {}
impl ::core::clone::Clone for StorageProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageStreamTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageStreamTransaction {}
impl ::core::clone::Clone for StorageStreamTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamedFileDataRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamedFileDataRequest {}
impl ::core::clone::Clone for StreamedFileDataRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamedFileDataRequestedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StreamedFileDataRequestedHandler {}
impl ::core::clone::Clone for StreamedFileDataRequestedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamedFileFailureMode(pub i32);
impl StreamedFileFailureMode {
    pub const Failed: Self = Self(0i32);
    pub const CurrentlyUnavailable: Self = Self(1i32);
    pub const Incomplete: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamedFileFailureMode {}
impl ::core::clone::Clone for StreamedFileFailureMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemAudioProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemAudioProperties {}
impl ::core::clone::Clone for SystemAudioProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemDataPaths(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemDataPaths {}
impl ::core::clone::Clone for SystemDataPaths {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemGPSProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemGPSProperties {}
impl ::core::clone::Clone for SystemGPSProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemImageProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemImageProperties {}
impl ::core::clone::Clone for SystemImageProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMediaProperties {}
impl ::core::clone::Clone for SystemMediaProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMusicProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMusicProperties {}
impl ::core::clone::Clone for SystemMusicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemPhotoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemPhotoProperties {}
impl ::core::clone::Clone for SystemPhotoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemVideoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemVideoProperties {}
impl ::core::clone::Clone for SystemVideoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataPaths(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataPaths {}
impl ::core::clone::Clone for UserDataPaths {
    fn clone(&self) -> Self {
        *self
    }
}
