#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce201f0_8b93_4eb6_8c46_2e4a6c6a26f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ReportCanMakePaymentResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e47aa53_8521_4969_a957_df2538a3a98f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedpaymentmethodids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentAppManagerStatics {
    type Vtable = IPaymentAppManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa341ac28_fc89_4406_b4d9_34e7fe79dfb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransaction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62581da0_26a5_4e9b_a6eb_66606cf001d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransaction_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PaymentRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateShippingAddressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shippingaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateShippingAddressAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateSelectedShippingOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectedshippingoption: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSelectedShippingOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AcceptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymenttoken: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptAsync: usize,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransactionAcceptResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060e3276_d30c_4817_95a2_df7ae9273b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransactionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaymentTransactionStatics {
    type Vtable = IPaymentTransactionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d639750_ee0a_4df5_9b1e_1c0f9ec59881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTriggerDetails(::windows::core::IUnknown);
impl PaymentAppCanMakePaymentTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn ReportCanMakePaymentResult<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentCanMakePaymentResult>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanMakePaymentResult)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppCanMakePaymentTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppCanMakePaymentTriggerDetails {}
impl ::core::fmt::Debug for PaymentAppCanMakePaymentTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppCanMakePaymentTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails;{0ce201f0-8b93-4eb6-8c46-2e4a6c6a26f6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentAppCanMakePaymentTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
}
impl ::core::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTriggerDetails {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentAppManager(::windows::core::IUnknown);
impl PaymentAppManager {
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, supportedpaymentmethodids: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterAsync)(::core::mem::transmute_copy(this), supportedpaymentmethodids.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnregisterAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn Current() -> ::windows::core::Result<PaymentAppManager> {
        Self::IPaymentAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PaymentAppManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentAppManagerStatics<R, F: FnOnce(&IPaymentAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaymentAppManager, IPaymentAppManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppManager {}
impl ::core::fmt::Debug for PaymentAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppManager;{0e47aa53-8521-4969-a957-df2538a3a98f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentAppManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
}
impl ::core::convert::From<PaymentAppManager> for ::windows::core::IUnknown {
    fn from(value: PaymentAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppManager> for ::windows::core::IUnknown {
    fn from(value: &PaymentAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentAppManager> for ::windows::core::IInspectable {
    fn from(value: PaymentAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppManager> for ::windows::core::IInspectable {
    fn from(value: &PaymentAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentAppManager {}
unsafe impl ::core::marker::Sync for PaymentAppManager {}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentTransaction(::windows::core::IUnknown);
impl PaymentTransaction {
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn PaymentRequest(&self) -> ::windows::core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PaymentRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PayerEmail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn SetPayerEmail<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPayerEmail)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PayerName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn SetPayerName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPayerName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PayerPhoneNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn SetPayerPhoneNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPayerPhoneNumber)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateShippingAddressAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentAddress>>(&self, shippingaddress: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateShippingAddressAsync)(::core::mem::transmute_copy(this), shippingaddress.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateSelectedShippingOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentShippingOption>>(&self, selectedshippingoption: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateSelectedShippingOptionAsync)(::core::mem::transmute_copy(this), selectedshippingoption.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceptAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PaymentToken>>(&self, paymenttoken: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcceptAsync)(::core::mem::transmute_copy(this), paymenttoken.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reject)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>> {
        Self::IPaymentTransactionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentTransactionStatics<R, F: FnOnce(&IPaymentTransactionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaymentTransaction, IPaymentTransactionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentTransaction {}
impl ::core::fmt::Debug for PaymentTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransaction;{62581da0-26a5-4e9b-a6eb-66606cf001d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentTransaction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
}
impl ::core::convert::From<PaymentTransaction> for ::windows::core::IUnknown {
    fn from(value: PaymentTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransaction> for ::windows::core::IUnknown {
    fn from(value: &PaymentTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentTransaction> for ::windows::core::IInspectable {
    fn from(value: PaymentTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransaction> for ::windows::core::IInspectable {
    fn from(value: &PaymentTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentTransaction {}
unsafe impl ::core::marker::Sync for PaymentTransaction {}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentTransactionAcceptResult(::windows::core::IUnknown);
impl PaymentTransactionAcceptResult {
    #[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<super::PaymentRequestCompletionStatus> {
        let this = self;
        unsafe {
            let mut result__: super::PaymentRequestCompletionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequestCompletionStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentTransactionAcceptResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentTransactionAcceptResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentTransactionAcceptResult {}
impl ::core::fmt::Debug for PaymentTransactionAcceptResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentTransactionAcceptResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentTransactionAcceptResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult;{060e3276-d30c-4817-95a2-df7ae9273b56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
    const IID: ::windows::core::GUID = <IPaymentTransactionAcceptResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
}
impl ::core::convert::From<PaymentTransactionAcceptResult> for ::windows::core::IUnknown {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransactionAcceptResult> for ::windows::core::IUnknown {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentTransactionAcceptResult> for ::windows::core::IInspectable {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransactionAcceptResult> for ::windows::core::IInspectable {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentTransactionAcceptResult {}
unsafe impl ::core::marker::Sync for PaymentTransactionAcceptResult {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
