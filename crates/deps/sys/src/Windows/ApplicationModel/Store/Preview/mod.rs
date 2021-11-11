#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[link(name = "windows")]
extern "system" {
    fn DeliveryOptimizationDownloadMode();
    fn DeliveryOptimizationDownloadModeSource();
    fn DeliveryOptimizationSettings();
    fn IDeliveryOptimizationSettings();
    fn IDeliveryOptimizationSettingsStatics();
    fn IStoreConfigurationStatics();
    fn IStoreConfigurationStatics2();
    fn IStoreConfigurationStatics3();
    fn IStoreConfigurationStatics4();
    fn IStoreConfigurationStatics5();
    fn IStoreHardwareManufacturerInfo();
    fn IStorePreview();
    fn IStorePreviewProductInfo();
    fn IStorePreviewPurchaseResults();
    fn IStorePreviewSkuInfo();
    fn IWebAuthenticationCoreManagerHelper();
    fn StoreConfiguration();
    fn StoreHardwareManufacturerInfo();
    fn StoreLogOptions();
    fn StorePreview();
    fn StorePreviewProductInfo();
    fn StorePreviewProductPurchaseStatus();
    fn StorePreviewPurchaseResults();
    fn StorePreviewSkuInfo();
    fn StoreSystemFeature();
    fn WebAuthenticationCoreManagerHelper();
}
