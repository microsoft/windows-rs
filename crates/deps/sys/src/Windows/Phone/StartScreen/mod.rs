#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DualSimTile(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DualSimTileContract(i32);
#[repr(transparent)]
pub struct IDualSimTile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDualSimTileStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationManagerStatics3(pub *mut ::core::ffi::c_void);
