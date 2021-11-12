#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IResourceLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceLoader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceLoaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceLoaderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceLoaderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceLoaderStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceLoaderStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceLoader(pub *mut ::core::ffi::c_void);
