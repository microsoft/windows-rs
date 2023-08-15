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
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationLanguagesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationLanguagesStatics {
    type Vtable = IApplicationLanguagesStatics_Vtbl;
}
impl ::core::clone::Clone for IApplicationLanguagesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IApplicationLanguagesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75b40847_0a4c_4a92_9565_fd63c95f7aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationLanguagesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrimaryLanguageOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPrimaryLanguageOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ManifestLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ManifestLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationLanguagesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationLanguagesStatics2 {
    type Vtable = IApplicationLanguagesStatics2_Vtbl;
}
impl ::core::clone::Clone for IApplicationLanguagesStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IApplicationLanguagesStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1df0de4f_072b_4d7b_8f06_cb2db40f2bb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationLanguagesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub GetLanguagesForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    GetLanguagesForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendar {
    type Vtable = ICalendar_Vtbl;
}
impl ::core::clone::Clone for ICalendar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca30221d_86d9_40fb_a26b_d44eb7cf08ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetToMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetToMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetCalendarSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ChangeCalendarSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ChangeClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDateTime: usize,
    pub SetToNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FirstEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfEras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Era: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddEras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eras: i32) -> ::windows_core::HRESULT,
    pub EraAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EraAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstYearInThisEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastYearInThisEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfYearsInThisEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddYears: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, years: i32) -> ::windows_core::HRESULT,
    pub YearAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YearAsTruncatedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remainingdigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YearAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstMonthInThisYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastMonthInThisYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfMonthsInThisYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddMonths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, months: i32) -> ::windows_core::HRESULT,
    pub MonthAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MonthAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MonthAsFullSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MonthAsSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MonthAsNumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MonthAsPaddedNumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddWeeks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weeks: i32) -> ::windows_core::HRESULT,
    pub FirstDayInThisMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastDayInThisMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfDaysInThisMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT,
    pub DayAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DayAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DayOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DayOfWeek) -> ::windows_core::HRESULT,
    pub DayOfWeekAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DayOfWeekAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DayOfWeekAsFullSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DayOfWeekAsSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstPeriodInThisDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastPeriodInThisDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfPeriodsInThisDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Period: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, periods: i32) -> ::windows_core::HRESULT,
    pub PeriodAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PeriodAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstHourInThisPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastHourInThisPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfHoursInThisPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Hour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hours: i32) -> ::windows_core::HRESULT,
    pub HourAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HourAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Minute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT,
    pub MinuteAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MinuteAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Second: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: i32) -> ::windows_core::HRESULT,
    pub SecondAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecondAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Nanosecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetNanosecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub AddNanoseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nanoseconds: i32) -> ::windows_core::HRESULT,
    pub NanosecondAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NanosecondAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompareDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: super::Foundation::DateTime, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompareDateTime: usize,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FirstMinuteInThisHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastMinuteInThisHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfMinutesInThisHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub FirstSecondInThisMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LastSecondInThisMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfSecondsInThisMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsDaylightSavingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarFactory {
    type Vtable = ICalendarFactory_Vtbl;
}
impl ::core::clone::Clone for ICalendarFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendarFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83f58412_e56b_4c75_a66e_0f63d57758a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCalendarDefaultCalendarAndClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCalendarDefaultCalendarAndClock: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCalendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, calendar: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clock: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCalendar: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarFactory2 {
    type Vtable = ICalendarFactory2_Vtbl;
}
impl ::core::clone::Clone for ICalendarFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendarFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb44b378c_ca7e_4590_9e72_ea2bec1a5115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCalendarWithTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, calendar: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clock: ::std::mem::MaybeUninit<::windows_core::HSTRING>, timezoneid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCalendarWithTimeZone: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarIdentifiersStatics {
    type Vtable = ICalendarIdentifiersStatics_Vtbl;
}
impl ::core::clone::Clone for ICalendarIdentifiersStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendarIdentifiersStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80653f68_2cb2_4c1f_b590_f0f52bf4fd1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Gregorian: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Hebrew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Hijri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Japanese: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Julian: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Korean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Taiwan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Thai: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UmAlQura: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarIdentifiersStatics2 {
    type Vtable = ICalendarIdentifiersStatics2_Vtbl;
}
impl ::core::clone::Clone for ICalendarIdentifiersStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendarIdentifiersStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7df4d488_5fd0_42a7_95b5_7d98d823075f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarIdentifiersStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Persian: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarIdentifiersStatics3 {
    type Vtable = ICalendarIdentifiersStatics3_Vtbl;
}
impl ::core::clone::Clone for ICalendarIdentifiersStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendarIdentifiersStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c225423_1fad_40c0_9334_a8eb90db04f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarIdentifiersStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChineseLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JapaneseLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KoreanLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaiwanLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VietnameseLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClockIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClockIdentifiersStatics {
    type Vtable = IClockIdentifiersStatics_Vtbl;
}
impl ::core::clone::Clone for IClockIdentifiersStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IClockIdentifiersStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x523805bb_12ec_4f83_bc31_b1b4376b0808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TwelveHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TwentyFourHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyAmount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyAmount {
    type Vtable = ICurrencyAmount_Vtbl;
}
impl ::core::clone::Clone for ICurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyAmount {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74b49942_eb75_443a_95b3_7d723f56f93c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyAmount_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyAmountFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyAmountFactory {
    type Vtable = ICurrencyAmountFactory_Vtbl;
}
impl ::core::clone::Clone for ICurrencyAmountFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyAmountFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48d7168f_ef3b_4aee_a6a1_4b036fe03ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyAmountFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: ::std::mem::MaybeUninit<::windows_core::HSTRING>, currency: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyIdentifiersStatics {
    type Vtable = ICurrencyIdentifiersStatics_Vtbl;
}
impl ::core::clone::Clone for ICurrencyIdentifiersStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyIdentifiersStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f1d091b_d586_4913_9b6a_a9bd2dc12874);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AED: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AFN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ALL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ANG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AOA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ARS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AUD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AWG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AZN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BAM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BBD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BDT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BGN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BHD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BIF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BOB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BRL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BTN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BWP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BYR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BZD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CAD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CDF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CLP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CNY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub COP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CRC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CUP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CVE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CZK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DJF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DKK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DZD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EGP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ERN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ETB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EUR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FJD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FKP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GBP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GEL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GHS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GIP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GNF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GTQ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GYD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HKD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HNL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HRK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HTG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HUF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IDR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ILS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub INR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IQD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IRR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ISK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JOD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JPY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KGS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KHR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KMF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KPW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KRW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KWD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KYD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KZT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LAK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LBP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LKR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LRD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LSL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LVL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LYD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MAD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MDL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MGA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MKD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MMK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MNT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MRO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MUR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MVR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MWK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MXN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MYR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MZN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NAD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NGN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NOK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NPR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NZD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OMR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PAB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PEN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PGK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PHP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PKR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PLN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PYG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub QAR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RON: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RUB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RWF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SAR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SBD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SCR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SDG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SEK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SGD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SHP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SLL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SOS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SRD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub STD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SYP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SZL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub THB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TJS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TMT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TRY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TTD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TWD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TZS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UAH: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UGX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub USD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UYU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UZS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VEF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VUV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WST: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub XAF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub XCD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub XOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub XPF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub XXX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YER: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZAR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZMW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZWL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyIdentifiersStatics2 {
    type Vtable = ICurrencyIdentifiersStatics2_Vtbl;
}
impl ::core::clone::Clone for ICurrencyIdentifiersStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyIdentifiersStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1814797f_c3b2_4c33_9591_980011950d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyIdentifiersStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BYN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyIdentifiersStatics3 {
    type Vtable = ICurrencyIdentifiersStatics3_Vtbl;
}
impl ::core::clone::Clone for ICurrencyIdentifiersStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyIdentifiersStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fb23bfa_ed25_4f4d_857f_237f1748c21c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyIdentifiersStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MRU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SSP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub STN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeographicRegion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeographicRegion {
    type Vtable = IGeographicRegion_Vtbl;
}
impl ::core::clone::Clone for IGeographicRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeographicRegion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01e9a621_4a64_4ed9_954f_9edeb07bd903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeographicRegion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CodeTwoLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CodeThreeLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CodeThreeDigit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CurrenciesInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CurrenciesInUse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeographicRegionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeographicRegionFactory {
    type Vtable = IGeographicRegionFactory_Vtbl;
}
impl ::core::clone::Clone for IGeographicRegionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeographicRegionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53425270_77b4_426b_859f_81e19d512546);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeographicRegionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geographicregioncode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeographicRegionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeographicRegionStatics {
    type Vtable = IGeographicRegionStatics_Vtbl;
}
impl ::core::clone::Clone for IGeographicRegionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeographicRegionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29e28974_7ad9_4ef4_8799_b3b44fadec08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeographicRegionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geographicregioncode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJapanesePhoneme(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJapanesePhoneme {
    type Vtable = IJapanesePhoneme_Vtbl;
}
impl ::core::clone::Clone for IJapanesePhoneme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IJapanesePhoneme {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f6a9300_e85b_43e6_897d_5d82f862df21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJapanesePhoneme_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YomiText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPhraseStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJapanesePhoneticAnalyzerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJapanesePhoneticAnalyzerStatics {
    type Vtable = IJapanesePhoneticAnalyzerStatics_Vtbl;
}
impl ::core::clone::Clone for IJapanesePhoneticAnalyzerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IJapanesePhoneticAnalyzerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88ab9e90_93de_41b2_b4d5_8edb227fd1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJapanesePhoneticAnalyzerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWords: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWordsWithMonoRubyOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows_core::HSTRING>, monoruby: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWordsWithMonoRubyOption: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguage {
    type Vtable = ILanguage_Vtbl;
}
impl ::core::clone::Clone for ILanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea79a752_f7c2_4265_b1bd_c4dec4e4f080);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Script: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguage2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguage2 {
    type Vtable = ILanguage2_Vtbl;
}
impl ::core::clone::Clone for ILanguage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a47e5b5_d94d_4886_a404_a5a5b9d5b494);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguage2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LayoutDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LanguageLayoutDirection) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguage3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguage3 {
    type Vtable = ILanguage3_Vtbl;
}
impl ::core::clone::Clone for ILanguage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguage3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6af3d10_641a_5ba4_bb43_5e12aed75954);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguage3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AbbreviatedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageExtensionSubtags(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageExtensionSubtags {
    type Vtable = ILanguageExtensionSubtags_Vtbl;
}
impl ::core::clone::Clone for ILanguageExtensionSubtags {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguageExtensionSubtags {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d7daf45_368d_4364_852b_dec927037b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExtensionSubtags_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionSubtags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, singleton: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionSubtags: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageFactory {
    type Vtable = ILanguageFactory_Vtbl;
}
impl ::core::clone::Clone for ILanguageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b0252ac_0c27_44f8_b792_9793fb66c63e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageStatics {
    type Vtable = ILanguageStatics_Vtbl;
}
impl ::core::clone::Clone for ILanguageStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguageStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb23cd557_0865_46d4_89b8_d59be8990f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsWellFormed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CurrentInputMethodLanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageStatics2 {
    type Vtable = ILanguageStatics2_Vtbl;
}
impl ::core::clone::Clone for ILanguageStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguageStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30199f6e_914b_4b2a_9d6e_e3b0e27dbe4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TrySetInputMethodLanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageStatics3 {
    type Vtable = ILanguageStatics3_Vtbl;
}
impl ::core::clone::Clone for ILanguageStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILanguageStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd15ecb5a_71de_5752_9542_fac5b4f27261);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMuiCompatibleLanguageListFromLanguageTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetags: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMuiCompatibleLanguageListFromLanguageTags: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INumeralSystemIdentifiersStatics {
    type Vtable = INumeralSystemIdentifiersStatics_Vtbl;
}
impl ::core::clone::Clone for INumeralSystemIdentifiersStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumeralSystemIdentifiersStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5c662c3_68c9_4d3d_b765_972029e21dec);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Arab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ArabExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Bali: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Beng: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Cham: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Deva: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FullWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Gujr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Guru: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HaniDec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Java: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kali: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Khmr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Knda: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Lana: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LanaTham: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Laoo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Latn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Lepc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Limb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mlym: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mtei: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mymr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MymrShan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Nkoo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Olck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Orya: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Saur: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sund: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Talu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TamlDec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Telu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Thai: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tibt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Vaii: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INumeralSystemIdentifiersStatics2 {
    type Vtable = INumeralSystemIdentifiersStatics2_Vtbl;
}
impl ::core::clone::Clone for INumeralSystemIdentifiersStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumeralSystemIdentifiersStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f003228_9ddb_4a34_9104_0260c091a7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemIdentifiersStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Brah: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Osma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MathBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MathDbl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MathSans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MathSanb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MathMono: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZmthBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZmthDbl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZmthSans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZmthSanb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ZmthMono: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneOnCalendar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeZoneOnCalendar {
    type Vtable = ITimeZoneOnCalendar_Vtbl;
}
impl ::core::clone::Clone for ITimeZoneOnCalendar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimeZoneOnCalendar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb3c25e5_46cf_4317_a3f5_02621ad54478);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneOnCalendar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ChangeTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timezoneid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TimeZoneAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TimeZoneAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct ApplicationLanguages;
impl ApplicationLanguages {
    pub fn PrimaryLanguageOverride() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IApplicationLanguagesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryLanguageOverride)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetPrimaryLanguageOverride(value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IApplicationLanguagesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetPrimaryLanguageOverride)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages() -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IApplicationLanguagesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ManifestLanguages() -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IApplicationLanguagesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManifestLanguages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn GetLanguagesForUser<P0>(user: P0) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<super::System::User>,
    {
        Self::IApplicationLanguagesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLanguagesForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationLanguagesStatics<R, F: FnOnce(&IApplicationLanguagesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ApplicationLanguages, IApplicationLanguagesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationLanguagesStatics2<R, F: FnOnce(&IApplicationLanguagesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ApplicationLanguages, IApplicationLanguagesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ApplicationLanguages {
    const NAME: &'static str = "Windows.Globalization.ApplicationLanguages";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct Calendar(::windows_core::IUnknown);
impl Calendar {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Calendar, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clone(&self) -> ::windows_core::Result<Calendar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetToMin(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToMin)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetToMax(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToMax)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetCalendarSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCalendarSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeCalendarSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ChangeCalendarSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetClock(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetClock)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeClock(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ChangeClock)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDateTime(&self) -> ::windows_core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDateTime(&self, value: super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDateTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetToNow(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToNow)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FirstEra(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstEra)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastEra(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastEra)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfEras(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfEras)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Era(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Era)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEra(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEra)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddEras(&self, eras: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddEras)(::windows_core::Interface::as_raw(this), eras).ok() }
    }
    pub fn EraAsFullString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EraAsFullString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EraAsString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EraAsString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    pub fn FirstYearInThisEra(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstYearInThisEra)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastYearInThisEra(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastYearInThisEra)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfYearsInThisEra(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfYearsInThisEra)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Year(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Year)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYear(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetYear)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddYears(&self, years: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddYears)(::windows_core::Interface::as_raw(this), years).ok() }
    }
    pub fn YearAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YearAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn YearAsTruncatedString(&self, remainingdigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YearAsTruncatedString)(::windows_core::Interface::as_raw(this), remainingdigits, &mut result__).from_abi(result__)
        }
    }
    pub fn YearAsPaddedString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YearAsPaddedString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn FirstMonthInThisYear(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstMonthInThisYear)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastMonthInThisYear(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastMonthInThisYear)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfMonthsInThisYear(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfMonthsInThisYear)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Month(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Month)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMonth(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMonth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddMonths(&self, months: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddMonths)(::windows_core::Interface::as_raw(this), months).ok() }
    }
    pub fn MonthAsFullString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonthAsFullString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MonthAsString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonthAsString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    pub fn MonthAsFullSoloString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonthAsFullSoloString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MonthAsSoloString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonthAsSoloString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    pub fn MonthAsNumericString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonthAsNumericString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MonthAsPaddedNumericString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonthAsPaddedNumericString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn AddWeeks(&self, weeks: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddWeeks)(::windows_core::Interface::as_raw(this), weeks).ok() }
    }
    pub fn FirstDayInThisMonth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstDayInThisMonth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastDayInThisMonth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastDayInThisMonth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfDaysInThisMonth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfDaysInThisMonth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Day(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Day)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDay(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddDays(&self, days: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDays)(::windows_core::Interface::as_raw(this), days).ok() }
    }
    pub fn DayAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DayAsPaddedString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayAsPaddedString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn DayOfWeek(&self) -> ::windows_core::Result<DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayOfWeek)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DayOfWeekAsFullString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayOfWeekAsFullString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DayOfWeekAsString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayOfWeekAsString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    pub fn DayOfWeekAsFullSoloString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayOfWeekAsFullSoloString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DayOfWeekAsSoloString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DayOfWeekAsSoloString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    pub fn FirstPeriodInThisDay(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstPeriodInThisDay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastPeriodInThisDay(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastPeriodInThisDay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfPeriodsInThisDay(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfPeriodsInThisDay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Period(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Period)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPeriod(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPeriod)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddPeriods(&self, periods: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPeriods)(::windows_core::Interface::as_raw(this), periods).ok() }
    }
    pub fn PeriodAsFullString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PeriodAsFullString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PeriodAsString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PeriodAsString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    pub fn FirstHourInThisPeriod(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstHourInThisPeriod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastHourInThisPeriod(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastHourInThisPeriod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfHoursInThisPeriod(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfHoursInThisPeriod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Hour(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hour)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHour(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHour)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddHours(&self, hours: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddHours)(::windows_core::Interface::as_raw(this), hours).ok() }
    }
    pub fn HourAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HourAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HourAsPaddedString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HourAsPaddedString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn Minute(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Minute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMinute(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinute)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddMinutes(&self, minutes: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddMinutes)(::windows_core::Interface::as_raw(this), minutes).ok() }
    }
    pub fn MinuteAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinuteAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinuteAsPaddedString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinuteAsPaddedString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn Second(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Second)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSecond(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSecond)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddSeconds(&self, seconds: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSeconds)(::windows_core::Interface::as_raw(this), seconds).ok() }
    }
    pub fn SecondAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecondAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SecondAsPaddedString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecondAsPaddedString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn Nanosecond(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Nanosecond)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNanosecond(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNanosecond)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddNanoseconds(&self, nanoseconds: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddNanoseconds)(::windows_core::Interface::as_raw(this), nanoseconds).ok() }
    }
    pub fn NanosecondAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NanosecondAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NanosecondAsPaddedString(&self, mindigits: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NanosecondAsPaddedString)(::windows_core::Interface::as_raw(this), mindigits, &mut result__).from_abi(result__)
        }
    }
    pub fn Compare<P0>(&self, other: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<Calendar>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compare)(::windows_core::Interface::as_raw(this), other.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompareDateTime(&self, other: super::Foundation::DateTime) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompareDateTime)(::windows_core::Interface::as_raw(this), other, &mut result__).from_abi(result__)
        }
    }
    pub fn CopyTo<P0>(&self, other: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Calendar>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyTo)(::windows_core::Interface::as_raw(this), other.into_param().abi()).ok() }
    }
    pub fn FirstMinuteInThisHour(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstMinuteInThisHour)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastMinuteInThisHour(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastMinuteInThisHour)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfMinutesInThisHour(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfMinutesInThisHour)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstSecondInThisMinute(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstSecondInThisMinute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastSecondInThisMinute(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastSecondInThisMinute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfSecondsInThisMinute(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfSecondsInThisMinute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDaylightSavingTime(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDaylightSavingTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCalendarDefaultCalendarAndClock<P0>(languages: P0) -> ::windows_core::Result<Calendar>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICalendarFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCalendarDefaultCalendarAndClock)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCalendar<P0>(languages: P0, calendar: &::windows_core::HSTRING, clock: &::windows_core::HSTRING) -> ::windows_core::Result<Calendar>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICalendarFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCalendar)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), ::core::mem::transmute_copy(calendar), ::core::mem::transmute_copy(clock), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCalendarWithTimeZone<P0>(languages: P0, calendar: &::windows_core::HSTRING, clock: &::windows_core::HSTRING, timezoneid: &::windows_core::HSTRING) -> ::windows_core::Result<Calendar>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICalendarFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCalendarWithTimeZone)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), ::core::mem::transmute_copy(calendar), ::core::mem::transmute_copy(clock), ::core::mem::transmute_copy(timezoneid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetTimeZone(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTimeZone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeTimeZone(&self, timezoneid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ChangeTimeZone)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(timezoneid)).ok() }
    }
    pub fn TimeZoneAsFullString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeZoneAsFullString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TimeZoneAsString(&self, ideallength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeZoneAsString)(::windows_core::Interface::as_raw(this), ideallength, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICalendarFactory<R, F: FnOnce(&ICalendarFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Calendar, ICalendarFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICalendarFactory2<R, F: FnOnce(&ICalendarFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Calendar, ICalendarFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Calendar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Calendar {}
impl ::core::fmt::Debug for Calendar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Calendar").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Calendar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Calendar;{ca30221d-86d9-40fb-a26b-d44eb7cf08ea})");
}
impl ::core::clone::Clone for Calendar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Calendar {
    type Vtable = ICalendar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Calendar {
    const IID: ::windows_core::GUID = <ICalendar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Calendar {
    const NAME: &'static str = "Windows.Globalization.Calendar";
}
::windows_core::imp::interface_hierarchy!(Calendar, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Calendar {}
unsafe impl ::core::marker::Sync for Calendar {}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct CalendarIdentifiers;
impl CalendarIdentifiers {
    pub fn Gregorian() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gregorian)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Hebrew() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hebrew)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Hijri() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hijri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Japanese() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Japanese)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Julian() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Julian)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Korean() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Korean)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Taiwan() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Taiwan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Thai() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thai)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UmAlQura() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UmAlQura)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Persian() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Persian)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ChineseLunar() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChineseLunar)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JapaneseLunar() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JapaneseLunar)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KoreanLunar() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KoreanLunar)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TaiwanLunar() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaiwanLunar)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VietnameseLunar() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VietnameseLunar)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICalendarIdentifiersStatics<R, F: FnOnce(&ICalendarIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CalendarIdentifiers, ICalendarIdentifiersStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICalendarIdentifiersStatics2<R, F: FnOnce(&ICalendarIdentifiersStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CalendarIdentifiers, ICalendarIdentifiersStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICalendarIdentifiersStatics3<R, F: FnOnce(&ICalendarIdentifiersStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CalendarIdentifiers, ICalendarIdentifiersStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CalendarIdentifiers {
    const NAME: &'static str = "Windows.Globalization.CalendarIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct ClockIdentifiers;
impl ClockIdentifiers {
    pub fn TwelveHour() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IClockIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TwelveHour)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TwentyFourHour() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IClockIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TwentyFourHour)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IClockIdentifiersStatics<R, F: FnOnce(&IClockIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ClockIdentifiers, IClockIdentifiersStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ClockIdentifiers {
    const NAME: &'static str = "Windows.Globalization.ClockIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct CurrencyAmount(::windows_core::IUnknown);
impl CurrencyAmount {
    pub fn Amount(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Amount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Currency(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Currency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(amount: &::windows_core::HSTRING, currency: &::windows_core::HSTRING) -> ::windows_core::Result<CurrencyAmount> {
        Self::ICurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(amount), ::core::mem::transmute_copy(currency), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrencyAmountFactory<R, F: FnOnce(&ICurrencyAmountFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrencyAmount, ICurrencyAmountFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CurrencyAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrencyAmount {}
impl ::core::fmt::Debug for CurrencyAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyAmount").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CurrencyAmount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.CurrencyAmount;{74b49942-eb75-443a-95b3-7d723f56f93c})");
}
impl ::core::clone::Clone for CurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CurrencyAmount {
    type Vtable = ICurrencyAmount_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CurrencyAmount {
    const IID: ::windows_core::GUID = <ICurrencyAmount as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CurrencyAmount {
    const NAME: &'static str = "Windows.Globalization.CurrencyAmount";
}
::windows_core::imp::interface_hierarchy!(CurrencyAmount, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CurrencyAmount {}
unsafe impl ::core::marker::Sync for CurrencyAmount {}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct CurrencyIdentifiers;
impl CurrencyIdentifiers {
    pub fn AED() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AED)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AFN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AFN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ALL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ALL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AMD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AMD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ANG() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ANG)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AOA() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AOA)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ARS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ARS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AUD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AUD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AWG() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AWG)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AZN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AZN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BAM() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BAM)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BBD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BBD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BDT() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BDT)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BGN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BGN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BHD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BHD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BIF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BIF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BMD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BMD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BND() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BND)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BOB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BOB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BRL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BRL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BSD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BSD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BTN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BTN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BWP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BWP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BYR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BYR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BZD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BZD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CAD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CAD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CDF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CDF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CHF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CHF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CLP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CLP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CNY() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CNY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn COP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).COP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CRC() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CRC)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CUP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CUP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CVE() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CVE)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CZK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CZK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DJF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DJF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DKK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DKK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DOP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DOP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DZD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DZD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn EGP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EGP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ERN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ERN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ETB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ETB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn EUR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EUR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FJD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FJD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FKP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FKP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GBP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GBP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GEL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GEL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GHS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GHS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GIP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GIP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GMD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GMD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GNF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GNF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GTQ() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GTQ)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GYD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GYD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HKD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HKD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HNL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HNL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HRK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HRK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HTG() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HTG)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HUF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HUF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IDR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IDR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ILS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ILS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn INR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).INR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IQD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IQD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IRR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IRR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ISK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ISK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JMD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JMD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JOD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JOD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JPY() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JPY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KES)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KGS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KGS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KHR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KHR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KMF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KMF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KPW() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KPW)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KRW() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KRW)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KWD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KWD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KYD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KYD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn KZT() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KZT)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LAK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LAK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LBP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LBP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LKR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LKR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LRD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LRD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LSL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LSL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LTL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LTL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LVL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LVL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LYD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LYD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MAD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MAD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MDL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MDL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MGA() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MGA)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MKD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MKD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MMK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MMK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MNT() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MNT)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MOP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MOP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MRO() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MRO)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MUR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MUR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MVR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MVR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MWK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MWK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MXN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MXN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MYR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MYR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MZN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MZN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NAD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NAD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NGN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NGN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NIO() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NIO)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NOK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NOK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NPR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NPR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NZD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NZD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn OMR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OMR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PAB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PAB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PEN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PEN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PGK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PGK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PHP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PHP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PKR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PKR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PLN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PLN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PYG() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PYG)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn QAR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QAR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RON() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RON)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RSD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RSD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RUB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RUB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RWF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RWF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SAR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SAR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SBD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SBD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SCR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SCR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SDG() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SDG)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SEK() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SEK)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SGD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SGD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SHP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SHP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SLL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SLL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SOS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SOS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SRD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SRD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn STD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).STD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SYP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SYP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SZL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SZL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn THB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).THB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TJS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TJS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TMT() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TMT)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TND() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TND)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TOP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TOP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TRY() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TRY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TTD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TTD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TWD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TWD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TZS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TZS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UAH() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UAH)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UGX() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UGX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn USD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).USD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UYU() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UYU)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UZS() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UZS)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VEF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VEF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VND() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VND)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VUV() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VUV)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn WST() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WST)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn XAF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XAF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn XCD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XCD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn XOF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XOF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn XPF() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XPF)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn XXX() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XXX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn YER() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YER)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZAR() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZAR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZMW() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZMW)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZWL() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZWL)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BYN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BYN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MRU() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MRU)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SSP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SSP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn STN() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).STN)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VES)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrencyIdentifiersStatics<R, F: FnOnce(&ICurrencyIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrencyIdentifiers, ICurrencyIdentifiersStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrencyIdentifiersStatics2<R, F: FnOnce(&ICurrencyIdentifiersStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrencyIdentifiers, ICurrencyIdentifiersStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrencyIdentifiersStatics3<R, F: FnOnce(&ICurrencyIdentifiersStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrencyIdentifiers, ICurrencyIdentifiersStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CurrencyIdentifiers {
    const NAME: &'static str = "Windows.Globalization.CurrencyIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct GeographicRegion(::windows_core::IUnknown);
impl GeographicRegion {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeographicRegion, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Code(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Code)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CodeTwoLetter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CodeTwoLetter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CodeThreeLetter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CodeThreeLetter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CodeThreeDigit(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CodeThreeDigit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NativeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NativeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CurrenciesInUse(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrenciesInUse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateGeographicRegion(geographicregioncode: &::windows_core::HSTRING) -> ::windows_core::Result<GeographicRegion> {
        Self::IGeographicRegionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateGeographicRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(geographicregioncode), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported(geographicregioncode: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IGeographicRegionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(geographicregioncode), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeographicRegionFactory<R, F: FnOnce(&IGeographicRegionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeographicRegion, IGeographicRegionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeographicRegionStatics<R, F: FnOnce(&IGeographicRegionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeographicRegion, IGeographicRegionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GeographicRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeographicRegion {}
impl ::core::fmt::Debug for GeographicRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeographicRegion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeographicRegion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.GeographicRegion;{01e9a621-4a64-4ed9-954f-9edeb07bd903})");
}
impl ::core::clone::Clone for GeographicRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GeographicRegion {
    type Vtable = IGeographicRegion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeographicRegion {
    const IID: ::windows_core::GUID = <IGeographicRegion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeographicRegion {
    const NAME: &'static str = "Windows.Globalization.GeographicRegion";
}
::windows_core::imp::interface_hierarchy!(GeographicRegion, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GeographicRegion {}
unsafe impl ::core::marker::Sync for GeographicRegion {}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct JapanesePhoneme(::windows_core::IUnknown);
impl JapanesePhoneme {
    pub fn DisplayText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn YomiText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YomiText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPhraseStart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPhraseStart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for JapanesePhoneme {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JapanesePhoneme {}
impl ::core::fmt::Debug for JapanesePhoneme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JapanesePhoneme").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for JapanesePhoneme {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.JapanesePhoneme;{2f6a9300-e85b-43e6-897d-5d82f862df21})");
}
impl ::core::clone::Clone for JapanesePhoneme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for JapanesePhoneme {
    type Vtable = IJapanesePhoneme_Vtbl;
}
unsafe impl ::windows_core::ComInterface for JapanesePhoneme {
    const IID: ::windows_core::GUID = <IJapanesePhoneme as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for JapanesePhoneme {
    const NAME: &'static str = "Windows.Globalization.JapanesePhoneme";
}
::windows_core::imp::interface_hierarchy!(JapanesePhoneme, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct JapanesePhoneticAnalyzer;
impl JapanesePhoneticAnalyzer {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWords(input: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<JapanesePhoneme>> {
        Self::IJapanesePhoneticAnalyzerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetWords)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWordsWithMonoRubyOption(input: &::windows_core::HSTRING, monoruby: bool) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<JapanesePhoneme>> {
        Self::IJapanesePhoneticAnalyzerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetWordsWithMonoRubyOption)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(input), monoruby, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJapanesePhoneticAnalyzerStatics<R, F: FnOnce(&IJapanesePhoneticAnalyzerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<JapanesePhoneticAnalyzer, IJapanesePhoneticAnalyzerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for JapanesePhoneticAnalyzer {
    const NAME: &'static str = "Windows.Globalization.JapanesePhoneticAnalyzer";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct Language(::windows_core::IUnknown);
impl Language {
    pub fn LanguageTag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LanguageTag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NativeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NativeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Script(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Script)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LayoutDirection(&self) -> ::windows_core::Result<LanguageLayoutDirection> {
        let this = &::windows_core::ComInterface::cast::<ILanguage2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AbbreviatedName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILanguage3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbbreviatedName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionSubtags(&self, singleton: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<ILanguageExtensionSubtags>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetExtensionSubtags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(singleton), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLanguage(languagetag: &::windows_core::HSTRING) -> ::windows_core::Result<Language> {
        Self::ILanguageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    pub fn IsWellFormed(languagetag: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::ILanguageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsWellFormed)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    pub fn CurrentInputMethodLanguageTag() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILanguageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentInputMethodLanguageTag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TrySetInputMethodLanguageTag(languagetag: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::ILanguageStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetInputMethodLanguageTag)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMuiCompatibleLanguageListFromLanguageTags<P0>(languagetags: P0) -> ::windows_core::Result<super::Foundation::Collections::IVector<::windows_core::HSTRING>>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ILanguageStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMuiCompatibleLanguageListFromLanguageTags)(::windows_core::Interface::as_raw(this), languagetags.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILanguageFactory<R, F: FnOnce(&ILanguageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Language, ILanguageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILanguageStatics<R, F: FnOnce(&ILanguageStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Language, ILanguageStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILanguageStatics2<R, F: FnOnce(&ILanguageStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Language, ILanguageStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILanguageStatics3<R, F: FnOnce(&ILanguageStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Language, ILanguageStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Language {}
impl ::core::fmt::Debug for Language {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Language").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Language {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Language;{ea79a752-f7c2-4265-b1bd-c4dec4e4f080})");
}
impl ::core::clone::Clone for Language {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Language {
    type Vtable = ILanguage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Language {
    const IID: ::windows_core::GUID = <ILanguage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Language {
    const NAME: &'static str = "Windows.Globalization.Language";
}
::windows_core::imp::interface_hierarchy!(Language, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Language {}
unsafe impl ::core::marker::Sync for Language {}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct NumeralSystemIdentifiers;
impl NumeralSystemIdentifiers {
    pub fn Arab() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arab)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ArabExt() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ArabExt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Bali() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bali)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Beng() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Beng)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Cham() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cham)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Deva() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deva)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FullWide() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullWide)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Gujr() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gujr)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Guru() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Guru)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HaniDec() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HaniDec)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Java() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Java)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Kali() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kali)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Khmr() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Khmr)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Knda() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Knda)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Lana() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lana)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LanaTham() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LanaTham)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Laoo() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Laoo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Latn() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Latn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Lepc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lepc)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Limb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Limb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mlym() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mlym)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mong() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mong)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mtei() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mtei)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mymr() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mymr)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MymrShan() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MymrShan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Nkoo() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Nkoo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Olck() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Olck)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Orya() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orya)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Saur() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Saur)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Sund() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Sund)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Talu() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Talu)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TamlDec() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TamlDec)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Telu() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Telu)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Thai() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thai)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Tibt() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tibt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Vaii() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Vaii)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Brah() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Brah)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Osma() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Osma)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MathBold() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MathBold)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MathDbl() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MathDbl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MathSans() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MathSans)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MathSanb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MathSanb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MathMono() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MathMono)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZmthBold() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZmthBold)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZmthDbl() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZmthDbl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZmthSans() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZmthSans)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZmthSanb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZmthSanb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ZmthMono() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZmthMono)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INumeralSystemIdentifiersStatics<R, F: FnOnce(&INumeralSystemIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NumeralSystemIdentifiers, INumeralSystemIdentifiersStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INumeralSystemIdentifiersStatics2<R, F: FnOnce(&INumeralSystemIdentifiersStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NumeralSystemIdentifiers, INumeralSystemIdentifiersStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for NumeralSystemIdentifiers {
    const NAME: &'static str = "Windows.Globalization.NumeralSystemIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for DayOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DayOfWeek {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DayOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayOfWeek").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DayOfWeek {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DayOfWeek;i4)");
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for LanguageLayoutDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LanguageLayoutDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LanguageLayoutDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanguageLayoutDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LanguageLayoutDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.LanguageLayoutDirection;i4)");
}
