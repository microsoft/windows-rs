#[cfg(feature = "implement_exclusive")]
pub trait IDateTimeFormatterImpl: Sized {
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Calendar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Clock(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Patterns(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Format(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IncludeYear(&self) -> ::windows::core::Result<YearFormat>;
    fn IncludeMonth(&self) -> ::windows::core::Result<MonthFormat>;
    fn IncludeDayOfWeek(&self) -> ::windows::core::Result<DayOfWeekFormat>;
    fn IncludeDay(&self) -> ::windows::core::Result<DayFormat>;
    fn IncludeHour(&self) -> ::windows::core::Result<HourFormat>;
    fn IncludeMinute(&self) -> ::windows::core::Result<MinuteFormat>;
    fn IncludeSecond(&self) -> ::windows::core::Result<SecondFormat>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.IDateTimeFormatter";
}
#[cfg(feature = "implement_exclusive")]
impl IDateTimeFormatterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDateTimeFormatterImpl, const OFFSET: isize>() -> IDateTimeFormatterVtbl {
        unsafe extern "system" fn Languages<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GeographicRegion<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Calendar<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calendar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clock<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumeralSystem<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNumeralSystem<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumeralSystem(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Patterns<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Patterns() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeYear<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut YearFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeYear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeMonth<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MonthFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeMonth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeDayOfWeek<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DayOfWeekFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeDayOfWeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeDay<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DayFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeDay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeHour<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HourFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeHour() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeMinute<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MinuteFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeMinute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludeSecond<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedLanguage<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolvedGeographicRegion<Impl: IDateTimeFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDateTimeFormatter>,
            ::windows::core::GetTrustLevel,
            Languages::<Impl, OFFSET>,
            GeographicRegion::<Impl, OFFSET>,
            Calendar::<Impl, OFFSET>,
            Clock::<Impl, OFFSET>,
            NumeralSystem::<Impl, OFFSET>,
            SetNumeralSystem::<Impl, OFFSET>,
            Patterns::<Impl, OFFSET>,
            Template::<Impl, OFFSET>,
            Format::<Impl, OFFSET>,
            IncludeYear::<Impl, OFFSET>,
            IncludeMonth::<Impl, OFFSET>,
            IncludeDayOfWeek::<Impl, OFFSET>,
            IncludeDay::<Impl, OFFSET>,
            IncludeHour::<Impl, OFFSET>,
            IncludeMinute::<Impl, OFFSET>,
            IncludeSecond::<Impl, OFFSET>,
            ResolvedLanguage::<Impl, OFFSET>,
            ResolvedGeographicRegion::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDateTimeFormatter2Impl: Sized {
    fn FormatUsingTimeZone(&self, datetime: &super::super::Foundation::DateTime, timezoneid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDateTimeFormatter2 {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2";
}
#[cfg(feature = "implement_exclusive")]
impl IDateTimeFormatter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDateTimeFormatter2Impl, const OFFSET: isize>() -> IDateTimeFormatter2Vtbl {
        unsafe extern "system" fn FormatUsingTimeZone<Impl: IDateTimeFormatter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datetime: super::super::Foundation::DateTime, timezoneid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatUsingTimeZone(&*(&datetime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&timezoneid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDateTimeFormatter2>, ::windows::core::GetTrustLevel, FormatUsingTimeZone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDateTimeFormatterFactoryImpl: Sized {
    fn CreateDateTimeFormatter(&self, formattemplate: &::windows::core::HSTRING) -> ::windows::core::Result<DateTimeFormatter>;
    fn CreateDateTimeFormatterLanguages(&self, formattemplate: &::windows::core::HSTRING, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<DateTimeFormatter>;
    fn CreateDateTimeFormatterContext(&self, formattemplate: &::windows::core::HSTRING, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING, calendar: &::windows::core::HSTRING, clock: &::windows::core::HSTRING) -> ::windows::core::Result<DateTimeFormatter>;
    fn CreateDateTimeFormatterDate(&self, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat) -> ::windows::core::Result<DateTimeFormatter>;
    fn CreateDateTimeFormatterTime(&self, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat) -> ::windows::core::Result<DateTimeFormatter>;
    fn CreateDateTimeFormatterDateTimeLanguages(&self, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<DateTimeFormatter>;
    fn CreateDateTimeFormatterDateTimeContext(&self, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING, calendar: &::windows::core::HSTRING, clock: &::windows::core::HSTRING) -> ::windows::core::Result<DateTimeFormatter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDateTimeFormatterFactory {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDateTimeFormatterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>() -> IDateTimeFormatterFactoryVtbl {
        unsafe extern "system" fn CreateDateTimeFormatter<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatter(&*(&formattemplate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeFormatterLanguages<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatterLanguages(&*(&formattemplate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeFormatterContext<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatterContext(
                &*(&formattemplate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&calendar as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clock as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeFormatterDate<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatterDate(yearformat, monthformat, dayformat, dayofweekformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeFormatterTime<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatterTime(hourformat, minuteformat, secondformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeFormatterDateTimeLanguages<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatterDateTimeLanguages(yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, &*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeFormatterDateTimeContext<Impl: IDateTimeFormatterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeFormatterDateTimeContext(
                yearformat,
                monthformat,
                dayformat,
                dayofweekformat,
                hourformat,
                minuteformat,
                secondformat,
                &*(&languages as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&geographicregion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&calendar as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clock as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDateTimeFormatterFactory>,
            ::windows::core::GetTrustLevel,
            CreateDateTimeFormatter::<Impl, OFFSET>,
            CreateDateTimeFormatterLanguages::<Impl, OFFSET>,
            CreateDateTimeFormatterContext::<Impl, OFFSET>,
            CreateDateTimeFormatterDate::<Impl, OFFSET>,
            CreateDateTimeFormatterTime::<Impl, OFFSET>,
            CreateDateTimeFormatterDateTimeLanguages::<Impl, OFFSET>,
            CreateDateTimeFormatterDateTimeContext::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDateTimeFormatterStaticsImpl: Sized {
    fn LongDate(&self) -> ::windows::core::Result<DateTimeFormatter>;
    fn LongTime(&self) -> ::windows::core::Result<DateTimeFormatter>;
    fn ShortDate(&self) -> ::windows::core::Result<DateTimeFormatter>;
    fn ShortTime(&self) -> ::windows::core::Result<DateTimeFormatter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDateTimeFormatterStatics {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDateTimeFormatterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDateTimeFormatterStaticsImpl, const OFFSET: isize>() -> IDateTimeFormatterStaticsVtbl {
        unsafe extern "system" fn LongDate<Impl: IDateTimeFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongTime<Impl: IDateTimeFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShortDate<Impl: IDateTimeFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShortDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShortTime<Impl: IDateTimeFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShortTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDateTimeFormatterStatics>, ::windows::core::GetTrustLevel, LongDate::<Impl, OFFSET>, LongTime::<Impl, OFFSET>, ShortDate::<Impl, OFFSET>, ShortTime::<Impl, OFFSET>)
    }
}
