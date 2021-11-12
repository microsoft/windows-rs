#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppExtension(i32);
pub struct AppExtensionCatalog(i32);
pub struct AppExtensionPackageInstalledEventArgs(i32);
pub struct AppExtensionPackageStatusChangedEventArgs(i32);
pub struct AppExtensionPackageUninstallingEventArgs(i32);
pub struct AppExtensionPackageUpdatedEventArgs(i32);
pub struct AppExtensionPackageUpdatingEventArgs(i32);
pub struct IAppExtension(pub *mut ::core::ffi::c_void);
pub struct IAppExtension2(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionCatalog(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionCatalogStatics(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionPackageInstalledEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionPackageUpdatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppExtensionPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
