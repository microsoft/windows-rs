#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWalletItemSystemStore(pub *mut ::core::ffi::c_void);
pub struct IWalletItemSystemStore2(pub *mut ::core::ffi::c_void);
pub struct IWalletManagerSystemStatics(pub *mut ::core::ffi::c_void);
pub struct WalletItemAppAssociation(i32);
pub struct WalletItemSystemStore(i32);
pub struct WalletManagerSystem(i32);
