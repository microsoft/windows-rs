#[cfg(feature = "ApplicationModel_Store_LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "ApplicationModel_Store_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentApp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentApp {
    type Vtable = ICurrentApp_Vtbl;
}
impl ::core::clone::Clone for ICurrentApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentApp {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd52dc065_da3f_4685_995e_9b482eb5e603);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentApp_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LicenseInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAppPurchaseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestProductPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadListingInformationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadListingInformationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppReceiptAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetProductReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProductReceiptAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentApp2Statics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentApp2Statics {
    type Vtable = ICurrentApp2Statics_Vtbl;
}
impl ::core::clone::Clone for ICurrentApp2Statics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentApp2Statics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf4e6e2d_3171_4ad3_8614_2c61244373cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentApp2Statics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::std::mem::MaybeUninit<::windows_core::HSTRING>, publisheruserid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerPurchaseIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::std::mem::MaybeUninit<::windows_core::HSTRING>, publisheruserid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerCollectionsIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulator {
    type Vtable = ICurrentAppSimulator_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppSimulator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppSimulator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf17f9db1_74cd_4787_9787_19866e9a5559);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LicenseInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAppPurchaseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestProductPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadListingInformationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadListingInformationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppReceiptAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetProductReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProductReceiptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ReloadSimulatorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simulatorsettingsfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ReloadSimulatorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulatorStaticsWithFiltering(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulatorStaticsWithFiltering {
    type Vtable = ICurrentAppSimulatorStaticsWithFiltering_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppSimulatorStaticsWithFiltering {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppSimulatorStaticsWithFiltering {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x617e70e2_f86f_4b54_9666_dde285092c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulatorStaticsWithFiltering_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByKeywordsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithCampaignId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulatorWithCampaignId {
    type Vtable = ICurrentAppSimulatorWithCampaignId_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppSimulatorWithCampaignId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppSimulatorWithCampaignId {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84678a43_df00_4672_a43f_b25b1441cfcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulatorWithCampaignId_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppPurchaseCampaignIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithConsumables(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulatorWithConsumables {
    type Vtable = ICurrentAppSimulatorWithConsumables_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppSimulatorWithConsumables {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppSimulatorWithConsumables {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e51f0ab_20e7_4412_9b85_59bb78388667);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulatorWithConsumables_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, transactionid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithResultsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, offerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithDisplayPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnfulfilledConsumablesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppStaticsWithFiltering(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppStaticsWithFiltering {
    type Vtable = ICurrentAppStaticsWithFiltering_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppStaticsWithFiltering {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppStaticsWithFiltering {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd36d6542_9085_438e_97ba_a25c976be2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppStaticsWithFiltering_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByKeywordsAsync: usize,
    pub ReportProductFulfillment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppWithCampaignId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppWithCampaignId {
    type Vtable = ICurrentAppWithCampaignId_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppWithCampaignId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppWithCampaignId {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x312f4cd0_36c1_44a6_b32b_432d608e4dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppWithCampaignId_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppPurchaseCampaignIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppWithConsumables(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppWithConsumables {
    type Vtable = ICurrentAppWithConsumables_Vtbl;
}
impl ::core::clone::Clone for ICurrentAppWithConsumables {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrentAppWithConsumables {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x844e0071_9e4f_4f79_995a_5f91172e6cef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppWithConsumables_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, transactionid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithResultsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, offerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithDisplayPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnfulfilledConsumablesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseInformation {
    type Vtable = ILicenseInformation_Vtbl;
}
impl ::core::clone::Clone for ILicenseInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILicenseInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8eb7dc30_f170_4ed5_8e21_1516da3fd367);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProductLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProductLicenses: usize,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub LicenseChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLicenseChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLicenseChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListingInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListingInformation {
    type Vtable = IListingInformation_Vtbl;
}
impl ::core::clone::Clone for IListingInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IListingInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x588b4abf_bc74_4383_b78c_99606323dece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListingInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentMarket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProductListings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProductListings: usize,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListingInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListingInformation2 {
    type Vtable = IListingInformation2_Vtbl;
}
impl ::core::clone::Clone for IListingInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IListingInformation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0fd2c1d_b30e_4384_84ea_72fefa82223e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListingInformation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductLicense(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductLicense {
    type Vtable = IProductLicense_Vtbl;
}
impl ::core::clone::Clone for IProductLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductLicense {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x363308c7_2bcf_4c0e_8f2f_e808aaa8f99d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductLicense_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductLicenseWithFulfillment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductLicenseWithFulfillment {
    type Vtable = IProductLicenseWithFulfillment_Vtbl;
}
impl ::core::clone::Clone for IProductLicenseWithFulfillment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductLicenseWithFulfillment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc535c8a_f667_40f3_ba3c_045a63abb3ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductLicenseWithFulfillment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsConsumable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListing {
    type Vtable = IProductListing_Vtbl;
}
impl ::core::clone::Clone for IProductListing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductListing {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45a7d6ad_c750_4d9c_947c_b00dcbf9e9c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListing_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListing2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListing2 {
    type Vtable = IProductListing2_Vtbl;
}
impl ::core::clone::Clone for IProductListing2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductListing2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf89e290f_73fe_494d_a939_08a9b2495abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListing2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListingWithConsumables(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListingWithConsumables {
    type Vtable = IProductListingWithConsumables_Vtbl;
}
impl ::core::clone::Clone for IProductListingWithConsumables {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductListingWithConsumables {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb9e9790_8f6b_481f_93a7_5c3a63068149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListingWithConsumables_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProductType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListingWithMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListingWithMetadata {
    type Vtable = IProductListingWithMetadata_Vtbl;
}
impl ::core::clone::Clone for IProductListingWithMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductListingWithMetadata {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x124da567_23f8_423e_9532_189943c40ace);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListingWithMetadata_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProductType) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductPurchaseDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductPurchaseDisplayProperties {
    type Vtable = IProductPurchaseDisplayProperties_Vtbl;
}
impl ::core::clone::Clone for IProductPurchaseDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductPurchaseDisplayProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd70b7420_bc92_401b_a809_c9b2e5dbbdaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductPurchaseDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Image: usize,
    #[cfg(feature = "Foundation")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetImage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductPurchaseDisplayPropertiesFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductPurchaseDisplayPropertiesFactory {
    type Vtable = IProductPurchaseDisplayPropertiesFactory_Vtbl;
}
impl ::core::clone::Clone for IProductPurchaseDisplayPropertiesFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProductPurchaseDisplayPropertiesFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f491df4_32d6_4b40_b474_b83038a4d9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductPurchaseDisplayPropertiesFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateProductPurchaseDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPurchaseResults(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPurchaseResults {
    type Vtable = IPurchaseResults_Vtbl;
}
impl ::core::clone::Clone for IPurchaseResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPurchaseResults {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed50b37e_8656_4f65_b8c8_ac7e0cb1a1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPurchaseResults_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProductPurchaseStatus) -> ::windows_core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ReceiptXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnfulfilledConsumable(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnfulfilledConsumable {
    type Vtable = IUnfulfilledConsumable_Vtbl;
}
impl ::core::clone::Clone for IUnfulfilledConsumable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUnfulfilledConsumable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2df7fbbb_1cdd_4cb8_a014_7b9cf8986927);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnfulfilledConsumable_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
pub struct CurrentApp;
impl CurrentApp {
    pub fn LicenseInformation() -> ::windows_core::Result<LicenseInformation> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LinkUri() -> ::windows_core::Result<super::super::Foundation::Uri> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinkUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AppId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAppPurchaseAsync(includereceipt: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAppPurchaseAsync)(::windows_core::Interface::as_raw(this), includereceipt, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestProductPurchaseAsync(productid: &::windows_core::HSTRING, includereceipt: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), includereceipt, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadListingInformationAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppReceiptAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppReceiptAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetProductReceiptAsync(productid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProductReceiptAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomerPurchaseIdAsync(serviceticket: &::windows_core::HSTRING, publisheruserid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp2Statics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomerPurchaseIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(serviceticket), ::core::mem::transmute_copy(publisheruserid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomerCollectionsIdAsync(serviceticket: &::windows_core::HSTRING, publisheruserid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp2Statics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomerCollectionsIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(serviceticket), ::core::mem::transmute_copy(publisheruserid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByProductIdsAsync<P0>(productids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByProductIdsAsync)(::windows_core::Interface::as_raw(this), productids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByKeywordsAsync<P0>(keywords: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByKeywordsAsync)(::windows_core::Interface::as_raw(this), keywords.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn ReportProductFulfillment(productid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe { (::windows_core::Interface::vtable(this).ReportProductFulfillment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppPurchaseCampaignIdAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppWithCampaignId(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppPurchaseCampaignIdAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportConsumableFulfillmentAsync(productid: &::windows_core::HSTRING, transactionid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), transactionid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseWithResultsAsync(productid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithResultsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseWithDisplayPropertiesAsync<P0>(productid: &::windows_core::HSTRING, offerid: &::windows_core::HSTRING, displayproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>
    where
        P0: ::windows_core::IntoParam<ProductPurchaseDisplayProperties>,
    {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithDisplayPropertiesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(offerid), displayproperties.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnfulfilledConsumablesAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUnfulfilledConsumablesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrentApp<R, F: FnOnce(&ICurrentApp) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentApp, ICurrentApp> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentApp2Statics<R, F: FnOnce(&ICurrentApp2Statics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentApp, ICurrentApp2Statics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppStaticsWithFiltering<R, F: FnOnce(&ICurrentAppStaticsWithFiltering) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentApp, ICurrentAppStaticsWithFiltering> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppWithCampaignId<R, F: FnOnce(&ICurrentAppWithCampaignId) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentApp, ICurrentAppWithCampaignId> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppWithConsumables<R, F: FnOnce(&ICurrentAppWithConsumables) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentApp, ICurrentAppWithConsumables> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CurrentApp {
    const NAME: &'static str = "Windows.ApplicationModel.Store.CurrentApp";
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
pub struct CurrentAppSimulator;
impl CurrentAppSimulator {
    pub fn LicenseInformation() -> ::windows_core::Result<LicenseInformation> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LinkUri() -> ::windows_core::Result<super::super::Foundation::Uri> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinkUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AppId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAppPurchaseAsync(includereceipt: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAppPurchaseAsync)(::windows_core::Interface::as_raw(this), includereceipt, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestProductPurchaseAsync(productid: &::windows_core::HSTRING, includereceipt: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), includereceipt, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadListingInformationAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppReceiptAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppReceiptAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetProductReceiptAsync(productid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProductReceiptAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ReloadSimulatorAsync<P0>(simulatorsettingsfile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReloadSimulatorAsync)(::windows_core::Interface::as_raw(this), simulatorsettingsfile.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByProductIdsAsync<P0>(productids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICurrentAppSimulatorStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByProductIdsAsync)(::windows_core::Interface::as_raw(this), productids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByKeywordsAsync<P0>(keywords: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICurrentAppSimulatorStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByKeywordsAsync)(::windows_core::Interface::as_raw(this), keywords.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppPurchaseCampaignIdAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulatorWithCampaignId(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppPurchaseCampaignIdAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportConsumableFulfillmentAsync(productid: &::windows_core::HSTRING, transactionid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), transactionid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseWithResultsAsync(productid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithResultsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseWithDisplayPropertiesAsync<P0>(productid: &::windows_core::HSTRING, offerid: &::windows_core::HSTRING, displayproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>
    where
        P0: ::windows_core::IntoParam<ProductPurchaseDisplayProperties>,
    {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithDisplayPropertiesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(offerid), displayproperties.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnfulfilledConsumablesAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUnfulfilledConsumablesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulator<R, F: FnOnce(&ICurrentAppSimulator) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulator> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulatorStaticsWithFiltering<R, F: FnOnce(&ICurrentAppSimulatorStaticsWithFiltering) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorStaticsWithFiltering> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulatorWithCampaignId<R, F: FnOnce(&ICurrentAppSimulatorWithCampaignId) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorWithCampaignId> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulatorWithConsumables<R, F: FnOnce(&ICurrentAppSimulatorWithConsumables) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorWithConsumables> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CurrentAppSimulator {
    const NAME: &'static str = "Windows.ApplicationModel.Store.CurrentAppSimulator";
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct LicenseInformation(::windows_core::IUnknown);
impl LicenseInformation {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProductLicenses(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ProductLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductLicenses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTrial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTrial)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<LicenseChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLicenseChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLicenseChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for LicenseInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseInformation {}
impl ::core::fmt::Debug for LicenseInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseInformation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LicenseInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseInformation;{8eb7dc30-f170-4ed5-8e21-1516da3fd367})");
}
impl ::core::clone::Clone for LicenseInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LicenseInformation {
    type Vtable = ILicenseInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LicenseInformation {
    const IID: ::windows_core::GUID = <ILicenseInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LicenseInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseInformation";
}
::windows_core::imp::interface_hierarchy!(LicenseInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LicenseInformation {}
unsafe impl ::core::marker::Sync for LicenseInformation {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct ListingInformation(::windows_core::IUnknown);
impl ListingInformation {
    pub fn CurrentMarket(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentMarket)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProductListings(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ProductListing>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductListings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FormattedPrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormattedPrice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AgeRating(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AgeRating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FormattedBasePrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormattedBasePrice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaleEndDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaleEndDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOnSale(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOnSale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrencyCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ListingInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ListingInformation {}
impl ::core::fmt::Debug for ListingInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListingInformation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ListingInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ListingInformation;{588b4abf-bc74-4383-b78c-99606323dece})");
}
impl ::core::clone::Clone for ListingInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ListingInformation {
    type Vtable = IListingInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ListingInformation {
    const IID: ::windows_core::GUID = <IListingInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ListingInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ListingInformation";
}
::windows_core::imp::interface_hierarchy!(ListingInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ListingInformation {}
unsafe impl ::core::marker::Sync for ListingInformation {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct ProductLicense(::windows_core::IUnknown);
impl ProductLicense {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConsumable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IProductLicenseWithFulfillment>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConsumable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProductLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProductLicense {}
impl ::core::fmt::Debug for ProductLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductLicense").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProductLicense {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ProductLicense;{363308c7-2bcf-4c0e-8f2f-e808aaa8f99d})");
}
impl ::core::clone::Clone for ProductLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ProductLicense {
    type Vtable = IProductLicense_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProductLicense {
    const IID: ::windows_core::GUID = <IProductLicense as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProductLicense {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductLicense";
}
::windows_core::imp::interface_hierarchy!(ProductLicense, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProductLicense {}
unsafe impl ::core::marker::Sync for ProductLicense {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct ProductListing(::windows_core::IUnknown);
impl ProductListing {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FormattedPrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormattedPrice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FormattedBasePrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormattedBasePrice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaleEndDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaleEndDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOnSale(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOnSale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrencyCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Keywords)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProductType(&self) -> ::windows_core::Result<ProductType> {
        let this = &::windows_core::ComInterface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProductListing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProductListing {}
impl ::core::fmt::Debug for ProductListing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductListing").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProductListing {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ProductListing;{45a7d6ad-c750-4d9c-947c-b00dcbf9e9c2})");
}
impl ::core::clone::Clone for ProductListing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ProductListing {
    type Vtable = IProductListing_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProductListing {
    const IID: ::windows_core::GUID = <IProductListing as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProductListing {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductListing";
}
::windows_core::imp::interface_hierarchy!(ProductListing, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProductListing {}
unsafe impl ::core::marker::Sync for ProductListing {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct ProductPurchaseDisplayProperties(::windows_core::IUnknown);
impl ProductPurchaseDisplayProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProductPurchaseDisplayProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Image(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Image)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetImage<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateProductPurchaseDisplayProperties(name: &::windows_core::HSTRING) -> ::windows_core::Result<ProductPurchaseDisplayProperties> {
        Self::IProductPurchaseDisplayPropertiesFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateProductPurchaseDisplayProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProductPurchaseDisplayPropertiesFactory<R, F: FnOnce(&IProductPurchaseDisplayPropertiesFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProductPurchaseDisplayProperties, IProductPurchaseDisplayPropertiesFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ProductPurchaseDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProductPurchaseDisplayProperties {}
impl ::core::fmt::Debug for ProductPurchaseDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductPurchaseDisplayProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProductPurchaseDisplayProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties;{d70b7420-bc92-401b-a809-c9b2e5dbbdaf})");
}
impl ::core::clone::Clone for ProductPurchaseDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ProductPurchaseDisplayProperties {
    type Vtable = IProductPurchaseDisplayProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProductPurchaseDisplayProperties {
    const IID: ::windows_core::GUID = <IProductPurchaseDisplayProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProductPurchaseDisplayProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties";
}
::windows_core::imp::interface_hierarchy!(ProductPurchaseDisplayProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProductPurchaseDisplayProperties {}
unsafe impl ::core::marker::Sync for ProductPurchaseDisplayProperties {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct PurchaseResults(::windows_core::IUnknown);
impl PurchaseResults {
    pub fn Status(&self) -> ::windows_core::Result<ProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransactionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransactionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReceiptXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReceiptXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OfferId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OfferId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PurchaseResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PurchaseResults {}
impl ::core::fmt::Debug for PurchaseResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PurchaseResults").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PurchaseResults {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.PurchaseResults;{ed50b37e-8656-4f65-b8c8-ac7e0cb1a1c2})");
}
impl ::core::clone::Clone for PurchaseResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PurchaseResults {
    type Vtable = IPurchaseResults_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PurchaseResults {
    const IID: ::windows_core::GUID = <IPurchaseResults as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.PurchaseResults";
}
::windows_core::imp::interface_hierarchy!(PurchaseResults, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PurchaseResults {}
unsafe impl ::core::marker::Sync for PurchaseResults {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct UnfulfilledConsumable(::windows_core::IUnknown);
impl UnfulfilledConsumable {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransactionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransactionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OfferId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OfferId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UnfulfilledConsumable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnfulfilledConsumable {}
impl ::core::fmt::Debug for UnfulfilledConsumable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnfulfilledConsumable").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UnfulfilledConsumable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.UnfulfilledConsumable;{2df7fbbb-1cdd-4cb8-a014-7b9cf8986927})");
}
impl ::core::clone::Clone for UnfulfilledConsumable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UnfulfilledConsumable {
    type Vtable = IUnfulfilledConsumable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UnfulfilledConsumable {
    const IID: ::windows_core::GUID = <IUnfulfilledConsumable as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UnfulfilledConsumable {
    const NAME: &'static str = "Windows.ApplicationModel.Store.UnfulfilledConsumable";
}
::windows_core::imp::interface_hierarchy!(UnfulfilledConsumable, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UnfulfilledConsumable {}
unsafe impl ::core::marker::Sync for UnfulfilledConsumable {}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for FulfillmentResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FulfillmentResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FulfillmentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FulfillmentResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FulfillmentResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.FulfillmentResult;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ProductPurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProductPurchaseStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProductPurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductPurchaseStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProductPurchaseStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.ProductPurchaseStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ProductType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProductType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProductType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProductType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.ProductType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct LicenseChangedEventHandler(pub ::windows_core::IUnknown);
impl LicenseChangedEventHandler {
    pub fn new<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = LicenseChangedEventHandlerBox::<F> { vtable: &LicenseChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct LicenseChangedEventHandlerBox<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const LicenseChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> LicenseChangedEventHandlerBox<F> {
    const VTABLE: LicenseChangedEventHandler_Vtbl = LicenseChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<LicenseChangedEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::cmp::PartialEq for LicenseChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseChangedEventHandler {}
impl ::core::fmt::Debug for LicenseChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for LicenseChangedEventHandler {
    type Vtable = LicenseChangedEventHandler_Vtbl;
}
impl ::core::clone::Clone for LicenseChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for LicenseChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4a50255_1369_4c36_832f_6f2d88e3659b);
}
impl ::windows_core::RuntimeType for LicenseChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d4a50255-1369-4c36-832f-6f2d88e3659b}");
}
#[repr(C)]
#[doc(hidden)]
pub struct LicenseChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
