#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce201f0_8b93_4eb6_8c46_2e4a6c6a26f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentAppManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaymentAppManager {
    type Vtable = IPaymentAppManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e47aa53_8521_4969_a957_df2538a3a98f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, supportedpaymentmethodids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaymentAppManagerStatics {
    type Vtable = IPaymentAppManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa341ac28_fc89_4406_b4d9_34e7fe79dfb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentTransaction(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaymentTransaction {
    type Vtable = IPaymentTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62581da0_26a5_4e9b_a6eb_66606cf001d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shippingaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selectedshippingoption: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, paymenttoken: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060e3276_d30c_4817_95a2_df7ae9273b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaymentTransactionStatics {
    type Vtable = IPaymentTransactionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d639750_ee0a_4df5_9b1e_1c0f9ec59881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PaymentAppCanMakePaymentTriggerDetails(pub ::windows::core::IInspectable);
impl PaymentAppCanMakePaymentTriggerDetails {
    pub fn Request(&self) -> ::windows::core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn ReportCanMakePaymentResult<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentCanMakePaymentResult>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails;{0ce201f0-8b93-4eb6-8c46-2e4a6c6a26f6})");
}
unsafe impl ::windows::core::Interface for PaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce201f0_8b93_4eb6_8c46_2e4a6c6a26f6);
}
impl ::windows::core::RuntimeName for PaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
}
impl ::core::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTriggerDetails {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PaymentAppManager(pub ::windows::core::IInspectable);
impl PaymentAppManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, supportedpaymentmethodids: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), supportedpaymentmethodids.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<PaymentAppManager> {
        Self::IPaymentAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PaymentAppManager>(result__)
        })
    }
    pub fn IPaymentAppManagerStatics<R, F: FnOnce(&IPaymentAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaymentAppManager, IPaymentAppManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppManager;{0e47aa53-8521-4969-a957-df2538a3a98f})");
}
unsafe impl ::windows::core::Interface for PaymentAppManager {
    type Vtable = IPaymentAppManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e47aa53_8521_4969_a957_df2538a3a98f);
}
impl ::windows::core::RuntimeName for PaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
}
impl ::core::convert::From<PaymentAppManager> for ::windows::core::IUnknown {
    fn from(value: PaymentAppManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PaymentAppManager> for ::windows::core::IUnknown {
    fn from(value: &PaymentAppManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PaymentAppManager> for ::windows::core::IInspectable {
    fn from(value: PaymentAppManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PaymentAppManager> for ::windows::core::IInspectable {
    fn from(value: &PaymentAppManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PaymentAppManager {}
unsafe impl ::core::marker::Sync for PaymentAppManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PaymentTransaction(pub ::windows::core::IInspectable);
impl PaymentTransaction {
    pub fn PaymentRequest(&self) -> ::windows::core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPayerEmail<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPayerName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPayerPhoneNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateShippingAddressAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentAddress>>(&self, shippingaddress: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), shippingaddress.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateSelectedShippingOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentShippingOption>>(&self, selectedshippingoption: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), selectedshippingoption.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AcceptAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentToken>>(&self, paymenttoken: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), paymenttoken.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>(result__)
        }
    }
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>> {
        Self::IPaymentTransactionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>(result__)
        })
    }
    pub fn IPaymentTransactionStatics<R, F: FnOnce(&IPaymentTransactionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaymentTransaction, IPaymentTransactionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransaction;{62581da0-26a5-4e9b-a6eb-66606cf001d3})");
}
unsafe impl ::windows::core::Interface for PaymentTransaction {
    type Vtable = IPaymentTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62581da0_26a5_4e9b_a6eb_66606cf001d3);
}
impl ::windows::core::RuntimeName for PaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
}
impl ::core::convert::From<PaymentTransaction> for ::windows::core::IUnknown {
    fn from(value: PaymentTransaction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PaymentTransaction> for ::windows::core::IUnknown {
    fn from(value: &PaymentTransaction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PaymentTransaction> for ::windows::core::IInspectable {
    fn from(value: PaymentTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PaymentTransaction> for ::windows::core::IInspectable {
    fn from(value: &PaymentTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PaymentTransaction {}
unsafe impl ::core::marker::Sync for PaymentTransaction {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PaymentTransactionAcceptResult(pub ::windows::core::IInspectable);
impl PaymentTransactionAcceptResult {
    pub fn Status(&self) -> ::windows::core::Result<super::PaymentRequestCompletionStatus> {
        let this = self;
        unsafe {
            let mut result__: super::PaymentRequestCompletionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequestCompletionStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentTransactionAcceptResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult;{060e3276-d30c-4817-95a2-df7ae9273b56})");
}
unsafe impl ::windows::core::Interface for PaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060e3276_d30c_4817_95a2_df7ae9273b56);
}
impl ::windows::core::RuntimeName for PaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
}
impl ::core::convert::From<PaymentTransactionAcceptResult> for ::windows::core::IUnknown {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PaymentTransactionAcceptResult> for ::windows::core::IUnknown {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PaymentTransactionAcceptResult> for ::windows::core::IInspectable {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PaymentTransactionAcceptResult> for ::windows::core::IInspectable {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PaymentTransactionAcceptResult {}
unsafe impl ::core::marker::Sync for PaymentTransactionAcceptResult {}
