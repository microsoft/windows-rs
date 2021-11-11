#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EnumDynamicTimeZoneInformation();
    fn FileTimeToSystemTime();
    fn GetDynamicTimeZoneInformation();
    fn GetDynamicTimeZoneInformationEffectiveYears();
    fn GetTimeZoneInformation();
    fn GetTimeZoneInformationForYear();
    fn LocalFileTimeToLocalSystemTime();
    fn LocalSystemTimeToLocalFileTime();
    fn SetDynamicTimeZoneInformation();
    fn SetTimeZoneInformation();
    fn SystemTimeToFileTime();
    fn SystemTimeToTzSpecificLocalTime();
    fn SystemTimeToTzSpecificLocalTimeEx();
    fn TzSpecificLocalTimeToSystemTime();
    fn TzSpecificLocalTimeToSystemTimeEx();
}
