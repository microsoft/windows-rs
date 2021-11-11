#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn LicenseProtectionStatus();
    fn RegisterLicenseKeyWithExpiration();
    fn ValidateLicenseKeyProtection();
}
