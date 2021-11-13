#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for Calendar {}
impl ::core::clone::Clone for Calendar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CurrencyAmount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrencyAmount {}
impl ::core::clone::Clone for CurrencyAmount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DayOfWeek(pub i32);
impl DayOfWeek {
    pub const Sunday: Self = Self(0i32);
    pub const Monday: Self = Self(1i32);
    pub const Tuesday: Self = Self(2i32);
    pub const Wednesday: Self = Self(3i32);
    pub const Thursday: Self = Self(4i32);
    pub const Friday: Self = Self(5i32);
    pub const Saturday: Self = Self(6i32);
}
impl ::core::marker::Copy for DayOfWeek {}
impl ::core::clone::Clone for DayOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeographicRegion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeographicRegion {}
impl ::core::clone::Clone for GeographicRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationLanguagesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationLanguagesStatics {}
impl ::core::clone::Clone for IApplicationLanguagesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationLanguagesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationLanguagesStatics2 {}
impl ::core::clone::Clone for IApplicationLanguagesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendar {}
impl ::core::clone::Clone for ICalendar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarFactory {}
impl ::core::clone::Clone for ICalendarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarFactory2 {}
impl ::core::clone::Clone for ICalendarFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarIdentifiersStatics {}
impl ::core::clone::Clone for ICalendarIdentifiersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarIdentifiersStatics2 {}
impl ::core::clone::Clone for ICalendarIdentifiersStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarIdentifiersStatics3 {}
impl ::core::clone::Clone for ICalendarIdentifiersStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClockIdentifiersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClockIdentifiersStatics {}
impl ::core::clone::Clone for IClockIdentifiersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyAmount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyAmount {}
impl ::core::clone::Clone for ICurrencyAmount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyAmountFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyAmountFactory {}
impl ::core::clone::Clone for ICurrencyAmountFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyIdentifiersStatics {}
impl ::core::clone::Clone for ICurrencyIdentifiersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyIdentifiersStatics2 {}
impl ::core::clone::Clone for ICurrencyIdentifiersStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyIdentifiersStatics3 {}
impl ::core::clone::Clone for ICurrencyIdentifiersStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeographicRegion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeographicRegion {}
impl ::core::clone::Clone for IGeographicRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeographicRegionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeographicRegionFactory {}
impl ::core::clone::Clone for IGeographicRegionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeographicRegionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeographicRegionStatics {}
impl ::core::clone::Clone for IGeographicRegionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJapanesePhoneme(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJapanesePhoneme {}
impl ::core::clone::Clone for IJapanesePhoneme {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJapanesePhoneticAnalyzerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJapanesePhoneticAnalyzerStatics {}
impl ::core::clone::Clone for IJapanesePhoneticAnalyzerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguage {}
impl ::core::clone::Clone for ILanguage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguage2 {}
impl ::core::clone::Clone for ILanguage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguage3 {}
impl ::core::clone::Clone for ILanguage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageExtensionSubtags(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageExtensionSubtags {}
impl ::core::clone::Clone for ILanguageExtensionSubtags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageFactory {}
impl ::core::clone::Clone for ILanguageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageStatics {}
impl ::core::clone::Clone for ILanguageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageStatics2 {}
impl ::core::clone::Clone for ILanguageStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageStatics3 {}
impl ::core::clone::Clone for ILanguageStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumeralSystemIdentifiersStatics {}
impl ::core::clone::Clone for INumeralSystemIdentifiersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumeralSystemIdentifiersStatics2 {}
impl ::core::clone::Clone for INumeralSystemIdentifiersStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeZoneOnCalendar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeZoneOnCalendar {}
impl ::core::clone::Clone for ITimeZoneOnCalendar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JapanesePhoneme(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JapanesePhoneme {}
impl ::core::clone::Clone for JapanesePhoneme {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Language(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Language {}
impl ::core::clone::Clone for Language {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LanguageLayoutDirection(pub i32);
impl LanguageLayoutDirection {
    pub const Ltr: Self = Self(0i32);
    pub const Rtl: Self = Self(1i32);
    pub const TtbLtr: Self = Self(2i32);
    pub const TtbRtl: Self = Self(3i32);
}
impl ::core::marker::Copy for LanguageLayoutDirection {}
impl ::core::clone::Clone for LanguageLayoutDirection {
    fn clone(&self) -> Self {
        *self
    }
}
