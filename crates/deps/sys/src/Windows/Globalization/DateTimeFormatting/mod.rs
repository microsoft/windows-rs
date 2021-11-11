#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DateTimeFormatter();
    fn DayFormat();
    fn DayOfWeekFormat();
    fn HourFormat();
    fn IDateTimeFormatter();
    fn IDateTimeFormatter2();
    fn IDateTimeFormatterFactory();
    fn IDateTimeFormatterStatics();
    fn MinuteFormat();
    fn MonthFormat();
    fn SecondFormat();
    fn YearFormat();
}
