#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct FileInformation(i32);
pub struct FileInformationFactory(i32);
pub struct FolderInformation(i32);
pub struct IFileInformationFactory(pub *mut ::core::ffi::c_void);
pub struct IFileInformationFactoryFactory(pub *mut ::core::ffi::c_void);
pub struct IStorageItemInformation(pub *mut ::core::ffi::c_void);
