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
#[repr(transparent)]
pub struct ApplicationLanguages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Calendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClockIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrencyAmount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrencyIdentifiers(pub *mut ::core::ffi::c_void);
pub struct DayOfWeek(i32);
#[repr(transparent)]
pub struct GeographicRegion(pub *mut ::core::ffi::c_void);
pub struct GlobalizationJapanesePhoneticAnalyzerContract(i32);
#[repr(transparent)]
pub struct IApplicationLanguagesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationLanguagesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClockIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyAmount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyAmountFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeographicRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeographicRegionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeographicRegionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJapanesePhoneme(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJapanesePhoneticAnalyzerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageExtensionSubtags(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeZoneOnCalendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JapanesePhoneme(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JapanesePhoneticAnalyzer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Language(pub *mut ::core::ffi::c_void);
pub struct LanguageLayoutDirection(i32);
#[repr(transparent)]
pub struct NumeralSystemIdentifiers(pub *mut ::core::ffi::c_void);
