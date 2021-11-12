#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct PickerLocationId(i32);
#[repr(C)]
pub struct PickerViewMode(i32);
