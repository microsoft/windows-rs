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
extern "system" {}
pub struct ApplicationLanguages(i32);
pub struct Calendar(i32);
pub struct CalendarIdentifiers(i32);
pub struct ClockIdentifiers(i32);
pub struct CurrencyAmount(i32);
pub struct CurrencyIdentifiers(i32);
pub struct DayOfWeek(i32);
pub struct GeographicRegion(i32);
pub struct GlobalizationJapanesePhoneticAnalyzerContract(i32);
pub struct IApplicationLanguagesStatics(pub *mut ::core::ffi::c_void);
pub struct IApplicationLanguagesStatics2(pub *mut ::core::ffi::c_void);
pub struct ICalendar(pub *mut ::core::ffi::c_void);
pub struct ICalendarFactory(pub *mut ::core::ffi::c_void);
pub struct ICalendarFactory2(pub *mut ::core::ffi::c_void);
pub struct ICalendarIdentifiersStatics(pub *mut ::core::ffi::c_void);
pub struct ICalendarIdentifiersStatics2(pub *mut ::core::ffi::c_void);
pub struct ICalendarIdentifiersStatics3(pub *mut ::core::ffi::c_void);
pub struct IClockIdentifiersStatics(pub *mut ::core::ffi::c_void);
pub struct ICurrencyAmount(pub *mut ::core::ffi::c_void);
pub struct ICurrencyAmountFactory(pub *mut ::core::ffi::c_void);
pub struct ICurrencyIdentifiersStatics(pub *mut ::core::ffi::c_void);
pub struct ICurrencyIdentifiersStatics2(pub *mut ::core::ffi::c_void);
pub struct ICurrencyIdentifiersStatics3(pub *mut ::core::ffi::c_void);
pub struct IGeographicRegion(pub *mut ::core::ffi::c_void);
pub struct IGeographicRegionFactory(pub *mut ::core::ffi::c_void);
pub struct IGeographicRegionStatics(pub *mut ::core::ffi::c_void);
pub struct IJapanesePhoneme(pub *mut ::core::ffi::c_void);
pub struct IJapanesePhoneticAnalyzerStatics(pub *mut ::core::ffi::c_void);
pub struct ILanguage(pub *mut ::core::ffi::c_void);
pub struct ILanguage2(pub *mut ::core::ffi::c_void);
pub struct ILanguage3(pub *mut ::core::ffi::c_void);
pub struct ILanguageExtensionSubtags(pub *mut ::core::ffi::c_void);
pub struct ILanguageFactory(pub *mut ::core::ffi::c_void);
pub struct ILanguageStatics(pub *mut ::core::ffi::c_void);
pub struct ILanguageStatics2(pub *mut ::core::ffi::c_void);
pub struct ILanguageStatics3(pub *mut ::core::ffi::c_void);
pub struct INumeralSystemIdentifiersStatics(pub *mut ::core::ffi::c_void);
pub struct INumeralSystemIdentifiersStatics2(pub *mut ::core::ffi::c_void);
pub struct ITimeZoneOnCalendar(pub *mut ::core::ffi::c_void);
pub struct JapanesePhoneme(i32);
pub struct JapanesePhoneticAnalyzer(i32);
pub struct Language(i32);
pub struct LanguageLayoutDirection(i32);
pub struct NumeralSystemIdentifiers(i32);
