#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPnpObject(pub *mut ::core::ffi::c_void);
pub struct IPnpObjectStatics(pub *mut ::core::ffi::c_void);
pub struct IPnpObjectUpdate(pub *mut ::core::ffi::c_void);
pub struct IPnpObjectWatcher(pub *mut ::core::ffi::c_void);
pub struct PnpObject(i32);
pub struct PnpObjectCollection(i32);
pub struct PnpObjectType(i32);
pub struct PnpObjectUpdate(i32);
pub struct PnpObjectWatcher(i32);
