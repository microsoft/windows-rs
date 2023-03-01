#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
#[doc(hidden)]
#[repr(transparent)]
pub struct IClipboardContentOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClipboardContentOptions {
    type Vtable = IClipboardContentOptions_Vtbl;
}
impl ::core::clone::Clone for IClipboardContentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipboardContentOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe888a98c_ad4b_5447_a056_ab3556276d2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardContentOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAllowedInHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAllowedInHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RoamingFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RoamingFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub HistoryFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HistoryFormats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClipboardHistoryChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClipboardHistoryChangedEventArgs {
    type Vtable = IClipboardHistoryChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IClipboardHistoryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipboardHistoryChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0be453f_8ea2_53ce_9aba_8d2212573452);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardHistoryChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClipboardHistoryItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClipboardHistoryItem {
    type Vtable = IClipboardHistoryItem_Vtbl;
}
impl ::core::clone::Clone for IClipboardHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipboardHistoryItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0173bd8a_afff_5c50_ab92_3d19f481ec58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardHistoryItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClipboardHistoryItemsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClipboardHistoryItemsResult {
    type Vtable = IClipboardHistoryItemsResult_Vtbl;
}
impl ::core::clone::Clone for IClipboardHistoryItemsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipboardHistoryItemsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6dfdee6_0ee2_52e3_852b_f295db65939a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardHistoryItemsResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClipboardHistoryItemsResultStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClipboardStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClipboardStatics {
    type Vtable = IClipboardStatics_Vtbl;
}
impl ::core::clone::Clone for IClipboardStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipboardStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc627e291_34e2_4963_8eed_93cbb0ea3d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClipboardStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClipboardStatics2 {
    type Vtable = IClipboardStatics2_Vtbl;
}
impl ::core::clone::Clone for IClipboardStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipboardStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac1b6a_d29f_554b_b303_f0452345fe02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipboardStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetHistoryItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHistoryItemsAsync: usize,
    pub ClearHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DeleteItemFromHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHistoryItemAsContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut SetHistoryItemAsContentStatus) -> ::windows::core::HRESULT,
    pub IsHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRoamingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetContentWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HistoryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoryChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHistoryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHistoryChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RoamingEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RoamingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRoamingEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRoamingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub HistoryEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoryEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHistoryEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHistoryEnabledChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackage {
    type Vtable = IDataPackage_Vtbl;
}
impl ::core::clone::Clone for IDataPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ebf5c7_efea_4346_9554_981d7e198ffe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataPackageOperation) -> ::windows::core::HRESULT,
    pub SetRequestedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DataPackageOperation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OperationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOperationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Destroyed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Destroyed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDestroyed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDestroyed: usize,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDataProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, delayrenderer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetUri: usize,
    pub SetHtmlFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ResourceMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ResourceMap: usize,
    pub SetRtf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBitmap: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SetStorageItemsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SetStorageItemsReadOnly: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SetStorageItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, readonly: bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SetStorageItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackage2 {
    type Vtable = IDataPackage2_Vtbl;
}
impl ::core::clone::Clone for IDataPackage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041c1fe9_2409_45e1_a538_4c53eeee04a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetApplicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetApplicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub SetWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWebLink: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackage3 {
    type Vtable = IDataPackage3_Vtbl;
}
impl ::core::clone::Clone for IDataPackage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f31f5d_787b_4d32_965a_a9838105a056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShareCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShareCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShareCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackage4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackage4 {
    type Vtable = IDataPackage4_Vtbl;
}
impl ::core::clone::Clone for IDataPackage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackage4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a24ec8_9382_536f_852a_3045e1b29a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShareCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShareCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShareCanceled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySet(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySet {
    type Vtable = IDataPackagePropertySet_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd1c93eb_4c4c_443a_a8d3_f5c241e91689);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypes: usize,
    pub ApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationListingUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationListingUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetApplicationListingUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetApplicationListingUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySet2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySet2 {
    type Vtable = IDataPackagePropertySet2_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySet2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySet2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb505d4a_9800_46aa_b181_7b6f0f2b919a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceWebLink: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentSourceWebLink: usize,
    #[cfg(feature = "Foundation")]
    pub ContentSourceApplicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceApplicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentSourceApplicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentSourceApplicationLink: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Square30x30Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSquare30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSquare30x30Logo: usize,
    #[cfg(feature = "UI")]
    pub LogoBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    LogoBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetLogoBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetLogoBackgroundColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySet3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySet3 {
    type Vtable = IDataPackagePropertySet3_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySet3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySet3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e87fd9b_5205_401b_874a_455653bd39e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySet4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySet4 {
    type Vtable = IDataPackagePropertySet4_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySet4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySet4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6390ebf5_1739_4c74_b22f_865fab5e8545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySet4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentSourceUserActivityJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentSourceUserActivityJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySetView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySetView {
    type Vtable = IDataPackagePropertySetView_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySetView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySetView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb94cec01_0c1a_4c57_be55_75d01289735d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypes: usize,
    pub ApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationListingUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationListingUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySetView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySetView2 {
    type Vtable = IDataPackagePropertySetView2_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySetView2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySetView2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6054509b_8ebe_4feb_9c1e_75e69de54b84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceWebLink: usize,
    #[cfg(feature = "Foundation")]
    pub ContentSourceApplicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceApplicationLink: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Square30x30Logo: usize,
    #[cfg(feature = "UI")]
    pub LogoBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    LogoBackgroundColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySetView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySetView3 {
    type Vtable = IDataPackagePropertySetView3_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySetView3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySetView3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb764ce5_d174_495c_84fc_1a51f6ab45d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySetView4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySetView4 {
    type Vtable = IDataPackagePropertySetView4_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySetView4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySetView4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4474c80d_d16f_40ae_9580_6f8562b94235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentSourceUserActivityJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackagePropertySetView5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackagePropertySetView5 {
    type Vtable = IDataPackagePropertySetView5_Vtbl;
}
impl ::core::clone::Clone for IDataPackagePropertySetView5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackagePropertySetView5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f0a9445_3760_50bb_8523_c4202ded7d78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackagePropertySetView5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFromRoamingClipboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackageView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackageView {
    type Vtable = IDataPackageView_Vtbl;
}
impl ::core::clone::Clone for IDataPackageView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackageView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b840471_5900_4d85_a90b_10cb85fe3552);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataPackageOperation) -> ::windows::core::HRESULT,
    pub ReportOperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DataPackageOperation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableFormats: usize,
    pub Contains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetHtmlFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHtmlFormatAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetResourceMapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetResourceMapAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRtfAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRtfAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetBitmapAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub GetStorageItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    GetStorageItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackageView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackageView2 {
    type Vtable = IDataPackageView2_Vtbl;
}
impl ::core::clone::Clone for IDataPackageView2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackageView2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40ecba95_2450_4c1d_b6b4_ed45463dee9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetApplicationLinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetApplicationLinkAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetWebLinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWebLinkAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackageView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackageView3 {
    type Vtable = IDataPackageView3_Vtbl;
}
impl ::core::clone::Clone for IDataPackageView3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackageView3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd37771a8_ddad_4288_8428_d1cae394128b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_EnterpriseData")))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    pub RequestAccessWithEnterpriseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterpriseid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_EnterpriseData")))]
    RequestAccessWithEnterpriseIdAsync: usize,
    #[cfg(feature = "Security_EnterpriseData")]
    pub UnlockAndAssumeEnterpriseIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_EnterpriseData"))]
    UnlockAndAssumeEnterpriseIdentity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPackageView4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataPackageView4 {
    type Vtable = IDataPackageView4_Vtbl;
}
impl ::core::clone::Clone for IDataPackageView4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataPackageView4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfe96f1f_e042_4433_a09f_26d6ffda8b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetAcceptedFormatId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProviderDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataProviderDeferral {
    type Vtable = IDataProviderDeferral_Vtbl;
}
impl ::core::clone::Clone for IDataProviderDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataProviderDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2cf2373_2d26_43d9_b69d_dcb86d03f6da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProviderDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProviderRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataProviderRequest {
    type Vtable = IDataProviderRequest_Vtbl;
}
impl ::core::clone::Clone for IDataProviderRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataProviderRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebbc7157_d3c8_47da_acde_f82388d5f716);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProviderRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FormatId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataRequest {
    type Vtable = IDataRequest_Vtbl;
}
impl ::core::clone::Clone for IDataRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4341ae3b_fc12_4e53_8c02_ac714c415a27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub FailWithDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataRequestDeferral {
    type Vtable = IDataRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for IDataRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc4b89f_0386_4263_87c1_ed7dce30890e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataRequestedEventArgs {
    type Vtable = IDataRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDataRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb8ba807_6ac5_43c9_8ac5_9ba232163182);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataTransferManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataTransferManager {
    type Vtable = IDataTransferManager_Vtbl;
}
impl ::core::clone::Clone for IDataTransferManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataTransferManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5caee9b_8708_49d1_8d36_67d25a8da00c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DataRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TargetApplicationChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetApplicationChosen: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetApplicationChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetApplicationChosen: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataTransferManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataTransferManager2 {
    type Vtable = IDataTransferManager2_Vtbl;
}
impl ::core::clone::Clone for IDataTransferManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataTransferManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30ae7d71_8ba8_4c02_8e3f_ddb23b388715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShareProvidersRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareProvidersRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShareProvidersRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShareProvidersRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataTransferManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataTransferManagerStatics {
    type Vtable = IDataTransferManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IDataTransferManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataTransferManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9da01aa_e00e_4cfe_aa44_2dd932dca3d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowShareUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataTransferManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataTransferManagerStatics2 {
    type Vtable = IDataTransferManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IDataTransferManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataTransferManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc54ec2ec_9f97_4d63_9868_395e271ad8f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataTransferManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataTransferManagerStatics3 {
    type Vtable = IDataTransferManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IDataTransferManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDataTransferManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05845473_6c82_4f5c_ac23_62e458361fac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTransferManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowShareUIWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlFormatHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHtmlFormatHelperStatics {
    type Vtable = IHtmlFormatHelperStatics_Vtbl;
}
impl ::core::clone::Clone for IHtmlFormatHelperStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHtmlFormatHelperStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe22e7749_dd70_446f_aefc_61cee59f655e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlFormatHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStaticFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlformat: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CreateHtmlFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlfragment: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOperationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOperationCompletedEventArgs {
    type Vtable = IOperationCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOperationCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7af329d_051d_4fab_b1a9_47fd77f70a41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataPackageOperation) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOperationCompletedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOperationCompletedEventArgs2 {
    type Vtable = IOperationCompletedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IOperationCompletedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOperationCompletedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858fa073_1e19_4105_b2f7_c8478808d562);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AcceptedFormatId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareCompletedEventArgs {
    type Vtable = IShareCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IShareCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4574c442_f913_4f60_9df7_cc4060ab1916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShareTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareProvider {
    type Vtable = IShareProvider_Vtbl;
}
impl ::core::clone::Clone for IShareProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fabe026_443e_4cda_af25_8d81070efd80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DisplayIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DisplayIcon: usize,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareProviderFactory {
    type Vtable = IShareProviderFactory_Vtbl;
}
impl ::core::clone::Clone for IShareProviderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareProviderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x172a174c_e79e_4f6d_b07d_128f469e0296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProviderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage_Streams", feature = "UI"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows::core::HSTRING>, displayicon: *mut ::core::ffi::c_void, backgroundcolor: super::super::UI::Color, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "UI")))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareProviderOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareProviderOperation {
    type Vtable = IShareProviderOperation_Vtbl;
}
impl ::core::clone::Clone for IShareProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareProviderOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19cef937_d435_4179_b6af_14e0492b69f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProviderOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Provider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareProvidersRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareProvidersRequestedEventArgs {
    type Vtable = IShareProvidersRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IShareProvidersRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareProvidersRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf888f356_a3f8_4fce_85e4_8826e63be799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareProvidersRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Providers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Providers: usize,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareTargetInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareTargetInfo {
    type Vtable = IShareTargetInfo_Vtbl;
}
impl ::core::clone::Clone for IShareTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareTargetInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x385be607_c6e8_4114_b294_28f3bb6f9904);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShareProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareUIOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareUIOptions {
    type Vtable = IShareUIOptions_Vtbl;
}
impl ::core::clone::Clone for IShareUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareUIOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72fa8a80_342f_4d90_9551_2ae04e37680c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareUIOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Theme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ShareUITheme) -> ::windows::core::HRESULT,
    pub SetTheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ShareUITheme) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedStorageAccessManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedStorageAccessManagerStatics {
    type Vtable = ISharedStorageAccessManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ISharedStorageAccessManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISharedStorageAccessManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6132ada_34b1_4849_bd5f_d09fee3158c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedStorageAccessManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub AddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AddFile: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub RedeemTokenForFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    RedeemTokenForFileAsync: usize,
    pub RemoveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardDataFormatsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardDataFormatsStatics {
    type Vtable = IStandardDataFormatsStatics_Vtbl;
}
impl ::core::clone::Clone for IStandardDataFormatsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStandardDataFormatsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed681a1_a880_40c9_b4ed_0bee1e15f549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Uri: usize,
    pub Html: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rtf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Bitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StorageItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardDataFormatsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardDataFormatsStatics2 {
    type Vtable = IStandardDataFormatsStatics2_Vtbl;
}
impl ::core::clone::Clone for IStandardDataFormatsStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStandardDataFormatsStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42a254f4_9d76_42e8_861b_47c25dd0cf71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ApplicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardDataFormatsStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardDataFormatsStatics3 {
    type Vtable = IStandardDataFormatsStatics3_Vtbl;
}
impl ::core::clone::Clone for IStandardDataFormatsStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStandardDataFormatsStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b57b069_01d4_474c_8b5f_bc8e27f38b21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardDataFormatsStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserActivityJsonArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetApplicationChosenEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetApplicationChosenEventArgs {
    type Vtable = ITargetApplicationChosenEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITargetApplicationChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITargetApplicationChosenEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca6fb8ac_2987_4ee3_9c54_d8afbcb86c1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetApplicationChosenEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
pub struct Clipboard;
impl Clipboard {
    pub fn GetContent() -> ::windows::core::Result<DataPackageView> {
        Self::IClipboardStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageView>();
            (::windows::core::Interface::vtable(this).GetContent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetContent(content: &DataPackage) -> ::windows::core::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetContent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content)).ok() })
    }
    pub fn Flush() -> ::windows::core::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Flush)(::windows::core::Interface::as_raw(this)).ok() })
    }
    pub fn Clear() -> ::windows::core::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ContentChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IClipboardStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveContentChanged)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetHistoryItemsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>>();
            (::windows::core::Interface::vtable(this).GetHistoryItemsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ClearHistory() -> ::windows::core::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ClearHistory)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DeleteItemFromHistory(item: &ClipboardHistoryItem) -> ::windows::core::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DeleteItemFromHistory)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(item), &mut result__).from_abi(result__)
        })
    }
    pub fn SetHistoryItemAsContent(item: &ClipboardHistoryItem) -> ::windows::core::Result<SetHistoryItemAsContentStatus> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SetHistoryItemAsContentStatus>();
            (::windows::core::Interface::vtable(this).SetHistoryItemAsContent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(item), &mut result__).from_abi(result__)
        })
    }
    pub fn IsHistoryEnabled() -> ::windows::core::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsHistoryEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsRoamingEnabled() -> ::windows::core::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRoamingEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetContentWithOptions(content: &DataPackage, options: &ClipboardContentOptions) -> ::windows::core::Result<bool> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SetContentWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HistoryChanged(handler: &super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).HistoryChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHistoryChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IClipboardStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveHistoryChanged)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RoamingEnabledChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).RoamingEnabledChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRoamingEnabledChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IClipboardStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRoamingEnabledChanged)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HistoryEnabledChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IClipboardStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).HistoryEnabledChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHistoryEnabledChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IClipboardStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveHistoryEnabledChanged)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IClipboardStatics<R, F: FnOnce(&IClipboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Clipboard, IClipboardStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IClipboardStatics2<R, F: FnOnce(&IClipboardStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Clipboard, IClipboardStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for Clipboard {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.Clipboard";
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ClipboardContentOptions(::windows::core::IUnknown);
impl ClipboardContentOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ClipboardContentOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsRoamable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRoamable)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRoamable)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAllowedInHistory(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAllowedInHistory)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAllowedInHistory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAllowedInHistory)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RoamingFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).RoamingFormats)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HistoryFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).HistoryFormats)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ClipboardContentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClipboardContentOptions {}
impl ::core::fmt::Debug for ClipboardContentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClipboardContentOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ClipboardContentOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardContentOptions;{e888a98c-ad4b-5447-a056-ab3556276d2b})");
}
impl ::core::clone::Clone for ClipboardContentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ClipboardContentOptions {
    type Vtable = IClipboardContentOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ClipboardContentOptions {
    const IID: ::windows::core::GUID = <IClipboardContentOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ClipboardContentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardContentOptions";
}
::windows::imp::interface_hierarchy!(ClipboardContentOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClipboardContentOptions {}
unsafe impl ::core::marker::Sync for ClipboardContentOptions {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ClipboardHistoryChangedEventArgs(::windows::core::IUnknown);
impl ClipboardHistoryChangedEventArgs {}
impl ::core::cmp::PartialEq for ClipboardHistoryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClipboardHistoryChangedEventArgs {}
impl ::core::fmt::Debug for ClipboardHistoryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClipboardHistoryChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ClipboardHistoryChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs;{c0be453f-8ea2-53ce-9aba-8d2212573452})");
}
impl ::core::clone::Clone for ClipboardHistoryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ClipboardHistoryChangedEventArgs {
    type Vtable = IClipboardHistoryChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ClipboardHistoryChangedEventArgs {
    const IID: ::windows::core::GUID = <IClipboardHistoryChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ClipboardHistoryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs";
}
::windows::imp::interface_hierarchy!(ClipboardHistoryChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClipboardHistoryChangedEventArgs {}
unsafe impl ::core::marker::Sync for ClipboardHistoryChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ClipboardHistoryItem(::windows::core::IUnknown);
impl ClipboardHistoryItem {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows::core::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageView>();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ClipboardHistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClipboardHistoryItem {}
impl ::core::fmt::Debug for ClipboardHistoryItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClipboardHistoryItem").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ClipboardHistoryItem {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem;{0173bd8a-afff-5c50-ab92-3d19f481ec58})");
}
impl ::core::clone::Clone for ClipboardHistoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ClipboardHistoryItem {
    type Vtable = IClipboardHistoryItem_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ClipboardHistoryItem {
    const IID: ::windows::core::GUID = <IClipboardHistoryItem as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ClipboardHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem";
}
::windows::imp::interface_hierarchy!(ClipboardHistoryItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClipboardHistoryItem {}
unsafe impl ::core::marker::Sync for ClipboardHistoryItem {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ClipboardHistoryItemsResult(::windows::core::IUnknown);
impl ClipboardHistoryItemsResult {
    pub fn Status(&self) -> ::windows::core::Result<ClipboardHistoryItemsResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ClipboardHistoryItemsResultStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ClipboardHistoryItemsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClipboardHistoryItemsResult {}
impl ::core::fmt::Debug for ClipboardHistoryItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClipboardHistoryItemsResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ClipboardHistoryItemsResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult;{e6dfdee6-0ee2-52e3-852b-f295db65939a})");
}
impl ::core::clone::Clone for ClipboardHistoryItemsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ClipboardHistoryItemsResult {
    type Vtable = IClipboardHistoryItemsResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ClipboardHistoryItemsResult {
    const IID: ::windows::core::GUID = <IClipboardHistoryItemsResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ClipboardHistoryItemsResult {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult";
}
::windows::imp::interface_hierarchy!(ClipboardHistoryItemsResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClipboardHistoryItemsResult {}
unsafe impl ::core::marker::Sync for ClipboardHistoryItemsResult {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataPackage(::windows::core::IUnknown);
impl DataPackage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DataPackage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetView(&self) -> ::windows::core::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageView>();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<DataPackagePropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackagePropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestedOperation(&self) -> ::windows::core::Result<DataPackageOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageOperation>();
            (::windows::core::Interface::vtable(this).RequestedOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestedOperation(&self, value: DataPackageOperation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestedOperation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OperationCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).OperationCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOperationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOperationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Destroyed(&self, handler: &super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Destroyed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDestroyed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDestroyed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetData<P0>(&self, formatid: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(formatid), value.into_param().abi()).ok() }
    }
    pub fn SetDataProvider(&self, formatid: &::windows::core::HSTRING, delayrenderer: &DataProviderHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataProvider)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(formatid), ::core::mem::transmute_copy(delayrenderer)).ok() }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetHtmlFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHtmlFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ResourceMap(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>();
            (::windows::core::Interface::vtable(this).ResourceMap)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRtf(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRtf)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBitmap(&self, value: &super::super::Storage::Streams::RandomAccessStreamReference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmap)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SetStorageItemsReadOnly<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStorageItemsReadOnly)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SetStorageItems<P0>(&self, value: P0, readonly: bool) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStorageItems)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), readonly).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetApplicationLink(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetApplicationLink)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetWebLink(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWebLink)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IDataPackage3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ShareCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShareCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackage3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShareCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareCanceled(&self, handler: &super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IDataPackage4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ShareCanceled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShareCanceled(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackage4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShareCanceled)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for DataPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataPackage {}
impl ::core::fmt::Debug for DataPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPackage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataPackage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackage;{61ebf5c7-efea-4346-9554-981d7e198ffe})");
}
impl ::core::clone::Clone for DataPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataPackage {
    type Vtable = IDataPackage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataPackage {
    const IID: ::windows::core::GUID = <IDataPackage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataPackage {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackage";
}
::windows::imp::interface_hierarchy!(DataPackage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataPackage {}
unsafe impl ::core::marker::Sync for DataPackage {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataPackagePropertySet(::windows::core::IUnknown);
impl DataPackagePropertySet {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnail)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).FileTypes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ApplicationName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetApplicationName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetApplicationName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationListingUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ApplicationListingUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetApplicationListingUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetApplicationListingUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ContentSourceWebLink)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentSourceWebLink(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContentSourceWebLink)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentSourceApplicationLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ContentSourceApplicationLink)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentSourceApplicationLink(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContentSourceApplicationLink)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPackageFamilyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).Square30x30Logo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSquare30x30Logo<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSquare30x30Logo)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn LogoBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Color>();
            (::windows::core::Interface::vtable(this).LogoBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetLogoBackgroundColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLogoBackgroundColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetEnterpriseId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentSourceUserActivityJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentSourceUserActivityJson)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentSourceUserActivityJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySet4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContentSourceUserActivityJson)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Insert)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for DataPackagePropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataPackagePropertySet {}
