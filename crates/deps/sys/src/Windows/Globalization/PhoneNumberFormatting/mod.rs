#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPhoneNumberFormatter();
    fn IPhoneNumberFormatterStatics();
    fn IPhoneNumberInfo();
    fn IPhoneNumberInfoFactory();
    fn IPhoneNumberInfoStatics();
    fn PhoneNumberFormat();
    fn PhoneNumberFormatter();
    fn PhoneNumberInfo();
    fn PhoneNumberMatchResult();
    fn PhoneNumberParseResult();
    fn PredictedPhoneNumberKind();
}
