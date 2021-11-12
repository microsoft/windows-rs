#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPnpObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPnpObjectStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPnpObjectUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPnpObjectWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObjectCollection(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PnpObjectType(i32);
#[repr(transparent)]
pub struct PnpObjectUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObjectWatcher(pub *mut ::core::ffi::c_void);
