#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AddContactResult(i32);
#[repr(transparent)]
pub struct ContactPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPickerUI2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
