#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppExtensionCatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppExtensionPackageInstalledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppExtensionPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppExtensionPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppExtensionPackageUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppExtensionPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtension2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionCatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionCatalogStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionPackageInstalledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
