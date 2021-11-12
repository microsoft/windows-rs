#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[link(name = "windows")]
extern "system" {}
pub struct DeliveryOptimizationDownloadMode(i32);
pub struct DeliveryOptimizationDownloadModeSource(i32);
pub struct DeliveryOptimizationSettings(i32);
pub struct IDeliveryOptimizationSettings(pub *mut ::core::ffi::c_void);
pub struct IDeliveryOptimizationSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IStoreConfigurationStatics(pub *mut ::core::ffi::c_void);
pub struct IStoreConfigurationStatics2(pub *mut ::core::ffi::c_void);
pub struct IStoreConfigurationStatics3(pub *mut ::core::ffi::c_void);
pub struct IStoreConfigurationStatics4(pub *mut ::core::ffi::c_void);
pub struct IStoreConfigurationStatics5(pub *mut ::core::ffi::c_void);
pub struct IStoreHardwareManufacturerInfo(pub *mut ::core::ffi::c_void);
pub struct IStorePreview(pub *mut ::core::ffi::c_void);
pub struct IStorePreviewProductInfo(pub *mut ::core::ffi::c_void);
pub struct IStorePreviewPurchaseResults(pub *mut ::core::ffi::c_void);
pub struct IStorePreviewSkuInfo(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationCoreManagerHelper(pub *mut ::core::ffi::c_void);
pub struct StoreConfiguration(i32);
pub struct StoreHardwareManufacturerInfo(i32);
pub struct StoreLogOptions(i32);
pub struct StorePreview(i32);
pub struct StorePreviewProductInfo(i32);
pub struct StorePreviewProductPurchaseStatus(i32);
pub struct StorePreviewPurchaseResults(i32);
pub struct StorePreviewSkuInfo(i32);
pub struct StoreSystemFeature(i32);
pub struct WebAuthenticationCoreManagerHelper(i32);
