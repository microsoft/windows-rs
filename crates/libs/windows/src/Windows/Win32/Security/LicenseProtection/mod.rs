#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<P0>(licensekey: P0, validityindays: u32) -> ::windows::core::Result<LicenseProtectionStatus>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "licenseprotection.dll""system" fn RegisterLicenseKeyWithExpiration ( licensekey : :: windows::core::PCWSTR , validityindays : u32 , status : *mut LicenseProtectionStatus ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<LicenseProtectionStatus>();
    RegisterLicenseKeyWithExpiration(licensekey.into_param().abi(), validityindays, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidateLicenseKeyProtection<P0>(licensekey: P0, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "licenseprotection.dll""system" fn ValidateLicenseKeyProtection ( licensekey : :: windows::core::PCWSTR , notvalidbefore : *mut super::super::Foundation:: FILETIME , notvalidafter : *mut super::super::Foundation:: FILETIME , status : *mut LicenseProtectionStatus ) -> :: windows::core::HRESULT );
    ValidateLicenseKeyProtection(licensekey.into_param().abi(), notvalidbefore, notvalidafter, status).ok()
}
#[doc = "*Required features: `\"Win32_Security_LicenseProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for LicenseProtectionStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LicenseProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseProtectionStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
