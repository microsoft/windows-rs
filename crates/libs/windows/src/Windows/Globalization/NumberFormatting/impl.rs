#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICurrencyFormatterImpl: Sized + INumberFormatterImpl + INumberFormatter2Impl + INumberFormatterOptionsImpl + INumberParserImpl {
    fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrency(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ICurrencyFormatter";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICurrencyFormatterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrencyFormatterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrencyFormatterVtbl {
        unsafe extern "system" fn Currency<Impl: ICurrencyFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCurrency<Impl: ICurrencyFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrency(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICurrencyFormatter>, ::windows::core::GetTrustLevel, Currency::<Impl, IMPL_OFFSET>, SetCurrency::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrencyFormatter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyFormatter2Impl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CurrencyFormatterMode>;
    fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows::core::Result<()>;
    fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrencyFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ICurrencyFormatter2";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrencyFormatter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrencyFormatter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrencyFormatter2Vtbl {
        unsafe extern "system" fn Mode<Impl: ICurrencyFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CurrencyFormatterMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: ICurrencyFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CurrencyFormatterMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn ApplyRoundingForCurrency<Impl: ICurrencyFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyRoundingForCurrency(roundingalgorithm).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICurrencyFormatter2>, ::windows::core::GetTrustLevel, Mode::<Impl, IMPL_OFFSET>, SetMode::<Impl, IMPL_OFFSET>, ApplyRoundingForCurrency::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrencyFormatter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICurrencyFormatterFactoryImpl: Sized {
    fn CreateCurrencyFormatterCode(&self, currencycode: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyFormatter>;
    fn CreateCurrencyFormatterCodeContext(&self, currencycode: &::windows::core::HSTRING, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyFormatter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrencyFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICurrencyFormatterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrencyFormatterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrencyFormatterFactoryVtbl {
        unsafe extern "system" fn CreateCurrencyFormatterCode<Impl: ICurrencyFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCurrencyFormatterCode(&*(&currencycode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCurrencyFormatterCodeContext<Impl: ICurrencyFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCurrencyFormatterCodeContext(
                &*(&currencycode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICurrencyFormatterFactory>, ::windows::core::GetTrustLevel, CreateCurrencyFormatterCode::<Impl, IMPL_OFFSET>, CreateCurrencyFormatterCodeContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrencyFormatterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDecimalFormatterFactoryImpl: Sized {
    fn CreateDecimalFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<DecimalFormatter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDecimalFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IDecimalFormatterFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDecimalFormatterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDecimalFormatterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDecimalFormatterFactoryVtbl {
        unsafe extern "system" fn CreateDecimalFormatter<Impl: IDecimalFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecimalFormatter(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDecimalFormatterFactory>, ::windows::core::GetTrustLevel, CreateDecimalFormatter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDecimalFormatterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIncrementNumberRounderImpl: Sized {
    fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm>;
    fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()>;
    fn Increment(&self) -> ::windows::core::Result<f64>;
    fn SetIncrement(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IIncrementNumberRounder";
}
#[cfg(feature = "implement_exclusive")]
impl IIncrementNumberRounderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIncrementNumberRounderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIncrementNumberRounderVtbl {
        unsafe extern "system" fn RoundingAlgorithm<Impl: IIncrementNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoundingAlgorithm<Impl: IIncrementNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoundingAlgorithm(value).into()
        }
        unsafe extern "system" fn Increment<Impl: IIncrementNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Increment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncrement<Impl: IIncrementNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncrement(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIncrementNumberRounder>, ::windows::core::GetTrustLevel, RoundingAlgorithm::<Impl, IMPL_OFFSET>, SetRoundingAlgorithm::<Impl, IMPL_OFFSET>, Increment::<Impl, IMPL_OFFSET>, SetIncrement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIncrementNumberRounder as ::windows::core::Interface>::IID
    }
}
pub trait INumberFormatterImpl: Sized {
    fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for INumberFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter";
}
impl INumberFormatterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberFormatterVtbl {
        unsafe extern "system" fn FormatInt<Impl: INumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Impl: INumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatUInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Impl: INumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumberFormatter>, ::windows::core::GetTrustLevel, FormatInt::<Impl, IMPL_OFFSET>, FormatUInt::<Impl, IMPL_OFFSET>, FormatDouble::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatter as ::windows::core::Interface>::IID
    }
}
pub trait INumberFormatter2Impl: Sized {
    fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for INumberFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter2";
}
impl INumberFormatter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberFormatter2Vtbl {
        unsafe extern "system" fn FormatInt<Impl: INumberFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Impl: INumberFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatUInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Impl: INumberFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumberFormatter2>, ::windows::core::GetTrustLevel, FormatInt::<Impl, IMPL_OFFSET>, FormatUInt::<Impl, IMPL_OFFSET>, FormatDouble::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait INumberFormatterOptionsImpl: Sized {
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IntegerDigits(&self) -> ::windows::core::Result<i32>;
    fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()>;
    fn FractionDigits(&self) -> ::windows::core::Result<i32>;
    fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsGrouped(&self) -> ::windows::core::Result<bool>;
    fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for INumberFormatterOptions {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatterOptions";
}
#[cfg(feature = "Foundation_Collections")]
impl INumberFormatterOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberFormatterOptionsVtbl {
        unsafe extern "system" fn Languages<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeographicRegion<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntegerDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntegerDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntegerDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntegerDigits(value).into()
        }
        unsafe extern "system" fn FractionDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFractionDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFractionDigits(value).into()
        }
        unsafe extern "system" fn IsGrouped<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrouped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGrouped<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsGrouped(value).into()
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDecimalPointAlwaysDisplayed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDecimalPointAlwaysDisplayed(value).into()
        }
        unsafe extern "system" fn NumeralSystem<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumeralSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumeralSystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolvedLanguage<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedGeographicRegion<Impl: INumberFormatterOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedGeographicRegion() {
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
            ::windows::core::GetRuntimeClassName::<INumberFormatterOptions>,
            ::windows::core::GetTrustLevel,
            Languages::<Impl, IMPL_OFFSET>,
            GeographicRegion::<Impl, IMPL_OFFSET>,
            IntegerDigits::<Impl, IMPL_OFFSET>,
            SetIntegerDigits::<Impl, IMPL_OFFSET>,
            FractionDigits::<Impl, IMPL_OFFSET>,
            SetFractionDigits::<Impl, IMPL_OFFSET>,
            IsGrouped::<Impl, IMPL_OFFSET>,
            SetIsGrouped::<Impl, IMPL_OFFSET>,
            IsDecimalPointAlwaysDisplayed::<Impl, IMPL_OFFSET>,
            SetIsDecimalPointAlwaysDisplayed::<Impl, IMPL_OFFSET>,
            NumeralSystem::<Impl, IMPL_OFFSET>,
            SetNumeralSystem::<Impl, IMPL_OFFSET>,
            ResolvedLanguage::<Impl, IMPL_OFFSET>,
            ResolvedGeographicRegion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatterOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait INumberParserImpl: Sized {
    fn ParseInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for INumberParser {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberParser";
}
#[cfg(feature = "Foundation")]
impl INumberParserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberParserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberParserVtbl {
        unsafe extern "system" fn ParseInt<Impl: INumberParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseInt(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseUInt<Impl: INumberParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseUInt(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDouble<Impl: INumberParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseDouble(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumberParser>, ::windows::core::GetTrustLevel, ParseInt::<Impl, IMPL_OFFSET>, ParseUInt::<Impl, IMPL_OFFSET>, ParseDouble::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberParser as ::windows::core::Interface>::IID
    }
}
pub trait INumberRounderImpl: Sized {
    fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32>;
    fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32>;
    fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64>;
    fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64>;
    fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32>;
    fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64>;
}
impl ::windows::core::RuntimeName for INumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounder";
}
impl INumberRounderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberRounderVtbl {
        unsafe extern "system" fn RoundInt32<Impl: INumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt32<Impl: INumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundUInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundInt64<Impl: INumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt64<Impl: INumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundUInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundSingle<Impl: INumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundSingle(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundDouble<Impl: INumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumberRounder>, ::windows::core::GetTrustLevel, RoundInt32::<Impl, IMPL_OFFSET>, RoundUInt32::<Impl, IMPL_OFFSET>, RoundInt64::<Impl, IMPL_OFFSET>, RoundUInt64::<Impl, IMPL_OFFSET>, RoundSingle::<Impl, IMPL_OFFSET>, RoundDouble::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberRounder as ::windows::core::Interface>::IID
    }
}
pub trait INumberRounderOptionImpl: Sized {
    fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder>;
    fn SetNumberRounder(&self, value: &::core::option::Option<INumberRounder>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INumberRounderOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounderOption";
}
impl INumberRounderOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounderOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberRounderOptionVtbl {
        unsafe extern "system" fn NumberRounder<Impl: INumberRounderOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberRounder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberRounder<Impl: INumberRounderOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberRounder(&*(&value as *const <INumberRounder as ::windows::core::Abi>::Abi as *const <INumberRounder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumberRounderOption>, ::windows::core::GetTrustLevel, NumberRounder::<Impl, IMPL_OFFSET>, SetNumberRounder::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberRounderOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INumeralSystemTranslatorImpl: Sized {
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TranslateNumerals(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumeralSystemTranslator";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INumeralSystemTranslatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumeralSystemTranslatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumeralSystemTranslatorVtbl {
        unsafe extern "system" fn Languages<Impl: INumeralSystemTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedLanguage<Impl: INumeralSystemTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumeralSystem<Impl: INumeralSystemTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumeralSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Impl: INumeralSystemTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumeralSystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslateNumerals<Impl: INumeralSystemTranslatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateNumerals(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumeralSystemTranslator>, ::windows::core::GetTrustLevel, Languages::<Impl, IMPL_OFFSET>, ResolvedLanguage::<Impl, IMPL_OFFSET>, NumeralSystem::<Impl, IMPL_OFFSET>, SetNumeralSystem::<Impl, IMPL_OFFSET>, TranslateNumerals::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumeralSystemTranslator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INumeralSystemTranslatorFactoryImpl: Sized {
    fn Create(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<NumeralSystemTranslator>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INumeralSystemTranslatorFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INumeralSystemTranslatorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumeralSystemTranslatorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumeralSystemTranslatorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: INumeralSystemTranslatorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INumeralSystemTranslatorFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumeralSystemTranslatorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPercentFormatterFactoryImpl: Sized {
    fn CreatePercentFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<PercentFormatter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPercentFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IPercentFormatterFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPercentFormatterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPercentFormatterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPercentFormatterFactoryVtbl {
        unsafe extern "system" fn CreatePercentFormatter<Impl: IPercentFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePercentFormatter(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPercentFormatterFactory>, ::windows::core::GetTrustLevel, CreatePercentFormatter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPercentFormatterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPermilleFormatterFactoryImpl: Sized {
    fn CreatePermilleFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<PermilleFormatter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPermilleFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IPermilleFormatterFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPermilleFormatterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPermilleFormatterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPermilleFormatterFactoryVtbl {
        unsafe extern "system" fn CreatePermilleFormatter<Impl: IPermilleFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePermilleFormatter(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPermilleFormatterFactory>, ::windows::core::GetTrustLevel, CreatePermilleFormatter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPermilleFormatterFactory as ::windows::core::Interface>::IID
    }
}
pub trait ISignedZeroOptionImpl: Sized {
    fn IsZeroSigned(&self) -> ::windows::core::Result<bool>;
    fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISignedZeroOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignedZeroOption";
}
impl ISignedZeroOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignedZeroOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignedZeroOptionVtbl {
        unsafe extern "system" fn IsZeroSigned<Impl: ISignedZeroOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsZeroSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZeroSigned<Impl: ISignedZeroOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsZeroSigned(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISignedZeroOption>, ::windows::core::GetTrustLevel, IsZeroSigned::<Impl, IMPL_OFFSET>, SetIsZeroSigned::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignedZeroOption as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISignificantDigitsNumberRounderImpl: Sized {
    fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm>;
    fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()>;
    fn SignificantDigits(&self) -> ::windows::core::Result<u32>;
    fn SetSignificantDigits(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignificantDigitsNumberRounder";
}
#[cfg(feature = "implement_exclusive")]
impl ISignificantDigitsNumberRounderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignificantDigitsNumberRounderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignificantDigitsNumberRounderVtbl {
        unsafe extern "system" fn RoundingAlgorithm<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoundingAlgorithm<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoundingAlgorithm(value).into()
        }
        unsafe extern "system" fn SignificantDigits<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignificantDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignificantDigits(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISignificantDigitsNumberRounder>, ::windows::core::GetTrustLevel, RoundingAlgorithm::<Impl, IMPL_OFFSET>, SetRoundingAlgorithm::<Impl, IMPL_OFFSET>, SignificantDigits::<Impl, IMPL_OFFSET>, SetSignificantDigits::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignificantDigitsNumberRounder as ::windows::core::Interface>::IID
    }
}
pub trait ISignificantDigitsOptionImpl: Sized {
    fn SignificantDigits(&self) -> ::windows::core::Result<i32>;
    fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISignificantDigitsOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
}
impl ISignificantDigitsOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignificantDigitsOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignificantDigitsOptionVtbl {
        unsafe extern "system" fn SignificantDigits<Impl: ISignificantDigitsOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignificantDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Impl: ISignificantDigitsOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignificantDigits(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISignificantDigitsOption>, ::windows::core::GetTrustLevel, SignificantDigits::<Impl, IMPL_OFFSET>, SetSignificantDigits::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignificantDigitsOption as ::windows::core::Interface>::IID
    }
}
