#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AddFileResult(i32);
pub struct FileOpenPickerUI(i32);
pub struct FileRemovedEventArgs(i32);
pub struct FileSavePickerUI(i32);
pub struct FileSelectionMode(i32);
pub struct IFileOpenPickerUI(pub *mut ::core::ffi::c_void);
pub struct IFileRemovedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IFileSavePickerUI(pub *mut ::core::ffi::c_void);
pub struct IPickerClosingDeferral(pub *mut ::core::ffi::c_void);
pub struct IPickerClosingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPickerClosingOperation(pub *mut ::core::ffi::c_void);
pub struct ITargetFileRequest(pub *mut ::core::ffi::c_void);
pub struct ITargetFileRequestDeferral(pub *mut ::core::ffi::c_void);
pub struct ITargetFileRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct PickerClosingDeferral(i32);
pub struct PickerClosingEventArgs(i32);
pub struct PickerClosingOperation(i32);
pub struct SetFileNameResult(i32);
pub struct TargetFileRequest(i32);
pub struct TargetFileRequestDeferral(i32);
pub struct TargetFileRequestedEventArgs(i32);
