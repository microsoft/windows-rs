#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DateTimeFormatter(pub *mut ::core::ffi::c_void);
pub struct DayFormat(i32);
pub struct DayOfWeekFormat(i32);
pub struct HourFormat(i32);
#[repr(transparent)]
pub struct IDateTimeFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeFormatter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeFormatterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDateTimeFormatterStatics(pub *mut ::core::ffi::c_void);
pub struct MinuteFormat(i32);
pub struct MonthFormat(i32);
pub struct SecondFormat(i32);
pub struct YearFormat(i32);
