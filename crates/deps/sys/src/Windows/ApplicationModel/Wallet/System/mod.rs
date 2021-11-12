#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWalletItemSystemStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemSystemStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletManagerSystemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct WalletItemSystemStore(pub *mut ::core::ffi::c_void);
