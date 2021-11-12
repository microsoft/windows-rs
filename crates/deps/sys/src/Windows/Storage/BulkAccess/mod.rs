#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FileInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileInformationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileInformationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileInformationFactoryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemInformation(pub *mut ::core::ffi::c_void);
