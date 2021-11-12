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
#[repr(C)]
pub struct AddResourcePackageOptions(i32);
#[repr(transparent)]
pub struct AppDisplayInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppExecutionContext(i32);
#[repr(transparent)]
pub struct AppInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppInstallerInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppInstallerPolicySource(i32);
#[repr(transparent)]
pub struct AppInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraApplicationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DesignMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FullTrustAppContract(i32);
#[repr(C)]
pub struct FullTrustLaunchResult(i32);
#[repr(transparent)]
pub struct FullTrustProcessLaunchResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FullTrustProcessLauncher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppDisplayInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInfo4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstallerInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppInstanceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraApplicationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesignModeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesignModeStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullTrustProcessLaunchResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILimitedAccessFeatureRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILimitedAccessFeaturesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackage8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalog2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalog3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalog4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalogAddOptionalPackageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalogAddResourcePackageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalogRemoveOptionalPackagesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalogRemoveResourcePackagesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageCatalogStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageContentGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageContentGroupStagingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageContentGroupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageIdWithMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageInstallingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageStagingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageStatus2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageUpdateAvailabilityResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageWithMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartupTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartupTaskStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISuspendingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISuspendingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISuspendingOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LimitedAccessFeatureRequestResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LimitedAccessFeatureStatus(i32);
#[repr(transparent)]
pub struct LimitedAccessFeatures(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Package(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageCatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageCatalogAddOptionalPackageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageCatalogAddResourcePackageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageCatalogRemoveOptionalPackagesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageCatalogRemoveResourcePackagesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageContentGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageContentGroupStagingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageContentGroupState(i32);
#[repr(transparent)]
pub struct PackageId(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageInstallProgress(i32);
#[repr(transparent)]
pub struct PackageInstallingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageSignatureKind(i32);
#[repr(transparent)]
pub struct PackageStagingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageUpdateAvailability(i32);
#[repr(transparent)]
pub struct PackageUpdateAvailabilityResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PackageVersion(i32);
#[repr(transparent)]
pub struct StartupTask(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StartupTaskContract(i32);
#[repr(C)]
pub struct StartupTaskState(i32);
#[repr(transparent)]
pub struct SuspendingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingOperation(pub *mut ::core::ffi::c_void);
