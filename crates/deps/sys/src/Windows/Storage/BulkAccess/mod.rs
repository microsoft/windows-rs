#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FileInformation();
    fn FileInformationFactory();
    fn FolderInformation();
    fn IFileInformationFactory();
    fn IFileInformationFactoryFactory();
    fn IStorageItemInformation();
}
