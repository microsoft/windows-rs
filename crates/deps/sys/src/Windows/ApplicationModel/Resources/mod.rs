#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
pub struct IResourceLoader(pub *mut ::core::ffi::c_void);
pub struct IResourceLoader2(pub *mut ::core::ffi::c_void);
pub struct IResourceLoaderFactory(pub *mut ::core::ffi::c_void);
pub struct IResourceLoaderStatics(pub *mut ::core::ffi::c_void);
pub struct IResourceLoaderStatics2(pub *mut ::core::ffi::c_void);
pub struct IResourceLoaderStatics3(pub *mut ::core::ffi::c_void);
pub struct IResourceLoaderStatics4(pub *mut ::core::ffi::c_void);
pub struct ResourceLoader(i32);
