#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {}
pub struct IResourceLoader(i32);
pub struct IResourceLoader2(i32);
pub struct IResourceLoaderFactory(i32);
pub struct IResourceLoaderStatics(i32);
pub struct IResourceLoaderStatics2(i32);
pub struct IResourceLoaderStatics3(i32);
pub struct IResourceLoaderStatics4(i32);
pub struct ResourceLoader(i32);
