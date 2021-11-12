#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportImportItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportItemImportedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSidecar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportStorageMedium(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportVideoSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportAccessMode(pub i32);
impl PhotoImportAccessMode {
    pub const ReadWrite: PhotoImportAccessMode = PhotoImportAccessMode(0i32);
    pub const ReadOnly: PhotoImportAccessMode = PhotoImportAccessMode(1i32);
    pub const ReadAndDelete: PhotoImportAccessMode = PhotoImportAccessMode(2i32);
}
#[repr(transparent)]
pub struct PhotoImportConnectionTransport(pub i32);
impl PhotoImportConnectionTransport {
    pub const Unknown: PhotoImportConnectionTransport = PhotoImportConnectionTransport(0i32);
    pub const Usb: PhotoImportConnectionTransport = PhotoImportConnectionTransport(1i32);
    pub const IP: PhotoImportConnectionTransport = PhotoImportConnectionTransport(2i32);
    pub const Bluetooth: PhotoImportConnectionTransport = PhotoImportConnectionTransport(3i32);
}
#[repr(transparent)]
pub struct PhotoImportContentType(pub i32);
impl PhotoImportContentType {
    pub const Unknown: PhotoImportContentType = PhotoImportContentType(0i32);
    pub const Image: PhotoImportContentType = PhotoImportContentType(1i32);
    pub const Video: PhotoImportContentType = PhotoImportContentType(2i32);
}
#[repr(transparent)]
pub struct PhotoImportContentTypeFilter(pub i32);
impl PhotoImportContentTypeFilter {
    pub const OnlyImages: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(0i32);
    pub const OnlyVideos: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(1i32);
    pub const ImagesAndVideos: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(2i32);
    pub const ImagesAndVideosFromCameraRoll: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(3i32);
}
#[repr(transparent)]
pub struct PhotoImportDeleteImportedItemsFromSourceResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportFindItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportImportItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportImportMode(pub i32);
impl PhotoImportImportMode {
    pub const ImportEverything: PhotoImportImportMode = PhotoImportImportMode(0i32);
    pub const IgnoreSidecars: PhotoImportImportMode = PhotoImportImportMode(1i32);
    pub const IgnoreSiblings: PhotoImportImportMode = PhotoImportImportMode(2i32);
    pub const IgnoreSidecarsAndSiblings: PhotoImportImportMode = PhotoImportImportMode(3i32);
}
#[repr(transparent)]
pub struct PhotoImportItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportItemImportedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportItemSelectionMode(pub i32);
impl PhotoImportItemSelectionMode {
    pub const SelectAll: PhotoImportItemSelectionMode = PhotoImportItemSelectionMode(0i32);
    pub const SelectNone: PhotoImportItemSelectionMode = PhotoImportItemSelectionMode(1i32);
    pub const SelectNew: PhotoImportItemSelectionMode = PhotoImportItemSelectionMode(2i32);
}
#[repr(transparent)]
pub struct PhotoImportOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportPowerSource(pub i32);
impl PhotoImportPowerSource {
    pub const Unknown: PhotoImportPowerSource = PhotoImportPowerSource(0i32);
    pub const Battery: PhotoImportPowerSource = PhotoImportPowerSource(1i32);
    pub const External: PhotoImportPowerSource = PhotoImportPowerSource(2i32);
}
#[repr(C)]
pub struct PhotoImportProgress(i32);
#[repr(transparent)]
pub struct PhotoImportSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSidecar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSourceType(pub i32);
impl PhotoImportSourceType {
    pub const Generic: PhotoImportSourceType = PhotoImportSourceType(0i32);
    pub const Camera: PhotoImportSourceType = PhotoImportSourceType(1i32);
    pub const MediaPlayer: PhotoImportSourceType = PhotoImportSourceType(2i32);
    pub const Phone: PhotoImportSourceType = PhotoImportSourceType(3i32);
    pub const Video: PhotoImportSourceType = PhotoImportSourceType(4i32);
    pub const PersonalInfoManager: PhotoImportSourceType = PhotoImportSourceType(5i32);
    pub const AudioRecorder: PhotoImportSourceType = PhotoImportSourceType(6i32);
}
#[repr(transparent)]
pub struct PhotoImportStage(pub i32);
impl PhotoImportStage {
    pub const NotStarted: PhotoImportStage = PhotoImportStage(0i32);
    pub const FindingItems: PhotoImportStage = PhotoImportStage(1i32);
    pub const ImportingItems: PhotoImportStage = PhotoImportStage(2i32);
    pub const DeletingImportedItemsFromSource: PhotoImportStage = PhotoImportStage(3i32);
}
#[repr(transparent)]
pub struct PhotoImportStorageMedium(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportStorageMediumType(pub i32);
impl PhotoImportStorageMediumType {
    pub const Undefined: PhotoImportStorageMediumType = PhotoImportStorageMediumType(0i32);
    pub const Fixed: PhotoImportStorageMediumType = PhotoImportStorageMediumType(1i32);
    pub const Removable: PhotoImportStorageMediumType = PhotoImportStorageMediumType(2i32);
}
#[repr(transparent)]
pub struct PhotoImportSubfolderCreationMode(pub i32);
impl PhotoImportSubfolderCreationMode {
    pub const DoNotCreateSubfolders: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(0i32);
    pub const CreateSubfoldersFromFileDate: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(1i32);
    pub const CreateSubfoldersFromExifDate: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(2i32);
    pub const KeepOriginalFolderStructure: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(3i32);
}
#[repr(transparent)]
pub struct PhotoImportSubfolderDateFormat(pub i32);
impl PhotoImportSubfolderDateFormat {
    pub const Year: PhotoImportSubfolderDateFormat = PhotoImportSubfolderDateFormat(0i32);
    pub const YearMonth: PhotoImportSubfolderDateFormat = PhotoImportSubfolderDateFormat(1i32);
    pub const YearMonthDay: PhotoImportSubfolderDateFormat = PhotoImportSubfolderDateFormat(2i32);
}
#[repr(transparent)]
pub struct PhotoImportVideoSegment(pub *mut ::core::ffi::c_void);
