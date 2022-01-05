#[cfg(feature = "implement_exclusive")]
pub trait IApplicationLanguagesStaticsImpl: Sized {
    fn PrimaryLanguageOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPrimaryLanguageOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Languages(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ManifestLanguages(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationLanguagesStatics2Impl: Sized {
    fn GetLanguagesForUser(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarImpl: Sized {
    fn Clone(&self) -> ::windows::core::Result<Calendar>;
    fn SetToMin(&self) -> ::windows::core::Result<()>;
    fn SetToMax(&self) -> ::windows::core::Result<()>;
    fn Languages(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetCalendarSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChangeCalendarSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetClock(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChangeClock(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDateTime(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn SetDateTime(&self, value: &super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetToNow(&self) -> ::windows::core::Result<()>;
    fn FirstEra(&self) -> ::windows::core::Result<i32>;
    fn LastEra(&self) -> ::windows::core::Result<i32>;
    fn NumberOfEras(&self) -> ::windows::core::Result<i32>;
    fn Era(&self) -> ::windows::core::Result<i32>;
    fn SetEra(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddEras(&self, eras: i32) -> ::windows::core::Result<()>;
    fn EraAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EraAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstYearInThisEra(&self) -> ::windows::core::Result<i32>;
    fn LastYearInThisEra(&self) -> ::windows::core::Result<i32>;
    fn NumberOfYearsInThisEra(&self) -> ::windows::core::Result<i32>;
    fn Year(&self) -> ::windows::core::Result<i32>;
    fn SetYear(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddYears(&self, years: i32) -> ::windows::core::Result<()>;
    fn YearAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YearAsTruncatedString(&self, remainingdigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YearAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstMonthInThisYear(&self) -> ::windows::core::Result<i32>;
    fn LastMonthInThisYear(&self) -> ::windows::core::Result<i32>;
    fn NumberOfMonthsInThisYear(&self) -> ::windows::core::Result<i32>;
    fn Month(&self) -> ::windows::core::Result<i32>;
    fn SetMonth(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddMonths(&self, months: i32) -> ::windows::core::Result<()>;
    fn MonthAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonthAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonthAsFullSoloString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonthAsSoloString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonthAsNumericString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonthAsPaddedNumericString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddWeeks(&self, weeks: i32) -> ::windows::core::Result<()>;
    fn FirstDayInThisMonth(&self) -> ::windows::core::Result<i32>;
    fn LastDayInThisMonth(&self) -> ::windows::core::Result<i32>;
    fn NumberOfDaysInThisMonth(&self) -> ::windows::core::Result<i32>;
    fn Day(&self) -> ::windows::core::Result<i32>;
    fn SetDay(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddDays(&self, days: i32) -> ::windows::core::Result<()>;
    fn DayAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DayAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DayOfWeek(&self) -> ::windows::core::Result<DayOfWeek>;
    fn DayOfWeekAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DayOfWeekAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DayOfWeekAsFullSoloString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DayOfWeekAsSoloString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstPeriodInThisDay(&self) -> ::windows::core::Result<i32>;
    fn LastPeriodInThisDay(&self) -> ::windows::core::Result<i32>;
    fn NumberOfPeriodsInThisDay(&self) -> ::windows::core::Result<i32>;
    fn Period(&self) -> ::windows::core::Result<i32>;
    fn SetPeriod(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddPeriods(&self, periods: i32) -> ::windows::core::Result<()>;
    fn PeriodAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PeriodAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstHourInThisPeriod(&self) -> ::windows::core::Result<i32>;
    fn LastHourInThisPeriod(&self) -> ::windows::core::Result<i32>;
    fn NumberOfHoursInThisPeriod(&self) -> ::windows::core::Result<i32>;
    fn Hour(&self) -> ::windows::core::Result<i32>;
    fn SetHour(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddHours(&self, hours: i32) -> ::windows::core::Result<()>;
    fn HourAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HourAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Minute(&self) -> ::windows::core::Result<i32>;
    fn SetMinute(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddMinutes(&self, minutes: i32) -> ::windows::core::Result<()>;
    fn MinuteAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinuteAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Second(&self) -> ::windows::core::Result<i32>;
    fn SetSecond(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddSeconds(&self, seconds: i32) -> ::windows::core::Result<()>;
    fn SecondAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecondAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nanosecond(&self) -> ::windows::core::Result<i32>;
    fn SetNanosecond(&self, value: i32) -> ::windows::core::Result<()>;
    fn AddNanoseconds(&self, nanoseconds: i32) -> ::windows::core::Result<()>;
    fn NanosecondAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NanosecondAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Compare(&self, other: &::core::option::Option<Calendar>) -> ::windows::core::Result<i32>;
    fn CompareDateTime(&self, other: &super::Foundation::DateTime) -> ::windows::core::Result<i32>;
    fn CopyTo(&self, other: &::core::option::Option<Calendar>) -> ::windows::core::Result<()>;
    fn FirstMinuteInThisHour(&self) -> ::windows::core::Result<i32>;
    fn LastMinuteInThisHour(&self) -> ::windows::core::Result<i32>;
    fn NumberOfMinutesInThisHour(&self) -> ::windows::core::Result<i32>;
    fn FirstSecondInThisMinute(&self) -> ::windows::core::Result<i32>;
    fn LastSecondInThisMinute(&self) -> ::windows::core::Result<i32>;
    fn NumberOfSecondsInThisMinute(&self) -> ::windows::core::Result<i32>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsDaylightSavingTime(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarFactoryImpl: Sized {
    fn CreateCalendarDefaultCalendarAndClock(&self, languages: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<Calendar>;
    fn CreateCalendar(&self, languages: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, calendar: &::windows::core::HSTRING, clock: &::windows::core::HSTRING) -> ::windows::core::Result<Calendar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarFactory2Impl: Sized {
    fn CreateCalendarWithTimeZone(&self, languages: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, calendar: &::windows::core::HSTRING, clock: &::windows::core::HSTRING, timezoneid: &::windows::core::HSTRING) -> ::windows::core::Result<Calendar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarIdentifiersStaticsImpl: Sized {
    fn Gregorian(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hebrew(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hijri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Japanese(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Julian(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Korean(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Taiwan(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Thai(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UmAlQura(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarIdentifiersStatics2Impl: Sized {
    fn Persian(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarIdentifiersStatics3Impl: Sized {
    fn ChineseLunar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JapaneseLunar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KoreanLunar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TaiwanLunar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VietnameseLunar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClockIdentifiersStaticsImpl: Sized {
    fn TwelveHour(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TwentyFourHour(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyAmountImpl: Sized {
    fn Amount(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyAmountFactoryImpl: Sized {
    fn Create(&self, amount: &::windows::core::HSTRING, currency: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyAmount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyIdentifiersStaticsImpl: Sized {
    fn AED(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AFN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ALL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AMD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ANG(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AOA(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ARS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AUD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AWG(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AZN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BAM(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BBD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BDT(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BGN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BHD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BIF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BMD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BND(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BOB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BRL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BSD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BTN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BWP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BYR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BZD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CAD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CDF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CHF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CLP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CNY(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn COP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CRC(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CUP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CVE(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CZK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DJF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DKK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DOP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DZD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EGP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ERN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ETB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EUR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FJD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FKP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GBP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GEL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GHS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GIP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GMD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GNF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GTQ(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GYD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HKD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HNL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HRK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HTG(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HUF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IDR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ILS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn INR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IQD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IRR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ISK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JMD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JOD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JPY(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KES(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KGS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KHR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KMF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KPW(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KRW(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KWD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KYD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KZT(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LAK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LBP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LKR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LRD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LSL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LTL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LVL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LYD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MAD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MDL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MGA(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MKD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MMK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MNT(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MOP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MRO(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MUR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MVR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MWK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MXN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MYR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MZN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NAD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NGN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NIO(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NOK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NPR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NZD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OMR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PAB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PEN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PGK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PHP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PKR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PLN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PYG(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QAR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RON(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RSD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RUB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RWF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SAR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SBD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SCR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SDG(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SEK(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SGD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SHP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SLL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SOS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SRD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn STD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SYP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SZL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn THB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TJS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TMT(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TND(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TOP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TRY(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TTD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TWD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TZS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UAH(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UGX(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn USD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UYU(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UZS(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VEF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VND(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VUV(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WST(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XAF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XCD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XOF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XPF(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XXX(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YER(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZAR(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZMW(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZWL(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyIdentifiersStatics2Impl: Sized {
    fn BYN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyIdentifiersStatics3Impl: Sized {
    fn MRU(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SSP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn STN(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VES(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeographicRegionImpl: Sized {
    fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CodeTwoLetter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CodeThreeLetter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CodeThreeDigit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NativeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrenciesInUse(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeographicRegionFactoryImpl: Sized {
    fn CreateGeographicRegion(&self, geographicregioncode: &::windows::core::HSTRING) -> ::windows::core::Result<GeographicRegion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeographicRegionStaticsImpl: Sized {
    fn IsSupported(&self, geographicregioncode: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJapanesePhonemeImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPhraseStart(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJapanesePhoneticAnalyzerStaticsImpl: Sized {
    fn GetWords(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<JapanesePhoneme>>;
    fn GetWordsWithMonoRubyOption(&self, input: &::windows::core::HSTRING, monoruby: bool) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<JapanesePhoneme>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageImpl: Sized {
    fn LanguageTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NativeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Script(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguage2Impl: Sized {
    fn LayoutDirection(&self) -> ::windows::core::Result<LanguageLayoutDirection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguage3Impl: Sized {
    fn AbbreviatedName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageExtensionSubtagsImpl: Sized {
    fn GetExtensionSubtags(&self, singleton: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFactoryImpl: Sized {
    fn CreateLanguage(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<Language>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageStaticsImpl: Sized {
    fn IsWellFormed(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn CurrentInputMethodLanguageTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageStatics2Impl: Sized {
    fn TrySetInputMethodLanguageTag(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageStatics3Impl: Sized {
    fn GetMuiCompatibleLanguageListFromLanguageTags(&self, languagetags: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INumeralSystemIdentifiersStaticsImpl: Sized {
    fn Arab(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ArabExt(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bali(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Beng(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cham(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Deva(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FullWide(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gujr(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Guru(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HaniDec(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Java(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kali(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Khmr(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Knda(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Lana(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanaTham(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Laoo(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Latn(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Lepc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Limb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mlym(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mong(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mtei(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mymr(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MymrShan(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nkoo(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Olck(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orya(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Saur(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sund(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Talu(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TamlDec(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Telu(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Thai(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tibt(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Vaii(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INumeralSystemIdentifiersStatics2Impl: Sized {
    fn Brah(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Osma(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MathBold(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MathDbl(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MathSans(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MathSanb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MathMono(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZmthBold(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZmthDbl(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZmthSans(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZmthSanb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ZmthMono(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeZoneOnCalendarImpl: Sized {
    fn GetTimeZone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChangeTimeZone(&self, timezoneid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TimeZoneAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TimeZoneAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
}
