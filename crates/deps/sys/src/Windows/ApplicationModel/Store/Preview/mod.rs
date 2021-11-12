#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[link(name = "windows")]
extern "system" {}
pub struct DeliveryOptimizationDownloadMode(i32);
pub struct DeliveryOptimizationDownloadModeSource(i32);
#[repr(transparent)]
pub struct DeliveryOptimizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeliveryOptimizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeliveryOptimizationSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreHardwareManufacturerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreviewProductInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreviewPurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreviewSkuInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreHardwareManufacturerInfo(pub *mut ::core::ffi::c_void);
pub struct StoreLogOptions(i32);
#[repr(transparent)]
pub struct StorePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePreviewProductInfo(pub *mut ::core::ffi::c_void);
pub struct StorePreviewProductPurchaseStatus(i32);
#[repr(transparent)]
pub struct StorePreviewPurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePreviewSkuInfo(pub *mut ::core::ffi::c_void);
pub struct StoreSystemFeature(i32);
#[repr(transparent)]
pub struct WebAuthenticationCoreManagerHelper(pub *mut ::core::ffi::c_void);
