#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WNV_API_MAJOR_VERSION_1();
    fn WNV_API_MINOR_VERSION_0();
    fn WNV_CA_NOTIFICATION_TYPE();
    fn WNV_CUSTOMER_ADDRESS_CHANGE_PARAM();
    fn WNV_IP_ADDRESS();
    fn WNV_NOTIFICATION_PARAM();
    fn WNV_NOTIFICATION_TYPE();
    fn WNV_OBJECT_CHANGE_PARAM();
    fn WNV_OBJECT_HEADER();
    fn WNV_OBJECT_TYPE();
    fn WNV_POLICY_MISMATCH_PARAM();
    fn WNV_PROVIDER_ADDRESS_CHANGE_PARAM();
    fn WNV_REDIRECT_PARAM();
    fn WnvOpen();
    fn WnvRequestNotification();
}