impl ::core::fmt::Debug for DataPackagePropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPackagePropertySet").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataPackagePropertySet {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackagePropertySet;{cd1c93eb-4c4c-443a-a8d3-f5c241e91689})");
}
impl ::core::clone::Clone for DataPackagePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataPackagePropertySet {
    type Vtable = IDataPackagePropertySet_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataPackagePropertySet {
    const IID: ::windows::core::GUID = <IDataPackagePropertySet as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataPackagePropertySet {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackagePropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for DataPackagePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &DataPackagePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::imp::interface_hierarchy!(DataPackagePropertySet, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for DataPackagePropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for DataPackagePropertySet {}
unsafe impl ::core::marker::Send for DataPackagePropertySet {}
unsafe impl ::core::marker::Sync for DataPackagePropertySet {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataPackagePropertySetView(::windows::core::IUnknown);
impl DataPackagePropertySetView {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::RandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).FileTypes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ApplicationName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationListingUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ApplicationListingUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ContentSourceWebLink)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentSourceApplicationLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ContentSourceApplicationLink)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).Square30x30Logo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn LogoBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Color>();
            (::windows::core::Interface::vtable(this).LogoBackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContentSourceUserActivityJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentSourceUserActivityJson)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFromRoamingClipboard(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IDataPackagePropertySetView5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsFromRoamingClipboard)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, second: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Split)(::windows::core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
impl ::core::cmp::PartialEq for DataPackagePropertySetView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataPackagePropertySetView {}
impl ::core::fmt::Debug for DataPackagePropertySetView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPackagePropertySetView").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataPackagePropertySetView {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView;{b94cec01-0c1a-4c57-be55-75d01289735d})");
}
impl ::core::clone::Clone for DataPackagePropertySetView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataPackagePropertySetView {
    type Vtable = IDataPackagePropertySetView_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataPackagePropertySetView {
    const IID: ::windows::core::GUID = <IDataPackagePropertySetView as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataPackagePropertySetView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for DataPackagePropertySetView {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &DataPackagePropertySetView {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::imp::interface_hierarchy!(DataPackagePropertySetView, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for DataPackagePropertySetView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> for DataPackagePropertySetView {}
unsafe impl ::core::marker::Send for DataPackagePropertySetView {}
unsafe impl ::core::marker::Sync for DataPackagePropertySetView {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataPackageView(::windows::core::IUnknown);
impl DataPackageView {
    pub fn Properties(&self) -> ::windows::core::Result<DataPackagePropertySetView> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackagePropertySetView>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestedOperation(&self) -> ::windows::core::Result<DataPackageOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageOperation>();
            (::windows::core::Interface::vtable(this).RequestedOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportOperationCompleted(&self, value: DataPackageOperation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportOperationCompleted)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).AvailableFormats)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contains(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Contains)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(formatid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDataAsync(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).GetDataAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(formatid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetTextAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCustomTextAsync(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetCustomTextAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(formatid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetUriAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).GetUriAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetHtmlFormatAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetHtmlFormatAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetResourceMapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>>();
            (::windows::core::Interface::vtable(this).GetResourceMapAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRtfAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetRtfAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>();
            (::windows::core::Interface::vtable(this).GetBitmapAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn GetStorageItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>>();
            (::windows::core::Interface::vtable(this).GetStorageItemsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetApplicationLinkAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = &::windows::core::ComInterface::cast::<IDataPackageView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).GetApplicationLinkAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetWebLinkAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = &::windows::core::ComInterface::cast::<IDataPackageView2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>();
            (::windows::core::Interface::vtable(this).GetWebLinkAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_EnterpriseData\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>> {
        let this = &::windows::core::ComInterface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_EnterpriseData\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    pub fn RequestAccessWithEnterpriseIdAsync(&self, enterpriseid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>> {
        let this = &::windows::core::ComInterface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>();
            (::windows::core::Interface::vtable(this).RequestAccessWithEnterpriseIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(enterpriseid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    #[cfg(feature = "Security_EnterpriseData")]
    pub fn UnlockAndAssumeEnterpriseIdentity(&self) -> ::windows::core::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult> {
        let this = &::windows::core::ComInterface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>();
            (::windows::core::Interface::vtable(this).UnlockAndAssumeEnterpriseIdentity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAcceptedFormatId(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataPackageView4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAcceptedFormatId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(formatid)).ok() }
    }
}
impl ::core::cmp::PartialEq for DataPackageView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataPackageView {}
impl ::core::fmt::Debug for DataPackageView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPackageView").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataPackageView {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataPackageView;{7b840471-5900-4d85-a90b-10cb85fe3552})");
}
impl ::core::clone::Clone for DataPackageView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataPackageView {
    type Vtable = IDataPackageView_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataPackageView {
    const IID: ::windows::core::GUID = <IDataPackageView as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackageView";
}
::windows::imp::interface_hierarchy!(DataPackageView, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataPackageView {}
unsafe impl ::core::marker::Sync for DataPackageView {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataProviderDeferral(::windows::core::IUnknown);
impl DataProviderDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for DataProviderDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataProviderDeferral {}
impl ::core::fmt::Debug for DataProviderDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProviderDeferral").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataProviderDeferral {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataProviderDeferral;{c2cf2373-2d26-43d9-b69d-dcb86d03f6da})");
}
impl ::core::clone::Clone for DataProviderDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataProviderDeferral {
    type Vtable = IDataProviderDeferral_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataProviderDeferral {
    const IID: ::windows::core::GUID = <IDataProviderDeferral as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataProviderDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataProviderDeferral";
}
::windows::imp::interface_hierarchy!(DataProviderDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataProviderDeferral {}
unsafe impl ::core::marker::Sync for DataProviderDeferral {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataProviderRequest(::windows::core::IUnknown);
impl DataProviderRequest {
    pub fn FormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FormatId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Deadline)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<DataProviderDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataProviderDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for DataProviderRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataProviderRequest {}
impl ::core::fmt::Debug for DataProviderRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProviderRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataProviderRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataProviderRequest;{ebbc7157-d3c8-47da-acde-f82388d5f716})");
}
impl ::core::clone::Clone for DataProviderRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataProviderRequest {
    type Vtable = IDataProviderRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataProviderRequest {
    const IID: ::windows::core::GUID = <IDataProviderRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataProviderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataProviderRequest";
}
::windows::imp::interface_hierarchy!(DataProviderRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataProviderRequest {}
unsafe impl ::core::marker::Sync for DataProviderRequest {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataRequest(::windows::core::IUnknown);
impl DataRequest {
    pub fn Data(&self) -> ::windows::core::Result<DataPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackage>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &DataPackage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Deadline)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FailWithDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).FailWithDisplayText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<DataRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataRequestDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DataRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataRequest {}
impl ::core::fmt::Debug for DataRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataRequest;{4341ae3b-fc12-4e53-8c02-ac714c415a27})");
}
impl ::core::clone::Clone for DataRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataRequest {
    type Vtable = IDataRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataRequest {
    const IID: ::windows::core::GUID = <IDataRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataRequest";
}
::windows::imp::interface_hierarchy!(DataRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataRequest {}
unsafe impl ::core::marker::Sync for DataRequest {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataRequestDeferral(::windows::core::IUnknown);
impl DataRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for DataRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataRequestDeferral {}
impl ::core::fmt::Debug for DataRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataRequestDeferral").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataRequestDeferral {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataRequestDeferral;{6dc4b89f-0386-4263-87c1-ed7dce30890e})");
}
impl ::core::clone::Clone for DataRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataRequestDeferral {
    type Vtable = IDataRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataRequestDeferral {
    const IID: ::windows::core::GUID = <IDataRequestDeferral as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataRequestDeferral";
}
::windows::imp::interface_hierarchy!(DataRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataRequestDeferral {}
unsafe impl ::core::marker::Sync for DataRequestDeferral {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataRequestedEventArgs(::windows::core::IUnknown);
impl DataRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<DataRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataRequest>();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DataRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataRequestedEventArgs {}
impl ::core::fmt::Debug for DataRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs;{cb8ba807-6ac5-43c9-8ac5-9ba232163182})");
}
impl ::core::clone::Clone for DataRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataRequestedEventArgs {
    type Vtable = IDataRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataRequestedEventArgs {
    const IID: ::windows::core::GUID = <IDataRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(DataRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DataRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataTransferManager(::windows::core::IUnknown);
impl DataTransferManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataRequested(&self, handler: &super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).DataRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetApplicationChosen(&self, handler: &super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TargetApplicationChosen)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetApplicationChosen(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTargetApplicationChosen)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShareProvidersRequested(&self, handler: &super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IDataTransferManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ShareProvidersRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShareProvidersRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDataTransferManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShareProvidersRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn ShowShareUI() -> ::windows::core::Result<()> {
        Self::IDataTransferManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowShareUI)(::windows::core::Interface::as_raw(this)).ok() })
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<DataTransferManager> {
        Self::IDataTransferManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DataTransferManager>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IDataTransferManagerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowShareUIWithOptions(options: &ShareUIOptions) -> ::windows::core::Result<()> {
        Self::IDataTransferManagerStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).ShowShareUIWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options)).ok() })
    }
    #[doc(hidden)]
    pub fn IDataTransferManagerStatics<R, F: FnOnce(&IDataTransferManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DataTransferManager, IDataTransferManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDataTransferManagerStatics2<R, F: FnOnce(&IDataTransferManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DataTransferManager, IDataTransferManagerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDataTransferManagerStatics3<R, F: FnOnce(&IDataTransferManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DataTransferManager, IDataTransferManagerStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DataTransferManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataTransferManager {}
impl ::core::fmt::Debug for DataTransferManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataTransferManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DataTransferManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.DataTransferManager;{a5caee9b-8708-49d1-8d36-67d25a8da00c})");
}
impl ::core::clone::Clone for DataTransferManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DataTransferManager {
    type Vtable = IDataTransferManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DataTransferManager {
    const IID: ::windows::core::GUID = <IDataTransferManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DataTransferManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataTransferManager";
}
::windows::imp::interface_hierarchy!(DataTransferManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
pub struct HtmlFormatHelper;
impl HtmlFormatHelper {
    pub fn GetStaticFragment(htmlformat: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHtmlFormatHelperStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetStaticFragment)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(htmlformat), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateHtmlFormat(htmlfragment: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHtmlFormatHelperStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CreateHtmlFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(htmlfragment), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHtmlFormatHelperStatics<R, F: FnOnce(&IHtmlFormatHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HtmlFormatHelper, IHtmlFormatHelperStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for HtmlFormatHelper {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.HtmlFormatHelper";
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct OperationCompletedEventArgs(::windows::core::IUnknown);
impl OperationCompletedEventArgs {
    pub fn Operation(&self) -> ::windows::core::Result<DataPackageOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AcceptedFormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IOperationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AcceptedFormatId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OperationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OperationCompletedEventArgs {}
impl ::core::fmt::Debug for OperationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OperationCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OperationCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs;{e7af329d-051d-4fab-b1a9-47fd77f70a41})");
}
impl ::core::clone::Clone for OperationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OperationCompletedEventArgs {
    type Vtable = IOperationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OperationCompletedEventArgs {
    const IID: ::windows::core::GUID = <IOperationCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OperationCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(OperationCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OperationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for OperationCompletedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareCompletedEventArgs(::windows::core::IUnknown);
impl ShareCompletedEventArgs {
    pub fn ShareTarget(&self) -> ::windows::core::Result<ShareTargetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ShareTargetInfo>();
            (::windows::core::Interface::vtable(this).ShareTarget)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ShareCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareCompletedEventArgs {}
impl ::core::fmt::Debug for ShareCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs;{4574c442-f913-4f60-9df7-cc4060ab1916})");
}
impl ::core::clone::Clone for ShareCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareCompletedEventArgs {
    type Vtable = IShareCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareCompletedEventArgs {
    const IID: ::windows::core::GUID = <IShareCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(ShareCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareCompletedEventArgs {}
unsafe impl ::core::marker::Sync for ShareCompletedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareProvider(::windows::core::IUnknown);
impl ShareProvider {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DisplayIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::RandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).DisplayIcon)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Color>();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "UI"))]
    pub fn Create(title: &::windows::core::HSTRING, displayicon: &super::super::Storage::Streams::RandomAccessStreamReference, backgroundcolor: super::super::UI::Color, handler: &ShareProviderHandler) -> ::windows::core::Result<ShareProvider> {
        Self::IShareProviderFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ShareProvider>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(displayicon), backgroundcolor, ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IShareProviderFactory<R, F: FnOnce(&IShareProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ShareProvider, IShareProviderFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ShareProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareProvider {}
impl ::core::fmt::Debug for ShareProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareProvider;{2fabe026-443e-4cda-af25-8d81070efd80})");
}
impl ::core::clone::Clone for ShareProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareProvider {
    type Vtable = IShareProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareProvider {
    const IID: ::windows::core::GUID = <IShareProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareProvider {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareProvider";
}
::windows::imp::interface_hierarchy!(ShareProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareProvider {}
unsafe impl ::core::marker::Sync for ShareProvider {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareProviderOperation(::windows::core::IUnknown);
impl ShareProviderOperation {
    pub fn Data(&self) -> ::windows::core::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageView>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Provider(&self) -> ::windows::core::Result<ShareProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ShareProvider>();
            (::windows::core::Interface::vtable(this).Provider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for ShareProviderOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareProviderOperation {}
impl ::core::fmt::Debug for ShareProviderOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareProviderOperation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareProviderOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareProviderOperation;{19cef937-d435-4179-b6af-14e0492b69f6})");
}
impl ::core::clone::Clone for ShareProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareProviderOperation {
    type Vtable = IShareProviderOperation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareProviderOperation {
    const IID: ::windows::core::GUID = <IShareProviderOperation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareProviderOperation";
}
::windows::imp::interface_hierarchy!(ShareProviderOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareProviderOperation {}
unsafe impl ::core::marker::Sync for ShareProviderOperation {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareProvidersRequestedEventArgs(::windows::core::IUnknown);
impl ShareProvidersRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Providers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ShareProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<ShareProvider>>();
            (::windows::core::Interface::vtable(this).Providers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DataPackageView>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ShareProvidersRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareProvidersRequestedEventArgs {}
impl ::core::fmt::Debug for ShareProvidersRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareProvidersRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareProvidersRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs;{f888f356-a3f8-4fce-85e4-8826e63be799})");
}
impl ::core::clone::Clone for ShareProvidersRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareProvidersRequestedEventArgs {
    type Vtable = IShareProvidersRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareProvidersRequestedEventArgs {
    const IID: ::windows::core::GUID = <IShareProvidersRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareProvidersRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(ShareProvidersRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareProvidersRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ShareProvidersRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareTargetInfo(::windows::core::IUnknown);
impl ShareTargetInfo {
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShareProvider(&self) -> ::windows::core::Result<ShareProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ShareProvider>();
            (::windows::core::Interface::vtable(this).ShareProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ShareTargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareTargetInfo {}
impl ::core::fmt::Debug for ShareTargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareTargetInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareTargetInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareTargetInfo;{385be607-c6e8-4114-b294-28f3bb6f9904})");
}
impl ::core::clone::Clone for ShareTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareTargetInfo {
    type Vtable = IShareTargetInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareTargetInfo {
    const IID: ::windows::core::GUID = <IShareTargetInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareTargetInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareTargetInfo";
}
::windows::imp::interface_hierarchy!(ShareTargetInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareTargetInfo {}
unsafe impl ::core::marker::Sync for ShareTargetInfo {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareUIOptions(::windows::core::IUnknown);
impl ShareUIOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ShareUIOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Theme(&self) -> ::windows::core::Result<ShareUITheme> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ShareUITheme>();
            (::windows::core::Interface::vtable(this).Theme)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTheme(&self, value: ShareUITheme) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTheme)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionRect(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::Rect>>();
            (::windows::core::Interface::vtable(this).SelectionRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectionRect<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::Rect>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSelectionRect)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for ShareUIOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareUIOptions {}
impl ::core::fmt::Debug for ShareUIOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareUIOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareUIOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.ShareUIOptions;{72fa8a80-342f-4d90-9551-2ae04e37680c})");
}
impl ::core::clone::Clone for ShareUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareUIOptions {
    type Vtable = IShareUIOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareUIOptions {
    const IID: ::windows::core::GUID = <IShareUIOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareUIOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareUIOptions";
}
::windows::imp::interface_hierarchy!(ShareUIOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ShareUIOptions {}
unsafe impl ::core::marker::Sync for ShareUIOptions {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
pub struct SharedStorageAccessManager;
impl SharedStorageAccessManager {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn AddFile<P0>(file: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::ISharedStorageAccessManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AddFile)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn RedeemTokenForFileAsync(token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        Self::ISharedStorageAccessManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).RedeemTokenForFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveFile(token: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::ISharedStorageAccessManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveFile)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(token)).ok() })
    }
    #[doc(hidden)]
    pub fn ISharedStorageAccessManagerStatics<R, F: FnOnce(&ISharedStorageAccessManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SharedStorageAccessManager, ISharedStorageAccessManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SharedStorageAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager";
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
pub struct StandardDataFormats;
impl StandardDataFormats {
    pub fn Text() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Uri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Html() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Html)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Rtf() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Rtf)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Bitmap() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Bitmap)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn StorageItems() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).StorageItems)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn WebLink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WebLink)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ApplicationLink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ApplicationLink)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UserActivityJsonArray() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardDataFormatsStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserActivityJsonArray)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStandardDataFormatsStatics<R, F: FnOnce(&IStandardDataFormatsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StandardDataFormats, IStandardDataFormatsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardDataFormatsStatics2<R, F: FnOnce(&IStandardDataFormatsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StandardDataFormats, IStandardDataFormatsStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardDataFormatsStatics3<R, F: FnOnce(&IStandardDataFormatsStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StandardDataFormats, IStandardDataFormatsStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StandardDataFormats {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.StandardDataFormats";
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct TargetApplicationChosenEventArgs(::windows::core::IUnknown);
impl TargetApplicationChosenEventArgs {
    pub fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ApplicationName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TargetApplicationChosenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetApplicationChosenEventArgs {}
impl ::core::fmt::Debug for TargetApplicationChosenEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetApplicationChosenEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TargetApplicationChosenEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs;{ca6fb8ac-2987-4ee3-9c54-d8afbcb86c1d})");
}
impl ::core::clone::Clone for TargetApplicationChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TargetApplicationChosenEventArgs {
    type Vtable = ITargetApplicationChosenEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TargetApplicationChosenEventArgs {
    const IID: ::windows::core::GUID = <ITargetApplicationChosenEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TargetApplicationChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs";
}
::windows::imp::interface_hierarchy!(TargetApplicationChosenEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TargetApplicationChosenEventArgs {}
unsafe impl ::core::marker::Sync for TargetApplicationChosenEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ClipboardHistoryItemsResultStatus(pub i32);
impl ClipboardHistoryItemsResultStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ClipboardHistoryDisabled: Self = Self(2i32);
}
impl ::core::marker::Copy for ClipboardHistoryItemsResultStatus {}
impl ::core::clone::Clone for ClipboardHistoryItemsResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClipboardHistoryItemsResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ClipboardHistoryItemsResultStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ClipboardHistoryItemsResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClipboardHistoryItemsResultStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ClipboardHistoryItemsResultStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResultStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: Self = Self(0u32);
    pub const Copy: Self = Self(1u32);
    pub const Move: Self = Self(2u32);
    pub const Link: Self = Self(4u32);
}
impl ::core::marker::Copy for DataPackageOperation {}
impl ::core::clone::Clone for DataPackageOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataPackageOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DataPackageOperation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DataPackageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPackageOperation").field(&self.0).finish()
    }
}
impl DataPackageOperation {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DataPackageOperation {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DataPackageOperation {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DataPackageOperation {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DataPackageOperation {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DataPackageOperation {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for DataPackageOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DataPackageOperation;u4)");
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SetHistoryItemAsContentStatus(pub i32);
impl SetHistoryItemAsContentStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ItemDeleted: Self = Self(2i32);
}
impl ::core::marker::Copy for SetHistoryItemAsContentStatus {}
impl ::core::clone::Clone for SetHistoryItemAsContentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetHistoryItemAsContentStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SetHistoryItemAsContentStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SetHistoryItemAsContentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetHistoryItemAsContentStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SetHistoryItemAsContentStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.SetHistoryItemAsContentStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ShareUITheme(pub i32);
impl ShareUITheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareUITheme {}
impl ::core::clone::Clone for ShareUITheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShareUITheme {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ShareUITheme {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ShareUITheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareUITheme").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareUITheme {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.ShareUITheme;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataProviderHandler(pub ::windows::core::IUnknown);
impl DataProviderHandler {
    pub fn new<F: FnMut(::core::option::Option<&DataProviderRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DataProviderHandlerBox::<F> { vtable: &DataProviderHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, request: &DataProviderRequest) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request)).ok() }
    }
}
#[repr(C)]
struct DataProviderHandlerBox<F: FnMut(::core::option::Option<&DataProviderRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DataProviderHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&DataProviderRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DataProviderHandlerBox<F> {
    const VTABLE: DataProviderHandler_Vtbl = DataProviderHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DataProviderHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&request)).into()
    }
}
impl ::core::cmp::PartialEq for DataProviderHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataProviderHandler {}
impl ::core::fmt::Debug for DataProviderHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProviderHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DataProviderHandler {
    type Vtable = DataProviderHandler_Vtbl;
}
impl ::core::clone::Clone for DataProviderHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for DataProviderHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ecd720_f2f4_4a2d_920e_170a2f482a27);
}
impl ::windows::core::RuntimeType for DataProviderHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e7ecd720-f2f4-4a2d-920e-170a2f482a27}");
}
#[repr(C)]
#[doc(hidden)]
pub struct DataProviderHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareProviderHandler(pub ::windows::core::IUnknown);
impl ShareProviderHandler {
    pub fn new<F: FnMut(::core::option::Option<&ShareProviderOperation>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ShareProviderHandlerBox::<F> { vtable: &ShareProviderHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, operation: &ShareProviderOperation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(operation)).ok() }
    }
}
#[repr(C)]
struct ShareProviderHandlerBox<F: FnMut(::core::option::Option<&ShareProviderOperation>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ShareProviderHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&ShareProviderOperation>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ShareProviderHandlerBox<F> {
    const VTABLE: ShareProviderHandler_Vtbl = ShareProviderHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<ShareProviderHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, operation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&operation)).into()
    }
}
impl ::core::cmp::PartialEq for ShareProviderHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareProviderHandler {}
impl ::core::fmt::Debug for ShareProviderHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareProviderHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ShareProviderHandler {
    type Vtable = ShareProviderHandler_Vtbl;
}
impl ::core::clone::Clone for ShareProviderHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ShareProviderHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7f9d9ba_e1ba_4e4d_bd65_d43845d3212f);
}
impl ::windows::core::RuntimeType for ShareProviderHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e7f9d9ba-e1ba-4e4d-bd65-d43845d3212f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ShareProviderHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
