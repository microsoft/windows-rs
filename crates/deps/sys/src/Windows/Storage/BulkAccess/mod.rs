#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct FileInformation(i32);
pub struct FileInformationFactory(i32);
pub struct FolderInformation(i32);
pub struct IFileInformationFactory(i32);
pub struct IFileInformationFactoryFactory(i32);
pub struct IStorageItemInformation(i32);
