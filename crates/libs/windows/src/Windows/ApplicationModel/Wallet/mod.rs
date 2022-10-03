#[cfg(feature = "ApplicationModel_Wallet_System")]
pub mod System;
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletBarcode(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletBarcode {
    type Vtable = IWalletBarcode_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletBarcode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f857b29_de80_4ea4_a1cd_81cd084dac27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletBarcode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Symbology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WalletBarcodeSymbology) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetImageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletBarcodeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletBarcodeFactory {
    type Vtable = IWalletBarcodeFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletBarcodeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30117161_ed9c_469e_bbfd_306c95ea7108);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletBarcodeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWalletBarcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbology: WalletBarcodeSymbology, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCustomWalletBarcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamtobarcodeimage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCustomWalletBarcode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletItem {
    type Vtable = IWalletItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20b54be8_118d_4ec4_996c_b963e7bd3e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsAcknowledged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAcknowledged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IssuerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIssuerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdated: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WalletItemKind) -> ::windows::core::HRESULT,
    pub Barcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBarcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationDate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Logo159x159: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo159x159: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo159x159: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo159x159: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Logo336x336: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo336x336: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo336x336: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo336x336: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Logo99x99: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo99x99: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo99x99: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo99x99: usize,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsDisplayMessageLaunchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDisplayMessageLaunchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub LogoText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLogoText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub HeaderColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    HeaderColor: usize,
    #[cfg(feature = "UI")]
    pub SetHeaderColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetHeaderColor: usize,
    #[cfg(feature = "UI")]
    pub BodyColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BodyColor: usize,
    #[cfg(feature = "UI")]
    pub SetBodyColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBodyColor: usize,
    #[cfg(feature = "UI")]
    pub HeaderFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    HeaderFontColor: usize,
    #[cfg(feature = "UI")]
    pub SetHeaderFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetHeaderFontColor: usize,
    #[cfg(feature = "UI")]
    pub BodyFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BodyFontColor: usize,
    #[cfg(feature = "UI")]
    pub SetBodyFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBodyFontColor: usize,
    #[cfg(feature = "Storage_Streams")]
    pub HeaderBackgroundImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    HeaderBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetHeaderBackgroundImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetHeaderBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BodyBackgroundImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BodyBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBodyBackgroundImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBodyBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LogoImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LogoImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogoImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogoImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PromotionalImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PromotionalImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPromotionalImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPromotionalImage: usize,
    #[cfg(feature = "Foundation")]
    pub RelevantDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelevantDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetRelevantDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRelevantDate: usize,
    pub RelevantDateDisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRelevantDateDisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TransactionHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransactionHistory: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RelevantLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelevantLocations: usize,
    pub IsMoreTransactionHistoryLaunchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsMoreTransactionHistoryLaunchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Verbs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Verbs: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemCustomProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletItemCustomProperty {
    type Vtable = IWalletItemCustomProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletItemCustomProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb94b40f3_fa00_40fd_98dc_9de46697f1e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemCustomProperty_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AutoDetectLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoDetectLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DetailViewPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WalletDetailViewPosition) -> ::windows::core::HRESULT,
    pub SetDetailViewPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WalletDetailViewPosition) -> ::windows::core::HRESULT,
    pub SummaryViewPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WalletSummaryViewPosition) -> ::windows::core::HRESULT,
    pub SetSummaryViewPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WalletSummaryViewPosition) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemCustomPropertyFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletItemCustomPropertyFactory {
    type Vtable = IWalletItemCustomPropertyFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletItemCustomPropertyFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0046a44_61a1_41aa_b259_a5610ab5d575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemCustomPropertyFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWalletItemCustomProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletItemFactory {
    type Vtable = IWalletItemFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletItemFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53e27470_4f0b_4a3e_99e5_0bbb1eab38d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWalletItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: WalletItemKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletItemStore {
    type Vtable = IWalletItemStore_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletItemStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7160484b_6d49_48f8_91a9_40a1d0f13ef4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetWalletItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWalletItemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsWithKindAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: WalletItemKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsWithKindAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletItemStore2 {
    type Vtable = IWalletItemStore2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletItemStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65e682f0_7009_4a15_bd54_4fff379bffe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemStore2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ItemsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletManagerStatics {
    type Vtable = IWalletManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5111d6b8_c9a4_4c64_b4dd_e1e548001c0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletRelevantLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletRelevantLocation {
    type Vtable = IWalletRelevantLocation_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletRelevantLocation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fd8782a_e3f9_4de1_bab3_bb192e46b3f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletRelevantLocation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetPosition: usize,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletTransaction(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletTransaction {
    type Vtable = IWalletTransaction_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletTransaction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40e1e940_2606_4519_81cb_bff1c60d1f79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletTransaction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IgnoreTimeOfDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnoreTimeOfDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisplayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransactionDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransactionDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransactionDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransactionDate: usize,
    pub IsLaunchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsLaunchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletVerb(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletVerb {
    type Vtable = IWalletVerb_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletVerb {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17b826d6_e3c1_4c74_8a94_217aadbc4884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletVerb_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletVerbFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWalletVerbFactory {
    type Vtable = IWalletVerbFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWalletVerbFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76012771_be58_4d5e_83ed_58b1669c7ad9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletVerbFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWalletVerb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletBarcode(::windows::core::IUnknown);
impl WalletBarcode {
    pub fn Symbology(&self) -> ::windows::core::Result<WalletBarcodeSymbology> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Symbology)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WalletBarcodeSymbology>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetImageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetImageAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        }
    }
    pub fn CreateWalletBarcode(symbology: WalletBarcodeSymbology, value: &::windows::core::HSTRING) -> ::windows::core::Result<WalletBarcode> {
        Self::IWalletBarcodeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWalletBarcode)(::windows::core::Vtable::as_raw(this), symbology, ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<WalletBarcode>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCustomWalletBarcode<'a, P0, E0>(streamtobarcodeimage: P0) -> ::windows::core::Result<WalletBarcode>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IWalletBarcodeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCustomWalletBarcode)(::windows::core::Vtable::as_raw(this), streamtobarcodeimage.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<WalletBarcode>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWalletBarcodeFactory<R, F: FnOnce(&IWalletBarcodeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletBarcode, IWalletBarcodeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WalletBarcode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletBarcode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletBarcode {}
impl ::core::fmt::Debug for WalletBarcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletBarcode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletBarcode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletBarcode;{4f857b29-de80-4ea4-a1cd-81cd084dac27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletBarcode {
    type Vtable = IWalletBarcode_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletBarcode {
    const IID: ::windows::core::GUID = <IWalletBarcode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletBarcode {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletBarcode";
}
::windows::core::interface_hierarchy!(WalletBarcode, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletBarcode {}
unsafe impl ::core::marker::Sync for WalletBarcode {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletItem(::windows::core::IUnknown);
impl WalletItem {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsAcknowledged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAcknowledged)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAcknowledged(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsAcknowledged)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IssuerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IssuerDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIssuerDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIssuerDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdated(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastUpdated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastUpdated<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastUpdated)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<WalletItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WalletItemKind>(result__)
        }
    }
    pub fn Barcode(&self) -> ::windows::core::Result<WalletBarcode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Barcode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WalletBarcode>(result__)
        }
    }
    pub fn SetBarcode(&self, value: &WalletBarcode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBarcode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationDate<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpirationDate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo159x159(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Logo159x159)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogo159x159<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogo159x159)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo336x336(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Logo336x336)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogo336x336<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogo336x336)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo99x99(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Logo99x99)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogo99x99<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogo99x99)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDisplayMessageLaunchable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisplayMessageLaunchable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDisplayMessageLaunchable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDisplayMessageLaunchable)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LogoText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LogoText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLogoText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogoText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn HeaderColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetHeaderColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHeaderColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BodyColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BodyColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBodyColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBodyColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn HeaderFontColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderFontColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetHeaderFontColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHeaderFontColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BodyFontColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BodyFontColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBodyFontColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBodyFontColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn HeaderBackgroundImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderBackgroundImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetHeaderBackgroundImage<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHeaderBackgroundImage)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BodyBackgroundImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BodyBackgroundImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBodyBackgroundImage<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBodyBackgroundImage)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LogoImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LogoImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogoImage<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogoImage)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PromotionalImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PromotionalImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPromotionalImage<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPromotionalImage)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelevantDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RelevantDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelevantDate<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRelevantDate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn RelevantDateDisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RelevantDateDisplayMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRelevantDateDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRelevantDateDisplayMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransactionHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletTransaction>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransactionHistory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RelevantLocations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletRelevantLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RelevantLocations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletRelevantLocation>>(result__)
        }
    }
    pub fn IsMoreTransactionHistoryLaunchable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMoreTransactionHistoryLaunchable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMoreTransactionHistoryLaunchable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsMoreTransactionHistoryLaunchable)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletItemCustomProperty>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletItemCustomProperty>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Verbs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletVerb>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Verbs)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletVerb>>(result__)
        }
    }
    pub fn CreateWalletItem(kind: WalletItemKind, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<WalletItem> {
        Self::IWalletItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWalletItem)(::windows::core::Vtable::as_raw(this), kind, ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi::<WalletItem>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWalletItemFactory<R, F: FnOnce(&IWalletItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletItem, IWalletItemFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WalletItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletItem {}
impl ::core::fmt::Debug for WalletItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletItem;{20b54be8-118d-4ec4-996c-b963e7bd3e74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletItem {
    type Vtable = IWalletItem_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletItem {
    const IID: ::windows::core::GUID = <IWalletItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletItem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletItem";
}
::windows::core::interface_hierarchy!(WalletItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletItem {}
unsafe impl ::core::marker::Sync for WalletItem {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletItemCustomProperty(::windows::core::IUnknown);
impl WalletItemCustomProperty {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AutoDetectLinks(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoDetectLinks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoDetectLinks(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoDetectLinks)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DetailViewPosition(&self) -> ::windows::core::Result<WalletDetailViewPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetailViewPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WalletDetailViewPosition>(result__)
        }
    }
    pub fn SetDetailViewPosition(&self, value: WalletDetailViewPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDetailViewPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SummaryViewPosition(&self) -> ::windows::core::Result<WalletSummaryViewPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SummaryViewPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WalletSummaryViewPosition>(result__)
        }
    }
    pub fn SetSummaryViewPosition(&self, value: WalletSummaryViewPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSummaryViewPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CreateWalletItemCustomProperty(name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<WalletItemCustomProperty> {
        Self::IWalletItemCustomPropertyFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWalletItemCustomProperty)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<WalletItemCustomProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWalletItemCustomPropertyFactory<R, F: FnOnce(&IWalletItemCustomPropertyFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletItemCustomProperty, IWalletItemCustomPropertyFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WalletItemCustomProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletItemCustomProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletItemCustomProperty {}
impl ::core::fmt::Debug for WalletItemCustomProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemCustomProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItemCustomProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletItemCustomProperty;{b94b40f3-fa00-40fd-98dc-9de46697f1e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletItemCustomProperty {
    type Vtable = IWalletItemCustomProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletItemCustomProperty {
    const IID: ::windows::core::GUID = <IWalletItemCustomProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletItemCustomProperty {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletItemCustomProperty";
}
::windows::core::interface_hierarchy!(WalletItemCustomProperty, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletItemCustomProperty {}
unsafe impl ::core::marker::Sync for WalletItemCustomProperty {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletItemStore(::windows::core::IUnknown);
impl WalletItemStore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddAsync(&self, id: &::windows::core::HSTRING, item: &WalletItem) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClearAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetWalletItemAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWalletItemAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WalletItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetItemsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsWithKindAsync(&self, kind: WalletItemKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetItemsWithKindAsync)(::windows::core::Vtable::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportItemAsync<'a, P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItem>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportItemAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WalletItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowItemAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowItemAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync(&self, item: &WalletItem) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for WalletItemStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletItemStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletItemStore {}
impl ::core::fmt::Debug for WalletItemStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItemStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletItemStore;{7160484b-6d49-48f8-91a9-40a1d0f13ef4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletItemStore {
    type Vtable = IWalletItemStore_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletItemStore {
    const IID: ::windows::core::GUID = <IWalletItemStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletItemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletItemStore";
}
::windows::core::interface_hierarchy!(WalletItemStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletItemStore {}
unsafe impl ::core::marker::Sync for WalletItemStore {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
pub struct WalletManager;
impl WalletManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItemStore>> {
        Self::IWalletManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestStoreAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WalletItemStore>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWalletManagerStatics<R, F: FnOnce(&IWalletManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletManager, IWalletManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WalletManager {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletManager";
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletRelevantLocation(::windows::core::IUnknown);
impl WalletRelevantLocation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletRelevantLocation, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::BasicGeoposition>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetPosition(&self, value: super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for WalletRelevantLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletRelevantLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletRelevantLocation {}
impl ::core::fmt::Debug for WalletRelevantLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletRelevantLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletRelevantLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletRelevantLocation;{9fd8782a-e3f9-4de1-bab3-bb192e46b3f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletRelevantLocation {
    type Vtable = IWalletRelevantLocation_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletRelevantLocation {
    const IID: ::windows::core::GUID = <IWalletRelevantLocation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletRelevantLocation {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletRelevantLocation";
}
::windows::core::interface_hierarchy!(WalletRelevantLocation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletRelevantLocation {}
unsafe impl ::core::marker::Sync for WalletRelevantLocation {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletTransaction(::windows::core::IUnknown);
impl WalletTransaction {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletTransaction, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayAmount(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayAmount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayAmount(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayAmount)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IgnoreTimeOfDay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IgnoreTimeOfDay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreTimeOfDay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIgnoreTimeOfDay)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DisplayLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayLocation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransactionDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransactionDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTransactionDate<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransactionDate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn IsLaunchable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLaunchable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLaunchable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsLaunchable)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for WalletTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletTransaction {}
impl ::core::fmt::Debug for WalletTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletTransaction;{40e1e940-2606-4519-81cb-bff1c60d1f79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletTransaction {
    type Vtable = IWalletTransaction_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletTransaction {
    const IID: ::windows::core::GUID = <IWalletTransaction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletTransaction";
}
::windows::core::interface_hierarchy!(WalletTransaction, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletTransaction {}
unsafe impl ::core::marker::Sync for WalletTransaction {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletVerb(::windows::core::IUnknown);
impl WalletVerb {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateWalletVerb(name: &::windows::core::HSTRING) -> ::windows::core::Result<WalletVerb> {
        Self::IWalletVerbFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWalletVerb)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<WalletVerb>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWalletVerbFactory<R, F: FnOnce(&IWalletVerbFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WalletVerb, IWalletVerbFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WalletVerb {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletVerb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletVerb {}
impl ::core::fmt::Debug for WalletVerb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletVerb").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletVerb {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.WalletVerb;{17b826d6-e3c1-4c74-8a94-217aadbc4884})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WalletVerb {
    type Vtable = IWalletVerb_Vtbl;
}
unsafe impl ::windows::core::Interface for WalletVerb {
    const IID: ::windows::core::GUID = <IWalletVerb as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WalletVerb {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletVerb";
}
::windows::core::interface_hierarchy!(WalletVerb, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WalletVerb {}
unsafe impl ::core::marker::Sync for WalletVerb {}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WalletActionKind(pub i32);
impl WalletActionKind {
    pub const OpenItem: Self = Self(0i32);
    pub const Transaction: Self = Self(1i32);
    pub const MoreTransactions: Self = Self(2i32);
    pub const Message: Self = Self(3i32);
    pub const Verb: Self = Self(4i32);
}
impl ::core::marker::Copy for WalletActionKind {}
impl ::core::clone::Clone for WalletActionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WalletActionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WalletActionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WalletActionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletActionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletActionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletActionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WalletBarcodeSymbology(pub i32);
impl WalletBarcodeSymbology {
    pub const Invalid: Self = Self(0i32);
    pub const Upca: Self = Self(1i32);
    pub const Upce: Self = Self(2i32);
    pub const Ean13: Self = Self(3i32);
    pub const Ean8: Self = Self(4i32);
    pub const Itf: Self = Self(5i32);
    pub const Code39: Self = Self(6i32);
    pub const Code128: Self = Self(7i32);
    pub const Qr: Self = Self(8i32);
    pub const Pdf417: Self = Self(9i32);
    pub const Aztec: Self = Self(10i32);
    pub const Custom: Self = Self(100000i32);
}
impl ::core::marker::Copy for WalletBarcodeSymbology {}
impl ::core::clone::Clone for WalletBarcodeSymbology {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WalletBarcodeSymbology {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WalletBarcodeSymbology {
    type Abi = Self;
}
impl ::core::fmt::Debug for WalletBarcodeSymbology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletBarcodeSymbology").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletBarcodeSymbology {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletBarcodeSymbology;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WalletDetailViewPosition(pub i32);
impl WalletDetailViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const HeaderField1: Self = Self(1i32);
    pub const HeaderField2: Self = Self(2i32);
    pub const PrimaryField1: Self = Self(3i32);
    pub const PrimaryField2: Self = Self(4i32);
    pub const SecondaryField1: Self = Self(5i32);
    pub const SecondaryField2: Self = Self(6i32);
    pub const SecondaryField3: Self = Self(7i32);
    pub const SecondaryField4: Self = Self(8i32);
    pub const SecondaryField5: Self = Self(9i32);
    pub const CenterField1: Self = Self(10i32);
    pub const FooterField1: Self = Self(11i32);
    pub const FooterField2: Self = Self(12i32);
    pub const FooterField3: Self = Self(13i32);
    pub const FooterField4: Self = Self(14i32);
}
impl ::core::marker::Copy for WalletDetailViewPosition {}
impl ::core::clone::Clone for WalletDetailViewPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WalletDetailViewPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WalletDetailViewPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for WalletDetailViewPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletDetailViewPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletDetailViewPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletDetailViewPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WalletItemKind(pub i32);
impl WalletItemKind {
    pub const Invalid: Self = Self(0i32);
    pub const Deal: Self = Self(1i32);
    pub const General: Self = Self(2i32);
    pub const PaymentInstrument: Self = Self(3i32);
    pub const Ticket: Self = Self(4i32);
    pub const BoardingPass: Self = Self(5i32);
    pub const MembershipCard: Self = Self(6i32);
}
impl ::core::marker::Copy for WalletItemKind {}
impl ::core::clone::Clone for WalletItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WalletItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WalletItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WalletItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WalletSummaryViewPosition(pub i32);
impl WalletSummaryViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const Field1: Self = Self(1i32);
    pub const Field2: Self = Self(2i32);
}
impl ::core::marker::Copy for WalletSummaryViewPosition {}
impl ::core::clone::Clone for WalletSummaryViewPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WalletSummaryViewPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WalletSummaryViewPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for WalletSummaryViewPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletSummaryViewPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WalletSummaryViewPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletSummaryViewPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
