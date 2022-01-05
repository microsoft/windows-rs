#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItemImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallType(&self) -> ::windows::core::Result<AppInstallType>;
    fn IsUserInitiated(&self) -> ::windows::core::Result<bool>;
    fn GetCurrentStatus(&self) -> ::windows::core::Result<AppInstallStatus>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Restart(&self) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem2Impl: Sized {
    fn CancelWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RestartWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem3Impl: Sized {
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
    fn ItemOperationsMightAffectOtherItems(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem4Impl: Sized {
    fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem5Impl: Sized {
    fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManagerImpl: Sized {
    fn AppInstallItems(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
    fn Cancel(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Pause(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Restart(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemCompleted(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemStatusChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemStatusChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoUpdateSetting(&self) -> ::windows::core::Result<AutoUpdateSetting>;
    fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows::core::Result<()>;
    fn AcquisitionIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAcquisitionIdentity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetIsApplicableAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn StartAppInstallAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn UpdateAppByPackageFamilyNameAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn IsStoreBlockedByPolicyAsync(&self, storeclientname: &::windows::core::HSTRING, storeclientpublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsAppAllowedToInstallAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager2Impl: Sized {
    fn StartAppInstallWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: &::windows::core::HSTRING, bundleid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn UpdateAppByPackageFamilyNameWithTelemetryAsync(&self, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesWithTelemetryAsync(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsAppAllowedToInstallWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn CancelWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RestartWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager3Impl: Sized {
    fn StartProductInstallAsync(&self, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn StartProductInstallForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn UpdateAppByPackageFamilyNameForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsAppAllowedToInstallForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsApplicableForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn MoveToFrontOfDownloadQueue(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager4Impl: Sized {
    fn GetFreeUserEntitlementAsync(&self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
    fn GetFreeUserEntitlementForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
    fn GetFreeDeviceEntitlementAsync(&self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager5Impl: Sized {
    fn AppInstallItemsWithGroupSupport(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager6Impl: Sized {
    fn SearchForAllUpdatesWithUpdateOptionsAsync(&self, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn SearchForAllUpdatesWithUpdateOptionsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn SearchForUpdatesWithUpdateOptionsAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesWithUpdateOptionsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn StartProductInstallWithOptionsAsync(&self, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &::core::option::Option<AppInstallOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn StartProductInstallWithOptionsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &::core::option::Option<AppInstallOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsPackageIdentityAllowedToInstallAsync(&self, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsPackageIdentityAllowedToInstallForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager7Impl: Sized {
    fn CanInstallForAllUsers(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManagerItemEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<AppInstallItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallOptionsImpl: Sized {
    fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCatalogId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ForceUseOfNonRemovableStorage(&self) -> ::windows::core::Result<bool>;
    fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()>;
    fn Repair(&self) -> ::windows::core::Result<bool>;
    fn SetRepair(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetVolume(&self) -> ::windows::core::Result<super::super::super::super::Management::Deployment::PackageVolume>;
    fn SetTargetVolume(&self, value: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<()>;
    fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallOptions2Impl: Sized {
    fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallForAllUsers(&self) -> ::windows::core::Result<bool>;
    fn SetInstallForAllUsers(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageButDoNotInstall(&self) -> ::windows::core::Result<bool>;
    fn SetStageButDoNotInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCampaignId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedCampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExtendedCampaignId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatusImpl: Sized {
    fn InstallState(&self) -> ::windows::core::Result<AppInstallState>;
    fn DownloadSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn BytesDownloaded(&self) -> ::windows::core::Result<u64>;
    fn PercentComplete(&self) -> ::windows::core::Result<f64>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatus2Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User>;
    fn ReadyForLaunch(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatus3Impl: Sized {
    fn IsStaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUpdateOptionsImpl: Sized {
    fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCatalogId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUpdateOptions2Impl: Sized {
    fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGetEntitlementResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GetEntitlementStatus>;
}
