#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub struct Calendar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrencyAmount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DayOfWeek(pub i32);
impl DayOfWeek {
    pub const Sunday: DayOfWeek = DayOfWeek(0i32);
    pub const Monday: DayOfWeek = DayOfWeek(1i32);
    pub const Tuesday: DayOfWeek = DayOfWeek(2i32);
    pub const Wednesday: DayOfWeek = DayOfWeek(3i32);
    pub const Thursday: DayOfWeek = DayOfWeek(4i32);
    pub const Friday: DayOfWeek = DayOfWeek(5i32);
    pub const Saturday: DayOfWeek = DayOfWeek(6i32);
}
#[repr(transparent)]
pub struct GeographicRegion(pub *mut ::core::ffi::c_void);
#[repr(C)]
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
pub struct Language(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LanguageLayoutDirection(pub i32);
impl LanguageLayoutDirection {
    pub const Ltr: LanguageLayoutDirection = LanguageLayoutDirection(0i32);
    pub const Rtl: LanguageLayoutDirection = LanguageLayoutDirection(1i32);
    pub const TtbLtr: LanguageLayoutDirection = LanguageLayoutDirection(2i32);
    pub const TtbRtl: LanguageLayoutDirection = LanguageLayoutDirection(3i32);
}
