#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAcquireLicenseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreAcquireLicenseResult {
    type Vtable = IStoreAcquireLicenseResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd7946d_f040_4cb3_9a39_29bcecdbe22d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAcquireLicenseResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StorePackageLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAppLicense(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreAppLicense {
    type Vtable = IStoreAppLicense_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf389f9de_73c0_45ce_9bab_b2fe3e5eafd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAppLicense_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddOnLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddOnLicenses: usize,
    #[cfg(feature = "Foundation")]
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrialTimeRemaining: usize,
    pub IsTrialOwnedByThisUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrialUniqueId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAppLicense2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreAppLicense2 {
    type Vtable = IStoreAppLicense2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4666e91_4443_40b3_993f_28904435bdc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAppLicense2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsDiscLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAvailability(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreAvailability {
    type Vtable = IStoreAvailability_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa060325_0ffd_4493_ad43_f1f9918f69fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAvailability_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndDate: usize,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreCanAcquireLicenseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreCanAcquireLicenseResult {
    type Vtable = IStoreCanAcquireLicenseResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a693db3_0088_482f_86d5_bd46522663ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreCanAcquireLicenseResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub LicensableSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreCanLicenseStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreCollectionData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreCollectionData {
    type Vtable = IStoreCollectionData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8aa4c3b3_5bb3_441a_2ab4_4dab73d5ce67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreCollectionData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeveloperOfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AcquiredDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcquiredDate: usize,
    #[cfg(feature = "Foundation")]
    pub StartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDate: usize,
    #[cfg(feature = "Foundation")]
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndDate: usize,
    #[cfg(feature = "Foundation")]
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrialTimeRemaining: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConsumableResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreConsumableResult {
    type Vtable = IStoreConsumableResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea5dab72_6a00_4052_be5b_bfdab4433352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConsumableResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreConsumableStatus) -> ::windows::core::HRESULT,
    pub TrackingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BalanceRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreContext {
    type Vtable = IStoreContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac98b6be_f4fd_4912_babd_5035e5e8bcab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "Foundation")]
    pub OfflineLicensesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OfflineLicensesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOfflineLicensesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOfflineLicensesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerPurchaseIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerCollectionsIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppLicenseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetStoreProductForCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStoreProductForCurrentAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreProductsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreProductsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreProductsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreProductsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreProductsWithPagingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, maxitemstoretrieveperpage: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreProductsWithPagingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUserCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUserCollectionAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUserCollectionWithPagingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, maxitemstoretrieveperpage: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUserCollectionWithPagingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quantity: u32, trackingid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConsumableBalanceRemainingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConsumableBalanceRemainingAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub AcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    AcquireStoreLicenseForOptionalPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAndOptionalStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAndOptionalStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreContext2 {
    type Vtable = IStoreContext2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18bc54da_7bd9_452c_9116_3bbd06ffc63a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindStoreProductForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, package: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindStoreProductForPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreContext3 {
    type Vtable = IStoreContext3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe26226ca_1a01_4730_85a6_ecc896e4ae38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanSilentlyDownloadStorePackageUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySilentDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySilentDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySilentDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySilentDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub CanAcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    CanAcquireStoreLicenseForOptionalPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CanAcquireStoreLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanAcquireStoreLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreProductsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, storeids: ::windows::core::RawPtr, storeproductoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreProductsWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreQueueItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreQueueItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, storepackageinstalloptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DownloadAndInstallStorePackagesAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub RequestUninstallStorePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    RequestUninstallStorePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUninstallStorePackageByStoreIdAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub UninstallStorePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    UninstallStorePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UninstallStorePackageByStoreIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreContext4 {
    type Vtable = IStoreContext4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf9c6f69_bea1_4bf4_8e74_ae03e206c6b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestRateAndReviewAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRateAndReviewAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInstallOrderForAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, items: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInstallOrderForAssociatedStoreQueueItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContextStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreContextStatics {
    type Vtable = IStoreContextStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c06ee5f_15c0_4e72_9330_d6191cebd19c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContextStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreImage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreImage {
    type Vtable = IStoreImage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081fd248_adb4_4b64_a993_784789926ed5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreImage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub ImagePurposeTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreLicense(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreLicense {
    type Vtable = IStoreLicense_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26dc9579_4c4f_4f30_bc89_649f60e36055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreLicense_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageInstallOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePackageInstallOptions {
    type Vtable = IStorePackageInstallOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d3d630c_0ccd_44dd_8c59_80810a729973);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageInstallOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageLicense(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePackageLicense {
    type Vtable = IStorePackageLicense_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c465714_14e1_4973_bd14_f77724271e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageLicense_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LicenseLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLicenseLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLicenseLost: usize,
    #[cfg(feature = "ApplicationModel")]
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Package: usize,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ReleaseLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePackageUpdate {
    type Vtable = IStorePackageUpdate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x140fa150_3cbf_4a35_b91f_48271c31b072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdate_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Package: usize,
    pub Mandatory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdateResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePackageUpdateResult {
    type Vtable = IStorePackageUpdateResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79142ed_61f9_4893_b4fe_cf191603af7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdateResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OverallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePackageUpdateState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorePackageUpdateStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorePackageUpdateStatuses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdateResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePackageUpdateResult2 {
    type Vtable = IStorePackageUpdateResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x071d012e_bc62_4f2e_87ea_99d801aeaf98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdateResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StoreQueueItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StoreQueueItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePrice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePrice {
    type Vtable = IStorePrice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ba94c4_15f1_407c_8f06_006380f4df0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePrice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormattedRecurrencePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProduct(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreProduct {
    type Vtable = IStoreProduct_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320e2c52_d760_450a_a42b_67d1e901ac90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProduct_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProductKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasDigitalDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Images: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Images: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Videos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Skus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Skus: usize,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAnySkuInstalledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAnySkuInstalledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreProductOptions {
    type Vtable = IStoreProductOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b34a0f9_a113_4811_8326_16199c927f31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ActionFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActionFilters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductPagedQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreProductPagedQueryResult {
    type Vtable = IStoreProductPagedQueryResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc92718c5_4dd5_4869_a462_ecc6872e43c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductPagedQueryResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Products: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Products: usize,
    pub HasMoreResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNextAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreProductQueryResult {
    type Vtable = IStoreProductQueryResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd805e6c5_d456_4ff6_8049_9076d5165f73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductQueryResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Products: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Products: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreProductResult {
    type Vtable = IStoreProductResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchaseProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePurchaseProperties {
    type Vtable = IStorePurchaseProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x836278f3_ff87_4364_a5b4_fd2153ebe43b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchaseProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchasePropertiesFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePurchasePropertiesFactory {
    type Vtable = IStorePurchasePropertiesFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa768f59e_fefd_489f_9a17_22a593e68b9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchasePropertiesFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchaseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePurchaseResult {
    type Vtable = IStorePurchaseResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadd28552_f96a_463d_a7bb_c20b4fca6952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchaseResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePurchaseStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreQueueItem {
    type Vtable = IStoreQueueItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56d5c32b_f830_4293_9188_cad2dcde7357);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InstallKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemKind) -> ::windows::core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreQueueItem2 {
    type Vtable = IStoreQueueItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69491ca8_1ad4_447c_ad8c_a95035f64d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItem2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CancelInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PauseInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeInstallAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItemCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreQueueItemCompletedEventArgs {
    type Vtable = IStoreQueueItemCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1247df6c_b44a_439b_bb07_1d3003d005c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItemCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItemStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreQueueItemStatus {
    type Vtable = IStoreQueueItemStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bd6796f_9cc3_4ec3_b2ef_7be433b30174);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItemStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PackageInstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemState) -> ::windows::core::HRESULT,
    pub PackageInstallExtendedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemExtendedState) -> ::windows::core::HRESULT,
    pub UpdateStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<StorePackageUpdateStatus>) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreRateAndReviewResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreRateAndReviewResult {
    type Vtable = IStoreRateAndReviewResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d209d56_a6b5_4121_9b61_ee6d0fbdbdbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreRateAndReviewResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WasUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreRateAndReviewStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreRequestHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreRequestHelperStatics {
    type Vtable = IStoreRequestHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ce5e5f9_a0c9_4b2c_96a6_a171c630038d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreRequestHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, requestkind: u32, parametersasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSendRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreSendRequestResult {
    type Vtable = IStoreSendRequestResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc73abe60_8272_4502_8a69_6e75153a4299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSendRequestResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSendRequestResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreSendRequestResult2 {
    type Vtable = IStoreSendRequestResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2901296f_c0b0_49d0_8e8d_aa940af9c10b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSendRequestResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Web_Http")]
    pub HttpStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Web::Http::HttpStatusCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpStatusCode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSku(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreSku {
    type Vtable = IStoreSku_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x397e6f55_4440_4f03_863c_91f3fec83d79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSku_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Images: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Images: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Videos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Availabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Availabilities: usize,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BundledSkus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BundledSkus: usize,
    pub CollectionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsInstalledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsInstalledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    pub IsSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SubscriptionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSubscriptionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreSubscriptionInfo {
    type Vtable = IStoreSubscriptionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4189776a_0559_43ac_a9c6_3ab0011fb8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSubscriptionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BillingPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub BillingPeriodUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows::core::HRESULT,
    pub HasTrialPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrialPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TrialPeriodUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreUninstallStorePackageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreUninstallStorePackageResult {
    type Vtable = IStoreUninstallStorePackageResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fca39fd_126f_4cda_b801_1346b8d0a260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreUninstallStorePackageResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreUninstallStorePackageStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreVideo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreVideo {
    type Vtable = IStoreVideo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf26cb184_6f5e_4dc2_886c_3c63083c2f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreVideo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub VideoPurposeTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PreviewImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreAcquireLicenseResult(::windows::core::IUnknown);
impl StoreAcquireLicenseResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn StorePackageLicense(&self) -> ::windows::core::Result<StorePackageLicense> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StorePackageLicense)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePackageLicense>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAcquireLicenseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreAcquireLicenseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreAcquireLicenseResult {}
impl ::core::fmt::Debug for StoreAcquireLicenseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreAcquireLicenseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreAcquireLicenseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAcquireLicenseResult;{fbd7946d-f040-4cb3-9a39-29bcecdbe22d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreAcquireLicenseResult {
    type Vtable = IStoreAcquireLicenseResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreAcquireLicenseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.StoreAcquireLicenseResult";
}
impl ::core::convert::From<StoreAcquireLicenseResult> for ::windows::core::IUnknown {
    fn from(value: StoreAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAcquireLicenseResult> for ::windows::core::IUnknown {
    fn from(value: &StoreAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreAcquireLicenseResult> for ::windows::core::IInspectable {
    fn from(value: StoreAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAcquireLicenseResult> for ::windows::core::IInspectable {
    fn from(value: &StoreAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreAcquireLicenseResult {}
unsafe impl ::core::marker::Sync for StoreAcquireLicenseResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreAppLicense(::windows::core::IUnknown);
impl StoreAppLicense {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SkuStoreId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTrial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpirationDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddOnLicenses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreLicense>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddOnLicenses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreLicense>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrialTimeRemaining)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsTrialOwnedByThisUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTrialOwnedByThisUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn TrialUniqueId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrialUniqueId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsDiscLicense(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStoreAppLicense2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDiscLicense)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAppLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreAppLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreAppLicense {}
impl ::core::fmt::Debug for StoreAppLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreAppLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreAppLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAppLicense;{f389f9de-73c0-45ce-9bab-b2fe3e5eafd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreAppLicense {
    type Vtable = IStoreAppLicense_Vtbl;
    const IID: ::windows::core::GUID = <IStoreAppLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreAppLicense {
    const NAME: &'static str = "Windows.Services.Store.StoreAppLicense";
}
impl ::core::convert::From<StoreAppLicense> for ::windows::core::IUnknown {
    fn from(value: StoreAppLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAppLicense> for ::windows::core::IUnknown {
    fn from(value: &StoreAppLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreAppLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreAppLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreAppLicense> for ::windows::core::IInspectable {
    fn from(value: StoreAppLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAppLicense> for ::windows::core::IInspectable {
    fn from(value: &StoreAppLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreAppLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreAppLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreAppLicense {}
unsafe impl ::core::marker::Sync for StoreAppLicense {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreAvailability(::windows::core::IUnknown);
impl StoreAvailability {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Price(&self) -> ::windows::core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Price)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePrice>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, StorePurchaseProperties>>(&self, storepurchaseproperties: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::core::mem::transmute_copy(this), storepurchaseproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAvailability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreAvailability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreAvailability {}
impl ::core::fmt::Debug for StoreAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAvailability;{fa060325-0ffd-4493-ad43-f1f9918f69fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreAvailability {
    type Vtable = IStoreAvailability_Vtbl;
    const IID: ::windows::core::GUID = <IStoreAvailability as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreAvailability {
    const NAME: &'static str = "Windows.Services.Store.StoreAvailability";
}
impl ::core::convert::From<StoreAvailability> for ::windows::core::IUnknown {
    fn from(value: StoreAvailability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAvailability> for ::windows::core::IUnknown {
    fn from(value: &StoreAvailability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreAvailability> for ::windows::core::IInspectable {
    fn from(value: StoreAvailability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAvailability> for ::windows::core::IInspectable {
    fn from(value: &StoreAvailability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreAvailability {}
unsafe impl ::core::marker::Sync for StoreAvailability {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreCanAcquireLicenseResult(::windows::core::IUnknown);
impl StoreCanAcquireLicenseResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn LicensableSku(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LicensableSku)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StoreCanLicenseStatus> {
        let this = self;
        unsafe {
            let mut result__: StoreCanLicenseStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreCanLicenseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreCanAcquireLicenseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreCanAcquireLicenseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreCanAcquireLicenseResult {}
impl ::core::fmt::Debug for StoreCanAcquireLicenseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreCanAcquireLicenseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreCanAcquireLicenseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreCanAcquireLicenseResult;{3a693db3-0088-482f-86d5-bd46522663ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreCanAcquireLicenseResult {
    type Vtable = IStoreCanAcquireLicenseResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreCanAcquireLicenseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreCanAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.StoreCanAcquireLicenseResult";
}
impl ::core::convert::From<StoreCanAcquireLicenseResult> for ::windows::core::IUnknown {
    fn from(value: StoreCanAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCanAcquireLicenseResult> for ::windows::core::IUnknown {
    fn from(value: &StoreCanAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreCanAcquireLicenseResult> for ::windows::core::IInspectable {
    fn from(value: StoreCanAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCanAcquireLicenseResult> for ::windows::core::IInspectable {
    fn from(value: &StoreCanAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreCanAcquireLicenseResult {}
unsafe impl ::core::marker::Sync for StoreCanAcquireLicenseResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreCanLicenseStatus(pub i32);
impl StoreCanLicenseStatus {
    pub const NotLicensableToUser: Self = Self(0i32);
    pub const Licensable: Self = Self(1i32);
    pub const LicenseActionNotApplicableToProduct: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreCanLicenseStatus {}
impl ::core::clone::Clone for StoreCanLicenseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreCanLicenseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreCanLicenseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreCanLicenseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreCanLicenseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreCanLicenseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreCanLicenseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreCollectionData(::windows::core::IUnknown);
impl StoreCollectionData {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTrial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CampaignId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn DeveloperOfferId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeveloperOfferId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcquiredDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcquiredDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrialTimeRemaining)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreCollectionData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreCollectionData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreCollectionData {}
impl ::core::fmt::Debug for StoreCollectionData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreCollectionData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreCollectionData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreCollectionData;{8aa4c3b3-5bb3-441a-2ab4-4dab73d5ce67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreCollectionData {
    type Vtable = IStoreCollectionData_Vtbl;
    const IID: ::windows::core::GUID = <IStoreCollectionData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreCollectionData {
    const NAME: &'static str = "Windows.Services.Store.StoreCollectionData";
}
impl ::core::convert::From<StoreCollectionData> for ::windows::core::IUnknown {
    fn from(value: StoreCollectionData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCollectionData> for ::windows::core::IUnknown {
    fn from(value: &StoreCollectionData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreCollectionData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreCollectionData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreCollectionData> for ::windows::core::IInspectable {
    fn from(value: StoreCollectionData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCollectionData> for ::windows::core::IInspectable {
    fn from(value: &StoreCollectionData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreCollectionData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreCollectionData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreCollectionData {}
unsafe impl ::core::marker::Sync for StoreCollectionData {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreConsumableResult(::windows::core::IUnknown);
impl StoreConsumableResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StoreConsumableStatus> {
        let this = self;
        unsafe {
            let mut result__: StoreConsumableStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreConsumableStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn TrackingId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackingId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn BalanceRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BalanceRemaining)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreConsumableResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreConsumableResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreConsumableResult {}
impl ::core::fmt::Debug for StoreConsumableResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreConsumableResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreConsumableResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreConsumableResult;{ea5dab72-6a00-4052-be5b-bfdab4433352})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreConsumableResult {
    type Vtable = IStoreConsumableResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreConsumableResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreConsumableResult {
    const NAME: &'static str = "Windows.Services.Store.StoreConsumableResult";
}
impl ::core::convert::From<StoreConsumableResult> for ::windows::core::IUnknown {
    fn from(value: StoreConsumableResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreConsumableResult> for ::windows::core::IUnknown {
    fn from(value: &StoreConsumableResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreConsumableResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreConsumableResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreConsumableResult> for ::windows::core::IInspectable {
    fn from(value: StoreConsumableResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreConsumableResult> for ::windows::core::IInspectable {
    fn from(value: &StoreConsumableResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreConsumableResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreConsumableResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreConsumableResult {}
unsafe impl ::core::marker::Sync for StoreConsumableResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreConsumableStatus(pub i32);
impl StoreConsumableStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const InsufficentQuantity: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for StoreConsumableStatus {}
impl ::core::clone::Clone for StoreConsumableStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreConsumableStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreConsumableStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreConsumableStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreConsumableStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreConsumableStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreConsumableStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreContext(::windows::core::IUnknown);
impl StoreContext {
    #[doc = "*Required features: `\"Services_Store\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OfflineLicensesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StoreContext, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OfflineLicensesChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOfflineLicensesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOfflineLicensesChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomerPurchaseIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, serviceticket: Param0, publisheruserid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCustomerPurchaseIdAsync)(::core::mem::transmute_copy(this), serviceticket.into_param().abi(), publisheruserid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomerCollectionsIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, serviceticket: Param0, publisheruserid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCustomerCollectionsIdAsync)(::core::mem::transmute_copy(this), serviceticket.into_param().abi(), publisheruserid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppLicenseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAppLicense>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppLicenseAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreAppLicense>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStoreProductForCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreProductForCurrentAppAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStoreProductsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, productkinds: Param0, storeids: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreProductsAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), storeids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAssociatedStoreProductsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, productkinds: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAssociatedStoreProductsAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAssociatedStoreProductsWithPagingAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, productkinds: Param0, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAssociatedStoreProductsWithPagingAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), maxitemstoretrieveperpage, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUserCollectionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, productkinds: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUserCollectionAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUserCollectionWithPagingAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, productkinds: Param0, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUserCollectionWithPagingAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), maxitemstoretrieveperpage, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportConsumableFulfillmentAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, productstoreid: Param0, quantity: u32, trackingid: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(::core::mem::transmute_copy(this), productstoreid.into_param().abi(), quantity, trackingid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreConsumableResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConsumableBalanceRemainingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productstoreid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConsumableBalanceRemainingAsync)(::core::mem::transmute_copy(this), productstoreid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreConsumableResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn AcquireStoreLicenseForOptionalPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>>(&self, optionalpackage: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAcquireLicenseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcquireStoreLicenseForOptionalPackageAsync)(::core::mem::transmute_copy(this), optionalpackage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreAcquireLicenseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseAsync)(::core::mem::transmute_copy(this), storeid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StorePurchaseProperties>>(&self, storeid: Param0, storepurchaseproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::core::mem::transmute_copy(this), storeid.into_param().abi(), storepurchaseproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppAndOptionalStorePackageUpdatesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StorePackageUpdate>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAppAndOptionalStorePackageUpdatesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StorePackageUpdate>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadStorePackageUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestDownloadStorePackageUpdatesAsync)(::core::mem::transmute_copy(this), storepackageupdates.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadAndInstallStorePackageUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestDownloadAndInstallStorePackageUpdatesAsync)(::core::mem::transmute_copy(this), storepackageupdates.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadAndInstallStorePackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, storeids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestDownloadAndInstallStorePackagesAsync)(::core::mem::transmute_copy(this), storeids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindStoreProductForPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>>(&self, productkinds: Param0, package: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindStoreProductForPackageAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), package.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn CanSilentlyDownloadStorePackageUpdates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanSilentlyDownloadStorePackageUpdates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrySilentDownloadStorePackageUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySilentDownloadStorePackageUpdatesAsync)(::core::mem::transmute_copy(this), storepackageupdates.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrySilentDownloadAndInstallStorePackageUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySilentDownloadAndInstallStorePackageUpdatesAsync)(::core::mem::transmute_copy(this), storepackageupdates.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn CanAcquireStoreLicenseForOptionalPackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>>(&self, optionalpackage: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanAcquireStoreLicenseForOptionalPackageAsync)(::core::mem::transmute_copy(this), optionalpackage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanAcquireStoreLicenseAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productstoreid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanAcquireStoreLicenseAsync)(::core::mem::transmute_copy(this), productstoreid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStoreProductsWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, StoreProductOptions>>(&self, productkinds: Param0, storeids: Param1, storeproductoptions: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreProductsWithOptionsAsync)(::core::mem::transmute_copy(this), productkinds.into_param().abi(), storeids.into_param().abi(), storeproductoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAssociatedStoreQueueItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAssociatedStoreQueueItemsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStoreQueueItemsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, storeids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreQueueItemsAsync)(::core::mem::transmute_copy(this), storeids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, StorePackageInstallOptions>>(&self, storeids: Param0, storepackageinstalloptions: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync)(::core::mem::transmute_copy(this), storeids.into_param().abi(), storepackageinstalloptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DownloadAndInstallStorePackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, storeids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadAndInstallStorePackagesAsync)(::core::mem::transmute_copy(this), storeids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn RequestUninstallStorePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>>(&self, package: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestUninstallStorePackageAsync)(::core::mem::transmute_copy(this), package.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUninstallStorePackageByStoreIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestUninstallStorePackageByStoreIdAsync)(::core::mem::transmute_copy(this), storeid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn UninstallStorePackageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Package>>(&self, package: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UninstallStorePackageAsync)(::core::mem::transmute_copy(this), package.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UninstallStorePackageByStoreIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UninstallStorePackageByStoreIdAsync)(::core::mem::transmute_copy(this), storeid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRateAndReviewAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreRateAndReviewResult>> {
        let this = &::windows::core::Interface::cast::<IStoreContext4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestRateAndReviewAppAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreRateAndReviewResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInstallOrderForAssociatedStoreQueueItemsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StoreQueueItem>>>(&self, items: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows::core::Interface::cast::<IStoreContext4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetInstallOrderForAssociatedStoreQueueItemsAsync)(::core::mem::transmute_copy(this), items.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<StoreContext> {
        Self::IStoreContextStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreContext>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<StoreContext> {
        Self::IStoreContextStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<StoreContext>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStoreContextStatics<R, F: FnOnce(&IStoreContextStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreContext, IStoreContextStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StoreContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreContext {}
impl ::core::fmt::Debug for StoreContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreContext;{ac98b6be-f4fd-4912-babd-5035e5e8bcab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreContext {
    type Vtable = IStoreContext_Vtbl;
    const IID: ::windows::core::GUID = <IStoreContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreContext {
    const NAME: &'static str = "Windows.Services.Store.StoreContext";
}
impl ::core::convert::From<StoreContext> for ::windows::core::IUnknown {
    fn from(value: StoreContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreContext> for ::windows::core::IUnknown {
    fn from(value: &StoreContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreContext> for ::windows::core::IInspectable {
    fn from(value: StoreContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreContext> for ::windows::core::IInspectable {
    fn from(value: &StoreContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreContext {}
unsafe impl ::core::marker::Sync for StoreContext {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreDurationUnit(pub i32);
impl StoreDurationUnit {
    pub const Minute: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const Day: Self = Self(2i32);
    pub const Week: Self = Self(3i32);
    pub const Month: Self = Self(4i32);
    pub const Year: Self = Self(5i32);
}
impl ::core::marker::Copy for StoreDurationUnit {}
impl ::core::clone::Clone for StoreDurationUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreDurationUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreDurationUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreDurationUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreDurationUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreDurationUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreDurationUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreImage(::windows::core::IUnknown);
impl StoreImage {
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ImagePurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImagePurposeTag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Caption)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreImage {}
impl ::core::fmt::Debug for StoreImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreImage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreImage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreImage;{081fd248-adb4-4b64-a993-784789926ed5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreImage {
    type Vtable = IStoreImage_Vtbl;
    const IID: ::windows::core::GUID = <IStoreImage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreImage {
    const NAME: &'static str = "Windows.Services.Store.StoreImage";
}
impl ::core::convert::From<StoreImage> for ::windows::core::IUnknown {
    fn from(value: StoreImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreImage> for ::windows::core::IUnknown {
    fn from(value: &StoreImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreImage> for ::windows::core::IInspectable {
    fn from(value: StoreImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreImage> for ::windows::core::IInspectable {
    fn from(value: &StoreImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreImage {}
unsafe impl ::core::marker::Sync for StoreImage {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreLicense(::windows::core::IUnknown);
impl StoreLicense {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SkuStoreId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpirationDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InAppOfferToken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreLicense {}
impl ::core::fmt::Debug for StoreLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreLicense;{26dc9579-4c4f-4f30-bc89-649f60e36055})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreLicense {
    type Vtable = IStoreLicense_Vtbl;
    const IID: ::windows::core::GUID = <IStoreLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreLicense {
    const NAME: &'static str = "Windows.Services.Store.StoreLicense";
}
impl ::core::convert::From<StoreLicense> for ::windows::core::IUnknown {
    fn from(value: StoreLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreLicense> for ::windows::core::IUnknown {
    fn from(value: &StoreLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreLicense> for ::windows::core::IInspectable {
    fn from(value: StoreLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreLicense> for ::windows::core::IInspectable {
    fn from(value: &StoreLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreLicense {}
unsafe impl ::core::marker::Sync for StoreLicense {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageInstallOptions(::windows::core::IUnknown);
impl StorePackageInstallOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorePackageInstallOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowForcedAppRestart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowForcedAppRestart)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for StorePackageInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageInstallOptions {}
impl ::core::fmt::Debug for StorePackageInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageInstallOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageInstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageInstallOptions;{1d3d630c-0ccd-44dd-8c59-80810a729973})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePackageInstallOptions {
    type Vtable = IStorePackageInstallOptions_Vtbl;
    const IID: ::windows::core::GUID = <IStorePackageInstallOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageInstallOptions {
    const NAME: &'static str = "Windows.Services.Store.StorePackageInstallOptions";
}
impl ::core::convert::From<StorePackageInstallOptions> for ::windows::core::IUnknown {
    fn from(value: StorePackageInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageInstallOptions> for ::windows::core::IUnknown {
    fn from(value: &StorePackageInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePackageInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePackageInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageInstallOptions> for ::windows::core::IInspectable {
    fn from(value: StorePackageInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageInstallOptions> for ::windows::core::IInspectable {
    fn from(value: &StorePackageInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePackageInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePackageInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePackageInstallOptions {}
unsafe impl ::core::marker::Sync for StorePackageInstallOptions {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageLicense(::windows::core::IUnknown);
impl StorePackageLicense {
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StorePackageLicense, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LicenseLost)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLicenseLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLicenseLost)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Package>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsValid(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsValid)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ReleaseLicense(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleaseLicense)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for StorePackageLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageLicense {}
impl ::core::fmt::Debug for StorePackageLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageLicense;{0c465714-14e1-4973-bd14-f77724271e99})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePackageLicense {
    type Vtable = IStorePackageLicense_Vtbl;
    const IID: ::windows::core::GUID = <IStorePackageLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageLicense {
    const NAME: &'static str = "Windows.Services.Store.StorePackageLicense";
}
impl ::core::convert::From<StorePackageLicense> for ::windows::core::IUnknown {
    fn from(value: StorePackageLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageLicense> for ::windows::core::IUnknown {
    fn from(value: &StorePackageLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePackageLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePackageLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageLicense> for ::windows::core::IInspectable {
    fn from(value: StorePackageLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageLicense> for ::windows::core::IInspectable {
    fn from(value: &StorePackageLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePackageLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePackageLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<StorePackageLicense> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StorePackageLicense) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StorePackageLicense> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorePackageLicense) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for StorePackageLicense {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &StorePackageLicense {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StorePackageLicense {}
unsafe impl ::core::marker::Sync for StorePackageLicense {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageUpdate(::windows::core::IUnknown);
impl StorePackageUpdate {
    #[doc = "*Required features: `\"Services_Store\"`, `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Package)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Package>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Mandatory(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mandatory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePackageUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageUpdate {}
impl ::core::fmt::Debug for StorePackageUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageUpdate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageUpdate;{140fa150-3cbf-4a35-b91f-48271c31b072})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePackageUpdate {
    type Vtable = IStorePackageUpdate_Vtbl;
    const IID: ::windows::core::GUID = <IStorePackageUpdate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageUpdate {
    const NAME: &'static str = "Windows.Services.Store.StorePackageUpdate";
}
impl ::core::convert::From<StorePackageUpdate> for ::windows::core::IUnknown {
    fn from(value: StorePackageUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdate> for ::windows::core::IUnknown {
    fn from(value: &StorePackageUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePackageUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePackageUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageUpdate> for ::windows::core::IInspectable {
    fn from(value: StorePackageUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdate> for ::windows::core::IInspectable {
    fn from(value: &StorePackageUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePackageUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePackageUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePackageUpdate {}
unsafe impl ::core::marker::Sync for StorePackageUpdate {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageUpdateResult(::windows::core::IUnknown);
impl StorePackageUpdateResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn OverallState(&self) -> ::windows::core::Result<StorePackageUpdateState> {
        let this = self;
        unsafe {
            let mut result__: StorePackageUpdateState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverallState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePackageUpdateState>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorePackageUpdateStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StorePackageUpdateStatuses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StorePackageUpdateStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StoreQueueItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreQueueItem>> {
        let this = &::windows::core::Interface::cast::<IStorePackageUpdateResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreQueueItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePackageUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageUpdateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageUpdateResult {}
impl ::core::fmt::Debug for StorePackageUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageUpdateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageUpdateResult;{e79142ed-61f9-4893-b4fe-cf191603af7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePackageUpdateResult {
    type Vtable = IStorePackageUpdateResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorePackageUpdateResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePackageUpdateResult {
    const NAME: &'static str = "Windows.Services.Store.StorePackageUpdateResult";
}
impl ::core::convert::From<StorePackageUpdateResult> for ::windows::core::IUnknown {
    fn from(value: StorePackageUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdateResult> for ::windows::core::IUnknown {
    fn from(value: &StorePackageUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePackageUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePackageUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageUpdateResult> for ::windows::core::IInspectable {
    fn from(value: StorePackageUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdateResult> for ::windows::core::IInspectable {
    fn from(value: &StorePackageUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePackageUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePackageUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePackageUpdateResult {}
unsafe impl ::core::marker::Sync for StorePackageUpdateResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorePackageUpdateState(pub i32);
impl StorePackageUpdateState {
    pub const Pending: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Deploying: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const OtherError: Self = Self(5i32);
    pub const ErrorLowBattery: Self = Self(6i32);
    pub const ErrorWiFiRecommended: Self = Self(7i32);
    pub const ErrorWiFiRequired: Self = Self(8i32);
}
impl ::core::marker::Copy for StorePackageUpdateState {}
impl ::core::clone::Clone for StorePackageUpdateState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorePackageUpdateState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorePackageUpdateState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorePackageUpdateState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageUpdateState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdateState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StorePackageUpdateState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Services_Store\"`*"]
pub struct StorePackageUpdateStatus {
    pub PackageFamilyName: ::windows::core::HSTRING,
    pub PackageDownloadSizeInBytes: u64,
    pub PackageBytesDownloaded: u64,
    pub PackageDownloadProgress: f64,
    pub TotalDownloadProgress: f64,
    pub PackageUpdateState: StorePackageUpdateState,
}
impl ::core::clone::Clone for StorePackageUpdateStatus {
    fn clone(&self) -> Self {
        Self {
            PackageFamilyName: self.PackageFamilyName.clone(),
            PackageDownloadSizeInBytes: self.PackageDownloadSizeInBytes,
            PackageBytesDownloaded: self.PackageBytesDownloaded,
            PackageDownloadProgress: self.PackageDownloadProgress,
            TotalDownloadProgress: self.TotalDownloadProgress,
            PackageUpdateState: self.PackageUpdateState,
        }
    }
}
impl ::core::fmt::Debug for StorePackageUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorePackageUpdateStatus").field("PackageFamilyName", &self.PackageFamilyName).field("PackageDownloadSizeInBytes", &self.PackageDownloadSizeInBytes).field("PackageBytesDownloaded", &self.PackageBytesDownloaded).field("PackageDownloadProgress", &self.PackageDownloadProgress).field("TotalDownloadProgress", &self.TotalDownloadProgress).field("PackageUpdateState", &self.PackageUpdateState).finish()
    }
}
unsafe impl ::windows::core::Abi for StorePackageUpdateStatus {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for StorePackageUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Services.Store.StorePackageUpdateStatus;string;u8;u8;f8;f8;enum(Windows.Services.Store.StorePackageUpdateState;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageUpdateStatus {
    fn eq(&self, other: &Self) -> bool {
        self.PackageFamilyName == other.PackageFamilyName && self.PackageDownloadSizeInBytes == other.PackageDownloadSizeInBytes && self.PackageBytesDownloaded == other.PackageBytesDownloaded && self.PackageDownloadProgress == other.PackageDownloadProgress && self.TotalDownloadProgress == other.TotalDownloadProgress && self.PackageUpdateState == other.PackageUpdateState
    }
}
impl ::core::cmp::Eq for StorePackageUpdateStatus {}
impl ::core::default::Default for StorePackageUpdateStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePrice(::windows::core::IUnknown);
impl StorePrice {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn FormattedBasePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedBasePrice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn FormattedPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedPrice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsOnSale(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOnSale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaleEndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaleEndDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrencyCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn FormattedRecurrencePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedRecurrencePrice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePrice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePrice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePrice {}
impl ::core::fmt::Debug for StorePrice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePrice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePrice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePrice;{55ba94c4-15f1-407c-8f06-006380f4df0b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePrice {
    type Vtable = IStorePrice_Vtbl;
    const IID: ::windows::core::GUID = <IStorePrice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePrice {
    const NAME: &'static str = "Windows.Services.Store.StorePrice";
}
impl ::core::convert::From<StorePrice> for ::windows::core::IUnknown {
    fn from(value: StorePrice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePrice> for ::windows::core::IUnknown {
    fn from(value: &StorePrice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePrice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePrice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePrice> for ::windows::core::IInspectable {
    fn from(value: StorePrice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePrice> for ::windows::core::IInspectable {
    fn from(value: &StorePrice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePrice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePrice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePrice {}
unsafe impl ::core::marker::Sync for StorePrice {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProduct(::windows::core::IUnknown);
impl StoreProduct {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ProductKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn HasDigitalDownload(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasDigitalDownload)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keywords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Images)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreImage>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Videos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreVideo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Skus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreSku>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Skus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreSku>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsInUserCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInUserCollection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Price(&self) -> ::windows::core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Price)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePrice>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LinkUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinkUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAnySkuInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsAnySkuInstalledAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, StorePurchaseProperties>>(&self, storepurchaseproperties: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::core::mem::transmute_copy(this), storepurchaseproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InAppOfferToken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProduct {}
impl ::core::fmt::Debug for StoreProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProduct").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProduct {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProduct;{320e2c52-d760-450a-a42b-67d1e901ac90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreProduct {
    type Vtable = IStoreProduct_Vtbl;
    const IID: ::windows::core::GUID = <IStoreProduct as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProduct {
    const NAME: &'static str = "Windows.Services.Store.StoreProduct";
}
impl ::core::convert::From<StoreProduct> for ::windows::core::IUnknown {
    fn from(value: StoreProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProduct> for ::windows::core::IUnknown {
    fn from(value: &StoreProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProduct> for ::windows::core::IInspectable {
    fn from(value: StoreProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProduct> for ::windows::core::IInspectable {
    fn from(value: &StoreProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProduct {}
unsafe impl ::core::marker::Sync for StoreProduct {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductOptions(::windows::core::IUnknown);
impl StoreProductOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreProductOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActionFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActionFilters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductOptions {}
impl ::core::fmt::Debug for StoreProductOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductOptions;{5b34a0f9-a113-4811-8326-16199c927f31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreProductOptions {
    type Vtable = IStoreProductOptions_Vtbl;
    const IID: ::windows::core::GUID = <IStoreProductOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductOptions {
    const NAME: &'static str = "Windows.Services.Store.StoreProductOptions";
}
impl ::core::convert::From<StoreProductOptions> for ::windows::core::IUnknown {
    fn from(value: StoreProductOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductOptions> for ::windows::core::IUnknown {
    fn from(value: &StoreProductOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreProductOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreProductOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductOptions> for ::windows::core::IInspectable {
    fn from(value: StoreProductOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductOptions> for ::windows::core::IInspectable {
    fn from(value: &StoreProductOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreProductOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreProductOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductOptions {}
unsafe impl ::core::marker::Sync for StoreProductOptions {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductPagedQueryResult(::windows::core::IUnknown);
impl StoreProductPagedQueryResult {
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Products)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn HasMoreResults(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasMoreResults)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNextAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductPagedQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductPagedQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductPagedQueryResult {}
impl ::core::fmt::Debug for StoreProductPagedQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductPagedQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductPagedQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductPagedQueryResult;{c92718c5-4dd5-4869-a462-ecc6872e43c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreProductPagedQueryResult {
    type Vtable = IStoreProductPagedQueryResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreProductPagedQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductPagedQueryResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductPagedQueryResult";
}
impl ::core::convert::From<StoreProductPagedQueryResult> for ::windows::core::IUnknown {
    fn from(value: StoreProductPagedQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductPagedQueryResult> for ::windows::core::IUnknown {
    fn from(value: &StoreProductPagedQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductPagedQueryResult> for ::windows::core::IInspectable {
    fn from(value: StoreProductPagedQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductPagedQueryResult> for ::windows::core::IInspectable {
    fn from(value: &StoreProductPagedQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductPagedQueryResult {}
unsafe impl ::core::marker::Sync for StoreProductPagedQueryResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductQueryResult(::windows::core::IUnknown);
impl StoreProductQueryResult {
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Products)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductQueryResult {}
impl ::core::fmt::Debug for StoreProductQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductQueryResult;{d805e6c5-d456-4ff6-8049-9076d5165f73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreProductQueryResult {
    type Vtable = IStoreProductQueryResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreProductQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductQueryResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductQueryResult";
}
impl ::core::convert::From<StoreProductQueryResult> for ::windows::core::IUnknown {
    fn from(value: StoreProductQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductQueryResult> for ::windows::core::IUnknown {
    fn from(value: &StoreProductQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreProductQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreProductQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductQueryResult> for ::windows::core::IInspectable {
    fn from(value: StoreProductQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductQueryResult> for ::windows::core::IInspectable {
    fn from(value: &StoreProductQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreProductQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreProductQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductQueryResult {}
unsafe impl ::core::marker::Sync for StoreProductQueryResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreProductResult(::windows::core::IUnknown);
impl StoreProductResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Product(&self) -> ::windows::core::Result<StoreProduct> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Product)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreProduct>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductResult {}
impl ::core::fmt::Debug for StoreProductResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreProductResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductResult;{b7674f73-3c87-4ee1-8201-f428359bd3af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreProductResult {
    type Vtable = IStoreProductResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreProductResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreProductResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductResult";
}
impl ::core::convert::From<StoreProductResult> for ::windows::core::IUnknown {
    fn from(value: StoreProductResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductResult> for ::windows::core::IUnknown {
    fn from(value: &StoreProductResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreProductResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreProductResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductResult> for ::windows::core::IInspectable {
    fn from(value: StoreProductResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductResult> for ::windows::core::IInspectable {
    fn from(value: &StoreProductResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreProductResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreProductResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductResult {}
unsafe impl ::core::marker::Sync for StoreProductResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePurchaseProperties(::windows::core::IUnknown);
impl StorePurchaseProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorePurchaseProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn SetExtendedJsonData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExtendedJsonData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<StorePurchaseProperties> {
        Self::IStorePurchasePropertiesFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<StorePurchaseProperties>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorePurchasePropertiesFactory<R, F: FnOnce(&IStorePurchasePropertiesFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorePurchaseProperties, IStorePurchasePropertiesFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorePurchaseProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePurchaseProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePurchaseProperties {}
impl ::core::fmt::Debug for StorePurchaseProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePurchaseProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePurchaseProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePurchaseProperties;{836278f3-ff87-4364-a5b4-fd2153ebe43b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePurchaseProperties {
    type Vtable = IStorePurchaseProperties_Vtbl;
    const IID: ::windows::core::GUID = <IStorePurchaseProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePurchaseProperties {
    const NAME: &'static str = "Windows.Services.Store.StorePurchaseProperties";
}
impl ::core::convert::From<StorePurchaseProperties> for ::windows::core::IUnknown {
    fn from(value: StorePurchaseProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseProperties> for ::windows::core::IUnknown {
    fn from(value: &StorePurchaseProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePurchaseProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePurchaseProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePurchaseProperties> for ::windows::core::IInspectable {
    fn from(value: StorePurchaseProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseProperties> for ::windows::core::IInspectable {
    fn from(value: &StorePurchaseProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePurchaseProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePurchaseProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePurchaseProperties {}
unsafe impl ::core::marker::Sync for StorePurchaseProperties {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePurchaseResult(::windows::core::IUnknown);
impl StorePurchaseResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StorePurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__: StorePurchaseStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePurchaseStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePurchaseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePurchaseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePurchaseResult {}
impl ::core::fmt::Debug for StorePurchaseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePurchaseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePurchaseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePurchaseResult;{add28552-f96a-463d-a7bb-c20b4fca6952})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePurchaseResult {
    type Vtable = IStorePurchaseResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorePurchaseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePurchaseResult {
    const NAME: &'static str = "Windows.Services.Store.StorePurchaseResult";
}
impl ::core::convert::From<StorePurchaseResult> for ::windows::core::IUnknown {
    fn from(value: StorePurchaseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseResult> for ::windows::core::IUnknown {
    fn from(value: &StorePurchaseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePurchaseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePurchaseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePurchaseResult> for ::windows::core::IInspectable {
    fn from(value: StorePurchaseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseResult> for ::windows::core::IInspectable {
    fn from(value: &StorePurchaseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePurchaseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePurchaseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePurchaseResult {}
unsafe impl ::core::marker::Sync for StorePurchaseResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorePurchaseStatus(pub i32);
impl StorePurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotPurchased: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for StorePurchaseStatus {}
impl ::core::clone::Clone for StorePurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorePurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorePurchaseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorePurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePurchaseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePurchaseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StorePurchaseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItem(::windows::core::IUnknown);
impl StoreQueueItem {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn InstallKind(&self) -> ::windows::core::Result<StoreQueueItemKind> {
        let this = self;
        unsafe {
            let mut result__: StoreQueueItemKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreQueueItemKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn GetCurrentStatus(&self) -> ::windows::core::Result<StoreQueueItemStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreQueueItemStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Completed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StoreQueueItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CancelInstallAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PauseInstallAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeInstallAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreQueueItem {}
impl ::core::fmt::Debug for StoreQueueItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItem;{56d5c32b-f830-4293-9188-cad2dcde7357})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreQueueItem {
    type Vtable = IStoreQueueItem_Vtbl;
    const IID: ::windows::core::GUID = <IStoreQueueItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreQueueItem {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItem";
}
impl ::core::convert::From<StoreQueueItem> for ::windows::core::IUnknown {
    fn from(value: StoreQueueItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItem> for ::windows::core::IUnknown {
    fn from(value: &StoreQueueItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreQueueItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreQueueItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreQueueItem> for ::windows::core::IInspectable {
    fn from(value: StoreQueueItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItem> for ::windows::core::IInspectable {
    fn from(value: &StoreQueueItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreQueueItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreQueueItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreQueueItem {}
unsafe impl ::core::marker::Sync for StoreQueueItem {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemCompletedEventArgs(::windows::core::IUnknown);
impl StoreQueueItemCompletedEventArgs {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StoreQueueItemStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreQueueItemStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItemCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreQueueItemCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreQueueItemCompletedEventArgs {}
impl ::core::fmt::Debug for StoreQueueItemCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItemCompletedEventArgs;{1247df6c-b44a-439b-bb07-1d3003d005c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreQueueItemCompletedEventArgs {
    type Vtable = IStoreQueueItemCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IStoreQueueItemCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreQueueItemCompletedEventArgs {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItemCompletedEventArgs";
}
impl ::core::convert::From<StoreQueueItemCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: StoreQueueItemCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &StoreQueueItemCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreQueueItemCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: StoreQueueItemCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &StoreQueueItemCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreQueueItemCompletedEventArgs {}
unsafe impl ::core::marker::Sync for StoreQueueItemCompletedEventArgs {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreQueueItemExtendedState(pub i32);
impl StoreQueueItemExtendedState {
    pub const ActivePending: Self = Self(0i32);
    pub const ActiveStarting: Self = Self(1i32);
    pub const ActiveAcquiringLicense: Self = Self(2i32);
    pub const ActiveDownloading: Self = Self(3i32);
    pub const ActiveRestoringData: Self = Self(4i32);
    pub const ActiveInstalling: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedPackagesInUse: Self = Self(10i32);
    pub const PausedLowBattery: Self = Self(11i32);
    pub const PausedWiFiRecommended: Self = Self(12i32);
    pub const PausedWiFiRequired: Self = Self(13i32);
    pub const PausedReadyToInstall: Self = Self(14i32);
}
impl ::core::marker::Copy for StoreQueueItemExtendedState {}
impl ::core::clone::Clone for StoreQueueItemExtendedState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreQueueItemExtendedState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreQueueItemExtendedState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreQueueItemExtendedState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemExtendedState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemExtendedState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemExtendedState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreQueueItemKind(pub i32);
impl StoreQueueItemKind {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for StoreQueueItemKind {}
impl ::core::clone::Clone for StoreQueueItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreQueueItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreQueueItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreQueueItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreQueueItemState(pub i32);
impl StoreQueueItemState {
    pub const Active: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreQueueItemState {}
impl ::core::clone::Clone for StoreQueueItemState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreQueueItemState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreQueueItemState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreQueueItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemStatus(::windows::core::IUnknown);
impl StoreQueueItemStatus {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn PackageInstallState(&self) -> ::windows::core::Result<StoreQueueItemState> {
        let this = self;
        unsafe {
            let mut result__: StoreQueueItemState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageInstallState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreQueueItemState>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn PackageInstallExtendedState(&self) -> ::windows::core::Result<StoreQueueItemExtendedState> {
        let this = self;
        unsafe {
            let mut result__: StoreQueueItemExtendedState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageInstallExtendedState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreQueueItemExtendedState>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn UpdateStatus(&self) -> ::windows::core::Result<StorePackageUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<StorePackageUpdateStatus> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePackageUpdateStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItemStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreQueueItemStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreQueueItemStatus {}
impl ::core::fmt::Debug for StoreQueueItemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreQueueItemStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItemStatus;{9bd6796f-9cc3-4ec3-b2ef-7be433b30174})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreQueueItemStatus {
    type Vtable = IStoreQueueItemStatus_Vtbl;
    const IID: ::windows::core::GUID = <IStoreQueueItemStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreQueueItemStatus {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItemStatus";
}
impl ::core::convert::From<StoreQueueItemStatus> for ::windows::core::IUnknown {
    fn from(value: StoreQueueItemStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemStatus> for ::windows::core::IUnknown {
    fn from(value: &StoreQueueItemStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreQueueItemStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreQueueItemStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreQueueItemStatus> for ::windows::core::IInspectable {
    fn from(value: StoreQueueItemStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemStatus> for ::windows::core::IInspectable {
    fn from(value: &StoreQueueItemStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreQueueItemStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreQueueItemStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreQueueItemStatus {}
unsafe impl ::core::marker::Sync for StoreQueueItemStatus {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreRateAndReviewResult(::windows::core::IUnknown);
impl StoreRateAndReviewResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn WasUpdated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasUpdated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StoreRateAndReviewStatus> {
        let this = self;
        unsafe {
            let mut result__: StoreRateAndReviewStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreRateAndReviewStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreRateAndReviewResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreRateAndReviewResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreRateAndReviewResult {}
impl ::core::fmt::Debug for StoreRateAndReviewResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreRateAndReviewResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreRateAndReviewResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreRateAndReviewResult;{9d209d56-a6b5-4121-9b61-ee6d0fbdbdbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreRateAndReviewResult {
    type Vtable = IStoreRateAndReviewResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreRateAndReviewResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreRateAndReviewResult {
    const NAME: &'static str = "Windows.Services.Store.StoreRateAndReviewResult";
}
impl ::core::convert::From<StoreRateAndReviewResult> for ::windows::core::IUnknown {
    fn from(value: StoreRateAndReviewResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreRateAndReviewResult> for ::windows::core::IUnknown {
    fn from(value: &StoreRateAndReviewResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreRateAndReviewResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreRateAndReviewResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreRateAndReviewResult> for ::windows::core::IInspectable {
    fn from(value: StoreRateAndReviewResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreRateAndReviewResult> for ::windows::core::IInspectable {
    fn from(value: &StoreRateAndReviewResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreRateAndReviewResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreRateAndReviewResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreRateAndReviewResult {}
unsafe impl ::core::marker::Sync for StoreRateAndReviewResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreRateAndReviewStatus(pub i32);
impl StoreRateAndReviewStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for StoreRateAndReviewStatus {}
impl ::core::clone::Clone for StoreRateAndReviewStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreRateAndReviewStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreRateAndReviewStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreRateAndReviewStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreRateAndReviewStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreRateAndReviewStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreRateAndReviewStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
pub struct StoreRequestHelper {}
impl StoreRequestHelper {
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, StoreContext>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(context: Param0, requestkind: u32, parametersasjson: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreSendRequestResult>> {
        Self::IStoreRequestHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendRequestAsync)(::core::mem::transmute_copy(this), context.into_param().abi(), requestkind, parametersasjson.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StoreSendRequestResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStoreRequestHelperStatics<R, F: FnOnce(&IStoreRequestHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreRequestHelper, IStoreRequestHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StoreRequestHelper {
    const NAME: &'static str = "Windows.Services.Store.StoreRequestHelper";
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreSendRequestResult(::windows::core::IUnknown);
impl StoreSendRequestResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Response)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn HttpStatusCode(&self) -> ::windows::core::Result<super::super::Web::Http::HttpStatusCode> {
        let this = &::windows::core::Interface::cast::<IStoreSendRequestResult2>(self)?;
        unsafe {
            let mut result__: super::super::Web::Http::HttpStatusCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HttpStatusCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Web::Http::HttpStatusCode>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSendRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreSendRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreSendRequestResult {}
impl ::core::fmt::Debug for StoreSendRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSendRequestResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSendRequestResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSendRequestResult;{c73abe60-8272-4502-8a69-6e75153a4299})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreSendRequestResult {
    type Vtable = IStoreSendRequestResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreSendRequestResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreSendRequestResult {
    const NAME: &'static str = "Windows.Services.Store.StoreSendRequestResult";
}
impl ::core::convert::From<StoreSendRequestResult> for ::windows::core::IUnknown {
    fn from(value: StoreSendRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSendRequestResult> for ::windows::core::IUnknown {
    fn from(value: &StoreSendRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreSendRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreSendRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreSendRequestResult> for ::windows::core::IInspectable {
    fn from(value: StoreSendRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSendRequestResult> for ::windows::core::IInspectable {
    fn from(value: &StoreSendRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreSendRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreSendRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreSendRequestResult {}
unsafe impl ::core::marker::Sync for StoreSendRequestResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreSku(::windows::core::IUnknown);
impl StoreSku {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTrial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomDeveloperData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Images)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreImage>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Videos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreVideo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Availabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreAvailability>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Availabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StoreAvailability>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Price(&self) -> ::windows::core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Price)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePrice>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedJsonData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsInUserCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInUserCollection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BundledSkus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BundledSkus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn CollectionData(&self) -> ::windows::core::Result<StoreCollectionData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollectionData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreCollectionData>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIsInstalledAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, StorePurchaseProperties>>(&self, storepurchaseproperties: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::core::mem::transmute_copy(this), storepurchaseproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn IsSubscription(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSubscription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn SubscriptionInfo(&self) -> ::windows::core::Result<StoreSubscriptionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SubscriptionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreSubscriptionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSku {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreSku {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreSku {}
impl ::core::fmt::Debug for StoreSku {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSku").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSku {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSku;{397e6f55-4440-4f03-863c-91f3fec83d79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreSku {
    type Vtable = IStoreSku_Vtbl;
    const IID: ::windows::core::GUID = <IStoreSku as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreSku {
    const NAME: &'static str = "Windows.Services.Store.StoreSku";
}
impl ::core::convert::From<StoreSku> for ::windows::core::IUnknown {
    fn from(value: StoreSku) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSku> for ::windows::core::IUnknown {
    fn from(value: &StoreSku) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreSku {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreSku {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreSku> for ::windows::core::IInspectable {
    fn from(value: StoreSku) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSku> for ::windows::core::IInspectable {
    fn from(value: &StoreSku) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreSku {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreSku {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreSku {}
unsafe impl ::core::marker::Sync for StoreSku {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreSubscriptionInfo(::windows::core::IUnknown);
impl StoreSubscriptionInfo {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn BillingPeriod(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BillingPeriod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn BillingPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit> {
        let this = self;
        unsafe {
            let mut result__: StoreDurationUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BillingPeriodUnit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreDurationUnit>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn HasTrialPeriod(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasTrialPeriod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn TrialPeriod(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrialPeriod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn TrialPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit> {
        let this = self;
        unsafe {
            let mut result__: StoreDurationUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrialPeriodUnit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreDurationUnit>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSubscriptionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreSubscriptionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreSubscriptionInfo {}
impl ::core::fmt::Debug for StoreSubscriptionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSubscriptionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSubscriptionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSubscriptionInfo;{4189776a-0559-43ac-a9c6-3ab0011fb8eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreSubscriptionInfo {
    type Vtable = IStoreSubscriptionInfo_Vtbl;
    const IID: ::windows::core::GUID = <IStoreSubscriptionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreSubscriptionInfo {
    const NAME: &'static str = "Windows.Services.Store.StoreSubscriptionInfo";
}
impl ::core::convert::From<StoreSubscriptionInfo> for ::windows::core::IUnknown {
    fn from(value: StoreSubscriptionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSubscriptionInfo> for ::windows::core::IUnknown {
    fn from(value: &StoreSubscriptionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreSubscriptionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreSubscriptionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreSubscriptionInfo> for ::windows::core::IInspectable {
    fn from(value: StoreSubscriptionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSubscriptionInfo> for ::windows::core::IInspectable {
    fn from(value: &StoreSubscriptionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreSubscriptionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreSubscriptionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreSubscriptionInfo {}
unsafe impl ::core::marker::Sync for StoreSubscriptionInfo {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreUninstallStorePackageResult(::windows::core::IUnknown);
impl StoreUninstallStorePackageResult {
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StoreUninstallStorePackageStatus> {
        let this = self;
        unsafe {
            let mut result__: StoreUninstallStorePackageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreUninstallStorePackageStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreUninstallStorePackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreUninstallStorePackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreUninstallStorePackageResult {}
impl ::core::fmt::Debug for StoreUninstallStorePackageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreUninstallStorePackageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreUninstallStorePackageResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreUninstallStorePackageResult;{9fca39fd-126f-4cda-b801-1346b8d0a260})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreUninstallStorePackageResult {
    type Vtable = IStoreUninstallStorePackageResult_Vtbl;
    const IID: ::windows::core::GUID = <IStoreUninstallStorePackageResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreUninstallStorePackageResult {
    const NAME: &'static str = "Windows.Services.Store.StoreUninstallStorePackageResult";
}
impl ::core::convert::From<StoreUninstallStorePackageResult> for ::windows::core::IUnknown {
    fn from(value: StoreUninstallStorePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreUninstallStorePackageResult> for ::windows::core::IUnknown {
    fn from(value: &StoreUninstallStorePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreUninstallStorePackageResult> for ::windows::core::IInspectable {
    fn from(value: StoreUninstallStorePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreUninstallStorePackageResult> for ::windows::core::IInspectable {
    fn from(value: &StoreUninstallStorePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreUninstallStorePackageResult {}
unsafe impl ::core::marker::Sync for StoreUninstallStorePackageResult {}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreUninstallStorePackageStatus(pub i32);
impl StoreUninstallStorePackageStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UninstallNotApplicable: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreUninstallStorePackageStatus {}
impl ::core::clone::Clone for StoreUninstallStorePackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreUninstallStorePackageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreUninstallStorePackageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreUninstallStorePackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreUninstallStorePackageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreUninstallStorePackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreUninstallStorePackageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreVideo(::windows::core::IUnknown);
impl StoreVideo {
    #[doc = "*Required features: `\"Services_Store\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn VideoPurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoPurposeTag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Caption)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Store\"`*"]
    pub fn PreviewImage(&self) -> ::windows::core::Result<StoreImage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewImage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreImage>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreVideo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreVideo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreVideo {}
impl ::core::fmt::Debug for StoreVideo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreVideo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreVideo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreVideo;{f26cb184-6f5e-4dc2-886c-3c63083c2f94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreVideo {
    type Vtable = IStoreVideo_Vtbl;
    const IID: ::windows::core::GUID = <IStoreVideo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreVideo {
    const NAME: &'static str = "Windows.Services.Store.StoreVideo";
}
impl ::core::convert::From<StoreVideo> for ::windows::core::IUnknown {
    fn from(value: StoreVideo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreVideo> for ::windows::core::IUnknown {
    fn from(value: &StoreVideo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreVideo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreVideo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreVideo> for ::windows::core::IInspectable {
    fn from(value: StoreVideo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreVideo> for ::windows::core::IInspectable {
    fn from(value: &StoreVideo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreVideo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreVideo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreVideo {}
unsafe impl ::core::marker::Sync for StoreVideo {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
