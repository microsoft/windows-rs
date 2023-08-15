#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
}
impl ::core::clone::Clone for IPaymentAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f2264e9_6f3a_4166_a018_0a0b06bb32b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAddress_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddressLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddressLines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAddressLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAddressLines: usize,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DependentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDependentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SortingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSortingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LanguageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Organization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOrganization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
}
impl ::core::clone::Clone for IPaymentCanMakePaymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentCanMakePaymentResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7696fe55_d5d3_4d3d_b345_45591759c510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCanMakePaymentResultFactory {
    type Vtable = IPaymentCanMakePaymentResultFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentCanMakePaymentResultFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentCanMakePaymentResultFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbdcaa3e_7d49_4f69_aa53_2a0f8164b7c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentCanMakePaymentResultStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCurrencyAmount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
}
impl ::core::clone::Clone for IPaymentCurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentCurrencyAmount {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3a3e9e0_b41f_4987_bdcb_071331f2daa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmount_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCurrencyAmountFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCurrencyAmountFactory {
    type Vtable = IPaymentCurrencyAmountFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentCurrencyAmountFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentCurrencyAmountFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3257d338_140c_4575_8535_f773178c09a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmountFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, currency: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithCurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, currency: ::std::mem::MaybeUninit<::windows_core::HSTRING>, currencysystem: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
}
impl ::core::clone::Clone for IPaymentDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53bb2d7d_e0eb_4053_8eae_ce7c48e02945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTotal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShippingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShippingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Modifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetailsFactory {
    type Vtable = IPaymentDetailsFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentDetailsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentDetailsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfe8afee_c0ea_4ca1_8bc7_6de67b1f3763);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, displayitems: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithDisplayItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsModifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
}
impl ::core::clone::Clone for IPaymentDetailsModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentDetailsModifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe1c7d65_4323_41d7_b305_dfcb765f69de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub JsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalDisplayItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsModifierFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetailsModifierFactory {
    type Vtable = IPaymentDetailsModifierFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentDetailsModifierFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentDetailsModifierFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79005286_54de_429c_9e4f_5dce6e10ebce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifierFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, additionaldisplayitems: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItemsAndJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, additionaldisplayitems: *mut ::core::ffi::c_void, jsondata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItemsAndJsonData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentItem {
    type Vtable = IPaymentItem_Vtbl;
}
impl ::core::clone::Clone for IPaymentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x685ac88b_79b2_4b76_9e03_a876223dfe72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentItemFactory {
    type Vtable = IPaymentItemFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentItemFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentItemFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6ab7ad8_2503_4d1d_a778_02b2e5927b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows_core::HSTRING>, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMediator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
}
impl ::core::clone::Clone for IPaymentMediator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentMediator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb0ee829_ec0c_449a_83da_7ae3073365a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedMethodIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedMethodIdsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestWithChangeHandlerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestWithChangeHandlerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMediator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMediator2 {
    type Vtable = IPaymentMediator2_Vtbl;
}
impl ::core::clone::Clone for IPaymentMediator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentMediator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xceef98f1_e407_4128_8e73_d93d5f822786);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CanMakePaymentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanMakePaymentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMerchantInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
}
impl ::core::clone::Clone for IPaymentMerchantInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentMerchantInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63445050_0e94_4ed6_aacb_e6012bd327a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMerchantInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMerchantInfoFactory {
    type Vtable = IPaymentMerchantInfoFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentMerchantInfoFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentMerchantInfoFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e89ced3_ccb7_4167_a8ec_e10ae96dbcd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMethodData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
}
impl ::core::clone::Clone for IPaymentMethodData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentMethodData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1d3caf4_de98_4129_b1b7_c3ad86237bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub JsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMethodDataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMethodDataFactory {
    type Vtable = IPaymentMethodDataFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentMethodDataFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentMethodDataFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8addd27f_9baa_4a82_8342_a8210992a36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: *mut ::core::ffi::c_void, jsondata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithJsonData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
}
impl ::core::clone::Clone for IPaymentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaa30854_1f2b_4365_8251_01b58915a5bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub SetRequestPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub RequestPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub SetRequestPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub RequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub SetRequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub RequestShipping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequestShipping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShippingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentShippingType) -> ::windows_core::HRESULT,
    pub SetShippingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentShippingType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb74942e1_ed7b_47eb_bc08_78cc5d6896b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MerchantInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MethodData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MethodData: usize,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequest2 {
    type Vtable = IPaymentRequest2_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb63ccfb5_5998_493e_a04c_67048a50f141);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequestChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequestChangedArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6145e44_cd8b_4be4_b555_27c99194c0c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestChangeKind) -> ::windows_core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectedShippingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeresult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequestChangedResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequestChangedResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf699e5c_16c4_47ad_9401_8440ec0757db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UpdatedPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetUpdatedPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestChangedResultFactory {
    type Vtable = IPaymentRequestChangedResultFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequestChangedResultFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequestChangedResultFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08740f56_1d33_4431_814b_67ea24bf21db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, updatedpaymentdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestFactory {
    type Vtable = IPaymentRequestFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequestFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequestFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e8a79dc_6b74_42d3_b103_f0de35fb1848);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestFactory2 {
    type Vtable = IPaymentRequestFactory2_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequestFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequestFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6ce1325_a506_4372_b7ef_1a031d5662d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoOptionsAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoOptionsAndId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestSubmitResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
}
impl ::core::clone::Clone for IPaymentRequestSubmitResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentRequestSubmitResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b9c3912_30f2_4e90_b249_8ce7d78ffe56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestSubmitResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestStatus) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
}
impl ::core::clone::Clone for IPaymentResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentResponse {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1389457_8bd2_4888_9fa8_97985545108e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PaymentToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShippingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PaymentRequestCompletionStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentShippingOption(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
}
impl ::core::clone::Clone for IPaymentShippingOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentShippingOption {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13372ada_9753_4574_8966_93145a76c7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOption_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentShippingOptionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentShippingOptionFactory {
    type Vtable = IPaymentShippingOptionFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentShippingOptionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentShippingOptionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5de5f917_b2d7_446b_9d73_6123fbca3bc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOptionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows_core::HSTRING>, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows_core::HSTRING>, amount: *mut ::core::ffi::c_void, selected: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithSelectedAndTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows_core::HSTRING>, amount: *mut ::core::ffi::c_void, selected: bool, tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentToken(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentToken {
    type Vtable = IPaymentToken_Vtbl;
}
impl ::core::clone::Clone for IPaymentToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentToken {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbcac013_ccd0_41f2_b2a1_0a2e4b5dce25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentToken_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PaymentMethodId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JsonDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTokenFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentTokenFactory {
    type Vtable = IPaymentTokenFactory_Vtbl;
}
impl ::core::clone::Clone for IPaymentTokenFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPaymentTokenFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x988cd7aa_4753_4904_8373_dd7b08b995c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTokenFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentmethodid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithJsonDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentmethodid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, jsondetails: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentAddress(::windows_core::IUnknown);
impl PaymentAddress {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentAddress, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCountry(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCountry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddressLines(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddressLines)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAddressLines<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddressLines)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Region(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Region)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRegion(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn City(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).City)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCity(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DependentLocality(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DependentLocality)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDependentLocality(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDependentLocality)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPostalCode(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPostalCode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SortingCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SortingCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSortingCode(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSortingCode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LanguageCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LanguageCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguageCode(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguageCode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Organization(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Organization)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOrganization(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrganization)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Recipient(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Recipient)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRecipient(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecipient)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPhoneNumber(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhoneNumber)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PaymentAddress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentAddress;{5f2264e9-6f3a-4166-a018-0a0b06bb32b5})");
}
impl ::core::clone::Clone for PaymentAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentAddress {
    const IID: ::windows_core::GUID = <IPaymentAddress as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentAddress";
}
::windows_core::imp::interface_hierarchy!(PaymentAddress, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentAddress {}
unsafe impl ::core::marker::Sync for PaymentAddress {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentCanMakePaymentResult(::windows_core::IUnknown);
impl PaymentCanMakePaymentResult {
    pub fn Status(&self) -> ::windows_core::Result<PaymentCanMakePaymentResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(value: PaymentCanMakePaymentResultStatus) -> ::windows_core::Result<PaymentCanMakePaymentResult> {
        Self::IPaymentCanMakePaymentResultFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentCanMakePaymentResultFactory<R, F: FnOnce(&IPaymentCanMakePaymentResultFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentCanMakePaymentResult, IPaymentCanMakePaymentResultFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentCanMakePaymentResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult;{7696fe55-d5d3-4d3d-b345-45591759c510})");
}
impl ::core::clone::Clone for PaymentCanMakePaymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentCanMakePaymentResult {
    const IID: ::windows_core::GUID = <IPaymentCanMakePaymentResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentCanMakePaymentResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
}
::windows_core::imp::interface_hierarchy!(PaymentCanMakePaymentResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentCanMakePaymentResult {}
unsafe impl ::core::marker::Sync for PaymentCanMakePaymentResult {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentCurrencyAmount(::windows_core::IUnknown);
impl PaymentCurrencyAmount {
    pub fn Currency(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Currency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCurrency(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrency)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CurrencySystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrencySystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCurrencySystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrencySystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(value: &::windows_core::HSTRING, currency: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), ::core::mem::transmute_copy(currency), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithCurrencySystem(value: &::windows_core::HSTRING, currency: &::windows_core::HSTRING, currencysystem: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithCurrencySystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), ::core::mem::transmute_copy(currency), ::core::mem::transmute_copy(currencysystem), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentCurrencyAmountFactory<R, F: FnOnce(&IPaymentCurrencyAmountFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentCurrencyAmount, IPaymentCurrencyAmountFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentCurrencyAmount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCurrencyAmount;{e3a3e9e0-b41f-4987-bdcb-071331f2daa4})");
}
impl ::core::clone::Clone for PaymentCurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentCurrencyAmount {
    const IID: ::windows_core::GUID = <IPaymentCurrencyAmount as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentCurrencyAmount {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
}
::windows_core::imp::interface_hierarchy!(PaymentCurrencyAmount, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentCurrencyAmount {}
unsafe impl ::core::marker::Sync for PaymentCurrencyAmount {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentDetails(::windows_core::IUnknown);
impl PaymentDetails {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentDetails, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Total(&self) -> ::windows_core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Total)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTotal<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PaymentItem>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTotal)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayItems(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayItems)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDisplayItems<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<PaymentItem>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayItems)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShippingOptions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShippingOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShippingOptions<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShippingOptions)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Modifiers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetModifiers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModifiers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Create<P0>(total: P0) -> ::windows_core::Result<PaymentDetails>
    where
        P0: ::windows_core::IntoParam<PaymentItem>,
    {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), total.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithDisplayItems<P0, P1>(total: P0, displayitems: P1) -> ::windows_core::Result<PaymentDetails>
    where
        P0: ::windows_core::IntoParam<PaymentItem>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentItem>>,
    {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithDisplayItems)(::windows_core::Interface::as_raw(this), total.into_param().abi(), displayitems.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentDetailsFactory<R, F: FnOnce(&IPaymentDetailsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentDetails, IPaymentDetailsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetails;{53bb2d7d-e0eb-4053-8eae-ce7c48e02945})");
}
impl ::core::clone::Clone for PaymentDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentDetails {
    const IID: ::windows_core::GUID = <IPaymentDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetails";
}
::windows_core::imp::interface_hierarchy!(PaymentDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentDetails {}
unsafe impl ::core::marker::Sync for PaymentDetails {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentDetailsModifier(::windows_core::IUnknown);
impl PaymentDetailsModifier {
    pub fn JsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JsonData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedMethodIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Total(&self) -> ::windows_core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Total)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AdditionalDisplayItems(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalDisplayItems)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0, P1>(supportedmethodids: P0, total: P1) -> ::windows_core::Result<PaymentDetailsModifier>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
        P1: ::windows_core::IntoParam<PaymentItem>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), supportedmethodids.try_into_param()?.abi(), total.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItems<P0, P1, P2>(supportedmethodids: P0, total: P1, additionaldisplayitems: P2) -> ::windows_core::Result<PaymentDetailsModifier>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
        P1: ::windows_core::IntoParam<PaymentItem>,
        P2: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentItem>>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAdditionalDisplayItems)(::windows_core::Interface::as_raw(this), supportedmethodids.try_into_param()?.abi(), total.into_param().abi(), additionaldisplayitems.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItemsAndJsonData<P0, P1, P2>(supportedmethodids: P0, total: P1, additionaldisplayitems: P2, jsondata: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentDetailsModifier>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
        P1: ::windows_core::IntoParam<PaymentItem>,
        P2: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentItem>>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAdditionalDisplayItemsAndJsonData)(::windows_core::Interface::as_raw(this), supportedmethodids.try_into_param()?.abi(), total.into_param().abi(), additionaldisplayitems.try_into_param()?.abi(), ::core::mem::transmute_copy(jsondata), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentDetailsModifierFactory<R, F: FnOnce(&IPaymentDetailsModifierFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentDetailsModifier, IPaymentDetailsModifierFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentDetailsModifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetailsModifier;{be1c7d65-4323-41d7-b305-dfcb765f69de})");
}
impl ::core::clone::Clone for PaymentDetailsModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentDetailsModifier {
    const IID: ::windows_core::GUID = <IPaymentDetailsModifier as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentDetailsModifier {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetailsModifier";
}
::windows_core::imp::interface_hierarchy!(PaymentDetailsModifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentDetailsModifier {}
unsafe impl ::core::marker::Sync for PaymentDetailsModifier {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentItem(::windows_core::IUnknown);
impl PaymentItem {
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Amount(&self) -> ::windows_core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Amount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAmount<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PaymentCurrencyAmount>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmount)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Pending(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pending)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPending(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPending)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(label: &::windows_core::HSTRING, amount: P0) -> ::windows_core::Result<PaymentItem>
    where
        P0: ::windows_core::IntoParam<PaymentCurrencyAmount>,
    {
        Self::IPaymentItemFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentItemFactory<R, F: FnOnce(&IPaymentItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentItem, IPaymentItemFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentItem;{685ac88b-79b2-4b76-9e03-a876223dfe72})");
}
impl ::core::clone::Clone for PaymentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentItem {
    type Vtable = IPaymentItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentItem {
    const IID: ::windows_core::GUID = <IPaymentItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentItem {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentItem";
}
::windows_core::imp::interface_hierarchy!(PaymentItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentItem {}
unsafe impl ::core::marker::Sync for PaymentItem {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentMediator(::windows_core::IUnknown);
impl PaymentMediator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentMediator, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedMethodIdsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedMethodIdsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SubmitPaymentRequestAsync<P0>(&self, paymentrequest: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>
    where
        P0: ::windows_core::IntoParam<PaymentRequest>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubmitPaymentRequestAsync)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SubmitPaymentRequestWithChangeHandlerAsync<P0, P1>(&self, paymentrequest: P0, changehandler: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>
    where
        P0: ::windows_core::IntoParam<PaymentRequest>,
        P1: ::windows_core::IntoParam<PaymentRequestChangedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubmitPaymentRequestWithChangeHandlerAsync)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), changehandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanMakePaymentAsync<P0>(&self, paymentrequest: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>
    where
        P0: ::windows_core::IntoParam<PaymentRequest>,
    {
        let this = &::windows_core::ComInterface::cast::<IPaymentMediator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMakePaymentAsync)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PaymentMediator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMediator;{fb0ee829-ec0c-449a-83da-7ae3073365a2})");
}
impl ::core::clone::Clone for PaymentMediator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentMediator {
    const IID: ::windows_core::GUID = <IPaymentMediator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentMediator {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMediator";
}
::windows_core::imp::interface_hierarchy!(PaymentMediator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentMediator {}
unsafe impl ::core::marker::Sync for PaymentMediator {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentMerchantInfo(::windows_core::IUnknown);
impl PaymentMerchantInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentMerchantInfo, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PackageFullName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFullName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<P0>(uri: P0) -> ::windows_core::Result<PaymentMerchantInfo>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IPaymentMerchantInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentMerchantInfoFactory<R, F: FnOnce(&IPaymentMerchantInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentMerchantInfo, IPaymentMerchantInfoFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentMerchantInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMerchantInfo;{63445050-0e94-4ed6-aacb-e6012bd327a7})");
}
impl ::core::clone::Clone for PaymentMerchantInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentMerchantInfo {
    const IID: ::windows_core::GUID = <IPaymentMerchantInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentMerchantInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMerchantInfo";
}
::windows_core::imp::interface_hierarchy!(PaymentMerchantInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentMerchantInfo {}
unsafe impl ::core::marker::Sync for PaymentMerchantInfo {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentMethodData(::windows_core::IUnknown);
impl PaymentMethodData {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedMethodIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn JsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JsonData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(supportedmethodids: P0) -> ::windows_core::Result<PaymentMethodData>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), supportedmethodids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithJsonData<P0>(supportedmethodids: P0, jsondata: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentMethodData>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithJsonData)(::windows_core::Interface::as_raw(this), supportedmethodids.try_into_param()?.abi(), ::core::mem::transmute_copy(jsondata), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentMethodDataFactory<R, F: FnOnce(&IPaymentMethodDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentMethodData, IPaymentMethodDataFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentMethodData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMethodData;{d1d3caf4-de98-4129-b1b7-c3ad86237bf4})");
}
impl ::core::clone::Clone for PaymentMethodData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentMethodData {
    const IID: ::windows_core::GUID = <IPaymentMethodData as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentMethodData {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMethodData";
}
::windows_core::imp::interface_hierarchy!(PaymentMethodData, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentMethodData {}
unsafe impl ::core::marker::Sync for PaymentMethodData {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentOptions(::windows_core::IUnknown);
impl PaymentOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestPayerEmail(&self) -> ::windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPayerEmail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestPayerEmail)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerName(&self) -> ::windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPayerName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestPayerName)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerPhoneNumber(&self) -> ::windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPayerPhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestPayerPhoneNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestShipping(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestShipping)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestShipping(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestShipping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShippingType(&self) -> ::windows_core::Result<PaymentShippingType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShippingType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShippingType(&self, value: PaymentShippingType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShippingType)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for PaymentOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentOptions;{aaa30854-1f2b-4365-8251-01b58915a5bc})");
}
impl ::core::clone::Clone for PaymentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentOptions {
    const IID: ::windows_core::GUID = <IPaymentOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentOptions";
}
::windows_core::imp::interface_hierarchy!(PaymentOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentOptions {}
unsafe impl ::core::marker::Sync for PaymentOptions {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequest(::windows_core::IUnknown);
impl PaymentRequest {
    pub fn MerchantInfo(&self) -> ::windows_core::Result<PaymentMerchantInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MerchantInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Details(&self) -> ::windows_core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MethodData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MethodData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Options(&self) -> ::windows_core::Result<PaymentOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPaymentRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0, P1>(details: P0, methoddata: P1) -> ::windows_core::Result<PaymentRequest>
    where
        P0: ::windows_core::IntoParam<PaymentDetails>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfo<P0, P1, P2>(details: P0, methoddata: P1, merchantinfo: P2) -> ::windows_core::Result<PaymentRequest>
    where
        P0: ::windows_core::IntoParam<PaymentDetails>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
        P2: ::windows_core::IntoParam<PaymentMerchantInfo>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMerchantInfo)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.try_into_param()?.abi(), merchantinfo.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoAndOptions<P0, P1, P2, P3>(details: P0, methoddata: P1, merchantinfo: P2, options: P3) -> ::windows_core::Result<PaymentRequest>
    where
        P0: ::windows_core::IntoParam<PaymentDetails>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
        P2: ::windows_core::IntoParam<PaymentMerchantInfo>,
        P3: ::windows_core::IntoParam<PaymentOptions>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMerchantInfoAndOptions)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.try_into_param()?.abi(), merchantinfo.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoOptionsAndId<P0, P1, P2, P3>(details: P0, methoddata: P1, merchantinfo: P2, options: P3, id: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentRequest>
    where
        P0: ::windows_core::IntoParam<PaymentDetails>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
        P2: ::windows_core::IntoParam<PaymentMerchantInfo>,
        P3: ::windows_core::IntoParam<PaymentOptions>,
    {
        Self::IPaymentRequestFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMerchantInfoOptionsAndId)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.try_into_param()?.abi(), merchantinfo.into_param().abi(), options.into_param().abi(), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentRequestFactory<R, F: FnOnce(&IPaymentRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentRequest, IPaymentRequestFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPaymentRequestFactory2<R, F: FnOnce(&IPaymentRequestFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentRequest, IPaymentRequestFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequest;{b74942e1-ed7b-47eb-bc08-78cc5d6896b6})");
}
impl ::core::clone::Clone for PaymentRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentRequest {
    const IID: ::windows_core::GUID = <IPaymentRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequest";
}
::windows_core::imp::interface_hierarchy!(PaymentRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentRequest {}
unsafe impl ::core::marker::Sync for PaymentRequest {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestChangedArgs(::windows_core::IUnknown);
impl PaymentRequestChangedArgs {
    pub fn ChangeKind(&self) -> ::windows_core::Result<PaymentRequestChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShippingAddress(&self) -> ::windows_core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShippingAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectedShippingOption(&self) -> ::windows_core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedShippingOption)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Acknowledge<P0>(&self, changeresult: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PaymentRequestChangedResult>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Acknowledge)(::windows_core::Interface::as_raw(this), changeresult.into_param().abi()).ok() }
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
impl ::windows_core::RuntimeType for PaymentRequestChangedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedArgs;{c6145e44-cd8b-4be4-b555-27c99194c0c5})");
}
impl ::core::clone::Clone for PaymentRequestChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentRequestChangedArgs {
    const IID: ::windows_core::GUID = <IPaymentRequestChangedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequestChangedArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
}
::windows_core::imp::interface_hierarchy!(PaymentRequestChangedArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentRequestChangedArgs {}
unsafe impl ::core::marker::Sync for PaymentRequestChangedArgs {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestChangedResult(::windows_core::IUnknown);
impl PaymentRequestChangedResult {
    pub fn ChangeAcceptedByMerchant(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAcceptedByMerchant)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChangeAcceptedByMerchant(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChangeAcceptedByMerchant)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdatedPaymentDetails(&self) -> ::windows_core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdatedPaymentDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUpdatedPaymentDetails<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PaymentDetails>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdatedPaymentDetails)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create(changeacceptedbymerchant: bool) -> ::windows_core::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), changeacceptedbymerchant, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithPaymentDetails<P0>(changeacceptedbymerchant: bool, updatedpaymentdetails: P0) -> ::windows_core::Result<PaymentRequestChangedResult>
    where
        P0: ::windows_core::IntoParam<PaymentDetails>,
    {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPaymentDetails)(::windows_core::Interface::as_raw(this), changeacceptedbymerchant, updatedpaymentdetails.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentRequestChangedResultFactory<R, F: FnOnce(&IPaymentRequestChangedResultFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentRequestChangedResult, IPaymentRequestChangedResultFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentRequestChangedResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedResult;{df699e5c-16c4-47ad-9401-8440ec0757db})");
}
impl ::core::clone::Clone for PaymentRequestChangedResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentRequestChangedResult {
    const IID: ::windows_core::GUID = <IPaymentRequestChangedResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequestChangedResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
}
::windows_core::imp::interface_hierarchy!(PaymentRequestChangedResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentRequestChangedResult {}
unsafe impl ::core::marker::Sync for PaymentRequestChangedResult {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestSubmitResult(::windows_core::IUnknown);
impl PaymentRequestSubmitResult {
    pub fn Status(&self) -> ::windows_core::Result<PaymentRequestStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Response(&self) -> ::windows_core::Result<PaymentResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PaymentRequestSubmitResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestSubmitResult;{7b9c3912-30f2-4e90-b249-8ce7d78ffe56})");
}
impl ::core::clone::Clone for PaymentRequestSubmitResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentRequestSubmitResult {
    const IID: ::windows_core::GUID = <IPaymentRequestSubmitResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequestSubmitResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
}
::windows_core::imp::interface_hierarchy!(PaymentRequestSubmitResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentRequestSubmitResult {}
unsafe impl ::core::marker::Sync for PaymentRequestSubmitResult {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentResponse(::windows_core::IUnknown);
impl PaymentResponse {
    pub fn PaymentToken(&self) -> ::windows_core::Result<PaymentToken> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaymentToken)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShippingOption(&self) -> ::windows_core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShippingOption)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShippingAddress(&self) -> ::windows_core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShippingAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PayerEmail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PayerName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PayerName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PayerPhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompleteAsync)(::windows_core::Interface::as_raw(this), status, &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PaymentResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentResponse;{e1389457-8bd2-4888-9fa8-97985545108e})");
}
impl ::core::clone::Clone for PaymentResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentResponse {
    const IID: ::windows_core::GUID = <IPaymentResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentResponse {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentResponse";
}
::windows_core::imp::interface_hierarchy!(PaymentResponse, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentResponse {}
unsafe impl ::core::marker::Sync for PaymentResponse {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentShippingOption(::windows_core::IUnknown);
impl PaymentShippingOption {
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Amount(&self) -> ::windows_core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Amount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAmount<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PaymentCurrencyAmount>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmount)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSelected)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(label: &::windows_core::HSTRING, amount: P0) -> ::windows_core::Result<PaymentShippingOption>
    where
        P0: ::windows_core::IntoParam<PaymentCurrencyAmount>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithSelected<P0>(label: &::windows_core::HSTRING, amount: P0, selected: bool) -> ::windows_core::Result<PaymentShippingOption>
    where
        P0: ::windows_core::IntoParam<PaymentCurrencyAmount>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSelected)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into_param().abi(), selected, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithSelectedAndTag<P0>(label: &::windows_core::HSTRING, amount: P0, selected: bool, tag: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentShippingOption>
    where
        P0: ::windows_core::IntoParam<PaymentCurrencyAmount>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSelectedAndTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(label), amount.into_param().abi(), selected, ::core::mem::transmute_copy(tag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentShippingOptionFactory<R, F: FnOnce(&IPaymentShippingOptionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentShippingOption, IPaymentShippingOptionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentShippingOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentShippingOption;{13372ada-9753-4574-8966-93145a76c7f9})");
}
impl ::core::clone::Clone for PaymentShippingOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentShippingOption {
    const IID: ::windows_core::GUID = <IPaymentShippingOption as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentShippingOption {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentShippingOption";
}
::windows_core::imp::interface_hierarchy!(PaymentShippingOption, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentShippingOption {}
unsafe impl ::core::marker::Sync for PaymentShippingOption {}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentToken(::windows_core::IUnknown);
impl PaymentToken {
    pub fn PaymentMethodId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaymentMethodId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn JsonDetails(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JsonDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(paymentmethodid: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(paymentmethodid), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithJsonDetails(paymentmethodid: &::windows_core::HSTRING, jsondetails: &::windows_core::HSTRING) -> ::windows_core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithJsonDetails)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(paymentmethodid), ::core::mem::transmute_copy(jsondetails), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentTokenFactory<R, F: FnOnce(&IPaymentTokenFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentToken, IPaymentTokenFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PaymentToken {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentToken;{bbcac013-ccd0-41f2-b2a1-0a2e4b5dce25})");
}
impl ::core::clone::Clone for PaymentToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PaymentToken {
    type Vtable = IPaymentToken_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentToken {
    const IID: ::windows_core::GUID = <IPaymentToken as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentToken {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentToken";
}
::windows_core::imp::interface_hierarchy!(PaymentToken, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PaymentToken {}
unsafe impl ::core::marker::Sync for PaymentToken {}
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
impl ::windows_core::TypeKind for PaymentCanMakePaymentResultStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PaymentCanMakePaymentResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCanMakePaymentResultStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PaymentCanMakePaymentResultStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus;i4)");
}
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
impl ::windows_core::TypeKind for PaymentOptionPresence {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PaymentOptionPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentOptionPresence").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PaymentOptionPresence {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentOptionPresence;i4)");
}
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
impl ::windows_core::TypeKind for PaymentRequestChangeKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PaymentRequestChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangeKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PaymentRequestChangeKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestChangeKind;i4)");
}
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
impl ::windows_core::TypeKind for PaymentRequestCompletionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PaymentRequestCompletionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestCompletionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PaymentRequestCompletionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus;i4)");
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
impl ::windows_core::TypeKind for PaymentRequestStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PaymentRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PaymentRequestStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestStatus;i4)");
}
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
impl ::windows_core::TypeKind for PaymentShippingType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PaymentShippingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentShippingType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PaymentShippingType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentShippingType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
pub struct PaymentRequestChangedHandler(pub ::windows_core::IUnknown);
impl PaymentRequestChangedHandler {
    pub fn new<F: FnMut(::core::option::Option<&PaymentRequest>, ::core::option::Option<&PaymentRequestChangedArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PaymentRequestChangedHandlerBox::<F> { vtable: &PaymentRequestChangedHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, paymentrequest: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PaymentRequest>,
        P1: ::windows_core::IntoParam<PaymentRequestChangedArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PaymentRequestChangedHandlerBox<F: FnMut(::core::option::Option<&PaymentRequest>, ::core::option::Option<&PaymentRequestChangedArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PaymentRequestChangedHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&PaymentRequest>, ::core::option::Option<&PaymentRequestChangedArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> PaymentRequestChangedHandlerBox<F> {
    const VTABLE: PaymentRequestChangedHandler_Vtbl = PaymentRequestChangedHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<PaymentRequestChangedHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&paymentrequest), ::windows_core::from_raw_borrowed(&args)).into()
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
unsafe impl ::windows_core::Interface for PaymentRequestChangedHandler {
    type Vtable = PaymentRequestChangedHandler_Vtbl;
}
impl ::core::clone::Clone for PaymentRequestChangedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for PaymentRequestChangedHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5078b9e1_f398_4f2c_a27e_94d371cf6c7d);
}
impl ::windows_core::RuntimeType for PaymentRequestChangedHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5078b9e1-f398-4f2c-a27e-94d371cf6c7d}");
}
#[repr(C)]
#[doc(hidden)]
pub struct PaymentRequestChangedHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
