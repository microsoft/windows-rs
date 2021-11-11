#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_LicenseProtection`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterLicenseKeyWithExpiration();
    #[doc = "*Required features: `Win32_Security_LicenseProtection`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateLicenseKeyProtection();
}
