#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[link(name = "windows")]
extern "system" {
    fn IResourceLoader();
    fn IResourceLoader2();
    fn IResourceLoaderFactory();
    fn IResourceLoaderStatics();
    fn IResourceLoaderStatics2();
    fn IResourceLoaderStatics3();
    fn IResourceLoaderStatics4();
    fn ResourceLoader();
}
