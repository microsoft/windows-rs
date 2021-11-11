#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Activation")]
pub mod Activation;
#[cfg(feature = "ApplicationModel_AppExtensions")]
pub mod AppExtensions;
#[cfg(feature = "ApplicationModel_AppService")]
pub mod AppService;
#[cfg(feature = "ApplicationModel_Appointments")]
pub mod Appointments;
#[cfg(feature = "ApplicationModel_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls")]
pub mod Calls;
#[cfg(feature = "ApplicationModel_Chat")]
pub mod Chat;
#[cfg(feature = "ApplicationModel_CommunicationBlocking")]
pub mod CommunicationBlocking;
#[cfg(feature = "ApplicationModel_Contacts")]
pub mod Contacts;
#[cfg(feature = "ApplicationModel_ConversationalAgent")]
pub mod ConversationalAgent;
#[cfg(feature = "ApplicationModel_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_DataTransfer")]
pub mod DataTransfer;
#[cfg(feature = "ApplicationModel_Email")]
pub mod Email;
#[cfg(feature = "ApplicationModel_ExtendedExecution")]
pub mod ExtendedExecution;
#[cfg(feature = "ApplicationModel_Holographic")]
pub mod Holographic;
#[cfg(feature = "ApplicationModel_LockScreen")]
pub mod LockScreen;
#[cfg(feature = "ApplicationModel_Payments")]
pub mod Payments;
#[cfg(feature = "ApplicationModel_Preview")]
pub mod Preview;
#[cfg(feature = "ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "ApplicationModel_Search")]
pub mod Search;
#[cfg(feature = "ApplicationModel_SocialInfo")]
pub mod SocialInfo;
#[cfg(feature = "ApplicationModel_Store")]
pub mod Store;
#[cfg(feature = "ApplicationModel_UserActivities")]
pub mod UserActivities;
#[cfg(feature = "ApplicationModel_UserDataAccounts")]
pub mod UserDataAccounts;
#[cfg(feature = "ApplicationModel_UserDataTasks")]
pub mod UserDataTasks;
#[cfg(feature = "ApplicationModel_VoiceCommands")]
pub mod VoiceCommands;
#[cfg(feature = "ApplicationModel_Wallet")]
pub mod Wallet;
#[link(name = "windows")]
extern "system" {
    fn AddResourcePackageOptions();
    fn AppDisplayInfo();
    fn AppExecutionContext();
    fn AppInfo();
    fn AppInstallerInfo();
    fn AppInstallerPolicySource();
    fn AppInstance();
    fn CameraApplicationManager();
    fn DesignMode();
    fn EnteredBackgroundEventArgs();
    fn FullTrustAppContract();
    fn FullTrustLaunchResult();
    fn FullTrustProcessLaunchResult();
    fn FullTrustProcessLauncher();
    fn IAppDisplayInfo();
    fn IAppInfo();
    fn IAppInfo2();
    fn IAppInfo3();
    fn IAppInfo4();
    fn IAppInfoStatics();
    fn IAppInstallerInfo();
    fn IAppInstallerInfo2();
    fn IAppInstance();
    fn IAppInstanceStatics();
    fn ICameraApplicationManagerStatics();
    fn IDesignModeStatics();
    fn IDesignModeStatics2();
    fn IEnteredBackgroundEventArgs();
    fn IFullTrustProcessLaunchResult();
    fn IFullTrustProcessLauncherStatics();
    fn IFullTrustProcessLauncherStatics2();
    fn ILeavingBackgroundEventArgs();
    fn ILimitedAccessFeatureRequestResult();
    fn ILimitedAccessFeaturesStatics();
    fn IPackage();
    fn IPackage2();
    fn IPackage3();
    fn IPackage4();
    fn IPackage5();
    fn IPackage6();
    fn IPackage7();
    fn IPackage8();
    fn IPackageCatalog();
    fn IPackageCatalog2();
    fn IPackageCatalog3();
    fn IPackageCatalog4();
    fn IPackageCatalogAddOptionalPackageResult();
    fn IPackageCatalogAddResourcePackageResult();
    fn IPackageCatalogRemoveOptionalPackagesResult();
    fn IPackageCatalogRemoveResourcePackagesResult();
    fn IPackageCatalogStatics();
    fn IPackageContentGroup();
    fn IPackageContentGroupStagingEventArgs();
    fn IPackageContentGroupStatics();
    fn IPackageId();
    fn IPackageIdWithMetadata();
    fn IPackageInstallingEventArgs();
    fn IPackageStagingEventArgs();
    fn IPackageStatics();
    fn IPackageStatus();
    fn IPackageStatus2();
    fn IPackageStatusChangedEventArgs();
    fn IPackageUninstallingEventArgs();
    fn IPackageUpdateAvailabilityResult();
    fn IPackageUpdatingEventArgs();
    fn IPackageWithMetadata();
    fn IStartupTask();
    fn IStartupTaskStatics();
    fn ISuspendingDeferral();
    fn ISuspendingEventArgs();
    fn ISuspendingOperation();
    fn LeavingBackgroundEventArgs();
    fn LimitedAccessFeatureRequestResult();
    fn LimitedAccessFeatureStatus();
    fn LimitedAccessFeatures();
    fn Package();
    fn PackageCatalog();
    fn PackageCatalogAddOptionalPackageResult();
    fn PackageCatalogAddResourcePackageResult();
    fn PackageCatalogRemoveOptionalPackagesResult();
    fn PackageCatalogRemoveResourcePackagesResult();
    fn PackageContentGroup();
    fn PackageContentGroupStagingEventArgs();
    fn PackageContentGroupState();
    fn PackageId();
    fn PackageInstallProgress();
    fn PackageInstallingEventArgs();
    fn PackageSignatureKind();
    fn PackageStagingEventArgs();
    fn PackageStatus();
    fn PackageStatusChangedEventArgs();
    fn PackageUninstallingEventArgs();
    fn PackageUpdateAvailability();
    fn PackageUpdateAvailabilityResult();
    fn PackageUpdatingEventArgs();
    fn PackageVersion();
    fn StartupTask();
    fn StartupTaskContract();
    fn StartupTaskState();
    fn SuspendingDeferral();
    fn SuspendingEventArgs();
    fn SuspendingOperation();
}
