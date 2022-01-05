#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppImpl: Sized {
    fn LicenseInformation(&self) -> ::windows::core::Result<LicenseInformation>;
    fn LinkUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestAppPurchaseAsync(&self, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestProductPurchaseAsync(&self, productid: &::windows::core::HSTRING, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn LoadListingInformationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn GetAppReceiptAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetProductReceiptAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentApp2StaticsImpl: Sized {
    fn GetCustomerPurchaseIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomerCollectionsIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppSimulatorImpl: Sized {
    fn LicenseInformation(&self) -> ::windows::core::Result<LicenseInformation>;
    fn LinkUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestAppPurchaseAsync(&self, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestProductPurchaseAsync(&self, productid: &::windows::core::HSTRING, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn LoadListingInformationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn GetAppReceiptAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetProductReceiptAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReloadSimulatorAsync(&self, simulatorsettingsfile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppSimulatorStaticsWithFilteringImpl: Sized {
    fn LoadListingInformationByProductIdsAsync(&self, productids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn LoadListingInformationByKeywordsAsync(&self, keywords: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppSimulatorWithCampaignIdImpl: Sized {
    fn GetAppPurchaseCampaignIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppSimulatorWithConsumablesImpl: Sized {
    fn ReportConsumableFulfillmentAsync(&self, productid: &::windows::core::HSTRING, transactionid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>>;
    fn RequestProductPurchaseWithResultsAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn RequestProductPurchaseWithDisplayPropertiesAsync(&self, productid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, displayproperties: &::core::option::Option<ProductPurchaseDisplayProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn GetUnfulfilledConsumablesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppStaticsWithFilteringImpl: Sized {
    fn LoadListingInformationByProductIdsAsync(&self, productids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn LoadListingInformationByKeywordsAsync(&self, keywords: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn ReportProductFulfillment(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppWithCampaignIdImpl: Sized {
    fn GetAppPurchaseCampaignIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentAppWithConsumablesImpl: Sized {
    fn ReportConsumableFulfillmentAsync(&self, productid: &::windows::core::HSTRING, transactionid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>>;
    fn RequestProductPurchaseWithResultsAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn RequestProductPurchaseWithDisplayPropertiesAsync(&self, productid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, displayproperties: &::core::option::Option<ProductPurchaseDisplayProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn GetUnfulfilledConsumablesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseInformationImpl: Sized {
    fn ProductLicenses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ProductLicense>>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LicenseChanged(&self, handler: &::core::option::Option<LicenseChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLicenseChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListingInformationImpl: Sized {
    fn CurrentMarket(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductListings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ProductListing>>;
    fn FormattedPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AgeRating(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IListingInformation2Impl: Sized {
    fn FormattedBasePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaleEndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsOnSale(&self) -> ::windows::core::Result<bool>;
    fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductLicenseImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductLicenseWithFulfillmentImpl: Sized + IProductLicenseImpl {
    fn IsConsumable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductListingImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductListing2Impl: Sized {
    fn FormattedBasePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaleEndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsOnSale(&self) -> ::windows::core::Result<bool>;
    fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductListingWithConsumablesImpl: Sized {
    fn ProductType(&self) -> ::windows::core::Result<ProductType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductListingWithMetadataImpl: Sized + IProductListingImpl {
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>;
    fn ProductType(&self) -> ::windows::core::Result<ProductType>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ImageUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductPurchaseDisplayPropertiesImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetImage(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductPurchaseDisplayPropertiesFactoryImpl: Sized {
    fn CreateProductPurchaseDisplayProperties(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ProductPurchaseDisplayProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPurchaseResultsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ProductPurchaseStatus>;
    fn TransactionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReceiptXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OfferId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnfulfilledConsumableImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransactionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OfferId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
