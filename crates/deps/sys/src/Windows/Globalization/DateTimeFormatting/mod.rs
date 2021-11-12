#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DateTimeFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: DayFormat = DayFormat(0i32);
    pub const Default: DayFormat = DayFormat(1i32);
}
#[repr(transparent)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: DayOfWeekFormat = DayOfWeekFormat(0i32);
    pub const Default: DayOfWeekFormat = DayOfWeekFormat(1i32);
    pub const Abbreviated: DayOfWeekFormat = DayOfWeekFormat(2i32);
    pub const Full: DayOfWeekFormat = DayOfWeekFormat(3i32);
}
#[repr(transparent)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: HourFormat = HourFormat(0i32);
    pub const Default: HourFormat = HourFormat(1i32);
}
#[repr(transparent)]
pub struct IDateTimeFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeFormatter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeFormatterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeFormatterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: MinuteFormat = MinuteFormat(0i32);
    pub const Default: MinuteFormat = MinuteFormat(1i32);
}
#[repr(transparent)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: MonthFormat = MonthFormat(0i32);
    pub const Default: MonthFormat = MonthFormat(1i32);
    pub const Abbreviated: MonthFormat = MonthFormat(2i32);
    pub const Full: MonthFormat = MonthFormat(3i32);
    pub const Numeric: MonthFormat = MonthFormat(4i32);
}
#[repr(transparent)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: SecondFormat = SecondFormat(0i32);
    pub const Default: SecondFormat = SecondFormat(1i32);
}
#[repr(transparent)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: YearFormat = YearFormat(0i32);
    pub const Default: YearFormat = YearFormat(1i32);
    pub const Abbreviated: YearFormat = YearFormat(2i32);
    pub const Full: YearFormat = YearFormat(3i32);
}
