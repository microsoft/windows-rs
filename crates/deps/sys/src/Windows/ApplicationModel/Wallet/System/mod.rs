#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWalletItemSystemStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWalletItemSystemStore {}
impl ::core::clone::Clone for IWalletItemSystemStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWalletItemSystemStore2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWalletItemSystemStore2 {}
impl ::core::clone::Clone for IWalletItemSystemStore2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWalletManagerSystemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWalletManagerSystemStatics {}
impl ::core::clone::Clone for IWalletManagerSystemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
impl ::core::marker::Copy for WalletItemAppAssociation {}
impl ::core::clone::Clone for WalletItemAppAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WalletItemSystemStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WalletItemSystemStore {}
impl ::core::clone::Clone for WalletItemSystemStore {
    fn clone(&self) -> Self {
        *self
    }
}
