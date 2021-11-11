#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppExtension();
    fn AppExtensionCatalog();
    fn AppExtensionPackageInstalledEventArgs();
    fn AppExtensionPackageStatusChangedEventArgs();
    fn AppExtensionPackageUninstallingEventArgs();
    fn AppExtensionPackageUpdatedEventArgs();
    fn AppExtensionPackageUpdatingEventArgs();
    fn IAppExtension();
    fn IAppExtension2();
    fn IAppExtensionCatalog();
    fn IAppExtensionCatalogStatics();
    fn IAppExtensionPackageInstalledEventArgs();
    fn IAppExtensionPackageStatusChangedEventArgs();
    fn IAppExtensionPackageUninstallingEventArgs();
    fn IAppExtensionPackageUpdatedEventArgs();
    fn IAppExtensionPackageUpdatingEventArgs();
}
