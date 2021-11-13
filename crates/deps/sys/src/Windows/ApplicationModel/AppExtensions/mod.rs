#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtension {}
impl ::core::clone::Clone for AppExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExtensionCatalog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtensionCatalog {}
impl ::core::clone::Clone for AppExtensionCatalog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExtensionPackageInstalledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtensionPackageInstalledEventArgs {}
impl ::core::clone::Clone for AppExtensionPackageInstalledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExtensionPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtensionPackageStatusChangedEventArgs {}
impl ::core::clone::Clone for AppExtensionPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExtensionPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtensionPackageUninstallingEventArgs {}
impl ::core::clone::Clone for AppExtensionPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExtensionPackageUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtensionPackageUpdatedEventArgs {}
impl ::core::clone::Clone for AppExtensionPackageUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExtensionPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppExtensionPackageUpdatingEventArgs {}
impl ::core::clone::Clone for AppExtensionPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtension {}
impl ::core::clone::Clone for IAppExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtension2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtension2 {}
impl ::core::clone::Clone for IAppExtension2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionCatalog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionCatalog {}
impl ::core::clone::Clone for IAppExtensionCatalog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionCatalogStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionCatalogStatics {}
impl ::core::clone::Clone for IAppExtensionCatalogStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionPackageInstalledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionPackageInstalledEventArgs {}
impl ::core::clone::Clone for IAppExtensionPackageInstalledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionPackageStatusChangedEventArgs {}
impl ::core::clone::Clone for IAppExtensionPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionPackageUninstallingEventArgs {}
impl ::core::clone::Clone for IAppExtensionPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionPackageUpdatedEventArgs {}
impl ::core::clone::Clone for IAppExtensionPackageUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppExtensionPackageUpdatingEventArgs {}
impl ::core::clone::Clone for IAppExtensionPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
