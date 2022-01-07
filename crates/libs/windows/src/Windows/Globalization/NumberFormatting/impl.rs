#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyFormatterImpl: Sized + INumberFormatterImpl + INumberFormatter2Impl + INumberFormatterOptionsImpl + INumberParserImpl {
    fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrency(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ICurrencyFormatter";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrencyFormatterVtbl {
    pub const fn new<Impl: ICurrencyFormatterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICurrencyFormatterVtbl {
        unsafe extern "system" fn Currency<Impl: ICurrencyFormatterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Currency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrency<Impl: ICurrencyFormatterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurrency(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICurrencyFormatter>, base.5, Currency::<Impl, OFFSET>, SetCurrency::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICurrencyFormatter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICurrencyFormatter2Vtbl {
        unsafe extern "system" fn Mode<Impl: ICurrencyFormatter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CurrencyFormatterMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: ICurrencyFormatter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CurrencyFormatterMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn ApplyRoundingForCurrency<Impl: ICurrencyFormatter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ApplyRoundingForCurrency(roundingalgorithm).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICurrencyFormatter2>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>, ApplyRoundingForCurrency::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyFormatterFactoryImpl: Sized {
    fn CreateCurrencyFormatterCode(&self, currencycode: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyFormatter>;
    fn CreateCurrencyFormatterCodeContext(&self, currencycode: &::windows::core::HSTRING, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyFormatter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrencyFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ICurrencyFormatterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrencyFormatterFactoryVtbl {
    pub const fn new<Impl: ICurrencyFormatterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICurrencyFormatterFactoryVtbl {
        unsafe extern "system" fn CreateCurrencyFormatterCode<Impl: ICurrencyFormatterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCurrencyFormatterCode(&*(&currencycode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCurrencyFormatterCodeContext<Impl: ICurrencyFormatterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICurrencyFormatterFactory>, base.5, CreateCurrencyFormatterCode::<Impl, OFFSET>, CreateCurrencyFormatterCodeContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDecimalFormatterFactoryImpl: Sized {
    fn CreateDecimalFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<DecimalFormatter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDecimalFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IDecimalFormatterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDecimalFormatterFactoryVtbl {
    pub const fn new<Impl: IDecimalFormatterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDecimalFormatterFactoryVtbl {
        unsafe extern "system" fn CreateDecimalFormatter<Impl: IDecimalFormatterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDecimalFormatter(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDecimalFormatterFactory>, base.5, CreateDecimalFormatter::<Impl, OFFSET>)
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
    pub const fn new<Impl: IIncrementNumberRounderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIncrementNumberRounderVtbl {
        unsafe extern "system" fn RoundingAlgorithm<Impl: IIncrementNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoundingAlgorithm<Impl: IIncrementNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoundingAlgorithm(value).into()
        }
        unsafe extern "system" fn Increment<Impl: IIncrementNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Increment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncrement<Impl: IIncrementNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIncrement(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIncrementNumberRounder>, base.5, RoundingAlgorithm::<Impl, OFFSET>, SetRoundingAlgorithm::<Impl, OFFSET>, Increment::<Impl, OFFSET>, SetIncrement::<Impl, OFFSET>)
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
    pub const fn new<Impl: INumberFormatterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumberFormatterVtbl {
        unsafe extern "system" fn FormatInt<Impl: INumberFormatterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Impl: INumberFormatterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatUInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Impl: INumberFormatterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumberFormatter>, base.5, FormatInt::<Impl, OFFSET>, FormatUInt::<Impl, OFFSET>, FormatDouble::<Impl, OFFSET>)
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
    pub const fn new<Impl: INumberFormatter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumberFormatter2Vtbl {
        unsafe extern "system" fn FormatInt<Impl: INumberFormatter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Impl: INumberFormatter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatUInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Impl: INumberFormatter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumberFormatter2>, base.5, FormatInt::<Impl, OFFSET>, FormatUInt::<Impl, OFFSET>, FormatDouble::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for INumberFormatterOptions {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatterOptions";
}
impl INumberFormatterOptionsVtbl {
    pub const fn new<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumberFormatterOptionsVtbl {
        unsafe extern "system" fn Languages<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeographicRegion<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntegerDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IntegerDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntegerDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIntegerDigits(value).into()
        }
        unsafe extern "system" fn FractionDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFractionDigits<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFractionDigits(value).into()
        }
        unsafe extern "system" fn IsGrouped<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGrouped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGrouped<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsGrouped(value).into()
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDecimalPointAlwaysDisplayed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDecimalPointAlwaysDisplayed(value).into()
        }
        unsafe extern "system" fn NumeralSystem<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumeralSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNumeralSystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolvedLanguage<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedGeographicRegion<Impl: INumberFormatterOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<INumberFormatterOptions>,
            base.5,
            Languages::<Impl, OFFSET>,
            GeographicRegion::<Impl, OFFSET>,
            IntegerDigits::<Impl, OFFSET>,
            SetIntegerDigits::<Impl, OFFSET>,
            FractionDigits::<Impl, OFFSET>,
            SetFractionDigits::<Impl, OFFSET>,
            IsGrouped::<Impl, OFFSET>,
            SetIsGrouped::<Impl, OFFSET>,
            IsDecimalPointAlwaysDisplayed::<Impl, OFFSET>,
            SetIsDecimalPointAlwaysDisplayed::<Impl, OFFSET>,
            NumeralSystem::<Impl, OFFSET>,
            SetNumeralSystem::<Impl, OFFSET>,
            ResolvedLanguage::<Impl, OFFSET>,
            ResolvedGeographicRegion::<Impl, OFFSET>,
        )
    }
}
pub trait INumberParserImpl: Sized {
    fn ParseInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
impl ::windows::core::RuntimeName for INumberParser {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberParser";
}
impl INumberParserVtbl {
    pub const fn new<Impl: INumberParserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumberParserVtbl {
        unsafe extern "system" fn ParseInt<Impl: INumberParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParseInt(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseUInt<Impl: INumberParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParseUInt(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDouble<Impl: INumberParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParseDouble(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumberParser>, base.5, ParseInt::<Impl, OFFSET>, ParseUInt::<Impl, OFFSET>, ParseDouble::<Impl, OFFSET>)
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
    pub const fn new<Impl: INumberRounderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumberRounderVtbl {
        unsafe extern "system" fn RoundInt32<Impl: INumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt32<Impl: INumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundUInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundInt64<Impl: INumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt64<Impl: INumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundUInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundSingle<Impl: INumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundSingle(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundDouble<Impl: INumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumberRounder>, base.5, RoundInt32::<Impl, OFFSET>, RoundUInt32::<Impl, OFFSET>, RoundInt64::<Impl, OFFSET>, RoundUInt64::<Impl, OFFSET>, RoundSingle::<Impl, OFFSET>, RoundDouble::<Impl, OFFSET>)
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
    pub const fn new<Impl: INumberRounderOptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumberRounderOptionVtbl {
        unsafe extern "system" fn NumberRounder<Impl: INumberRounderOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberRounder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberRounder<Impl: INumberRounderOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNumberRounder(&*(&value as *const <INumberRounder as ::windows::core::Abi>::Abi as *const <INumberRounder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumberRounderOption>, base.5, NumberRounder::<Impl, OFFSET>, SetNumberRounder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INumeralSystemTranslatorImpl: Sized {
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TranslateNumerals(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumeralSystemTranslator";
}
#[cfg(feature = "implement_exclusive")]
impl INumeralSystemTranslatorVtbl {
    pub const fn new<Impl: INumeralSystemTranslatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumeralSystemTranslatorVtbl {
        unsafe extern "system" fn Languages<Impl: INumeralSystemTranslatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedLanguage<Impl: INumeralSystemTranslatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumeralSystem<Impl: INumeralSystemTranslatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumeralSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Impl: INumeralSystemTranslatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNumeralSystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslateNumerals<Impl: INumeralSystemTranslatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateNumerals(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumeralSystemTranslator>, base.5, Languages::<Impl, OFFSET>, ResolvedLanguage::<Impl, OFFSET>, NumeralSystem::<Impl, OFFSET>, SetNumeralSystem::<Impl, OFFSET>, TranslateNumerals::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INumeralSystemTranslatorFactoryImpl: Sized {
    fn Create(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<NumeralSystemTranslator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INumeralSystemTranslatorFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumeralSystemTranslatorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INumeralSystemTranslatorFactoryVtbl {
    pub const fn new<Impl: INumeralSystemTranslatorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INumeralSystemTranslatorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: INumeralSystemTranslatorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INumeralSystemTranslatorFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPercentFormatterFactoryImpl: Sized {
    fn CreatePercentFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<PercentFormatter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPercentFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IPercentFormatterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPercentFormatterFactoryVtbl {
    pub const fn new<Impl: IPercentFormatterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPercentFormatterFactoryVtbl {
        unsafe extern "system" fn CreatePercentFormatter<Impl: IPercentFormatterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePercentFormatter(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPercentFormatterFactory>, base.5, CreatePercentFormatter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPermilleFormatterFactoryImpl: Sized {
    fn CreatePermilleFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<PermilleFormatter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPermilleFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IPermilleFormatterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPermilleFormatterFactoryVtbl {
    pub const fn new<Impl: IPermilleFormatterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPermilleFormatterFactoryVtbl {
        unsafe extern "system" fn CreatePermilleFormatter<Impl: IPermilleFormatterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePermilleFormatter(&*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPermilleFormatterFactory>, base.5, CreatePermilleFormatter::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISignedZeroOptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISignedZeroOptionVtbl {
        unsafe extern "system" fn IsZeroSigned<Impl: ISignedZeroOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsZeroSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZeroSigned<Impl: ISignedZeroOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsZeroSigned(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISignedZeroOption>, base.5, IsZeroSigned::<Impl, OFFSET>, SetIsZeroSigned::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISignificantDigitsNumberRounderVtbl {
        unsafe extern "system" fn RoundingAlgorithm<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoundingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoundingAlgorithm<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoundingAlgorithm(value).into()
        }
        unsafe extern "system" fn SignificantDigits<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignificantDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Impl: ISignificantDigitsNumberRounderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSignificantDigits(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISignificantDigitsNumberRounder>, base.5, RoundingAlgorithm::<Impl, OFFSET>, SetRoundingAlgorithm::<Impl, OFFSET>, SignificantDigits::<Impl, OFFSET>, SetSignificantDigits::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISignificantDigitsOptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISignificantDigitsOptionVtbl {
        unsafe extern "system" fn SignificantDigits<Impl: ISignificantDigitsOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignificantDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Impl: ISignificantDigitsOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSignificantDigits(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISignificantDigitsOption>, base.5, SignificantDigits::<Impl, OFFSET>, SetSignificantDigits::<Impl, OFFSET>)
    }
}
