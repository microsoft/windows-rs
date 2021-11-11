#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DYNAMIC_TIME_ZONE_INFORMATION();
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
    fn TIME_ZONE_INFORMATION();
    fn TSF_Authenticated();
    fn TSF_Hardware();
    fn TSF_IPv6();
    fn TSF_SignatureAuthenticated();
    fn TzSpecificLocalTimeToSystemTime();
    fn TzSpecificLocalTimeToSystemTimeEx();
}
