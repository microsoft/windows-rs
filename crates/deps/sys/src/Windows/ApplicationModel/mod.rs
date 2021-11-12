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
extern "system" {}
pub struct AddResourcePackageOptions(i32);
pub struct AppDisplayInfo(i32);
pub struct AppExecutionContext(i32);
pub struct AppInfo(i32);
pub struct AppInstallerInfo(i32);
pub struct AppInstallerPolicySource(i32);
pub struct AppInstance(i32);
pub struct CameraApplicationManager(i32);
pub struct DesignMode(i32);
pub struct EnteredBackgroundEventArgs(i32);
pub struct FullTrustAppContract(i32);
pub struct FullTrustLaunchResult(i32);
pub struct FullTrustProcessLaunchResult(i32);
pub struct FullTrustProcessLauncher(i32);
pub struct IAppDisplayInfo(pub *mut ::core::ffi::c_void);
pub struct IAppInfo(pub *mut ::core::ffi::c_void);
pub struct IAppInfo2(pub *mut ::core::ffi::c_void);
pub struct IAppInfo3(pub *mut ::core::ffi::c_void);
pub struct IAppInfo4(pub *mut ::core::ffi::c_void);
pub struct IAppInfoStatics(pub *mut ::core::ffi::c_void);
pub struct IAppInstallerInfo(pub *mut ::core::ffi::c_void);
pub struct IAppInstallerInfo2(pub *mut ::core::ffi::c_void);
pub struct IAppInstance(pub *mut ::core::ffi::c_void);
pub struct IAppInstanceStatics(pub *mut ::core::ffi::c_void);
pub struct ICameraApplicationManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IDesignModeStatics(pub *mut ::core::ffi::c_void);
pub struct IDesignModeStatics2(pub *mut ::core::ffi::c_void);
pub struct IEnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
pub struct IFullTrustProcessLaunchResult(pub *mut ::core::ffi::c_void);
pub struct IFullTrustProcessLauncherStatics(pub *mut ::core::ffi::c_void);
pub struct IFullTrustProcessLauncherStatics2(pub *mut ::core::ffi::c_void);
pub struct ILeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
pub struct ILimitedAccessFeatureRequestResult(pub *mut ::core::ffi::c_void);
pub struct ILimitedAccessFeaturesStatics(pub *mut ::core::ffi::c_void);
pub struct IPackage(pub *mut ::core::ffi::c_void);
pub struct IPackage2(pub *mut ::core::ffi::c_void);
pub struct IPackage3(pub *mut ::core::ffi::c_void);
pub struct IPackage4(pub *mut ::core::ffi::c_void);
pub struct IPackage5(pub *mut ::core::ffi::c_void);
pub struct IPackage6(pub *mut ::core::ffi::c_void);
pub struct IPackage7(pub *mut ::core::ffi::c_void);
pub struct IPackage8(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalog(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalog2(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalog3(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalog4(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalogAddOptionalPackageResult(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalogAddResourcePackageResult(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalogRemoveOptionalPackagesResult(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalogRemoveResourcePackagesResult(pub *mut ::core::ffi::c_void);
pub struct IPackageCatalogStatics(pub *mut ::core::ffi::c_void);
pub struct IPackageContentGroup(pub *mut ::core::ffi::c_void);
pub struct IPackageContentGroupStagingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPackageContentGroupStatics(pub *mut ::core::ffi::c_void);
pub struct IPackageId(pub *mut ::core::ffi::c_void);
pub struct IPackageIdWithMetadata(pub *mut ::core::ffi::c_void);
pub struct IPackageInstallingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPackageStagingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPackageStatics(pub *mut ::core::ffi::c_void);
pub struct IPackageStatus(pub *mut ::core::ffi::c_void);
pub struct IPackageStatus2(pub *mut ::core::ffi::c_void);
pub struct IPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPackageUpdateAvailabilityResult(pub *mut ::core::ffi::c_void);
pub struct IPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPackageWithMetadata(pub *mut ::core::ffi::c_void);
pub struct IStartupTask(pub *mut ::core::ffi::c_void);
pub struct IStartupTaskStatics(pub *mut ::core::ffi::c_void);
pub struct ISuspendingDeferral(pub *mut ::core::ffi::c_void);
pub struct ISuspendingEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISuspendingOperation(pub *mut ::core::ffi::c_void);
pub struct LeavingBackgroundEventArgs(i32);
pub struct LimitedAccessFeatureRequestResult(i32);
pub struct LimitedAccessFeatureStatus(i32);
pub struct LimitedAccessFeatures(i32);
pub struct Package(i32);
pub struct PackageCatalog(i32);
pub struct PackageCatalogAddOptionalPackageResult(i32);
pub struct PackageCatalogAddResourcePackageResult(i32);
pub struct PackageCatalogRemoveOptionalPackagesResult(i32);
pub struct PackageCatalogRemoveResourcePackagesResult(i32);
pub struct PackageContentGroup(i32);
pub struct PackageContentGroupStagingEventArgs(i32);
pub struct PackageContentGroupState(i32);
pub struct PackageId(i32);
pub struct PackageInstallProgress(i32);
pub struct PackageInstallingEventArgs(i32);
pub struct PackageSignatureKind(i32);
pub struct PackageStagingEventArgs(i32);
pub struct PackageStatus(i32);
pub struct PackageStatusChangedEventArgs(i32);
pub struct PackageUninstallingEventArgs(i32);
pub struct PackageUpdateAvailability(i32);
pub struct PackageUpdateAvailabilityResult(i32);
pub struct PackageUpdatingEventArgs(i32);
pub struct PackageVersion(i32);
pub struct StartupTask(i32);
pub struct StartupTaskContract(i32);
pub struct StartupTaskState(i32);
pub struct SuspendingDeferral(i32);
pub struct SuspendingEventArgs(i32);
pub struct SuspendingOperation(i32);
