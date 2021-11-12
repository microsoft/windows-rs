#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Store_LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "ApplicationModel_Store_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FulfillmentResult(pub i32);
impl FulfillmentResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NothingToFulfill: Self = Self(1i32);
    pub const PurchasePending: Self = Self(2i32);
    pub const PurchaseReverted: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
#[repr(transparent)]
pub struct ICurrentApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentApp2Statics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppSimulator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppSimulatorStaticsWithFiltering(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithCampaignId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithConsumables(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppStaticsWithFiltering(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppWithCampaignId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentAppWithConsumables(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILicenseInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListingInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListingInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductLicenseWithFulfillment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductListing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductListing2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductListingWithConsumables(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductListingWithMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductPurchaseDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProductPurchaseDisplayPropertiesFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnfulfilledConsumable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LicenseChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LicenseInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListingInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProductLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProductListing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProductPurchaseDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProductPurchaseStatus(pub i32);
impl ProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ProductType(pub i32);
impl ProductType {
    pub const Unknown: Self = Self(0i32);
    pub const Durable: Self = Self(1i32);
    pub const Consumable: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnfulfilledConsumable(pub *mut ::core::ffi::c_void);
