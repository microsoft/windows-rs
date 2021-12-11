#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub type LicenseProtectionStatus = i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const Success: LicenseProtectionStatus = 0i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(licensekey: Param0, validityindays: u32) -> ::windows::core::Result<LicenseProtectionStatus> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterLicenseKeyWithExpiration(licensekey: super::super::Foundation::PWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows::core::HRESULT;
        }
        let mut result__: LicenseProtectionStatus = ::core::mem::zeroed();
        RegisterLicenseKeyWithExpiration(licensekey.into_param().abi(), ::core::mem::transmute(validityindays), ::core::mem::transmute(&mut result__)).from_abi::<LicenseProtectionStatus>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Security_LicenseProtection', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidateLicenseKeyProtection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(licensekey: Param0, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidateLicenseKeyProtection(licensekey: super::super::Foundation::PWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows::core::HRESULT;
        }
        ValidateLicenseKeyProtection(licensekey.into_param().abi(), ::core::mem::transmute(notvalidbefore), ::core::mem::transmute(notvalidafter), ::core::mem::transmute(status)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
