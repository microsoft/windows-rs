#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn FileExtensionVector();
    fn FileOpenPicker();
    fn FilePickerFileTypesOrderedMap();
    fn FilePickerSelectedFilesArray();
    fn FileSavePicker();
    fn FolderPicker();
    fn IFileOpenPicker();
    fn IFileOpenPicker2();
    fn IFileOpenPicker3();
    fn IFileOpenPickerStatics();
    fn IFileOpenPickerStatics2();
    fn IFileOpenPickerWithOperationId();
    fn IFileSavePicker();
    fn IFileSavePicker2();
    fn IFileSavePicker3();
    fn IFileSavePicker4();
    fn IFileSavePickerStatics();
    fn IFolderPicker();
    fn IFolderPicker2();
    fn IFolderPicker3();
    fn IFolderPickerStatics();
    fn PickerLocationId();
    fn PickerViewMode();
}
