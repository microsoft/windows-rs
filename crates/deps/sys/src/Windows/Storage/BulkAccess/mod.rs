#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FileInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileInformation {}
impl ::core::clone::Clone for FileInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileInformationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileInformationFactory {}
impl ::core::clone::Clone for FileInformationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FolderInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FolderInformation {}
impl ::core::clone::Clone for FolderInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileInformationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileInformationFactory {}
impl ::core::clone::Clone for IFileInformationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileInformationFactoryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileInformationFactoryFactory {}
impl ::core::clone::Clone for IFileInformationFactoryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemInformation {}
impl ::core::clone::Clone for IStorageItemInformation {
    fn clone(&self) -> Self {
        *self
    }
}
