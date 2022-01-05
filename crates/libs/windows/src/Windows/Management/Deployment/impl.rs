#[cfg(feature = "implement_exclusive")]
pub trait IAddPackageOptionsImpl: Sized {
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn SetTargetVolume(&self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption>;
    fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()>;
    fn DeveloperMode(&self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool>;
    fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn RetainFilesOnFailure(&self) -> ::windows::core::Result<bool>;
    fn SetRetainFilesOnFailure(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()>;
    fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool>;
    fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallerManagerImpl: Sized {
    fn SetAutoUpdateSettings(&self, packagefamilyname: &::windows::core::HSTRING, appinstallerinfo: &::core::option::Option<AutoUpdateSettingsOptions>) -> ::windows::core::Result<()>;
    fn ClearAutoUpdateSettings(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseAutoUpdatesUntil(&self, packagefamilyname: &::windows::core::HSTRING, datetime: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallerManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppInstallerManager>;
    fn GetForSystem(&self) -> ::windows::core::Result<AppInstallerManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoUpdateSettingsOptionsImpl: Sized {
    fn Version(&self) -> ::windows::core::Result<super::super::ApplicationModel::PackageVersion>;
    fn SetVersion(&self, value: &super::super::ApplicationModel::PackageVersion) -> ::windows::core::Result<()>;
    fn AppInstallerUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetAppInstallerUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OnLaunch(&self) -> ::windows::core::Result<bool>;
    fn SetOnLaunch(&self, value: bool) -> ::windows::core::Result<()>;
    fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32>;
    fn SetHoursBetweenUpdateChecks(&self, value: u32) -> ::windows::core::Result<()>;
    fn ShowPrompt(&self) -> ::windows::core::Result<bool>;
    fn SetShowPrompt(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool>;
    fn SetUpdateBlocksActivation(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticBackgroundTask(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutoRepairEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RepairUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoUpdateSettingsOptionsStaticsImpl: Sized {
    fn CreateFromAppInstallerInfo(&self, appinstallerinfo: &::core::option::Option<super::super::ApplicationModel::AppInstallerInfo>) -> ::windows::core::Result<AutoUpdateSettingsOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateSharedPackageContainerOptionsImpl: Sized {
    fn Members(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>;
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn CreateCollisionOption(&self) -> ::windows::core::Result<SharedPackageContainerCreationCollisionOptions>;
    fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateSharedPackageContainerResultImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<SharedPackageContainer>;
    fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeleteSharedPackageContainerOptionsImpl: Sized {
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllUsers(&self) -> ::windows::core::Result<bool>;
    fn SetAllUsers(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeleteSharedPackageContainerResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeploymentResultImpl: Sized {
    fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeploymentResult2Impl: Sized {
    fn IsRegistered(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFindSharedPackageContainerOptionsImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageAllUserProvisioningOptionsImpl: Sized {
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProjectionOrderPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManagerImpl: Sized {
    fn AddPackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn UpdatePackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageAsync(&self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityId(&self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindUsers(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageUserInformation>>;
    fn SetPackageState(&self, packagefullname: &::windows::core::HSTRING, packagestate: PackageState) -> ::windows::core::Result<()>;
    fn FindPackageByPackageFullName(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn CleanupPackageForUserAsync(&self, packagename: &::windows::core::HSTRING, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager10Impl: Sized {
    fn ProvisionPackageForAllUsersWithOptionsAsync(&self, mainpackagefamilyname: &::windows::core::HSTRING, options: &::core::option::Option<PackageAllUserProvisioningOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager2Impl: Sized {
    fn RemovePackageWithOptionsAsync(&self, packagefullname: &::windows::core::HSTRING, removaloptions: RemovalOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageWithOptionsAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByFullNameAsync(&self, mainpackagefullname: &::windows::core::HSTRING, dependencypackagefullnames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisherWithPackageTypes(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn StageUserDataAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager3Impl: Sized {
    fn AddPackageVolumeAsync(&self, packagestorepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PackageVolume>>;
    fn AddPackageToVolumeAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn ClearPackageStatus(&self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()>;
    fn RegisterPackageWithAppDataVolumeAsync(&self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, appdatavolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackageVolumeByName(&self, volumename: &::windows::core::HSTRING) -> ::windows::core::Result<PackageVolume>;
    fn FindPackageVolumes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageVolume>>;
    fn GetDefaultPackageVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn MovePackageToVolumeAsync(&self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RemovePackageVolumeAsync(&self, volume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetDefaultPackageVolume(&self, volume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn SetPackageStatus(&self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()>;
    fn SetPackageVolumeOfflineAsync(&self, packagevolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetPackageVolumeOnlineAsync(&self, packagevolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StageUserDataWithOptionsAsync(&self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager4Impl: Sized {
    fn GetPackageVolumesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager5Impl: Sized {
    fn AddPackageToVolumeAndOptionalPackagesAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, externalpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAndOptionalPackagesAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, externalpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByFamilyNameAndOptionalPackagesAsync(&self, mainpackagefamilyname: &::windows::core::HSTRING, dependencypackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, deploymentoptions: DeploymentOptions, appdatavolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn DebugSettings(&self) -> ::windows::core::Result<PackageManagerDebugSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager6Impl: Sized {
    fn ProvisionPackageForAllUsersAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn AddPackageByAppInstallerFileAsync(&self, appinstallerfileuri: &::core::option::Option<super::super::Foundation::Uri>, options: AddPackageByAppInstallerOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RequestAddPackageByAppInstallerFileAsync(&self, appinstallerfileuri: &::core::option::Option<super::super::Foundation::Uri>, options: AddPackageByAppInstallerOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn AddPackageToVolumeAndRelatedSetAsync(
        &self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        options: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAndRelatedSetAsync(
        &self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        options: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RequestAddPackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager7Impl: Sized {
    fn RequestAddPackageAndRelatedSetAsync(
        &self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        deploymentoptions: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager8Impl: Sized {
    fn DeprovisionPackageForAllUsersAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManager9Impl: Sized {
    fn FindProvisionedPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn AddPackageByUriAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<AddPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageByUriAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<StagePackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByUriAsync(&self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<RegisterPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackagesByFullNameAsync(&self, packagefullnames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, options: &::core::option::Option<RegisterPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetPackageStubPreference(&self, packagefamilyname: &::windows::core::HSTRING, usestub: PackageStubPreference) -> ::windows::core::Result<()>;
    fn GetPackageStubPreference(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<PackageStubPreference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageManagerDebugSettingsImpl: Sized {
    fn SetContentGroupStateAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetContentGroupStateWithPercentageAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUserInformationImpl: Sized {
    fn UserSecurityId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallState(&self) -> ::windows::core::Result<PackageInstallState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageVolumeImpl: Sized {
    fn IsOffline(&self) -> ::windows::core::Result<bool>;
    fn IsSystemVolume(&self) -> ::windows::core::Result<bool>;
    fn MountPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageStorePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportsHardLinks(&self) -> ::windows::core::Result<bool>;
    fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisherWithPackagesTypes(&self, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackageByPackageFullName(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityId(&self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageVolume2Impl: Sized {
    fn IsFullTrustPackageSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAppxInstallSupported(&self) -> ::windows::core::Result<bool>;
    fn GetAvailableSpaceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegisterPackageOptionsImpl: Sized {
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn AppDataVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn SetAppDataVolume(&self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DeveloperMode(&self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()>;
    fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool>;
    fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetMembers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>;
    fn RemovePackageFamily(&self, packagefamilyname: &::windows::core::HSTRING, options: &::core::option::Option<UpdateSharedPackageContainerOptions>) -> ::windows::core::Result<UpdateSharedPackageContainerResult>;
    fn ResetData(&self) -> ::windows::core::Result<UpdateSharedPackageContainerResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerManagerImpl: Sized {
    fn CreateContainer(&self, name: &::windows::core::HSTRING, options: &::core::option::Option<CreateSharedPackageContainerOptions>) -> ::windows::core::Result<CreateSharedPackageContainerResult>;
    fn DeleteContainer(&self, id: &::windows::core::HSTRING, options: &::core::option::Option<DeleteSharedPackageContainerOptions>) -> ::windows::core::Result<DeleteSharedPackageContainerResult>;
    fn GetContainer(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainer>;
    fn FindContainers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>>;
    fn FindContainersWithOptions(&self, options: &::core::option::Option<FindSharedPackageContainerOptions>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SharedPackageContainerManager>;
    fn GetForUser(&self, usersid: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerManager>;
    fn GetForProvisioning(&self) -> ::windows::core::Result<SharedPackageContainerManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerMemberImpl: Sized {
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerMemberFactoryImpl: Sized {
    fn CreateInstance(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerMember>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStagePackageOptionsImpl: Sized {
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn SetTargetVolume(&self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption>;
    fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()>;
    fn DeveloperMode(&self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool>;
    fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUpdateSharedPackageContainerOptionsImpl: Sized {
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequirePackagesPresent(&self) -> ::windows::core::Result<bool>;
    fn SetRequirePackagesPresent(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUpdateSharedPackageContainerResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
