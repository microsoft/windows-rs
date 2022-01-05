#[cfg(feature = "implement_exclusive")]
pub trait IWalletBarcodeImpl: Sized {
    fn Symbology(&self) -> ::windows::core::Result<WalletBarcodeSymbology>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetImageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamReference>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletBarcodeFactoryImpl: Sized {
    fn CreateWalletBarcode(&self, symbology: WalletBarcodeSymbology, value: &::windows::core::HSTRING) -> ::windows::core::Result<WalletBarcode>;
    fn CreateCustomWalletBarcode(&self, streamtobarcodeimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<WalletBarcode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsAcknowledged(&self) -> ::windows::core::Result<bool>;
    fn SetIsAcknowledged(&self, value: bool) -> ::windows::core::Result<()>;
    fn IssuerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIssuerDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastUpdated(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetLastUpdated(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<WalletItemKind>;
    fn Barcode(&self) -> ::windows::core::Result<WalletBarcode>;
    fn SetBarcode(&self, value: &::core::option::Option<WalletBarcode>) -> ::windows::core::Result<()>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationDate(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Logo159x159(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo159x159(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Logo336x336(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo336x336(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Logo99x99(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo99x99(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsDisplayMessageLaunchable(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisplayMessageLaunchable(&self, value: bool) -> ::windows::core::Result<()>;
    fn LogoText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogoText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HeaderColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetHeaderColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BodyColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBodyColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn HeaderFontColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetHeaderFontColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BodyFontColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBodyFontColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn HeaderBackgroundImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetHeaderBackgroundImage(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn BodyBackgroundImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetBodyBackgroundImage(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn LogoImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogoImage(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn PromotionalImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetPromotionalImage(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn RelevantDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetRelevantDate(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn RelevantDateDisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRelevantDateDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransactionHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletTransaction>>;
    fn RelevantLocations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletRelevantLocation>>;
    fn IsMoreTransactionHistoryLaunchable(&self) -> ::windows::core::Result<bool>;
    fn SetIsMoreTransactionHistoryLaunchable(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletItemCustomProperty>>;
    fn Verbs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletVerb>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemCustomPropertyImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AutoDetectLinks(&self) -> ::windows::core::Result<bool>;
    fn SetAutoDetectLinks(&self, value: bool) -> ::windows::core::Result<()>;
    fn DetailViewPosition(&self) -> ::windows::core::Result<WalletDetailViewPosition>;
    fn SetDetailViewPosition(&self, value: WalletDetailViewPosition) -> ::windows::core::Result<()>;
    fn SummaryViewPosition(&self) -> ::windows::core::Result<WalletSummaryViewPosition>;
    fn SetSummaryViewPosition(&self, value: WalletSummaryViewPosition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemCustomPropertyFactoryImpl: Sized {
    fn CreateWalletItemCustomProperty(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<WalletItemCustomProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemFactoryImpl: Sized {
    fn CreateWalletItem(&self, kind: WalletItemKind, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<WalletItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemStoreImpl: Sized {
    fn AddAsync(&self, id: &::windows::core::HSTRING, item: &::core::option::Option<WalletItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetWalletItemAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItem>>;
    fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>>;
    fn GetItemsWithKindAsync(&self, kind: WalletItemKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>>;
    fn ImportItemAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItem>>;
    fn DeleteAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowItemAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateAsync(&self, item: &::core::option::Option<WalletItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemStore2Impl: Sized {
    fn ItemsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WalletItemStore, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItemStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletRelevantLocationImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::BasicGeoposition>;
    fn SetPosition(&self, value: &super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::Result<()>;
    fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletTransactionImpl: Sized {
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayAmount(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayAmount(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IgnoreTimeOfDay(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTimeOfDay(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransactionDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTransactionDate(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn IsLaunchable(&self) -> ::windows::core::Result<bool>;
    fn SetIsLaunchable(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletVerbImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletVerbFactoryImpl: Sized {
    fn CreateWalletVerb(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<WalletVerb>;
}
