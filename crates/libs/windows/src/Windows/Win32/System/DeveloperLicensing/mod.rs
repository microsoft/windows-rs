#[doc = "*Required features: `\"Win32_System_DeveloperLicensing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AcquireDeveloperLicense<P0>(hwndparent: P0) -> ::windows::core::Result<super::super::Foundation::FILETIME>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "wsclient.dll""system" fn AcquireDeveloperLicense ( hwndparent : super::super::Foundation:: HWND , pexpiration : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    AcquireDeveloperLicense(hwndparent.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_DeveloperLicensing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckDeveloperLicense() -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    ::windows::core::link ! ( "wsclient.dll""system" fn CheckDeveloperLicense ( pexpiration : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CheckDeveloperLicense(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_DeveloperLicensing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDeveloperLicense<P0>(hwndparent: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "wsclient.dll""system" fn RemoveDeveloperLicense ( hwndparent : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    RemoveDeveloperLicense(hwndparent.into()).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
