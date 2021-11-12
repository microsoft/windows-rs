#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DualSimTile(i32);
pub struct DualSimTileContract(i32);
pub struct IDualSimTile(pub *mut ::core::ffi::c_void);
pub struct IDualSimTileStatics(pub *mut ::core::ffi::c_void);
pub struct IToastNotificationManagerStatics3(pub *mut ::core::ffi::c_void);
