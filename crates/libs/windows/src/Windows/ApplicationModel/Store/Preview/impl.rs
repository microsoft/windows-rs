#[cfg(feature = "implement_exclusive")]
pub trait IDeliveryOptimizationSettingsImpl: Sized {
    fn DownloadMode(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadMode>;
    fn DownloadModeSource(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadModeSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeliveryOptimizationSettingsStaticsImpl: Sized {
    fn GetCurrentSettings(&self) -> ::windows::core::Result<DeliveryOptimizationSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStaticsImpl: Sized {
    fn SetSystemConfiguration(&self, cataloghardwaremanufacturerid: &::windows::core::HSTRING, catalogstorecontentmodifierid: &::windows::core::HSTRING, systemconfigurationexpiration: &super::super::super::Foundation::DateTime, cataloghardwaredescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetMobileOperatorConfiguration(&self, mobileoperatorid: &::windows::core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::Result<()>;
    fn SetStoreWebAccountId(&self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsStoreWebAccountId(&self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn HardwareManufacturerInfo(&self) -> ::windows::core::Result<StoreHardwareManufacturerInfo>;
    fn FilterUnsupportedSystemFeaturesAsync(&self, systemfeatures: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics2Impl: Sized {
    fn PurchasePromptingPolicy(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetPurchasePromptingPolicy(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics3Impl: Sized {
    fn HasStoreWebAccount(&self) -> ::windows::core::Result<bool>;
    fn HasStoreWebAccountForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<bool>;
    fn GetStoreLogDataAsync(&self, options: StoreLogOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>>;
    fn SetStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetPurchasePromptingPolicyForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetPurchasePromptingPolicyForUser(&self, user: &::core::option::Option<super::super::super::System::User>, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics4Impl: Sized {
    fn GetStoreWebAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseStoreWebAccountId(&self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetEnterpriseStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetEnterpriseStoreWebAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetEnterpriseStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShouldRestrictToEnterpriseStoreOnly(&self) -> ::windows::core::Result<bool>;
    fn ShouldRestrictToEnterpriseStoreOnlyForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics5Impl: Sized {
    fn IsPinToDesktopSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPinToTaskbarSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPinToStartSupported(&self) -> ::windows::core::Result<bool>;
    fn PinToDesktop(&self, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PinToDesktopForUser(&self, user: &::core::option::Option<super::super::super::System::User>, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreHardwareManufacturerInfoImpl: Sized {
    fn HardwareManufacturerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StoreContentModifierId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewImpl: Sized {
    fn RequestProductPurchaseByProductIdAndSkuIdAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>>;
    fn LoadAddOnProductInfosAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewProductInfoImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuInfoList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewPurchaseResultsImpl: Sized {
    fn ProductPurchaseStatus(&self) -> ::windows::core::Result<StorePreviewProductPurchaseStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewSkuInfoImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedListPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExtendedData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerHelperImpl: Sized {
    fn RequestTokenWithUIElementHostingAsync(&self, request: &::core::option::Option<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, uielement: &::core::option::Option<super::super::super::UI::Xaml::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>;
    fn RequestTokenWithUIElementHostingAndWebAccountAsync(&self, request: &::core::option::Option<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Security::Credentials::WebAccount>, uielement: &::core::option::Option<super::super::super::UI::Xaml::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>;
}
