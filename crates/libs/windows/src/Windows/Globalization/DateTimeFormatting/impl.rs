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
pub trait IDateTimeFormatter2Impl: Sized {
    fn FormatUsingTimeZone(&self, datetime: &super::super::Foundation::DateTime, timezoneid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
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
pub trait IDateTimeFormatterStaticsImpl: Sized {
    fn LongDate(&self) -> ::windows::core::Result<DateTimeFormatter>;
    fn LongTime(&self) -> ::windows::core::Result<DateTimeFormatter>;
    fn ShortDate(&self) -> ::windows::core::Result<DateTimeFormatter>;
    fn ShortTime(&self) -> ::windows::core::Result<DateTimeFormatter>;
}
