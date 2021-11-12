#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Store_LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "ApplicationModel_Store_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
pub struct CurrentApp(i32);
pub struct CurrentAppSimulator(i32);
pub struct FulfillmentResult(i32);
pub struct ICurrentApp(pub *mut ::core::ffi::c_void);
pub struct ICurrentApp2Statics(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppSimulator(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppSimulatorStaticsWithFiltering(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppSimulatorWithCampaignId(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppSimulatorWithConsumables(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppStaticsWithFiltering(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppWithCampaignId(pub *mut ::core::ffi::c_void);
pub struct ICurrentAppWithConsumables(pub *mut ::core::ffi::c_void);
pub struct ILicenseInformation(pub *mut ::core::ffi::c_void);
pub struct IListingInformation(pub *mut ::core::ffi::c_void);
pub struct IListingInformation2(pub *mut ::core::ffi::c_void);
pub struct IProductLicense(pub *mut ::core::ffi::c_void);
pub struct IProductLicenseWithFulfillment(pub *mut ::core::ffi::c_void);
pub struct IProductListing(pub *mut ::core::ffi::c_void);
pub struct IProductListing2(pub *mut ::core::ffi::c_void);
pub struct IProductListingWithConsumables(pub *mut ::core::ffi::c_void);
pub struct IProductListingWithMetadata(pub *mut ::core::ffi::c_void);
pub struct IProductPurchaseDisplayProperties(pub *mut ::core::ffi::c_void);
pub struct IProductPurchaseDisplayPropertiesFactory(pub *mut ::core::ffi::c_void);
pub struct IPurchaseResults(pub *mut ::core::ffi::c_void);
pub struct IUnfulfilledConsumable(pub *mut ::core::ffi::c_void);
pub struct LicenseChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct LicenseInformation(i32);
pub struct ListingInformation(i32);
pub struct ProductLicense(i32);
pub struct ProductListing(i32);
pub struct ProductPurchaseDisplayProperties(i32);
pub struct ProductPurchaseStatus(i32);
pub struct ProductType(i32);
pub struct PurchaseResults(i32);
pub struct UnfulfilledConsumable(i32);
