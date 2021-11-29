#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DateTimeFormatter(pub ::windows::core::IInspectable);
impl DateTimeFormatter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Calendar(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Clock(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Patterns(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Format<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IncludeYear(&self) -> ::windows::core::Result<YearFormat> {
        let this = self;
        unsafe {
            let mut result__: YearFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<YearFormat>(result__)
        }
    }
    pub fn IncludeMonth(&self) -> ::windows::core::Result<MonthFormat> {
        let this = self;
        unsafe {
            let mut result__: MonthFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MonthFormat>(result__)
        }
    }
    pub fn IncludeDayOfWeek(&self) -> ::windows::core::Result<DayOfWeekFormat> {
        let this = self;
        unsafe {
            let mut result__: DayOfWeekFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DayOfWeekFormat>(result__)
        }
    }
    pub fn IncludeDay(&self) -> ::windows::core::Result<DayFormat> {
        let this = self;
        unsafe {
            let mut result__: DayFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DayFormat>(result__)
        }
    }
    pub fn IncludeHour(&self) -> ::windows::core::Result<HourFormat> {
        let this = self;
        unsafe {
            let mut result__: HourFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HourFormat>(result__)
        }
    }
    pub fn IncludeMinute(&self) -> ::windows::core::Result<MinuteFormat> {
        let this = self;
        unsafe {
            let mut result__: MinuteFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MinuteFormat>(result__)
        }
    }
    pub fn IncludeSecond(&self) -> ::windows::core::Result<SecondFormat> {
        let this = self;
        unsafe {
            let mut result__: SecondFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondFormat>(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FormatUsingTimeZone<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, datetime: Param0, timezoneid: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDateTimeFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datetime.into_param().abi(), timezoneid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateDateTimeFormatter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(formattemplate: Param0) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), formattemplate.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterLanguages<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(formattemplate: Param0, languages: Param1) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), formattemplate.into_param().abi(), languages.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(formattemplate: Param0, languages: Param1, geographicregion: Param2, calendar: Param3, clock: Param4) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), formattemplate.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), calendar.into_param().abi(), clock.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn CreateDateTimeFormatterDate(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), yearformat, monthformat, dayformat, dayofweekformat, &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn CreateDateTimeFormatterTime(hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), hourformat, minuteformat, secondformat, &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeLanguages<'a, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: Param7) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeContext<'a, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param8: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param9: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param10: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: Param7, geographicregion: Param8, calendar: Param9, clock: Param10) -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.into_param().abi(), geographicregion.into_param().abi(), calendar.into_param().abi(), clock.into_param().abi(), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn LongDate() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn LongTime() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn ShortDate() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn ShortTime() -> ::windows::core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn IDateTimeFormatterFactory<R, F: FnOnce(&IDateTimeFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DateTimeFormatter, IDateTimeFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDateTimeFormatterStatics<R, F: FnOnce(&IDateTimeFormatterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DateTimeFormatter, IDateTimeFormatterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DateTimeFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.DateTimeFormatting.DateTimeFormatter;{95eeca10-73e0-4e4b-a183-3d6ad0ba35ec})");
}
unsafe impl ::windows::core::Interface for DateTimeFormatter {
    type Vtable = IDateTimeFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95eeca10_73e0_4e4b_a183_3d6ad0ba35ec);
}
impl ::windows::core::RuntimeName for DateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
}
impl ::core::convert::From<DateTimeFormatter> for ::windows::core::IUnknown {
    fn from(value: DateTimeFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows::core::IUnknown {
    fn from(value: &DateTimeFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DateTimeFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DateTimeFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DateTimeFormatter> for ::windows::core::IInspectable {
    fn from(value: DateTimeFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows::core::IInspectable {
    fn from(value: &DateTimeFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DateTimeFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DateTimeFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DateTimeFormatter {}
unsafe impl ::core::marker::Sync for DateTimeFormatter {}
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
unsafe impl ::windows::core::Abi for DayFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DayFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayFormat;i4)");
}
impl ::windows::core::DefaultType for DayFormat {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for DayOfWeekFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DayOfWeekFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayOfWeekFormat;i4)");
}
impl ::windows::core::DefaultType for DayOfWeekFormat {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for HourFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HourFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.HourFormat;i4)");
}
impl ::windows::core::DefaultType for HourFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDateTimeFormatter {
    type Vtable = IDateTimeFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95eeca10_73e0_4e4b_a183_3d6ad0ba35ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut YearFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MonthFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DayOfWeekFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DayFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HourFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MinuteFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SecondFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDateTimeFormatter2 {
    type Vtable = IDateTimeFormatter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27c91a86_bdaa_4fd0_9e36_671d5aa5ee03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datetime: super::super::Foundation::DateTime, timezoneid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDateTimeFormatterFactory {
    type Vtable = IDateTimeFormatterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec8d8a53_1a2e_412d_8815_3b745fb1a2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, formattemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDateTimeFormatterStatics {
    type Vtable = IDateTimeFormatterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcde7c0_df4c_4a2e_9012_f47daf3f1212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
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
unsafe impl ::windows::core::Abi for MinuteFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MinuteFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MinuteFormat;i4)");
}
impl ::windows::core::DefaultType for MinuteFormat {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for MonthFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MonthFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MonthFormat;i4)");
}
impl ::windows::core::DefaultType for MonthFormat {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for SecondFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SecondFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.SecondFormat;i4)");
}
impl ::windows::core::DefaultType for SecondFormat {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for YearFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for YearFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.YearFormat;i4)");
}
impl ::windows::core::DefaultType for YearFormat {
    type DefaultType = Self;
}
