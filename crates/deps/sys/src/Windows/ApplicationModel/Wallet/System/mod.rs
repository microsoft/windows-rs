#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWalletItemSystemStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemSystemStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletManagerSystemStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WalletItemAppAssociation(i32);
#[repr(transparent)]
pub struct WalletItemSystemStore(pub *mut ::core::ffi::c_void);
