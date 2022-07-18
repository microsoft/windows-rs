#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f2264e9_6f3a_4166_a018_0a0b06bb32b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAddress_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddressLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddressLines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAddressLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAddressLines: usize,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DependentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDependentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SortingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSortingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LanguageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLanguageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Organization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetOrganization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7696fe55_d5d3_4d3d_b345_45591759c510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResultFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentCanMakePaymentResultFactory {
    type Vtable = IPaymentCanMakePaymentResultFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbdcaa3e_7d49_4f69_aa53_2a0f8164b7c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResultFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentCanMakePaymentResultStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCurrencyAmount(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3a3e9e0_b41f_4987_bdcb_071331f2daa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmount_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCurrencyAmountFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentCurrencyAmountFactory {
    type Vtable = IPaymentCurrencyAmountFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3257d338_140c_4575_8535_f773178c09a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmountFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithCurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currencysystem: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53bb2d7d_e0eb_4053_8eae_ce7c48e02945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTotal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShippingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShippingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Modifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentDetailsFactory {
    type Vtable = IPaymentDetailsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe8afee_c0ea_4ca1_8bc7_6de67b1f3763);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, displayitems: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithDisplayItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsModifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1c7d65_4323_41d7_b305_dfcb765f69de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifier_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub JsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalDisplayItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsModifierFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentDetailsModifierFactory {
    type Vtable = IPaymentDetailsModifierFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79005286_54de_429c_9e4f_5dce6e10ebce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifierFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, additionaldisplayitems: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItemsAndJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, additionaldisplayitems: *mut ::core::ffi::c_void, jsondata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItemsAndJsonData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentItem {
    type Vtable = IPaymentItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x685ac88b_79b2_4b76_9e03_a876223dfe72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItem_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentItemFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentItemFactory {
    type Vtable = IPaymentItemFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6ab7ad8_2503_4d1d_a778_02b2e5927b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItemFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMediator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb0ee829_ec0c_449a_83da_7ae3073365a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedMethodIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedMethodIdsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestWithChangeHandlerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestWithChangeHandlerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMediator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentMediator2 {
    type Vtable = IPaymentMediator2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceef98f1_e407_4128_8e73_d93d5f822786);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CanMakePaymentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanMakePaymentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMerchantInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63445050_0e94_4ed6_aacb_e6012bd327a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMerchantInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentMerchantInfoFactory {
    type Vtable = IPaymentMerchantInfoFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e89ced3_ccb7_4167_a8ec_e10ae96dbcd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfoFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMethodData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1d3caf4_de98_4129_b1b7_c3ad86237bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodData_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub JsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMethodDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentMethodDataFactory {
    type Vtable = IPaymentMethodDataFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8addd27f_9baa_4a82_8342_a8210992a36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, jsondata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithJsonData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa30854_1f2b_4365_8251_01b58915a5bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentOptions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RequestPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows::core::HRESULT,
    pub SetRequestPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows::core::HRESULT,
    pub RequestPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows::core::HRESULT,
    pub SetRequestPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows::core::HRESULT,
    pub RequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows::core::HRESULT,
    pub SetRequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows::core::HRESULT,
    pub RequestShipping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequestShipping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShippingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentShippingType) -> ::windows::core::HRESULT,
    pub SetShippingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentShippingType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb74942e1_ed7b_47eb_bc08_78cc5d6896b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MerchantInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MethodData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MethodData: usize,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequest2 {
    type Vtable = IPaymentRequest2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb63ccfb5_5998_493e_a04c_67048a50f141);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6145e44_cd8b_4be4_b555_27c99194c0c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ChangeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestChangeKind) -> ::windows::core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectedShippingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeresult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf699e5c_16c4_47ad_9401_8440ec0757db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UpdatedPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetUpdatedPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedResultFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequestChangedResultFactory {
    type Vtable = IPaymentRequestChangedResultFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08740f56_1d33_4431_814b_67ea24bf21db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResultFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, updatedpaymentdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequestFactory {
    type Vtable = IPaymentRequestFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e8a79dc_6b74_42d3_b103_f0de35fb1848);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequestFactory2 {
    type Vtable = IPaymentRequestFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6ce1325_a506_4372_b7ef_1a031d5662d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoOptionsAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoOptionsAndId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestSubmitResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b9c3912_30f2_4e90_b249_8ce7d78ffe56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestSubmitResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestStatus) -> ::windows::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1389457_8bd2_4888_9fa8_97985545108e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentResponse_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PaymentToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShippingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PaymentRequestCompletionStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentShippingOption(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13372ada_9753_4574_8966_93145a76c7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOption_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentShippingOptionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentShippingOptionFactory {
    type Vtable = IPaymentShippingOptionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5de5f917_b2d7_446b_9d73_6123fbca3bc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOptionFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: *mut ::core::ffi::c_void, selected: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithSelectedAndTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: *mut ::core::ffi::c_void, selected: bool, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentToken(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentToken {
    type Vtable = IPaymentToken_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbcac013_ccd0_41f2_b2a1_0a2e4b5dce25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentToken_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PaymentMethodId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JsonDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTokenFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentTokenFactory {
    type Vtable = IPaymentTokenFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x988cd7aa_4753_4904_8373_dd7b08b995c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTokenFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentmethodid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithJsonDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentmethodid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, jsondetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentAddress(::windows::core::IUnknown);
impl PaymentAddress {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentAddress, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Country)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCountry)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddressLines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AddressLines)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAddressLines<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddressLines)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Region)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRegion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).City)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DependentLocality(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DependentLocality)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDependentLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDependentLocality)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PostalCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPostalCode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SortingCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SortingCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSortingCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSortingCode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LanguageCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LanguageCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguageCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguageCode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Organization(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Organization)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetOrganization(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOrganization)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Recipient(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Recipient)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRecipient(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecipient)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPhoneNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPhoneNumber)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAddress {}
impl ::core::fmt::Debug for PaymentAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentAddress;{5f2264e9-6f3a-4166-a018-0a0b06bb32b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentAddress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentAddress";
}
impl ::core::convert::From<PaymentAddress> for ::windows::core::IUnknown {
    fn from(value: PaymentAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAddress> for ::windows::core::IUnknown {
    fn from(value: &PaymentAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentAddress> for &::windows::core::IUnknown {
    fn from(value: &PaymentAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentAddress> for ::windows::core::IInspectable {
    fn from(value: PaymentAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAddress> for ::windows::core::IInspectable {
    fn from(value: &PaymentAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentAddress> for &::windows::core::IInspectable {
    fn from(value: &PaymentAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentAddress {}
unsafe impl ::core::marker::Sync for PaymentAddress {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentCanMakePaymentResult(::windows::core::IUnknown);
impl PaymentCanMakePaymentResult {
    pub fn Status(&self) -> ::windows::core::Result<PaymentCanMakePaymentResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentCanMakePaymentResultStatus>(result__)
        }
    }
    pub fn Create(value: PaymentCanMakePaymentResultStatus) -> ::windows::core::Result<PaymentCanMakePaymentResult> {
        Self::IPaymentCanMakePaymentResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<PaymentCanMakePaymentResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentCanMakePaymentResultFactory<R, F: FnOnce(&IPaymentCanMakePaymentResultFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentCanMakePaymentResult, IPaymentCanMakePaymentResultFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentCanMakePaymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentCanMakePaymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentCanMakePaymentResult {}
impl ::core::fmt::Debug for PaymentCanMakePaymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCanMakePaymentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentCanMakePaymentResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult;{7696fe55-d5d3-4d3d-b345-45591759c510})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentCanMakePaymentResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentCanMakePaymentResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
}
impl ::core::convert::From<PaymentCanMakePaymentResult> for ::windows::core::IUnknown {
    fn from(value: PaymentCanMakePaymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCanMakePaymentResult> for ::windows::core::IUnknown {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentCanMakePaymentResult> for &::windows::core::IUnknown {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentCanMakePaymentResult> for ::windows::core::IInspectable {
    fn from(value: PaymentCanMakePaymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCanMakePaymentResult> for ::windows::core::IInspectable {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentCanMakePaymentResult> for &::windows::core::IInspectable {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentCanMakePaymentResult {}
unsafe impl ::core::marker::Sync for PaymentCanMakePaymentResult {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PaymentCanMakePaymentResultStatus(pub i32);
impl PaymentCanMakePaymentResultStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Yes: Self = Self(1i32);
    pub const No: Self = Self(2i32);
    pub const NotAllowed: Self = Self(3i32);
    pub const UserNotSignedIn: Self = Self(4i32);
    pub const SpecifiedPaymentMethodIdsNotSupported: Self = Self(5i32);
    pub const NoQualifyingCardOnFile: Self = Self(6i32);
}
impl ::core::marker::Copy for PaymentCanMakePaymentResultStatus {}
impl ::core::clone::Clone for PaymentCanMakePaymentResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentCanMakePaymentResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PaymentCanMakePaymentResultStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentCanMakePaymentResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCanMakePaymentResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentCanMakePaymentResultStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentCurrencyAmount(::windows::core::IUnknown);
impl PaymentCurrencyAmount {
    pub fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Currency)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCurrency(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCurrency)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CurrencySystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrencySystem)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCurrencySystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCurrencySystem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), ::core::mem::transmute_copy(currency), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        })
    }
    pub fn CreateWithCurrencySystem(value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING, currencysystem: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithCurrencySystem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), ::core::mem::transmute_copy(currency), ::core::mem::transmute_copy(currencysystem), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentCurrencyAmountFactory<R, F: FnOnce(&IPaymentCurrencyAmountFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentCurrencyAmount, IPaymentCurrencyAmountFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentCurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentCurrencyAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentCurrencyAmount {}
impl ::core::fmt::Debug for PaymentCurrencyAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCurrencyAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentCurrencyAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCurrencyAmount;{e3a3e9e0-b41f-4987-bdcb-071331f2daa4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentCurrencyAmount as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentCurrencyAmount {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
}
impl ::core::convert::From<PaymentCurrencyAmount> for ::windows::core::IUnknown {
    fn from(value: PaymentCurrencyAmount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCurrencyAmount> for ::windows::core::IUnknown {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentCurrencyAmount> for &::windows::core::IUnknown {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentCurrencyAmount> for ::windows::core::IInspectable {
    fn from(value: PaymentCurrencyAmount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCurrencyAmount> for ::windows::core::IInspectable {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentCurrencyAmount> for &::windows::core::IInspectable {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentCurrencyAmount {}
unsafe impl ::core::marker::Sync for PaymentCurrencyAmount {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentDetails(::windows::core::IUnknown);
impl PaymentDetails {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentDetails, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Total(&self) -> ::windows::core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Total)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentItem>(result__)
        }
    }
    pub fn SetTotal<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentItem>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTotal)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayItems)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDisplayItems<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IVectorView<PaymentItem>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayItems)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShippingOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShippingOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShippingOptions<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShippingOptions)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Modifiers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Modifiers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetModifiers<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModifiers)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Create<'a, P0>(total: P0) -> ::windows::core::Result<PaymentDetails>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentItem>>,
    {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), total.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithDisplayItems<'a, P0, P1, E1>(total: P0, displayitems: P1) -> ::windows::core::Result<PaymentDetails>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentItem>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithDisplayItems)(::windows::core::Interface::as_raw(this), total.into().abi(), displayitems.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentDetailsFactory<R, F: FnOnce(&IPaymentDetailsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentDetails, IPaymentDetailsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentDetails {}
impl ::core::fmt::Debug for PaymentDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetails;{53bb2d7d-e0eb-4053-8eae-ce7c48e02945})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetails";
}
impl ::core::convert::From<PaymentDetails> for ::windows::core::IUnknown {
    fn from(value: PaymentDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetails> for ::windows::core::IUnknown {
    fn from(value: &PaymentDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentDetails> for &::windows::core::IUnknown {
    fn from(value: &PaymentDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentDetails> for ::windows::core::IInspectable {
    fn from(value: PaymentDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetails> for ::windows::core::IInspectable {
    fn from(value: &PaymentDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentDetails> for &::windows::core::IInspectable {
    fn from(value: &PaymentDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentDetails {}
unsafe impl ::core::marker::Sync for PaymentDetails {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentDetailsModifier(::windows::core::IUnknown);
impl PaymentDetailsModifier {
    pub fn JsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).JsonData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedMethodIds)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Total(&self) -> ::windows::core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Total)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentItem>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AdditionalDisplayItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AdditionalDisplayItems)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, P0, E0, P1>(supportedmethodids: P0, total: P1) -> ::windows::core::Result<PaymentDetailsModifier>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PaymentItem>>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), supportedmethodids.try_into().map_err(|e| e.into())?.abi(), total.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItems<'a, P0, E0, P1, P2, E2>(supportedmethodids: P0, total: P1, additionaldisplayitems: P2) -> ::windows::core::Result<PaymentDetailsModifier>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PaymentItem>>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithAdditionalDisplayItems)(::windows::core::Interface::as_raw(this), supportedmethodids.try_into().map_err(|e| e.into())?.abi(), total.into().abi(), additionaldisplayitems.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItemsAndJsonData<'a, P0, E0, P1, P2, E2>(supportedmethodids: P0, total: P1, additionaldisplayitems: P2, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentDetailsModifier>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PaymentItem>>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithAdditionalDisplayItemsAndJsonData)(::windows::core::Interface::as_raw(this), supportedmethodids.try_into().map_err(|e| e.into())?.abi(), total.into().abi(), additionaldisplayitems.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(jsondata), result__.as_mut_ptr()).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentDetailsModifierFactory<R, F: FnOnce(&IPaymentDetailsModifierFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentDetailsModifier, IPaymentDetailsModifierFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentDetailsModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentDetailsModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentDetailsModifier {}
impl ::core::fmt::Debug for PaymentDetailsModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentDetailsModifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentDetailsModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetailsModifier;{be1c7d65-4323-41d7-b305-dfcb765f69de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentDetailsModifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentDetailsModifier {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetailsModifier";
}
impl ::core::convert::From<PaymentDetailsModifier> for ::windows::core::IUnknown {
    fn from(value: PaymentDetailsModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetailsModifier> for ::windows::core::IUnknown {
    fn from(value: &PaymentDetailsModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentDetailsModifier> for &::windows::core::IUnknown {
    fn from(value: &PaymentDetailsModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentDetailsModifier> for ::windows::core::IInspectable {
    fn from(value: PaymentDetailsModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetailsModifier> for ::windows::core::IInspectable {
    fn from(value: &PaymentDetailsModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentDetailsModifier> for &::windows::core::IInspectable {
    fn from(value: &PaymentDetailsModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentDetailsModifier {}
unsafe impl ::core::marker::Sync for PaymentDetailsModifier {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentItem(::windows::core::IUnknown);
impl PaymentItem {
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Amount(&self) -> ::windows::core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Amount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        }
    }
    pub fn SetAmount<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentCurrencyAmount>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAmount)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Pending(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Pending)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPending(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPending)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, P0>(label: &::windows::core::HSTRING, amount: P0) -> ::windows::core::Result<PaymentItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentCurrencyAmount>>,
    {
        Self::IPaymentItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentItem>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentItemFactory<R, F: FnOnce(&IPaymentItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentItem, IPaymentItemFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentItem {}
impl ::core::fmt::Debug for PaymentItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentItem;{685ac88b-79b2-4b76-9e03-a876223dfe72})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentItem {
    type Vtable = IPaymentItem_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentItem {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentItem";
}
impl ::core::convert::From<PaymentItem> for ::windows::core::IUnknown {
    fn from(value: PaymentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentItem> for ::windows::core::IUnknown {
    fn from(value: &PaymentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentItem> for &::windows::core::IUnknown {
    fn from(value: &PaymentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentItem> for ::windows::core::IInspectable {
    fn from(value: PaymentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentItem> for ::windows::core::IInspectable {
    fn from(value: &PaymentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentItem> for &::windows::core::IInspectable {
    fn from(value: &PaymentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentItem {}
unsafe impl ::core::marker::Sync for PaymentItem {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentMediator(::windows::core::IUnknown);
impl PaymentMediator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentMediator, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedMethodIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedMethodIdsAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SubmitPaymentRequestAsync<'a, P0>(&self, paymentrequest: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubmitPaymentRequestAsync)(::windows::core::Interface::as_raw(this), paymentrequest.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SubmitPaymentRequestWithChangeHandlerAsync<'a, P0, P1>(&self, paymentrequest: P0, changehandler: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequestChangedHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubmitPaymentRequestWithChangeHandlerAsync)(::windows::core::Interface::as_raw(this), paymentrequest.into().abi(), changehandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanMakePaymentAsync<'a, P0>(&self, paymentrequest: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequest>>,
    {
        let this = &::windows::core::Interface::cast::<IPaymentMediator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanMakePaymentAsync)(::windows::core::Interface::as_raw(this), paymentrequest.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentMediator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentMediator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentMediator {}
impl ::core::fmt::Debug for PaymentMediator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentMediator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentMediator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMediator;{fb0ee829-ec0c-449a-83da-7ae3073365a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentMediator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentMediator {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMediator";
}
impl ::core::convert::From<PaymentMediator> for ::windows::core::IUnknown {
    fn from(value: PaymentMediator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMediator> for ::windows::core::IUnknown {
    fn from(value: &PaymentMediator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentMediator> for &::windows::core::IUnknown {
    fn from(value: &PaymentMediator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentMediator> for ::windows::core::IInspectable {
    fn from(value: PaymentMediator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMediator> for ::windows::core::IInspectable {
    fn from(value: &PaymentMediator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentMediator> for &::windows::core::IInspectable {
    fn from(value: &PaymentMediator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentMediator {}
unsafe impl ::core::marker::Sync for PaymentMediator {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentMerchantInfo(::windows::core::IUnknown);
impl PaymentMerchantInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentMerchantInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PackageFullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PackageFullName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, P0>(uri: P0) -> ::windows::core::Result<PaymentMerchantInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        Self::IPaymentMerchantInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), uri.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentMerchantInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentMerchantInfoFactory<R, F: FnOnce(&IPaymentMerchantInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentMerchantInfo, IPaymentMerchantInfoFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentMerchantInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentMerchantInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentMerchantInfo {}
impl ::core::fmt::Debug for PaymentMerchantInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentMerchantInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentMerchantInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMerchantInfo;{63445050-0e94-4ed6-aacb-e6012bd327a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentMerchantInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentMerchantInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMerchantInfo";
}
impl ::core::convert::From<PaymentMerchantInfo> for ::windows::core::IUnknown {
    fn from(value: PaymentMerchantInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMerchantInfo> for ::windows::core::IUnknown {
    fn from(value: &PaymentMerchantInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentMerchantInfo> for &::windows::core::IUnknown {
    fn from(value: &PaymentMerchantInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentMerchantInfo> for ::windows::core::IInspectable {
    fn from(value: PaymentMerchantInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMerchantInfo> for ::windows::core::IInspectable {
    fn from(value: &PaymentMerchantInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentMerchantInfo> for &::windows::core::IInspectable {
    fn from(value: &PaymentMerchantInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentMerchantInfo {}
unsafe impl ::core::marker::Sync for PaymentMerchantInfo {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentMethodData(::windows::core::IUnknown);
impl PaymentMethodData {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedMethodIds)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn JsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).JsonData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, P0, E0>(supportedmethodids: P0) -> ::windows::core::Result<PaymentMethodData>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), supportedmethodids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<PaymentMethodData>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithJsonData<'a, P0, E0>(supportedmethodids: P0, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentMethodData>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithJsonData)(::windows::core::Interface::as_raw(this), supportedmethodids.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(jsondata), result__.as_mut_ptr()).from_abi::<PaymentMethodData>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentMethodDataFactory<R, F: FnOnce(&IPaymentMethodDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentMethodData, IPaymentMethodDataFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentMethodData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentMethodData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentMethodData {}
impl ::core::fmt::Debug for PaymentMethodData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentMethodData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentMethodData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMethodData;{d1d3caf4-de98-4129-b1b7-c3ad86237bf4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentMethodData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentMethodData {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMethodData";
}
impl ::core::convert::From<PaymentMethodData> for ::windows::core::IUnknown {
    fn from(value: PaymentMethodData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMethodData> for ::windows::core::IUnknown {
    fn from(value: &PaymentMethodData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentMethodData> for &::windows::core::IUnknown {
    fn from(value: &PaymentMethodData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentMethodData> for ::windows::core::IInspectable {
    fn from(value: PaymentMethodData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMethodData> for ::windows::core::IInspectable {
    fn from(value: &PaymentMethodData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentMethodData> for &::windows::core::IInspectable {
    fn from(value: &PaymentMethodData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentMethodData {}
unsafe impl ::core::marker::Sync for PaymentMethodData {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PaymentOptionPresence(pub i32);
impl PaymentOptionPresence {
    pub const None: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentOptionPresence {}
impl ::core::clone::Clone for PaymentOptionPresence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentOptionPresence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PaymentOptionPresence {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentOptionPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentOptionPresence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentOptionPresence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentOptionPresence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentOptions(::windows::core::IUnknown);
impl PaymentOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestPayerEmail(&self) -> ::windows::core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestPayerEmail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    pub fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestPayerEmail)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerName(&self) -> ::windows::core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestPayerName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    pub fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestPayerName)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerPhoneNumber(&self) -> ::windows::core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestPayerPhoneNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    pub fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestPayerPhoneNumber)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestShipping(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestShipping)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequestShipping(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestShipping)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShippingType(&self) -> ::windows::core::Result<PaymentShippingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShippingType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentShippingType>(result__)
        }
    }
    pub fn SetShippingType(&self, value: PaymentShippingType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShippingType)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PaymentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentOptions {}
impl ::core::fmt::Debug for PaymentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentOptions;{aaa30854-1f2b-4365-8251-01b58915a5bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentOptions";
}
impl ::core::convert::From<PaymentOptions> for ::windows::core::IUnknown {
    fn from(value: PaymentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentOptions> for ::windows::core::IUnknown {
    fn from(value: &PaymentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentOptions> for &::windows::core::IUnknown {
    fn from(value: &PaymentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentOptions> for ::windows::core::IInspectable {
    fn from(value: PaymentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentOptions> for ::windows::core::IInspectable {
    fn from(value: &PaymentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentOptions> for &::windows::core::IInspectable {
    fn from(value: &PaymentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentOptions {}
unsafe impl ::core::marker::Sync for PaymentOptions {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequest(::windows::core::IUnknown);
impl PaymentRequest {
    pub fn MerchantInfo(&self) -> ::windows::core::Result<PaymentMerchantInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MerchantInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentMerchantInfo>(result__)
        }
    }
    pub fn Details(&self) -> ::windows::core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MethodData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MethodData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentMethodData>>(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<PaymentOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptions>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPaymentRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, P0, P1, E1>(details: P0, methoddata: P1) -> ::windows::core::Result<PaymentRequest>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentDetails>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), details.into().abi(), methoddata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfo<'a, P0, P1, E1, P2>(details: P0, methoddata: P1, merchantinfo: P2) -> ::windows::core::Result<PaymentRequest>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentDetails>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, PaymentMerchantInfo>>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithMerchantInfo)(::windows::core::Interface::as_raw(this), details.into().abi(), methoddata.try_into().map_err(|e| e.into())?.abi(), merchantinfo.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoAndOptions<'a, P0, P1, E1, P2, P3>(details: P0, methoddata: P1, merchantinfo: P2, options: P3) -> ::windows::core::Result<PaymentRequest>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentDetails>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, PaymentMerchantInfo>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, PaymentOptions>>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithMerchantInfoAndOptions)(::windows::core::Interface::as_raw(this), details.into().abi(), methoddata.try_into().map_err(|e| e.into())?.abi(), merchantinfo.into().abi(), options.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoOptionsAndId<'a, P0, P1, E1, P2, P3>(details: P0, methoddata: P1, merchantinfo: P2, options: P3, id: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentRequest>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentDetails>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, PaymentMerchantInfo>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, PaymentOptions>>,
    {
        Self::IPaymentRequestFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithMerchantInfoOptionsAndId)(::windows::core::Interface::as_raw(this), details.into().abi(), methoddata.try_into().map_err(|e| e.into())?.abi(), merchantinfo.into().abi(), options.into().abi(), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentRequestFactory<R, F: FnOnce(&IPaymentRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentRequest, IPaymentRequestFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPaymentRequestFactory2<R, F: FnOnce(&IPaymentRequestFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentRequest, IPaymentRequestFactory2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequest {}
impl ::core::fmt::Debug for PaymentRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequest;{b74942e1-ed7b-47eb-bc08-78cc5d6896b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequest";
}
impl ::core::convert::From<PaymentRequest> for ::windows::core::IUnknown {
    fn from(value: PaymentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequest> for ::windows::core::IUnknown {
    fn from(value: &PaymentRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequest> for &::windows::core::IUnknown {
    fn from(value: &PaymentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentRequest> for ::windows::core::IInspectable {
    fn from(value: PaymentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequest> for ::windows::core::IInspectable {
    fn from(value: &PaymentRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequest> for &::windows::core::IInspectable {
    fn from(value: &PaymentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentRequest {}
unsafe impl ::core::marker::Sync for PaymentRequest {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PaymentRequestChangeKind(pub i32);
impl PaymentRequestChangeKind {
    pub const ShippingOption: Self = Self(0i32);
    pub const ShippingAddress: Self = Self(1i32);
}
impl ::core::marker::Copy for PaymentRequestChangeKind {}
impl ::core::clone::Clone for PaymentRequestChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentRequestChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PaymentRequestChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentRequestChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestChangedArgs(::windows::core::IUnknown);
impl PaymentRequestChangedArgs {
    pub fn ChangeKind(&self) -> ::windows::core::Result<PaymentRequestChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChangeKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentRequestChangeKind>(result__)
        }
    }
    pub fn ShippingAddress(&self) -> ::windows::core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShippingAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentAddress>(result__)
        }
    }
    pub fn SelectedShippingOption(&self) -> ::windows::core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedShippingOption)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        }
    }
    pub fn Acknowledge<'a, P0>(&self, changeresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequestChangedResult>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Acknowledge)(::windows::core::Interface::as_raw(this), changeresult.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for PaymentRequestChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestChangedArgs {}
impl ::core::fmt::Debug for PaymentRequestChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestChangedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedArgs;{c6145e44-cd8b-4be4-b555-27c99194c0c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentRequestChangedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentRequestChangedArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
}
impl ::core::convert::From<PaymentRequestChangedArgs> for ::windows::core::IUnknown {
    fn from(value: PaymentRequestChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedArgs> for ::windows::core::IUnknown {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequestChangedArgs> for &::windows::core::IUnknown {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentRequestChangedArgs> for ::windows::core::IInspectable {
    fn from(value: PaymentRequestChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedArgs> for ::windows::core::IInspectable {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequestChangedArgs> for &::windows::core::IInspectable {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentRequestChangedArgs {}
unsafe impl ::core::marker::Sync for PaymentRequestChangedArgs {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestChangedHandler(pub ::windows::core::IUnknown);
impl PaymentRequestChangedHandler {
    pub fn new<F: FnMut(&::core::option::Option<PaymentRequest>, &::core::option::Option<PaymentRequestChangedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PaymentRequestChangedHandlerBox::<F> { vtable: &PaymentRequestChangedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, P1>(&self, paymentrequest: P0, args: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PaymentRequestChangedArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), paymentrequest.into().abi(), args.into().abi()).ok() }
    }
}
#[repr(C)]
struct PaymentRequestChangedHandlerBox<F: FnMut(&::core::option::Option<PaymentRequest>, &::core::option::Option<PaymentRequestChangedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PaymentRequestChangedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<PaymentRequest>, &::core::option::Option<PaymentRequestChangedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> PaymentRequestChangedHandlerBox<F> {
    const VTABLE: PaymentRequestChangedHandler_Vtbl = PaymentRequestChangedHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<PaymentRequestChangedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&paymentrequest), ::core::mem::transmute(&args)).into()
    }
}
impl ::core::clone::Clone for PaymentRequestChangedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestChangedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestChangedHandler {}
impl ::core::fmt::Debug for PaymentRequestChangedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for PaymentRequestChangedHandler {
    type Vtable = PaymentRequestChangedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5078b9e1_f398_4f2c_a27e_94d371cf6c7d);
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestChangedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5078b9e1-f398-4f2c-a27e-94d371cf6c7d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PaymentRequestChangedHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestChangedResult(::windows::core::IUnknown);
impl PaymentRequestChangedResult {
    pub fn ChangeAcceptedByMerchant(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ChangeAcceptedByMerchant)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetChangeAcceptedByMerchant(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetChangeAcceptedByMerchant)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdatedPaymentDetails(&self) -> ::windows::core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UpdatedPaymentDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        }
    }
    pub fn SetUpdatedPaymentDetails<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentDetails>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUpdatedPaymentDetails)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Create(changeacceptedbymerchant: bool) -> ::windows::core::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), changeacceptedbymerchant, result__.as_mut_ptr()).from_abi::<PaymentRequestChangedResult>(result__)
        })
    }
    pub fn CreateWithPaymentDetails<'a, P0>(changeacceptedbymerchant: bool, updatedpaymentdetails: P0) -> ::windows::core::Result<PaymentRequestChangedResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentDetails>>,
    {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithPaymentDetails)(::windows::core::Interface::as_raw(this), changeacceptedbymerchant, updatedpaymentdetails.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequestChangedResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentRequestChangedResultFactory<R, F: FnOnce(&IPaymentRequestChangedResultFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentRequestChangedResult, IPaymentRequestChangedResultFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentRequestChangedResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestChangedResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestChangedResult {}
impl ::core::fmt::Debug for PaymentRequestChangedResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangedResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestChangedResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedResult;{df699e5c-16c4-47ad-9401-8440ec0757db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentRequestChangedResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentRequestChangedResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
}
impl ::core::convert::From<PaymentRequestChangedResult> for ::windows::core::IUnknown {
    fn from(value: PaymentRequestChangedResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedResult> for ::windows::core::IUnknown {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequestChangedResult> for &::windows::core::IUnknown {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentRequestChangedResult> for ::windows::core::IInspectable {
    fn from(value: PaymentRequestChangedResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedResult> for ::windows::core::IInspectable {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequestChangedResult> for &::windows::core::IInspectable {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentRequestChangedResult {}
unsafe impl ::core::marker::Sync for PaymentRequestChangedResult {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PaymentRequestCompletionStatus(pub i32);
impl PaymentRequestCompletionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentRequestCompletionStatus {}
impl ::core::clone::Clone for PaymentRequestCompletionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentRequestCompletionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PaymentRequestCompletionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentRequestCompletionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestCompletionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestCompletionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PaymentRequestStatus(pub i32);
impl PaymentRequestStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentRequestStatus {}
impl ::core::clone::Clone for PaymentRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentRequestStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PaymentRequestStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestSubmitResult(::windows::core::IUnknown);
impl PaymentRequestSubmitResult {
    pub fn Status(&self) -> ::windows::core::Result<PaymentRequestStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentRequestStatus>(result__)
        }
    }
    pub fn Response(&self) -> ::windows::core::Result<PaymentResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Response)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentResponse>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentRequestSubmitResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestSubmitResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestSubmitResult {}
impl ::core::fmt::Debug for PaymentRequestSubmitResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestSubmitResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentRequestSubmitResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestSubmitResult;{7b9c3912-30f2-4e90-b249-8ce7d78ffe56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentRequestSubmitResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentRequestSubmitResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
}
impl ::core::convert::From<PaymentRequestSubmitResult> for ::windows::core::IUnknown {
    fn from(value: PaymentRequestSubmitResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestSubmitResult> for ::windows::core::IUnknown {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequestSubmitResult> for &::windows::core::IUnknown {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentRequestSubmitResult> for ::windows::core::IInspectable {
    fn from(value: PaymentRequestSubmitResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestSubmitResult> for ::windows::core::IInspectable {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentRequestSubmitResult> for &::windows::core::IInspectable {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentRequestSubmitResult {}
unsafe impl ::core::marker::Sync for PaymentRequestSubmitResult {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentResponse(::windows::core::IUnknown);
impl PaymentResponse {
    pub fn PaymentToken(&self) -> ::windows::core::Result<PaymentToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PaymentToken)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentToken>(result__)
        }
    }
    pub fn ShippingOption(&self) -> ::windows::core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShippingOption)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        }
    }
    pub fn ShippingAddress(&self) -> ::windows::core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShippingAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentAddress>(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PayerEmail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PayerName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PayerPhoneNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CompleteAsync)(::windows::core::Interface::as_raw(this), status, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentResponse {}
impl ::core::fmt::Debug for PaymentResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentResponse;{e1389457-8bd2-4888-9fa8-97985545108e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentResponse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentResponse {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentResponse";
}
impl ::core::convert::From<PaymentResponse> for ::windows::core::IUnknown {
    fn from(value: PaymentResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentResponse> for ::windows::core::IUnknown {
    fn from(value: &PaymentResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentResponse> for &::windows::core::IUnknown {
    fn from(value: &PaymentResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentResponse> for ::windows::core::IInspectable {
    fn from(value: PaymentResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentResponse> for ::windows::core::IInspectable {
    fn from(value: &PaymentResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentResponse> for &::windows::core::IInspectable {
    fn from(value: &PaymentResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentResponse {}
unsafe impl ::core::marker::Sync for PaymentResponse {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentShippingOption(::windows::core::IUnknown);
impl PaymentShippingOption {
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Amount(&self) -> ::windows::core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Amount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        }
    }
    pub fn SetAmount<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentCurrencyAmount>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAmount)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Tag)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSelected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSelected)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, P0>(label: &::windows::core::HSTRING, amount: P0) -> ::windows::core::Result<PaymentShippingOption>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentCurrencyAmount>>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into().abi(), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        })
    }
    pub fn CreateWithSelected<'a, P0>(label: &::windows::core::HSTRING, amount: P0, selected: bool) -> ::windows::core::Result<PaymentShippingOption>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentCurrencyAmount>>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithSelected)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into().abi(), selected, result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        })
    }
    pub fn CreateWithSelectedAndTag<'a, P0>(label: &::windows::core::HSTRING, amount: P0, selected: bool, tag: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentShippingOption>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PaymentCurrencyAmount>>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithSelectedAndTag)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into().abi(), selected, ::core::mem::transmute_copy(tag), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentShippingOptionFactory<R, F: FnOnce(&IPaymentShippingOptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentShippingOption, IPaymentShippingOptionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentShippingOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentShippingOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentShippingOption {}
impl ::core::fmt::Debug for PaymentShippingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentShippingOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentShippingOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentShippingOption;{13372ada-9753-4574-8966-93145a76c7f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentShippingOption as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentShippingOption {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentShippingOption";
}
impl ::core::convert::From<PaymentShippingOption> for ::windows::core::IUnknown {
    fn from(value: PaymentShippingOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentShippingOption> for ::windows::core::IUnknown {
    fn from(value: &PaymentShippingOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentShippingOption> for &::windows::core::IUnknown {
    fn from(value: &PaymentShippingOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentShippingOption> for ::windows::core::IInspectable {
    fn from(value: PaymentShippingOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentShippingOption> for ::windows::core::IInspectable {
    fn from(value: &PaymentShippingOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentShippingOption> for &::windows::core::IInspectable {
    fn from(value: &PaymentShippingOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentShippingOption {}
unsafe impl ::core::marker::Sync for PaymentShippingOption {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PaymentShippingType(pub i32);
impl PaymentShippingType {
    pub const Shipping: Self = Self(0i32);
    pub const Delivery: Self = Self(1i32);
    pub const Pickup: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentShippingType {}
impl ::core::clone::Clone for PaymentShippingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentShippingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PaymentShippingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentShippingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentShippingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentShippingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentShippingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentToken(::windows::core::IUnknown);
impl PaymentToken {
    pub fn PaymentMethodId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PaymentMethodId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn JsonDetails(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).JsonDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Create(paymentmethodid: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(paymentmethodid), result__.as_mut_ptr()).from_abi::<PaymentToken>(result__)
        })
    }
    pub fn CreateWithJsonDetails(paymentmethodid: &::windows::core::HSTRING, jsondetails: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithJsonDetails)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(paymentmethodid), ::core::mem::transmute_copy(jsondetails), result__.as_mut_ptr()).from_abi::<PaymentToken>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentTokenFactory<R, F: FnOnce(&IPaymentTokenFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentToken, IPaymentTokenFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentToken {}
impl ::core::fmt::Debug for PaymentToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentToken").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentToken {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentToken;{bbcac013-ccd0-41f2-b2a1-0a2e4b5dce25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentToken {
    type Vtable = IPaymentToken_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentToken as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentToken {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentToken";
}
impl ::core::convert::From<PaymentToken> for ::windows::core::IUnknown {
    fn from(value: PaymentToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentToken> for ::windows::core::IUnknown {
    fn from(value: &PaymentToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentToken> for &::windows::core::IUnknown {
    fn from(value: &PaymentToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PaymentToken> for ::windows::core::IInspectable {
    fn from(value: PaymentToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentToken> for ::windows::core::IInspectable {
    fn from(value: &PaymentToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PaymentToken> for &::windows::core::IInspectable {
    fn from(value: &PaymentToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PaymentToken {}
unsafe impl ::core::marker::Sync for PaymentToken {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
