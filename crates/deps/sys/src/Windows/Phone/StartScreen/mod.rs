#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DualSimTile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DualSimTile {}
impl ::core::clone::Clone for DualSimTile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDualSimTile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDualSimTile {}
impl ::core::clone::Clone for IDualSimTile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDualSimTileStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDualSimTileStatics {}
impl ::core::clone::Clone for IDualSimTileStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationManagerStatics3 {}
impl ::core::clone::Clone for IToastNotificationManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
