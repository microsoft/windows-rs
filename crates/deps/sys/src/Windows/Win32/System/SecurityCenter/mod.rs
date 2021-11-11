#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWSCDefaultProduct();
    fn IWSCProductList();
    fn IWscProduct();
    fn IWscProduct2();
    fn IWscProduct3();
    fn SECURITY_PRODUCT_TYPE();
    fn WSCDefaultProduct();
    fn WSCProductList();
    fn WSC_SECURITY_PRODUCT_STATE();
    fn WSC_SECURITY_PRODUCT_SUBSTATUS();
    fn WSC_SECURITY_PROVIDER();
    fn WSC_SECURITY_PROVIDER_HEALTH();
    fn WSC_SECURITY_SIGNATURE_STATUS();
    fn WscGetAntiMalwareUri();
    fn WscGetSecurityProviderHealth();
    fn WscQueryAntiMalwareUri();
    fn WscRegisterForChanges();
    fn WscRegisterForUserNotifications();
    fn WscUnRegisterChanges();
}
