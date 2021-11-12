#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LicenseProtectionStatus(pub i32);
pub const Success: LicenseProtectionStatus = LicenseProtectionStatus(0i32);
pub const LicenseKeyNotFound: LicenseProtectionStatus = LicenseProtectionStatus(1i32);
pub const LicenseKeyUnprotected: LicenseProtectionStatus = LicenseProtectionStatus(2i32);
pub const LicenseKeyCorrupted: LicenseProtectionStatus = LicenseProtectionStatus(3i32);
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = LicenseProtectionStatus(4i32);
impl ::core::convert::From<i32> for LicenseProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LicenseProtectionStatus {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(licensekey: Param0, validityindays: u32) -> ::windows::core::Result<LicenseProtectionStatus> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterLicenseKeyWithExpiration(licensekey: super::super::Foundation::PWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows::core::HRESULT;
        }
        let mut result__: <LicenseProtectionStatus as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        RegisterLicenseKeyWithExpiration(licensekey.into_param().abi(), ::core::mem::transmute(validityindays), &mut result__).from_abi::<LicenseProtectionStatus>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
