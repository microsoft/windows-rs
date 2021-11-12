#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct FileExtensionVector(i32);
pub struct FileOpenPicker(i32);
pub struct FilePickerFileTypesOrderedMap(i32);
pub struct FilePickerSelectedFilesArray(i32);
pub struct FileSavePicker(i32);
pub struct FolderPicker(i32);
pub struct IFileOpenPicker(pub *mut ::core::ffi::c_void);
pub struct IFileOpenPicker2(pub *mut ::core::ffi::c_void);
pub struct IFileOpenPicker3(pub *mut ::core::ffi::c_void);
pub struct IFileOpenPickerStatics(pub *mut ::core::ffi::c_void);
pub struct IFileOpenPickerStatics2(pub *mut ::core::ffi::c_void);
pub struct IFileOpenPickerWithOperationId(pub *mut ::core::ffi::c_void);
pub struct IFileSavePicker(pub *mut ::core::ffi::c_void);
pub struct IFileSavePicker2(pub *mut ::core::ffi::c_void);
pub struct IFileSavePicker3(pub *mut ::core::ffi::c_void);
pub struct IFileSavePicker4(pub *mut ::core::ffi::c_void);
pub struct IFileSavePickerStatics(pub *mut ::core::ffi::c_void);
pub struct IFolderPicker(pub *mut ::core::ffi::c_void);
pub struct IFolderPicker2(pub *mut ::core::ffi::c_void);
pub struct IFolderPicker3(pub *mut ::core::ffi::c_void);
pub struct IFolderPickerStatics(pub *mut ::core::ffi::c_void);
pub struct PickerLocationId(i32);
pub struct PickerViewMode(i32);
