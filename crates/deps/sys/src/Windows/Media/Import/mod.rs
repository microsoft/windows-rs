#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportDeleteImportedItemsFromSourceResult {}
impl ::core::clone::Clone for IPhotoImportDeleteImportedItemsFromSourceResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportFindItemsResult {}
impl ::core::clone::Clone for IPhotoImportFindItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportFindItemsResult2 {}
impl ::core::clone::Clone for IPhotoImportFindItemsResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportImportItemsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportImportItemsResult {}
impl ::core::clone::Clone for IPhotoImportImportItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportItem {}
impl ::core::clone::Clone for IPhotoImportItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportItem2 {}
impl ::core::clone::Clone for IPhotoImportItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportItemImportedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportItemImportedEventArgs {}
impl ::core::clone::Clone for IPhotoImportItemImportedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportManagerStatics {}
impl ::core::clone::Clone for IPhotoImportManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportOperation {}
impl ::core::clone::Clone for IPhotoImportOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportSelectionChangedEventArgs {}
impl ::core::clone::Clone for IPhotoImportSelectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportSession {}
impl ::core::clone::Clone for IPhotoImportSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportSession2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportSession2 {}
impl ::core::clone::Clone for IPhotoImportSession2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportSidecar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportSidecar {}
impl ::core::clone::Clone for IPhotoImportSidecar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportSource {}
impl ::core::clone::Clone for IPhotoImportSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportSourceStatics {}
impl ::core::clone::Clone for IPhotoImportSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportStorageMedium(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportStorageMedium {}
impl ::core::clone::Clone for IPhotoImportStorageMedium {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoImportVideoSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoImportVideoSegment {}
impl ::core::clone::Clone for IPhotoImportVideoSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportAccessMode(pub i32);
impl PhotoImportAccessMode {
    pub const ReadWrite: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
    pub const ReadAndDelete: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportAccessMode {}
impl ::core::clone::Clone for PhotoImportAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportConnectionTransport(pub i32);
impl PhotoImportConnectionTransport {
    pub const Unknown: Self = Self(0i32);
    pub const Usb: Self = Self(1i32);
    pub const IP: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportConnectionTransport {}
impl ::core::clone::Clone for PhotoImportConnectionTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportContentType(pub i32);
impl PhotoImportContentType {
    pub const Unknown: Self = Self(0i32);
    pub const Image: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportContentType {}
impl ::core::clone::Clone for PhotoImportContentType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportContentTypeFilter(pub i32);
impl PhotoImportContentTypeFilter {
    pub const OnlyImages: Self = Self(0i32);
    pub const OnlyVideos: Self = Self(1i32);
    pub const ImagesAndVideos: Self = Self(2i32);
    pub const ImagesAndVideosFromCameraRoll: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportContentTypeFilter {}
impl ::core::clone::Clone for PhotoImportContentTypeFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportDeleteImportedItemsFromSourceResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportDeleteImportedItemsFromSourceResult {}
impl ::core::clone::Clone for PhotoImportDeleteImportedItemsFromSourceResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportFindItemsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportFindItemsResult {}
impl ::core::clone::Clone for PhotoImportFindItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportImportItemsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportImportItemsResult {}
impl ::core::clone::Clone for PhotoImportImportItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportImportMode(pub i32);
impl PhotoImportImportMode {
    pub const ImportEverything: Self = Self(0i32);
    pub const IgnoreSidecars: Self = Self(1i32);
    pub const IgnoreSiblings: Self = Self(2i32);
    pub const IgnoreSidecarsAndSiblings: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportImportMode {}
impl ::core::clone::Clone for PhotoImportImportMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportItem {}
impl ::core::clone::Clone for PhotoImportItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportItemImportedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportItemImportedEventArgs {}
impl ::core::clone::Clone for PhotoImportItemImportedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportItemSelectionMode(pub i32);
impl PhotoImportItemSelectionMode {
    pub const SelectAll: Self = Self(0i32);
    pub const SelectNone: Self = Self(1i32);
    pub const SelectNew: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportItemSelectionMode {}
impl ::core::clone::Clone for PhotoImportItemSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportOperation {}
impl ::core::clone::Clone for PhotoImportOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportPowerSource(pub i32);
impl PhotoImportPowerSource {
    pub const Unknown: Self = Self(0i32);
    pub const Battery: Self = Self(1i32);
    pub const External: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportPowerSource {}
impl ::core::clone::Clone for PhotoImportPowerSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PhotoImportProgress {
    pub ItemsImported: u32,
    pub TotalItemsToImport: u32,
    pub BytesImported: u64,
    pub TotalBytesToImport: u64,
    pub ImportProgress: f64,
}
impl ::core::marker::Copy for PhotoImportProgress {}
impl ::core::clone::Clone for PhotoImportProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportSelectionChangedEventArgs {}
impl ::core::clone::Clone for PhotoImportSelectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportSession {}
impl ::core::clone::Clone for PhotoImportSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSidecar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportSidecar {}
impl ::core::clone::Clone for PhotoImportSidecar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportSource {}
impl ::core::clone::Clone for PhotoImportSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSourceType(pub i32);
impl PhotoImportSourceType {
    pub const Generic: Self = Self(0i32);
    pub const Camera: Self = Self(1i32);
    pub const MediaPlayer: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const PersonalInfoManager: Self = Self(5i32);
    pub const AudioRecorder: Self = Self(6i32);
}
impl ::core::marker::Copy for PhotoImportSourceType {}
impl ::core::clone::Clone for PhotoImportSourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportStage(pub i32);
impl PhotoImportStage {
    pub const NotStarted: Self = Self(0i32);
    pub const FindingItems: Self = Self(1i32);
    pub const ImportingItems: Self = Self(2i32);
    pub const DeletingImportedItemsFromSource: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportStage {}
impl ::core::clone::Clone for PhotoImportStage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportStorageMedium(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportStorageMedium {}
impl ::core::clone::Clone for PhotoImportStorageMedium {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportStorageMediumType(pub i32);
impl PhotoImportStorageMediumType {
    pub const Undefined: Self = Self(0i32);
    pub const Fixed: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportStorageMediumType {}
impl ::core::clone::Clone for PhotoImportStorageMediumType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSubfolderCreationMode(pub i32);
impl PhotoImportSubfolderCreationMode {
    pub const DoNotCreateSubfolders: Self = Self(0i32);
    pub const CreateSubfoldersFromFileDate: Self = Self(1i32);
    pub const CreateSubfoldersFromExifDate: Self = Self(2i32);
    pub const KeepOriginalFolderStructure: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderCreationMode {}
impl ::core::clone::Clone for PhotoImportSubfolderCreationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportSubfolderDateFormat(pub i32);
impl PhotoImportSubfolderDateFormat {
    pub const Year: Self = Self(0i32);
    pub const YearMonth: Self = Self(1i32);
    pub const YearMonthDay: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderDateFormat {}
impl ::core::clone::Clone for PhotoImportSubfolderDateFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoImportVideoSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoImportVideoSegment {}
impl ::core::clone::Clone for PhotoImportVideoSegment {
    fn clone(&self) -> Self {
        *self
    }
}
