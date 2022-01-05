#[cfg(feature = "implement_exclusive")]
pub trait IStoreAcquireLicenseResultImpl: Sized {
    fn StorePackageLicense(&self) -> ::windows::core::Result<StorePackageLicense>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreAppLicenseImpl: Sized {
    fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddOnLicenses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreLicense>>;
    fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsTrialOwnedByThisUser(&self) -> ::windows::core::Result<bool>;
    fn TrialUniqueId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreAppLicense2Impl: Sized {
    fn IsDiscLicense(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreAvailabilityImpl: Sized {
    fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Price(&self) -> ::windows::core::Result<StorePrice>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreCanAcquireLicenseResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn LicensableSku(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<StoreCanLicenseStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreCollectionDataImpl: Sized {
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeveloperOfferId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AcquiredDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConsumableResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StoreConsumableStatus>;
    fn TrackingId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BalanceRemaining(&self) -> ::windows::core::Result<u32>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreContextImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn OfflineLicensesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StoreContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOfflineLicensesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCustomerPurchaseIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomerCollectionsIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAppLicenseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAppLicense>>;
    fn GetStoreProductForCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>>;
    fn GetStoreProductsAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetAssociatedStoreProductsAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetAssociatedStoreProductsWithPagingAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>;
    fn GetUserCollectionAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetUserCollectionWithPagingAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>;
    fn ReportConsumableFulfillmentAsync(&self, productstoreid: &::windows::core::HSTRING, quantity: u32, trackingid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>>;
    fn GetConsumableBalanceRemainingAsync(&self, productstoreid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>>;
    fn AcquireStoreLicenseForOptionalPackageAsync(&self, optionalpackage: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAcquireLicenseResult>>;
    fn RequestPurchaseAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storeid: &::windows::core::HSTRING, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn GetAppAndOptionalStorePackageUpdatesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StorePackageUpdate>>>;
    fn RequestDownloadStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn RequestDownloadAndInstallStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn RequestDownloadAndInstallStorePackagesAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreContext2Impl: Sized {
    fn FindStoreProductForPackageAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, package: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreContext3Impl: Sized {
    fn CanSilentlyDownloadStorePackageUpdates(&self) -> ::windows::core::Result<bool>;
    fn TrySilentDownloadStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn TrySilentDownloadAndInstallStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn CanAcquireStoreLicenseForOptionalPackageAsync(&self, optionalpackage: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>;
    fn CanAcquireStoreLicenseAsync(&self, productstoreid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>;
    fn GetStoreProductsWithOptionsAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storeproductoptions: &::core::option::Option<StoreProductOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetAssociatedStoreQueueItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>;
    fn GetStoreQueueItemsAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>;
    fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storepackageinstalloptions: &::core::option::Option<StorePackageInstallOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn DownloadAndInstallStorePackagesAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn RequestUninstallStorePackageAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
    fn RequestUninstallStorePackageByStoreIdAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
    fn UninstallStorePackageAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
    fn UninstallStorePackageByStoreIdAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreContext4Impl: Sized {
    fn RequestRateAndReviewAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreRateAndReviewResult>>;
    fn SetInstallOrderForAssociatedStoreQueueItemsAsync(&self, items: &::core::option::Option<super::super::Foundation::Collections::IIterable<StoreQueueItem>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreContextStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<StoreContext>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StoreContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreImageImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn ImagePurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreLicenseImpl: Sized {
    fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePackageInstallOptionsImpl: Sized {
    fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorePackageLicenseImpl: Sized + IClosableImpl {
    fn LicenseLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StorePackageLicense, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLicenseLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn IsValid(&self) -> ::windows::core::Result<bool>;
    fn ReleaseLicense(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePackageUpdateImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn Mandatory(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePackageUpdateResultImpl: Sized {
    fn OverallState(&self) -> ::windows::core::Result<StorePackageUpdateState>;
    fn StorePackageUpdateStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorePackageUpdateStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePackageUpdateResult2Impl: Sized {
    fn StoreQueueItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePriceImpl: Sized {
    fn FormattedBasePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOnSale(&self) -> ::windows::core::Result<bool>;
    fn SaleEndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedRecurrencePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreProductImpl: Sized {
    fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasDigitalDownload(&self) -> ::windows::core::Result<bool>;
    fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>>;
    fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>>;
    fn Skus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreSku>>;
    fn IsInUserCollection(&self) -> ::windows::core::Result<bool>;
    fn Price(&self) -> ::windows::core::Result<StorePrice>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinkUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetIsAnySkuInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreProductOptionsImpl: Sized {
    fn ActionFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreProductPagedQueryResultImpl: Sized {
    fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>>;
    fn HasMoreResults(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreProductQueryResultImpl: Sized {
    fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreProductResultImpl: Sized {
    fn Product(&self) -> ::windows::core::Result<StoreProduct>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePurchasePropertiesImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExtendedJsonData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePurchasePropertiesFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<StorePurchaseProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePurchaseResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StorePurchaseStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreQueueItemImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallKind(&self) -> ::windows::core::Result<StoreQueueItemKind>;
    fn GetCurrentStatus(&self) -> ::windows::core::Result<StoreQueueItemStatus>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StoreQueueItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreQueueItem2Impl: Sized {
    fn CancelInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreQueueItemCompletedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StoreQueueItemStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreQueueItemStatusImpl: Sized {
    fn PackageInstallState(&self) -> ::windows::core::Result<StoreQueueItemState>;
    fn PackageInstallExtendedState(&self) -> ::windows::core::Result<StoreQueueItemExtendedState>;
    fn UpdateStatus(&self) -> ::windows::core::Result<StorePackageUpdateStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreRateAndReviewResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WasUpdated(&self) -> ::windows::core::Result<bool>;
    fn Status(&self) -> ::windows::core::Result<StoreRateAndReviewStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreRequestHelperStaticsImpl: Sized {
    fn SendRequestAsync(&self, context: &::core::option::Option<StoreContext>, requestkind: u32, parametersasjson: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreSendRequestResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreSendRequestResultImpl: Sized {
    fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreSendRequestResult2Impl: Sized {
    fn HttpStatusCode(&self) -> ::windows::core::Result<super::super::Web::Http::HttpStatusCode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreSkuImpl: Sized {
    fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>>;
    fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>>;
    fn Availabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreAvailability>>;
    fn Price(&self) -> ::windows::core::Result<StorePrice>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsInUserCollection(&self) -> ::windows::core::Result<bool>;
    fn BundledSkus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CollectionData(&self) -> ::windows::core::Result<StoreCollectionData>;
    fn GetIsInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn IsSubscription(&self) -> ::windows::core::Result<bool>;
    fn SubscriptionInfo(&self) -> ::windows::core::Result<StoreSubscriptionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreSubscriptionInfoImpl: Sized {
    fn BillingPeriod(&self) -> ::windows::core::Result<u32>;
    fn BillingPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit>;
    fn HasTrialPeriod(&self) -> ::windows::core::Result<bool>;
    fn TrialPeriod(&self) -> ::windows::core::Result<u32>;
    fn TrialPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreUninstallStorePackageResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Status(&self) -> ::windows::core::Result<StoreUninstallStorePackageStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreVideoImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn VideoPurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PreviewImage(&self) -> ::windows::core::Result<StoreImage>;
}
