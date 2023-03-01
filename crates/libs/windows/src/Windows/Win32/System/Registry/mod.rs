#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegistryValueWithFallbackW<P0, P1, P2, P3, P4>(hkeyprimary: P0, pwszprimarysubkey: P1, hkeyfallback: P2, pwszfallbacksubkey: P3, pwszvalue: P4, dwflags: u32, pdwtype: ::core::option::Option<*mut u32>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, cbdatain: u32, pcbdataout: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<HKEY>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "api-ms-win-core-state-helpers-l1-1-0.dll""system" fn GetRegistryValueWithFallbackW ( hkeyprimary : HKEY , pwszprimarysubkey : :: windows::core::PCWSTR , hkeyfallback : HKEY , pwszfallbacksubkey : :: windows::core::PCWSTR , pwszvalue : :: windows::core::PCWSTR , dwflags : u32 , pdwtype : *mut u32 , pvdata : *mut ::core::ffi::c_void , cbdatain : u32 , pcbdataout : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    GetRegistryValueWithFallbackW(hkeyprimary.into_param().abi(), pwszprimarysubkey.into_param().abi(), hkeyfallback.into_param().abi(), pwszfallbacksubkey.into_param().abi(), pwszvalue.into_param().abi(), dwflags, ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), cbdatain, ::core::mem::transmute(pcbdataout.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCloseKey<P0>(hkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCloseKey ( hkey : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegCloseKey(hkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryA<P0, P1>(lpmachinename: P0, hkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegConnectRegistryA ( lpmachinename : :: windows::core::PCSTR , hkey : HKEY , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegConnectRegistryA(lpmachinename.into_param().abi(), hkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[inline]
pub unsafe fn RegConnectRegistryExA<P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: *mut HKEY) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegConnectRegistryExA ( lpmachinename : :: windows::core::PCSTR , hkey : HKEY , flags : u32 , phkresult : *mut HKEY ) -> i32 );
    RegConnectRegistryExA(lpmachinename.into_param().abi(), hkey.into_param().abi(), flags, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[inline]
pub unsafe fn RegConnectRegistryExW<P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: *mut HKEY) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegConnectRegistryExW ( lpmachinename : :: windows::core::PCWSTR , hkey : HKEY , flags : u32 , phkresult : *mut HKEY ) -> i32 );
    RegConnectRegistryExW(lpmachinename.into_param().abi(), hkey.into_param().abi(), flags, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryW<P0, P1>(lpmachinename: P0, hkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegConnectRegistryW ( lpmachinename : :: windows::core::PCWSTR , hkey : HKEY , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegConnectRegistryW(lpmachinename.into_param().abi(), hkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeA<P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCopyTreeA ( hkeysrc : HKEY , lpsubkey : :: windows::core::PCSTR , hkeydest : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegCopyTreeA(hkeysrc.into_param().abi(), lpsubkey.into_param().abi(), hkeydest.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeW<P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCopyTreeW ( hkeysrc : HKEY , lpsubkey : :: windows::core::PCWSTR , hkeydest : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegCopyTreeW(hkeysrc.into_param().abi(), lpsubkey.into_param().abi(), hkeydest.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyA<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCreateKeyA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegCreateKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExA<P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCreateKeyExA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , reserved : u32 , lpclass : :: windows::core::PCSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut HKEY , lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION ) -> super::super::Foundation:: WIN32_ERROR );
    RegCreateKeyExA(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExW<P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCreateKeyExW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , reserved : u32 , lpclass : :: windows::core::PCWSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut HKEY , lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION ) -> super::super::Foundation:: WIN32_ERROR );
    RegCreateKeyExW(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedA<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCreateKeyTransactedA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , reserved : u32 , lpclass : :: windows::core::PCSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut HKEY , lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION , htransaction : super::super::Foundation:: HANDLE , pextendedparemeter : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    RegCreateKeyTransactedA(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedW<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCreateKeyTransactedW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , reserved : u32 , lpclass : :: windows::core::PCWSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut HKEY , lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION , htransaction : super::super::Foundation:: HANDLE , pextendedparemeter : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    RegCreateKeyTransactedW(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyW<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegCreateKeyW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegCreateKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyA<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExA<P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyExA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , samdesired : u32 , reserved : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyExA(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExW<P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyExW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , samdesired : u32 , reserved : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyExW(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedA<P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyTransactedA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , samdesired : u32 , reserved : u32 , htransaction : super::super::Foundation:: HANDLE , pextendedparameter : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyTransactedA(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparameter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedW<P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyTransactedW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , samdesired : u32 , reserved : u32 , htransaction : super::super::Foundation:: HANDLE , pextendedparameter : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyTransactedW(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparameter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyValueA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , lpvaluename : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyValueW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , lpvaluename : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyW<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteKeyW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeA<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteTreeA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteTreeA(hkey.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeW<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteTreeW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteTreeW(hkey.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueA<P0, P1>(hkey: P0, lpvaluename: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteValueA ( hkey : HKEY , lpvaluename : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteValueA(hkey.into_param().abi(), lpvaluename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueW<P0, P1>(hkey: P0, lpvaluename: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDeleteValueW ( hkey : HKEY , lpvaluename : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegDeleteValueW(hkey.into_param().abi(), lpvaluename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCache() -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDisablePredefinedCache ( ) -> super::super::Foundation:: WIN32_ERROR );
    RegDisablePredefinedCache()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCacheEx() -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDisablePredefinedCacheEx ( ) -> super::super::Foundation:: WIN32_ERROR );
    RegDisablePredefinedCacheEx()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisableReflectionKey<P0>(hbase: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegDisableReflectionKey ( hbase : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegDisableReflectionKey(hbase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnableReflectionKey<P0>(hbase: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnableReflectionKey ( hbase : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnableReflectionKey(hbase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyA<P0>(hkey: P0, dwindex: u32, lpname: ::core::option::Option<&mut [u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnumKeyA ( hkey : HKEY , dwindex : u32 , lpname : :: windows::core::PSTR , cchname : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnumKeyA(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExA<P0>(hkey: P0, dwindex: u32, lpname: ::windows::core::PSTR, lpcchname: *mut u32, lpreserved: ::core::option::Option<*const u32>, lpclass: ::windows::core::PSTR, lpcchclass: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnumKeyExA ( hkey : HKEY , dwindex : u32 , lpname : :: windows::core::PSTR , lpcchname : *mut u32 , lpreserved : *const u32 , lpclass : :: windows::core::PSTR , lpcchclass : *mut u32 , lpftlastwritetime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnumKeyExA(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname), lpcchname, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExW<P0>(hkey: P0, dwindex: u32, lpname: ::windows::core::PWSTR, lpcchname: *mut u32, lpreserved: ::core::option::Option<*const u32>, lpclass: ::windows::core::PWSTR, lpcchclass: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnumKeyExW ( hkey : HKEY , dwindex : u32 , lpname : :: windows::core::PWSTR , lpcchname : *mut u32 , lpreserved : *const u32 , lpclass : :: windows::core::PWSTR , lpcchclass : *mut u32 , lpftlastwritetime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnumKeyExW(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname), lpcchname, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyW<P0>(hkey: P0, dwindex: u32, lpname: ::core::option::Option<&mut [u16]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnumKeyW ( hkey : HKEY , dwindex : u32 , lpname : :: windows::core::PWSTR , cchname : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnumKeyW(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueA<P0>(hkey: P0, dwindex: u32, lpvaluename: ::windows::core::PSTR, lpcchvaluename: *mut u32, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnumValueA ( hkey : HKEY , dwindex : u32 , lpvaluename : :: windows::core::PSTR , lpcchvaluename : *mut u32 , lpreserved : *const u32 , lptype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnumValueA(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpvaluename), lpcchvaluename, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueW<P0>(hkey: P0, dwindex: u32, lpvaluename: ::windows::core::PWSTR, lpcchvaluename: *mut u32, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegEnumValueW ( hkey : HKEY , dwindex : u32 , lpvaluename : :: windows::core::PWSTR , lpcchvaluename : *mut u32 , lpreserved : *const u32 , lptype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegEnumValueW(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpvaluename), lpcchvaluename, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegFlushKey<P0>(hkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegFlushKey ( hkey : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegFlushKey(hkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegGetKeySecurity<P0>(hkey: P0, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegGetKeySecurity ( hkey : HKEY , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegGetKeySecurity(hkey.into_param().abi(), securityinformation, psecuritydescriptor, lpcbsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: REG_ROUTINE_FLAGS, pdwtype: ::core::option::Option<*mut REG_VALUE_TYPE>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, pcbdata: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegGetValueA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , lpvalue : :: windows::core::PCSTR , dwflags : REG_ROUTINE_FLAGS , pdwtype : *mut REG_VALUE_TYPE , pvdata : *mut ::core::ffi::c_void , pcbdata : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegGetValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvalue.into_param().abi(), dwflags, ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: REG_ROUTINE_FLAGS, pdwtype: ::core::option::Option<*mut REG_VALUE_TYPE>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, pcbdata: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegGetValueW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , lpvalue : :: windows::core::PCWSTR , dwflags : REG_ROUTINE_FLAGS , pdwtype : *mut REG_VALUE_TYPE , pvdata : *mut ::core::ffi::c_void , pcbdata : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegGetValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvalue.into_param().abi(), dwflags, ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyA<P0>(lpfile: P0, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegLoadAppKeyA ( lpfile : :: windows::core::PCSTR , phkresult : *mut HKEY , samdesired : u32 , dwoptions : u32 , reserved : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegLoadAppKeyA(lpfile.into_param().abi(), phkresult, samdesired, dwoptions, reserved)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyW<P0>(lpfile: P0, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegLoadAppKeyW ( lpfile : :: windows::core::PCWSTR , phkresult : *mut HKEY , samdesired : u32 , dwoptions : u32 , reserved : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegLoadAppKeyW(lpfile.into_param().abi(), phkresult, samdesired, dwoptions, reserved)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegLoadKeyA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , lpfile : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegLoadKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegLoadKeyW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , lpfile : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegLoadKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringA<P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: ::core::option::Option<&mut [u8]>, pcbdata: ::core::option::Option<*mut u32>, flags: u32, pszdirectory: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegLoadMUIStringA ( hkey : HKEY , pszvalue : :: windows::core::PCSTR , pszoutbuf : :: windows::core::PSTR , cboutbuf : u32 , pcbdata : *mut u32 , flags : u32 , pszdirectory : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegLoadMUIStringA(hkey.into_param().abi(), pszvalue.into_param().abi(), ::core::mem::transmute(pszoutbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszoutbuf.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())), flags, pszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringW<P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: ::windows::core::PWSTR, cboutbuf: u32, pcbdata: ::core::option::Option<*mut u32>, flags: u32, pszdirectory: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegLoadMUIStringW ( hkey : HKEY , pszvalue : :: windows::core::PCWSTR , pszoutbuf : :: windows::core::PWSTR , cboutbuf : u32 , pcbdata : *mut u32 , flags : u32 , pszdirectory : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegLoadMUIStringW(hkey.into_param().abi(), pszvalue.into_param().abi(), ::core::mem::transmute(pszoutbuf), cboutbuf, ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())), flags, pszdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegNotifyChangeKeyValue<P0, P1, P2, P3>(hkey: P0, bwatchsubtree: P1, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: P2, fasynchronous: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegNotifyChangeKeyValue ( hkey : HKEY , bwatchsubtree : super::super::Foundation:: BOOL , dwnotifyfilter : REG_NOTIFY_FILTER , hevent : super::super::Foundation:: HANDLE , fasynchronous : super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
    RegNotifyChangeKeyValue(hkey.into_param().abi(), bwatchsubtree.into_param().abi(), dwnotifyfilter, hevent.into_param().abi(), fasynchronous.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenCurrentUser(samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenCurrentUser ( samdesired : u32 , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenCurrentUser(samdesired, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyA<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenKeyA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExA<P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenKeyExA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenKeyExA(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExW<P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenKeyExW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenKeyExW(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedA<P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: P2, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenKeyTransactedA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : *mut HKEY , htransaction : super::super::Foundation:: HANDLE , pextendedparemeter : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenKeyTransactedA(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedW<P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: P2, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenKeyTransactedW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : *mut HKEY , htransaction : super::super::Foundation:: HANDLE , pextendedparemeter : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenKeyTransactedW(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyW<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenKeyW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenUserClassesRoot<P0>(htoken: P0, dwoptions: u32, samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOpenUserClassesRoot ( htoken : super::super::Foundation:: HANDLE , dwoptions : u32 , samdesired : u32 , phkresult : *mut HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOpenUserClassesRoot(htoken.into_param().abi(), dwoptions, samdesired, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOverridePredefKey<P0, P1>(hkey: P0, hnewhkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegOverridePredefKey ( hkey : HKEY , hnewhkey : HKEY ) -> super::super::Foundation:: WIN32_ERROR );
    RegOverridePredefKey(hkey.into_param().abi(), hnewhkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyA<P0>(hkey: P0, lpclass: ::windows::core::PSTR, lpcchclass: ::core::option::Option<*mut u32>, lpreserved: ::core::option::Option<*const u32>, lpcsubkeys: ::core::option::Option<*mut u32>, lpcbmaxsubkeylen: ::core::option::Option<*mut u32>, lpcbmaxclasslen: ::core::option::Option<*mut u32>, lpcvalues: ::core::option::Option<*mut u32>, lpcbmaxvaluenamelen: ::core::option::Option<*mut u32>, lpcbmaxvaluelen: ::core::option::Option<*mut u32>, lpcbsecuritydescriptor: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryInfoKeyA ( hkey : HKEY , lpclass : :: windows::core::PSTR , lpcchclass : *mut u32 , lpreserved : *const u32 , lpcsubkeys : *mut u32 , lpcbmaxsubkeylen : *mut u32 , lpcbmaxclasslen : *mut u32 , lpcvalues : *mut u32 , lpcbmaxvaluenamelen : *mut u32 , lpcbmaxvaluelen : *mut u32 , lpcbsecuritydescriptor : *mut u32 , lpftlastwritetime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryInfoKeyA(
        hkey.into_param().abi(),
        ::core::mem::transmute(lpclass),
        ::core::mem::transmute(lpcchclass.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(lpcsubkeys.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxsubkeylen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxclasslen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcvalues.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxvaluenamelen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxvaluelen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyW<P0>(hkey: P0, lpclass: ::windows::core::PWSTR, lpcchclass: ::core::option::Option<*mut u32>, lpreserved: ::core::option::Option<*const u32>, lpcsubkeys: ::core::option::Option<*mut u32>, lpcbmaxsubkeylen: ::core::option::Option<*mut u32>, lpcbmaxclasslen: ::core::option::Option<*mut u32>, lpcvalues: ::core::option::Option<*mut u32>, lpcbmaxvaluenamelen: ::core::option::Option<*mut u32>, lpcbmaxvaluelen: ::core::option::Option<*mut u32>, lpcbsecuritydescriptor: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryInfoKeyW ( hkey : HKEY , lpclass : :: windows::core::PWSTR , lpcchclass : *mut u32 , lpreserved : *const u32 , lpcsubkeys : *mut u32 , lpcbmaxsubkeylen : *mut u32 , lpcbmaxclasslen : *mut u32 , lpcvalues : *mut u32 , lpcbmaxvaluenamelen : *mut u32 , lpcbmaxvaluelen : *mut u32 , lpcbsecuritydescriptor : *mut u32 , lpftlastwritetime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryInfoKeyW(
        hkey.into_param().abi(),
        ::core::mem::transmute(lpclass),
        ::core::mem::transmute(lpcchclass.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(lpcsubkeys.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxsubkeylen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxclasslen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcvalues.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxvaluenamelen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbmaxvaluelen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesA<P0>(hkey: P0, val_list: &mut [VALENTA], lpvaluebuf: ::windows::core::PSTR, ldwtotsize: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryMultipleValuesA ( hkey : HKEY , val_list : *mut VALENTA , num_vals : u32 , lpvaluebuf : :: windows::core::PSTR , ldwtotsize : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryMultipleValuesA(hkey.into_param().abi(), ::core::mem::transmute(val_list.as_ptr()), val_list.len() as _, ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(ldwtotsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesW<P0>(hkey: P0, val_list: &mut [VALENTW], lpvaluebuf: ::windows::core::PWSTR, ldwtotsize: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryMultipleValuesW ( hkey : HKEY , val_list : *mut VALENTW , num_vals : u32 , lpvaluebuf : :: windows::core::PWSTR , ldwtotsize : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryMultipleValuesW(hkey.into_param().abi(), ::core::mem::transmute(val_list.as_ptr()), val_list.len() as _, ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(ldwtotsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryReflectionKey<P0>(hbase: P0, bisreflectiondisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryReflectionKey ( hbase : HKEY , bisreflectiondisabled : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryReflectionKey(hbase.into_param().abi(), bisreflectiondisabled)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueA<P0, P1>(hkey: P0, lpsubkey: P1, lpdata: ::windows::core::PSTR, lpcbdata: ::core::option::Option<*mut i32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryValueA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , lpdata : :: windows::core::PSTR , lpcbdata : *mut i32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExA<P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut REG_VALUE_TYPE>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryValueExA ( hkey : HKEY , lpvaluename : :: windows::core::PCSTR , lpreserved : *const u32 , lptype : *mut REG_VALUE_TYPE , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryValueExA(hkey.into_param().abi(), lpvaluename.into_param().abi(), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExW<P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut REG_VALUE_TYPE>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryValueExW ( hkey : HKEY , lpvaluename : :: windows::core::PCWSTR , lpreserved : *const u32 , lptype : *mut REG_VALUE_TYPE , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryValueExW(hkey.into_param().abi(), lpvaluename.into_param().abi(), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueW<P0, P1>(hkey: P0, lpsubkey: P1, lpdata: ::windows::core::PWSTR, lpcbdata: ::core::option::Option<*mut i32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegQueryValueW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , lpdata : :: windows::core::PWSTR , lpcbdata : *mut i32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegQueryValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRenameKey<P0, P1, P2>(hkey: P0, lpsubkeyname: P1, lpnewkeyname: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegRenameKey ( hkey : HKEY , lpsubkeyname : :: windows::core::PCWSTR , lpnewkeyname : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegRenameKey(hkey.into_param().abi(), lpsubkeyname.into_param().abi(), lpnewkeyname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyA<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegReplaceKeyA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , lpnewfile : :: windows::core::PCSTR , lpoldfile : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegReplaceKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpnewfile.into_param().abi(), lpoldfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyW<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegReplaceKeyW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , lpnewfile : :: windows::core::PCWSTR , lpoldfile : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegReplaceKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpnewfile.into_param().abi(), lpoldfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyA<P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegRestoreKeyA ( hkey : HKEY , lpfile : :: windows::core::PCSTR , dwflags : REG_RESTORE_KEY_FLAGS ) -> super::super::Foundation:: WIN32_ERROR );
    RegRestoreKeyA(hkey.into_param().abi(), lpfile.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyW<P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegRestoreKeyW ( hkey : HKEY , lpfile : :: windows::core::PCWSTR , dwflags : REG_RESTORE_KEY_FLAGS ) -> super::super::Foundation:: WIN32_ERROR );
    RegRestoreKeyW(hkey.into_param().abi(), lpfile.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyA<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSaveKeyA ( hkey : HKEY , lpfile : :: windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: WIN32_ERROR );
    RegSaveKeyA(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExA<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSaveKeyExA ( hkey : HKEY , lpfile : :: windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , flags : REG_SAVE_FORMAT ) -> super::super::Foundation:: WIN32_ERROR );
    RegSaveKeyExA(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExW<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSaveKeyExW ( hkey : HKEY , lpfile : :: windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , flags : REG_SAVE_FORMAT ) -> super::super::Foundation:: WIN32_ERROR );
    RegSaveKeyExW(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyW<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSaveKeyW ( hkey : HKEY , lpfile : :: windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: WIN32_ERROR );
    RegSaveKeyW(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSetKeySecurity<P0, P1>(hkey: P0, securityinformation: u32, psecuritydescriptor: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetKeySecurity ( hkey : HKEY , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetKeySecurity(hkey.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cbdata: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetKeyValueA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , lpvaluename : :: windows::core::PCSTR , dwtype : u32 , lpdata : *const ::core::ffi::c_void , cbdata : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetKeyValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cbdata)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cbdata: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetKeyValueW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , lpvaluename : :: windows::core::PCWSTR , dwtype : u32 , lpdata : *const ::core::ffi::c_void , cbdata : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetKeyValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cbdata)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueA<P0, P1>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetValueA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR , dwtype : REG_VALUE_TYPE , lpdata : :: windows::core::PCSTR , cbdata : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExA<P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetValueExA ( hkey : HKEY , lpvaluename : :: windows::core::PCSTR , reserved : u32 , dwtype : REG_VALUE_TYPE , lpdata : *const u8 , cbdata : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetValueExA(hkey.into_param().abi(), lpvaluename.into_param().abi(), reserved, dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExW<P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetValueExW ( hkey : HKEY , lpvaluename : :: windows::core::PCWSTR , reserved : u32 , dwtype : REG_VALUE_TYPE , lpdata : *const u8 , cbdata : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetValueExW(hkey.into_param().abi(), lpvaluename.into_param().abi(), reserved, dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: P2, cbdata: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegSetValueW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR , dwtype : REG_VALUE_TYPE , lpdata : :: windows::core::PCWSTR , cbdata : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    RegSetValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), dwtype, lpdata.into_param().abi(), cbdata)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyA<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegUnLoadKeyA ( hkey : HKEY , lpsubkey : :: windows::core::PCSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegUnLoadKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyW<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<HKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn RegUnLoadKeyW ( hkey : HKEY , lpsubkey : :: windows::core::PCWSTR ) -> super::super::Foundation:: WIN32_ERROR );
    RegUnLoadKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_NO_1X_RATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_NO_2X_RATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_NO_4X_RATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_NO_8X_RATE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_NO_FW_ENABLE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_NO_SBA_ENABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_REVERSE_INITIALIZATION: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_SPECIAL_RESERVE: i32 = 1015808i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const AGP_FLAG_SPECIAL_TARGET: i32 = 1048575i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const APMMENUSUSPEND_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const APMMENUSUSPEND_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const APMMENUSUSPEND_NOCHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const APMMENUSUSPEND_UNDOCKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const APMTIMEOUT_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const BIF_RAWDEVICENEEDSDRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const BIF_SHOWSIMILARDRIVERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_BOOT_DEVICE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_CANTSTOPACHILD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_FAILEDINSTALL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_FINISHINSTALL_ACTION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_FINISHINSTALL_UI: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_FINISH_INSTALL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_IGNORE_BOOT_LC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_MANUAL_INSTALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_NEEDS_CLASS_CONFIG: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_NEEDS_FORCED_CONFIG: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_NETBOOT_CARD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_NET_BOOT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_NOREMOVEEXIT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_OKREMOVEROM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_PARTIAL_LOG_CONF: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_REINSTALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_REMOVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_SUPPRESS_SURPRISE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CONFIGFLAG_VERIFY_HARDWARE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CSCONFIGFLAG_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CSCONFIGFLAG_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CSCONFIGFLAG_DO_NOT_CREATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const CSCONFIGFLAG_DO_NOT_START: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DMSTATEFLAG_APPLYTOALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_ALWAYSUSE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_DEFAULT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_INDOSSTART: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_MULTIPLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_NEEDSETUP: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_PROVIDESUMB: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_SUPPORTED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTF_USESPMODE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DOSOPTGF_DEFCLEAN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DRIVERSIGN_BLOCKING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DRIVERSIGN_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DRIVERSIGN_WARNING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DTRESULTFIX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DTRESULTOK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DTRESULTPART: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const DTRESULTPROB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const EISAFLAG_NO_IO_MERGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const EISAFLAG_SLOT_IO_FIRST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const EISA_NO_MAX_FUNCTION: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_CLASSES_ROOT: HKEY = HKEY(-2147483648i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_CURRENT_CONFIG: HKEY = HKEY(-2147483643i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_CURRENT_USER: HKEY = HKEY(-2147483647i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = HKEY(-2147483641i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_DYN_DATA: HKEY = HKEY(-2147483642i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_LOCAL_MACHINE: HKEY = HKEY(-2147483646i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_PERFORMANCE_DATA: HKEY = HKEY(-2147483644i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = HKEY(-2147483552i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_PERFORMANCE_TEXT: HKEY = HKEY(-2147483568i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const HKEY_USERS: HKEY = HKEY(-2147483645i32 as _);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const IT_COMPACT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const IT_CUSTOM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const IT_PORTABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const IT_TYPICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const LASTGOOD_OPERATION: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const LASTGOOD_OPERATION_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const LASTGOOD_OPERATION_NOPOSTPROC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const MF_FLAGS_CREATE_BUT_NO_SHOW_DISABLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const MF_FLAGS_EVEN_IF_NO_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const MF_FLAGS_FILL_IN_UNKNOWN_RESOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const MF_FLAGS_NO_CREATE_IF_NO_RESOURCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const NUM_EISA_RANGES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const NUM_RESOURCE_MAP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCIC_DEFAULT_IRQMASK: u32 = 20152u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCIC_DEFAULT_NUMSOCKETS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCI_OPTIONS_USE_BIOS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCI_OPTIONS_USE_IRQ_STEERING: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_DEF_MEMBEGIN: u32 = 786432u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_DEF_MEMEND: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_DEF_MEMLEN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_DEF_MIN_REGION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_OPT_AUTOMEM: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_OPT_HAVE_SOCKET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_OPT_NO_APMREMOVE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_OPT_NO_AUDIO: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PCMCIA_OPT_NO_SOUND: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_OPTION_DEFAULT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_OPTION_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_OPTION_MSSPEC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_OPTION_REALMODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_OPTION_REGISTRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_COMPATIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_INVALID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_MAX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_NOKEY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_NONE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_OVERRIDE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_MINIPORT_SUCCESS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_BAD: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_MAX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_MSSPEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_NONE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_REALMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_REGISTRY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PIR_STATUS_TABLE_SUCCESS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_CONFLICTDMA: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_CONFLICTIO: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_CONFLICTIRQ: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_CONFLICTMEM: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_GENFORCEDCONFIG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_MAPIRQ2TO9: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NEEDFULLCONFIG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NODETCONFIG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NOTDETDMA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NOTDETIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NOTDETIRQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NOTDETMEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGDF_NOTVERIFIED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DATA_NETOS_IPX: ::windows::core::PCWSTR = ::windows::w!("IPX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DATA_NETOS_NDIS: ::windows::core::PCWSTR = ::windows::w!("NDIS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DATA_NETOS_ODI: ::windows::core::PCWSTR = ::windows::w!("ODI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DEFAULT_INSTANCE: ::windows::core::PCWSTR = ::windows::w!("0000");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ACPIENUM: ::windows::core::PCWSTR = ::windows::w!("ACPI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_APM: ::windows::core::PCWSTR = ::windows::w!("*PNP0C05");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_BIOSENUM: ::windows::core::PCWSTR = ::windows::w!("BIOS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CLASS: ::windows::core::PCWSTR = ::windows::w!("Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CONFIG: ::windows::core::PCWSTR = ::windows::w!("Config");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CONTROL: ::windows::core::PCWSTR = ::windows::w!("Control");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CRASHES: ::windows::core::PCWSTR = ::windows::w!("Crashes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CURRENT: ::windows::core::PCWSTR = ::windows::w!("Current");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CURRENT_ENV: ::windows::core::PCWSTR = ::windows::w!("\\Windows 4.0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DANGERS: ::windows::core::PCWSTR = ::windows::w!("Dangers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("Default");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DETMODVARS: ::windows::core::PCWSTR = ::windows::w!("DetModVars");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DEVICEPARAMETERS: ::windows::core::PCWSTR = ::windows::w!("Device Parameters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DEVICE_PROPERTIES: ::windows::core::PCWSTR = ::windows::w!("Properties");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DISPLAY_CLASS: ::windows::core::PCWSTR = ::windows::w!("Display");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DOSOPTCDROM: ::windows::core::PCWSTR = ::windows::w!("CD-ROM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DOSOPTMOUSE: ::windows::core::PCWSTR = ::windows::w!("MOUSE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DRIVERPARAMETERS: ::windows::core::PCWSTR = ::windows::w!("Driver Parameters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DRIVERS: ::windows::core::PCWSTR = ::windows::w!("\\Drivers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDAUTOEXECBATKEYBOARD: ::windows::core::PCWSTR = ::windows::w!("EBDAutoexecBatKeyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDAUTOEXECBATLOCAL: ::windows::core::PCWSTR = ::windows::w!("EBDAutoexecBatLocale");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDCONFIGSYSKEYBOARD: ::windows::core::PCWSTR = ::windows::w!("EBDConfigSysKeyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDCONFIGSYSLOCAL: ::windows::core::PCWSTR = ::windows::w!("EBDConfigSysLocale");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDFILESKEYBOARD: ::windows::core::PCWSTR = ::windows::w!("EBDFilesKeyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDFILESLOCAL: ::windows::core::PCWSTR = ::windows::w!("EBDFilesLocale");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EISAENUM: ::windows::core::PCWSTR = ::windows::w!("EISA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ENUM: ::windows::core::PCWSTR = ::windows::w!("Enum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EXPLORER: ::windows::core::PCWSTR = ::windows::w!("Explorer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_FILTERS: ::windows::core::PCWSTR = ::windows::w!("Filters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_INIUPDATE: ::windows::core::PCWSTR = ::windows::w!("IniUpdate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ISAENUM: ::windows::core::PCWSTR = ::windows::w!("ISAPnP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_JOYCURR: ::windows::core::PCWSTR = ::windows::w!("CurrentJoystickSettings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_JOYSETTINGS: ::windows::core::PCWSTR = ::windows::w!("JoystickSettings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_KEYBOARD_CLASS: ::windows::core::PCWSTR = ::windows::w!("Keyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_KNOWNDOCKINGSTATES: ::windows::core::PCWSTR = ::windows::w!("Hardware Profiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_LOGCONFIG: ::windows::core::PCWSTR = ::windows::w!("LogConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_LOGON: ::windows::core::PCWSTR = ::windows::w!("\\Logon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_LOWER_FILTER_LEVEL_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("*Lower");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MEDIA_CLASS: ::windows::core::PCWSTR = ::windows::w!("MEDIA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MODEM_CLASS: ::windows::core::PCWSTR = ::windows::w!("Modem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MODES: ::windows::core::PCWSTR = ::windows::w!("Modes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MONITOR_CLASS: ::windows::core::PCWSTR = ::windows::w!("Monitor");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MOUSE_CLASS: ::windows::core::PCWSTR = ::windows::w!("Mouse");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NDISINFO: ::windows::core::PCWSTR = ::windows::w!("NDISInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORK: ::windows::core::PCWSTR = ::windows::w!("Network");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORKPROVIDER: ::windows::core::PCWSTR = ::windows::w!("\\NetworkProvider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORK_PERSISTENT: ::windows::core::PCWSTR = ::windows::w!("\\Persistent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORK_RECENT: ::windows::core::PCWSTR = ::windows::w!("\\Recent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_OVERRIDE: ::windows::core::PCWSTR = ::windows::w!("Override");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCIENUM: ::windows::core::PCWSTR = ::windows::w!("PCI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMCIA: ::windows::core::PCWSTR = ::windows::w!("PCMCIA\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMCIAENUM: ::windows::core::PCWSTR = ::windows::w!("PCMCIA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMCIA_CLASS: ::windows::core::PCWSTR = ::windows::w!("PCMCIA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMTD: ::windows::core::PCWSTR = ::windows::w!("MTD-");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCUNKNOWN: ::windows::core::PCWSTR = ::windows::w!("UNKNOWN_MANUFACTURER");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_COMPUTERS: ::windows::core::PCWSTR = ::windows::w!("Computers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_DEFAULT: ::windows::core::PCWSTR = ::windows::w!(".default");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_USERGROUPDATA: ::windows::core::PCWSTR = ::windows::w!("GroupData\\UserGroups\\Priority");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_USERGROUPS: ::windows::core::PCWSTR = ::windows::w!("UserGroups");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_USERS: ::windows::core::PCWSTR = ::windows::w!("Users");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PORTS_CLASS: ::windows::core::PCWSTR = ::windows::w!("ports");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PRINTERS: ::windows::core::PCWSTR = ::windows::w!("Printers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PRINT_PROC: ::windows::core::PCWSTR = ::windows::w!("\\Print Processors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ROOTENUM: ::windows::core::PCWSTR = ::windows::w!("Root");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_RUNHISTORY: ::windows::core::PCWSTR = ::windows::w!("RunHistory");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SCSI_CLASS: ::windows::core::PCWSTR = ::windows::w!("SCSIAdapter");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SETUP: ::windows::core::PCWSTR = ::windows::w!("\\Setup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SHARES: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\LanMan");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SYSTEM: ::windows::core::PCWSTR = ::windows::w!("System");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SYSTEMBOARD: ::windows::core::PCWSTR = ::windows::w!("*PNP0C01");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_UPPER_FILTER_LEVEL_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("*Upper");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_USER: ::windows::core::PCWSTR = ::windows::w!("User");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_VPOWERDENUM: ::windows::core::PCWSTR = ::windows::w!("VPOWERD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_WINOLDAPP: ::windows::core::PCWSTR = ::windows::w!("WinOldApp");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_ATT_PC: ::windows::core::PCWSTR = ::windows::w!("AT&T PC");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_HP_VECTRA: ::windows::core::PCWSTR = ::windows::w!("HP Vectra");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPC: ::windows::core::PCWSTR = ::windows::w!("IBM PC");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCAT: ::windows::core::PCWSTR = ::windows::w!("IBM PC/AT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCCONV: ::windows::core::PCWSTR = ::windows::w!("IBM PC Convertible");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCJR: ::windows::core::PCWSTR = ::windows::w!("IBM PCjr");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCXT: ::windows::core::PCWSTR = ::windows::w!("IBM PC/XT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCXT_286: ::windows::core::PCWSTR = ::windows::w!("IBM PC/XT 286");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS1: ::windows::core::PCWSTR = ::windows::w!("IBM PS/1");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_25: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-25");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_30: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-30");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_30_286: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-30 286");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_50: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-50");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_50Z: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-50Z");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_55SX: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-55SX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_60: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-60");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_65SX: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-65SX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_70: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-70");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_70_80: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-70/80");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_80: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-80");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_90: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-90");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_P70: ::windows::core::PCWSTR = ::windows::w!("IBM PS/2-P70");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_PHOENIX_PCAT: ::windows::core::PCWSTR = ::windows::w!("Phoenix PC/AT Compatible");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_UNKNOWN: ::windows::core::PCWSTR = ::windows::w!("Unknown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_ZENITH_PC: ::windows::core::PCWSTR = ::windows::w!("Zenith PC");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ADDRARB: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\Arbitrators\\AddrArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_AEDEBUG: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\AeDebug");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_APPEARANCE: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Appearance");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_APPPATCH: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\AppPatches");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_APPPATHS: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\App Paths");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_BIOSINFO: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\BiosInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_BUSINFORMATION: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\PnP\\BusInformation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CDFS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\FileSystem\\CDFS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKBADAPPS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKBADAPPS400: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps400");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKDISK: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKDISKSET: ::windows::core::PCWSTR = ::windows::w!("Settings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKDISKUDRVS: ::windows::core::PCWSTR = ::windows::w!("NoUnknownDDErrDrvs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKVERDLLS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckVerDLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHILD_PREFIX: ::windows::core::PCWSTR = ::windows::w!("Child");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHKLASTCHECK: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastCheck");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHKLASTSURFAN: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastSurfaceAnalysis");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CLASS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CLASS_NT: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CODEPAGE: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Nls\\Codepage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CODEVICEINSTALLERS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\CoDeviceInstallers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_COLORS: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Colors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_COMPUTRNAME: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\ComputerName\\ComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CONTROLPANEL: ::windows::core::PCWSTR = ::windows::w!("Control Panel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CONTROLSFOLDER: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Controls Folder");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CRITICALDEVICEDATABASE: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\CriticalDeviceDatabase");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CURRENTCONTROLSET: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CURRENT_CONTROL_SET: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CURSORS: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Cursors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CVNETWORK: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DESKTOP: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Desktop");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DETECT: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Detect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DEVICEINSTALLER: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Device Installer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DEVICE_CLASSES: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\DeviceClasses");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DIFX: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\DIFX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DISPLAYSETTINGS: ::windows::core::PCWSTR = ::windows::w!("Display\\Settings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DMAARB: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\Arbitrators\\DMAArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DRIVERSIGN: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DRIVERSIGN_POLICY: ::windows::core::PCWSTR = ::windows::w!("Software\\Policies\\Microsoft\\Windows NT\\Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ENUM: ::windows::core::PCWSTR = ::windows::w!("Enum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ENVIRONMENTS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Print\\Environments");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_EVENTLABELS: ::windows::core::PCWSTR = ::windows::w!("AppEvents\\EventLabels");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_EXPLORER: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FAULT: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Fault");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FILESYSTEM: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\FileSystem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FILESYSTEM_NOVOLTRACK: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\FileSystem\\NoVolTrack");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR: ::windows::core::PCWSTR = ::windows::w!("HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR0: ::windows::core::PCWSTR = ::windows::w!("HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor\\0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FONTS: ::windows::core::PCWSTR = ::windows::w!("Display\\Fonts");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_GRPCONV: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\GrpConv");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_HACKINIFILE: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\HackIniFiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_HWPROFILES: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Hardware Profiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_HWPROFILESCURRENT: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Hardware Profiles\\Current");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ICONS: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Icons");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IDCONFIGDB: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\IDConfigDB");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_INSTALLEDFILES: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\InstalledFiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IOARB: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\Arbitrators\\IOArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IOS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD\\IOS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IRQARB: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\Arbitrators\\IRQArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KEYBOARD: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Keyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KNOWN16DLLS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\Known16DLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KNOWNDLLS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\KnownDLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KNOWNVXDS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\KnownVxDs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTBACKUP: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastBackup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTCHECK: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastCheck");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTGOOD: ::windows::core::PCWSTR = ::windows::w!("System\\LastKnownGoodRecovery\\LastGood");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTGOODTMP: ::windows::core::PCWSTR = ::windows::w!("System\\LastKnownGoodRecovery\\LastGood.Tmp");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTOPTIMIZE: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastOptimize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LOOKSCHEMES: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Appearance\\Schemes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_METRICS: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Desktop\\WindowMetrics");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MONITORS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Print\\Monitors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MOUSE: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Mouse");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MSDOSOPTS: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSOptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MULTIMEDIA_AUDIO: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Multimedia\\Audio");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MULTI_FUNCTION: ::windows::core::PCWSTR = ::windows::w!("MF");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NCPSERVER: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\NcpServer\\Parameters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NETEQUIV: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Equivalent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NETWORK_USERSETTINGS: ::windows::core::PCWSTR = ::windows::w!("Network");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NEWDOSBOX: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSSpecialConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NONDRIVERSIGN: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Non-Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NONDRIVERSIGN_POLICY: ::windows::core::PCWSTR = ::windows::w!("Software\\Policies\\Microsoft\\Windows NT\\Non-Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NOSUGGMSDOS: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\NoMSDOSWarn");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NT_CURRENTVERSION: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows NT\\CurrentVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NWREDIR: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD\\NWREDIR");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PCIIR: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Pnp\\PciIrqRouting");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PER_HW_ID_STORAGE: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\PerHwIdStorage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PIFCONVERT: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PIFConvert");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_POLICIES: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Policies");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PRINT: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Print");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PRINTERS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Print\\Printers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PROPERTYSYSTEM: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PropertySystem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PROVIDERS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Print\\Providers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PWDPROVIDER: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\PwdProvider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_REALMODENET: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Real Mode Net");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_REINSTALL: ::windows::core::PCWSTR = ::windows::w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Reinstall");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Reliability");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY: ::windows::core::PCWSTR = ::windows::w!("Software\\Policies\\Microsoft\\Windows NT\\Reliability");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY_REPORTSNAPSHOT: ::windows::core::PCWSTR = ::windows::w!("ReportSnapshot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY_SHUTDOWNREASONUI: ::windows::core::PCWSTR = ::windows::w!("ShutdownReasonUI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY_SNAPSHOT: ::windows::core::PCWSTR = ::windows::w!("Snapshot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ROOT: ::windows::core::PCWSTR = ::windows::w!("Enum\\Root");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUN: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNONCE: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNONCEEX: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNSERVICES: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunServices");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNSERVICESONCE: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunServicesOnce");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SCHEMES: ::windows::core::PCWSTR = ::windows::w!("AppEvents\\Schemes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SCREENSAVE: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Desktop");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SERVICES: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SETUP: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SHUTDOWN: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Shutdown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SOUND: ::windows::core::PCWSTR = ::windows::w!("Control Panel\\Sound");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SYSTEMENUM: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Enum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SYSTRAY: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\SysTray");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_TIMEZONE: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\TimeZoneInformation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_UNINSTALL: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_UPDATE: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Update");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VCOMM: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD\\VCOMM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VMM: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD\\VMM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VMM32FILES: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\VMM32Files");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VNETSUP: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD\\VNETSUP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VOLUMECACHE: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VolumeCaches");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VPOWERD: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD\\VPOWERD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VXD: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Services\\VxD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WARNVERDLLS: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\SessionManager\\WarnVerDLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WINBOOT: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\WinBoot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WINDOWSAPPLETS: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WINLOGON: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Winlogon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WMI_SECURITY: ::windows::core::PCWSTR = ::windows::w!("System\\CurrentControlSet\\Control\\Wmi\\Security");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PCI_DUAL_IDE: ::windows::core::PCWSTR = ::windows::w!("PCIDualIDE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PCI_OPTIONS: ::windows::core::PCWSTR = ::windows::w!("Options");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_DEFAULTLOC: ::windows::core::PCWSTR = ::windows::w!("UseDefaultNetLocation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_ENABLE: ::windows::core::PCWSTR = ::windows::w!("Enable");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_LOWPOWERACTIVE: ::windows::core::PCWSTR = ::windows::w!("ScreenSaveLowPowerActive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_LOWPOWERTIMEOUT: ::windows::core::PCWSTR = ::windows::w!("ScreenSaveLowPowerTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_NETPATH: ::windows::core::PCWSTR = ::windows::w!("NetworkPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_POWEROFFACTIVE: ::windows::core::PCWSTR = ::windows::w!("ScreenSavePowerOffActive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_POWEROFFTIMEOUT: ::windows::core::PCWSTR = ::windows::w!("ScreenSavePowerOffTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_SCRPASSWORD: ::windows::core::PCWSTR = ::windows::w!("ScreenSave_Data");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_USESCRPASSWORD: ::windows::core::PCWSTR = ::windows::w!("ScreenSaveUsePassword");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_VERBOSE: ::windows::core::PCWSTR = ::windows::w!("Verbose");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ACDRIVESPINDOWN: ::windows::core::PCWSTR = ::windows::w!("ACDriveSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ACSPINDOWNPREVIOUS: ::windows::core::PCWSTR = ::windows::w!("ACSpinDownPrevious");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ACTIVESERVICE: ::windows::core::PCWSTR = ::windows::w!("ActiveService");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("Address");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AEDEBUG_AUTO: ::windows::core::PCWSTR = ::windows::w!("Auto");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AEDEBUG_DEBUGGER: ::windows::core::PCWSTR = ::windows::w!("Debugger");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ALPHANUMPWDS: ::windows::core::PCWSTR = ::windows::w!("AlphanumPwds");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APISUPPORT: ::windows::core::PCWSTR = ::windows::w!("APISupport");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMACTIMEOUT: ::windows::core::PCWSTR = ::windows::w!("APMACTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMBATTIMEOUT: ::windows::core::PCWSTR = ::windows::w!("APMBatTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMBIOSVER: ::windows::core::PCWSTR = ::windows::w!("APMBiosVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMFLAGS: ::windows::core::PCWSTR = ::windows::w!("APMFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMMENUSUSPEND: ::windows::core::PCWSTR = ::windows::w!("APMMenuSuspend");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMSHUTDOWNPOWER: ::windows::core::PCWSTR = ::windows::w!("APMShutDownPower");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APPINSTPATH: ::windows::core::PCWSTR = ::windows::w!("AppInstallPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ASKFORCONFIG: ::windows::core::PCWSTR = ::windows::w!("AskForConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ASKFORCONFIGFUNC: ::windows::core::PCWSTR = ::windows::w!("AskForConfigFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ASYNCFILECOMMIT: ::windows::core::PCWSTR = ::windows::w!("AsyncFileCommit");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUDIO_BITMAP: ::windows::core::PCWSTR = ::windows::w!("bitmap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUDIO_ICON: ::windows::core::PCWSTR = ::windows::w!("icon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTHENT_AGENT: ::windows::core::PCWSTR = ::windows::w!("AuthenticatingAgent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOEXEC: ::windows::core::PCWSTR = ::windows::w!("Autoexec.Bat");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOINSNOTE: ::windows::core::PCWSTR = ::windows::w!("AutoInsertNotification");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOLOGON: ::windows::core::PCWSTR = ::windows::w!("AutoLogon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOMOUNT: ::windows::core::PCWSTR = ::windows::w!("AutoMountDrives");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOSTART: ::windows::core::PCWSTR = ::windows::w!("AutoStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BASICPROPERTIES: ::windows::core::PCWSTR = ::windows::w!("BasicProperties");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BASICPROPERTIES_32: ::windows::core::PCWSTR = ::windows::w!("BasicProperties32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BATDRIVESPINDOWN: ::windows::core::PCWSTR = ::windows::w!("BatDriveSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BATSPINDOWNPREVIOUS: ::windows::core::PCWSTR = ::windows::w!("BatSpinDownPrevious");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BEHAVIOR_ON_FAILED_VERIFY: ::windows::core::PCWSTR = ::windows::w!("BehaviorOnFailedVerify");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BIOSDATE: ::windows::core::PCWSTR = ::windows::w!("BIOSDate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BIOSNAME: ::windows::core::PCWSTR = ::windows::w!("BIOSName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BIOSVERSION: ::windows::core::PCWSTR = ::windows::w!("BIOSVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BITSPERPIXEL: ::windows::core::PCWSTR = ::windows::w!("BitsPerPixel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BOOTCONFIG: ::windows::core::PCWSTR = ::windows::w!("BootConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BOOTCOUNT: ::windows::core::PCWSTR = ::windows::w!("BootCount");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BOOTDIR: ::windows::core::PCWSTR = ::windows::w!("BootDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BPP: ::windows::core::PCWSTR = ::windows::w!("BPP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BT: ::windows::core::PCWSTR = ::windows::w!("6005BT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BUFFAGETIMEOUT: ::windows::core::PCWSTR = ::windows::w!("BufferAgeTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BUFFIDLETIMEOUT: ::windows::core::PCWSTR = ::windows::w!("BufferIdleTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BUSTYPE: ::windows::core::PCWSTR = ::windows::w!("BusType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CAPABILITIES: ::windows::core::PCWSTR = ::windows::w!("Capabilities");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CARDSPECIFIC: ::windows::core::PCWSTR = ::windows::w!("CardSpecific");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDCACHESIZE: ::windows::core::PCWSTR = ::windows::w!("CacheSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDCOMPATNAMES: ::windows::core::PCWSTR = ::windows::w!("MSCDEXCompatNames");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDEXTERRORS: ::windows::core::PCWSTR = ::windows::w!("ExtendedErrors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDNOREADAHEAD: ::windows::core::PCWSTR = ::windows::w!("NoReadAhead");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDPREFETCH: ::windows::core::PCWSTR = ::windows::w!("Prefetch");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDPREFETCHTAIL: ::windows::core::PCWSTR = ::windows::w!("PrefetchTail");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDRAWCACHE: ::windows::core::PCWSTR = ::windows::w!("RawCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDROM: ::windows::core::PCWSTR = ::windows::w!("GenCD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDROMCLASSNAME: ::windows::core::PCWSTR = ::windows::w!("CDROM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDSHOWVERSIONS: ::windows::core::PCWSTR = ::windows::w!("ShowVersions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDSVDSENSE: ::windows::core::PCWSTR = ::windows::w!("SVDSense");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CHECKSUM: ::windows::core::PCWSTR = ::windows::w!("CurrentChecksum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CLASS: ::windows::core::PCWSTR = ::windows::w!("Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CLASSDESC: ::windows::core::PCWSTR = ::windows::w!("ClassDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CLASSGUID: ::windows::core::PCWSTR = ::windows::w!("ClassGUID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CMDRIVFLAGS: ::windows::core::PCWSTR = ::windows::w!("CMDrivFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CMENUMFLAGS: ::windows::core::PCWSTR = ::windows::w!("CMEnumFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COINSTALLERS_32: ::windows::core::PCWSTR = ::windows::w!("CoInstallers32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMINFO: ::windows::core::PCWSTR = ::windows::w!("ComInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMMENT: ::windows::core::PCWSTR = ::windows::w!("Comment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPATIBLEIDS: ::windows::core::PCWSTR = ::windows::w!("CompatibleIDs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPRESSIONMETHOD: ::windows::core::PCWSTR = ::windows::w!("CompressionAlgorithm");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPRESSIONTHRESHOLD: ::windows::core::PCWSTR = ::windows::w!("CompressionThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPUTERNAME: ::windows::core::PCWSTR = ::windows::w!("ComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPUTRNAME: ::windows::core::PCWSTR = ::windows::w!("ComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMVERIFYBASE: ::windows::core::PCWSTR = ::windows::w!("COMVerifyBase");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIG: ::windows::core::PCWSTR = ::windows::w!("ConfigPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIGFLAGS: ::windows::core::PCWSTR = ::windows::w!("ConfigFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIGMG: ::windows::core::PCWSTR = ::windows::w!("CONFIGMG");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIGSYS: ::windows::core::PCWSTR = ::windows::w!("Config.Sys");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONNECTION_TYPE: ::windows::core::PCWSTR = ::windows::w!("ConnectionType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONTAINERID: ::windows::core::PCWSTR = ::windows::w!("ContainerID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONTIGFILEALLOC: ::windows::core::PCWSTR = ::windows::w!("ContigFileAllocSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONVMEM: ::windows::core::PCWSTR = ::windows::w!("ConvMem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CPU: ::windows::core::PCWSTR = ::windows::w!("CPU");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CRASHFUNCS: ::windows::core::PCWSTR = ::windows::w!("CrashFuncs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CSCONFIGFLAGS: ::windows::core::PCWSTR = ::windows::w!("CSConfigFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURCONFIG: ::windows::core::PCWSTR = ::windows::w!("CurrentConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURDRVLET: ::windows::core::PCWSTR = ::windows::w!("CurrentDriveLetterAssignment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENTCONFIG: ::windows::core::PCWSTR = ::windows::w!("CurrentConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_BUILD: ::windows::core::PCWSTR = ::windows::w!("CurrentBuildNumber");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_CSDVERSION: ::windows::core::PCWSTR = ::windows::w!("CSDVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_TYPE: ::windows::core::PCWSTR = ::windows::w!("CurrentType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_USER: ::windows::core::PCWSTR = ::windows::w!("Current User");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_VERSION: ::windows::core::PCWSTR = ::windows::w!("CurrentVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CUSTOMCOLORS: ::windows::core::PCWSTR = ::windows::w!("CustomColors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CUSTOM_PROPERTY_CACHE_DATE: ::windows::core::PCWSTR = ::windows::w!("CustomPropertyCacheDate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CUSTOM_PROPERTY_HW_ID_KEY: ::windows::core::PCWSTR = ::windows::w!("CustomPropertyHwIdKey");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("Default");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETCONFIG: ::windows::core::PCWSTR = ::windows::w!("DetConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETECT: ::windows::core::PCWSTR = ::windows::w!("Detect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETECTFUNC: ::windows::core::PCWSTR = ::windows::w!("DetectFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETFLAGS: ::windows::core::PCWSTR = ::windows::w!("DetFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETFUNC: ::windows::core::PCWSTR = ::windows::w!("DetFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVDESC: ::windows::core::PCWSTR = ::windows::w!("DeviceDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICEDRIVER: ::windows::core::PCWSTR = ::windows::w!("DeviceDriver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICEPATH: ::windows::core::PCWSTR = ::windows::w!("DevicePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_CHARACTERISTICS: ::windows::core::PCWSTR = ::windows::w!("DeviceCharacteristics");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_EXCLUSIVE: ::windows::core::PCWSTR = ::windows::w!("Exclusive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_INSTANCE: ::windows::core::PCWSTR = ::windows::w!("DeviceInstance");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_SECURITY_DESCRIPTOR: ::windows::core::PCWSTR = ::windows::w!("Security");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_TYPE: ::windows::core::PCWSTR = ::windows::w!("DeviceType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVLOADER: ::windows::core::PCWSTR = ::windows::w!("DevLoader");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVTYPE: ::windows::core::PCWSTR = ::windows::w!("DeviceType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DIRECTHOST: ::windows::core::PCWSTR = ::windows::w!("DirectHost");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DIRTYSHUTDOWN: ::windows::core::PCWSTR = ::windows::w!("DirtyShutdown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DIRTYSHUTDOWNTIME: ::windows::core::PCWSTR = ::windows::w!("DirtyShutdownTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISABLECOUNT: ::windows::core::PCWSTR = ::windows::w!("DisableCount");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISABLEPWDCACHING: ::windows::core::PCWSTR = ::windows::w!("DisablePwdCaching");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISABLEREGTOOLS: ::windows::core::PCWSTR = ::windows::w!("DisableRegistryTools");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISCONNECT: ::windows::core::PCWSTR = ::windows::w!("Disconnect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISK: ::windows::core::PCWSTR = ::windows::w!("GenDisk");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISKCLASSNAME: ::windows::core::PCWSTR = ::windows::w!("DiskDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOAPPEARANCEPAGE: ::windows::core::PCWSTR = ::windows::w!("NoDispAppearancePage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOBACKGROUNDPAGE: ::windows::core::PCWSTR = ::windows::w!("NoDispBackgroundPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NODISPCPL: ::windows::core::PCWSTR = ::windows::w!("NoDispCPL");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOSCRSAVPAGE: ::windows::core::PCWSTR = ::windows::w!("NoDispScrSavPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOSETTINGSPAGE: ::windows::core::PCWSTR = ::windows::w!("NoDispSettingsPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPLAY: ::windows::core::PCWSTR = ::windows::w!("display");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPLAYFLAGS: ::windows::core::PCWSTR = ::windows::w!("DisplayFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOCKED: ::windows::core::PCWSTR = ::windows::w!("CurrentDockedState");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOCKSTATE: ::windows::core::PCWSTR = ::windows::w!("DockState");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOES_POLLING: ::windows::core::PCWSTR = ::windows::w!("PollingSupportNeeded");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DONTLOADIFCONFLICT: ::windows::core::PCWSTR = ::windows::w!("DontLoadIfConflict");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DONTUSEMEM: ::windows::core::PCWSTR = ::windows::w!("DontAllocLastMem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSCP: ::windows::core::PCWSTR = ::windows::w!("OEMCP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSOPTFLAGS: ::windows::core::PCWSTR = ::windows::w!("Flags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSOPTGLOBALFLAGS: ::windows::core::PCWSTR = ::windows::w!("GlobalFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSOPTTIP: ::windows::core::PCWSTR = ::windows::w!("TipText");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSPAGER: ::windows::core::PCWSTR = ::windows::w!("DOSPager");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOS_SPOOL_MASK: ::windows::core::PCWSTR = ::windows::w!("DOSSpoolMask");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOUBLEBUFFER: ::windows::core::PCWSTR = ::windows::w!("DoubleBuffer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPI: ::windows::core::PCWSTR = ::windows::w!("dpi");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPILOGICALX: ::windows::core::PCWSTR = ::windows::w!("DPILogicalX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPILOGICALY: ::windows::core::PCWSTR = ::windows::w!("DPILogicalY");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPIPHYSICALX: ::windows::core::PCWSTR = ::windows::w!("DPIPhysicalX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPIPHYSICALY: ::windows::core::PCWSTR = ::windows::w!("DPIPhysicalY");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPMS: ::windows::core::PCWSTR = ::windows::w!("DPMS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVER: ::windows::core::PCWSTR = ::windows::w!("Driver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERCACHEPATH: ::windows::core::PCWSTR = ::windows::w!("DriverCachePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERDATE: ::windows::core::PCWSTR = ::windows::w!("DriverDate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERDATEDATA: ::windows::core::PCWSTR = ::windows::w!("DriverDateData");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERVERSION: ::windows::core::PCWSTR = ::windows::w!("DriverVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVESPINDOWN: ::windows::core::PCWSTR = ::windows::w!("DriveSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVEWRITEBEHIND: ::windows::core::PCWSTR = ::windows::w!("DriveWriteBehind");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVE_SPINDOWN: ::windows::core::PCWSTR = ::windows::w!("NoDispSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRV: ::windows::core::PCWSTR = ::windows::w!("drv");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRVDESC: ::windows::core::PCWSTR = ::windows::w!("DriverDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DYNAMIC: ::windows::core::PCWSTR = ::windows::w!("Dynamic");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_FLAGS: ::windows::core::PCWSTR = ::windows::w!("EISAFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_FUNCTIONS: ::windows::core::PCWSTR = ::windows::w!("EISAFunctions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_FUNCTIONS_MASK: ::windows::core::PCWSTR = ::windows::w!("EISAFunctionsMask");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_RANGES: ::windows::core::PCWSTR = ::windows::w!("EISARanges");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_SIMULATE_INT15: ::windows::core::PCWSTR = ::windows::w!("EISASimulateInt15");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EJECT_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("EjectPriority");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENABLEINTS: ::windows::core::PCWSTR = ::windows::w!("EnableInts");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENUMERATOR: ::windows::core::PCWSTR = ::windows::w!("Enumerator");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENUMPROPPAGES: ::windows::core::PCWSTR = ::windows::w!("EnumPropPages");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENUMPROPPAGES_32: ::windows::core::PCWSTR = ::windows::w!("EnumPropPages32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ESDI: ::windows::core::PCWSTR = ::windows::w!("ESDI\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EXISTS: ::windows::core::PCWSTR = ::windows::w!("Exists");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EXTMEM: ::windows::core::PCWSTR = ::windows::w!("ExtMem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FAULT_LOGFILE: ::windows::core::PCWSTR = ::windows::w!("LogFile");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FIFODEPTH: ::windows::core::PCWSTR = ::windows::w!("FIFODepth");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FILESHARING: ::windows::core::PCWSTR = ::windows::w!("FileSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FIRSTINSTALLDATETIME: ::windows::core::PCWSTR = ::windows::w!("FirstInstallDateTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FIRSTNETDRIVE: ::windows::core::PCWSTR = ::windows::w!("FirstNetworkDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FLOP: ::windows::core::PCWSTR = ::windows::w!("FLOP\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FLOPPY: ::windows::core::PCWSTR = ::windows::w!("FLOPPY");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FONTSIZE: ::windows::core::PCWSTR = ::windows::w!("FontSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCECL: ::windows::core::PCWSTR = ::windows::w!("ForceChangeLine");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEDCONFIG: ::windows::core::PCWSTR = ::windows::w!("ForcedConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEFIFO: ::windows::core::PCWSTR = ::windows::w!("ForceFIFO");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCELOAD: ::windows::core::PCWSTR = ::windows::w!("ForceLoadPD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEPMIO: ::windows::core::PCWSTR = ::windows::w!("ForcePMIO");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEREBOOT: ::windows::core::PCWSTR = ::windows::w!("ForceReboot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCERMIO: ::windows::core::PCWSTR = ::windows::w!("ForceRMIO");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FREESPACERATIO: ::windows::core::PCWSTR = ::windows::w!("FreeSpaceRatio");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FRIENDLYNAME: ::windows::core::PCWSTR = ::windows::w!("FriendlyName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FSFILTERCLASS: ::windows::core::PCWSTR = ::windows::w!("FSFilterClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FULLTRACE: ::windows::core::PCWSTR = ::windows::w!("FullTrace");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FUNCDESC: ::windows::core::PCWSTR = ::windows::w!("FunctionDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_GAPTIME: ::windows::core::PCWSTR = ::windows::w!("GapTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_GRB: ::windows::core::PCWSTR = ::windows::w!("grb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HARDWAREID: ::windows::core::PCWSTR = ::windows::w!("HardwareID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HIDESHAREPWDS: ::windows::core::PCWSTR = ::windows::w!("HideSharePwds");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HRES: ::windows::core::PCWSTR = ::windows::w!("HRes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HWDETECT: ::windows::core::PCWSTR = ::windows::w!("HardwareDetect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HWMECHANISM: ::windows::core::PCWSTR = ::windows::w!("HWMechanism");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HWREV: ::windows::core::PCWSTR = ::windows::w!("HWRevision");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ID: ::windows::core::PCWSTR = ::windows::w!("CurrentID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_IDE_FORCE_SERIALIZE: ::windows::core::PCWSTR = ::windows::w!("ForceSerialization");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_IDE_NO_SERIALIZE: ::windows::core::PCWSTR = ::windows::w!("IDENoSerialize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFNAME: ::windows::core::PCWSTR = ::windows::w!("InfName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFPATH: ::windows::core::PCWSTR = ::windows::w!("InfPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFSECTION: ::windows::core::PCWSTR = ::windows::w!("InfSection");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFSECTIONEXT: ::windows::core::PCWSTR = ::windows::w!("InfSectionExt");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INHIBITRESULTS: ::windows::core::PCWSTR = ::windows::w!("InhibitResults");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSICON: ::windows::core::PCWSTR = ::windows::w!("Icon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSTALLER: ::windows::core::PCWSTR = ::windows::w!("Installer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSTALLER_32: ::windows::core::PCWSTR = ::windows::w!("Installer32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSTALLTYPE: ::windows::core::PCWSTR = ::windows::w!("InstallType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INT13: ::windows::core::PCWSTR = ::windows::w!("Int13");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ISAPNP: ::windows::core::PCWSTR = ::windows::w!("ISAPNP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ISAPNP_RDP_OVERRIDE: ::windows::core::PCWSTR = ::windows::w!("RDPOverRide");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYCALLOUT: ::windows::core::PCWSTR = ::windows::w!("JoystickCallout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYNCONFIG: ::windows::core::PCWSTR = ::windows::w!("Joystick%dConfiguration");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYNOEMCALLOUT: ::windows::core::PCWSTR = ::windows::w!("Joystick%dOEMCallout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYNOEMNAME: ::windows::core::PCWSTR = ::windows::w!("Joystick%dOEMName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL1: ::windows::core::PCWSTR = ::windows::w!("OEMCal1");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL10: ::windows::core::PCWSTR = ::windows::w!("OEMCal10");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL11: ::windows::core::PCWSTR = ::windows::w!("OEMCal11");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL12: ::windows::core::PCWSTR = ::windows::w!("OEMCal12");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL2: ::windows::core::PCWSTR = ::windows::w!("OEMCal2");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL3: ::windows::core::PCWSTR = ::windows::w!("OEMCal3");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL4: ::windows::core::PCWSTR = ::windows::w!("OEMCal4");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL5: ::windows::core::PCWSTR = ::windows::w!("OEMCal5");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL6: ::windows::core::PCWSTR = ::windows::w!("OEMCal6");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL7: ::windows::core::PCWSTR = ::windows::w!("OEMCal7");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL8: ::windows::core::PCWSTR = ::windows::w!("OEMCal8");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL9: ::windows::core::PCWSTR = ::windows::w!("OEMCal9");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCALCAP: ::windows::core::PCWSTR = ::windows::w!("OEMCalCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCALLOUT: ::windows::core::PCWSTR = ::windows::w!("OEMCallout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCALWINCAP: ::windows::core::PCWSTR = ::windows::w!("OEMCalWinCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMDATA: ::windows::core::PCWSTR = ::windows::w!("OEMData");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMNAME: ::windows::core::PCWSTR = ::windows::w!("OEMName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMPOVLABEL: ::windows::core::PCWSTR = ::windows::w!("OEMPOVLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMRLABEL: ::windows::core::PCWSTR = ::windows::w!("OEMRLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTBUTTONCAP: ::windows::core::PCWSTR = ::windows::w!("OEMTestButtonCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTBUTTONDESC: ::windows::core::PCWSTR = ::windows::w!("OEMTestButtonDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTMOVECAP: ::windows::core::PCWSTR = ::windows::w!("OEMTestMoveCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTMOVEDESC: ::windows::core::PCWSTR = ::windows::w!("OEMTestMoveDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTWINCAP: ::windows::core::PCWSTR = ::windows::w!("OEMTestWinCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMULABEL: ::windows::core::PCWSTR = ::windows::w!("OEMULabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMVLABEL: ::windows::core::PCWSTR = ::windows::w!("OEMVLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMXYLABEL: ::windows::core::PCWSTR = ::windows::w!("OEMXYLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMZLABEL: ::windows::core::PCWSTR = ::windows::w!("OEMZLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYUSERVALUES: ::windows::core::PCWSTR = ::windows::w!("JoystickUserValues");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEBT: ::windows::core::PCWSTR = ::windows::w!("LastAliveBT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEINTERVAL: ::windows::core::PCWSTR = ::windows::w!("TimeStampInterval");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEPMPOLICY: ::windows::core::PCWSTR = ::windows::w!("LastAlivePMPolicy");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMP: ::windows::core::PCWSTR = ::windows::w!("LastAliveStamp");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMPFORCED: ::windows::core::PCWSTR = ::windows::w!("LastAliveStampForced");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMPINTERVAL: ::windows::core::PCWSTR = ::windows::w!("LastAliveStampInterval");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMPPOLICYINTERVAL: ::windows::core::PCWSTR = ::windows::w!("LastAliveStampPolicyInterval");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEUPTIME: ::windows::core::PCWSTR = ::windows::w!("LastAliveUptime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTBOOTPMDRVS: ::windows::core::PCWSTR = ::windows::w!("LastBootPMDrvs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTCOMPUTERNAME: ::windows::core::PCWSTR = ::windows::w!("LastComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTPCIBUSNUM: ::windows::core::PCWSTR = ::windows::w!("LastPCIBusNum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LAST_UPDATE_TIME: ::windows::core::PCWSTR = ::windows::w!("LastUpdateTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LEGALNOTICECAPTION: ::windows::core::PCWSTR = ::windows::w!("LegalNoticeCaption");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LEGALNOTICETEXT: ::windows::core::PCWSTR = ::windows::w!("LegalNoticeText");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LICENSINGINFO: ::windows::core::PCWSTR = ::windows::w!("LicensingInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LINKED: ::windows::core::PCWSTR = ::windows::w!("Linked");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOADHI: ::windows::core::PCWSTR = ::windows::w!("LoadHi");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOADRMDRIVERS: ::windows::core::PCWSTR = ::windows::w!("LoadRMDrivers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOCATION_INFORMATION: ::windows::core::PCWSTR = ::windows::w!("LocationInformation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOCATION_INFORMATION_OVERRIDE: ::windows::core::PCWSTR = ::windows::w!("LocationInformationOverride");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOWERFILTERS: ::windows::core::PCWSTR = ::windows::w!("LowerFilters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOWER_FILTER_DEFAULT_LEVEL: ::windows::core::PCWSTR = ::windows::w!("LowerFilterDefaultLevel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOWER_FILTER_LEVELS: ::windows::core::PCWSTR = ::windows::w!("LowerFilterLevels");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MACHINETYPE: ::windows::core::PCWSTR = ::windows::w!("MachineType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MANUFACTURER: ::windows::core::PCWSTR = ::windows::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAP: ::windows::core::PCWSTR = ::windows::w!("Map");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MATCHINGDEVID: ::windows::core::PCWSTR = ::windows::w!("MatchingDeviceId");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXCONNECTIONS: ::windows::core::PCWSTR = ::windows::w!("MaxConnections");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXLIP: ::windows::core::PCWSTR = ::windows::w!("MaxLIP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXRES: ::windows::core::PCWSTR = ::windows::w!("MaxResolution");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXRETRY: ::windows::core::PCWSTR = ::windows::w!("MaxRetry");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MEDIA: ::windows::core::PCWSTR = ::windows::w!("MediaPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MFG: ::windows::core::PCWSTR = ::windows::w!("Mfg");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MF_FLAGS: ::windows::core::PCWSTR = ::windows::w!("MFFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MINIPORT_STAT: ::windows::core::PCWSTR = ::windows::w!("MiniportStatus");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MINPWDLEN: ::windows::core::PCWSTR = ::windows::w!("MinPwdLen");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MINRETRY: ::windows::core::PCWSTR = ::windows::w!("MinRetry");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MODE: ::windows::core::PCWSTR = ::windows::w!("Mode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MODEL: ::windows::core::PCWSTR = ::windows::w!("Model");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MSDOSMODE: ::windows::core::PCWSTR = ::windows::w!("MSDOSMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MSDOSMODEDISCARD: ::windows::core::PCWSTR = ::windows::w!("Discard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MUSTBEVALIDATED: ::windows::core::PCWSTR = ::windows::w!("MustBeValidated");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NAMECACHECOUNT: ::windows::core::PCWSTR = ::windows::w!("NameCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NAMENUMERICTAIL: ::windows::core::PCWSTR = ::windows::w!("NameNumericTail");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NCP_BROWSEMASTER: ::windows::core::PCWSTR = ::windows::w!("BrowseMaster");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NCP_USEPEERBROWSING: ::windows::core::PCWSTR = ::windows::w!("Use_PeerBrowsing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NCP_USESAP: ::windows::core::PCWSTR = ::windows::w!("Use_Sap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NDP: ::windows::core::PCWSTR = ::windows::w!("NDP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETCARD: ::windows::core::PCWSTR = ::windows::w!("Netcard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETCLEAN: ::windows::core::PCWSTR = ::windows::w!("NetClean");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETOSTYPE: ::windows::core::PCWSTR = ::windows::w!("NetOSType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_DISABLE: ::windows::core::PCWSTR = ::windows::w!("NoNetSetup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_NOCONFIGPAGE: ::windows::core::PCWSTR = ::windows::w!("NoNetSetupConfigPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_NOIDPAGE: ::windows::core::PCWSTR = ::windows::w!("NoNetSetupIDPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_NOSECURITYPAGE: ::windows::core::PCWSTR = ::windows::w!("NoNetSetupSecurityPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOCMOSORFDPT: ::windows::core::PCWSTR = ::windows::w!("NoCMOSorFDPT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NODISPLAYCLASS: ::windows::core::PCWSTR = ::windows::w!("NoDisplayClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOENTIRENETWORK: ::windows::core::PCWSTR = ::windows::w!("NoEntireNetwork");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOFILESHARING: ::windows::core::PCWSTR = ::windows::w!("NoFileSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOFILESHARINGCTRL: ::windows::core::PCWSTR = ::windows::w!("NoFileSharingControl");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOIDE: ::windows::core::PCWSTR = ::windows::w!("NoIDE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOINSTALLCLASS: ::windows::core::PCWSTR = ::windows::w!("NoInstallClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NONSTANDARD_ATAPI: ::windows::core::PCWSTR = ::windows::w!("NonStandardATAPI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOPRINTSHARING: ::windows::core::PCWSTR = ::windows::w!("NoPrintSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOPRINTSHARINGCTRL: ::windows::core::PCWSTR = ::windows::w!("NoPrintSharingControl");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOUSECLASS: ::windows::core::PCWSTR = ::windows::w!("NoUseClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOWORKGROUPCONTENTS: ::windows::core::PCWSTR = ::windows::w!("NoWorkgroupContents");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OLDMSDOSVER: ::windows::core::PCWSTR = ::windows::w!("OldMSDOSVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OLDWINDIR: ::windows::core::PCWSTR = ::windows::w!("OldWinDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OPTIMIZESFN: ::windows::core::PCWSTR = ::windows::w!("OptimizeSFN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OPTIONS: ::windows::core::PCWSTR = ::windows::w!("Options");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OPTORDER: ::windows::core::PCWSTR = ::windows::w!("Order");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_P1284MDL: ::windows::core::PCWSTR = ::windows::w!("Model");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_P1284MFG: ::windows::core::PCWSTR = ::windows::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PATHCACHECOUNT: ::windows::core::PCWSTR = ::windows::w!("PathCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCCARD_POWER: ::windows::core::PCWSTR = ::windows::w!("EnablePowerManagement");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCI: ::windows::core::PCWSTR = ::windows::w!("PCI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCIBIOSVER: ::windows::core::PCWSTR = ::windows::w!("PCIBIOSVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCICIRQMAP: ::windows::core::PCWSTR = ::windows::w!("PCICIRQMap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCICOPTIONS: ::windows::core::PCWSTR = ::windows::w!("PCICOptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_ALLOC: ::windows::core::PCWSTR = ::windows::w!("AllocMemWin");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_ATAD: ::windows::core::PCWSTR = ::windows::w!("ATADelay");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_MEM: ::windows::core::PCWSTR = ::windows::w!("Memory");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_OPT: ::windows::core::PCWSTR = ::windows::w!("Options");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_SIZ: ::windows::core::PCWSTR = ::windows::w!("MinRegionSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMTDRIVER: ::windows::core::PCWSTR = ::windows::w!("MTD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCSSDRIVER: ::windows::core::PCWSTR = ::windows::w!("Driver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PHYSICALDEVICEOBJECT: ::windows::core::PCWSTR = ::windows::w!("PhysicalDeviceObject");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PMODE_INT13: ::windows::core::PCWSTR = ::windows::w!("PModeInt13");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PNPBIOSVER: ::windows::core::PCWSTR = ::windows::w!("PnPBIOSVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PNPSTRUCOFFSET: ::windows::core::PCWSTR = ::windows::w!("PnPStrucOffset");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_POLICY: ::windows::core::PCWSTR = ::windows::w!("Policy");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_POLLING: ::windows::core::PCWSTR = ::windows::w!("Polling");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PORTNAME: ::windows::core::PCWSTR = ::windows::w!("PortName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PORTSUBCLASS: ::windows::core::PCWSTR = ::windows::w!("PortSubClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PREFREDIR: ::windows::core::PCWSTR = ::windows::w!("PreferredRedir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRESERVECASE: ::windows::core::PCWSTR = ::windows::w!("PreserveCase");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRESERVELONGNAMES: ::windows::core::PCWSTR = ::windows::w!("PreserveLongNames");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_HIDETABS: ::windows::core::PCWSTR = ::windows::w!("NoPrinterTabs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_MASK: ::windows::core::PCWSTR = ::windows::w!("PrintersMask");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_NOADD: ::windows::core::PCWSTR = ::windows::w!("NoAddPrinter");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_NODELETE: ::windows::core::PCWSTR = ::windows::w!("NoDeletePrinter");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTSHARING: ::windows::core::PCWSTR = ::windows::w!("PrintSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("Priority");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIVATE: ::windows::core::PCWSTR = ::windows::w!("Private");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIVATEFUNC: ::windows::core::PCWSTR = ::windows::w!("PrivateFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIVATEPROBLEM: ::windows::core::PCWSTR = ::windows::w!("PrivateProblem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRODUCTID: ::windows::core::PCWSTR = ::windows::w!("ProductId");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRODUCTTYPE: ::windows::core::PCWSTR = ::windows::w!("ProductType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROFILEFLAGS: ::windows::core::PCWSTR = ::windows::w!("ProfileFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROPERTIES: ::windows::core::PCWSTR = ::windows::w!("Properties");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROTINIPATH: ::windows::core::PCWSTR = ::windows::w!("ProtIniPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROVIDER_NAME: ::windows::core::PCWSTR = ::windows::w!("ProviderName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDEXPIRATION: ::windows::core::PCWSTR = ::windows::w!("PwdExpiration");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_CHANGEORDER: ::windows::core::PCWSTR = ::windows::w!("ChangeOrder");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWD: ::windows::core::PCWSTR = ::windows::w!("ChangePassword");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWDHWND: ::windows::core::PCWSTR = ::windows::w!("ChangePasswordHwnd");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_DESC: ::windows::core::PCWSTR = ::windows::w!("Description");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_GETPWDSTATUS: ::windows::core::PCWSTR = ::windows::w!("GetPasswordStatus");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_ISNP: ::windows::core::PCWSTR = ::windows::w!("NetworkProvider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_PATH: ::windows::core::PCWSTR = ::windows::w!("ProviderPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RDINTTHRESHOLD: ::windows::core::PCWSTR = ::windows::w!("RDIntThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_READAHEADTHRESHOLD: ::windows::core::PCWSTR = ::windows::w!("ReadAheadThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_READCACHING: ::windows::core::PCWSTR = ::windows::w!("ReadCaching");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REALNETSTART: ::windows::core::PCWSTR = ::windows::w!("RealNetStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REASONCODE: ::windows::core::PCWSTR = ::windows::w!("ReasonCode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REFRESHRATE: ::windows::core::PCWSTR = ::windows::w!("RefreshRate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REGITEMDELETEMESSAGE: ::windows::core::PCWSTR = ::windows::w!("Removal Message");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REGORGANIZATION: ::windows::core::PCWSTR = ::windows::w!("RegisteredOrganization");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REGOWNER: ::windows::core::PCWSTR = ::windows::w!("RegisteredOwner");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REINSTALL_DEVICEINSTANCEIDS: ::windows::core::PCWSTR = ::windows::w!("DeviceInstanceIds");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REINSTALL_DISPLAYNAME: ::windows::core::PCWSTR = ::windows::w!("DisplayName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REINSTALL_STRING: ::windows::core::PCWSTR = ::windows::w!("ReinstallString");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOTE_PATH: ::windows::core::PCWSTR = ::windows::w!("RemotePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVABLE: ::windows::core::PCWSTR = ::windows::w!("Removable");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVAL_POLICY: ::windows::core::PCWSTR = ::windows::w!("RemovalPolicy");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVEROMOKAY: ::windows::core::PCWSTR = ::windows::w!("RemoveRomOkay");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVEROMOKAYFUNC: ::windows::core::PCWSTR = ::windows::w!("RemoveRomOkayFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESERVED_DEVNODE: ::windows::core::PCWSTR = ::windows::w!("HTREE\\RESERVED\\0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOLUTION: ::windows::core::PCWSTR = ::windows::w!("Resolution");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCES: ::windows::core::PCWSTR = ::windows::w!("Resources");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCE_MAP: ::windows::core::PCWSTR = ::windows::w!("ResourceMap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCE_PICKER_EXCEPTIONS: ::windows::core::PCWSTR = ::windows::w!("ResourcePickerExceptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCE_PICKER_TAGS: ::windows::core::PCWSTR = ::windows::w!("ResourcePickerTags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESTRICTRUN: ::windows::core::PCWSTR = ::windows::w!("RestrictRun");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESUMERESET: ::windows::core::PCWSTR = ::windows::w!("ResumeReset");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REVISION: ::windows::core::PCWSTR = ::windows::w!("Revision");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REVLEVEL: ::windows::core::PCWSTR = ::windows::w!("RevisionLevel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ROOT_DEVNODE: ::windows::core::PCWSTR = ::windows::w!("HTREE\\ROOT\\0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RUNLOGINSCRIPT: ::windows::core::PCWSTR = ::windows::w!("ProcessLoginScript");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCANNER: ::windows::core::PCWSTR = ::windows::w!("SCANNER");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCAN_ONLY_FIRST: ::windows::core::PCWSTR = ::windows::w!("ScanOnlyFirstDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCSI: ::windows::core::PCWSTR = ::windows::w!("SCSI\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCSILUN: ::windows::core::PCWSTR = ::windows::w!("SCSILUN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCSITID: ::windows::core::PCWSTR = ::windows::w!("SCSITargetID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SEARCHMODE: ::windows::core::PCWSTR = ::windows::w!("SearchMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SEARCHOPTIONS: ::windows::core::PCWSTR = ::windows::w!("SearchOptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOADMINPAGE: ::windows::core::PCWSTR = ::windows::w!("NoAdminPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOPROFILEPAGE: ::windows::core::PCWSTR = ::windows::w!("NoProfilePage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOPWDPAGE: ::windows::core::PCWSTR = ::windows::w!("NoPwdPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOSECCPL: ::windows::core::PCWSTR = ::windows::w!("NoSecCPL");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SERVICE: ::windows::core::PCWSTR = ::windows::w!("Service");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPFLAGS: ::windows::core::PCWSTR = ::windows::w!("SetupFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPMACHINETYPE: ::windows::core::PCWSTR = ::windows::w!("SetupMachineType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPN: ::windows::core::PCWSTR = ::windows::w!("SetupN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPNPATH: ::windows::core::PCWSTR = ::windows::w!("SetupNPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPPROGRAMRAN: ::windows::core::PCWSTR = ::windows::w!("SetupProgramRan");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_FLAGS: ::windows::core::PCWSTR = ::windows::w!("Flags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_PATH: ::windows::core::PCWSTR = ::windows::w!("Path");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_REMARK: ::windows::core::PCWSTR = ::windows::w!("Remark");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_RO_PASS: ::windows::core::PCWSTR = ::windows::w!("Parm2");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_RW_PASS: ::windows::core::PCWSTR = ::windows::w!("Parm1");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_TYPE: ::windows::core::PCWSTR = ::windows::w!("Type");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARE_IRQ: ::windows::core::PCWSTR = ::windows::w!("ForceIRQSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHELLVERSION: ::windows::core::PCWSTR = ::windows::w!("ShellVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHOWDOTS: ::windows::core::PCWSTR = ::windows::w!("ShowDots");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHOWREASONUI: ::windows::core::PCWSTR = ::windows::w!("ShutdownReasonUI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON: ::windows::core::PCWSTR = ::windows::w!("ShutdownReason");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_CODE: ::windows::core::PCWSTR = ::windows::w!("ShutdownReasonCode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_COMMENT: ::windows::core::PCWSTR = ::windows::w!("ShutdownReasonComment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_PROCESS: ::windows::core::PCWSTR = ::windows::w!("ShutdownReasonProcess");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_USERNAME: ::windows::core::PCWSTR = ::windows::w!("ShutdownReasonUserName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWN_FLAGS: ::windows::core::PCWSTR = ::windows::w!("ShutdownFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWN_IGNORE_PREDEFINED: ::windows::core::PCWSTR = ::windows::w!("ShutdownIgnorePredefinedReasons");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWN_STATE_SNAPSHOT: ::windows::core::PCWSTR = ::windows::w!("ShutdownStateSnapshot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SILENTINSTALL: ::windows::core::PCWSTR = ::windows::w!("SilentInstall");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SLSUPPORT: ::windows::core::PCWSTR = ::windows::w!("SLSupport");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SOFTCOMPATMODE: ::windows::core::PCWSTR = ::windows::w!("SoftCompatMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRCPATH: ::windows::core::PCWSTR = ::windows::w!("SourcePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRVNAMECACHE: ::windows::core::PCWSTR = ::windows::w!("ServerNameCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRVNAMECACHECOUNT: ::windows::core::PCWSTR = ::windows::w!("ServerNameCacheMax");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRVNAMECACHENETPROV: ::windows::core::PCWSTR = ::windows::w!("ServerNameCacheNumNets");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_START_ON_BOOT: ::windows::core::PCWSTR = ::windows::w!("StartOnBoot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STAT: ::windows::core::PCWSTR = ::windows::w!("Status");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STATICDRIVE: ::windows::core::PCWSTR = ::windows::w!("StaticDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STATICVXD: ::windows::core::PCWSTR = ::windows::w!("StaticVxD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STDDOSOPTION: ::windows::core::PCWSTR = ::windows::w!("StdOption");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUBMODEL: ::windows::core::PCWSTR = ::windows::w!("Submodel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUPPORTBURST: ::windows::core::PCWSTR = ::windows::w!("SupportBurst");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUPPORTLFN: ::windows::core::PCWSTR = ::windows::w!("SupportLFN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUPPORTTUNNELLING: ::windows::core::PCWSTR = ::windows::w!("SupportTunnelling");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYMBOLIC_LINK: ::windows::core::PCWSTR = ::windows::w!("SymbolicLink");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYNCDATAXFER: ::windows::core::PCWSTR = ::windows::w!("SyncDataXfer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSDM: ::windows::core::PCWSTR = ::windows::w!("SysDM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSDMFUNC: ::windows::core::PCWSTR = ::windows::w!("SysDMFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NOCONFIGPAGE: ::windows::core::PCWSTR = ::windows::w!("NoConfigPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NODEVMGRPAGE: ::windows::core::PCWSTR = ::windows::w!("NoDevMgrPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NOFILESYSPAGE: ::windows::core::PCWSTR = ::windows::w!("NoFileSysPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NOVIRTMEMPAGE: ::windows::core::PCWSTR = ::windows::w!("NoVirtMemPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMROOT: ::windows::core::PCWSTR = ::windows::w!("SystemRoot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTRAYBATFLAGS: ::windows::core::PCWSTR = ::windows::w!("PowerFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTRAYPCCARDFLAGS: ::windows::core::PCWSTR = ::windows::w!("PCMCIAFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTRAYSVCS: ::windows::core::PCWSTR = ::windows::w!("Services");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TABLE_STAT: ::windows::core::PCWSTR = ::windows::w!("TableStatus");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TAPE: ::windows::core::PCWSTR = ::windows::w!("TAPE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TRANSITION: ::windows::core::PCWSTR = ::windows::w!("Transition");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TRANSPORT: ::windows::core::PCWSTR = ::windows::w!("Transport");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZACTBIAS: ::windows::core::PCWSTR = ::windows::w!("ActiveTimeBias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZBIAS: ::windows::core::PCWSTR = ::windows::w!("Bias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTBIAS: ::windows::core::PCWSTR = ::windows::w!("DaylightBias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTFLAG: ::windows::core::PCWSTR = ::windows::w!("DaylightFlag");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTNAME: ::windows::core::PCWSTR = ::windows::w!("DaylightName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTSTART: ::windows::core::PCWSTR = ::windows::w!("DaylightStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZNOAUTOTIME: ::windows::core::PCWSTR = ::windows::w!("DisableAutoDaylightTimeSet");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZNOCHANGEEND: ::windows::core::PCWSTR = ::windows::w!("NoChangeEnd");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZNOCHANGESTART: ::windows::core::PCWSTR = ::windows::w!("NoChangeStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZSTDBIAS: ::windows::core::PCWSTR = ::windows::w!("StandardBias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZSTDNAME: ::windows::core::PCWSTR = ::windows::w!("StandardName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZSTDSTART: ::windows::core::PCWSTR = ::windows::w!("StandardStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UI_NUMBER: ::windows::core::PCWSTR = ::windows::w!("UINumber");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UI_NUMBER_DESC_FORMAT: ::windows::core::PCWSTR = ::windows::w!("UINumberDescFormat");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UNDOCK_WITHOUT_LOGON: ::windows::core::PCWSTR = ::windows::w!("UndockWithoutLogon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UNINSTALLER_COMMANDLINE: ::windows::core::PCWSTR = ::windows::w!("UninstallString");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UNINSTALLER_DISPLAYNAME: ::windows::core::PCWSTR = ::windows::w!("DisplayName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPGRADE: ::windows::core::PCWSTR = ::windows::w!("Upgrade");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPPERFILTERS: ::windows::core::PCWSTR = ::windows::w!("UpperFilters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPPER_FILTER_DEFAULT_LEVEL: ::windows::core::PCWSTR = ::windows::w!("UpperFilterDefaultLevel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPPER_FILTER_LEVELS: ::windows::core::PCWSTR = ::windows::w!("UpperFilterLevels");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_USERSETTINGS: ::windows::core::PCWSTR = ::windows::w!("AdapterSettings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_USER_NAME: ::windows::core::PCWSTR = ::windows::w!("UserName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_USRDRVLET: ::windows::core::PCWSTR = ::windows::w!("UserDriveLetterAssignment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VDD: ::windows::core::PCWSTR = ::windows::w!("vdd");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VER: ::windows::core::PCWSTR = ::windows::w!("Ver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VERIFYKEY: ::windows::core::PCWSTR = ::windows::w!("VerifyKey");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VIRTUALHDIRQ: ::windows::core::PCWSTR = ::windows::w!("VirtualHDIRQ");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VOLIDLETIMEOUT: ::windows::core::PCWSTR = ::windows::w!("VolumeIdleTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VPOWERDFLAGS: ::windows::core::PCWSTR = ::windows::w!("Flags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VRES: ::windows::core::PCWSTR = ::windows::w!("VRes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VXDGROUPS: ::windows::core::PCWSTR = ::windows::w!("VXDGroups");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WAITFORUNDOCK: ::windows::core::PCWSTR = ::windows::w!("WaitForUndock");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WAITFORUNDOCKFUNC: ::windows::core::PCWSTR = ::windows::w!("WaitForUndockFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WIN31FILESYSTEM: ::windows::core::PCWSTR = ::windows::w!("Win31FileSystem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WIN31PROVIDER: ::windows::core::PCWSTR = ::windows::w!("Win31Provider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINBOOTDIR: ::windows::core::PCWSTR = ::windows::w!("WinbootDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINCP: ::windows::core::PCWSTR = ::windows::w!("ACP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINDIR: ::windows::core::PCWSTR = ::windows::w!("WinDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINOLDAPP_DISABLED: ::windows::core::PCWSTR = ::windows::w!("Disabled");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINOLDAPP_NOREALMODE: ::windows::core::PCWSTR = ::windows::w!("NoRealMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WORKGROUP: ::windows::core::PCWSTR = ::windows::w!("Workgroup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRAPPER: ::windows::core::PCWSTR = ::windows::w!("Wrapper");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRINTTHRESHOLD: ::windows::core::PCWSTR = ::windows::w!("WRIntThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRKGRP_FORCEMAPPING: ::windows::core::PCWSTR = ::windows::w!("WrkgrpForceMapping");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRKGRP_REQUIRED: ::windows::core::PCWSTR = ::windows::w!("WrkgrpRequired");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_KEY_INSTDEV: ::windows::core::PCWSTR = ::windows::w!("Installed");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_PROCESS_APPKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_SECURE_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_BATCHINF: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_CLEAN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_EXPRESS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_FIRSTTIME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_INSETUP: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_NETHDBOOT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_NETRPLBOOT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_NETSETUP: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const SUF_SBSCOPYOK: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const VPDF_DISABLEPWRMGMT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const VPDF_DISABLEPWRSTATUSPOLL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const VPDF_DISABLERINGRESUME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const VPDF_FORCEAPM10MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const VPDF_SHOWMULTIBATT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const VPDF_SKIPINTELSLCHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_CREATE_KEY_DISPOSITION(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(2u32);
impl ::core::marker::Copy for REG_CREATE_KEY_DISPOSITION {}
impl ::core::clone::Clone for REG_CREATE_KEY_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_CREATE_KEY_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_CREATE_KEY_DISPOSITION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_CREATE_KEY_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_CREATE_KEY_DISPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_NOTIFY_FILTER(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(268435456u32);
impl ::core::marker::Copy for REG_NOTIFY_FILTER {}
impl ::core::clone::Clone for REG_NOTIFY_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_NOTIFY_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_NOTIFY_FILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_NOTIFY_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_NOTIFY_FILTER").field(&self.0).finish()
    }
}
impl REG_NOTIFY_FILTER {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_NOTIFY_FILTER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_NOTIFY_FILTER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_NOTIFY_FILTER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_OPEN_CREATE_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(16u32);
impl ::core::marker::Copy for REG_OPEN_CREATE_OPTIONS {}
impl ::core::clone::Clone for REG_OPEN_CREATE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_OPEN_CREATE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_OPEN_CREATE_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_OPEN_CREATE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_OPEN_CREATE_OPTIONS").field(&self.0).finish()
    }
}
impl REG_OPEN_CREATE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_RESTORE_KEY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(1i32);
impl ::core::marker::Copy for REG_RESTORE_KEY_FLAGS {}
impl ::core::clone::Clone for REG_RESTORE_KEY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_RESTORE_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_RESTORE_KEY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_RESTORE_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_RESTORE_KEY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_ROUTINE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_DWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(24u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_QWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(72u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_NONE: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_EXPAND_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_BINARY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_DWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_MULTI_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_QWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_ANY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(65535u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_SUBKEY_WOW6464KEY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_SUBKEY_WOW6432KEY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_WOW64_MASK: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(196608u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_NOEXPAND: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_ZEROONFAILURE: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(536870912u32);
impl ::core::marker::Copy for REG_ROUTINE_FLAGS {}
impl ::core::clone::Clone for REG_ROUTINE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_ROUTINE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_ROUTINE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_ROUTINE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_ROUTINE_FLAGS").field(&self.0).finish()
    }
}
impl REG_ROUTINE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for REG_ROUTINE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_ROUTINE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_ROUTINE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_ROUTINE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_ROUTINE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_SAM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_SET_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_CREATE_SUB_KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_ENUMERATE_SUB_KEYS: REG_SAM_FLAGS = REG_SAM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_NOTIFY: REG_SAM_FLAGS = REG_SAM_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_CREATE_LINK: REG_SAM_FLAGS = REG_SAM_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WOW64_32KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WOW64_64KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WOW64_RES: REG_SAM_FLAGS = REG_SAM_FLAGS(768u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_READ: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WRITE: REG_SAM_FLAGS = REG_SAM_FLAGS(131078u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_EXECUTE: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_ALL_ACCESS: REG_SAM_FLAGS = REG_SAM_FLAGS(983103u32);
impl ::core::marker::Copy for REG_SAM_FLAGS {}
impl ::core::clone::Clone for REG_SAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_SAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_SAM_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_SAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_SAM_FLAGS").field(&self.0).finish()
    }
}
impl REG_SAM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for REG_SAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_SAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_SAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_SAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_SAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_SAVE_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = REG_SAVE_FORMAT(4u32);
impl ::core::marker::Copy for REG_SAVE_FORMAT {}
impl ::core::clone::Clone for REG_SAVE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_SAVE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_SAVE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_SAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_SAVE_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_VALUE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NONE: REG_VALUE_TYPE = REG_VALUE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_BINARY: REG_VALUE_TYPE = REG_VALUE_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_DWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_DWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_DWORD_BIG_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(5u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_LINK: REG_VALUE_TYPE = REG_VALUE_TYPE(6u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_MULTI_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(7u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_RESOURCE_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_FULL_RESOURCE_DESCRIPTOR: REG_VALUE_TYPE = REG_VALUE_TYPE(9u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_RESOURCE_REQUIREMENTS_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(10u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_QWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_QWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
impl ::core::marker::Copy for REG_VALUE_TYPE {}
impl ::core::clone::Clone for REG_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REG_VALUE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REG_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_VALUE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct DSKTLSYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
    pub wResult: u16,
}
impl ::core::marker::Copy for DSKTLSYSTEMTIME {}
impl ::core::clone::Clone for DSKTLSYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSKTLSYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSKTLSYSTEMTIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDayOfWeek", &self.wDayOfWeek).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).field("wResult", &self.wResult).finish()
    }
}
impl ::windows::core::TypeKind for DSKTLSYSTEMTIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DSKTLSYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDayOfWeek == other.wDayOfWeek && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond && self.wMilliseconds == other.wMilliseconds && self.wResult == other.wResult
    }
}
impl ::core::cmp::Eq for DSKTLSYSTEMTIME {}
impl ::core::default::Default for DSKTLSYSTEMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HKEY(pub isize);
impl HKEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HKEY {}
impl ::core::fmt::Debug for HKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HKEY").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HKEY {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct PVALUEA {
    pub pv_valuename: ::windows::core::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::core::ffi::c_void,
    pub pv_type: u32,
}
impl ::core::marker::Copy for PVALUEA {}
impl ::core::clone::Clone for PVALUEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PVALUEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PVALUEA").field("pv_valuename", &self.pv_valuename).field("pv_valuelen", &self.pv_valuelen).field("pv_value_context", &self.pv_value_context).field("pv_type", &self.pv_type).finish()
    }
}
impl ::windows::core::TypeKind for PVALUEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PVALUEA {
    fn eq(&self, other: &Self) -> bool {
        self.pv_valuename == other.pv_valuename && self.pv_valuelen == other.pv_valuelen && self.pv_value_context == other.pv_value_context && self.pv_type == other.pv_type
    }
}
impl ::core::cmp::Eq for PVALUEA {}
impl ::core::default::Default for PVALUEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct PVALUEW {
    pub pv_valuename: ::windows::core::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::core::ffi::c_void,
    pub pv_type: u32,
}
impl ::core::marker::Copy for PVALUEW {}
impl ::core::clone::Clone for PVALUEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PVALUEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PVALUEW").field("pv_valuename", &self.pv_valuename).field("pv_valuelen", &self.pv_valuelen).field("pv_value_context", &self.pv_value_context).field("pv_type", &self.pv_type).finish()
    }
}
impl ::windows::core::TypeKind for PVALUEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PVALUEW {
    fn eq(&self, other: &Self) -> bool {
        self.pv_valuename == other.pv_valuename && self.pv_valuelen == other.pv_valuelen && self.pv_value_context == other.pv_value_context && self.pv_type == other.pv_type
    }
}
impl ::core::cmp::Eq for PVALUEW {}
impl ::core::default::Default for PVALUEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct REG_PROVIDER {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for REG_PROVIDER {}
impl ::core::clone::Clone for REG_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REG_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REG_PROVIDER").field("pi_flags", &self.pi_flags).field("pi_key_context", &self.pi_key_context).finish()
    }
}
impl ::windows::core::TypeKind for REG_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for REG_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct VALENTA {
    pub ve_valuename: ::windows::core::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
impl ::core::marker::Copy for VALENTA {}
impl ::core::clone::Clone for VALENTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VALENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VALENTA").field("ve_valuename", &self.ve_valuename).field("ve_valuelen", &self.ve_valuelen).field("ve_valueptr", &self.ve_valueptr).field("ve_type", &self.ve_type).finish()
    }
}
impl ::windows::core::TypeKind for VALENTA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VALENTA {
    fn eq(&self, other: &Self) -> bool {
        self.ve_valuename == other.ve_valuename && self.ve_valuelen == other.ve_valuelen && self.ve_valueptr == other.ve_valueptr && self.ve_type == other.ve_type
    }
}
impl ::core::cmp::Eq for VALENTA {}
impl ::core::default::Default for VALENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct VALENTW {
    pub ve_valuename: ::windows::core::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
impl ::core::marker::Copy for VALENTW {}
impl ::core::clone::Clone for VALENTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VALENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VALENTW").field("ve_valuename", &self.ve_valuename).field("ve_valuelen", &self.ve_valuelen).field("ve_valueptr", &self.ve_valueptr).field("ve_type", &self.ve_type).finish()
    }
}
impl ::windows::core::TypeKind for VALENTW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VALENTW {
    fn eq(&self, other: &Self) -> bool {
        self.ve_valuename == other.ve_valuename && self.ve_valuelen == other.ve_valuelen && self.ve_valueptr == other.ve_valueptr && self.ve_type == other.ve_type
    }
}
impl ::core::cmp::Eq for VALENTW {}
impl ::core::default::Default for VALENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut ::core::ffi::c_void,
    pub val_buff_ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for val_context {}
impl ::core::clone::Clone for val_context {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for val_context {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("val_context").field("valuelen", &self.valuelen).field("value_context", &self.value_context).field("val_buff_ptr", &self.val_buff_ptr).finish()
    }
}
impl ::windows::core::TypeKind for val_context {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for val_context {
    fn eq(&self, other: &Self) -> bool {
        self.valuelen == other.valuelen && self.value_context == other.value_context && self.val_buff_ptr == other.val_buff_ptr
    }
}
impl ::core::cmp::Eq for val_context {}
impl ::core::default::Default for val_context {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub type PQUERYHANDLER = ::core::option::Option<unsafe extern "system" fn(keycontext: *mut ::core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut ::core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
