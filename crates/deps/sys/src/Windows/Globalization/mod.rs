#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Globalization_Collation")]
pub mod Collation;
#[cfg(feature = "Globalization_DateTimeFormatting")]
pub mod DateTimeFormatting;
#[cfg(feature = "Globalization_Fonts")]
pub mod Fonts;
#[cfg(feature = "Globalization_NumberFormatting")]
pub mod NumberFormatting;
#[cfg(feature = "Globalization_PhoneNumberFormatting")]
pub mod PhoneNumberFormatting;
#[link(name = "windows")]
extern "system" {
    fn ApplicationLanguages();
    fn Calendar();
    fn CalendarIdentifiers();
    fn ClockIdentifiers();
    fn CurrencyAmount();
    fn CurrencyIdentifiers();
    fn DayOfWeek();
    fn GeographicRegion();
    fn GlobalizationJapanesePhoneticAnalyzerContract();
    fn IApplicationLanguagesStatics();
    fn IApplicationLanguagesStatics2();
    fn ICalendar();
    fn ICalendarFactory();
    fn ICalendarFactory2();
    fn ICalendarIdentifiersStatics();
    fn ICalendarIdentifiersStatics2();
    fn ICalendarIdentifiersStatics3();
    fn IClockIdentifiersStatics();
    fn ICurrencyAmount();
    fn ICurrencyAmountFactory();
    fn ICurrencyIdentifiersStatics();
    fn ICurrencyIdentifiersStatics2();
    fn ICurrencyIdentifiersStatics3();
    fn IGeographicRegion();
    fn IGeographicRegionFactory();
    fn IGeographicRegionStatics();
    fn IJapanesePhoneme();
    fn IJapanesePhoneticAnalyzerStatics();
    fn ILanguage();
    fn ILanguage2();
    fn ILanguage3();
    fn ILanguageExtensionSubtags();
    fn ILanguageFactory();
    fn ILanguageStatics();
    fn ILanguageStatics2();
    fn ILanguageStatics3();
    fn INumeralSystemIdentifiersStatics();
    fn INumeralSystemIdentifiersStatics2();
    fn ITimeZoneOnCalendar();
    fn JapanesePhoneme();
    fn JapanesePhoneticAnalyzer();
    fn Language();
    fn LanguageLayoutDirection();
    fn NumeralSystemIdentifiers();
}
