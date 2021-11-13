#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for FulfillmentResult {}
impl ::core::clone::Clone for FulfillmentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentApp {}
impl ::core::clone::Clone for ICurrentApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentApp2Statics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentApp2Statics {}
impl ::core::clone::Clone for ICurrentApp2Statics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppSimulator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppSimulator {}
impl ::core::clone::Clone for ICurrentAppSimulator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppSimulatorStaticsWithFiltering(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppSimulatorStaticsWithFiltering {}
impl ::core::clone::Clone for ICurrentAppSimulatorStaticsWithFiltering {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithCampaignId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppSimulatorWithCampaignId {}
impl ::core::clone::Clone for ICurrentAppSimulatorWithCampaignId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithConsumables(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppSimulatorWithConsumables {}
impl ::core::clone::Clone for ICurrentAppSimulatorWithConsumables {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppStaticsWithFiltering(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppStaticsWithFiltering {}
impl ::core::clone::Clone for ICurrentAppStaticsWithFiltering {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppWithCampaignId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppWithCampaignId {}
impl ::core::clone::Clone for ICurrentAppWithCampaignId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentAppWithConsumables(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentAppWithConsumables {}
impl ::core::clone::Clone for ICurrentAppWithConsumables {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILicenseInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILicenseInformation {}
impl ::core::clone::Clone for ILicenseInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListingInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListingInformation {}
impl ::core::clone::Clone for IListingInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListingInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListingInformation2 {}
impl ::core::clone::Clone for IListingInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductLicense(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductLicense {}
impl ::core::clone::Clone for IProductLicense {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductLicenseWithFulfillment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductLicenseWithFulfillment {}
impl ::core::clone::Clone for IProductLicenseWithFulfillment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductListing(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductListing {}
impl ::core::clone::Clone for IProductListing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductListing2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductListing2 {}
impl ::core::clone::Clone for IProductListing2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductListingWithConsumables(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductListingWithConsumables {}
impl ::core::clone::Clone for IProductListingWithConsumables {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductListingWithMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductListingWithMetadata {}
impl ::core::clone::Clone for IProductListingWithMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductPurchaseDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductPurchaseDisplayProperties {}
impl ::core::clone::Clone for IProductPurchaseDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProductPurchaseDisplayPropertiesFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProductPurchaseDisplayPropertiesFactory {}
impl ::core::clone::Clone for IProductPurchaseDisplayPropertiesFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPurchaseResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPurchaseResults {}
impl ::core::clone::Clone for IPurchaseResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnfulfilledConsumable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnfulfilledConsumable {}
impl ::core::clone::Clone for IUnfulfilledConsumable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LicenseChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LicenseChangedEventHandler {}
impl ::core::clone::Clone for LicenseChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LicenseInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LicenseInformation {}
impl ::core::clone::Clone for LicenseInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListingInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListingInformation {}
impl ::core::clone::Clone for ListingInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProductLicense(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProductLicense {}
impl ::core::clone::Clone for ProductLicense {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProductListing(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProductListing {}
impl ::core::clone::Clone for ProductListing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProductPurchaseDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProductPurchaseDisplayProperties {}
impl ::core::clone::Clone for ProductPurchaseDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProductPurchaseStatus(pub i32);
impl ProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl ::core::marker::Copy for ProductPurchaseStatus {}
impl ::core::clone::Clone for ProductPurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProductType(pub i32);
impl ProductType {
    pub const Unknown: Self = Self(0i32);
    pub const Durable: Self = Self(1i32);
    pub const Consumable: Self = Self(2i32);
}
impl ::core::marker::Copy for ProductType {}
impl ::core::clone::Clone for ProductType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PurchaseResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PurchaseResults {}
impl ::core::clone::Clone for PurchaseResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnfulfilledConsumable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnfulfilledConsumable {}
impl ::core::clone::Clone for UnfulfilledConsumable {
    fn clone(&self) -> Self {
        *self
    }
}
