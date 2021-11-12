#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DataProtectionProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataProtectionProvider {}
impl ::core::clone::Clone for DataProtectionProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataProtectionProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataProtectionProvider {}
impl ::core::clone::Clone for IDataProtectionProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataProtectionProviderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataProtectionProviderFactory {}
impl ::core::clone::Clone for IDataProtectionProviderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
