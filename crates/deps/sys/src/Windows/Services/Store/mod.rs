#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct StoreCanLicenseStatus(i32);
#[repr(transparent)]
pub struct StoreCollectionData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreConsumableResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreConsumableStatus(i32);
#[repr(transparent)]
pub struct StoreContext(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreContract(i32);
#[repr(C)]
pub struct StoreDurationUnit(i32);
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
#[repr(C)]
pub struct StorePackageUpdateState(i32);
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
#[repr(C)]
pub struct StorePurchaseStatus(i32);
#[repr(transparent)]
pub struct StoreQueueItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreQueueItemCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreQueueItemExtendedState(i32);
#[repr(C)]
pub struct StoreQueueItemKind(i32);
#[repr(C)]
pub struct StoreQueueItemState(i32);
#[repr(transparent)]
pub struct StoreQueueItemStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreRateAndReviewResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreRateAndReviewStatus(i32);
#[repr(transparent)]
pub struct StoreRequestHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSendRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSku(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSubscriptionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreUninstallStorePackageResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StoreUninstallStorePackageStatus(i32);
#[repr(transparent)]
pub struct StoreVideo(pub *mut ::core::ffi::c_void);
