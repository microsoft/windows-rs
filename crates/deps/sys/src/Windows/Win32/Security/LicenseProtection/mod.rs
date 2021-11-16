#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterLicenseKeyWithExpiration(licensekey: super::super::Foundation::PWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateLicenseKeyProtection(licensekey: super::super::Foundation::PWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_sys::core::HRESULT;
}
pub const Success: i32 = 0i32;
pub const LicenseKeyNotFound: i32 = 1i32;
pub const LicenseKeyUnprotected: i32 = 2i32;
pub const LicenseKeyCorrupted: i32 = 3i32;
pub const LicenseKeyAlreadyExists: i32 = 4i32;
