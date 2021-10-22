#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DateTimeFormatter(::windows::runtime::IInspectable);
impl DateTimeFormatter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Calendar(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Clock(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Patterns(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn Template(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Format<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IncludeYear(&self) -> ::windows::runtime::Result<YearFormat> {
        let this = self;
        unsafe {
            let mut result__: YearFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<YearFormat>(result__)
        }
    }
    pub fn IncludeMonth(&self) -> ::windows::runtime::Result<MonthFormat> {
        let this = self;
        unsafe {
            let mut result__: MonthFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MonthFormat>(result__)
        }
    }
    pub fn IncludeDayOfWeek(&self) -> ::windows::runtime::Result<DayOfWeekFormat> {
        let this = self;
        unsafe {
            let mut result__: DayOfWeekFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DayOfWeekFormat>(result__)
        }
    }
    pub fn IncludeDay(&self) -> ::windows::runtime::Result<DayFormat> {
        let this = self;
        unsafe {
            let mut result__: DayFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DayFormat>(result__)
        }
    }
    pub fn IncludeHour(&self) -> ::windows::runtime::Result<HourFormat> {
        let this = self;
        unsafe {
            let mut result__: HourFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HourFormat>(result__)
        }
    }
    pub fn IncludeMinute(&self) -> ::windows::runtime::Result<MinuteFormat> {
        let this = self;
        unsafe {
            let mut result__: MinuteFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MinuteFormat>(result__)
        }
    }
    pub fn IncludeSecond(&self) -> ::windows::runtime::Result<SecondFormat> {
        let this = self;
        unsafe {
            let mut result__: SecondFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SecondFormat>(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FormatUsingTimeZone<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        datetime: Param0,
        timezoneid: Param1,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDateTimeFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                datetime.into_param().abi(),
                timezoneid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CreateDateTimeFormatter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        formattemplate: Param0,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                formattemplate.into_param().abi(),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterLanguages<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        formattemplate: Param0,
        languages: Param1,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                formattemplate.into_param().abi(),
                languages.into_param().abi(),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        formattemplate: Param0,
        languages: Param1,
        geographicregion: Param2,
        calendar: Param3,
        clock: Param4,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                formattemplate.into_param().abi(),
                languages.into_param().abi(),
                geographicregion.into_param().abi(),
                calendar.into_param().abi(),
                clock.into_param().abi(),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn CreateDateTimeFormatterDate(
        yearformat: YearFormat,
        monthformat: MonthFormat,
        dayformat: DayFormat,
        dayofweekformat: DayOfWeekFormat,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                yearformat,
                monthformat,
                dayformat,
                dayofweekformat,
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn CreateDateTimeFormatterTime(
        hourformat: HourFormat,
        minuteformat: MinuteFormat,
        secondformat: SecondFormat,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                hourformat,
                minuteformat,
                secondformat,
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeLanguages<
        'a,
        Param7: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        yearformat: YearFormat,
        monthformat: MonthFormat,
        dayformat: DayFormat,
        dayofweekformat: DayOfWeekFormat,
        hourformat: HourFormat,
        minuteformat: MinuteFormat,
        secondformat: SecondFormat,
        languages: Param7,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                yearformat,
                monthformat,
                dayformat,
                dayofweekformat,
                hourformat,
                minuteformat,
                secondformat,
                languages.into_param().abi(),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeContext<
        'a,
        Param7: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
        Param8: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param9: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param10: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        yearformat: YearFormat,
        monthformat: MonthFormat,
        dayformat: DayFormat,
        dayofweekformat: DayOfWeekFormat,
        hourformat: HourFormat,
        minuteformat: MinuteFormat,
        secondformat: SecondFormat,
        languages: Param7,
        geographicregion: Param8,
        calendar: Param9,
        clock: Param10,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                yearformat,
                monthformat,
                dayformat,
                dayofweekformat,
                hourformat,
                minuteformat,
                secondformat,
                languages.into_param().abi(),
                geographicregion.into_param().abi(),
                calendar.into_param().abi(),
                clock.into_param().abi(),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn LongDate() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn LongTime() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn ShortDate() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn ShortTime() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn IDateTimeFormatterFactory<
        R,
        F: FnOnce(&IDateTimeFormatterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DateTimeFormatter,
            IDateTimeFormatterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDateTimeFormatterStatics<
        R,
        F: FnOnce(&IDateTimeFormatterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DateTimeFormatter,
            IDateTimeFormatterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DateTimeFormatter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Globalization.DateTimeFormatting.DateTimeFormatter;{95eeca10-73e0-4e4b-a183-3d6ad0ba35ec})" ) ;
}
unsafe impl ::windows::runtime::Interface for DateTimeFormatter {
    type Vtable = IDateTimeFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2515454480,
        29664,
        20043,
        [161, 131, 61, 106, 208, 186, 53, 236],
    );
}
impl ::windows::runtime::RuntimeName for DateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
}
impl ::std::convert::From<DateTimeFormatter> for ::windows::runtime::IUnknown {
    fn from(value: DateTimeFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DateTimeFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &DateTimeFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DateTimeFormatter> for ::windows::runtime::IInspectable {
    fn from(value: DateTimeFormatter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DateTimeFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &DateTimeFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DateTimeFormatter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DateTimeFormatter {}
unsafe impl ::std::marker::Sync for DateTimeFormatter {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: DayFormat = DayFormat(0i32);
    pub const Default: DayFormat = DayFormat(1i32);
}
impl ::std::convert::From<i32> for DayFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DayFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DayFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.DayFormat;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: DayOfWeekFormat = DayOfWeekFormat(0i32);
    pub const Default: DayOfWeekFormat = DayOfWeekFormat(1i32);
    pub const Abbreviated: DayOfWeekFormat = DayOfWeekFormat(2i32);
    pub const Full: DayOfWeekFormat = DayOfWeekFormat(3i32);
}
impl ::std::convert::From<i32> for DayOfWeekFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DayOfWeekFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DayOfWeekFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.DayOfWeekFormat;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: HourFormat = HourFormat(0i32);
    pub const Default: HourFormat = HourFormat(1i32);
}
impl ::std::convert::From<i32> for HourFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HourFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HourFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.HourFormat;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDateTimeFormatter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatter {
    type Vtable = IDateTimeFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2515454480,
        29664,
        20043,
        [161, 131, 61, 106, 208, 186, 53, 236],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::DateTime,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut YearFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MonthFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut DayOfWeekFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut DayFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut HourFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MinuteFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SecondFormat,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDateTimeFormatter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatter2 {
    type Vtable = IDateTimeFormatter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        667490950,
        48554,
        20432,
        [158, 54, 103, 29, 90, 165, 238, 3],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        datetime: super::super::Foundation::DateTime,
        timezoneid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatterFactory {
    type Vtable = IDateTimeFormatterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3968698963,
        6702,
        16685,
        [136, 21, 59, 116, 95, 177, 162, 160],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        formattemplate: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        formattemplate: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        languages: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        formattemplate: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        languages: ::windows::runtime::RawPtr,
        geographicregion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        calendar: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        clock: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        yearformat: YearFormat,
        monthformat: MonthFormat,
        dayformat: DayFormat,
        dayofweekformat: DayOfWeekFormat,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hourformat: HourFormat,
        minuteformat: MinuteFormat,
        secondformat: SecondFormat,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        yearformat: YearFormat,
        monthformat: MonthFormat,
        dayformat: DayFormat,
        dayofweekformat: DayOfWeekFormat,
        hourformat: HourFormat,
        minuteformat: MinuteFormat,
        secondformat: SecondFormat,
        languages: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        yearformat: YearFormat,
        monthformat: MonthFormat,
        dayformat: DayFormat,
        dayofweekformat: DayOfWeekFormat,
        hourformat: HourFormat,
        minuteformat: MinuteFormat,
        secondformat: SecondFormat,
        languages: ::windows::runtime::RawPtr,
        geographicregion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        calendar: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        clock: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatterStatics {
    type Vtable = IDateTimeFormatterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3217942464,
        57164,
        18990,
        [144, 18, 244, 125, 175, 63, 18, 18],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: MinuteFormat = MinuteFormat(0i32);
    pub const Default: MinuteFormat = MinuteFormat(1i32);
}
impl ::std::convert::From<i32> for MinuteFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MinuteFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MinuteFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.MinuteFormat;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: MonthFormat = MonthFormat(0i32);
    pub const Default: MonthFormat = MonthFormat(1i32);
    pub const Abbreviated: MonthFormat = MonthFormat(2i32);
    pub const Full: MonthFormat = MonthFormat(3i32);
    pub const Numeric: MonthFormat = MonthFormat(4i32);
}
impl ::std::convert::From<i32> for MonthFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MonthFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MonthFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.MonthFormat;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: SecondFormat = SecondFormat(0i32);
    pub const Default: SecondFormat = SecondFormat(1i32);
}
impl ::std::convert::From<i32> for SecondFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.SecondFormat;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: YearFormat = YearFormat(0i32);
    pub const Default: YearFormat = YearFormat(1i32);
    pub const Abbreviated: YearFormat = YearFormat(2i32);
    pub const Full: YearFormat = YearFormat(3i32);
}
impl ::std::convert::From<i32> for YearFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for YearFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for YearFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Globalization.DateTimeFormatting.YearFormat;i4)",
    );
}
