#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPhoneCallOrigin();
    fn IPhoneCallOrigin2();
    fn IPhoneCallOrigin3();
    fn IPhoneCallOriginManagerStatics();
    fn IPhoneCallOriginManagerStatics2();
    fn IPhoneCallOriginManagerStatics3();
    fn PhoneCallOrigin();
    fn PhoneCallOriginManager();
}
