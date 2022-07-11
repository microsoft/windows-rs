#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct DateTimeFormatter(::windows::core::IUnknown);
impl DateTimeFormatter {
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Languages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GeographicRegion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn Calendar(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Calendar)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn Clock(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Clock)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).NumeralSystem)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn SetNumeralSystem<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNumeralSystem)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Patterns(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Patterns)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Template)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Format(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeYear(&self) -> ::windows::core::Result<YearFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<YearFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeYear)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<YearFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeMonth(&self) -> ::windows::core::Result<MonthFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MonthFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeMonth)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MonthFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeDayOfWeek(&self) -> ::windows::core::Result<DayOfWeekFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DayOfWeekFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeDayOfWeek)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DayOfWeekFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeDay(&self) -> ::windows::core::Result<DayFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DayFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeDay)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DayFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeHour(&self) -> ::windows::core::Result<HourFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HourFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeHour)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HourFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeMinute(&self) -> ::windows::core::Result<MinuteFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MinuteFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeMinute)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MinuteFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn IncludeSecond(&self) -> ::windows::core::Result<SecondFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SecondFormat>::zeroed();
            (::windows::core::Interface::vtable(this).IncludeSecond)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SecondFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ResolvedLanguage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ResolvedGeographicRegion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FormatUsingTimeZone<'a, Param1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(&self, datetime: super::super::Foundation::DateTime, timezoneid: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDateTimeFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).FormatUsingTimeZone)(::windows::core::Interface::as_raw(this), datetime, timezoneid.into().abi(), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn CreateDateTimeFormatter<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(formattemplate: Param0) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatter)(::windows::core::Interface::as_raw(this), formattemplate.into().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterLanguages<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>, Param1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E1>, E1: ::std::convert::Into<::windows::core::Error>>(formattemplate: Param0, languages: Param1) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatterLanguages)(::windows::core::Interface::as_raw(this), formattemplate.into().abi(), languages.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterContext<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>, Param1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E1>, E1: ::std::convert::Into<::windows::core::Error>, Param2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>, Param3: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>, Param4: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(formattemplate: Param0, languages: Param1, geographicregion: Param2, calendar: Param3, clock: Param4) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatterContext)(::windows::core::Interface::as_raw(this), formattemplate.into().abi(), languages.try_into().map_err(|e| e.into())?.abi(), geographicregion.into().abi(), calendar.into().abi(), clock.into().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn CreateDateTimeFormatterDate<'a, Param0: ::std::convert::Into<YearFormat>, Param1: ::std::convert::Into<MonthFormat>, Param2: ::std::convert::Into<DayFormat>, Param3: ::std::convert::Into<DayOfWeekFormat>>(yearformat: Param0, monthformat: Param1, dayformat: Param2, dayofweekformat: Param3) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatterDate)(::windows::core::Interface::as_raw(this), yearformat.into(), monthformat.into(), dayformat.into(), dayofweekformat.into(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn CreateDateTimeFormatterTime<'a, Param0: ::std::convert::Into<HourFormat>, Param1: ::std::convert::Into<MinuteFormat>, Param2: ::std::convert::Into<SecondFormat>>(hourformat: Param0, minuteformat: Param1, secondformat: Param2) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatterTime)(::windows::core::Interface::as_raw(this), hourformat.into(), minuteformat.into(), secondformat.into(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeLanguages<'a, Param0: ::std::convert::Into<YearFormat>, Param1: ::std::convert::Into<MonthFormat>, Param2: ::std::convert::Into<DayFormat>, Param3: ::std::convert::Into<DayOfWeekFormat>, Param4: ::std::convert::Into<HourFormat>, Param5: ::std::convert::Into<MinuteFormat>, Param6: ::std::convert::Into<SecondFormat>, Param7: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E7>, E7: ::std::convert::Into<::windows::core::Error>>(yearformat: Param0, monthformat: Param1, dayformat: Param2, dayofweekformat: Param3, hourformat: Param4, minuteformat: Param5, secondformat: Param6, languages: Param7) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatterDateTimeLanguages)(::windows::core::Interface::as_raw(this), yearformat.into(), monthformat.into(), dayformat.into(), dayofweekformat.into(), hourformat.into(), minuteformat.into(), secondformat.into(), languages.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeContext<
        'a,
        Param0: ::std::convert::Into<YearFormat>,
        Param1: ::std::convert::Into<MonthFormat>,
        Param2: ::std::convert::Into<DayFormat>,
        Param3: ::std::convert::Into<DayOfWeekFormat>,
        Param4: ::std::convert::Into<HourFormat>,
        Param5: ::std::convert::Into<MinuteFormat>,
        Param6: ::std::convert::Into<SecondFormat>,
        Param7: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E7>,
        E7: ::std::convert::Into<::windows::core::Error>,
        Param8: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
        Param9: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
        Param10: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    >(
        yearformat: Param0,
        monthformat: Param1,
        dayformat: Param2,
        dayofweekformat: Param3,
        hourformat: Param4,
        minuteformat: Param5,
        secondformat: Param6,
        languages: Param7,
        geographicregion: Param8,
        calendar: Param9,
        clock: Param10,
    ) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeFormatterDateTimeContext)(::windows::core::Interface::as_raw(this), yearformat.into(), monthformat.into(), dayformat.into(), dayofweekformat.into(), hourformat.into(), minuteformat.into(), secondformat.into(), languages.try_into().map_err(|e| e.into())?.abi(), geographicregion.into().abi(), calendar.into().abi(), clock.into().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn LongDate() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).LongDate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn LongTime() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).LongTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn ShortDate() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ShortDate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
    pub fn ShortTime() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ShortTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDateTimeFormatterFactory<R, F: FnOnce(&IDateTimeFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DateTimeFormatter, IDateTimeFormatterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDateTimeFormatterStatics<R, F: FnOnce(&IDateTimeFormatterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DateTimeFormatter, IDateTimeFormatterStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DateTimeFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DateTimeFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DateTimeFormatter {}
impl ::core::fmt::Debug for DateTimeFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DateTimeFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DateTimeFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.DateTimeFormatting.DateTimeFormatter;{95eeca10-73e0-4e4b-a183-3d6ad0ba35ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DateTimeFormatter {
    type Vtable = IDateTimeFormatter_Vtbl;
    const IID: ::windows::core::GUID = <IDateTimeFormatter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
}
impl ::core::convert::From<DateTimeFormatter> for ::windows::core::IUnknown {
    fn from(value: DateTimeFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows::core::IUnknown {
    fn from(value: &DateTimeFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DateTimeFormatter> for &::windows::core::IUnknown {
    fn from(value: &DateTimeFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DateTimeFormatter> for ::windows::core::IInspectable {
    fn from(value: DateTimeFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows::core::IInspectable {
    fn from(value: &DateTimeFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DateTimeFormatter> for &::windows::core::IInspectable {
    fn from(value: &DateTimeFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DateTimeFormatter {}
unsafe impl ::core::marker::Sync for DateTimeFormatter {}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for DayFormat {}
impl ::core::clone::Clone for DayFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DayFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DayFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for DayFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DayFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for DayOfWeekFormat {}
impl ::core::clone::Clone for DayOfWeekFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DayOfWeekFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DayOfWeekFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for DayOfWeekFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayOfWeekFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DayOfWeekFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayOfWeekFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for HourFormat {}
impl ::core::clone::Clone for HourFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HourFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HourFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for HourFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HourFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HourFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.HourFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDateTimeFormatter {
    type Vtable = IDateTimeFormatter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95eeca10_73e0_4e4b_a183_3d6ad0ba35ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Calendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Clock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Patterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Patterns: usize,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Format: usize,
    pub IncludeYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut YearFormat) -> ::windows::core::HRESULT,
    pub IncludeMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MonthFormat) -> ::windows::core::HRESULT,
    pub IncludeDayOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DayOfWeekFormat) -> ::windows::core::HRESULT,
    pub IncludeDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DayFormat) -> ::windows::core::HRESULT,
    pub IncludeHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HourFormat) -> ::windows::core::HRESULT,
    pub IncludeMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MinuteFormat) -> ::windows::core::HRESULT,
    pub IncludeSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondFormat) -> ::windows::core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDateTimeFormatter2 {
    type Vtable = IDateTimeFormatter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27c91a86_bdaa_4fd0_9e36_671d5aa5ee03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FormatUsingTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datetime: super::super::Foundation::DateTime, timezoneid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatUsingTimeZone: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDateTimeFormatterFactory {
    type Vtable = IDateTimeFormatterFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec8d8a53_1a2e_412d_8815_3b745fb1a2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateDateTimeFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterLanguages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: *mut ::core::ffi::c_void, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterContext: usize,
    pub CreateDateTimeFormatterDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDateTimeFormatterTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterDateTimeLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterDateTimeLanguages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterDateTimeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: *mut ::core::ffi::c_void, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterDateTimeContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDateTimeFormatterStatics {
    type Vtable = IDateTimeFormatterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcde7c0_df4c_4a2e_9012_f47daf3f1212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub LongDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LongTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShortDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShortTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for MinuteFormat {}
impl ::core::clone::Clone for MinuteFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MinuteFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MinuteFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MinuteFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MinuteFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MinuteFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MinuteFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
    pub const Numeric: Self = Self(4i32);
}
impl ::core::marker::Copy for MonthFormat {}
impl ::core::clone::Clone for MonthFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MonthFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MonthFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MonthFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MonthFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MonthFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MonthFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for SecondFormat {}
impl ::core::clone::Clone for SecondFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecondFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SecondFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecondFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecondFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.SecondFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for YearFormat {}
impl ::core::clone::Clone for YearFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for YearFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for YearFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for YearFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("YearFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for YearFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.YearFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
