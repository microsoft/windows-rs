#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddFileResult();
    fn FileOpenPickerUI();
    fn FileRemovedEventArgs();
    fn FileSavePickerUI();
    fn FileSelectionMode();
    fn IFileOpenPickerUI();
    fn IFileRemovedEventArgs();
    fn IFileSavePickerUI();
    fn IPickerClosingDeferral();
    fn IPickerClosingEventArgs();
    fn IPickerClosingOperation();
    fn ITargetFileRequest();
    fn ITargetFileRequestDeferral();
    fn ITargetFileRequestedEventArgs();
    fn PickerClosingDeferral();
    fn PickerClosingEventArgs();
    fn PickerClosingOperation();
    fn SetFileNameResult();
    fn TargetFileRequest();
    fn TargetFileRequestDeferral();
    fn TargetFileRequestedEventArgs();
}
