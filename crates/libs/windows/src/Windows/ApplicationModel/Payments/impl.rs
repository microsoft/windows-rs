#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentAddressImpl: Sized {
    fn Country(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddressLines(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetAddressLines(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn Region(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn City(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCity(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DependentLocality(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDependentLocality(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SortingCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSortingCode(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LanguageCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguageCode(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Organization(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOrganization(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Recipient(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRecipient(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhoneNumber(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentAddress, BASE_OFFSET>(),
            Country: Country::<Impl, IMPL_OFFSET>,
            SetCountry: SetCountry::<Impl, IMPL_OFFSET>,
            AddressLines: AddressLines::<Impl, IMPL_OFFSET>,
            SetAddressLines: SetAddressLines::<Impl, IMPL_OFFSET>,
            Region: Region::<Impl, IMPL_OFFSET>,
            SetRegion: SetRegion::<Impl, IMPL_OFFSET>,
            City: City::<Impl, IMPL_OFFSET>,
            SetCity: SetCity::<Impl, IMPL_OFFSET>,
            DependentLocality: DependentLocality::<Impl, IMPL_OFFSET>,
            SetDependentLocality: SetDependentLocality::<Impl, IMPL_OFFSET>,
            PostalCode: PostalCode::<Impl, IMPL_OFFSET>,
            SetPostalCode: SetPostalCode::<Impl, IMPL_OFFSET>,
            SortingCode: SortingCode::<Impl, IMPL_OFFSET>,
            SetSortingCode: SetSortingCode::<Impl, IMPL_OFFSET>,
            LanguageCode: LanguageCode::<Impl, IMPL_OFFSET>,
            SetLanguageCode: SetLanguageCode::<Impl, IMPL_OFFSET>,
            Organization: Organization::<Impl, IMPL_OFFSET>,
            SetOrganization: SetOrganization::<Impl, IMPL_OFFSET>,
            Recipient: Recipient::<Impl, IMPL_OFFSET>,
            SetRecipient: SetRecipient::<Impl, IMPL_OFFSET>,
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            SetPhoneNumber: SetPhoneNumber::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCanMakePaymentResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PaymentCanMakePaymentResultStatus>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentCanMakePaymentResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCanMakePaymentResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCanMakePaymentResultFactoryImpl: Sized {
    fn Create(&mut self, value: PaymentCanMakePaymentResultStatus) -> ::windows::core::Result<PaymentCanMakePaymentResult>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentCanMakePaymentResultFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCanMakePaymentResultFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCurrencyAmountImpl: Sized {
    fn Currency(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrency(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CurrencySystem(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrencySystem(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentCurrencyAmount, BASE_OFFSET>(),
            Currency: Currency::<Impl, IMPL_OFFSET>,
            SetCurrency: SetCurrency::<Impl, IMPL_OFFSET>,
            CurrencySystem: CurrencySystem::<Impl, IMPL_OFFSET>,
            SetCurrencySystem: SetCurrencySystem::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCurrencyAmount as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCurrencyAmountFactoryImpl: Sized {
    fn Create(&mut self, value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn CreateWithCurrencySystem(&mut self, value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING, currencysystem: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentCurrencyAmountFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithCurrencySystem: CreateWithCurrencySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentCurrencyAmountFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsImpl: Sized {
    fn Total(&mut self) -> ::windows::core::Result<PaymentItem>;
    fn SetTotal(&mut self, value: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<()>;
    fn DisplayItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>>;
    fn SetDisplayItems(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentItem>>) -> ::windows::core::Result<()>;
    fn ShippingOptions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>;
    fn SetShippingOptions(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>) -> ::windows::core::Result<()>;
    fn Modifiers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>;
    fn SetModifiers(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentDetails, BASE_OFFSET>(),
            Total: Total::<Impl, IMPL_OFFSET>,
            SetTotal: SetTotal::<Impl, IMPL_OFFSET>,
            DisplayItems: DisplayItems::<Impl, IMPL_OFFSET>,
            SetDisplayItems: SetDisplayItems::<Impl, IMPL_OFFSET>,
            ShippingOptions: ShippingOptions::<Impl, IMPL_OFFSET>,
            SetShippingOptions: SetShippingOptions::<Impl, IMPL_OFFSET>,
            Modifiers: Modifiers::<Impl, IMPL_OFFSET>,
            SetModifiers: SetModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsFactoryImpl: Sized {
    fn Create(&mut self, total: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<PaymentDetails>;
    fn CreateWithDisplayItems(&mut self, total: &::core::option::Option<PaymentItem>, displayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>) -> ::windows::core::Result<PaymentDetails>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentDetailsFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithDisplayItems: CreateWithDisplayItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetailsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsModifierImpl: Sized {
    fn JsonData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedMethodIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Total(&mut self) -> ::windows::core::Result<PaymentItem>;
    fn AdditionalDisplayItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentDetailsModifier, BASE_OFFSET>(),
            JsonData: JsonData::<Impl, IMPL_OFFSET>,
            SupportedMethodIds: SupportedMethodIds::<Impl, IMPL_OFFSET>,
            Total: Total::<Impl, IMPL_OFFSET>,
            AdditionalDisplayItems: AdditionalDisplayItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetailsModifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentDetailsModifierFactoryImpl: Sized {
    fn Create(&mut self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<PaymentDetailsModifier>;
    fn CreateWithAdditionalDisplayItems(&mut self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>, additionaldisplayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>) -> ::windows::core::Result<PaymentDetailsModifier>;
    fn CreateWithAdditionalDisplayItemsAndJsonData(&mut self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>, additionaldisplayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentDetailsModifier>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentDetailsModifierFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAdditionalDisplayItems: CreateWithAdditionalDisplayItems::<Impl, IMPL_OFFSET>,
            CreateWithAdditionalDisplayItemsAndJsonData: CreateWithAdditionalDisplayItemsAndJsonData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentDetailsModifierFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentItemImpl: Sized {
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Amount(&mut self) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn SetAmount(&mut self, value: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<()>;
    fn Pending(&mut self) -> ::windows::core::Result<bool>;
    fn SetPending(&mut self, value: bool) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentItem, BASE_OFFSET>(),
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            Amount: Amount::<Impl, IMPL_OFFSET>,
            SetAmount: SetAmount::<Impl, IMPL_OFFSET>,
            Pending: Pending::<Impl, IMPL_OFFSET>,
            SetPending: SetPending::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentItemFactoryImpl: Sized {
    fn Create(&mut self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<PaymentItem>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentItemFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentItemFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentMediatorImpl: Sized {
    fn GetSupportedMethodIdsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SubmitPaymentRequestAsync(&mut self, paymentrequest: &::core::option::Option<PaymentRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>;
    fn SubmitPaymentRequestWithChangeHandlerAsync(&mut self, paymentrequest: &::core::option::Option<PaymentRequest>, changehandler: &::core::option::Option<PaymentRequestChangedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentMediator, BASE_OFFSET>(),
            GetSupportedMethodIdsAsync: GetSupportedMethodIdsAsync::<Impl, IMPL_OFFSET>,
            SubmitPaymentRequestAsync: SubmitPaymentRequestAsync::<Impl, IMPL_OFFSET>,
            SubmitPaymentRequestWithChangeHandlerAsync: SubmitPaymentRequestWithChangeHandlerAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMediator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentMediator2Impl: Sized {
    fn CanMakePaymentAsync(&mut self, paymentrequest: &::core::option::Option<PaymentRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentMediator2, BASE_OFFSET>(),
            CanMakePaymentAsync: CanMakePaymentAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMediator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentMerchantInfoImpl: Sized {
    fn PackageFullName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentMerchantInfo, BASE_OFFSET>(),
            PackageFullName: PackageFullName::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMerchantInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentMerchantInfoFactoryImpl: Sized {
    fn Create(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<PaymentMerchantInfo>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentMerchantInfoFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMerchantInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentMethodDataImpl: Sized {
    fn SupportedMethodIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn JsonData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentMethodData, BASE_OFFSET>(),
            SupportedMethodIds: SupportedMethodIds::<Impl, IMPL_OFFSET>,
            JsonData: JsonData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMethodData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentMethodDataFactoryImpl: Sized {
    fn Create(&mut self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PaymentMethodData>;
    fn CreateWithJsonData(&mut self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentMethodData>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentMethodDataFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithJsonData: CreateWithJsonData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentMethodDataFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentOptionsImpl: Sized {
    fn RequestPayerEmail(&mut self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerEmail(&mut self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestPayerName(&mut self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerName(&mut self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestPayerPhoneNumber(&mut self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerPhoneNumber(&mut self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestShipping(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequestShipping(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ShippingType(&mut self) -> ::windows::core::Result<PaymentShippingType>;
    fn SetShippingType(&mut self, value: PaymentShippingType) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentOptions, BASE_OFFSET>(),
            RequestPayerEmail: RequestPayerEmail::<Impl, IMPL_OFFSET>,
            SetRequestPayerEmail: SetRequestPayerEmail::<Impl, IMPL_OFFSET>,
            RequestPayerName: RequestPayerName::<Impl, IMPL_OFFSET>,
            SetRequestPayerName: SetRequestPayerName::<Impl, IMPL_OFFSET>,
            RequestPayerPhoneNumber: RequestPayerPhoneNumber::<Impl, IMPL_OFFSET>,
            SetRequestPayerPhoneNumber: SetRequestPayerPhoneNumber::<Impl, IMPL_OFFSET>,
            RequestShipping: RequestShipping::<Impl, IMPL_OFFSET>,
            SetRequestShipping: SetRequestShipping::<Impl, IMPL_OFFSET>,
            ShippingType: ShippingType::<Impl, IMPL_OFFSET>,
            SetShippingType: SetShippingType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentRequestImpl: Sized {
    fn MerchantInfo(&mut self) -> ::windows::core::Result<PaymentMerchantInfo>;
    fn Details(&mut self) -> ::windows::core::Result<PaymentDetails>;
    fn MethodData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>>;
    fn Options(&mut self) -> ::windows::core::Result<PaymentOptions>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequest, BASE_OFFSET>(),
            MerchantInfo: MerchantInfo::<Impl, IMPL_OFFSET>,
            Details: Details::<Impl, IMPL_OFFSET>,
            MethodData: MethodData::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequest2Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequest2, BASE_OFFSET>(), Id: Id::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedArgsImpl: Sized {
    fn ChangeKind(&mut self) -> ::windows::core::Result<PaymentRequestChangeKind>;
    fn ShippingAddress(&mut self) -> ::windows::core::Result<PaymentAddress>;
    fn SelectedShippingOption(&mut self) -> ::windows::core::Result<PaymentShippingOption>;
    fn Acknowledge(&mut self, changeresult: &::core::option::Option<PaymentRequestChangedResult>) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequestChangedArgs, BASE_OFFSET>(),
            ChangeKind: ChangeKind::<Impl, IMPL_OFFSET>,
            ShippingAddress: ShippingAddress::<Impl, IMPL_OFFSET>,
            SelectedShippingOption: SelectedShippingOption::<Impl, IMPL_OFFSET>,
            Acknowledge: Acknowledge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestChangedArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedResultImpl: Sized {
    fn ChangeAcceptedByMerchant(&mut self) -> ::windows::core::Result<bool>;
    fn SetChangeAcceptedByMerchant(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdatedPaymentDetails(&mut self) -> ::windows::core::Result<PaymentDetails>;
    fn SetUpdatedPaymentDetails(&mut self, value: &::core::option::Option<PaymentDetails>) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequestChangedResult, BASE_OFFSET>(),
            ChangeAcceptedByMerchant: ChangeAcceptedByMerchant::<Impl, IMPL_OFFSET>,
            SetChangeAcceptedByMerchant: SetChangeAcceptedByMerchant::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            SetMessage: SetMessage::<Impl, IMPL_OFFSET>,
            UpdatedPaymentDetails: UpdatedPaymentDetails::<Impl, IMPL_OFFSET>,
            SetUpdatedPaymentDetails: SetUpdatedPaymentDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestChangedResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedResultFactoryImpl: Sized {
    fn Create(&mut self, changeacceptedbymerchant: bool) -> ::windows::core::Result<PaymentRequestChangedResult>;
    fn CreateWithPaymentDetails(&mut self, changeacceptedbymerchant: bool, updatedpaymentdetails: &::core::option::Option<PaymentDetails>) -> ::windows::core::Result<PaymentRequestChangedResult>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequestChangedResultFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithPaymentDetails: CreateWithPaymentDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestChangedResultFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentRequestFactoryImpl: Sized {
    fn Create(&mut self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>) -> ::windows::core::Result<PaymentRequest>;
    fn CreateWithMerchantInfo(&mut self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>) -> ::windows::core::Result<PaymentRequest>;
    fn CreateWithMerchantInfoAndOptions(&mut self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>, options: &::core::option::Option<PaymentOptions>) -> ::windows::core::Result<PaymentRequest>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequestFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithMerchantInfo: CreateWithMerchantInfo::<Impl, IMPL_OFFSET>,
            CreateWithMerchantInfoAndOptions: CreateWithMerchantInfoAndOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPaymentRequestFactory2Impl: Sized {
    fn CreateWithMerchantInfoOptionsAndId(&mut self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>, options: &::core::option::Option<PaymentOptions>, id: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentRequest>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequestFactory2, BASE_OFFSET>(),
            CreateWithMerchantInfoOptionsAndId: CreateWithMerchantInfoOptionsAndId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestSubmitResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PaymentRequestStatus>;
    fn Response(&mut self) -> ::windows::core::Result<PaymentResponse>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentRequestSubmitResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Response: Response::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentRequestSubmitResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPaymentResponseImpl: Sized {
    fn PaymentToken(&mut self) -> ::windows::core::Result<PaymentToken>;
    fn ShippingOption(&mut self) -> ::windows::core::Result<PaymentShippingOption>;
    fn ShippingAddress(&mut self) -> ::windows::core::Result<PaymentAddress>;
    fn PayerEmail(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PayerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PayerPhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompleteAsync(&mut self, status: PaymentRequestCompletionStatus) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentResponse, BASE_OFFSET>(),
            PaymentToken: PaymentToken::<Impl, IMPL_OFFSET>,
            ShippingOption: ShippingOption::<Impl, IMPL_OFFSET>,
            ShippingAddress: ShippingAddress::<Impl, IMPL_OFFSET>,
            PayerEmail: PayerEmail::<Impl, IMPL_OFFSET>,
            PayerName: PayerName::<Impl, IMPL_OFFSET>,
            PayerPhoneNumber: PayerPhoneNumber::<Impl, IMPL_OFFSET>,
            CompleteAsync: CompleteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentShippingOptionImpl: Sized {
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Amount(&mut self) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn SetAmount(&mut self, value: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<()>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTag(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSelected(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&mut self, value: bool) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentShippingOption, BASE_OFFSET>(),
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            Amount: Amount::<Impl, IMPL_OFFSET>,
            SetAmount: SetAmount::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            SetIsSelected: SetIsSelected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentShippingOption as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentShippingOptionFactoryImpl: Sized {
    fn Create(&mut self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<PaymentShippingOption>;
    fn CreateWithSelected(&mut self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>, selected: bool) -> ::windows::core::Result<PaymentShippingOption>;
    fn CreateWithSelectedAndTag(&mut self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>, selected: bool, tag: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentShippingOption>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentShippingOptionFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithSelected: CreateWithSelected::<Impl, IMPL_OFFSET>,
            CreateWithSelectedAndTag: CreateWithSelectedAndTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentShippingOptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTokenImpl: Sized {
    fn PaymentMethodId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JsonDetails(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentToken, BASE_OFFSET>(),
            PaymentMethodId: PaymentMethodId::<Impl, IMPL_OFFSET>,
            JsonDetails: JsonDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentToken as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTokenFactoryImpl: Sized {
    fn Create(&mut self, paymentmethodid: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken>;
    fn CreateWithJsonDetails(&mut self, paymentmethodid: &::windows::core::HSTRING, jsondetails: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaymentTokenFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithJsonDetails: CreateWithJsonDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaymentTokenFactory as ::windows::core::Interface>::IID
    }
}
