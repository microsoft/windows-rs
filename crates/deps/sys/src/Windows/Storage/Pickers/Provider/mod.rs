#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddFileResult(pub i32);
impl AddFileResult {
    pub const Added: AddFileResult = AddFileResult(0i32);
    pub const AlreadyAdded: AddFileResult = AddFileResult(1i32);
    pub const NotAllowed: AddFileResult = AddFileResult(2i32);
    pub const Unavailable: AddFileResult = AddFileResult(3i32);
}
#[repr(transparent)]
pub struct FileOpenPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileSavePickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileSelectionMode(pub i32);
impl FileSelectionMode {
    pub const Single: FileSelectionMode = FileSelectionMode(0i32);
    pub const Multiple: FileSelectionMode = FileSelectionMode(1i32);
}
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
#[repr(transparent)]
pub struct SetFileNameResult(pub i32);
impl SetFileNameResult {
    pub const Succeeded: SetFileNameResult = SetFileNameResult(0i32);
    pub const NotAllowed: SetFileNameResult = SetFileNameResult(1i32);
    pub const Unavailable: SetFileNameResult = SetFileNameResult(2i32);
}
#[repr(transparent)]
pub struct TargetFileRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetFileRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetFileRequestedEventArgs(pub *mut ::core::ffi::c_void);
