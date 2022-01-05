#[cfg(feature = "implement_exclusive")]
pub trait IAppDisplayInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLogo(&self, size: &super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayInfo(&self) -> ::windows::core::Result<AppDisplayInfo>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo2Impl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo3Impl: Sized {
    fn ExecutionContext(&self) -> ::windows::core::Result<AppExecutionContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo4Impl: Sized {
    fn SupportedFileExtensions(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfoStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<AppInfo>;
    fn GetFromAppUserModelId(&self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo>;
    fn GetFromAppUserModelIdForUser(&self, user: &::core::option::Option<super::System::User>, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallerInfoImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallerInfo2Impl: Sized {
    fn OnLaunch(&self) -> ::windows::core::Result<bool>;
    fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32>;
    fn ShowPrompt(&self) -> ::windows::core::Result<bool>;
    fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool>;
    fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool>;
    fn Version(&self) -> ::windows::core::Result<PackageVersion>;
    fn LastChecked(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn PausedUntil(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>>;
    fn UpdateUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn RepairUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn PolicySource(&self) -> ::windows::core::Result<AppInstallerPolicySource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstanceImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsCurrentInstance(&self) -> ::windows::core::Result<bool>;
    fn RedirectActivationTo(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstanceStaticsImpl: Sized {
    fn RecommendedInstance(&self) -> ::windows::core::Result<AppInstance>;
    fn GetActivatedEventArgs(&self) -> ::windows::core::Result<Activation::IActivatedEventArgs>;
    fn FindOrRegisterInstanceForKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<AppInstance>;
    fn Unregister(&self) -> ::windows::core::Result<()>;
    fn GetInstances(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppInstance>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraApplicationManagerStaticsImpl: Sized {
    fn ShowInstalledApplicationsUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignModeStaticsImpl: Sized {
    fn DesignModeEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignModeStatics2Impl: Sized {
    fn DesignMode2Enabled(&self) -> ::windows::core::Result<bool>;
}
pub trait IEnteredBackgroundEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullTrustProcessLaunchResultImpl: Sized {
    fn LaunchResult(&self) -> ::windows::core::Result<FullTrustLaunchResult>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullTrustProcessLauncherStaticsImpl: Sized {
    fn LaunchFullTrustProcessForCurrentAppAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForCurrentAppWithParametersAsync(&self, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForAppAsync(&self, fulltrustpackagerelativeappid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForAppWithParametersAsync(&self, fulltrustpackagerelativeappid: &::windows::core::HSTRING, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullTrustProcessLauncherStatics2Impl: Sized {
    fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(&self, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>;
    fn LaunchFullTrustProcessForAppWithArgumentsAsync(&self, fulltrustpackagerelativeappid: &::windows::core::HSTRING, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>;
}
pub trait ILeavingBackgroundEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILimitedAccessFeatureRequestResultImpl: Sized {
    fn FeatureId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<LimitedAccessFeatureStatus>;
    fn EstimatedRemovalDate(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILimitedAccessFeaturesStaticsImpl: Sized {
    fn TryUnlockFeature(&self, featureid: &::windows::core::HSTRING, token: &::windows::core::HSTRING, attestation: &::windows::core::HSTRING) -> ::windows::core::Result<LimitedAccessFeatureRequestResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<PackageId>;
    fn InstalledLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn IsFramework(&self) -> ::windows::core::Result<bool>;
    fn Dependencies(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage2Impl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublisherDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Logo(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn IsResourcePackage(&self) -> ::windows::core::Result<bool>;
    fn IsBundle(&self) -> ::windows::core::Result<bool>;
    fn IsDevelopmentMode(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage3Impl: Sized {
    fn Status(&self) -> ::windows::core::Result<PackageStatus>;
    fn InstalledDate(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn GetAppListEntriesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage4Impl: Sized {
    fn SignatureKind(&self) -> ::windows::core::Result<PackageSignatureKind>;
    fn IsOptional(&self) -> ::windows::core::Result<bool>;
    fn VerifyContentIntegrityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage5Impl: Sized {
    fn GetContentGroupsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn GetContentGroupAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageContentGroup>>;
    fn StageContentGroupsAsync(&self, names: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn StageContentGroupsWithPriorityAsync(&self, names: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, movetoheadofqueue: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn SetInUseAsync(&self, inuse: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage6Impl: Sized {
    fn GetAppInstallerInfo(&self) -> ::windows::core::Result<AppInstallerInfo>;
    fn CheckUpdateAvailabilityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage7Impl: Sized {
    fn MutableLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn EffectiveLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackage8Impl: Sized {
    fn EffectiveExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn MachineExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn UserExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn InstalledPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MutablePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EffectivePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EffectiveExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MachineExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLogoAsRandomAccessStreamReference(&self, size: &super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
    fn GetAppListEntries(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Core::AppListEntry>>;
    fn IsStub(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogImpl: Sized {
    fn PackageStaging(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageStaging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageInstalling(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageInstalling(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdating(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdating(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUninstalling(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageUninstalling(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageStatusChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageStatusChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalog2Impl: Sized {
    fn PackageContentGroupStaging(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageContentGroupStaging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddOptionalPackageAsync(&self, optionalpackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalog3Impl: Sized {
    fn RemoveOptionalPackagesAsync(&self, optionalpackagefamilynames: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalog4Impl: Sized {
    fn AddResourcePackageAsync(&self, resourcepackagefamilyname: &::windows::core::HSTRING, resourceid: &::windows::core::HSTRING, options: AddResourcePackageOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>>;
    fn RemoveResourcePackagesAsync(&self, resourcepackages: &::core::option::Option<super::Foundation::Collections::IIterable<Package>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogAddOptionalPackageResultImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogAddResourcePackageResultImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogRemoveOptionalPackagesResultImpl: Sized {
    fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogRemoveResourcePackagesResultImpl: Sized {
    fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogStaticsImpl: Sized {
    fn OpenForCurrentPackage(&self) -> ::windows::core::Result<PackageCatalog>;
    fn OpenForCurrentUser(&self) -> ::windows::core::Result<PackageCatalog>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<PackageContentGroupState>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupStagingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ContentGroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsContentGroupRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupStaticsImpl: Sized {
    fn RequiredGroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageIdImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<PackageVersion>;
    fn Architecture(&self) -> ::windows::core::Result<super::System::ProcessorArchitecture>;
    fn ResourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublisherId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageIdWithMetadataImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStagingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<Package>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatusImpl: Sized {
    fn VerifyIsOK(&self) -> ::windows::core::Result<bool>;
    fn NotAvailable(&self) -> ::windows::core::Result<bool>;
    fn PackageOffline(&self) -> ::windows::core::Result<bool>;
    fn DataOffline(&self) -> ::windows::core::Result<bool>;
    fn Disabled(&self) -> ::windows::core::Result<bool>;
    fn NeedsRemediation(&self) -> ::windows::core::Result<bool>;
    fn LicenseIssue(&self) -> ::windows::core::Result<bool>;
    fn Modified(&self) -> ::windows::core::Result<bool>;
    fn Tampered(&self) -> ::windows::core::Result<bool>;
    fn DependencyIssue(&self) -> ::windows::core::Result<bool>;
    fn Servicing(&self) -> ::windows::core::Result<bool>;
    fn DeploymentInProgress(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatus2Impl: Sized {
    fn IsPartiallyStaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatusChangedEventArgsImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUninstallingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUpdateAvailabilityResultImpl: Sized {
    fn Availability(&self) -> ::windows::core::Result<PackageUpdateAvailability>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUpdatingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SourcePackage(&self) -> ::windows::core::Result<Package>;
    fn TargetPackage(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageWithMetadataImpl: Sized {
    fn InstallDate(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn GetThumbnailToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Launch(&self, parameters: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStartupTaskImpl: Sized {
    fn RequestEnableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTaskState>>;
    fn Disable(&self) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<StartupTaskState>;
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStartupTaskStaticsImpl: Sized {
    fn GetForCurrentPackageAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>>;
    fn GetAsync(&self, taskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTask>>;
}
pub trait ISuspendingDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
pub trait ISuspendingEventArgsImpl: Sized {
    fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation>;
}
pub trait ISuspendingOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
}
