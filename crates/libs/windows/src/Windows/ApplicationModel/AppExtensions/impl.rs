#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&self) -> ::windows::core::Result<super::Package>;
    fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo>;
    fn GetExtensionPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>>;
    fn GetPublicFolderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtension2Impl: Sized {
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionCatalogImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>>;
    fn RequestRemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn PackageInstalled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageInstalled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdating(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdating(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUninstalling(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageUninstalling(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionCatalogStaticsImpl: Sized {
    fn Open(&self, appextensionname: &::windows::core::HSTRING) -> ::windows::core::Result<AppExtensionCatalog>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageInstalledEventArgsImpl: Sized {
    fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&self) -> ::windows::core::Result<super::Package>;
    fn Extensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageStatusChangedEventArgsImpl: Sized {
    fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&self) -> ::windows::core::Result<super::Package>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageUninstallingEventArgsImpl: Sized {
    fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&self) -> ::windows::core::Result<super::Package>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageUpdatedEventArgsImpl: Sized {
    fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&self) -> ::windows::core::Result<super::Package>;
    fn Extensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageUpdatingEventArgsImpl: Sized {
    fn AppExtensionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&self) -> ::windows::core::Result<super::Package>;
}
