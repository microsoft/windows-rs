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
    pub const NotLicensableToUser: Self = Self(0i32);
    pub const Licensable: Self = Self(1i32);
    pub const LicenseActionNotApplicableToProduct: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
#[repr(transparent)]
pub struct StoreCollectionData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreConsumableResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreConsumableStatus(pub i32);
impl StoreConsumableStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const InsufficentQuantity: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
#[repr(transparent)]
pub struct StoreContext(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreContract(i32);
#[repr(transparent)]
pub struct StoreDurationUnit(pub i32);
impl StoreDurationUnit {
    pub const Minute: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const Day: Self = Self(2i32);
    pub const Week: Self = Self(3i32);
    pub const Month: Self = Self(4i32);
    pub const Year: Self = Self(5i32);
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
    pub const Pending: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Deploying: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const OtherError: Self = Self(5i32);
    pub const ErrorLowBattery: Self = Self(6i32);
    pub const ErrorWiFiRecommended: Self = Self(7i32);
    pub const ErrorWiFiRequired: Self = Self(8i32);
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
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotPurchased: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
#[repr(transparent)]
pub struct StoreQueueItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreQueueItemCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreQueueItemExtendedState(pub i32);
impl StoreQueueItemExtendedState {
    pub const ActivePending: Self = Self(0i32);
    pub const ActiveStarting: Self = Self(1i32);
    pub const ActiveAcquiringLicense: Self = Self(2i32);
    pub const ActiveDownloading: Self = Self(3i32);
    pub const ActiveRestoringData: Self = Self(4i32);
    pub const ActiveInstalling: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedPackagesInUse: Self = Self(10i32);
    pub const PausedLowBattery: Self = Self(11i32);
    pub const PausedWiFiRecommended: Self = Self(12i32);
    pub const PausedWiFiRequired: Self = Self(13i32);
    pub const PausedReadyToInstall: Self = Self(14i32);
}
#[repr(transparent)]
pub struct StoreQueueItemKind(pub i32);
impl StoreQueueItemKind {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
#[repr(transparent)]
pub struct StoreQueueItemState(pub i32);
impl StoreQueueItemState {
    pub const Active: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
#[repr(transparent)]
pub struct StoreQueueItemStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreRateAndReviewResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreRateAndReviewStatus(pub i32);
impl StoreRateAndReviewStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
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
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UninstallNotApplicable: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
#[repr(transparent)]
pub struct StoreVideo(pub *mut ::core::ffi::c_void);
