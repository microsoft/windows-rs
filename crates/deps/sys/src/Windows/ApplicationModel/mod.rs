#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct AddResourcePackageOptions(pub u32);
impl AddResourcePackageOptions {
    pub const None: Self = Self(0u32);
    pub const ForceTargetAppShutdown: Self = Self(1u32);
    pub const ApplyUpdateIfAvailable: Self = Self(2u32);
}
impl ::core::marker::Copy for AddResourcePackageOptions {}
impl ::core::clone::Clone for AddResourcePackageOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppDisplayInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppDisplayInfo {}
impl ::core::clone::Clone for AppDisplayInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppExecutionContext(pub i32);
impl AppExecutionContext {
    pub const Unknown: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Guest: Self = Self(2i32);
}
impl ::core::marker::Copy for AppExecutionContext {}
impl ::core::clone::Clone for AppExecutionContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInfo {}
impl ::core::clone::Clone for AppInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstallerInfo {}
impl ::core::clone::Clone for AppInstallerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstallerPolicySource(pub i32);
impl AppInstallerPolicySource {
    pub const Default: Self = Self(0i32);
    pub const System: Self = Self(1i32);
}
impl ::core::marker::Copy for AppInstallerPolicySource {}
impl ::core::clone::Clone for AppInstallerPolicySource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppInstance {}
impl ::core::clone::Clone for AppInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnteredBackgroundEventArgs {}
impl ::core::clone::Clone for EnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FullTrustLaunchResult(pub i32);
impl FullTrustLaunchResult {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for FullTrustLaunchResult {}
impl ::core::clone::Clone for FullTrustLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FullTrustProcessLaunchResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FullTrustProcessLaunchResult {}
impl ::core::clone::Clone for FullTrustProcessLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppDisplayInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppDisplayInfo {}
impl ::core::clone::Clone for IAppDisplayInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInfo {}
impl ::core::clone::Clone for IAppInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInfo2 {}
impl ::core::clone::Clone for IAppInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInfo3 {}
impl ::core::clone::Clone for IAppInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInfo4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInfo4 {}
impl ::core::clone::Clone for IAppInfo4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInfoStatics {}
impl ::core::clone::Clone for IAppInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallerInfo {}
impl ::core::clone::Clone for IAppInstallerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstallerInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstallerInfo2 {}
impl ::core::clone::Clone for IAppInstallerInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstance {}
impl ::core::clone::Clone for IAppInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppInstanceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppInstanceStatics {}
impl ::core::clone::Clone for IAppInstanceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraApplicationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraApplicationManagerStatics {}
impl ::core::clone::Clone for ICameraApplicationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesignModeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesignModeStatics {}
impl ::core::clone::Clone for IDesignModeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesignModeStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesignModeStatics2 {}
impl ::core::clone::Clone for IDesignModeStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnteredBackgroundEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnteredBackgroundEventArgs {}
impl ::core::clone::Clone for IEnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFullTrustProcessLaunchResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFullTrustProcessLaunchResult {}
impl ::core::clone::Clone for IFullTrustProcessLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFullTrustProcessLauncherStatics {}
impl ::core::clone::Clone for IFullTrustProcessLauncherStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFullTrustProcessLauncherStatics2 {}
impl ::core::clone::Clone for IFullTrustProcessLauncherStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILeavingBackgroundEventArgs {}
impl ::core::clone::Clone for ILeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILimitedAccessFeatureRequestResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILimitedAccessFeatureRequestResult {}
impl ::core::clone::Clone for ILimitedAccessFeatureRequestResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILimitedAccessFeaturesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILimitedAccessFeaturesStatics {}
impl ::core::clone::Clone for ILimitedAccessFeaturesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage {}
impl ::core::clone::Clone for IPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage2 {}
impl ::core::clone::Clone for IPackage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage3 {}
impl ::core::clone::Clone for IPackage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage4 {}
impl ::core::clone::Clone for IPackage4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage5 {}
impl ::core::clone::Clone for IPackage5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage6 {}
impl ::core::clone::Clone for IPackage6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage7 {}
impl ::core::clone::Clone for IPackage7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackage8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackage8 {}
impl ::core::clone::Clone for IPackage8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalog {}
impl ::core::clone::Clone for IPackageCatalog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalog2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalog2 {}
impl ::core::clone::Clone for IPackageCatalog2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalog3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalog3 {}
impl ::core::clone::Clone for IPackageCatalog3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalog4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalog4 {}
impl ::core::clone::Clone for IPackageCatalog4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalogAddOptionalPackageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalogAddOptionalPackageResult {}
impl ::core::clone::Clone for IPackageCatalogAddOptionalPackageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalogAddResourcePackageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalogAddResourcePackageResult {}
impl ::core::clone::Clone for IPackageCatalogAddResourcePackageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalogRemoveOptionalPackagesResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalogRemoveOptionalPackagesResult {}
impl ::core::clone::Clone for IPackageCatalogRemoveOptionalPackagesResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalogRemoveResourcePackagesResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalogRemoveResourcePackagesResult {}
impl ::core::clone::Clone for IPackageCatalogRemoveResourcePackagesResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageCatalogStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageCatalogStatics {}
impl ::core::clone::Clone for IPackageCatalogStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageContentGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageContentGroup {}
impl ::core::clone::Clone for IPackageContentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageContentGroupStagingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageContentGroupStagingEventArgs {}
impl ::core::clone::Clone for IPackageContentGroupStagingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageContentGroupStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageContentGroupStatics {}
impl ::core::clone::Clone for IPackageContentGroupStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageId {}
impl ::core::clone::Clone for IPackageId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageIdWithMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageIdWithMetadata {}
impl ::core::clone::Clone for IPackageIdWithMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageInstallingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageInstallingEventArgs {}
impl ::core::clone::Clone for IPackageInstallingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageStagingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageStagingEventArgs {}
impl ::core::clone::Clone for IPackageStagingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageStatics {}
impl ::core::clone::Clone for IPackageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageStatus {}
impl ::core::clone::Clone for IPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageStatus2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageStatus2 {}
impl ::core::clone::Clone for IPackageStatus2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageStatusChangedEventArgs {}
impl ::core::clone::Clone for IPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageUninstallingEventArgs {}
impl ::core::clone::Clone for IPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageUpdateAvailabilityResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageUpdateAvailabilityResult {}
impl ::core::clone::Clone for IPackageUpdateAvailabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageUpdatingEventArgs {}
impl ::core::clone::Clone for IPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageWithMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageWithMetadata {}
impl ::core::clone::Clone for IPackageWithMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStartupTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStartupTask {}
impl ::core::clone::Clone for IStartupTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStartupTaskStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStartupTaskStatics {}
impl ::core::clone::Clone for IStartupTaskStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISuspendingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISuspendingDeferral {}
impl ::core::clone::Clone for ISuspendingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISuspendingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISuspendingEventArgs {}
impl ::core::clone::Clone for ISuspendingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISuspendingOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISuspendingOperation {}
impl ::core::clone::Clone for ISuspendingOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LeavingBackgroundEventArgs {}
impl ::core::clone::Clone for LeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LimitedAccessFeatureRequestResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LimitedAccessFeatureRequestResult {}
impl ::core::clone::Clone for LimitedAccessFeatureRequestResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LimitedAccessFeatureStatus(pub i32);
impl LimitedAccessFeatureStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
    pub const AvailableWithoutToken: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LimitedAccessFeatureStatus {}
impl ::core::clone::Clone for LimitedAccessFeatureStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Package(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Package {}
impl ::core::clone::Clone for Package {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageCatalog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageCatalog {}
impl ::core::clone::Clone for PackageCatalog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageCatalogAddOptionalPackageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageCatalogAddOptionalPackageResult {}
impl ::core::clone::Clone for PackageCatalogAddOptionalPackageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageCatalogAddResourcePackageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageCatalogAddResourcePackageResult {}
impl ::core::clone::Clone for PackageCatalogAddResourcePackageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageCatalogRemoveOptionalPackagesResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageCatalogRemoveOptionalPackagesResult {}
impl ::core::clone::Clone for PackageCatalogRemoveOptionalPackagesResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageCatalogRemoveResourcePackagesResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageCatalogRemoveResourcePackagesResult {}
impl ::core::clone::Clone for PackageCatalogRemoveResourcePackagesResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageContentGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageContentGroup {}
impl ::core::clone::Clone for PackageContentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageContentGroupStagingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageContentGroupStagingEventArgs {}
impl ::core::clone::Clone for PackageContentGroupStagingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageContentGroupState(pub i32);
impl PackageContentGroupState {
    pub const NotStaged: Self = Self(0i32);
    pub const Queued: Self = Self(1i32);
    pub const Staging: Self = Self(2i32);
    pub const Staged: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageContentGroupState {}
impl ::core::clone::Clone for PackageContentGroupState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageId {}
impl ::core::clone::Clone for PackageId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PackageInstallProgress {
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for PackageInstallProgress {}
impl ::core::clone::Clone for PackageInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageInstallingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageInstallingEventArgs {}
impl ::core::clone::Clone for PackageInstallingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageSignatureKind(pub i32);
impl PackageSignatureKind {
    pub const None: Self = Self(0i32);
    pub const Developer: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
    pub const Store: Self = Self(3i32);
    pub const System: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageSignatureKind {}
impl ::core::clone::Clone for PackageSignatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageStagingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageStagingEventArgs {}
impl ::core::clone::Clone for PackageStagingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageStatus {}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageStatusChangedEventArgs {}
impl ::core::clone::Clone for PackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageUninstallingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageUninstallingEventArgs {}
impl ::core::clone::Clone for PackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageUpdateAvailability(pub i32);
impl PackageUpdateAvailability {
    pub const Unknown: Self = Self(0i32);
    pub const NoUpdates: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Required: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageUpdateAvailability {}
impl ::core::clone::Clone for PackageUpdateAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageUpdateAvailabilityResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageUpdateAvailabilityResult {}
impl ::core::clone::Clone for PackageUpdateAvailabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageUpdatingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageUpdatingEventArgs {}
impl ::core::clone::Clone for PackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PackageVersion {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for PackageVersion {}
impl ::core::clone::Clone for PackageVersion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StartupTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StartupTask {}
impl ::core::clone::Clone for StartupTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StartupTaskState(pub i32);
impl StartupTaskState {
    pub const Disabled: Self = Self(0i32);
    pub const DisabledByUser: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const EnabledByPolicy: Self = Self(4i32);
}
impl ::core::marker::Copy for StartupTaskState {}
impl ::core::clone::Clone for StartupTaskState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingDeferral {}
impl ::core::clone::Clone for SuspendingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingEventArgs {}
impl ::core::clone::Clone for SuspendingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingOperation {}
impl ::core::clone::Clone for SuspendingOperation {
    fn clone(&self) -> Self {
        *self
    }
}
