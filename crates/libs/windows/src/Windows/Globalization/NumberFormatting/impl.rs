pub trait INumberFormatter_Impl: Sized {
    fn FormatInt(&mut self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&mut self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&mut self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for INumberFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter";
}
impl INumberFormatter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberFormatter_Vtbl {
        unsafe extern "system" fn FormatInt<Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormatUInt<Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormatDouble<Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberFormatter, BASE_OFFSET>(),
            FormatInt: FormatInt::<Impl, IMPL_OFFSET>,
            FormatUInt: FormatUInt::<Impl, IMPL_OFFSET>,
            FormatDouble: FormatDouble::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatter as ::windows::core::Interface>::IID
    }
}
pub trait INumberFormatter2_Impl: Sized {
    fn FormatInt(&mut self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&mut self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&mut self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for INumberFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter2";
}
impl INumberFormatter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberFormatter2_Vtbl {
        unsafe extern "system" fn FormatInt<Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormatUInt<Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormatDouble<Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberFormatter2, BASE_OFFSET>(),
            FormatInt: FormatInt::<Impl, IMPL_OFFSET>,
            FormatUInt: FormatUInt::<Impl, IMPL_OFFSET>,
            FormatDouble: FormatDouble::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait INumberFormatterOptions_Impl: Sized {
    fn Languages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GeographicRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IntegerDigits(&mut self) -> ::windows::core::Result<i32>;
    fn SetIntegerDigits(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn FractionDigits(&mut self) -> ::windows::core::Result<i32>;
    fn SetFractionDigits(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn IsGrouped(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsGrouped(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecimalPointAlwaysDisplayed(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDecimalPointAlwaysDisplayed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn NumeralSystem(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResolvedLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResolvedGeographicRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for INumberFormatterOptions {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatterOptions";
}
#[cfg(feature = "Foundation_Collections")]
impl INumberFormatterOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberFormatterOptions_Vtbl {
        unsafe extern "system" fn Languages<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GeographicRegion<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IntegerDigits<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIntegerDigits<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntegerDigits(value).into()
        }
        unsafe extern "system" fn FractionDigits<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFractionDigits<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFractionDigits(value).into()
        }
        unsafe extern "system" fn IsGrouped<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsGrouped<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsGrouped(value).into()
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDecimalPointAlwaysDisplayed(value).into()
        }
        unsafe extern "system" fn NumeralSystem<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNumeralSystem<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumeralSystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolvedLanguage<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolvedGeographicRegion<Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberFormatterOptions, BASE_OFFSET>(),
            Languages: Languages::<Impl, IMPL_OFFSET>,
            GeographicRegion: GeographicRegion::<Impl, IMPL_OFFSET>,
            IntegerDigits: IntegerDigits::<Impl, IMPL_OFFSET>,
            SetIntegerDigits: SetIntegerDigits::<Impl, IMPL_OFFSET>,
            FractionDigits: FractionDigits::<Impl, IMPL_OFFSET>,
            SetFractionDigits: SetFractionDigits::<Impl, IMPL_OFFSET>,
            IsGrouped: IsGrouped::<Impl, IMPL_OFFSET>,
            SetIsGrouped: SetIsGrouped::<Impl, IMPL_OFFSET>,
            IsDecimalPointAlwaysDisplayed: IsDecimalPointAlwaysDisplayed::<Impl, IMPL_OFFSET>,
            SetIsDecimalPointAlwaysDisplayed: SetIsDecimalPointAlwaysDisplayed::<Impl, IMPL_OFFSET>,
            NumeralSystem: NumeralSystem::<Impl, IMPL_OFFSET>,
            SetNumeralSystem: SetNumeralSystem::<Impl, IMPL_OFFSET>,
            ResolvedLanguage: ResolvedLanguage::<Impl, IMPL_OFFSET>,
            ResolvedGeographicRegion: ResolvedGeographicRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatterOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait INumberParser_Impl: Sized {
    fn ParseInt(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for INumberParser {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberParser";
}
#[cfg(feature = "Foundation")]
impl INumberParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberParser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberParser_Vtbl {
        unsafe extern "system" fn ParseInt<Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ParseUInt<Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ParseDouble<Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberParser, BASE_OFFSET>(),
            ParseInt: ParseInt::<Impl, IMPL_OFFSET>,
            ParseUInt: ParseUInt::<Impl, IMPL_OFFSET>,
            ParseDouble: ParseDouble::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberParser as ::windows::core::Interface>::IID
    }
}
pub trait INumberRounder_Impl: Sized {
    fn RoundInt32(&mut self, value: i32) -> ::windows::core::Result<i32>;
    fn RoundUInt32(&mut self, value: u32) -> ::windows::core::Result<u32>;
    fn RoundInt64(&mut self, value: i64) -> ::windows::core::Result<i64>;
    fn RoundUInt64(&mut self, value: u64) -> ::windows::core::Result<u64>;
    fn RoundSingle(&mut self, value: f32) -> ::windows::core::Result<f32>;
    fn RoundDouble(&mut self, value: f64) -> ::windows::core::Result<f64>;
}
impl ::windows::core::RuntimeName for INumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounder";
}
impl INumberRounder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberRounder_Vtbl {
        unsafe extern "system" fn RoundInt32<Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoundUInt32<Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoundInt64<Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoundUInt64<Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoundSingle<Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoundDouble<Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberRounder, BASE_OFFSET>(),
            RoundInt32: RoundInt32::<Impl, IMPL_OFFSET>,
            RoundUInt32: RoundUInt32::<Impl, IMPL_OFFSET>,
            RoundInt64: RoundInt64::<Impl, IMPL_OFFSET>,
            RoundUInt64: RoundUInt64::<Impl, IMPL_OFFSET>,
            RoundSingle: RoundSingle::<Impl, IMPL_OFFSET>,
            RoundDouble: RoundDouble::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberRounder as ::windows::core::Interface>::IID
    }
}
pub trait INumberRounderOption_Impl: Sized {
    fn NumberRounder(&mut self) -> ::windows::core::Result<INumberRounder>;
    fn SetNumberRounder(&mut self, value: &::core::option::Option<INumberRounder>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INumberRounderOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounderOption";
}
impl INumberRounderOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounderOption_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INumberRounderOption_Vtbl {
        unsafe extern "system" fn NumberRounder<Impl: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNumberRounder<Impl: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberRounder(&*(&value as *const <INumberRounder as ::windows::core::Abi>::Abi as *const <INumberRounder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberRounderOption, BASE_OFFSET>(),
            NumberRounder: NumberRounder::<Impl, IMPL_OFFSET>,
            SetNumberRounder: SetNumberRounder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberRounderOption as ::windows::core::Interface>::IID
    }
}
pub trait ISignedZeroOption_Impl: Sized {
    fn IsZeroSigned(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsZeroSigned(&mut self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISignedZeroOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignedZeroOption";
}
impl ISignedZeroOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignedZeroOption_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignedZeroOption_Vtbl {
        unsafe extern "system" fn IsZeroSigned<Impl: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsZeroSigned<Impl: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsZeroSigned(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISignedZeroOption, BASE_OFFSET>(),
            IsZeroSigned: IsZeroSigned::<Impl, IMPL_OFFSET>,
            SetIsZeroSigned: SetIsZeroSigned::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignedZeroOption as ::windows::core::Interface>::IID
    }
}
pub trait ISignificantDigitsOption_Impl: Sized {
    fn SignificantDigits(&mut self) -> ::windows::core::Result<i32>;
    fn SetSignificantDigits(&mut self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISignificantDigitsOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
}
impl ISignificantDigitsOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignificantDigitsOption_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignificantDigitsOption_Vtbl {
        unsafe extern "system" fn SignificantDigits<Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignificantDigits<Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignificantDigits(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISignificantDigitsOption, BASE_OFFSET>(),
            SignificantDigits: SignificantDigits::<Impl, IMPL_OFFSET>,
            SetSignificantDigits: SetSignificantDigits::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignificantDigitsOption as ::windows::core::Interface>::IID
    }
}
