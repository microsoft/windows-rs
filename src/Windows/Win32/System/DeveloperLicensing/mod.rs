#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AcquireDeveloperLicense<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndparent: Param0,
) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireDeveloperLicense(
                hwndparent: super::super::Foundation::HWND,
                pexpiration: *mut super::super::Foundation::FILETIME,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        AcquireDeveloperLicense(hwndparent.into_param().abi(), &mut result__)
            .from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CheckDeveloperLicense(
) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckDeveloperLicense(
                pexpiration: *mut super::super::Foundation::FILETIME,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        CheckDeveloperLicense(&mut result__)
            .from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveDeveloperLicense<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndparent: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDeveloperLicense(
                hwndparent: super::super::Foundation::HWND,
            ) -> ::windows::runtime::HRESULT;
        }
        RemoveDeveloperLicense(hwndparent.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
