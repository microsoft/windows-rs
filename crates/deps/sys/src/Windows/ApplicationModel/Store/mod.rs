#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Store_LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "ApplicationModel_Store_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {
    fn CurrentApp();
    fn CurrentAppSimulator();
    fn FulfillmentResult();
    fn ICurrentApp();
    fn ICurrentApp2Statics();
    fn ICurrentAppSimulator();
    fn ICurrentAppSimulatorStaticsWithFiltering();
    fn ICurrentAppSimulatorWithCampaignId();
    fn ICurrentAppSimulatorWithConsumables();
    fn ICurrentAppStaticsWithFiltering();
    fn ICurrentAppWithCampaignId();
    fn ICurrentAppWithConsumables();
    fn ILicenseInformation();
    fn IListingInformation();
    fn IListingInformation2();
    fn IProductLicense();
    fn IProductLicenseWithFulfillment();
    fn IProductListing();
    fn IProductListing2();
    fn IProductListingWithConsumables();
    fn IProductListingWithMetadata();
    fn IProductPurchaseDisplayProperties();
    fn IProductPurchaseDisplayPropertiesFactory();
    fn IPurchaseResults();
    fn IUnfulfilledConsumable();
    fn LicenseChangedEventHandler();
    fn LicenseInformation();
    fn ListingInformation();
    fn ProductLicense();
    fn ProductListing();
    fn ProductPurchaseDisplayProperties();
    fn ProductPurchaseStatus();
    fn ProductType();
    fn PurchaseResults();
    fn UnfulfilledConsumable();
}
