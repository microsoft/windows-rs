#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentAddressImpl: Sized {
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddressLines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetAddressLines(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DependentLocality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDependentLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SortingCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSortingCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LanguageCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguageCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Organization(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOrganization(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Recipient(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRecipient(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhoneNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentAddress";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentAddressVtbl {
        unsafe extern "system" fn Country<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Country() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCountry<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCountry(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddressLines<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressLines<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressLines(&*(&value as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Region<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Region() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegion<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn City<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCity<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCity(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DependentLocality<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DependentLocality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDependentLocality<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDependentLocality(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PostalCode<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalCode<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalCode(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SortingCode<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortingCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSortingCode<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSortingCode(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LanguageCode<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageCode<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageCode(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Organization<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Organization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrganization<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrganization(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Recipient<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecipient<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecipient(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhoneNumber<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneNumber<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IPaymentAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPaymentAddress>,
            ::windows::core::GetTrustLevel,
            Country::<Impl, IMPL_OFFSET>,
            SetCountry::<Impl, IMPL_OFFSET>,
            AddressLines::<Impl, IMPL_OFFSET>,
            SetAddressLines::<Impl, IMPL_OFFSET>,
            Region::<Impl, IMPL_OFFSET>,
            SetRegion::<Impl, IMPL_OFFSET>,
            City::<Impl, IMPL_OFFSET>,
            SetCity::<Impl, IMPL_OFFSET>,
            DependentLocality::<Impl, IMPL_OFFSET>,
            SetDependentLocality::<Impl, IMPL_OFFSET>,
            PostalCode::<Impl, IMPL_OFFSET>,
            SetPostalCode::<Impl, IMPL_OFFSET>,
            SortingCode::<Impl, IMPL_OFFSET>,
            SetSortingCode::<Impl, IMPL_OFFSET>,
            LanguageCode::<Impl, IMPL_OFFSET>,
            SetLanguageCode::<Impl, IMPL_OFFSET>,
            Organization::<Impl, IMPL_OFFSET>,
            SetOrganization::<Impl, IMPL_OFFSET>,
            Recipient::<Impl, IMPL_OFFSET>,
            SetRecipient::<Impl, IMPL_OFFSET>,
            PhoneNumber::<Impl, IMPL_OFFSET>,
            SetPhoneNumber::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCanMakePaymentResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PaymentCanMakePaymentResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentCanMakePaymentResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentCanMakePaymentResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentCanMakePaymentResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentCanMakePaymentResultVtbl {
        unsafe extern "system" fn Status<Impl: IPaymentCanMakePaymentResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentCanMakePaymentResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCanMakePaymentResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCanMakePaymentResultFactoryImpl: Sized {
    fn Create(&self, value: PaymentCanMakePaymentResultStatus) -> ::windows::core::Result<PaymentCanMakePaymentResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentCanMakePaymentResultFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentCanMakePaymentResultFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentCanMakePaymentResultFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentCanMakePaymentResultFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentCanMakePaymentResultFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PaymentCanMakePaymentResultStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentCanMakePaymentResultFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCanMakePaymentResultFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCurrencyAmountImpl: Sized {
    fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrency(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CurrencySystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrencySystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentCurrencyAmount {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentCurrencyAmount";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentCurrencyAmountVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentCurrencyAmountImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentCurrencyAmountVtbl {
        unsafe extern "system" fn Currency<Impl: IPaymentCurrencyAmountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Currency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrency<Impl: IPaymentCurrencyAmountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrency(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrencySystem<Impl: IPaymentCurrencyAmountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrencySystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrencySystem<Impl: IPaymentCurrencyAmountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrencySystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: IPaymentCurrencyAmountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPaymentCurrencyAmountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentCurrencyAmount>, ::windows::core::GetTrustLevel, Currency::<Impl, IMPL_OFFSET>, SetCurrency::<Impl, IMPL_OFFSET>, CurrencySystem::<Impl, IMPL_OFFSET>, SetCurrencySystem::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCurrencyAmount as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCurrencyAmountFactoryImpl: Sized {
    fn Create(&self, value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn CreateWithCurrencySystem(&self, value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING, currencysystem: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentCurrencyAmountFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentCurrencyAmountFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentCurrencyAmountFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentCurrencyAmountFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentCurrencyAmountFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&currency as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithCurrencySystem<Impl: IPaymentCurrencyAmountFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currencysystem: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithCurrencySystem(
                &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&currency as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&currencysystem as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentCurrencyAmountFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithCurrencySystem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCurrencyAmountFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsImpl: Sized {
    fn Total(&self) -> ::windows::core::Result<PaymentItem>;
    fn SetTotal(&self, value: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<()>;
    fn DisplayItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>>;
    fn SetDisplayItems(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentItem>>) -> ::windows::core::Result<()>;
    fn ShippingOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>;
    fn SetShippingOptions(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>) -> ::windows::core::Result<()>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>;
    fn SetModifiers(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentDetailsVtbl {
        unsafe extern "system" fn Total<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Total() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTotal<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTotal(&*(&value as *const <PaymentItem as ::windows::core::Abi>::Abi as *const <PaymentItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayItems<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayItems<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayItems(&*(&value as *const <super::super::Foundation::Collections::IVectorView<PaymentItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<PaymentItem> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShippingOptions<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShippingOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShippingOptions<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShippingOptions(&*(&value as *const <super::super::Foundation::Collections::IVectorView<PaymentShippingOption> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<PaymentShippingOption> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Modifiers<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModifiers<Impl: IPaymentDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModifiers(&*(&value as *const <super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPaymentDetails>,
            ::windows::core::GetTrustLevel,
            Total::<Impl, IMPL_OFFSET>,
            SetTotal::<Impl, IMPL_OFFSET>,
            DisplayItems::<Impl, IMPL_OFFSET>,
            SetDisplayItems::<Impl, IMPL_OFFSET>,
            ShippingOptions::<Impl, IMPL_OFFSET>,
            SetShippingOptions::<Impl, IMPL_OFFSET>,
            Modifiers::<Impl, IMPL_OFFSET>,
            SetModifiers::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsFactoryImpl: Sized {
    fn Create(&self, total: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<PaymentDetails>;
    fn CreateWithDisplayItems(&self, total: &::core::option::Option<PaymentItem>, displayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>) -> ::windows::core::Result<PaymentDetails>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentDetailsFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentDetailsFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentDetailsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentDetailsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentDetailsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentDetailsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, total: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&total as *const <PaymentItem as ::windows::core::Abi>::Abi as *const <PaymentItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithDisplayItems<Impl: IPaymentDetailsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, total: ::windows::core::RawPtr, displayitems: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithDisplayItems(&*(&total as *const <PaymentItem as ::windows::core::Abi>::Abi as *const <PaymentItem as ::windows::core::DefaultType>::DefaultType), &*(&displayitems as *const <super::super::Foundation::Collections::IIterable<PaymentItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentItem> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentDetailsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithDisplayItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetailsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsModifierImpl: Sized {
    fn JsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedMethodIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Total(&self) -> ::windows::core::Result<PaymentItem>;
    fn AdditionalDisplayItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentDetailsModifier {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentDetailsModifier";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentDetailsModifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentDetailsModifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentDetailsModifierVtbl {
        unsafe extern "system" fn JsonData<Impl: IPaymentDetailsModifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMethodIds<Impl: IPaymentDetailsModifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedMethodIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Total<Impl: IPaymentDetailsModifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Total() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalDisplayItems<Impl: IPaymentDetailsModifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalDisplayItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentDetailsModifier>, ::windows::core::GetTrustLevel, JsonData::<Impl, IMPL_OFFSET>, SupportedMethodIds::<Impl, IMPL_OFFSET>, Total::<Impl, IMPL_OFFSET>, AdditionalDisplayItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetailsModifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsModifierFactoryImpl: Sized {
    fn Create(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<PaymentDetailsModifier>;
    fn CreateWithAdditionalDisplayItems(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>, additionaldisplayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>) -> ::windows::core::Result<PaymentDetailsModifier>;
    fn CreateWithAdditionalDisplayItemsAndJsonData(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>, additionaldisplayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentDetailsModifier>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentDetailsModifierFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentDetailsModifierFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentDetailsModifierFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentDetailsModifierFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentDetailsModifierFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows::core::RawPtr, total: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&supportedmethodids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&total as *const <PaymentItem as ::windows::core::Abi>::Abi as *const <PaymentItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAdditionalDisplayItems<Impl: IPaymentDetailsModifierFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows::core::RawPtr, total: ::windows::core::RawPtr, additionaldisplayitems: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAdditionalDisplayItems(
                &*(&supportedmethodids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&total as *const <PaymentItem as ::windows::core::Abi>::Abi as *const <PaymentItem as ::windows::core::DefaultType>::DefaultType),
                &*(&additionaldisplayitems as *const <super::super::Foundation::Collections::IIterable<PaymentItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentItem> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAdditionalDisplayItemsAndJsonData<Impl: IPaymentDetailsModifierFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows::core::RawPtr, total: ::windows::core::RawPtr, additionaldisplayitems: ::windows::core::RawPtr, jsondata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAdditionalDisplayItemsAndJsonData(
                &*(&supportedmethodids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&total as *const <PaymentItem as ::windows::core::Abi>::Abi as *const <PaymentItem as ::windows::core::DefaultType>::DefaultType),
                &*(&additionaldisplayitems as *const <super::super::Foundation::Collections::IIterable<PaymentItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentItem> as ::windows::core::DefaultType>::DefaultType),
                &*(&jsondata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentDetailsModifierFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithAdditionalDisplayItems::<Impl, IMPL_OFFSET>, CreateWithAdditionalDisplayItemsAndJsonData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetailsModifierFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentItemImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Amount(&self) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn SetAmount(&self, value: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<()>;
    fn Pending(&self) -> ::windows::core::Result<bool>;
    fn SetPending(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentItem {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentItem";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentItemVtbl {
        unsafe extern "system" fn Label<Impl: IPaymentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IPaymentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Amount<Impl: IPaymentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Amount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmount<Impl: IPaymentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAmount(&*(&value as *const <PaymentCurrencyAmount as ::windows::core::Abi>::Abi as *const <PaymentCurrencyAmount as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pending<Impl: IPaymentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pending() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPending<Impl: IPaymentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPending(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentItem>, ::windows::core::GetTrustLevel, Label::<Impl, IMPL_OFFSET>, SetLabel::<Impl, IMPL_OFFSET>, Amount::<Impl, IMPL_OFFSET>, SetAmount::<Impl, IMPL_OFFSET>, Pending::<Impl, IMPL_OFFSET>, SetPending::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentItemFactoryImpl: Sized {
    fn Create(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<PaymentItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentItemFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentItemFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentItemFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentItemFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentItemFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&amount as *const <PaymentCurrencyAmount as ::windows::core::Abi>::Abi as *const <PaymentCurrencyAmount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentItemFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentItemFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentMediatorImpl: Sized {
    fn GetSupportedMethodIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SubmitPaymentRequestAsync(&self, paymentrequest: &::core::option::Option<PaymentRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>;
    fn SubmitPaymentRequestWithChangeHandlerAsync(&self, paymentrequest: &::core::option::Option<PaymentRequest>, changehandler: &::core::option::Option<PaymentRequestChangedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentMediator {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentMediator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentMediatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentMediatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentMediatorVtbl {
        unsafe extern "system" fn GetSupportedMethodIdsAsync<Impl: IPaymentMediatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedMethodIdsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitPaymentRequestAsync<Impl: IPaymentMediatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paymentrequest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitPaymentRequestAsync(&*(&paymentrequest as *const <PaymentRequest as ::windows::core::Abi>::Abi as *const <PaymentRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitPaymentRequestWithChangeHandlerAsync<Impl: IPaymentMediatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paymentrequest: ::windows::core::RawPtr, changehandler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitPaymentRequestWithChangeHandlerAsync(&*(&paymentrequest as *const <PaymentRequest as ::windows::core::Abi>::Abi as *const <PaymentRequest as ::windows::core::DefaultType>::DefaultType), &*(&changehandler as *const <PaymentRequestChangedHandler as ::windows::core::Abi>::Abi as *const <PaymentRequestChangedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentMediator>, ::windows::core::GetTrustLevel, GetSupportedMethodIdsAsync::<Impl, IMPL_OFFSET>, SubmitPaymentRequestAsync::<Impl, IMPL_OFFSET>, SubmitPaymentRequestWithChangeHandlerAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMediator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentMediator2Impl: Sized {
    fn CanMakePaymentAsync(&self, paymentrequest: &::core::option::Option<PaymentRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentMediator2 {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentMediator2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPaymentMediator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentMediator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentMediator2Vtbl {
        unsafe extern "system" fn CanMakePaymentAsync<Impl: IPaymentMediator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paymentrequest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMakePaymentAsync(&*(&paymentrequest as *const <PaymentRequest as ::windows::core::Abi>::Abi as *const <PaymentRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentMediator2>, ::windows::core::GetTrustLevel, CanMakePaymentAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMediator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentMerchantInfoImpl: Sized {
    fn PackageFullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentMerchantInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentMerchantInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPaymentMerchantInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentMerchantInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentMerchantInfoVtbl {
        unsafe extern "system" fn PackageFullName<Impl: IPaymentMerchantInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFullName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IPaymentMerchantInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentMerchantInfo>, ::windows::core::GetTrustLevel, PackageFullName::<Impl, IMPL_OFFSET>, Uri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMerchantInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentMerchantInfoFactoryImpl: Sized {
    fn Create(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<PaymentMerchantInfo>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentMerchantInfoFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPaymentMerchantInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentMerchantInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentMerchantInfoFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentMerchantInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentMerchantInfoFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMerchantInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentMethodDataImpl: Sized {
    fn SupportedMethodIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn JsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentMethodData {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentMethodData";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentMethodDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentMethodDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentMethodDataVtbl {
        unsafe extern "system" fn SupportedMethodIds<Impl: IPaymentMethodDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedMethodIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JsonData<Impl: IPaymentMethodDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentMethodData>, ::windows::core::GetTrustLevel, SupportedMethodIds::<Impl, IMPL_OFFSET>, JsonData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMethodData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentMethodDataFactoryImpl: Sized {
    fn Create(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PaymentMethodData>;
    fn CreateWithJsonData(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentMethodData>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentMethodDataFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentMethodDataFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentMethodDataFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentMethodDataFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentMethodDataFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentMethodDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&supportedmethodids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithJsonData<Impl: IPaymentMethodDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows::core::RawPtr, jsondata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithJsonData(&*(&supportedmethodids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&jsondata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentMethodDataFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithJsonData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMethodDataFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentOptionsImpl: Sized {
    fn RequestPayerEmail(&self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestPayerName(&self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestPayerPhoneNumber(&self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestShipping(&self) -> ::windows::core::Result<bool>;
    fn SetRequestShipping(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShippingType(&self) -> ::windows::core::Result<PaymentShippingType>;
    fn SetShippingType(&self, value: PaymentShippingType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentOptionsVtbl {
        unsafe extern "system" fn RequestPayerEmail<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPayerEmail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestPayerEmail<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestPayerEmail(value).into()
        }
        unsafe extern "system" fn RequestPayerName<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPayerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestPayerName<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestPayerName(value).into()
        }
        unsafe extern "system" fn RequestPayerPhoneNumber<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPayerPhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestPayerPhoneNumber<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestPayerPhoneNumber(value).into()
        }
        unsafe extern "system" fn RequestShipping<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestShipping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestShipping<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestShipping(value).into()
        }
        unsafe extern "system" fn ShippingType<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentShippingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShippingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShippingType<Impl: IPaymentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PaymentShippingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShippingType(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPaymentOptions>,
            ::windows::core::GetTrustLevel,
            RequestPayerEmail::<Impl, IMPL_OFFSET>,
            SetRequestPayerEmail::<Impl, IMPL_OFFSET>,
            RequestPayerName::<Impl, IMPL_OFFSET>,
            SetRequestPayerName::<Impl, IMPL_OFFSET>,
            RequestPayerPhoneNumber::<Impl, IMPL_OFFSET>,
            SetRequestPayerPhoneNumber::<Impl, IMPL_OFFSET>,
            RequestShipping::<Impl, IMPL_OFFSET>,
            SetRequestShipping::<Impl, IMPL_OFFSET>,
            ShippingType::<Impl, IMPL_OFFSET>,
            SetShippingType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentRequestImpl: Sized {
    fn MerchantInfo(&self) -> ::windows::core::Result<PaymentMerchantInfo>;
    fn Details(&self) -> ::windows::core::Result<PaymentDetails>;
    fn MethodData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>>;
    fn Options(&self) -> ::windows::core::Result<PaymentOptions>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequest";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestVtbl {
        unsafe extern "system" fn MerchantInfo<Impl: IPaymentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MerchantInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Details<Impl: IPaymentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MethodData<Impl: IPaymentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MethodData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: IPaymentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequest>, ::windows::core::GetTrustLevel, MerchantInfo::<Impl, IMPL_OFFSET>, Details::<Impl, IMPL_OFFSET>, MethodData::<Impl, IMPL_OFFSET>, Options::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequest2Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentRequest2 {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequest2Vtbl {
        unsafe extern "system" fn Id<Impl: IPaymentRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequest2>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedArgsImpl: Sized {
    fn ChangeKind(&self) -> ::windows::core::Result<PaymentRequestChangeKind>;
    fn ShippingAddress(&self) -> ::windows::core::Result<PaymentAddress>;
    fn SelectedShippingOption(&self) -> ::windows::core::Result<PaymentShippingOption>;
    fn Acknowledge(&self, changeresult: &::core::option::Option<PaymentRequestChangedResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentRequestChangedArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentRequestChangedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestChangedArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestChangedArgsVtbl {
        unsafe extern "system" fn ChangeKind<Impl: IPaymentRequestChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestChangeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShippingAddress<Impl: IPaymentRequestChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShippingAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedShippingOption<Impl: IPaymentRequestChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedShippingOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acknowledge<Impl: IPaymentRequestChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changeresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acknowledge(&*(&changeresult as *const <PaymentRequestChangedResult as ::windows::core::Abi>::Abi as *const <PaymentRequestChangedResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequestChangedArgs>, ::windows::core::GetTrustLevel, ChangeKind::<Impl, IMPL_OFFSET>, ShippingAddress::<Impl, IMPL_OFFSET>, SelectedShippingOption::<Impl, IMPL_OFFSET>, Acknowledge::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestChangedArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedResultImpl: Sized {
    fn ChangeAcceptedByMerchant(&self) -> ::windows::core::Result<bool>;
    fn SetChangeAcceptedByMerchant(&self, value: bool) -> ::windows::core::Result<()>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdatedPaymentDetails(&self) -> ::windows::core::Result<PaymentDetails>;
    fn SetUpdatedPaymentDetails(&self, value: &::core::option::Option<PaymentDetails>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentRequestChangedResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequestChangedResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentRequestChangedResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestChangedResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestChangedResultVtbl {
        unsafe extern "system" fn ChangeAcceptedByMerchant<Impl: IPaymentRequestChangedResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeAcceptedByMerchant() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeAcceptedByMerchant<Impl: IPaymentRequestChangedResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChangeAcceptedByMerchant(value).into()
        }
        unsafe extern "system" fn Message<Impl: IPaymentRequestChangedResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: IPaymentRequestChangedResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdatedPaymentDetails<Impl: IPaymentRequestChangedResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatedPaymentDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdatedPaymentDetails<Impl: IPaymentRequestChangedResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdatedPaymentDetails(&*(&value as *const <PaymentDetails as ::windows::core::Abi>::Abi as *const <PaymentDetails as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPaymentRequestChangedResult>,
            ::windows::core::GetTrustLevel,
            ChangeAcceptedByMerchant::<Impl, IMPL_OFFSET>,
            SetChangeAcceptedByMerchant::<Impl, IMPL_OFFSET>,
            Message::<Impl, IMPL_OFFSET>,
            SetMessage::<Impl, IMPL_OFFSET>,
            UpdatedPaymentDetails::<Impl, IMPL_OFFSET>,
            SetUpdatedPaymentDetails::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestChangedResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedResultFactoryImpl: Sized {
    fn Create(&self, changeacceptedbymerchant: bool) -> ::windows::core::Result<PaymentRequestChangedResult>;
    fn CreateWithPaymentDetails(&self, changeacceptedbymerchant: bool, updatedpaymentdetails: &::core::option::Option<PaymentDetails>) -> ::windows::core::Result<PaymentRequestChangedResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentRequestChangedResultFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentRequestChangedResultFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestChangedResultFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestChangedResultFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentRequestChangedResultFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(changeacceptedbymerchant) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPaymentDetails<Impl: IPaymentRequestChangedResultFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, updatedpaymentdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPaymentDetails(changeacceptedbymerchant, &*(&updatedpaymentdetails as *const <PaymentDetails as ::windows::core::Abi>::Abi as *const <PaymentDetails as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequestChangedResultFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithPaymentDetails::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestChangedResultFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentRequestFactoryImpl: Sized {
    fn Create(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>) -> ::windows::core::Result<PaymentRequest>;
    fn CreateWithMerchantInfo(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>) -> ::windows::core::Result<PaymentRequest>;
    fn CreateWithMerchantInfoAndOptions(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>, options: &::core::option::Option<PaymentOptions>) -> ::windows::core::Result<PaymentRequest>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentRequestFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequestFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentRequestFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, details: ::windows::core::RawPtr, methoddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&details as *const <PaymentDetails as ::windows::core::Abi>::Abi as *const <PaymentDetails as ::windows::core::DefaultType>::DefaultType), &*(&methoddata as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMerchantInfo<Impl: IPaymentRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, details: ::windows::core::RawPtr, methoddata: ::windows::core::RawPtr, merchantinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMerchantInfo(
                &*(&details as *const <PaymentDetails as ::windows::core::Abi>::Abi as *const <PaymentDetails as ::windows::core::DefaultType>::DefaultType),
                &*(&methoddata as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::DefaultType>::DefaultType),
                &*(&merchantinfo as *const <PaymentMerchantInfo as ::windows::core::Abi>::Abi as *const <PaymentMerchantInfo as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMerchantInfoAndOptions<Impl: IPaymentRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, details: ::windows::core::RawPtr, methoddata: ::windows::core::RawPtr, merchantinfo: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMerchantInfoAndOptions(
                &*(&details as *const <PaymentDetails as ::windows::core::Abi>::Abi as *const <PaymentDetails as ::windows::core::DefaultType>::DefaultType),
                &*(&methoddata as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::DefaultType>::DefaultType),
                &*(&merchantinfo as *const <PaymentMerchantInfo as ::windows::core::Abi>::Abi as *const <PaymentMerchantInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <PaymentOptions as ::windows::core::Abi>::Abi as *const <PaymentOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequestFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithMerchantInfo::<Impl, IMPL_OFFSET>, CreateWithMerchantInfoAndOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentRequestFactory2Impl: Sized {
    fn CreateWithMerchantInfoOptionsAndId(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>, options: &::core::option::Option<PaymentOptions>, id: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentRequest>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentRequestFactory2 {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequestFactory2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPaymentRequestFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestFactory2Vtbl {
        unsafe extern "system" fn CreateWithMerchantInfoOptionsAndId<Impl: IPaymentRequestFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, details: ::windows::core::RawPtr, methoddata: ::windows::core::RawPtr, merchantinfo: ::windows::core::RawPtr, options: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMerchantInfoOptionsAndId(
                &*(&details as *const <PaymentDetails as ::windows::core::Abi>::Abi as *const <PaymentDetails as ::windows::core::DefaultType>::DefaultType),
                &*(&methoddata as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PaymentMethodData> as ::windows::core::DefaultType>::DefaultType),
                &*(&merchantinfo as *const <PaymentMerchantInfo as ::windows::core::Abi>::Abi as *const <PaymentMerchantInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <PaymentOptions as ::windows::core::Abi>::Abi as *const <PaymentOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequestFactory2>, ::windows::core::GetTrustLevel, CreateWithMerchantInfoOptionsAndId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestSubmitResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PaymentRequestStatus>;
    fn Response(&self) -> ::windows::core::Result<PaymentResponse>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentRequestSubmitResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentRequestSubmitResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentRequestSubmitResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentRequestSubmitResultVtbl {
        unsafe extern "system" fn Status<Impl: IPaymentRequestSubmitResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Response<Impl: IPaymentRequestSubmitResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentRequestSubmitResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, Response::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestSubmitResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentResponseImpl: Sized {
    fn PaymentToken(&self) -> ::windows::core::Result<PaymentToken>;
    fn ShippingOption(&self) -> ::windows::core::Result<PaymentShippingOption>;
    fn ShippingAddress(&self) -> ::windows::core::Result<PaymentAddress>;
    fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaymentResponse {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentResponse";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPaymentResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentResponseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentResponseVtbl {
        unsafe extern "system" fn PaymentToken<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaymentToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShippingOption<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShippingOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShippingAddress<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShippingAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayerEmail<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PayerName<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PayerPhoneNumber<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompleteAsync<Impl: IPaymentResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: PaymentRequestCompletionStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteAsync(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPaymentResponse>,
            ::windows::core::GetTrustLevel,
            PaymentToken::<Impl, IMPL_OFFSET>,
            ShippingOption::<Impl, IMPL_OFFSET>,
            ShippingAddress::<Impl, IMPL_OFFSET>,
            PayerEmail::<Impl, IMPL_OFFSET>,
            PayerName::<Impl, IMPL_OFFSET>,
            PayerPhoneNumber::<Impl, IMPL_OFFSET>,
            CompleteAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentShippingOptionImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Amount(&self) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn SetAmount(&self, value: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentShippingOption {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentShippingOption";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentShippingOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentShippingOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentShippingOptionVtbl {
        unsafe extern "system" fn Label<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Amount<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Amount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmount<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAmount(&*(&value as *const <PaymentCurrencyAmount as ::windows::core::Abi>::Abi as *const <PaymentCurrencyAmount as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSelected<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSelected<Impl: IPaymentShippingOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSelected(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPaymentShippingOption>,
            ::windows::core::GetTrustLevel,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            Amount::<Impl, IMPL_OFFSET>,
            SetAmount::<Impl, IMPL_OFFSET>,
            Tag::<Impl, IMPL_OFFSET>,
            SetTag::<Impl, IMPL_OFFSET>,
            IsSelected::<Impl, IMPL_OFFSET>,
            SetIsSelected::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentShippingOption as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentShippingOptionFactoryImpl: Sized {
    fn Create(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<PaymentShippingOption>;
    fn CreateWithSelected(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>, selected: bool) -> ::windows::core::Result<PaymentShippingOption>;
    fn CreateWithSelectedAndTag(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>, selected: bool, tag: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentShippingOption>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentShippingOptionFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentShippingOptionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentShippingOptionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentShippingOptionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentShippingOptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&amount as *const <PaymentCurrencyAmount as ::windows::core::Abi>::Abi as *const <PaymentCurrencyAmount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSelected<Impl: IPaymentShippingOptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: ::windows::core::RawPtr, selected: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithSelected(&*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&amount as *const <PaymentCurrencyAmount as ::windows::core::Abi>::Abi as *const <PaymentCurrencyAmount as ::windows::core::DefaultType>::DefaultType), selected) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSelectedAndTag<Impl: IPaymentShippingOptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, amount: ::windows::core::RawPtr, selected: bool, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithSelectedAndTag(
                &*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&amount as *const <PaymentCurrencyAmount as ::windows::core::Abi>::Abi as *const <PaymentCurrencyAmount as ::windows::core::DefaultType>::DefaultType),
                selected,
                &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentShippingOptionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithSelected::<Impl, IMPL_OFFSET>, CreateWithSelectedAndTag::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentShippingOptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTokenImpl: Sized {
    fn PaymentMethodId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JsonDetails(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentToken {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentToken";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentTokenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentTokenImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentTokenVtbl {
        unsafe extern "system" fn PaymentMethodId<Impl: IPaymentTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaymentMethodId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JsonDetails<Impl: IPaymentTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JsonDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentToken>, ::windows::core::GetTrustLevel, PaymentMethodId::<Impl, IMPL_OFFSET>, JsonDetails::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentToken as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTokenFactoryImpl: Sized {
    fn Create(&self, paymentmethodid: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken>;
    fn CreateWithJsonDetails(&self, paymentmethodid: &::windows::core::HSTRING, jsondetails: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaymentTokenFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.IPaymentTokenFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPaymentTokenFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaymentTokenFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaymentTokenFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPaymentTokenFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paymentmethodid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&paymentmethodid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithJsonDetails<Impl: IPaymentTokenFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paymentmethodid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, jsondetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithJsonDetails(&*(&paymentmethodid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&jsondetails as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaymentTokenFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithJsonDetails::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentTokenFactory as ::windows::core::Interface>::IID
    }
}
