#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AddContactResult(i32);
pub struct ContactPickerUI(i32);
pub struct ContactRemovedEventArgs(i32);
pub struct IContactPickerUI(pub *mut ::core::ffi::c_void);
pub struct IContactPickerUI2(pub *mut ::core::ffi::c_void);
pub struct IContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
