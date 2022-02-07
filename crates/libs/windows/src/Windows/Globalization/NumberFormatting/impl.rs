pub trait INumberFormatter_Impl: Sized {
    fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for INumberFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter";
}
impl INumberFormatter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter_Impl, const OFFSET: isize>() -> INumberFormatter_Vtbl {
        unsafe extern "system" fn FormatInt<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatUInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberFormatter, OFFSET>(),
            FormatInt: FormatInt::<Identity, Impl, OFFSET>,
            FormatUInt: FormatUInt::<Identity, Impl, OFFSET>,
            FormatDouble: FormatDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatter as ::windows::core::Interface>::IID
    }
}
pub trait INumberFormatter2_Impl: Sized {
    fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for INumberFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter2";
}
impl INumberFormatter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter2_Impl, const OFFSET: isize>() -> INumberFormatter2_Vtbl {
        unsafe extern "system" fn FormatInt<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatUInt(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberFormatter2, OFFSET>(),
            FormatInt: FormatInt::<Identity, Impl, OFFSET>,
            FormatUInt: FormatUInt::<Identity, Impl, OFFSET>,
            FormatDouble: FormatDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait INumberFormatterOptions_Impl: Sized {
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
impl INumberFormatterOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>() -> INumberFormatterOptions_Vtbl {
        unsafe extern "system" fn Languages<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeographicRegion<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntegerDigits<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IntegerDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntegerDigits<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIntegerDigits(value).into()
        }
        unsafe extern "system" fn FractionDigits<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFractionDigits<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFractionDigits(value).into()
        }
        unsafe extern "system" fn IsGrouped<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsGrouped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGrouped<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsGrouped(value).into()
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDecimalPointAlwaysDisplayed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsDecimalPointAlwaysDisplayed(value).into()
        }
        unsafe extern "system" fn NumeralSystem<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumeralSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumeralSystem(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ResolvedLanguage<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedGeographicRegion<Identity: ::windows::core::IUnknownImpl, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberFormatterOptions, OFFSET>(),
            Languages: Languages::<Identity, Impl, OFFSET>,
            GeographicRegion: GeographicRegion::<Identity, Impl, OFFSET>,
            IntegerDigits: IntegerDigits::<Identity, Impl, OFFSET>,
            SetIntegerDigits: SetIntegerDigits::<Identity, Impl, OFFSET>,
            FractionDigits: FractionDigits::<Identity, Impl, OFFSET>,
            SetFractionDigits: SetFractionDigits::<Identity, Impl, OFFSET>,
            IsGrouped: IsGrouped::<Identity, Impl, OFFSET>,
            SetIsGrouped: SetIsGrouped::<Identity, Impl, OFFSET>,
            IsDecimalPointAlwaysDisplayed: IsDecimalPointAlwaysDisplayed::<Identity, Impl, OFFSET>,
            SetIsDecimalPointAlwaysDisplayed: SetIsDecimalPointAlwaysDisplayed::<Identity, Impl, OFFSET>,
            NumeralSystem: NumeralSystem::<Identity, Impl, OFFSET>,
            SetNumeralSystem: SetNumeralSystem::<Identity, Impl, OFFSET>,
            ResolvedLanguage: ResolvedLanguage::<Identity, Impl, OFFSET>,
            ResolvedGeographicRegion: ResolvedGeographicRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberFormatterOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait INumberParser_Impl: Sized {
    fn ParseInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for INumberParser {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberParser";
}
#[cfg(feature = "Foundation")]
impl INumberParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberParser_Impl, const OFFSET: isize>() -> INumberParser_Vtbl {
        unsafe extern "system" fn ParseInt<Identity: ::windows::core::IUnknownImpl, Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParseInt(::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseUInt<Identity: ::windows::core::IUnknownImpl, Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParseUInt(::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDouble<Identity: ::windows::core::IUnknownImpl, Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParseDouble(::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberParser, OFFSET>(),
            ParseInt: ParseInt::<Identity, Impl, OFFSET>,
            ParseUInt: ParseUInt::<Identity, Impl, OFFSET>,
            ParseDouble: ParseDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberParser as ::windows::core::Interface>::IID
    }
}
pub trait INumberRounder_Impl: Sized {
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
impl INumberRounder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>() -> INumberRounder_Vtbl {
        unsafe extern "system" fn RoundInt32<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoundInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt32<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoundUInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundInt64<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoundInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt64<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoundUInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundSingle<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoundSingle(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundDouble<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberRounder, OFFSET>(),
            RoundInt32: RoundInt32::<Identity, Impl, OFFSET>,
            RoundUInt32: RoundUInt32::<Identity, Impl, OFFSET>,
            RoundInt64: RoundInt64::<Identity, Impl, OFFSET>,
            RoundUInt64: RoundUInt64::<Identity, Impl, OFFSET>,
            RoundSingle: RoundSingle::<Identity, Impl, OFFSET>,
            RoundDouble: RoundDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberRounder as ::windows::core::Interface>::IID
    }
}
pub trait INumberRounderOption_Impl: Sized {
    fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder>;
    fn SetNumberRounder(&self, value: &::core::option::Option<INumberRounder>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INumberRounderOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounderOption";
}
impl INumberRounderOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounderOption_Impl, const OFFSET: isize>() -> INumberRounderOption_Vtbl {
        unsafe extern "system" fn NumberRounder<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberRounder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberRounder<Identity: ::windows::core::IUnknownImpl, Impl: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumberRounder(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INumberRounderOption, OFFSET>(),
            NumberRounder: NumberRounder::<Identity, Impl, OFFSET>,
            SetNumberRounder: SetNumberRounder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INumberRounderOption as ::windows::core::Interface>::IID
    }
}
pub trait ISignedZeroOption_Impl: Sized {
    fn IsZeroSigned(&self) -> ::windows::core::Result<bool>;
    fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISignedZeroOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignedZeroOption";
}
impl ISignedZeroOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignedZeroOption_Impl, const OFFSET: isize>() -> ISignedZeroOption_Vtbl {
        unsafe extern "system" fn IsZeroSigned<Identity: ::windows::core::IUnknownImpl, Impl: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsZeroSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZeroSigned<Identity: ::windows::core::IUnknownImpl, Impl: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsZeroSigned(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISignedZeroOption, OFFSET>(),
            IsZeroSigned: IsZeroSigned::<Identity, Impl, OFFSET>,
            SetIsZeroSigned: SetIsZeroSigned::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignedZeroOption as ::windows::core::Interface>::IID
    }
}
pub trait ISignificantDigitsOption_Impl: Sized {
    fn SignificantDigits(&self) -> ::windows::core::Result<i32>;
    fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISignificantDigitsOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
}
impl ISignificantDigitsOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>() -> ISignificantDigitsOption_Vtbl {
        unsafe extern "system" fn SignificantDigits<Identity: ::windows::core::IUnknownImpl, Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SignificantDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Identity: ::windows::core::IUnknownImpl, Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSignificantDigits(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISignificantDigitsOption, OFFSET>(),
            SignificantDigits: SignificantDigits::<Identity, Impl, OFFSET>,
            SetSignificantDigits: SetSignificantDigits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignificantDigitsOption as ::windows::core::Interface>::IID
    }
}
