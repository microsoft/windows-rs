#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AddFileResult(i32);
#[repr(transparent)]
pub struct FileOpenPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileSavePickerUI(pub *mut ::core::ffi::c_void);
pub struct FileSelectionMode(i32);
#[repr(transparent)]
pub struct IFileOpenPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSavePickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerClosingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerClosingOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFileRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFileRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFileRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerClosingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerClosingOperation(pub *mut ::core::ffi::c_void);
pub struct SetFileNameResult(i32);
#[repr(transparent)]
pub struct TargetFileRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetFileRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetFileRequestedEventArgs(pub *mut ::core::ffi::c_void);
