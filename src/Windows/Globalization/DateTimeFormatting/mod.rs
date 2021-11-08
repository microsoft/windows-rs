#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DateTimeFormatter(pub ::windows::runtime::IInspectable);
impl DateTimeFormatter {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn Calendar(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn Clock(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation_Collections`*"]
    pub fn Patterns(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn Template(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation`*"]
    pub fn Format<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeYear(&self) -> ::windows::runtime::Result<YearFormat> {
        let this = self;
        unsafe {
            let mut result__: YearFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<YearFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeMonth(&self) -> ::windows::runtime::Result<MonthFormat> {
        let this = self;
        unsafe {
            let mut result__: MonthFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MonthFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeDayOfWeek(&self) -> ::windows::runtime::Result<DayOfWeekFormat> {
        let this = self;
        unsafe {
            let mut result__: DayOfWeekFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DayOfWeekFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeDay(&self) -> ::windows::runtime::Result<DayFormat> {
        let this = self;
        unsafe {
            let mut result__: DayFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DayFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeHour(&self) -> ::windows::runtime::Result<HourFormat> {
        let this = self;
        unsafe {
            let mut result__: HourFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HourFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeMinute(&self) -> ::windows::runtime::Result<MinuteFormat> {
        let this = self;
        unsafe {
            let mut result__: MinuteFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MinuteFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn IncludeSecond(&self) -> ::windows::runtime::Result<SecondFormat> {
        let this = self;
        unsafe {
            let mut result__: SecondFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondFormat>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation`*"]
    pub fn FormatUsingTimeZone<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, datetime: Param0, timezoneid: Param1) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDateTimeFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datetime.into_param().abi(), timezoneid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn CreateDateTimeFormatter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(formattemplate: Param0) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), formattemplate.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation_Collections`*"]
    pub fn CreateDateTimeFormatterLanguages<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(formattemplate: Param0, languages: Param1) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), formattemplate.into_param().abi(), languages.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation_Collections`*"]
    pub fn CreateDateTimeFormatterContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        formattemplate: Param0,
        languages: Param1,
        geographicregion: Param2,
        calendar: Param3,
        clock: Param4,
    ) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), formattemplate.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), calendar.into_param().abi(), clock.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn CreateDateTimeFormatterDate(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), yearformat, monthformat, dayformat, dayofweekformat, &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn CreateDateTimeFormatterTime(hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), hourformat, minuteformat, secondformat, &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation_Collections`*"]
    pub fn CreateDateTimeFormatterDateTimeLanguages<'a, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: Param7) -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_DateTimeFormatting`, `Foundation_Collections`*"]
    pub fn CreateDateTimeFormatterDateTimeContext<'a, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param8: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param9: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param10: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
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
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.into_param().abi(), geographicregion.into_param().abi(), calendar.into_param().abi(), clock.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn LongDate() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn LongTime() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn ShortDate() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
    pub fn ShortTime() -> ::windows::runtime::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn IDateTimeFormatterFactory<R, F: FnOnce(&IDateTimeFormatterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DateTimeFormatter, IDateTimeFormatterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDateTimeFormatterStatics<R, F: FnOnce(&IDateTimeFormatterStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DateTimeFormatter, IDateTimeFormatterStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DateTimeFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.DateTimeFormatting.DateTimeFormatter;{95eeca10-73e0-4e4b-a183-3d6ad0ba35ec})");
}
unsafe impl ::windows::runtime::Interface for DateTimeFormatter {
    type Vtable = IDateTimeFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2515454480, 29664, 20043, [161, 131, 61, 106, 208, 186, 53, 236]);
}
impl ::windows::runtime::RuntimeName for DateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
}
impl ::core::convert::From<DateTimeFormatter> for ::windows::runtime::IUnknown {
    fn from(value: DateTimeFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &DateTimeFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DateTimeFormatter> for ::windows::runtime::IInspectable {
    fn from(value: DateTimeFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &DateTimeFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DateTimeFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DateTimeFormatter {}
unsafe impl ::core::marker::Sync for DateTimeFormatter {}
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: DayFormat = DayFormat(0i32);
    pub const Default: DayFormat = DayFormat(1i32);
}
impl ::core::convert::From<i32> for DayFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DayFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DayFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayFormat;i4)");
}
impl ::windows::runtime::DefaultType for DayFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: DayOfWeekFormat = DayOfWeekFormat(0i32);
    pub const Default: DayOfWeekFormat = DayOfWeekFormat(1i32);
    pub const Abbreviated: DayOfWeekFormat = DayOfWeekFormat(2i32);
    pub const Full: DayOfWeekFormat = DayOfWeekFormat(3i32);
}
impl ::core::convert::From<i32> for DayOfWeekFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DayOfWeekFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DayOfWeekFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayOfWeekFormat;i4)");
}
impl ::windows::runtime::DefaultType for DayOfWeekFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: HourFormat = HourFormat(0i32);
    pub const Default: HourFormat = HourFormat(1i32);
}
impl ::core::convert::From<i32> for HourFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HourFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HourFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.HourFormat;i4)");
}
impl ::windows::runtime::DefaultType for HourFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatter {
    type Vtable = IDateTimeFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2515454480, 29664, 20043, [161, 131, 61, 106, 208, 186, 53, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut YearFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MonthFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DayOfWeekFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DayFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HourFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MinuteFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SecondFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatter2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatter2 {
    type Vtable = IDateTimeFormatter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(667490950, 48554, 20432, [158, 54, 103, 29, 90, 165, 238, 3]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datetime: super::super::Foundation::DateTime, timezoneid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatterFactory {
    type Vtable = IDateTimeFormatterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3968698963, 6702, 16685, [136, 21, 59, 116, 95, 177, 162, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formattemplate: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formattemplate: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, languages: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formattemplate: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, languages: ::windows::runtime::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
        geographicregion: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        calendar: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        clock: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDateTimeFormatterStatics {
    type Vtable = IDateTimeFormatterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3217942464, 57164, 18990, [144, 18, 244, 125, 175, 63, 18, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: MinuteFormat = MinuteFormat(0i32);
    pub const Default: MinuteFormat = MinuteFormat(1i32);
}
impl ::core::convert::From<i32> for MinuteFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MinuteFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MinuteFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MinuteFormat;i4)");
}
impl ::windows::runtime::DefaultType for MinuteFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: MonthFormat = MonthFormat(0i32);
    pub const Default: MonthFormat = MonthFormat(1i32);
    pub const Abbreviated: MonthFormat = MonthFormat(2i32);
    pub const Full: MonthFormat = MonthFormat(3i32);
    pub const Numeric: MonthFormat = MonthFormat(4i32);
}
impl ::core::convert::From<i32> for MonthFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MonthFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MonthFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MonthFormat;i4)");
}
impl ::windows::runtime::DefaultType for MonthFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: SecondFormat = SecondFormat(0i32);
    pub const Default: SecondFormat = SecondFormat(1i32);
}
impl ::core::convert::From<i32> for SecondFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.SecondFormat;i4)");
}
impl ::windows::runtime::DefaultType for SecondFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_DateTimeFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: YearFormat = YearFormat(0i32);
    pub const Default: YearFormat = YearFormat(1i32);
    pub const Abbreviated: YearFormat = YearFormat(2i32);
    pub const Full: YearFormat = YearFormat(3i32);
}
impl ::core::convert::From<i32> for YearFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for YearFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for YearFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.YearFormat;i4)");
}
impl ::windows::runtime::DefaultType for YearFormat {
    type DefaultType = Self;
}
