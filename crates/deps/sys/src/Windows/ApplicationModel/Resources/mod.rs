#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IResourceLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoader {}
impl ::core::clone::Clone for IResourceLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceLoader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoader2 {}
impl ::core::clone::Clone for IResourceLoader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceLoaderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoaderFactory {}
impl ::core::clone::Clone for IResourceLoaderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceLoaderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoaderStatics {}
impl ::core::clone::Clone for IResourceLoaderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceLoaderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoaderStatics2 {}
impl ::core::clone::Clone for IResourceLoaderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceLoaderStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoaderStatics3 {}
impl ::core::clone::Clone for IResourceLoaderStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceLoaderStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceLoaderStatics4 {}
impl ::core::clone::Clone for IResourceLoaderStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceLoader {}
impl ::core::clone::Clone for ResourceLoader {
    fn clone(&self) -> Self {
        *self
    }
}
