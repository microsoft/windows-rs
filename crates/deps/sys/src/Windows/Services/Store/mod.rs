#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IStoreAcquireLicenseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreAppLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreAppLicense2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreAvailability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreCanAcquireLicenseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreCollectionData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConsumableResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreContext3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreContext4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreContextStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePackageInstallOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePackageLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePackageUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePackageUpdateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePackageUpdateResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePrice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreProduct(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreProductOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreProductPagedQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreProductQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreProductResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePurchaseProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePurchasePropertiesFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePurchaseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreQueueItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreQueueItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreQueueItemCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreQueueItemStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreRateAndReviewResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreRequestHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreSendRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreSendRequestResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreSku(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreSubscriptionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreUninstallStorePackageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreVideo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreAcquireLicenseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreAppLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreAvailability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreCanAcquireLicenseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreCanLicenseStatus(pub i32);
impl StoreCanLicenseStatus {
    pub const NotLicensableToUser: StoreCanLicenseStatus = StoreCanLicenseStatus(0i32);
    pub const Licensable: StoreCanLicenseStatus = StoreCanLicenseStatus(1i32);
    pub const LicenseActionNotApplicableToProduct: StoreCanLicenseStatus = StoreCanLicenseStatus(2i32);
    pub const NetworkError: StoreCanLicenseStatus = StoreCanLicenseStatus(3i32);
    pub const ServerError: StoreCanLicenseStatus = StoreCanLicenseStatus(4i32);
}
#[repr(transparent)]
pub struct StoreCollectionData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreConsumableResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreConsumableStatus(pub i32);
impl StoreConsumableStatus {
    pub const Succeeded: StoreConsumableStatus = StoreConsumableStatus(0i32);
    pub const InsufficentQuantity: StoreConsumableStatus = StoreConsumableStatus(1i32);
    pub const NetworkError: StoreConsumableStatus = StoreConsumableStatus(2i32);
    pub const ServerError: StoreConsumableStatus = StoreConsumableStatus(3i32);
}
#[repr(transparent)]
pub struct StoreContext(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreContract(i32);
#[repr(transparent)]
pub struct StoreDurationUnit(pub i32);
impl StoreDurationUnit {
    pub const Minute: StoreDurationUnit = StoreDurationUnit(0i32);
    pub const Hour: StoreDurationUnit = StoreDurationUnit(1i32);
    pub const Day: StoreDurationUnit = StoreDurationUnit(2i32);
    pub const Week: StoreDurationUnit = StoreDurationUnit(3i32);
    pub const Month: StoreDurationUnit = StoreDurationUnit(4i32);
    pub const Year: StoreDurationUnit = StoreDurationUnit(5i32);
}
#[repr(transparent)]
pub struct StoreImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePackageInstallOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePackageLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePackageUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePackageUpdateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePackageUpdateState(pub i32);
impl StorePackageUpdateState {
    pub const Pending: StorePackageUpdateState = StorePackageUpdateState(0i32);
    pub const Downloading: StorePackageUpdateState = StorePackageUpdateState(1i32);
    pub const Deploying: StorePackageUpdateState = StorePackageUpdateState(2i32);
    pub const Completed: StorePackageUpdateState = StorePackageUpdateState(3i32);
    pub const Canceled: StorePackageUpdateState = StorePackageUpdateState(4i32);
    pub const OtherError: StorePackageUpdateState = StorePackageUpdateState(5i32);
    pub const ErrorLowBattery: StorePackageUpdateState = StorePackageUpdateState(6i32);
    pub const ErrorWiFiRecommended: StorePackageUpdateState = StorePackageUpdateState(7i32);
    pub const ErrorWiFiRequired: StorePackageUpdateState = StorePackageUpdateState(8i32);
}
#[repr(C)]
pub struct StorePackageUpdateStatus(i32);
#[repr(transparent)]
pub struct StorePrice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreProduct(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreProductOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreProductPagedQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreProductQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreProductResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePurchaseProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePurchaseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePurchaseStatus(pub i32);
impl StorePurchaseStatus {
    pub const Succeeded: StorePurchaseStatus = StorePurchaseStatus(0i32);
    pub const AlreadyPurchased: StorePurchaseStatus = StorePurchaseStatus(1i32);
    pub const NotPurchased: StorePurchaseStatus = StorePurchaseStatus(2i32);
    pub const NetworkError: StorePurchaseStatus = StorePurchaseStatus(3i32);
    pub const ServerError: StorePurchaseStatus = StorePurchaseStatus(4i32);
}
#[repr(transparent)]
pub struct StoreQueueItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreQueueItemCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreQueueItemExtendedState(pub i32);
impl StoreQueueItemExtendedState {
    pub const ActivePending: StoreQueueItemExtendedState = StoreQueueItemExtendedState(0i32);
    pub const ActiveStarting: StoreQueueItemExtendedState = StoreQueueItemExtendedState(1i32);
    pub const ActiveAcquiringLicense: StoreQueueItemExtendedState = StoreQueueItemExtendedState(2i32);
    pub const ActiveDownloading: StoreQueueItemExtendedState = StoreQueueItemExtendedState(3i32);
    pub const ActiveRestoringData: StoreQueueItemExtendedState = StoreQueueItemExtendedState(4i32);
    pub const ActiveInstalling: StoreQueueItemExtendedState = StoreQueueItemExtendedState(5i32);
    pub const Completed: StoreQueueItemExtendedState = StoreQueueItemExtendedState(6i32);
    pub const Canceled: StoreQueueItemExtendedState = StoreQueueItemExtendedState(7i32);
    pub const Paused: StoreQueueItemExtendedState = StoreQueueItemExtendedState(8i32);
    pub const Error: StoreQueueItemExtendedState = StoreQueueItemExtendedState(9i32);
    pub const PausedPackagesInUse: StoreQueueItemExtendedState = StoreQueueItemExtendedState(10i32);
    pub const PausedLowBattery: StoreQueueItemExtendedState = StoreQueueItemExtendedState(11i32);
    pub const PausedWiFiRecommended: StoreQueueItemExtendedState = StoreQueueItemExtendedState(12i32);
    pub const PausedWiFiRequired: StoreQueueItemExtendedState = StoreQueueItemExtendedState(13i32);
    pub const PausedReadyToInstall: StoreQueueItemExtendedState = StoreQueueItemExtendedState(14i32);
}
#[repr(transparent)]
pub struct StoreQueueItemKind(pub i32);
impl StoreQueueItemKind {
    pub const Install: StoreQueueItemKind = StoreQueueItemKind(0i32);
    pub const Update: StoreQueueItemKind = StoreQueueItemKind(1i32);
    pub const Repair: StoreQueueItemKind = StoreQueueItemKind(2i32);
}
#[repr(transparent)]
pub struct StoreQueueItemState(pub i32);
impl StoreQueueItemState {
    pub const Active: StoreQueueItemState = StoreQueueItemState(0i32);
    pub const Completed: StoreQueueItemState = StoreQueueItemState(1i32);
    pub const Canceled: StoreQueueItemState = StoreQueueItemState(2i32);
    pub const Error: StoreQueueItemState = StoreQueueItemState(3i32);
    pub const Paused: StoreQueueItemState = StoreQueueItemState(4i32);
}
#[repr(transparent)]
pub struct StoreQueueItemStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreRateAndReviewResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreRateAndReviewStatus(pub i32);
impl StoreRateAndReviewStatus {
    pub const Succeeded: StoreRateAndReviewStatus = StoreRateAndReviewStatus(0i32);
    pub const CanceledByUser: StoreRateAndReviewStatus = StoreRateAndReviewStatus(1i32);
    pub const NetworkError: StoreRateAndReviewStatus = StoreRateAndReviewStatus(2i32);
    pub const Error: StoreRateAndReviewStatus = StoreRateAndReviewStatus(3i32);
}
#[repr(transparent)]
pub struct StoreSendRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSku(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSubscriptionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreUninstallStorePackageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreUninstallStorePackageStatus(pub i32);
impl StoreUninstallStorePackageStatus {
    pub const Succeeded: StoreUninstallStorePackageStatus = StoreUninstallStorePackageStatus(0i32);
    pub const CanceledByUser: StoreUninstallStorePackageStatus = StoreUninstallStorePackageStatus(1i32);
    pub const NetworkError: StoreUninstallStorePackageStatus = StoreUninstallStorePackageStatus(2i32);
    pub const UninstallNotApplicable: StoreUninstallStorePackageStatus = StoreUninstallStorePackageStatus(3i32);
    pub const Error: StoreUninstallStorePackageStatus = StoreUninstallStorePackageStatus(4i32);
}
#[repr(transparent)]
pub struct StoreVideo(pub *mut ::core::ffi::c_void);
