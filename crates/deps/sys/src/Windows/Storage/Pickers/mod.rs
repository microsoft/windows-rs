#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FileExtensionVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileOpenPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FilePickerFileTypesOrderedMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FilePickerSelectedFilesArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileSavePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPickerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenPickerWithOperationId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePicker4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderPicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderPicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerLocationId(pub i32);
impl PickerLocationId {
    pub const DocumentsLibrary: PickerLocationId = PickerLocationId(0i32);
    pub const ComputerFolder: PickerLocationId = PickerLocationId(1i32);
    pub const Desktop: PickerLocationId = PickerLocationId(2i32);
    pub const Downloads: PickerLocationId = PickerLocationId(3i32);
    pub const HomeGroup: PickerLocationId = PickerLocationId(4i32);
    pub const MusicLibrary: PickerLocationId = PickerLocationId(5i32);
    pub const PicturesLibrary: PickerLocationId = PickerLocationId(6i32);
    pub const VideosLibrary: PickerLocationId = PickerLocationId(7i32);
    pub const Objects3D: PickerLocationId = PickerLocationId(8i32);
    pub const Unspecified: PickerLocationId = PickerLocationId(9i32);
}
#[repr(transparent)]
pub struct PickerViewMode(pub i32);
impl PickerViewMode {
    pub const List: PickerViewMode = PickerViewMode(0i32);
    pub const Thumbnail: PickerViewMode = PickerViewMode(1i32);
}
