#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ILicenseManagerStatics();
    fn ILicenseManagerStatics2();
    fn ILicenseSatisfactionInfo();
    fn ILicenseSatisfactionResult();
    fn LicenseManager();
    fn LicenseRefreshOption();
    fn LicenseSatisfactionInfo();
    fn LicenseSatisfactionResult();
}
