#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppCanMakePaymentTriggerDetailsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<super::PaymentRequest>;
    fn ReportCanMakePaymentResult(&self, value: &::core::option::Option<super::PaymentCanMakePaymentResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentAppCanMakePaymentTriggerDetailsVtbl {
    pub const fn new<Impl: IPaymentAppCanMakePaymentTriggerDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaymentAppCanMakePaymentTriggerDetailsVtbl {
        unsafe extern "system" fn Request<Impl: IPaymentAppCanMakePaymentTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCanMakePaymentResult<Impl: IPaymentAppCanMakePaymentTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCanMakePaymentResult(&*(&value as *const <super::PaymentCanMakePaymentResult as ::windows::core::Abi>::Abi as *const <super::PaymentCanMakePaymentResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaymentAppCanMakePaymentTriggerDetails>, base.5, Request::<Impl, OFFSET>, ReportCanMakePaymentResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppManagerImpl: Sized {
    fn RegisterAsync(&self, supportedpaymentmethodids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentAppManager";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentAppManagerVtbl {
    pub const fn new<Impl: IPaymentAppManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaymentAppManagerVtbl {
        unsafe extern "system" fn RegisterAsync<Impl: IPaymentAppManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedpaymentmethodids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterAsync(&*(&supportedpaymentmethodids as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAsync<Impl: IPaymentAppManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaymentAppManager>, base.5, RegisterAsync::<Impl, OFFSET>, UnregisterAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppManagerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<PaymentAppManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentAppManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentAppManagerStaticsVtbl {
    pub const fn new<Impl: IPaymentAppManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaymentAppManagerStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IPaymentAppManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaymentAppManagerStatics>, base.5, Current::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionImpl: Sized {
    fn PaymentRequest(&self) -> ::windows::core::Result<super::PaymentRequest>;
    fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerEmail(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerPhoneNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateShippingAddressAsync(&self, shippingaddress: &::core::option::Option<super::PaymentAddress>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>;
    fn UpdateSelectedShippingOptionAsync(&self, selectedshippingoption: &::core::option::Option<super::PaymentShippingOption>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>;
    fn AcceptAsync(&self, paymenttoken: &::core::option::Option<super::PaymentToken>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>;
    fn Reject(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentTransaction";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentTransactionVtbl {
    pub const fn new<Impl: IPaymentTransactionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaymentTransactionVtbl {
        unsafe extern "system" fn PaymentRequest<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PaymentRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayerEmail<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PayerEmail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayerEmail<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPayerEmail(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PayerName<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PayerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayerName<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPayerName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PayerPhoneNumber<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PayerPhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayerPhoneNumber<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPayerPhoneNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateShippingAddressAsync<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shippingaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateShippingAddressAsync(&*(&shippingaddress as *const <super::PaymentAddress as ::windows::core::Abi>::Abi as *const <super::PaymentAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSelectedShippingOptionAsync<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectedshippingoption: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateSelectedShippingOptionAsync(&*(&selectedshippingoption as *const <super::PaymentShippingOption as ::windows::core::Abi>::Abi as *const <super::PaymentShippingOption as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptAsync<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paymenttoken: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcceptAsync(&*(&paymenttoken as *const <super::PaymentToken as ::windows::core::Abi>::Abi as *const <super::PaymentToken as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IPaymentTransactionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaymentTransaction>, base.5, PaymentRequest::<Impl, OFFSET>, PayerEmail::<Impl, OFFSET>, SetPayerEmail::<Impl, OFFSET>, PayerName::<Impl, OFFSET>, SetPayerName::<Impl, OFFSET>, PayerPhoneNumber::<Impl, OFFSET>, SetPayerPhoneNumber::<Impl, OFFSET>, UpdateShippingAddressAsync::<Impl, OFFSET>, UpdateSelectedShippingOptionAsync::<Impl, OFFSET>, AcceptAsync::<Impl, OFFSET>, Reject::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionAcceptResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<super::PaymentRequestCompletionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentTransactionAcceptResultVtbl {
    pub const fn new<Impl: IPaymentTransactionAcceptResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaymentTransactionAcceptResultVtbl {
        unsafe extern "system" fn Status<Impl: IPaymentTransactionAcceptResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaymentTransactionAcceptResult>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionStaticsImpl: Sized {
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentTransactionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentTransactionStaticsVtbl {
    pub const fn new<Impl: IPaymentTransactionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaymentTransactionStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPaymentTransactionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaymentTransactionStatics>, base.5, FromIdAsync::<Impl, OFFSET>)
    }
}
