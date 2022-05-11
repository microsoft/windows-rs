#[doc = "*Required features: `\"Win32_System_DeveloperLicensing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AcquireDeveloperLicense<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireDeveloperLicense(hwndparent: super::super::Foundation::HWND, pexpiration: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::FILETIME>::zeroed();
        AcquireDeveloperLicense(hwndparent.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_DeveloperLicensing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckDeveloperLicense() -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckDeveloperLicense(pexpiration: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::FILETIME>::zeroed();
        CheckDeveloperLicense(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_DeveloperLicensing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDeveloperLicense<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDeveloperLicense(hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
        }
        RemoveDeveloperLicense(hwndparent.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
