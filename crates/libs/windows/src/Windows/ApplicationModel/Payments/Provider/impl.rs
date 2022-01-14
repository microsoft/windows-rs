#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppCanMakePaymentTriggerDetails_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<super::PaymentRequest>;
    fn ReportCanMakePaymentResult(&mut self, value: &::core::option::Option<super::PaymentCanMakePaymentResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentAppCanMakePaymentTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentAppCanMakePaymentTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentAppCanMakePaymentTriggerDetails_Vtbl {
        unsafe extern "system" fn Request<Impl: IPaymentAppCanMakePaymentTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCanMakePaymentResult<Impl: IPaymentAppCanMakePaymentTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCanMakePaymentResult(&*(&value as *const <super::PaymentCanMakePaymentResult as ::windows::core::Abi>::Abi as *const <super::PaymentCanMakePaymentResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentAppCanMakePaymentTriggerDetails, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            ReportCanMakePaymentResult: ReportCanMakePaymentResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentAppCanMakePaymentTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentAppManager_Impl: Sized {
    fn RegisterAsync(&mut self, supportedpaymentmethodids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn UnregisterAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentAppManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentAppManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentAppManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentAppManager_Vtbl {
        unsafe extern "system" fn RegisterAsync<Impl: IPaymentAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedpaymentmethodids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAsync(&*(&supportedpaymentmethodids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAsync<Impl: IPaymentAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentAppManager, BASE_OFFSET>(),
            RegisterAsync: RegisterAsync::<Impl, IMPL_OFFSET>,
            UnregisterAsync: UnregisterAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentAppManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppManagerStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<PaymentAppManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentAppManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentAppManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentAppManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentAppManagerStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IPaymentAppManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentAppManagerStatics, BASE_OFFSET>(), Current: Current::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentAppManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentTransaction_Impl: Sized {
    fn PaymentRequest(&mut self) -> ::windows::core::Result<super::PaymentRequest>;
    fn PayerEmail(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerEmail(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PayerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PayerPhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerPhoneNumber(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateShippingAddressAsync(&mut self, shippingaddress: &::core::option::Option<super::PaymentAddress>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>;
    fn UpdateSelectedShippingOptionAsync(&mut self, selectedshippingoption: &::core::option::Option<super::PaymentShippingOption>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>;
    fn AcceptAsync(&mut self, paymenttoken: &::core::option::Option<super::PaymentToken>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentTransaction";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPaymentTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentTransaction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentTransaction_Vtbl {
        unsafe extern "system" fn PaymentRequest<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaymentRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayerEmail<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayerEmail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayerEmail<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayerEmail(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PayerName<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayerName<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayerName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PayerPhoneNumber<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayerPhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayerPhoneNumber<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayerPhoneNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateShippingAddressAsync<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shippingaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateShippingAddressAsync(&*(&shippingaddress as *const <super::PaymentAddress as ::windows::core::Abi>::Abi as *const <super::PaymentAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSelectedShippingOptionAsync<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectedshippingoption: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateSelectedShippingOptionAsync(&*(&selectedshippingoption as *const <super::PaymentShippingOption as ::windows::core::Abi>::Abi as *const <super::PaymentShippingOption as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptAsync<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paymenttoken: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptAsync(&*(&paymenttoken as *const <super::PaymentToken as ::windows::core::Abi>::Abi as *const <super::PaymentToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IPaymentTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentTransaction, BASE_OFFSET>(),
            PaymentRequest: PaymentRequest::<Impl, IMPL_OFFSET>,
            PayerEmail: PayerEmail::<Impl, IMPL_OFFSET>,
            SetPayerEmail: SetPayerEmail::<Impl, IMPL_OFFSET>,
            PayerName: PayerName::<Impl, IMPL_OFFSET>,
            SetPayerName: SetPayerName::<Impl, IMPL_OFFSET>,
            PayerPhoneNumber: PayerPhoneNumber::<Impl, IMPL_OFFSET>,
            SetPayerPhoneNumber: SetPayerPhoneNumber::<Impl, IMPL_OFFSET>,
            UpdateShippingAddressAsync: UpdateShippingAddressAsync::<Impl, IMPL_OFFSET>,
            UpdateSelectedShippingOptionAsync: UpdateSelectedShippingOptionAsync::<Impl, IMPL_OFFSET>,
            AcceptAsync: AcceptAsync::<Impl, IMPL_OFFSET>,
            Reject: Reject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionAcceptResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<super::PaymentRequestCompletionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentTransactionAcceptResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentTransactionAcceptResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentTransactionAcceptResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IPaymentTransactionAcceptResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentTransactionAcceptResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentTransactionAcceptResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentTransactionStatics_Impl: Sized {
    fn FromIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentTransactionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPaymentTransactionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentTransactionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentTransactionStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPaymentTransactionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentTransactionStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentTransactionStatics as ::windows::core::Interface>::IID
    }
}
