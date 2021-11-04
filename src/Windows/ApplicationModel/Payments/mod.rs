#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentAddress(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentAddress {
    type Vtable = IPaymentAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1596089577, 28474, 16742, [160, 24, 10, 11, 6, 187, 50, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1989606997, 54739, 19773, [179, 69, 69, 89, 23, 89, 197, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResultFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentCanMakePaymentResultFactory {
    type Vtable = IPaymentCanMakePaymentResultFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3151800894, 32073, 20329, [170, 83, 42, 15, 129, 100, 183, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResultFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PaymentCanMakePaymentResultStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmount(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3819170272, 46111, 18823, [189, 203, 7, 19, 49, 242, 218, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmount_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmountFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentCurrencyAmountFactory {
    type Vtable = IPaymentCurrencyAmountFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844616504, 5132, 17781, [133, 53, 247, 115, 23, 140, 9, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmountFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, currency: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, currency: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, currencysystem: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentDetails {
    type Vtable = IPaymentDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1404775805, 57579, 16467, [142, 174, 206, 124, 72, 224, 41, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentDetailsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentDetailsFactory {
    type Vtable = IPaymentDetailsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3488133102, 49386, 19617, [139, 199, 109, 230, 123, 31, 55, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, total: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, total: ::windows::runtime::RawPtr, displayitems: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentDetailsModifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189538149, 17187, 16855, [179, 5, 223, 203, 118, 95, 105, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentDetailsModifierFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentDetailsModifierFactory {
    type Vtable = IPaymentDetailsModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2030064262, 21726, 17052, [158, 79, 93, 206, 110, 16, 235, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifierFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedmethodids: ::windows::runtime::RawPtr, total: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedmethodids: ::windows::runtime::RawPtr, total: ::windows::runtime::RawPtr, additionaldisplayitems: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedmethodids: ::windows::runtime::RawPtr, total: ::windows::runtime::RawPtr, additionaldisplayitems: ::windows::runtime::RawPtr, jsondata: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentItem(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentItem {
    type Vtable = IPaymentItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750780043, 31154, 19318, [158, 3, 168, 118, 34, 61, 254, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentItemFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentItemFactory {
    type Vtable = IPaymentItemFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3333126872, 9475, 19741, [167, 120, 2, 178, 229, 146, 123, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItemFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, amount: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentMediator(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentMediator {
    type Vtable = IPaymentMediator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4212058153, 60428, 17562, [131, 218, 122, 227, 7, 51, 101, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymentrequest: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymentrequest: ::windows::runtime::RawPtr, changehandler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentMediator2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentMediator2 {
    type Vtable = IPaymentMediator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3471808753, 58375, 16680, [142, 115, 217, 61, 95, 130, 39, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymentrequest: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentMerchantInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1665421392, 3732, 20182, [170, 203, 230, 1, 43, 211, 39, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentMerchantInfoFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentMerchantInfoFactory {
    type Vtable = IPaymentMerchantInfoFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2659831507, 52407, 16743, [168, 236, 225, 10, 233, 109, 188, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentMethodData(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentMethodData {
    type Vtable = IPaymentMethodData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3520318196, 56984, 16681, [177, 183, 195, 173, 134, 35, 123, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentMethodDataFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentMethodDataFactory {
    type Vtable = IPaymentMethodDataFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2329793151, 39850, 19074, [131, 66, 168, 33, 9, 146, 163, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedmethodids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedmethodids: ::windows::runtime::RawPtr, jsondata: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentOptions {
    type Vtable = IPaymentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2862811220, 7979, 17253, [130, 81, 1, 181, 137, 21, 165, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentOptionPresence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PaymentOptionPresence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentOptionPresence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PaymentOptionPresence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentOptionPresence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PaymentOptionPresence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentShippingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PaymentShippingType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequest {
    type Vtable = IPaymentRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3075031777, 60795, 18411, [188, 8, 120, 204, 93, 104, 150, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequest2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequest2 {
    type Vtable = IPaymentRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3057438645, 22936, 18750, [160, 76, 103, 4, 138, 80, 241, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequestChangedArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3323223620, 52619, 19428, [181, 85, 39, 201, 145, 148, 192, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentRequestChangeKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changeresult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3748240988, 5828, 18349, [148, 1, 132, 64, 236, 7, 87, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResultFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequestChangedResultFactory {
    type Vtable = IPaymentRequestChangedResultFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(141823830, 7475, 17457, [129, 75, 103, 234, 36, 191, 33, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResultFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changeacceptedbymerchant: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changeacceptedbymerchant: bool, updatedpaymentdetails: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequestFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequestFactory {
    type Vtable = IPaymentRequestFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1049262556, 27508, 17107, [177, 3, 240, 222, 53, 251, 24, 72]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, details: ::windows::runtime::RawPtr, methoddata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, details: ::windows::runtime::RawPtr, methoddata: ::windows::runtime::RawPtr, merchantinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, details: ::windows::runtime::RawPtr, methoddata: ::windows::runtime::RawPtr, merchantinfo: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequestFactory2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequestFactory2 {
    type Vtable = IPaymentRequestFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872264997, 42246, 17266, [183, 239, 26, 3, 29, 86, 98, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, details: ::windows::runtime::RawPtr, methoddata: ::windows::runtime::RawPtr, merchantinfo: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentRequestSubmitResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2073835794, 12530, 20112, [178, 73, 140, 231, 215, 143, 254, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestSubmitResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PaymentRequestStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentResponse(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentResponse {
    type Vtable = IPaymentResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3778581591, 35794, 18568, [159, 168, 151, 152, 85, 69, 16, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentResponse_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: PaymentRequestCompletionStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentShippingOption(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentShippingOption {
    type Vtable = IPaymentShippingOption_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(322382554, 38739, 17780, [137, 102, 147, 20, 90, 118, 199, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOption_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentShippingOptionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentShippingOptionFactory {
    type Vtable = IPaymentShippingOptionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1575352599, 45783, 17515, [157, 115, 97, 35, 251, 202, 59, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOptionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, amount: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, amount: ::windows::runtime::RawPtr, selected: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, amount: ::windows::runtime::RawPtr, selected: bool, tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentToken(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentToken {
    type Vtable = IPaymentToken_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3150626835, 52432, 16882, [178, 161, 10, 46, 75, 93, 206, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentToken_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentTokenFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentTokenFactory {
    type Vtable = IPaymentTokenFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2559367082, 18259, 18692, [131, 115, 221, 123, 8, 185, 149, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTokenFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymentmethodid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymentmethodid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, jsondetails: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentAddress(pub ::windows::runtime::IInspectable);
impl PaymentAddress {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentAddress, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Country(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetCountry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn AddressLines(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn SetAddressLines<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Region(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetRegion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn City(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetCity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn DependentLocality(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetDependentLocality<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PostalCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetPostalCode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SortingCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetSortingCode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn LanguageCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetLanguageCode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Organization(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetOrganization<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Recipient(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetRecipient<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetPhoneNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentAddress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentAddress;{5f2264e9-6f3a-4166-a018-0a0b06bb32b5})");
}
unsafe impl ::windows::runtime::Interface for PaymentAddress {
    type Vtable = IPaymentAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1596089577, 28474, 16742, [160, 24, 10, 11, 6, 187, 50, 181]);
}
impl ::windows::runtime::RuntimeName for PaymentAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentAddress";
}
impl ::std::convert::From<PaymentAddress> for ::windows::runtime::IUnknown {
    fn from(value: PaymentAddress) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentAddress> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentAddress) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentAddress> for ::windows::runtime::IInspectable {
    fn from(value: PaymentAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentAddress> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentAddress {}
unsafe impl ::std::marker::Sync for PaymentAddress {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentCanMakePaymentResult(pub ::windows::runtime::IInspectable);
impl PaymentCanMakePaymentResult {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PaymentCanMakePaymentResultStatus> {
        let this = self;
        unsafe {
            let mut result__: PaymentCanMakePaymentResultStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentCanMakePaymentResultStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create(value: PaymentCanMakePaymentResultStatus) -> ::windows::runtime::Result<PaymentCanMakePaymentResult> {
        Self::IPaymentCanMakePaymentResultFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<PaymentCanMakePaymentResult>(result__)
        })
    }
    pub fn IPaymentCanMakePaymentResultFactory<R, F: FnOnce(&IPaymentCanMakePaymentResultFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentCanMakePaymentResult, IPaymentCanMakePaymentResultFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentCanMakePaymentResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult;{7696fe55-d5d3-4d3d-b345-45591759c510})");
}
unsafe impl ::windows::runtime::Interface for PaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1989606997, 54739, 19773, [179, 69, 69, 89, 23, 89, 197, 16]);
}
impl ::windows::runtime::RuntimeName for PaymentCanMakePaymentResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
}
impl ::std::convert::From<PaymentCanMakePaymentResult> for ::windows::runtime::IUnknown {
    fn from(value: PaymentCanMakePaymentResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentCanMakePaymentResult> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentCanMakePaymentResult> for ::windows::runtime::IInspectable {
    fn from(value: PaymentCanMakePaymentResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentCanMakePaymentResult> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentCanMakePaymentResult {}
unsafe impl ::std::marker::Sync for PaymentCanMakePaymentResult {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PaymentCanMakePaymentResultStatus(pub i32);
impl PaymentCanMakePaymentResultStatus {
    pub const Unknown: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(0i32);
    pub const Yes: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(1i32);
    pub const No: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(2i32);
    pub const NotAllowed: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(3i32);
    pub const UserNotSignedIn: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(4i32);
    pub const SpecifiedPaymentMethodIdsNotSupported: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(5i32);
    pub const NoQualifyingCardOnFile: PaymentCanMakePaymentResultStatus = PaymentCanMakePaymentResultStatus(6i32);
}
impl ::std::convert::From<i32> for PaymentCanMakePaymentResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PaymentCanMakePaymentResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PaymentCanMakePaymentResultStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus;i4)");
}
impl ::windows::runtime::DefaultType for PaymentCanMakePaymentResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentCurrencyAmount(pub ::windows::runtime::IInspectable);
impl PaymentCurrencyAmount {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Currency(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetCurrency<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn CurrencySystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetCurrencySystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0, currency: Param1) -> ::windows::runtime::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), currency.into_param().abi(), &mut result__).from_abi::<PaymentCurrencyAmount>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn CreateWithCurrencySystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0, currency: Param1, currencysystem: Param2) -> ::windows::runtime::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi(), currency.into_param().abi(), currencysystem.into_param().abi(), &mut result__).from_abi::<PaymentCurrencyAmount>(result__)
        })
    }
    pub fn IPaymentCurrencyAmountFactory<R, F: FnOnce(&IPaymentCurrencyAmountFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentCurrencyAmount, IPaymentCurrencyAmountFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentCurrencyAmount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCurrencyAmount;{e3a3e9e0-b41f-4987-bdcb-071331f2daa4})");
}
unsafe impl ::windows::runtime::Interface for PaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3819170272, 46111, 18823, [189, 203, 7, 19, 49, 242, 218, 164]);
}
impl ::windows::runtime::RuntimeName for PaymentCurrencyAmount {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
}
impl ::std::convert::From<PaymentCurrencyAmount> for ::windows::runtime::IUnknown {
    fn from(value: PaymentCurrencyAmount) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentCurrencyAmount> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentCurrencyAmount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentCurrencyAmount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentCurrencyAmount> for ::windows::runtime::IInspectable {
    fn from(value: PaymentCurrencyAmount) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentCurrencyAmount> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentCurrencyAmount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentCurrencyAmount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentCurrencyAmount {}
unsafe impl ::std::marker::Sync for PaymentCurrencyAmount {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentDetails(pub ::windows::runtime::IInspectable);
impl PaymentDetails {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentDetails, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Total(&self) -> ::windows::runtime::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentItem>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetTotal<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentItem>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn DisplayItems(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn SetDisplayItems<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<PaymentItem>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn ShippingOptions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn SetShippingOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn Modifiers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn SetModifiers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentItem>>(total: Param0) -> ::windows::runtime::Result<PaymentDetails> {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), total.into_param().abi(), &mut result__).from_abi::<PaymentDetails>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithDisplayItems<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentItem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>>(total: Param0, displayitems: Param1) -> ::windows::runtime::Result<PaymentDetails> {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), total.into_param().abi(), displayitems.into_param().abi(), &mut result__).from_abi::<PaymentDetails>(result__)
        })
    }
    pub fn IPaymentDetailsFactory<R, F: FnOnce(&IPaymentDetailsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentDetails, IPaymentDetailsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetails;{53bb2d7d-e0eb-4053-8eae-ce7c48e02945})");
}
unsafe impl ::windows::runtime::Interface for PaymentDetails {
    type Vtable = IPaymentDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1404775805, 57579, 16467, [142, 174, 206, 124, 72, 224, 41, 69]);
}
impl ::windows::runtime::RuntimeName for PaymentDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetails";
}
impl ::std::convert::From<PaymentDetails> for ::windows::runtime::IUnknown {
    fn from(value: PaymentDetails) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentDetails> for ::windows::runtime::IInspectable {
    fn from(value: PaymentDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentDetails> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentDetails {}
unsafe impl ::std::marker::Sync for PaymentDetails {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentDetailsModifier(pub ::windows::runtime::IInspectable);
impl PaymentDetailsModifier {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn JsonData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn SupportedMethodIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Total(&self) -> ::windows::runtime::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentItem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn AdditionalDisplayItems(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, PaymentItem>>(supportedmethodids: Param0, total: Param1) -> ::windows::runtime::Result<PaymentDetailsModifier> {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), supportedmethodids.into_param().abi(), total.into_param().abi(), &mut result__).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithAdditionalDisplayItems<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, PaymentItem>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>>(supportedmethodids: Param0, total: Param1, additionaldisplayitems: Param2) -> ::windows::runtime::Result<PaymentDetailsModifier> {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), supportedmethodids.into_param().abi(), total.into_param().abi(), additionaldisplayitems.into_param().abi(), &mut result__).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithAdditionalDisplayItemsAndJsonData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, PaymentItem>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        supportedmethodids: Param0,
        total: Param1,
        additionaldisplayitems: Param2,
        jsondata: Param3,
    ) -> ::windows::runtime::Result<PaymentDetailsModifier> {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), supportedmethodids.into_param().abi(), total.into_param().abi(), additionaldisplayitems.into_param().abi(), jsondata.into_param().abi(), &mut result__).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    pub fn IPaymentDetailsModifierFactory<R, F: FnOnce(&IPaymentDetailsModifierFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentDetailsModifier, IPaymentDetailsModifierFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentDetailsModifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetailsModifier;{be1c7d65-4323-41d7-b305-dfcb765f69de})");
}
unsafe impl ::windows::runtime::Interface for PaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189538149, 17187, 16855, [179, 5, 223, 203, 118, 95, 105, 222]);
}
impl ::windows::runtime::RuntimeName for PaymentDetailsModifier {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetailsModifier";
}
impl ::std::convert::From<PaymentDetailsModifier> for ::windows::runtime::IUnknown {
    fn from(value: PaymentDetailsModifier) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentDetailsModifier> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentDetailsModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentDetailsModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentDetailsModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentDetailsModifier> for ::windows::runtime::IInspectable {
    fn from(value: PaymentDetailsModifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentDetailsModifier> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentDetailsModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentDetailsModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentDetailsModifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentDetailsModifier {}
unsafe impl ::std::marker::Sync for PaymentDetailsModifier {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentItem(pub ::windows::runtime::IInspectable);
impl PaymentItem {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Amount(&self) -> ::windows::runtime::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentCurrencyAmount>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetAmount<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentCurrencyAmount>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Pending(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetPending(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PaymentCurrencyAmount>>(label: Param0, amount: Param1) -> ::windows::runtime::Result<PaymentItem> {
        Self::IPaymentItemFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), label.into_param().abi(), amount.into_param().abi(), &mut result__).from_abi::<PaymentItem>(result__)
        })
    }
    pub fn IPaymentItemFactory<R, F: FnOnce(&IPaymentItemFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentItem, IPaymentItemFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentItem;{685ac88b-79b2-4b76-9e03-a876223dfe72})");
}
unsafe impl ::windows::runtime::Interface for PaymentItem {
    type Vtable = IPaymentItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750780043, 31154, 19318, [158, 3, 168, 118, 34, 61, 254, 114]);
}
impl ::windows::runtime::RuntimeName for PaymentItem {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentItem";
}
impl ::std::convert::From<PaymentItem> for ::windows::runtime::IUnknown {
    fn from(value: PaymentItem) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentItem> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentItem> for ::windows::runtime::IInspectable {
    fn from(value: PaymentItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentItem> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentItem {}
unsafe impl ::std::marker::Sync for PaymentItem {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentMediator(pub ::windows::runtime::IInspectable);
impl PaymentMediator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentMediator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSupportedMethodIdsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`*"]
    pub fn SubmitPaymentRequestAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentRequest>>(&self, paymentrequest: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), paymentrequest.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`*"]
    pub fn SubmitPaymentRequestWithChangeHandlerAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentRequest>, Param1: ::windows::runtime::IntoParam<'a, PaymentRequestChangedHandler>>(&self, paymentrequest: Param0, changehandler: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), paymentrequest.into_param().abi(), changehandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`*"]
    pub fn CanMakePaymentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentRequest>>(&self, paymentrequest: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>> {
        let this = &::windows::runtime::Interface::cast::<IPaymentMediator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), paymentrequest.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentMediator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMediator;{fb0ee829-ec0c-449a-83da-7ae3073365a2})");
}
unsafe impl ::windows::runtime::Interface for PaymentMediator {
    type Vtable = IPaymentMediator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4212058153, 60428, 17562, [131, 218, 122, 227, 7, 51, 101, 162]);
}
impl ::windows::runtime::RuntimeName for PaymentMediator {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMediator";
}
impl ::std::convert::From<PaymentMediator> for ::windows::runtime::IUnknown {
    fn from(value: PaymentMediator) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentMediator> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentMediator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentMediator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentMediator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentMediator> for ::windows::runtime::IInspectable {
    fn from(value: PaymentMediator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentMediator> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentMediator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentMediator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentMediator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentMediator {}
unsafe impl ::std::marker::Sync for PaymentMediator {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentMerchantInfo(pub ::windows::runtime::IInspectable);
impl PaymentMerchantInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentMerchantInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PackageFullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<PaymentMerchantInfo> {
        Self::IPaymentMerchantInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<PaymentMerchantInfo>(result__)
        })
    }
    pub fn IPaymentMerchantInfoFactory<R, F: FnOnce(&IPaymentMerchantInfoFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentMerchantInfo, IPaymentMerchantInfoFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentMerchantInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMerchantInfo;{63445050-0e94-4ed6-aacb-e6012bd327a7})");
}
unsafe impl ::windows::runtime::Interface for PaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1665421392, 3732, 20182, [170, 203, 230, 1, 43, 211, 39, 167]);
}
impl ::windows::runtime::RuntimeName for PaymentMerchantInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMerchantInfo";
}
impl ::std::convert::From<PaymentMerchantInfo> for ::windows::runtime::IUnknown {
    fn from(value: PaymentMerchantInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentMerchantInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentMerchantInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentMerchantInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentMerchantInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentMerchantInfo> for ::windows::runtime::IInspectable {
    fn from(value: PaymentMerchantInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentMerchantInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentMerchantInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentMerchantInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentMerchantInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentMerchantInfo {}
unsafe impl ::std::marker::Sync for PaymentMerchantInfo {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentMethodData(pub ::windows::runtime::IInspectable);
impl PaymentMethodData {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn SupportedMethodIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn JsonData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(supportedmethodids: Param0) -> ::windows::runtime::Result<PaymentMethodData> {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), supportedmethodids.into_param().abi(), &mut result__).from_abi::<PaymentMethodData>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithJsonData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(supportedmethodids: Param0, jsondata: Param1) -> ::windows::runtime::Result<PaymentMethodData> {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), supportedmethodids.into_param().abi(), jsondata.into_param().abi(), &mut result__).from_abi::<PaymentMethodData>(result__)
        })
    }
    pub fn IPaymentMethodDataFactory<R, F: FnOnce(&IPaymentMethodDataFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentMethodData, IPaymentMethodDataFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentMethodData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMethodData;{d1d3caf4-de98-4129-b1b7-c3ad86237bf4})");
}
unsafe impl ::windows::runtime::Interface for PaymentMethodData {
    type Vtable = IPaymentMethodData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3520318196, 56984, 16681, [177, 183, 195, 173, 134, 35, 123, 244]);
}
impl ::windows::runtime::RuntimeName for PaymentMethodData {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMethodData";
}
impl ::std::convert::From<PaymentMethodData> for ::windows::runtime::IUnknown {
    fn from(value: PaymentMethodData) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentMethodData> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentMethodData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentMethodData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentMethodData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentMethodData> for ::windows::runtime::IInspectable {
    fn from(value: PaymentMethodData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentMethodData> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentMethodData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentMethodData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentMethodData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentMethodData {}
unsafe impl ::std::marker::Sync for PaymentMethodData {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PaymentOptionPresence(pub i32);
impl PaymentOptionPresence {
    pub const None: PaymentOptionPresence = PaymentOptionPresence(0i32);
    pub const Optional: PaymentOptionPresence = PaymentOptionPresence(1i32);
    pub const Required: PaymentOptionPresence = PaymentOptionPresence(2i32);
}
impl ::std::convert::From<i32> for PaymentOptionPresence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PaymentOptionPresence {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PaymentOptionPresence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentOptionPresence;i4)");
}
impl ::windows::runtime::DefaultType for PaymentOptionPresence {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentOptions(pub ::windows::runtime::IInspectable);
impl PaymentOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn RequestPayerEmail(&self) -> ::windows::runtime::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__: PaymentOptionPresence = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn RequestPayerName(&self) -> ::windows::runtime::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__: PaymentOptionPresence = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn RequestPayerPhoneNumber(&self) -> ::windows::runtime::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__: PaymentOptionPresence = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn RequestShipping(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetRequestShipping(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn ShippingType(&self) -> ::windows::runtime::Result<PaymentShippingType> {
        let this = self;
        unsafe {
            let mut result__: PaymentShippingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentShippingType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetShippingType(&self, value: PaymentShippingType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentOptions;{aaa30854-1f2b-4365-8251-01b58915a5bc})");
}
unsafe impl ::windows::runtime::Interface for PaymentOptions {
    type Vtable = IPaymentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2862811220, 7979, 17253, [130, 81, 1, 181, 137, 21, 165, 188]);
}
impl ::windows::runtime::RuntimeName for PaymentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentOptions";
}
impl ::std::convert::From<PaymentOptions> for ::windows::runtime::IUnknown {
    fn from(value: PaymentOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentOptions> for ::windows::runtime::IInspectable {
    fn from(value: PaymentOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentOptions {}
unsafe impl ::std::marker::Sync for PaymentOptions {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentRequest(pub ::windows::runtime::IInspectable);
impl PaymentRequest {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn MerchantInfo(&self) -> ::windows::runtime::Result<PaymentMerchantInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentMerchantInfo>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Details(&self) -> ::windows::runtime::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentDetails>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn MethodData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentMethodData>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Options(&self) -> ::windows::runtime::Result<PaymentOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentDetails>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>>(details: Param0, methoddata: Param1) -> ::windows::runtime::Result<PaymentRequest> {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), details.into_param().abi(), methoddata.into_param().abi(), &mut result__).from_abi::<PaymentRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithMerchantInfo<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentDetails>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Param2: ::windows::runtime::IntoParam<'a, PaymentMerchantInfo>>(details: Param0, methoddata: Param1, merchantinfo: Param2) -> ::windows::runtime::Result<PaymentRequest> {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), details.into_param().abi(), methoddata.into_param().abi(), merchantinfo.into_param().abi(), &mut result__).from_abi::<PaymentRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithMerchantInfoAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentDetails>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Param2: ::windows::runtime::IntoParam<'a, PaymentMerchantInfo>, Param3: ::windows::runtime::IntoParam<'a, PaymentOptions>>(details: Param0, methoddata: Param1, merchantinfo: Param2, options: Param3) -> ::windows::runtime::Result<PaymentRequest> {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), details.into_param().abi(), methoddata.into_param().abi(), merchantinfo.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<PaymentRequest>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPaymentRequest2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation_Collections`*"]
    pub fn CreateWithMerchantInfoOptionsAndId<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentDetails>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Param2: ::windows::runtime::IntoParam<'a, PaymentMerchantInfo>, Param3: ::windows::runtime::IntoParam<'a, PaymentOptions>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        details: Param0,
        methoddata: Param1,
        merchantinfo: Param2,
        options: Param3,
        id: Param4,
    ) -> ::windows::runtime::Result<PaymentRequest> {
        Self::IPaymentRequestFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), details.into_param().abi(), methoddata.into_param().abi(), merchantinfo.into_param().abi(), options.into_param().abi(), id.into_param().abi(), &mut result__).from_abi::<PaymentRequest>(result__)
        })
    }
    pub fn IPaymentRequestFactory<R, F: FnOnce(&IPaymentRequestFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentRequest, IPaymentRequestFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPaymentRequestFactory2<R, F: FnOnce(&IPaymentRequestFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentRequest, IPaymentRequestFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequest;{b74942e1-ed7b-47eb-bc08-78cc5d6896b6})");
}
unsafe impl ::windows::runtime::Interface for PaymentRequest {
    type Vtable = IPaymentRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3075031777, 60795, 18411, [188, 8, 120, 204, 93, 104, 150, 182]);
}
impl ::windows::runtime::RuntimeName for PaymentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequest";
}
impl ::std::convert::From<PaymentRequest> for ::windows::runtime::IUnknown {
    fn from(value: PaymentRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentRequest> for ::windows::runtime::IInspectable {
    fn from(value: PaymentRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentRequest {}
unsafe impl ::std::marker::Sync for PaymentRequest {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PaymentRequestChangeKind(pub i32);
impl PaymentRequestChangeKind {
    pub const ShippingOption: PaymentRequestChangeKind = PaymentRequestChangeKind(0i32);
    pub const ShippingAddress: PaymentRequestChangeKind = PaymentRequestChangeKind(1i32);
}
impl ::std::convert::From<i32> for PaymentRequestChangeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PaymentRequestChangeKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestChangeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestChangeKind;i4)");
}
impl ::windows::runtime::DefaultType for PaymentRequestChangeKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentRequestChangedArgs(pub ::windows::runtime::IInspectable);
impl PaymentRequestChangedArgs {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn ChangeKind(&self) -> ::windows::runtime::Result<PaymentRequestChangeKind> {
        let this = self;
        unsafe {
            let mut result__: PaymentRequestChangeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentRequestChangeKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn ShippingAddress(&self) -> ::windows::runtime::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentAddress>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SelectedShippingOption(&self) -> ::windows::runtime::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentShippingOption>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Acknowledge<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentRequestChangedResult>>(&self, changeresult: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), changeresult.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestChangedArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedArgs;{c6145e44-cd8b-4be4-b555-27c99194c0c5})");
}
unsafe impl ::windows::runtime::Interface for PaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3323223620, 52619, 19428, [181, 85, 39, 201, 145, 148, 192, 197]);
}
impl ::windows::runtime::RuntimeName for PaymentRequestChangedArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
}
impl ::std::convert::From<PaymentRequestChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: PaymentRequestChangedArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentRequestChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentRequestChangedArgs> for ::windows::runtime::IInspectable {
    fn from(value: PaymentRequestChangedArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentRequestChangedArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentRequestChangedArgs {}
unsafe impl ::std::marker::Sync for PaymentRequestChangedArgs {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentRequestChangedHandler(::windows::runtime::IUnknown);
impl PaymentRequestChangedHandler {
    pub fn new<F: FnMut(&::std::option::Option<PaymentRequest>, &::std::option::Option<PaymentRequestChangedArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = PaymentRequestChangedHandler_box::<F> {
            vtable: &PaymentRequestChangedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentRequest>, Param1: ::windows::runtime::IntoParam<'a, PaymentRequestChangedArgs>>(&self, paymentrequest: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), paymentrequest.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestChangedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({5078b9e1-f398-4f2c-a27e-94d371cf6c7d})");
}
unsafe impl ::windows::runtime::Interface for PaymentRequestChangedHandler {
    type Vtable = PaymentRequestChangedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1350089185, 62360, 20268, [162, 126, 148, 211, 113, 207, 108, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct PaymentRequestChangedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymentrequest: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct PaymentRequestChangedHandler_box<F: FnMut(&::std::option::Option<PaymentRequest>, &::std::option::Option<PaymentRequestChangedArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const PaymentRequestChangedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<PaymentRequest>, &::std::option::Option<PaymentRequestChangedArgs>) -> ::windows::runtime::Result<()> + 'static> PaymentRequestChangedHandler_box<F> {
    const VTABLE: PaymentRequestChangedHandler_abi = PaymentRequestChangedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<PaymentRequestChangedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, paymentrequest: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&paymentrequest as *const <PaymentRequest as ::windows::runtime::Abi>::Abi as *const <PaymentRequest as ::windows::runtime::DefaultType>::DefaultType),
            &*(&args as *const <PaymentRequestChangedArgs as ::windows::runtime::Abi>::Abi as *const <PaymentRequestChangedArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentRequestChangedResult(pub ::windows::runtime::IInspectable);
impl PaymentRequestChangedResult {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn ChangeAcceptedByMerchant(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetChangeAcceptedByMerchant(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn UpdatedPaymentDetails(&self) -> ::windows::runtime::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentDetails>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetUpdatedPaymentDetails<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentDetails>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create(changeacceptedbymerchant: bool) -> ::windows::runtime::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), changeacceptedbymerchant, &mut result__).from_abi::<PaymentRequestChangedResult>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn CreateWithPaymentDetails<'a, Param1: ::windows::runtime::IntoParam<'a, PaymentDetails>>(changeacceptedbymerchant: bool, updatedpaymentdetails: Param1) -> ::windows::runtime::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), changeacceptedbymerchant, updatedpaymentdetails.into_param().abi(), &mut result__).from_abi::<PaymentRequestChangedResult>(result__)
        })
    }
    pub fn IPaymentRequestChangedResultFactory<R, F: FnOnce(&IPaymentRequestChangedResultFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentRequestChangedResult, IPaymentRequestChangedResultFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestChangedResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedResult;{df699e5c-16c4-47ad-9401-8440ec0757db})");
}
unsafe impl ::windows::runtime::Interface for PaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3748240988, 5828, 18349, [148, 1, 132, 64, 236, 7, 87, 219]);
}
impl ::windows::runtime::RuntimeName for PaymentRequestChangedResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
}
impl ::std::convert::From<PaymentRequestChangedResult> for ::windows::runtime::IUnknown {
    fn from(value: PaymentRequestChangedResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentRequestChangedResult> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentRequestChangedResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentRequestChangedResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentRequestChangedResult> for ::windows::runtime::IInspectable {
    fn from(value: PaymentRequestChangedResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentRequestChangedResult> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentRequestChangedResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentRequestChangedResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentRequestChangedResult {}
unsafe impl ::std::marker::Sync for PaymentRequestChangedResult {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PaymentRequestCompletionStatus(pub i32);
impl PaymentRequestCompletionStatus {
    pub const Succeeded: PaymentRequestCompletionStatus = PaymentRequestCompletionStatus(0i32);
    pub const Failed: PaymentRequestCompletionStatus = PaymentRequestCompletionStatus(1i32);
    pub const Unknown: PaymentRequestCompletionStatus = PaymentRequestCompletionStatus(2i32);
}
impl ::std::convert::From<i32> for PaymentRequestCompletionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PaymentRequestCompletionStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestCompletionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus;i4)");
}
impl ::windows::runtime::DefaultType for PaymentRequestCompletionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PaymentRequestStatus(pub i32);
impl PaymentRequestStatus {
    pub const Succeeded: PaymentRequestStatus = PaymentRequestStatus(0i32);
    pub const Failed: PaymentRequestStatus = PaymentRequestStatus(1i32);
    pub const Canceled: PaymentRequestStatus = PaymentRequestStatus(2i32);
}
impl ::std::convert::From<i32> for PaymentRequestStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PaymentRequestStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestStatus;i4)");
}
impl ::windows::runtime::DefaultType for PaymentRequestStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentRequestSubmitResult(pub ::windows::runtime::IInspectable);
impl PaymentRequestSubmitResult {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PaymentRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: PaymentRequestStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentRequestStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Response(&self) -> ::windows::runtime::Result<PaymentResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentResponse>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentRequestSubmitResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestSubmitResult;{7b9c3912-30f2-4e90-b249-8ce7d78ffe56})");
}
unsafe impl ::windows::runtime::Interface for PaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2073835794, 12530, 20112, [178, 73, 140, 231, 215, 143, 254, 86]);
}
impl ::windows::runtime::RuntimeName for PaymentRequestSubmitResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
}
impl ::std::convert::From<PaymentRequestSubmitResult> for ::windows::runtime::IUnknown {
    fn from(value: PaymentRequestSubmitResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentRequestSubmitResult> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentRequestSubmitResult> for ::windows::runtime::IInspectable {
    fn from(value: PaymentRequestSubmitResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentRequestSubmitResult> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentRequestSubmitResult {}
unsafe impl ::std::marker::Sync for PaymentRequestSubmitResult {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentResponse(pub ::windows::runtime::IInspectable);
impl PaymentResponse {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PaymentToken(&self) -> ::windows::runtime::Result<PaymentToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentToken>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn ShippingOption(&self) -> ::windows::runtime::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentShippingOption>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn ShippingAddress(&self) -> ::windows::runtime::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentAddress>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PayerEmail(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PayerName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PayerPhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Payments`, `Foundation`*"]
    pub fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentResponse;{e1389457-8bd2-4888-9fa8-97985545108e})");
}
unsafe impl ::windows::runtime::Interface for PaymentResponse {
    type Vtable = IPaymentResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3778581591, 35794, 18568, [159, 168, 151, 152, 85, 69, 16, 142]);
}
impl ::windows::runtime::RuntimeName for PaymentResponse {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentResponse";
}
impl ::std::convert::From<PaymentResponse> for ::windows::runtime::IUnknown {
    fn from(value: PaymentResponse) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentResponse> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentResponse) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentResponse> for ::windows::runtime::IInspectable {
    fn from(value: PaymentResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentResponse> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentResponse {}
unsafe impl ::std::marker::Sync for PaymentResponse {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentShippingOption(pub ::windows::runtime::IInspectable);
impl PaymentShippingOption {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Amount(&self) -> ::windows::runtime::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentCurrencyAmount>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetAmount<'a, Param0: ::windows::runtime::IntoParam<'a, PaymentCurrencyAmount>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn IsSelected(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn SetIsSelected(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PaymentCurrencyAmount>>(label: Param0, amount: Param1) -> ::windows::runtime::Result<PaymentShippingOption> {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), label.into_param().abi(), amount.into_param().abi(), &mut result__).from_abi::<PaymentShippingOption>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn CreateWithSelected<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PaymentCurrencyAmount>>(label: Param0, amount: Param1, selected: bool) -> ::windows::runtime::Result<PaymentShippingOption> {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), label.into_param().abi(), amount.into_param().abi(), selected, &mut result__).from_abi::<PaymentShippingOption>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn CreateWithSelectedAndTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PaymentCurrencyAmount>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(label: Param0, amount: Param1, selected: bool, tag: Param3) -> ::windows::runtime::Result<PaymentShippingOption> {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), label.into_param().abi(), amount.into_param().abi(), selected, tag.into_param().abi(), &mut result__).from_abi::<PaymentShippingOption>(result__)
        })
    }
    pub fn IPaymentShippingOptionFactory<R, F: FnOnce(&IPaymentShippingOptionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentShippingOption, IPaymentShippingOptionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentShippingOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentShippingOption;{13372ada-9753-4574-8966-93145a76c7f9})");
}
unsafe impl ::windows::runtime::Interface for PaymentShippingOption {
    type Vtable = IPaymentShippingOption_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(322382554, 38739, 17780, [137, 102, 147, 20, 90, 118, 199, 249]);
}
impl ::windows::runtime::RuntimeName for PaymentShippingOption {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentShippingOption";
}
impl ::std::convert::From<PaymentShippingOption> for ::windows::runtime::IUnknown {
    fn from(value: PaymentShippingOption) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentShippingOption> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentShippingOption) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentShippingOption {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentShippingOption {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentShippingOption> for ::windows::runtime::IInspectable {
    fn from(value: PaymentShippingOption) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentShippingOption> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentShippingOption) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentShippingOption {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentShippingOption {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentShippingOption {}
unsafe impl ::std::marker::Sync for PaymentShippingOption {}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PaymentShippingType(pub i32);
impl PaymentShippingType {
    pub const Shipping: PaymentShippingType = PaymentShippingType(0i32);
    pub const Delivery: PaymentShippingType = PaymentShippingType(1i32);
    pub const Pickup: PaymentShippingType = PaymentShippingType(2i32);
}
impl ::std::convert::From<i32> for PaymentShippingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PaymentShippingType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PaymentShippingType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentShippingType;i4)");
}
impl ::windows::runtime::DefaultType for PaymentShippingType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Payments`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentToken(pub ::windows::runtime::IInspectable);
impl PaymentToken {
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn PaymentMethodId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn JsonDetails(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(paymentmethodid: Param0) -> ::windows::runtime::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), paymentmethodid.into_param().abi(), &mut result__).from_abi::<PaymentToken>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Payments`*"]
    pub fn CreateWithJsonDetails<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(paymentmethodid: Param0, jsondetails: Param1) -> ::windows::runtime::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), paymentmethodid.into_param().abi(), jsondetails.into_param().abi(), &mut result__).from_abi::<PaymentToken>(result__)
        })
    }
    pub fn IPaymentTokenFactory<R, F: FnOnce(&IPaymentTokenFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentToken, IPaymentTokenFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentToken {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentToken;{bbcac013-ccd0-41f2-b2a1-0a2e4b5dce25})");
}
unsafe impl ::windows::runtime::Interface for PaymentToken {
    type Vtable = IPaymentToken_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3150626835, 52432, 16882, [178, 161, 10, 46, 75, 93, 206, 37]);
}
impl ::windows::runtime::RuntimeName for PaymentToken {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentToken";
}
impl ::std::convert::From<PaymentToken> for ::windows::runtime::IUnknown {
    fn from(value: PaymentToken) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PaymentToken> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentToken) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PaymentToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PaymentToken> for ::windows::runtime::IInspectable {
    fn from(value: PaymentToken) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentToken> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentToken) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentToken {}
unsafe impl ::std::marker::Sync for PaymentToken {}
