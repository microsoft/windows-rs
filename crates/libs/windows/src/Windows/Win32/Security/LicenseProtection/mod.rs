#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LicenseProtectionStatus(pub i32);
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
pub const Success: LicenseProtectionStatus = LicenseProtectionStatus(0i32);
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
pub const LicenseKeyNotFound: LicenseProtectionStatus = LicenseProtectionStatus(1i32);
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
pub const LicenseKeyUnprotected: LicenseProtectionStatus = LicenseProtectionStatus(2i32);
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
pub const LicenseKeyCorrupted: LicenseProtectionStatus = LicenseProtectionStatus(3i32);
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = LicenseProtectionStatus(4i32);
impl ::core::marker::Copy for LicenseProtectionStatus {}
impl ::core::clone::Clone for LicenseProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LicenseProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LicenseProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LicenseProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseProtectionStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(licensekey: Param0, validityindays: u32) -> ::windows::core::Result<LicenseProtectionStatus> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterLicenseKeyWithExpiration(licensekey: ::windows::core::PCWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows::core::HRESULT;
        }
        let mut result__: LicenseProtectionStatus = ::core::mem::zeroed();
        RegisterLicenseKeyWithExpiration(licensekey.into_param().abi(), ::core::mem::transmute(validityindays), ::core::mem::transmute(&mut result__)).from_abi::<LicenseProtectionStatus>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidateLicenseKeyProtection<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(licensekey: Param0, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidateLicenseKeyProtection(licensekey: ::windows::core::PCWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows::core::HRESULT;
        }
        ValidateLicenseKeyProtection(licensekey.into_param().abi(), ::core::mem::transmute(notvalidbefore), ::core::mem::transmute(notvalidafter), ::core::mem::transmute(status)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
