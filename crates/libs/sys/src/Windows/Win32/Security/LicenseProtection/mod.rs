windows_link::link!("licenseprotection.dll" "system" fn RegisterLicenseKeyWithExpiration(licensekey : windows_sys::core::PCWSTR, validityindays : u32, status : *mut LicenseProtectionStatus) -> windows_sys::core::HRESULT);
windows_link::link!("licenseprotection.dll" "system" fn ValidateLicenseKeyProtection(licensekey : windows_sys::core::PCWSTR, notvalidbefore : *mut super::super::Foundation::FILETIME, notvalidafter : *mut super::super::Foundation::FILETIME, status : *mut LicenseProtectionStatus) -> windows_sys::core::HRESULT);
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4;
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3;
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1;
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2;
pub type LicenseProtectionStatus = i32;
pub const Success: LicenseProtectionStatus = 0;
